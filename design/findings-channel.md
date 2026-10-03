# The findings channel — `jigc-feedback` · `inconsistency` (M55)

**M55 design of record.** The place a finding goes, shipped before the port that fills it ([roadmap.md](../implementation/roadmap.md) → M55). Two methodology-pack doctypes, one doc per finding, each filed — and later triaged — through a code-less workflow that can land nothing but its own doc; the read surface that makes a populated store triageable; the three latent defects the port would hit on every branch; and the seed that turns a new doctype into a populated ledger on the day the port adopts it. Settled at M55 planning, every fork by the human ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; the working settle log, with ids S1–S17 cited below and the design-draft resolutions at its end, is [completions/artifacts/M55/settle-log.md](../completions/artifacts/M55/settle-log.md); the evidence is [planning-findings.md](../completions/artifacts/M55/planning-findings.md), [baseline-ledger.md](../completions/artifacts/M55/baseline-ledger.md) and [gap-list.md](../completions/artifacts/M55/gap-list.md); the field set began as Revision 1 of [doctype-proposal.md](../completions/artifacts/M55/doctype-proposal.md), cross-reviewed in [review-doctypes-codex.md](../completions/artifacts/M55/review-doctypes-codex.md), and superseded by this doc). **Revised at the design review the same day** — an independent adversarial read of the written Settle, every finding accepted by the human (settle log → Review phase, R1–R4): R1 revised S4 (§3), R2 revised S7 (§4), R3 settled S2's mechanism (§6), and R4's findings are folded where they land, the revision list among them (§13). **Revised again at the planning gate-record** ([planning-gate-record.md](../completions/artifacts/M55/planning-gate-record.md)) — its five halts resolved by the human the same day (settle log → R5): L2's route changes at both scopes (§6, §13), S2's omission set is derived from the packs rather than hand-listed (§6), an `allows-create` entry refuses unknown keys at pack-load (§4), and L1's store arm reuses `file-state.hash-matches` behind the conformance check (§6); its sizing corrections land in §1.7, §2, §3 and §7.

Reads after [team-ready-state.md](team-ready-state.md) (the methodology pack's work-doc surface it extends) and before [measurement.md](measurement.md). Notation is **illustrative** ([what that disclaims](../CLAUDE.md#how-we-work-together)): the YAML, argv and output sketches below are shapes, and the schema files the build ships are their source of truth.

## What M55 settles

| Settle | Decision | § |
|---|---|---|
| S1 | **One doc per finding** — a located, multi-instance doctype; reporters only create | 1 |
| S2 | **Two doctypes by purpose** — `inconsistency` (what a project's own docs and code disagree on; generic, adopter-facing) and `jigc-feedback` (findings *about* jigc; internal for now) | 1 |
| S3 | `jigc-feedback` ships in the methodology pack with its workflow **hidden**; `report-inconsistency` is router-visible | 2 |
| S4 | A new pack step **`step:finalize-doc-only`** whose commit is **path-scoped** — a report lands its doc and nothing else, and every other staged path stays staged (revised by R1) | 3 |
| Mechanism line | Build new mechanism now when that is cheaper than reworking it or being blocked later | below |
| S5 | **Append-only by convention**, stated; the engine does not enforce it; after filing only `status` · `resolution` · `duplicate-of` · `pinned-by` change (widened by R4) | 1.5 |
| S6 | `pinned-by` is a plain string in [pinning.md](../implementation/pinning.md) §3's grammar | 1.3 |
| S7 | **Create-only on the create-gate entry** — `allows-create: [{type, as, new: true}]` — refuses an existing doc at `doc create` and `doc author` alike; an unknown key on the entry is refused at pack-load (R5); the `write.title-ignored` route is corrected (revised by R2) | 4 |
| S8 | Gate speed lands as its own `work/gate-speed` PR before the build — not part of this design | 12 |
| S9 | The field sets — Revision 1 | 1.1–1.2 |
| S10 | Read surface: `title` on `doc show`; `title` + header `fields` on `doc list` rows; an absent defaulted field projects its default — the effective value (row values fixed by R4) | 5 |
| S11 | This repo's feedback before the port: a **seed** generated in M55, adopted at M56 | 7 |
| S12 | The seed set and its rules | 7 |
| S13 | Fix **L1** (with a store-scope arm, R4, on `file-state.hash-matches`, R5), **L2** (its route changed at both scopes, R5), **S2** (by the CLI-emitted composition mold, R3, over an omission set derived from the packs, R5); declare **L3** and **two code tasks in one checkout** as bounds | 6 |
| S14 | Fold "every fork lists the option that removes the artifact that can drift" into the `cheap-vs-robust` gate | 9 |
| S15 | A committed, generated crate README with absolute links | 8 |
| S16 | M55's partial re-review axes, merged with M54's | 12 |
| S17 | Status changes after filing go through **two hidden triage workflows**, `triage-jigc-feedback` and `triage-inconsistency` | 2, 1.5 |

## The mechanism line

M53's *"no new mechanism — registry rows and guard conditions only"* was a fix pass's boundary, and M55's charter inherited it as *"no new engine mechanism."* The human revised it at this Settle: **it is not absolute. If introducing a mechanism now is cheaper than rebuilding it later or being blocked by its absence later, introduce it now** — definitely when the cheap cut would need rework or would block. Guards inside existing CLI doors are fine regardless. Every fork below was weighed on *cost now vs (rework + block) later*, and M55 admits these under the line:

| New | Kind | Why now rather than later |
|---|---|---|
| `step:finalize-doc-only` + its path-scoped commit | pack step + a third commit model inside an existing door, on the milestone-record door's mechanism over a new multi-path commit helper (R5) | the port runs report tasks beside code tasks in one checkout; without it, a report silently commits a teammate's code (§3) |
| `new: true` on the create-gate entry + its refusal | one optional key on the workflow front-matter's `allows-create` entry + a write reject at the gate + unknown entry keys refused at pack-load (R5) | one doc per finding makes a same-title create a silent overwrite of an earlier finding (§4) |
| `title` / `fields` on the read surfaces | additive read keys | free inside the pre-1.0 window, a versioned extension after the pin (§5) |
| L1 / L2 / S2 | conditions on existing arms; a composition rule on an existing CLI-emitted mold, its omission set derived from the packs and fenced against them (R5) | the port's branch-per-milestone model hits all three constantly (§6) |
| the crate README generator | `dev/` tooling + a fence | owed before release PR #2 merges, and a publish-time rewrite would ship untested bytes (§8) |

Not admitted, each parked with its trigger (→ Parked): a split/merge transform between one file and many, an edit gate, a code anchor, server-side filtering, a per-worktree store root, per-task path claims.

## 1. Two doctypes, one doc per finding

**One doc per finding (S1).** One file, one purpose: a per-batch doc (the advocate's pick) holds many findings, against that rule; a singleton ledger serializes every fan-out (`join.same-doc-clash`), hits L1 after every pull and conflicts in git across branches, where a naive union resolve dropped a field silently (planning-findings F8). One doc per finding gives reporters disjoint files, a finding is referenced precisely by its own identity, and append-only is free at the doc level (there is no doc-delete verb). **Costs carried:** one created finding per task (`write.identity-change`, F19); the same-title silent overwrite (bounded by §4); reads are `doc list` + `doc show` per item until §5 lands. A singleton↔per-doc transform would make the choice reversible — parked ([ideas/cardinality-split-merge-transform.md](../ideas/cardinality-split-merge-transform.md)).

**Two doctypes, by purpose (S2).** What an adoption or migration surfaces about **the project** — a code↔doc or doc↔doc disagreement — is the adopter's own; it is generic, router-visible, and not named after migration, because drift found later is the same thing. What anyone working with jigc finds **about jigc** — a bug, an inconvenience, feedback — is ours: it lands in the reporter's own repository, since the CLI makes no network calls, so it is **internal only** until a feedback interface exists ([ideas/feedback-web-service.md](../ideas/feedback-web-service.md)). Two doctypes also answer fork 2: drift is not a `kind:` on a shared doctype, because no conditional requiredness exists in the schema format and the two field sets share almost nothing.

**Names.** Neither id says *finding* — that word already means validation output across the CLI and its pinned envelope ([command-output-contract.md](command-output-contract.md)); neither says *drift* — jigc uses it for out-of-band file edits ([reconciliation.md](reconciliation.md)). `jigc-feedback` is namespaced so it shadows nothing an adopter defines (S3; F13 measured that a same-id built-in silently demotes a listed-pack doctype). `inconsistency` keeps its plain id: an adopter's same-named doctype would be shadowed, but that is F13's general defect — true of every built-in doctype id — and is seeded as such, not dodged per doctype.

**Neither is a backlog.** [doctype-map.md](../implementation/doctype-map.md) → Deliberate outs excludes a `TODO`/backlog doctype because the work-unit model and the roadmap own committed work. A finding records **observed evidence**, not scheduled work: it carries no trigger and no owner, accepted work is promoted to the roadmap/milestone system, and the finding keeps only its resolution. That is why `trigger` was not taken (Revision 1 → Not taken).

### 1.1 `jigc-feedback`

Methodology pack · `location: jigc-feedback/` (resolved through `docs-root`, so `docs/jigc-feedback/<slug>.md` under the composed `[dev ▸ methodology]` cascade) · `id-from: title` · multi-instance · manifest-frozen at **schema-version 1** ([corpus-migration.md](corpus-migration.md) → The freeze-exempt sibling, the M40 revision).

| Leaf | Type | Req | Meaning |
|---|---|---|---|
| `kind` | enum `bug · inconvenience · feedback` | yes | **bug** — the binary does something wrong (a false statement, a lost byte, a refused correct act, a wrong exit); **inconvenience** — it does what it says, at a cost an agent or human should not pay; **feedback** — an observation or suggestion that names no defect. Each member is defined in the schema's `usage` (the circular `feedback`-in-`jigc-feedback` is called out there) |
| `found-in` | string, `<kind>:<label>` (§1.3) | yes | **where it was found** — the human's context reference, so feedback can be sorted, grouped and filtered by origin |
| `about` | string (§1.3) | no | the surface it concerns |
| `jigc-version` | string, `<semver>[+<short-sha>]` | yes | the jigc version in use when it was seen. Not `version`: that collides with the injected `schema-version` in `doc show` json |
| `status` | enum `open · resolved · declined · duplicate · refuted`, default `open` | yes | §1.5 |
| `tier` | enum `tier-1 · tier-2 · tier-3` | no | the consequence grade (§1.4). Numerals were refused at pack-load (`expected a string`) |
| `duplicate-of` | ref → `jigc-feedback`, card 0..1 | no | the row this one duplicates; resolves at finalize, dangling blocks |
| `pinned-by` | string, pinning.md §3 grammar | no | the test that pins the fix (S6) |
| `date` | date, `set: on-create` | yes | the **filing** date, CLI-set |
| `description` | slot | yes | what was observed |
| `repro` | slot | no | the repro, as a **fenced** block (an unfenced heading is refused `write.slot-heading-depth`; the author step says so) |
| `resolution` | slot | no | why the status left `open` — what fixed it, or why declined/duplicate/refuted |

### 1.2 `inconsistency`

Methodology pack · `location: inconsistencies/` · `id-from: title` · multi-instance · schema-version 1.

| Leaf | Type | Req | Meaning |
|---|---|---|---|
| `kind` | enum `code-doc · doc-doc` | yes | which artifacts disagree |
| `status` | enum `open · resolved · intended · refuted`, default `open` | yes | §1.5 |
| `date` | date, `set: on-create` | yes | filing date |
| `sides` | **repeatable section**; item title = the path or address of one side, optional `says` slot | ≥2 by convention | each artifact in the disagreement. A list, not `left`/`right`: the seed already holds three- to five-sided cases (D1, D3), and two fields → a list later is a refused removal |
| `description` | slot | yes | what disagrees |
| `evidence` | slot | no | how it was established |
| `resolution` | slot | no | why the status left `open` |

`found-in` is not on `inconsistency` — `sides` already locates it; an optional field is a cheap later addition (Revision 1 → Not taken).

### 1.3 The string grammars

All four are **plain strings with a convention**, validated by nothing — a typed ref cannot hold them: tasks, workflows and reviews are not docs, seed rows have no milestone-record to point at, and a dangling ref blocks finalize with a route a reporter cannot act on (driven at the review).

| Field | Grammar | Examples |
|---|---|---|
| `found-in` | `<kind>:<label>` — kind one of `milestone` · `task` · `workflow` · `review` · `trial`; the label is **free text, not a slug** | `milestone:findings-channel` · `review:M52-per-axis/(2,DEFECT C)` · `review:M55-completion/F3` |
| `about` | `jigc <verb>` · `workflow:<id>` · `step:<id>` · `doctype:<id>` · `<finding-code>` · `guide:<file>` · `dev:<tool>` · `test:<module>` | `jigc task finalize` · `finalize.carried-staged` · `guide:QUICKSTART.md` · `dev:jigc-rig` · `test:doc_link_fence` |
| `jigc-version` | `<semver>[+<short-sha>]` — the version in use when the finding was seen | `1.0.0-rc.23` · `1.0.0-rc.23+0a1b2c3d` |
| `pinned-by` | `<module>::<test_name>` or `UNPINNED: <why>` — [pinning.md](../implementation/pinning.md) §3 is its one home; a repro block points at the field rather than repeating it | `doc_read_surface::bare_single_hop_alias_resolves` |

### 1.4 The tier scale

`tier` is the M52-onward review grade; its meaning is **not restated here**. The scale's one home is the three-tier definition in the rc.16 charter — [decisions-pending.md](../implementation/decisions-pending.md) → *The rc.16 wave (M52)*, its three tier headings (*exit-0 loss or repository harm through a committing or destroying door* · *posture and route dead ends, code-less or undeclared refusals* · *surfaces that say something the binary does not do*). The charter numbers them 0 · 1 · 2; the review grade, from [the M52 per-axis review](../completions/artifacts/M52/per-axis-review/README.md) on, numbers the same predicate `tier-1` · `tier-2` · `tier-3`, and the enum takes the review's. The exit rule (same file → *The rc.17 fix pass (M53)*) *uses* the scale — a tier-1 row blocks the 1.0.0 call, tier-2 and tier-3 never do — and is not its home. `inconsistency` carries no tier — the predicate grades defects in the binary, not disagreements in a project.

### 1.5 Lifecycle — append-only by convention (S5)

| Doctype | `open` → | Set by |
|---|---|---|
| `jigc-feedback` | `resolved` (fixed, improved, documented — the how goes in `resolution`) · `declined` (the razor's *we counted and still say no, because…*, M57) · `duplicate` (+ `duplicate-of`) · `refuted` (re-driven, does not reproduce) | the task that fixes it, or `triage-jigc-feedback` |
| `inconsistency` | `resolved` (one side corrected) · `intended` (the difference is deliberate) · `refuted` | the task that reconciles it, or `triage-inconsistency` |

**The convention, stated in each doctype's `usage` and here:** a finding is append-only — **after filing, only `status`, `resolution`, `duplicate-of` and `pinned-by` change** (`inconsistency` carries only the first two); a non-`open` status carries a `resolution`. **The engine does not enforce it.** Doc-level append-only is free (no doc-delete verb), the report workflows only create, and the triage workflows (S17, §2) are the one guided door for the change — their step sets those four and nothing else, and its text says so (R4, I2).

**No `promoted` status (R4, M5).** A finding whose work is accepted onto the roadmap gets no status of its own now; if triage needs one, widening the enum is a cheap later change. What is not free is the field level: any task can `set-field`/`set-slot`/`remove-item` on any committed doc whatever its workflow's `allows-create` (baseline C7, F14). An edit gate would close that for every doctype; it is not cheaper now than later and blocks nothing, so it is parked ([ideas/committed-doc-edit-gate.md](../ideas/committed-doc-edit-gate.md)) and filed as a `jigc-feedback` seed row.

### 1.6 Routing — where a finding goes

- **Until the port, nothing moves.** Findings are recorded as today — review READMEs, the planning register — and agents and `.claude/agents/*` switch to reporting through jigc at M56, when this repository adopts. The two doctypes are not a fourth record home in this repository before then.
- **A `completion-record` finding with `disposition: deferred` that is about jigc** is filed as a `jigc-feedback` row with `found-in: review:<milestone>-completion/<finding-id>` — the cross-reference lives in `found-in`, since `jigc-feedback` has no `evidence` slot.
- **A seeded `decisions-pending.md` row becomes a one-line pointer** to its seed doc; the dated review READMEs are never edited (§7).

### 1.7 Registration

Both schemas are listed in `crates/cli/packs/methodology/config/schema-manifest.yaml` at schema-version 1, with **no snapshot and no migration** (a new doctype has no prior corpus). Registration trips the hand-listed fences the planning census measured ([gap-list.md](../completions/artifacts/M55/gap-list.md) → Important, the registration census) — the manifest set in `pack.rs`, the count fences, the read-surface union, `doctype_map_versions` (a row per doctype in [doctype-map.md](../implementation/doctype-map.md)), the roundtrip snapshot, the schema-load and registry-seam hard counts — plus the unfenced count prose. **The census is 19 reds, not the baseline's 14** (R5; driven at the gate-record on a spike registering the two schemas and four workflows and nothing else — [planning-gate-record.md](../completions/artifacts/M55/planning-gate-record.md) → row 3): 8 doctype-registration fences, 8 workflow-registration fences (the registry seam's workflow count and the six compose-golden sweeps among them), 3 step-text fences — and among the doctype eight, **two the baseline did not name**, both from `inconsistency.sides` being a new item-slot context: `item_slot_ceiling_axis` derives each shipped item context's ceiling, and requires every item-slot doctype to be gated at every reserved depth through a gate-granting `migrate-*` workflow or be named in its unreachable set with a reason, so the sides/`migrate-*` pair is the registering increment's to settle. [doctype-authoring.md](../implementation/doctype-authoring.md)'s checklist names none of them (D9); the registering increment adds them. Both doctypes appear in every adopter's `jigc describe` — doctypes have no hide knob — which S3 accepted.

## 2. The workflows — two to file, two to triage

All four are `creates-task: true` and code-less, and all four end in `step:finalize-doc-only` (§3), so each lands one finding's doc and nothing else.

**Filing.** Both report workflows' create-gate entries carry `new: true` (§4) and allow exactly one create, so a report task files exactly one **new** finding.

| | `report-jigc-feedback` | `report-inconsistency` |
|---|---|---|
| Router | **hidden** — `selectable: false` + `suppressed: {reason, expires: never}`, no `door` key, so callable by name (`jigc start --workflow report-jigc-feedback "<what>"`) — the `record-dogfood` precedent | **visible** — `selectable: true` + a `when:` hint (the `park-idea` precedent) |
| `allows-create` | `{type: jigc-feedback, as: feedback, new: true}` | `{type: inconsistency, as: inconsistency, new: true}` |
| Body | `step:author-jigc-feedback` · `step:author-commit` · `step:finalize-doc-only` | `step:author-inconsistency` · `step:author-commit` · `step:finalize-doc-only` |
| Opens up | when a feedback web service exists (S3) | — |

```yaml
# illustrative — crates/cli/packs/methodology/workflows/record-dogfood.yaml is the mold
selectable: false
suppressed:
  reason: invoked by name — jigc's own agents report what they hit about jigc; opened when a feedback interface exists
  expires: never
```

**Triage (S17).** Two small workflows, `triage-jigc-feedback` and `triage-inconsistency`, move a filed finding off `open`: the intent names the finding, one step sets `status`, `duplicate-of` (for a `duplicate`) and `pinned-by` (for a fix a test pins) and authors `resolution` through `jigc doc set-field`/`set-slot --task`, then `step:author-commit` and `step:finalize-doc-only`. They allow no create. Both are **hidden** on the `report-jigc-feedback` mold — `selectable: false` + `suppressed: {…, expires: never}`, called by name (`jigc start --workflow triage-inconsistency "<intent>"`) — because triage is a maintainer's act, not a catalog entry. **The intent form is named in the triage step (R5):** the task id is minted from the intent, and an address-shaped intent mangles it — driven at the gate-record, `"<type>:<slug>: <why>"` minted `jigc-feedbackfinalize-sweeps-a-teammates`, the `:` dropped and two words fused — so the step tells the agent to name the finding in plain words (its slug, without the `type:` prefix and its colon) and to give the address to the `doc` verbs, never to `jigc start`. They close the declared lifecycle (§1.5): append-only except the four fields it names, and the step text says which four. **The triage steps get distinct names** (R4, M3) — `step:triage` is already the methodology pack's completion-workflow step, so the new ones are named for their doctype (`step:author-jigc-feedback-triage`, `step:author-inconsistency-triage`; illustrative).

**Why hidden, and why that is enough.** `jigc-feedback`'s vocabulary (`tier`, `about`'s finding codes) is jigc-internal, and a "report a bug in jigc" entry in every adopter's router catalog would invite reports into a repository the maintainers never see. The doctype stays in `describe`; the workflow leaves the catalog. A per-doctype visibility knob or a third embedded pack would be new composition plumbing — parked ([ideas/pack-doctype-visibility.md](../ideas/pack-doctype-visibility.md), the 2026-10-02 addendum).

**Why `report-inconsistency` is visible.** An adopter migrating onto jigc is exactly who finds code↔doc disagreements, and parking a finding must cost less than losing it (the `park-idea` argument). It is also the new router-visible workflow the blind trial's justification cites. Cost: the router-visible compose-golden blast — **28 goldens move**, the files that carry a router-visible methodology workflow's catalog line, re-counted at the gate-record (R5); the gap list's *~58* (G4) was relayed, never reproduced.

**The author steps** read the schema through `jigc doc schema <ty>`, set fields and slots with `--task`, read the write back with `doc show --task`, and say to fence `repro`. `step:author-inconsistency` adds each side with **`jigc doc add-item … --slug <slug>`** (R4, M1): a side's title is a path or an address, which slugs into a long, unstable item id, so the step names a short one. Either create door is safe in a report task — `doc create` and the `doc author` batch alike consult the gate entry's `new: true` (§4) — so the draft's reason for withholding the batch alternative is gone.

## 3. `step:finalize-doc-only` — the path-scoped doc-only commit (S4, revised by R1)

**The defect it closes** (F1, driven in the S4 spike over three states): a code-less task's `jigc task finalize` commits **anything staged during the task**. With a code task open in the same checkout and its files staged *after* the report task started, the report's finalize exits 0 and commits the code task's files under `docs(…)`; the code task then fails `finalize.empty-commit`, whose only route is discard. `finalize.carried-staged` catches only paths staged *before* the report started.

**The mechanism — a path-scoped commit.** A new pack step, `step:finalize-doc-only`, composed by the report and triage workflows (§2) in place of `step:finalize`, wiring the same `{{ cli.finalize-task }}` ref. `jigc task finalize` of a task **whose recorded composed workflow includes the step** commits **path-scoped**: it stages exactly the task's promoted docs — the doc it created, or for triage the doc it edited — and its recorded owner-artifacts, and commits those paths alone (`git add <paths> && git commit -- <paths>`), never the index. That is the milestone-record door's mechanism ([team-ready-state.md](team-ready-state.md) → The commit model — path-scoped commit at each milestone op), *"never sweeping the agent's in-flight index"*. **There is no refusal.** Whatever else is staged — by a code task open in the same checkout, before or after the report started — stays staged, and the code task keeps its staging and finalizes its own files later. Pending `.jigc/config` deltas, which an ordinary code-less finalize commits beside the doc (gap G8), are outside the path set by construction and wait for the next ordinary finalize. The finalize transaction around the commit — validate, render, promote, rollback — is the ordinary one.

**Why not the Settle's refusal.** The Settle had the step refuse every foreign staged path with a blocking finding routed `git restore --staged <path>`. Driven at the design review, following that route un-staged the code task's work, which then failed `finalize.nothing-staged`: the refusal moved the harm from the report to the code task. A path-scoped commit leaves nothing to route, so the refusal is withdrawn and its code is never minted.

**What stayed staged is narrated, not refused.** The finalize ack names every staged path the commit left out — the existing **left-out** section ([finalize.md](finalize.md) → Surfaced, not prevented), on this commit model's spelling. On the ordinary model a left-out path is a choice (*"git add to include"*); here it is not, because no `git add` can bring a path into a path-scoped commit, so the guidance says the path stays staged for the task it belongs to. Narration only: no finding, no new code, exit unchanged.

**A third commit model, on the existing axis.** `cli::render::CommitModel` already keys every surface the amend arm shares with the ordinary model ([finalize.md](finalize.md) → *Every surface of the arm is the arm's*). The doc-only commit joins that axis rather than branching surfaces one at a time: the left-out guidance, the pre-commit advisory's stem, the `--dry-run` manifest (it forecasts the path set, not the index) and the index-gate member's spelling in `cli::gate_coverage` (next paragraph).

**The reuse is not free — sized at the gate-record (R5).** The record doors' commit stages and commits **one** path (`commit_record_only` over `git_commit_pathspec` in `crates/cli/src/milestone.rs`), and a doc-only task commits several — the doc plus its owner-artifacts — so the build adds a **multi-path commit helper** beside it (the git seam itself was driven: `git commit -F <msg> -- <a> <b>` committed exactly those two paths while two others stayed staged, with jigc's pre-commit hook installed). And the doc-only commit is a **third case on `CommitModel`**, not a flag beside it: `CommitModel::of` derives the model from the amend sha alone today, so the third case needs a second input — the recorded composed workflow — at each of its call sites, one of them at compose time; `cli::gate_coverage`'s amend spelling is a two-arm match that becomes a three-arm one.

**Beside `finalize.carried-staged` — re-derived.** The carryover gate (`engine::finalize::decide_carryover`) compares the pre-task staged snapshot with the index at finalize and refuses each path staged before the task existed, because the ordinary commit **is** the index and such a path would cross the boundary undeclared. A path-scoped commit never takes a path outside its set, so a path staged before a doc-only task started cannot cross this boundary, declared or not: it is not committed, and the left-out narration names it like any other staged path. So on a doc-only task **the carryover gate does not fire** and **`--carry-staged` is inert in every state** — the amend arm's precedent, where the gate is exempt and the arm's own index gate replaces it. Here the replacement is the path scope itself, so the previewed `carryover` member (the *index gate*, `cli::gate_coverage::GATE_COVERAGE`) gains this arm's spelling, as it gained `AmendSpelling` for the amend arm: one member, previewed by `jigc task validate` and `--dry-run` as the committing door decides it. A task that means to commit code is a code task.

**In a fan-out sub-task the step is never composed** (S2, §6): a sub-task has no per-task commit, and the join commit is the milestone's.

**Where it sits — on existing molds, no new seam:**

| Part | Mold |
|---|---|
| Keyed on the composed workflow's steps | `composes_review_hold` / `MIGRATION_FINALIZE_STEP` in `crates/cli/src/task.rs` |
| The path-scoped commit | the milestone-record doors' `git commit -- <record>` ([team-ready-state.md](team-ready-state.md) → The commit model), widened to several paths by a new multi-path commit helper (R5) |
| The surfaces it shares with the ordinary model | the `cli::render::CommitModel` axis — a third case beside the ordinary model and the amend arm (R5) — and the left-out section it keys |
| The carryover gate on this arm | the amend arm's exemption — gate exempt, `--carry-staged` inert, the index-gate member respelled in `cli::gate_coverage` |

**The name.** Never *record-only*: that term stays with the milestone-record's commit doors and their rollback registry (`cli::rollback::ROLLBACK_POPULATIONS`, count-fenced). This is **the doc-only finalize step** (D1).

**Declared bound — two code tasks in one checkout.** Two open *code* tasks still mis-attribute each other's staged files; the fix is per-task path claims, a new mechanism with no cheaper-now argument. Filed as an open `jigc-feedback` seed row (S13).

## 4. Create-only on the create-gate entry, its refusal, and the `write.title-ignored` route (S7, revised by R2)

**The defect** (F2): `jigc doc create <ty> --title X` where a committed doc already carries X's slug acks `(already existed — copied in for update)` at exit 0, and the next `set-slot` overwrites the earlier finding; `validate` stays clean. Create-or-update is the default and stays so ([write-commands.md](write-commands.md) → The verbs, the title pre-check; → Instance provisioning) — planning's idempotent singleton create depends on it.

**The mechanism — a key on the create-gate entry, not a flag.** A workflow's `allows-create` entry gains one optional key, **`new: true`** — `allows-create: [{ type: jigc-feedback, as: feedback, new: true }]`, and `{ type: inconsistency, as: inconsistency, new: true }`. A create under that entry is **create-only**: when the minted identity already exists, the create-gate refuses it **before copy-in**, so nothing is staged, with a new finding — working name **`create.already-exists`** — routed *"choose a distinct title, or pass `--slug <slug>`."* Both create doors already consult the entry — `jigc doc create`, and `jigc doc author`, which mints from its payload's `title:` ([write-commands.md](write-commands.md) → The create-gate) — so a task whose entry carries `new: true` cannot overwrite an existing doc by either path, and the command refs pass no flag (a `create-<ty>` entry per doctype in `crates/cli/packs/methodology/config/commands.yaml`, on the `create-idea` mold). Both report workflows' entries carry it; other workflows may opt in later. **No `--new` flag is pinned at 1.0.** Cost: one optional key in the workflow front-matter format, which no manifest gates.

**Scope.** *Already exists* means the minted identity's canonical home is a file **on disk** — committed, or an untracked file placed there, which is wider than the committed store and the safe direction — probed independently of anything the task has staged, so a committed doc the task already copied in through another verb is still refused (M55 Increment 2 planning, P3). Re-running the create in the same task over its own fresh staged doc stays idempotent, because that doc is not at its home yet, so a create retried after a crash does not refuse itself. An entry without `new: true` keeps today's create-or-update.

**The key is validated — unknown entry keys are refused at pack-load (R5).** Today the entry's deserializer (`AllowsCreate`, `crates/engine/src/compose.rs`) accepts any key: driven at the gate-record, a project shadow of `park-idea` carrying `{ type: idea, as: idea, new: true, nwe: 7 }` composed, its same-title create acked *copied in*, and `jigc validate` exited 0. A guard that a misspelt key (`nwe: true`) switches off silently is the M49 T0-2 shape (`set: on-creat` loaded clean). So M55 makes the entry strict: an unknown key on an `allows-create` entry is a load error naming the key, at the point where entry membership is decided (`load_workflow_def`, beside the existing uniqueness fence), so every door that loads a workflow refuses it. The same commit fences it. Any project shadow carrying a stray key on an entry fails loudly from then on, which is the intent.

**Precedence — the gate refusal comes first.** The gate refuses before copy-in, ahead of every write check, so inside a task whose entry carries `new: true` neither the overwrite nor the `write.title-ignored` misroute can arise: a report whose *different* title slugs onto an existing id is refused `create.already-exists`, never `write.title-ignored` (flow C).

**The family.** `create.*` is already a minted finding family — `create.gate-blocked`, `create.unknown-doctype` and `create.empty-title` are the doctype-scoped blocks of [command-output-contract.md](command-output-contract.md) → The stable finding key, whose `target` is the bare doctype id, and **`create.serial-collision` is already instance-scoped**: it keys at the colliding instance's address (`instance_collision_finding` in `crates/engine/src/state.rs`). `create.already-exists` joins the family beside that sibling — an identity exists, so its `target` is that doc's `type:slug` address. The code name stays a working name until the build mints it. `new: true` does not change fan-out join suffixing: two sub-tasks that mint one slug in isolation still land `-2` at the join, ordered by task id.

**The route fix, same increment** (F3) — the general case. Under an entry **without** `new: true`, a different title that slugs onto an existing id is refused `write.title-ignored`, whose route tells the reporter to `jigc doc rename` the **existing** doc — someone else's work. The route instead names a distinct title or `--slug`. No new code; severity and exit unchanged. Inside a report task this arm cannot arise (precedence, above); it stays in scope for every other create.

## 5. The read surface (S10)

**The gap** (F18, F10): *"all open feedback, grouped by `found-in`"* costs `doc list` plus one `doc show` per row; `doc show --format json` on a per-instance doc carries no title key, so titles come from markdown; and a hand-deleted defaulted `status` vanishes from `fields`, so a client filtering `status == "open"` misses rows silently.

**Settled — additive keys inside the pre-1.0 window:**

| Surface | Adds |
|---|---|
| `jigc doc show --format json`, whole-doc serve (committed and staged) | top-level **`title`** — the doc's `# H1` |
| `jigc doc list --format json`, each row | **`title`** + **`fields`** — the header fields, in `doc show`'s `fields` shape |
| both | an **absent defaulted field projects its schema default** — the read-side fix of F10; the stored bytes are untouched |

**The row values, in every state (R4, B5)** — declared in [doc-read-surface.md](doc-read-surface.md) beside `item-count`, whose additive-key precedent this follows:

| Row | `title` | `fields` |
|---|---|---|
| `managed`, parses | the `# H1` | the header fields |
| `managed`, does not parse | the `# H1`, or `null` | `null` |
| `unregistered` · `orphaned` | the `# H1`, or `null` | `null` |
| `--task` (the staged copy) | read from the staged copy | read from the staged copy, by the rules above |

`title` is the H1 or `null` on every row; `fields` is present only on a managed row that parses and `null` otherwise — **never `{}`**, which would read as *a doc with no header fields*.

**`fields` reports the effective value (R4, I5).** A projected default is not distinguishable from a stored value on either surface, and that is the statement, not an omission: a reader filtering `status == "open"` wants the value the doctype gives the doc, which is what a defaulted absent field means. The store-side sibling — a `set: on-create` `date` deletable with no conformance finding (F9) — is not addressed here; it is seeded open.

**No version integer moves.** `doc show` and `doc list` carry no in-band version integer and evolve by additive keys pre-1.0 ([doc-read-surface.md](doc-read-surface.md) → Evolution posture) — M49's top-level `schema-version` key on `doc show` landed the same way, with no bump. `contract-version` is `doc schema`'s, whose projection S10 does not touch; S10's *"contract-version bump"* is corrected accordingly.

Filtering stays client-side (`jq`); `doc list --where` is parked ([ideas/doc-list-field-filter.md](../ideas/doc-list-field-filter.md)). Read-surface goldens and [doc-read-surface.md](doc-read-surface.md) are revised in the same increment.

**Engaging the doc-read-surface framing.** That doc declares `doc list` an **index/identity projection, not a content read** — identity, path, registration state, *"never authored prose"* — and M49 refused a `schema-version` key on its rows because the comparison it serves is per-doc and `doc show` answers it. Header fields belong on the row now, for reasons that framing did not have to weigh:

- **They are not authored prose.** Header fields are typed structure (enums, dates, strings); the slots stay `doc show`'s. A row with `status`/`kind`/`found-in` is still an index row — it is the index a triage groups by.
- **The parse is already paid.** M44's `item-count` made `doc list` parse every instance; M49's refusal rested on *a value nothing on that path computes*, and the header fields are exactly what that parse yields.
- **The window closes at the pin.** After 1.0 the same keys cost an explicitly versioned extension; M57's triage would otherwise freeze *list + N shows* into every consumer first.
- **`title` is identity-adjacent.** For an `id-from: title` doctype it is the id-source the slug was minted from.

## 6. The latent defects — three fixes, two bounds (S13)

The port runs on 1.0.0 with a branch per milestone, `origin/main` merged before every PR, and fan-outs. Each fixed row is one the port would hit constantly; M57 comes after the port.

| Row | Defect | Fix |
|---|---|---|
| **L1** (F4) | After a pull or merge changed a committed doc, the next task that edits it is blocked `reconciliation.conflict-block` — `start`'s absorb is in-memory only — and the route says to revert the external edit, i.e. the teammate's commit | in the DRIFTED + TOUCHED arm of the task-gate reconcile (`crates/engine/src/file_state.rs`): **absorb when the on-disk bytes equal the blob at the task's base pin** — the change predates the task, so its copy already carries it. A change made during the task still conflict-blocks. **A pinned edit that does not conform keeps the caller's conflict-block unchanged, never a conformance-block (P1, M55 Increment 4)** — it is never baselined, and a re-grade would retire M46's path-keyed migration exit, whose subject is exactly a break committed before `jigc migrate`; the milestone-record door passes no pin, so F3 holds (P3). ~50–100 lines. Bears on [reconciliation.md](reconciliation.md)'s *"start is a pure reader"* (D10). **And a store-scope arm (R4, B4):** at `jigc validate`, a drifted doc whose on-disk bytes equal its blob at `HEAD` is **advisory**, routed *"the baseline lags `HEAD`; absorbed at the next finalize"* — a pull, not an out-of-band edit. **Its code (R5):** the arm reuses `file-state.hash-matches` at advisory — the shipped no-new-id pattern (`rename_dangling_baseline_finding` reuses `reconciliation.rename` at advisory), so the `(code, target)` key does not move and the severity is the change, declared in [reconciliation.md](reconciliation.md) (§13). **Its population is wider than a pull (R5):** a hand edit committed with plain `git commit` leaves the same bytes at `HEAD` as a pulled one, so matching bytes are not on their own a clean doc — the absorb runs the conformance check first (the UNTOUCHED arm's whole body: conformance gate → record → index absorb, not only the re-hash), and a non-conformant committed edit keeps its conformance finding instead of being baselined. The route is true of both cases |
| **L2** (F5) | after a branch switch, store-scope `jigc validate` exits **1** with a blocking `reconciliation.rename … missing` for a doc that exists only on the other branch, and routes `jigc unmanage` | thread M45 Decision 7's history predicate (`git log HEAD -1 -- <path>` empty → advisory) through the read-only store twin instead of its always-present stub, so store scope agrees with task scope; narrow the `oob-rename` member of `cli::render::STORE_EXIT_FLIPS`, which matches on the code, to the blocking arm. **The route changes at both scopes (R5):** the producer the history predicate selects is the one task scope already uses, `rename_dangling_baseline_finding` (`crates/engine/src/file_state.rs`), whose route today is M45's *"prune the stale baseline: `jigc unmanage <path>`"* — driven at the gate-record on the branch-switch case at task scope. So the shared producer's route is replaced, for task and store scope alike: it names the branch switch and offers switching back, and never offers an index drop (`jigc unmanage`); one producer, one route, one `(code, target)` key reading the same at both scopes. M45's prune-first route is retired where [storage.md](storage.md) and [validation.md](validation.md) state it (§13); the M52 live-record carve-out, a dangling-baseline route that names no verb, is the precedent. The exact text is the L2 increment's build detail |
| **S2** (F7) | a fan-out sub-task composed from a workflow carrying a finalize step is told `Run: jigc task finalize <sub>`, which exits 3 `finalize.milestone-sub-task` — orientation already names the milestone door (`crates/cli/src/render.rs`), the composed `{{ cli.finalize-task }}` ref does not | **the CLI-emitted mold** (R3): when the CLI composes for a fan-out sub-task it **omits the commit-boundary steps** — and emits its existing sub-task trailer in their place (the composed task-state footer, `task_state_lines` in `crates/cli/src/render.rs`: *"this task is a sub-task of milestone `<m>`, whose `jigc milestone finalize <m>` is its only commit boundary"*). **The omission set is derived, not hand-listed (R5):** it is every step whose text carries a finalize command or authors the commit doc — the steps carrying `{{ cli.finalize-task }}` and the wrappers that include one, so today `step:finalize` (both packs), `step:finalize-doc-only`, `step:amend-message`, `step:migration-finalize`'s wrapper text, `step:planning-finalize`, and `step:author-commit` — computed from the composed packs where the set is decided, with a test that keeps it in step with the packs. The settled two-id constant was short: the gate-record's census found the per-task door reaching **28** workflows' composed text through three carriers and two wrappers, against the 14 a `step:finalize` include alone names, and drove `amend`'s `Run:` and the migration wrapper's *"re-run the same finalize with `--approve`"* surviving in a sub-task. So the omission removes every false line, not only `Run:` — the per-task `Run:` line, the wrappers' prose around it, and `step:author-commit`'s *"finalize renders the commit doc"*, which no boundary of a docs-only sub-task reads. **The commit-doc-author clause is keyed on `finalize.fan-out.squash`** (Open question 2, settled at M55 Increment 3): a step whose own body writes the commit doc through a catalog ref is in the set exactly when the knob resolves `true` at compose, where no boundary reads a sub-task's commit doc; under `false` the join renders and gates a code-carrying sub-task's, so the author stays. The clause is not closed under inclusion — `step:sub-task-commit` keeps its never-commit text while its nested author leaves. Two bounds, declared: a **docs-only sub-task under `squash: false`** still composes an author step whose doc the join will not read, because code-carrying is decided at the join, not at compose; and a **knob flipped from `true` to `false` between compose and join** meets the join's existing routed block (`required-slot-present` at `commit:<sub>`, routed at `jigc doc set-slot`), and re-composing through the `Spawn:` line then shows the author step. It is CLI-emitted text after step content, the mold of the `create-gates:` line and the `task minted:` header ([workflow-dialect.md](workflow-dialect.md) → Emitted format); it adds no conditional to the dialect ([command-catalog.md](command-catalog.md) → What the catalog does NOT do stands) and changes no catalog entry. One composition rule, not a sibling workflow per workflow; the seed fan-out (§7) composes report workflows into every sub-task with no finalize step in them. **Not taken:** an engine data value `{{task.commit-door}}` — it fixes only the command line and leaves the rest of the step's text false |

**L2 revises a recorded decision, and engages it.** The store/task severity split was set at [DECISIONS.md](../DECISIONS.md) → 2026-07-24 M45 Increment 7 planning: only the task path had been measured, so the store twin was kept *byte-identical to today* — a scoping-by-evidence choice whose own text called the store follow-up *"a cheap, non-one-way-door change, deferred."* The branch model of 2026-10-01 made a branch switch the ordinary case in this repository and the port's, which is the trigger that deferral lacked. The pinned test `file_state_history_gate::store_scope_stays_blocking_where_task_scope_is_advisory` flips with the decision, in the same commit.

**Declared bounds, seeded open as `jigc-feedback`:** **L3** (F6) — in a user-created git worktree, committed-store reads bind to the main checkout and the history check to the worktree's `HEAD`, so after one report lands every later finalize exits 3; the fix is a per-worktree store root, new mechanism. **Two code tasks in one checkout** (F1-general) — per-task path claims (§3).

## 7. The seed (S11, S12)

**Why a seed and not adoption now (S11).** This repository has no `.jigc/`; running `jigc setup` here before the call would be the port's first act taken early, and L1/L2 would hit every branch switch. No seed would leave the channel empty on the day the port opens it. So M55 **generates the seed through the real binary and commits it as a plain record**; M56 adopts it.

**Generation — its own increment (R4, I3).** The seed (one row per distinct finding, each re-driven; the count of record is the [seed ledger](../completions/artifacts/M55/seed-ledger.md)) is one increment of its own, which drives the M55 build in a rig ([`dev/jigc-rig`](../dev/jigc-rig)) as **a fan-out with several reporters into one store** — one sub-task per row, each `--workflow report-jigc-feedback` or `report-inconsistency`, joined by `jigc milestone finalize` — which is the *two reporters* acceptance configuration (§11). It commits the conformant files under `completions/artifacts/M55/seed/`, one directory per doctype as the doctypes' own homes lay them out — `seed/jigc-feedback/` and `seed/inconsistencies/` — so M56 places the tree under its `docs-root` unchanged.

**The seed fence.** A test copies the committed seed into a fresh rig under its `docs-root`, runs `jigc ingest` and then `jigc validate --format json`, and **passes iff the report carries zero findings** — the real binary adopting the real files, which is M56's first act rehearsed, not a parse-only check. The re-review and blind trial before the call record their rows as today, in their READMEs.

**The set (S12):**

| Source | Rows | → |
|---|---|---|
| the rc.16 wave's tier-2/3 — [M52 per-axis review](../completions/artifacts/M52/per-axis-review/README.md) | 23 | `jigc-feedback` |
| the M53 Settle's six — [decisions-pending.md](../implementation/decisions-pending.md) → *Deferred at the M53 Settle — the 1.x ledger rows* (`implementation/decisions-pending.md:446`), rows (a)–(f) (pointer corrected at the gate-record, R5) | 6 | `jigc-feedback` |
| M53's four re-reviews' tier-2/3 rows sent *"to the ledger for 1.x"* — `completions/artifacts/M53/per-axis-review{,-rc18,-rc19,-rc20}/README.md` | counted in the [seed ledger](../completions/artifacts/M55/seed-ledger.md) | `jigc-feedback` |
| decisions-pending → *Owed after M53's post-review arcs* (the rig's `eval` capture, `--private-target` litter; the blind trial is an owed act, excluded) + the 2026-09-27 CI rows (GPG-signed test commits, undeclared `python3`, the git-marker contract) | 5 | `jigc-feedback` |
| this planning's register F1–F20 + T1–T5 — [planning-findings.md](../completions/artifacts/M55/planning-findings.md) (T5 added at the R5 fold, whose disposition routes it here) | 25 | `jigc-feedback` |
| the declared bound *two code tasks in one checkout* (§6; the bound L3 is F6, a register row already) | 1 | `jigc-feedback` |
| the register's D1–D12 | 12 | `inconsistency` |

**Rules.** Every row is **re-driven against the M55 build before filing** → `open`, `resolved` (with `resolution` and `pinned-by`) or `refuted`. A fixed row is seeded `resolved`, never dropped. `found-in` points at the source (`review:M52-per-axis/(2,DEFECT C)`, `milestone:M55-planning/F4` — free-text labels, §1.3). A seeded `decisions-pending.md` row becomes a one-line pointer; the dated review READMEs are untouched; a seeded register row gets a pointer and is not edited again. The `date` is the filing date by construction (`set: on-create`); the origin's date is reachable through `found-in`. A seeded row's `jigc-version` is the version the row was **first seen** on; the M55 re-drive's result goes into `resolution` (or `description`), never into `jigc-version`.

**At M56.** `jigc setup` → place the seed at the port's `docs-root` → `jigc ingest` — and commit, since `ingest` leaves adopted files untracked (F20) — then file the re-review and blind-trial rows through 1.0.0. That act is keyed to **M56's Settle** as a row of its own ([decisions-pending.md](../implementation/decisions-pending.md) → M56 — the port; R5), because a deferral left only in prose has no trigger. `DECISIONS.md`'s 2026-09-27 *"go to the ledger M55 mints"* gets a dated correction to *"the seed M56 adopts"* at the planning commit.

## 8. The crates.io README (S15)

**The defect.** crates.io rewrites the README's relative links against the package's `path_in_vcs` (`blob/HEAD/crates/cli/<link>`), so `1.0.0-rc.22`'s page breaks four of six link targets ([publish-proof.md](../completions/artifacts/M54/publish-proof.md) → The README as crates.io renders it). This discharges [decisions-pending.md](../implementation/decisions-pending.md) → *Before the next release PR is merged*.

**Settled — generated at commit time:**

- A `dev/` script, [`dev/crate-readme`](../dev/crate-readme), generates a committed `crates/cli/README.md` from the root `README.md`, rewriting each relative link to the workspace `repository` URL + `/blob/HEAD/<root path>` — the base crates.io itself uses, so a link means on crates.io what it means on GitHub.
- `jigc`'s `crates/cli/Cargo.toml` `readme` points at the generated file; `jigc-engine` stays readme-less.
- A fence asserts the crate README is byte-for-byte the transform of the root README; the one-allowed-install-line-copy fence (`crates/cli/tests/install_line.rs`) treats the generated file as a derived artifact, not a second copy.
- After rc.23 publishes, its crates.io page is re-read and **every link must answer 200 at its intended target** — the M54 publish-proof's method.
- Must merge before release PR #2.

**Why not at publish time.** A package-only copy fixes nothing (crates.io's rewrite still applies), and a publish-time rewrite needs `--allow-dirty` and ships bytes no test saw — against [release.md](../implementation/release.md) → Installing, *no release tooling touches a doc*. **No new conflict source:** the generated file changes only when the root README does, and its merge resolution is *resolve the root, regenerate* (a note in [dev-workflow.md](../implementation/dev-workflow.md) → Before a pull request to `main`). Tag-pinned links are parked ([ideas/version-pinned-readme-links.md](../ideas/version-pinned-readme-links.md)).

## 9. The cheap-vs-robust fold (S14)

*"Every fork lists the option that removes the artifact that can drift"* joins the existing **`cheap-vs-robust`** gate, not a new row: in the `planning-record` schema's `cheap-vs-robust` slot `hint:` (`crates/cli/packs/methodology/schemas/planning-record.yaml`) and in the row text at [methodology-docs.md](methodology-docs.md) → The planning gate-record. **Free:** slot hint text is erased from `schema-hash` by the M47 presentation projection, so no schema-version bump and no corpus migration (a new slot would cost both, and a ProseNeeding migration that strands committed records). The order rule — forks only after the gap pass — stays prose in [milestone-planning-workflow.md](../implementation/milestone-planning-workflow.md) → The loop.

## 10. Every new check, scoped

| Check | Door | Fires when | Reads | Workflows | Severity · exit |
|---|---|---|---|---|---|
| the doc-only commit (not a check — a commit model) | `jigc task finalize <id>` and its `--dry-run` | the task is **not** a fan-out sub-task and its recorded composed workflow includes `step:finalize-doc-only` | the task's promoted docs and recorded owner-artifacts | `report-jigc-feedback`, `report-inconsistency`, `triage-jigc-feedback`, `triage-inconsistency`, and any later workflow composing the step | **no finding, no refusal**: the commit is path-scoped, every other staged path stays staged and is narrated in the left-out section; the carryover gate does not fire and `--carry-staged` is inert. Never at `jigc milestone finalize` (a sub-task never composes the step), never at store scope |
| `create.already-exists` (working name, new; joins `create.*`) | `jigc doc create` and `jigc doc author`, at the create-gate, before copy-in | the task's create-gate entry for the doctype carries `new: true` **and** the minted identity already exists on disk at the doctype's location | the doctype's home on disk (committed or untracked) — the task's own fresh staged copy is not *existing*, so a re-run stays idempotent | both report workflows; others opt in on their entry | **blocked write · exit 1**, nothing staged — the write family's exit ([command-output-contract.md](command-output-contract.md) → The exit-code taxonomy); it precedes `write.title-ignored` |
| unknown `allows-create` entry key (R5) | pack-load — every door that loads a workflow definition | an `allows-create` entry carries a key other than the format's three (`type`, `as`, `new`) | the workflow's front-matter | any workflow, shipped or project-layer | **load error** naming the key — a misspelt `new` never loads as an absent one |
| `write.title-ignored` route | `jigc doc create`, `jigc doc author` | unchanged — reachable only under an entry without `new: true` | unchanged | all but the report workflows | unchanged (exit 1); route text only |
| L1 absorb | the task-scope gates — `jigc task validate`, `jigc task finalize`, `jigc milestone finalize` | a committed doc the task touched has drifted from its file-state record **and** its on-disk bytes equal the blob at the task's base pin | committed store vs the file-state record | all | the existing advisory absorb in place of a blocking `reconciliation.conflict-block`; no new code. A pinned edit that fails the conformance gate keeps the caller's `reconciliation.conflict-block` unchanged (P1); the milestone-record door passes no pin and keeps conflict-blocking (P3) |
| L1 store-scope arm | `jigc validate` (store scope) | a recorded doc has drifted from its file-state record **and** its on-disk bytes equal its blob at `HEAD` | the committed store vs the file-state record and `HEAD` | — | **advisory · exit 0** under the existing `file-state.hash-matches` code (R5), routed *"the baseline lags `HEAD`; absorbed at the next finalize"*; a non-conformant committed edit keeps its conformance finding — the absorb runs the conformance check first |
| L2 downgrade | `jigc validate` (store scope) | a recorded doc is missing from the worktree **and** has no history at `HEAD` | the committed store | — | **advisory · exit 0**, routed at the branch switch, never at `unmanage` — the same route at task scope, where the shared producer already grades it advisory (R5); the `oob-rename` exit flip keeps only the blocking arm; the genuine-deletion arm still blocks · exit 1 |
| S2 | `jigc start` / `jigc workflow` / `jigc milestone execute` composing a sub-task | the task is a fan-out sub-task; the commit-doc-author clause also needs `finalize.fan-out.squash` to resolve `true` at compose | the composed packs (the derived omission set) and the knob, read from the layer the join reads | every workflow composing a step in the derived set (§6) | composed text only — the commit-boundary steps omitted, the sub-task trailer in their place; no finding. Bounds (§6): a docs-only sub-task under `squash: false` still authors an unread doc; a knob flipped `true` → `false` between compose and join meets the join's routed `required-slot-present` block at `commit:<sub>` |
| S2 omission-set fence (R5) | `cargo test` | the derived set differs, under either knob value, from the steps in the shipped packs that carry a finalize command (closed under inclusion) or — under `squash: true` — author the commit doc | the shipped packs | — | test red |
| absent-default projection | `jigc doc show` (whole doc), `jigc doc list` | a defaulted header field is absent from the stored doc | committed or staged copy | — | read shape only; `fields` reports the effective value |
| seed fence | `cargo test` | `jigc ingest` then `jigc validate --format json` over the committed seed, copied into a fresh rig, reports any finding | the committed seed | — | test red |
| README fence | `cargo test` | `crates/cli/README.md` ≠ transform(root `README.md`) | both files | — | test red |

## 11. Acceptance flows

Driven through the real binary in throwaway repos ([worked-examples.md](worked-examples.md); the next free flow numbers at planning are 55 onward). Sketches, illustrative:

**A · Report and read back.** In a `[dev ▸ methodology]` repo: `jigc start --workflow report-jigc-feedback "<what>"` → `jigc doc create` (the gate entry carries `new: true`; no flag) → set `kind`, `found-in`, `jigc-version`, `description`, `repro` → finalize lands one commit containing only the doc. `jigc doc list jigc-feedback --format json` shows the row with `title` and `fields.status == "open"`; `jigc doc show jigc-feedback:<slug> --format json` carries `title`. Hand-delete `status:` and commit → both surfaces still project `"open"`. The same arm for `report-inconsistency` with three `sides`, each added with `add-item --slug`, reached from the router catalog; `report-jigc-feedback` absent from it, reachable by name. Then `jigc start --workflow triage-jigc-feedback` on the filed row → `status: resolved` + a `resolution` + a `pinned-by` → finalize lands that doc alone; both triage workflows absent from the catalog.

**B · Mid-code-task reporting — the S4 spike's three states.** A `dev-task` is open with an edit to a tracked file and a new file:

| State | Code task's files | Report finalize, after M55 |
|---|---|---|
| 1 | unstaged throughout | lands the report doc only, exit 0 (as today) |
| 2 | staged **before** the report started | lands the report doc only, exit 0 (today: refused, exit 3, `finalize.carried-staged`) — the carryover gate does not fire on the doc-only commit, the code task's files stay staged and are named in the left-out narration; with `--carry-staged`, the same (inert) |
| 3 | staged **after** the report started | lands the report doc only, exit 0 (today: exit 0, the code task's files swept into the report's commit) — the code task's files stay staged, named in the left-out narration |

In every state the report's commit holds its doc alone, the code task's index is what it was before the report finalized, and the code task then finalizes its own files cleanly. Also asserted: a pending `.jigc/config` delta is not in the report's commit.

**C · Overwrite refused.** A second report with the first's title → `create.already-exists`, exit 1, nothing staged — by `doc create` and by `doc author` alike; with `--slug` it lands beside the first, both readable. A second report whose **different** title slugs onto the first's id → the same gate refusal, `create.already-exists` — never `write.title-ignored`, which cannot arise under a `new: true` entry. The general case, in a workflow whose entry carries no `new: true`: a different title slugging onto an existing id → `write.title-ignored`, routed at a distinct title or `--slug`, never at renaming the existing doc. And a project shadow whose entry misspells the key (`nwe: true`) is refused at pack-load, naming the key (R5).

**D · The seed fan-out — several reporters into one store.** A milestone with ≥2 report sub-tasks (the seed increment runs the full set) → each sub-task's composed text carries no step of the derived omission set — no finalize step, no commit-doc authoring — and ends in the sub-task trailer naming `jigc milestone finalize`, and nowhere names the refused `task finalize` (S2) → the join lands every doc; `jigc doc list` lists them all; the seed fence passes on the committed copy.

**E · Branch and pull (L1, L2).** Pull a teammate's change to a committed doc → store-scope `jigc validate` reports it advisory at exit 0, routed *"the baseline lags `HEAD`"*; then edit it in a new task → finalize lands (absorb), where today it conflict-blocks. Create a doc on a milestone branch, switch to `main` → `jigc validate` exits 0 with an advisory row routed at the branch switch, and a task's gate reports the same path advisory under the same route — neither names `jigc unmanage` (R5); delete a managed doc with history and commit → still blocks.

## 12. Around M55

- **Gate speed (S8)** lands first as the `work/gate-speed` PR, not an increment: real git ahead of the `/usr/bin/git` trampoline (with `SDKROOT`), nextest over the whole suite (+ `cargo test --doc`, a loud fallback to `cargo test`), deps at `opt-level = 2`, with T1–T3 fixed alongside; its record travels with that PR.
- **The release** is rc.23 through release PR #2, carrying M54's audit fixes and M55; the README fix merges first (§8).
- **`implementation/project-history.md`'s fate** stays keyed to M56.

### The partial re-review axes (S16)

Owed on the release candidate carrying M54 and M55, beside the blind trial and before the call — not before the candidate is cut. M54's S18 axes and M55's, merged where they overlap:

| Axis | M54 (S18) | M55 (S16) |
|---|---|---|
| setup · the hook · install · release | ✓ | — |
| probe integrity · measurement / the invocation log | ✓ | — |
| store exit codes / reconciliation | ✓ | L1, L2 and the `unmanage` route |
| pack-load / manifest freeze · migration | ✓ | the two new doctypes at v1, the `cheap-vs-robust` hint |
| finalize / transaction | — | the doc-only step's path-scoped commit, its left-out narration and the carryover gate's exemption |
| write surface | — | the `new: true` create-gate entry at both create doors, the `title-ignored` route |
| pinned read contracts | — | `title` + `fields` keys, the default projection |
| composed surfaces | — | the S2 fix (commit-boundary steps omitted for a sub-task); the two report workflows, hidden and visible (and, since S17, the two hidden triage workflows) |
| adopter docs & help | — | the generated crate README; `report-inconsistency` in the router |

## 13. Docs this milestone revises (R4, B2)

Each row is a rule a shipped doc states that M55 changes; the doc is revised **by the increment that changes the rule**, in the same commit, so no doc states the old rule over the new binary. This design is their pointer until then, never their replacement. This doc itself joined `doc_link_fence`'s live-doc list in the planning commit.

| Doc | The rule it changes | § |
|---|---|---|
| [reconciliation.md](reconciliation.md) | the `DRIFTED` + `TOUCHED` row and its *"block at file level"* paragraph gain L1's absorb when the on-disk bytes equal the base-pin blob; the store scope gains L1's advisory arm when they equal `HEAD` — under the existing `file-state.hash-matches` code at advisory, the severity being the change, and absorbed only through the conformance check (R5); *"start is a pure reader"* (D10) is engaged where it stands | 6 |
| [validation.md](validation.md) | *Exit semantics*: L2 narrows the `oob-rename` exit-flip exception to its blocking arm — a doc with no history at `HEAD` is advisory at store scope, as at task scope; → The M45 registrations (`design/validation.md:621`): the dangling-baseline downgrade's route is no longer *"routed to the existing `jigc unmanage`"* — it names the branch switch and offers switching back, at both scopes (R5, found at the gate-record) | 6 |
| [storage.md](storage.md) | → Derived caches (`design/storage.md:313`): the history-gated downgrade's *"prune route to the existing `jigc unmanage`"* is retired for the branch-switch route, at both scopes (R5, found at the gate-record) | 6 |
| [finalize.md](finalize.md) | the code-less census grows by the four new code-less workflows; the path-scoped doc-only commit joins the commit models beside the ordinary and the amend arm — its left-out guidance, its pre-commit advisory, its `--dry-run` manifest, and its carryover-gate exemption; `cli::gate_coverage`'s index-gate member gains its spelling | 3 |
| [write-commands.md](write-commands.md) | → The create-gate: an `allows-create` entry may carry `new: true`, consulted by `doc create` and `doc author` before copy-in; the `write.title-ignored` route names a distinct title or `--slug` | 4 |
| [workflow-dialect.md](workflow-dialect.md) | → Emitted format: composing for a fan-out sub-task omits the commit-boundary steps and emits the sub-task trailer in their place — an M55 revision on the `create-gates:` line's mold; → On-disk definition format: the `allows-create` entry's optional `new` key, and an unknown entry key refused at pack-load (R5) | 4, 6 |
| [command-output-contract.md](command-output-contract.md) | → The stable finding key: `create.already-exists` (its build name) joins the `create.*` rows at an instance address | 4 |
| [doc-read-surface.md](doc-read-surface.md) | `title` on `doc show`; `title` + `fields` on `doc list` rows with their per-state values, beside `item-count`; the effective-value projection, stated | 5 |
| [methodology-docs.md](methodology-docs.md) | the two doctypes and four workflows join the pack's work-doc surface; → The planning gate-record: the `cheap-vs-robust` row's text (S14) | 1, 2, 9 |
| [pinning.md](../implementation/pinning.md) | §3 stays the one home of the `pinned-by` grammar and names the `jigc-feedback` field as its second consumer | 1.3 |
| [doctype-map.md](../implementation/doctype-map.md) | a row per new doctype (the `doctype_map_versions` fence); → Deliberate outs: why neither is the excluded backlog doctype | 1 |
| [worked-examples.md](worked-examples.md) | flows A–E, numbered from 55 | 11 |

## Parked by this Settle

Each in one file with its trigger, indexed from [VISION.md](../VISION.md) → Open questions: [cardinality-split-merge-transform](../ideas/cardinality-split-merge-transform.md) (S1) · [feedback-web-service](../ideas/feedback-web-service.md) (S2, S9) · [pack-doctype-visibility](../ideas/pack-doctype-visibility.md), the 2026-10-02 addendum — further packs and moving things out of the built-in ones (S3) · [project-authored-doctype-packs](../ideas/project-authored-doctype-packs.md) (S3) · [committed-doc-edit-gate](../ideas/committed-doc-edit-gate.md) (S5) · the code anchor on findings, in [finding-doctype](../ideas/finding-doctype.md)'s head note (S6) · [doc-list-field-filter](../ideas/doc-list-field-filter.md) (S10) · [version-pinned-readme-links](../ideas/version-pinned-readme-links.md) (S15).

## Open questions

Found while writing this design, its review revision and its gate-record revision (R5); none was settled by the design, and an item a build has since settled says so in its own words (questions 1 and 2). The draft's three questions are resolved — `doc list`'s per-state values by R4 (B5, §5), the projected default's indistinguishability by R4 (I5, §5), and finalize's own `.jigc/config` deltas by R1's path scope (§3) — and the rest by S17 and the design-draft resolutions at the end of the [settle log](../completions/artifacts/M55/settle-log.md).

1. **How the doc-only arm's left-out section tags a path left *staged*.** The agent text's guidance is keyed on the commit model (§3), but the JSON `left_out[]` entry's `kind` today describes how a path differs from the commit (`modified`, `added`, `untracked`, …), not that it sits staged for another task. Whether this arm needs a further additive value — the M43 `carried-over` precedent, declared as it ships ([command-output-contract.md](command-output-contract.md)) — or reads correctly under the existing ones is the finalize increment's call, measured against what a driver filters on. **Settled at M55 Increment 1 (T4): it needs one** — the additive value **`left-staged`**, carried by every staged path the doc-only commit leaves out, one entry per path, and declared as it shipped ([command-output-contract.md](command-output-contract.md) → The M55 additive kind; [finalize.md](finalize.md) → Surfaced, not prevented). Under the existing values a staged new file would read as an *included* `added` and a staged edit could not be told from an unstaged one, which is the distinction a driver filters on ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 Increment 1 / T4).
2. **Whether `step:author-commit` leaves a code-carrying sub-task's composed text.** R5 puts every step that authors the commit doc in S2's derived omission set (§6). For a docs-only sub-task that is right — neither squash mode reads its commit doc (driven at the gate-record: `milestone finalize` landed with both report sub-tasks' commit docs unfilled). A **code-carrying** sub-task under `squash: false` is different: the join renders that sub-task's commit doc as its commit message and gates it (`crates/cli/src/milestone.rs`, the `code_carrying_worktrees` subject), so omitting the step that tells the agent to author it would turn a clean join into an unfilled-commit-doc block. The derivation as written cannot tell the two apart at compose time, because whether a sub-task carries code is known at the join, not at compose. Which of these holds — the commit-doc authoring step stays out of the set and only its false *"finalize renders the commit doc"* line is respelled for a sub-task; or it stays in the set and the join stops reading a commit doc no step asked for; or the set is keyed on the milestone's `squash` mode, known at compose — is the S2 increment's to settle, against the join's gate as the test. Recorded here rather than resolved, because R5 named the step and the gate-record's own row 11 showed this case. **Settled at M55 Increment 3: the third** — a step that authors the commit doc is in the set exactly when `finalize.fan-out.squash` resolves `true` at compose, read through the join's own reader from the layer the join reads (the main checkout's project layer, which a sub-task's worktree compose resolves too). Driven against the join: under `true` no boundary reads any sub-task's commit doc, docs-only or code-carrying, so the author leaves; under `false` the join renders and gates every code-carrying sub-task's, so it stays, and *"finalize renders the commit doc"* is true there. The first option keeps a docs-only sub-task authoring an unread doc in the default mode; the second synthesizes the message `squash: false` exists to carry. Its two bounds are §6's ([DECISIONS.md](../DECISIONS.md) → 2026-10-03 M55 Increment 3 planning; → Increment 3 / T2).
