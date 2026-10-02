# The findings channel — `jigc-feedback` · `inconsistency` (M55)

**M55 design of record.** The place a finding goes, shipped before the port that fills it ([roadmap.md](../implementation/roadmap.md) → M55). Two methodology-pack doctypes, one doc per finding, each filed — and later triaged — through a code-less workflow that can land nothing but its own doc; the read surface that makes a populated store triageable; the three latent defects the port would hit on every branch; and the seed that turns a new doctype into a populated ledger on the day the port adopts it. Settled at M55 planning, every fork by the human ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; the working settle log, with ids S1–S17 cited below and the design-draft resolutions at its end, is [completions/artifacts/M55/settle-log.md](../completions/artifacts/M55/settle-log.md); the evidence is [planning-findings.md](../completions/artifacts/M55/planning-findings.md), [baseline-ledger.md](../completions/artifacts/M55/baseline-ledger.md) and [gap-list.md](../completions/artifacts/M55/gap-list.md); the field set is Revision 1 of [doctype-proposal.md](../completions/artifacts/M55/doctype-proposal.md), cross-reviewed in [review-doctypes-codex.md](../completions/artifacts/M55/review-doctypes-codex.md)).

Reads after [team-ready-state.md](team-ready-state.md) (the methodology pack's work-doc surface it extends) and before [measurement.md](measurement.md). Notation is **illustrative** ([what that disclaims](../CLAUDE.md#how-we-work-together)): the YAML, argv and output sketches below are shapes, and the schema files the build ships are their source of truth.

## What M55 settles

| Settle | Decision | § |
|---|---|---|
| S1 | **One doc per finding** — a located, multi-instance doctype; reporters only create | 1 |
| S2 | **Two doctypes by purpose** — `inconsistency` (what a project's own docs and code disagree on; generic, adopter-facing) and `jigc-feedback` (findings *about* jigc; internal for now) | 1 |
| S3 | `jigc-feedback` ships in the methodology pack with its workflow **hidden**; `report-inconsistency` is router-visible | 2 |
| S4 | A new pack step **`step:finalize-doc-only`** + a new blocking finding **`finalize.foreign-staged`** — a report lands its doc and nothing else | 3 |
| Mechanism line | Build new mechanism now when that is cheaper than reworking it or being blocked later | below |
| S5 | **Append-only by convention**, stated; the engine does not enforce it | 1.5 |
| S6 | `pinned-by` is a plain string in [pinning.md](../implementation/pinning.md) §3's grammar | 1.3 |
| S7 | **`jigc doc create --new`** refuses an existing doc; the `write.title-ignored` route is corrected | 4 |
| S8 | Gate speed lands as its own `work/gate-speed` PR before the build — not part of this design | 12 |
| S9 | The field sets — Revision 1 | 1.1–1.2 |
| S10 | Read surface: `title` on `doc show`; `title` + header `fields` on `doc list` rows; an absent defaulted field projects its default | 5 |
| S11 | This repo's feedback before the port: a **seed** generated in M55, adopted at M56 | 7 |
| S12 | The seed set and its rules | 7 |
| S13 | Fix **L1**, **L2**, **S2**; declare **L3** and **two code tasks in one checkout** as bounds | 6 |
| S14 | Fold "every fork lists the option that removes the artifact that can drift" into the `cheap-vs-robust` gate | 9 |
| S15 | A committed, generated crate README with absolute links | 8 |
| S16 | M55's partial re-review axes, merged with M54's | 12 |
| S17 | Status changes after filing go through **two hidden triage workflows**, `triage-jigc-feedback` and `triage-inconsistency` | 2, 1.5 |

## The mechanism line

M53's *"no new mechanism — registry rows and guard conditions only"* was a fix pass's boundary, and M55's charter inherited it as *"no new engine mechanism."* The human revised it at this Settle: **it is not absolute. If introducing a mechanism now is cheaper than rebuilding it later or being blocked by its absence later, introduce it now** — definitely when the cheap cut would need rework or would block. Guards inside existing CLI doors are fine regardless. Every fork below was weighed on *cost now vs (rework + block) later*, and M55 admits these under the line:

| New | Kind | Why now rather than later |
|---|---|---|
| `step:finalize-doc-only` + `finalize.foreign-staged` | pack step + guard in an existing door + a finding code | the port runs report tasks beside code tasks in one checkout; without it, a report silently commits a teammate's code (§3) |
| `doc create --new` + its refusal | CLI flag + write reject | one doc per finding makes a same-title create a silent overwrite of an earlier finding (§4) |
| `title` / `fields` on the read surfaces | additive read keys | free inside the pre-1.0 window, a versioned extension after the pin (§5) |
| L1 / L2 / S2 | conditions on existing arms, a composed-text fix | the port's branch-per-milestone model hits all three constantly (§6) |
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
| `jigc-version` | string, `<semver>[+<short-sha>]` | yes | the jigc version it was seen on. Not `version`: that collides with the injected `schema-version` in `doc show` json |
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
| `about` | `jigc <verb>` · `workflow:<id>` · `step:<id>` · `doctype:<id>` · `<finding-code>` · `guide:<file>` | `jigc task finalize` · `finalize.foreign-staged` · `guide:QUICKSTART.md` |
| `jigc-version` | `<semver>[+<short-sha>]` | `1.0.0-rc.23` · `1.0.0-rc.23+0a1b2c3d` |
| `pinned-by` | `<module>::<test_name>` or `UNPINNED: <why>` — [pinning.md](../implementation/pinning.md) §3 is its one home; a repro block points at the field rather than repeating it | `doc_read_surface::bare_single_hop_alias_resolves` |

### 1.4 The tier scale

`tier` is the M52-onward review grade; its meaning is **not restated here**. The scale's one home is the three-tier definition in the rc.16 charter — [decisions-pending.md](../implementation/decisions-pending.md) → *The rc.16 wave (M52)*, its three tier headings (*exit-0 loss or repository harm through a committing or destroying door* · *posture and route dead ends, code-less or undeclared refusals* · *surfaces that say something the binary does not do*). The charter numbers them 0 · 1 · 2; the review grade, from [the M52 per-axis review](../completions/artifacts/M52/per-axis-review/README.md) on, numbers the same predicate `tier-1` · `tier-2` · `tier-3`, and the enum takes the review's. The exit rule (same file → *The rc.17 fix pass (M53)*) *uses* the scale — a tier-1 row blocks the 1.0.0 call, tier-2 and tier-3 never do — and is not its home. `inconsistency` carries no tier — the predicate grades defects in the binary, not disagreements in a project.

### 1.5 Lifecycle — append-only by convention (S5)

| Doctype | `open` → | Set by |
|---|---|---|
| `jigc-feedback` | `resolved` (fixed, improved, documented — the how goes in `resolution`) · `declined` (the razor's *we counted and still say no, because…*, M57) · `duplicate` (+ `duplicate-of`) · `refuted` (re-driven, does not reproduce) | the task that fixes it, or `triage-jigc-feedback` |
| `inconsistency` | `resolved` (one side corrected) · `intended` (the difference is deliberate) · `refuted` | the task that reconciles it, or `triage-inconsistency` |

**The convention, stated in each doctype's `usage` and here:** a finding is append-only — **after filing, only `status`, `resolution`, `duplicate-of` and `pinned-by` change**; a non-`open` status carries a `resolution`. **The engine does not enforce it.** Doc-level append-only is free (no doc-delete verb), the report workflows only create, and the triage workflows (S17, §2) are the one guided door for the status change — they set `status` and author `resolution`, nothing else. What is not free is the field level: any task can `set-field`/`set-slot`/`remove-item` on any committed doc whatever its workflow's `allows-create` (baseline C7, F14). An edit gate would close that for every doctype; it is not cheaper now than later and blocks nothing, so it is parked ([ideas/committed-doc-edit-gate.md](../ideas/committed-doc-edit-gate.md)) and filed as a `jigc-feedback` seed row.

### 1.6 Routing — where a finding goes

- **Until the port, nothing moves.** Findings are recorded as today — review READMEs, the planning register — and agents and `.claude/agents/*` switch to reporting through jigc at M56, when this repository adopts. The two doctypes are not a fourth record home in this repository before then.
- **A `completion-record` finding with `disposition: deferred` that is about jigc** is filed as a `jigc-feedback` row with `found-in: review:<milestone>-completion/<finding-id>` — the cross-reference lives in `found-in`, since `jigc-feedback` has no `evidence` slot.
- **A seeded `decisions-pending.md` row becomes a one-line pointer** to its seed doc; the dated review READMEs are never edited (§7).

### 1.7 Registration

Both schemas are listed in `crates/cli/packs/methodology/config/schema-manifest.yaml` at schema-version 1, with **no snapshot and no migration** (a new doctype has no prior corpus). Registration trips the hand-listed fences the planning census measured ([gap-list.md](../completions/artifacts/M55/gap-list.md) → Important, the registration census) — the manifest set in `pack.rs`, the count fences, the read-surface union, `doctype_map_versions` (a row per doctype in [doctype-map.md](../implementation/doctype-map.md)), the roundtrip snapshot, the schema-load and registry-seam hard counts — plus the unfenced count prose. [doctype-authoring.md](../implementation/doctype-authoring.md)'s checklist names none of them (D9); the registering increment adds them. Both doctypes appear in every adopter's `jigc describe` — doctypes have no hide knob — which S3 accepted.

## 2. The workflows — two to file, two to triage

All four are `creates-task: true` and code-less, and all four end in `step:finalize-doc-only` (§3), so each lands one finding's doc and nothing else.

**Filing.** Both report workflows create with `--new` (§4) and allow exactly one create, so a report task files exactly one finding.

| | `report-jigc-feedback` | `report-inconsistency` |
|---|---|---|
| Router | **hidden** — `selectable: false` + `suppressed: {reason, expires: never}`, no `door` key, so callable by name (`jigc start --workflow report-jigc-feedback "<what>"`) — the `record-dogfood` precedent | **visible** — `selectable: true` + a `when:` hint (the `park-idea` precedent) |
| `allows-create` | `{type: jigc-feedback, as: feedback}` | `{type: inconsistency, as: inconsistency}` |
| Body | `step:author-jigc-feedback` · `step:author-commit` · `step:finalize-doc-only` | `step:author-inconsistency` · `step:author-commit` · `step:finalize-doc-only` |
| Opens up | when a feedback web service exists (S3) | — |

```yaml
# illustrative — crates/cli/packs/methodology/workflows/record-dogfood.yaml is the mold
selectable: false
suppressed:
  reason: invoked by name — jigc's own agents report what they hit about jigc; opened when a feedback interface exists
  expires: never
```

**Triage (S17).** Two small workflows, `triage-jigc-feedback` and `triage-inconsistency`, move a filed finding off `open`: the intent names the finding's address, one step sets `status` (and `duplicate-of` for a `duplicate`) and authors `resolution` through `jigc doc set-field`/`set-slot --task`, then `step:author-commit` and `step:finalize-doc-only`. They allow no create. Both are **hidden** on the `report-jigc-feedback` mold — `selectable: false` + `suppressed: {…, expires: never}`, called by name (`jigc start --workflow triage-inconsistency "<address>: <why>"`) — because triage is a maintainer's act, not a catalog entry. They close the declared lifecycle (§1.5): append-only except `status` and `resolution`. The step names are illustrative.

**Why hidden, and why that is enough.** `jigc-feedback`'s vocabulary (`tier`, `about`'s finding codes) is jigc-internal, and a "report a bug in jigc" entry in every adopter's router catalog would invite reports into a repository the maintainers never see. The doctype stays in `describe`; the workflow leaves the catalog. A per-doctype visibility knob or a third embedded pack would be new composition plumbing — parked ([ideas/pack-doctype-visibility.md](../ideas/pack-doctype-visibility.md), the 2026-10-02 addendum).

**Why `report-inconsistency` is visible.** An adopter migrating onto jigc is exactly who finds code↔doc disagreements, and parking a finding must cost less than losing it (the `park-idea` argument). It is also the new router-visible workflow the blind trial's justification cites. Cost: the router-visible compose-golden blast (~58 goldens, G4).

**The author steps** read the schema through `jigc doc schema <ty>`, set fields and slots with `--task`, read the write back with `doc show --task`, and say to fence `repro`. They **do not offer the `doc author` batch alternative**: `--new` is `doc create`'s alone (§4), so the batch door would re-open the overwrite §4 closes.

## 3. `step:finalize-doc-only` and `finalize.foreign-staged` (S4)

**The defect it closes** (F1, driven in the S4 spike over three states): a code-less task's `jigc task finalize` commits **anything staged during the task**. With a code task open in the same checkout and its files staged *after* the report task started, the report's finalize exits 0 and commits the code task's files under `docs(…)`; the code task then fails `finalize.empty-commit`, whose only route is discard. `finalize.carried-staged` catches only paths staged *before* the report started.

**The mechanism.** A new pack step, `step:finalize-doc-only`, composed by the report and triage workflows (§2) in place of `step:finalize`. Its text states the constraint (`states-constraints: [finalize.foreign-staged, …]`) and wires the same `{{ cli.finalize-task }}` ref. `jigc task finalize` of a task **whose recorded composed workflow includes the step** refuses every staged path other than the doc the task wrote through jigc (created or, for triage, edited) and its recorded owner-artifacts — one blocking `finalize.foreign-staged` per path, routed `git restore --staged <path>`. **There is no waiver** — not `--carry-staged`, not a new flag — on the `--amend` arm's precedent, whose index-dirty gate has none either. A report task cannot carry code; a task that means to commit code is a code task.

**Beside `finalize.carried-staged`.** Both codes stay. `carried-staged` catches paths staged *before* the task started, `foreign-staged` any staged path outside the allowed set, so a path staged before a doc-only task started fires both; in a doc-only task `--carry-staged` waives the former and never the latter, so the refusal stands until the path is unstaged (flow B, state 2).

**Where it sits — on existing molds, no new seam:**

| Part | Mold |
|---|---|
| Keyed on the composed workflow's steps | `composes_review_hold` / `MIGRATION_FINALIZE_STEP` in `crates/cli/src/task.rs` |
| Body: staged set vs `HEAD`, one finding per path | `amend_index_findings` in `crates/cli/src/task.rs` |
| Asked at both positions — the committing door ahead of `plan_finalize`, and `preview_gates` | the same method at both, so *same check, same severity, same exit* holds (`crate::gate_coverage::Door::Previewed`) |
| Registry rows | the finalize family (`cli::render::FINALIZE_FAMILY`, fenced by `finalize_family_registry`) · the `gate_coverage` door row (previewed) · the contract's code registry in [command-output-contract.md](command-output-contract.md) |

**The name.** Never *record-only*: that term stays with the milestone-record's commit doors and their rollback registry (`cli::rollback::ROLLBACK_POPULATIONS`, count-fenced). This is **the doc-only finalize step** (D1).

**Declared bound — two code tasks in one checkout.** Two open *code* tasks still mis-attribute each other's staged files; the fix is per-task path claims, a new mechanism with no cheaper-now argument. Filed as an open `jigc-feedback` seed row (S13).

## 4. `doc create --new`, its refusal, and the `write.title-ignored` route (S7)

**The defect** (F2): `jigc doc create <ty> --title X` where a committed doc already carries X's slug acks `(already existed — copied in for update)` at exit 0, and the next `set-slot` overwrites the earlier finding; `validate` stays clean. Create-or-update is the default and stays so ([write-commands.md](write-commands.md) → The verbs, the title pre-check; → Instance provisioning) — planning's idempotent singleton create depends on it.

**Proposal — no code exists for either name yet.** A flag `--new` on `jigc doc create` makes the create **create-only**: when the minted identity already exists, the call is refused before anything is staged, with a new finding — working name **`create.already-exists`** — routed *"choose a distinct title, or pass `--slug <slug>`."* Both report workflows' command refs always pass it (`crates/cli/packs/methodology/config/commands.yaml`, a `create-<ty>` entry per doctype on the `create-idea` mold). Other per-doc creates (`park-idea`, `do-research`, …) may opt in later.

**Scope.** `--new` is a `doc create` flag only — `doc author`, which mints from its payload's `title:`, does not take it (§2). *Already exists* means the committed store: re-running the create in the same task over its own staged doc stays idempotent, so a create retried after a crash does not refuse itself.

**The family.** `create.*` is already a minted finding family — `create.gate-blocked`, `create.unknown-doctype`, `create.empty-title`, the doctype-scoped blocks of [command-output-contract.md](command-output-contract.md) → The stable finding key, whose `target` is the bare doctype id. `create.already-exists` joins it as its first **instance**-scoped member: an identity exists, so its `target` is that doc's `type:slug` address. The code name stays a working name until the build mints it. `--new` does not change fan-out join suffixing: two sub-tasks that mint one slug in isolation still land `-2` at the join, ordered by task id.

**The route fix, same increment** (F3): a different title that slugs onto an existing id is refused `write.title-ignored`, whose route tells the reporter to `jigc doc rename` the **existing** doc — someone else's finding. The route instead names a distinct title or `--slug`. No new code; severity and exit unchanged.

## 5. The read surface (S10)

**The gap** (F18, F10): *"all open feedback, grouped by `found-in`"* costs `doc list` plus one `doc show` per row; `doc show --format json` on a per-instance doc carries no title key, so titles come from markdown; and a hand-deleted defaulted `status` vanishes from `fields`, so a client filtering `status == "open"` misses rows silently.

**Settled — additive keys inside the pre-1.0 window:**

| Surface | Adds |
|---|---|
| `jigc doc show --format json`, whole-doc serve (committed and staged) | top-level **`title`** — the doc's `# H1` |
| `jigc doc list --format json`, each row | **`title`** + **`fields`** — the header fields, in `doc show`'s `fields` shape |
| both | an **absent defaulted field projects its schema default** — the read-side fix of F10; the stored bytes are untouched |

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
| **L1** (F4) | After a pull or merge changed a committed doc, the next task that edits it is blocked `reconciliation.conflict-block` — `start`'s absorb is in-memory only — and the route says to revert the external edit, i.e. the teammate's commit | in the DRIFTED + TOUCHED arm of the task-gate reconcile (`crates/engine/src/file_state.rs`): **absorb when the on-disk bytes equal the blob at the task's base pin** — the change predates the task, so its copy already carries it. A change made during the task still conflict-blocks. ~50–100 lines. Bears on [reconciliation.md](reconciliation.md)'s *"start is a pure reader"* (D10) |
| **L2** (F5) | after a branch switch, store-scope `jigc validate` exits **1** with a blocking `reconciliation.rename … missing` for a doc that exists only on the other branch, and routes `jigc unmanage` | thread M45 Decision 7's history predicate (`git log HEAD -1 -- <path>` empty → advisory) through the read-only store twin instead of its always-present stub, so store scope agrees with task scope; narrow the `oob-rename` member of `cli::render::STORE_EXIT_FLIPS`, which matches on the code, to the blocking arm. **The route** names the branch switch and offers switching back; it never offers an index drop (`jigc unmanage`) — its exact text is the L2 increment's build detail |
| **S2** (F7) | a fan-out sub-task composed from a workflow carrying a finalize step is told `Run: jigc task finalize <sub>`, which exits 3 `finalize.milestone-sub-task` — orientation already names the milestone door (`crates/cli/src/render.rs`), the composed `{{ cli.finalize-task }}` ref does not | a **sub-task-aware command ref**: in a fan-out sub-task, the composed finalize step — `step:finalize` and `step:finalize-doc-only` alike — says *stop; the orchestrator joins* (`jigc milestone finalize`) instead of `jigc task finalize <sub>`. One ref fix, not a sibling workflow per workflow; the seed fan-out (§7) composes `step:finalize-doc-only` into every sub-task |

**L2 revises a recorded decision, and engages it.** The store/task severity split was set at [DECISIONS.md](../DECISIONS.md) → 2026-07-24 M45 Increment 7 planning: only the task path had been measured, so the store twin was kept *byte-identical to today* — a scoping-by-evidence choice whose own text called the store follow-up *"a cheap, non-one-way-door change, deferred."* The branch model of 2026-10-01 made a branch switch the ordinary case in this repository and the port's, which is the trigger that deferral lacked. The pinned test `file_state_history_gate::store_scope_stays_blocking_where_task_scope_is_advisory` flips with the decision, in the same commit.

**Declared bounds, seeded open as `jigc-feedback`:** **L3** (F6) — in a user-created git worktree, committed-store reads bind to the main checkout and the history check to the worktree's `HEAD`, so after one report lands every later finalize exits 3; the fix is a per-worktree store root, new mechanism. **Two code tasks in one checkout** (F1-general) — per-task path claims (§3).

## 7. The seed (S11, S12)

**Why a seed and not adoption now (S11).** This repository has no `.jigc/`; running `jigc setup` here before the call would be the port's first act taken early, and L1/L2 would hit every branch switch. No seed would leave the channel empty on the day the port opens it. So M55 **generates the seed through the real binary and commits it as a plain record**; M56 adopts it.

**Generation.** One increment drives the M55 build in a rig ([`dev/jigc-rig`](../dev/jigc-rig)) as **a fan-out with several reporters into one store** — one sub-task per row, each `--workflow report-jigc-feedback` or `report-inconsistency`, joined by `jigc milestone finalize` — which is the *two reporters* acceptance configuration (§11). It commits the conformant files under `completions/artifacts/M55/seed/`, fenced by a test that every file parses and validates against the shipped schemas at their current schema-version. The re-review and blind trial before the call record their rows as today, in their READMEs.

**The set (S12):**

| Source | Rows | → |
|---|---|---|
| the rc.16 wave's tier-2/3 — [M52 per-axis review](../completions/artifacts/M52/per-axis-review/README.md) | 23 | `jigc-feedback` |
| the M53 Settle's six — [decisions-pending.md](../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)*, rows (a)–(f) | 6 | `jigc-feedback` |
| M53's four re-reviews' tier-2/3 rows sent *"to the ledger for 1.x"* — `completions/artifacts/M53/per-axis-review{,-rc18,-rc19,-rc20}/README.md` | counted by the seed increment | `jigc-feedback` |
| decisions-pending → *Owed after M53's post-review arcs* (the rig's `eval` capture, `--private-target` litter; the blind trial is an owed act, excluded) + the 2026-09-27 CI rows (GPG-signed test commits, undeclared `python3`, the git-marker contract) | 5 | `jigc-feedback` |
| this planning's register F1–F20 + T1–T4 — [planning-findings.md](../completions/artifacts/M55/planning-findings.md) | 24 | `jigc-feedback` |
| the register's D1–D12 | 12 | `inconsistency` |

**Rules.** Every row is **re-driven against the M55 build before filing** → `open`, `resolved` (with `resolution` and `pinned-by`) or `refuted`. A fixed row is seeded `resolved`, never dropped. `found-in` points at the source (`review:M52-per-axis/(2,DEFECT C)`, `milestone:M55-planning/F4` — free-text labels, §1.3). A seeded `decisions-pending.md` row becomes a one-line pointer; the dated review READMEs are untouched; a seeded register row gets a pointer and is not edited again. The `date` is the filing date by construction (`set: on-create`); the origin's date is reachable through `found-in`. A seeded row's `jigc-version` is the version the row was **first seen** on; the M55 re-drive's result goes into `resolution` (or `description`), never into `jigc-version`.

**At M56.** `jigc setup` → place the seed at the port's `docs-root` → `jigc ingest` — and commit, since `ingest` leaves adopted files untracked (F20) — then file the re-review and blind-trial rows through 1.0.0. `DECISIONS.md`'s 2026-09-27 *"go to the ledger M55 mints"* gets a dated correction to *"the seed M56 adopts"* at the planning commit.

## 8. The crates.io README (S15)

**The defect.** crates.io rewrites the README's relative links against the package's `path_in_vcs` (`blob/HEAD/crates/cli/<link>`), so `1.0.0-rc.22`'s page breaks four of six link targets ([publish-proof.md](../completions/artifacts/M54/publish-proof.md) → The README as crates.io renders it). This discharges [decisions-pending.md](../implementation/decisions-pending.md) → *Before the next release PR is merged*.

**Settled — generated at commit time:**

- A `dev/` script generates a committed `crates/cli/README.md` from the root `README.md`, rewriting each relative link to the workspace `repository` URL + `/blob/HEAD/<root path>` — the base crates.io itself uses, so a link means on crates.io what it means on GitHub.
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
| `finalize.foreign-staged` (new) | `jigc task finalize <id>` and its `--dry-run`; previewed by `jigc task validate <id>` | the task is **not** a fan-out sub-task, its recorded composed workflow includes `step:finalize-doc-only`, and a path is staged that is neither the doc the task wrote through jigc (its promote destination — created, or edited by triage) nor a recorded owner-artifact | the git index vs `HEAD` | `report-jigc-feedback`, `report-inconsistency`, `triage-jigc-feedback`, `triage-inconsistency`, and any later workflow composing the step | **blocking · exit 3**, one per path; no waiver — `--carry-staged` waives only `finalize.carried-staged`, which still fires beside it for a path staged before the task. Never at `jigc milestone finalize` (sub-task code is worktree-isolated and the join commit is the milestone's), never at store scope |
| `create.already-exists` (working name, new; joins `create.*`) | `jigc doc create … --new` only | the minted identity already exists in the committed store | the committed store — the task's own staged copy is not *existing*, so a re-run stays idempotent | both report workflows always; others opt in | **blocked write · exit 1**, nothing staged — the write family's exit ([command-output-contract.md](command-output-contract.md) → The exit-code taxonomy) |
| `write.title-ignored` route | `jigc doc create`, `jigc doc author` | unchanged | unchanged | all | unchanged (exit 1); route text only |
| L1 absorb | the task-scope gates — `jigc task validate`, `jigc task finalize`, `jigc milestone finalize` | a committed doc the task touched has drifted from its file-state record **and** its on-disk bytes equal the blob at the task's base pin | committed store vs the file-state record | all | the existing advisory absorb in place of a blocking `reconciliation.conflict-block`; no new code |
| L2 downgrade | `jigc validate` (store scope) | a recorded doc is missing from the worktree **and** has no history at `HEAD` | the committed store | — | **advisory · exit 0**, routed at the branch switch, never at `unmanage`; the `oob-rename` exit flip keeps only the blocking arm; the genuine-deletion arm still blocks · exit 1 |
| S2 | `jigc start` / `jigc workflow` / `jigc milestone execute` composing a sub-task | the task is a fan-out sub-task | — | every workflow composing `step:finalize` or `step:finalize-doc-only` | composed text only (the sub-task-aware ref); no finding |
| absent-default projection | `jigc doc show` (whole doc), `jigc doc list` | a defaulted header field is absent from the stored doc | committed or staged copy | — | read shape only |
| seed fence | `cargo test` | a file under `completions/artifacts/M55/seed/` fails to parse or validate against the shipped schema | the committed seed | — | test red |
| README fence | `cargo test` | `crates/cli/README.md` ≠ transform(root `README.md`) | both files | — | test red |

## 11. Acceptance flows

Driven through the real binary in throwaway repos ([worked-examples.md](worked-examples.md); the next free flow numbers at planning are 55 onward). Sketches, illustrative:

**A · Report and read back.** In a `[dev ▸ methodology]` repo: `jigc start --workflow report-jigc-feedback "<what>"` → create with `--new` → set `kind`, `found-in`, `jigc-version`, `description`, `repro` → finalize lands one commit containing only the doc. `jigc doc list jigc-feedback --format json` shows the row with `title` and `fields.status == "open"`; `jigc doc show jigc-feedback:<slug> --format json` carries `title`. Hand-delete `status:` and commit → both surfaces still project `"open"`. The same arm for `report-inconsistency` with three `sides`, reached from the router catalog; `report-jigc-feedback` absent from it, reachable by name. Then `jigc start --workflow triage-jigc-feedback` on the filed row → `status: resolved` + a `resolution` → finalize lands that doc alone; both triage workflows absent from the catalog.

**B · Mid-code-task reporting — the S4 spike's three states.** A `dev-task` is open with an edit to a tracked file and a new file:

| State | Code task's files | Report finalize, after M55 |
|---|---|---|
| 1 | unstaged throughout | lands the report doc only, exit 0 (as today) |
| 2 | staged **before** the report started | refused, exit 3 — `finalize.carried-staged` as today **and** `finalize.foreign-staged`; `--carry-staged` does not waive the latter |
| 3 | staged **after** the report started | refused, exit 3, `finalize.foreign-staged` per path (today: exit 0, swept); after `git restore --staged`, the report lands its doc alone and the code task then finalizes its own files cleanly |

**C · Overwrite refused.** A second report with the first's title → `create.already-exists`, exit 1, nothing staged; with `--slug` it lands beside the first, both readable. A different title slugging onto the first's id → `write.title-ignored` routed at a distinct title or `--slug`, never at renaming the existing doc.

**D · The seed fan-out — several reporters into one store.** A milestone with ≥2 report sub-tasks (the seed increment runs the full set) → each sub-task's composed text names `jigc milestone finalize`, never the refused `task finalize` (S2) → the join lands every doc; `jigc doc list` lists them all; the seed fence passes on the committed copy.

**E · Branch and pull (L1, L2).** Pull a teammate's change to a committed doc, then edit it in a new task → finalize lands (absorb), where today it conflict-blocks. Create a doc on a milestone branch, switch to `main` → `jigc validate` exits 0 with an advisory row; delete a managed doc with history and commit → still blocks.

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
| finalize / transaction | — | the doc-only step, `finalize.foreign-staged` |
| write surface | — | `create --new`, the `title-ignored` route |
| pinned read contracts | — | `title` + `fields` keys, the default projection |
| composed surfaces | — | the S2 fix; the two report workflows, hidden and visible (and, since S17, the two hidden triage workflows) |
| adopter docs & help | — | the generated crate README; `report-inconsistency` in the router |

## Parked by this Settle

Each in one file with its trigger, indexed from [VISION.md](../VISION.md) → Open questions: [cardinality-split-merge-transform](../ideas/cardinality-split-merge-transform.md) (S1) · [feedback-web-service](../ideas/feedback-web-service.md) (S2, S9) · [pack-doctype-visibility](../ideas/pack-doctype-visibility.md), the 2026-10-02 addendum — further packs and moving things out of the built-in ones (S3) · [project-authored-doctype-packs](../ideas/project-authored-doctype-packs.md) (S3) · [committed-doc-edit-gate](../ideas/committed-doc-edit-gate.md) (S5) · the code anchor on findings, in [finding-doctype](../ideas/finding-doctype.md)'s head note (S6) · [doc-list-field-filter](../ideas/doc-list-field-filter.md) (S10) · [version-pinned-readme-links](../ideas/version-pinned-readme-links.md) (S15).

## Open questions

Found while writing this design; none is settled here. The rest of the draft's questions were resolved by S17 and the design-draft resolutions at the end of the [settle log](../completions/artifacts/M55/settle-log.md).

1. **What do `title` and `fields` carry on an `unregistered` or `orphaned` `doc list` row**, and on a `--task` row? An instance that does not parse counts `item-count` 0 today; `null`, absent or best-effort is the same kind of call.
2. **Is the projected default distinguishable from a stored value?** S10 projects silently; the cross-review suggested flagging a missing stored field. F9 (a `set: on-create` date deletable with no conformance finding) is a store-side sibling neither addresses.
3. **What finalize adds itself.** Code-less finalize also commits unstaged `.jigc/config` deltas (gap G8); whether those paths sit inside `finalize.foreign-staged`'s allowed set is unsettled.
