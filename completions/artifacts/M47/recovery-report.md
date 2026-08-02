# M47 — the rc.10 wave (the surface-fundament wave): recovery report

**Reconstructed 2026-08-02 from the mechanically extracted event export of the lost driving session**
(`scratchpad/recovery/m47/`: `narrative.md` 6 265 lines · `bash-ledger.md` · `agents-ledger.md` · `asks.md` ·
`writes-journal.txt` · `patches/` (12 Edit old→new pairs) · `files/` (4 full Write bodies)).
Everything below is sourced from that extraction; where a claim is *inferred* rather than quoted it says so.

**The lost arc, in one line.** Session ran 2026-07-26 03:37 → 2026-07-27 (local clock in the export runs to
19:21): milestone-planning of M47 (Scope → 4 gap probes → 14-fork Settle → in-house pre-decompose review →
cross-model Codex review → gate-record → eleven increments), committed as `443921c` → `b28d31f` → `504d43b`;
then a full build (76 commits `504d43b` → `7e300e4`, `1.0.0-rc.10` bumped, flow 47 landed), one mid-build halt
+ re-Settle (`931d7ef`), a clean-ish completion audit (4 LOW + 4 e2e defects) and **twelve** post-audit fix
commits ending at `b415c67`, with a thirteenth fixer dispatched and never returning — the crash. All of it was
unpushed and is gone from git.

**What survives byte-exact** (in `recovery/m47/`): `completions/artifacts/M47/baseline.md` (23 296 B) ·
`completions/artifacts/M47/planning-gate-record.md` (17 494 B) · the four big `DECISIONS.md` entries (the
Settle, the pre-decompose review, the cross-model review, the Inc-3 halt resolution) as patch NEW-text ·
two `decisions-pending.md` blocks (the graduated M47 charter + entries 10/11 + the two M46 deferrals) ·
six small `roadmap.md` edit pairs. **What is lost:** all product code and tests; the eleven-increment
decomposition body in `implementation/roadmap.md` (only ~6 fragments survive); the four per-increment
`design(m47): increment N task decomposition` commits; every per-increment `DECISIONS.md` build entry; flow 47;
the completion-audit artifacts.

---

# 1. The M47 charter as settled

## 1.1 Identity, why-now, boundary

From the Scope restatement (narrative :3822–3848) and the charter block in `decisions-pending.md`:

> **M47 · the rc.10 wave — the surface-fundament wave: make every shipped surface true, then trial it blind on
> greenfield.** *Why now.* The project-alpha-4.0 unseeded trial found zero data-loss defects and zero regressions,
> but its confirmed set is *incomplete-sweep siblings on prose/route/ack axes* — the tool lying about itself.
> This is the last wave before the 1.0.0 call, so anything contract-pinned or post-1.0-breaking ships now.
> **Executes before M46.**

**What it proves** (baseline.md §1, verbatim): *"that jigc's own surfaces tell the truth about jigc — a blind
agent driving the tool from a cold start is carried by what the tool says, not by what a human already knows."*

**The boundary, checkable** (verbatim): *"the wave changes what existing surfaces say, never what surfaces
exist"* — anything needing a new verb/flag/projection routes to M46. Recorded consequence: H1's arm (1) is a
*behaviour* change, not a wording change; it adds no surface so it is inside the boundary as written, but it is
"the one tier-1 item where the razor is load-bearing rather than obvious."

**Three deliberate bends of that boundary were recorded** (this matters for any rebuild): (1) the
greenfield-path five (N6–N10), ruled in by the human because leaving them means the acceptance trial burns its
probes on known defects; (2) N5, the slug fix (a frozen-identity change); (3) `--carry-staged` joining
`jigc task validate` — *"the third and last deliberate bend"*, added after the Codex review.

## 1.2 The verified baseline (the thing the decisions rest on)

Six `capability-auditor`s (areas A–F) + four `gap-detector`s + three `robust-advocate`s, all **exercising the
real binary** at HEAD `f4a6a2b` / installed `jigc 1.0.0-rc.9`. Persisted as
`completions/artifacts/M47/baseline.md` after the in-house review found the scratchpad it lived in had been
cleaned mid-planning (this file survives in `recovery/m47/files/`).

It **corrected eight charter premises** (baseline §2): (1) H1's proposed "third arm" — advertise the shipped
`finalize --dry-run` — is **dead** (`plan_finalize` is a sequential abort-chain and dry-run halts at
`finalize.empty-commit` in the exact state an agent previews from); (2) the promise is on **nine** surfaces,
not four (the missed one is the highest-traffic: `render.rs:266`, the `what's-left:` line on every id-carrying
compose); (3) **no construction makes the promise literally true**, so prose-scoping is forced in every arm;
(4) P5-4's stated premise is false — the field-group-absent case *succeeds*; the real split is `--value` vs
`--unset` on one address; (5) `task diff` falsifies a recorded clean confirmation and is three
nonconformances; (6) tier 3's framing is half wrong (the non-empty-`left_out` **stdout** arm is pinned, the
**stderr** arm is not); (7) P3-2 narrows to the *collision* case; (8) golden count is **612** on disk
(`M45/VERDICT.md:23` says 558, `pinning.md:17` says ~1500).

It found **22 defects outside the charter, N1–N22** (baseline §3), five integrity-tier and five sitting on the
acceptance trial's own probe routes:

* **Integrity tier** — N1 the four milestone record-only doors brick permanently on one rejected commit
  (`create`/`add-task`/`add-from-spec`/`discard` seed the file-state baseline *after* the commit while every
  later write reconciles *before* it; every escape blocks incl. `discard --force` and a raw `git commit`;
  recovery required hand-editing `.jigc/state/file-state.json`) · N2 H2's dead end splits per clone · N3
  `set-field`/`doc author` at an undeclared **item field** = exit 0 + positive ack, then the doc is dead
  (`store.unparseable`, only `task discard` recovers) · N4 `set-slot` at an undeclared **leaf** of a
  single-slot item writes the real slot and acks the phantom leaf · N5 the slug drops the leading *and
  trailing* component of a hyphenated compound (`"On-call handoff artifact"` → `call-handoff-artifact`).
* **Greenfield-path tier** — N6 code-less dev workflows compose a finalize their own gate refuses · N7 the
  advertised `resume:` door dead-ends on any HEAD advance · N8 the pre-commit hook cries drift on every commit
  forever · N9 `single-task`'s changelog step cannot record a change (+ a false `describe` suppression reason)
  · N10 the store-scope trailer claims advisories gate.
* **Surface tier** — N11 `retitle-item` sibling · N12 unenriched `--unset` route · N13 silent
  `milestone create` commit · N14 `<<author: brief#vision>>` renders a role not an address · N15 `Create::task`
  leaks into `--help` · N16 the `doc schema` legend sits where its markers aren't · N17 `empty-commit`'s route
  omits `jigc task discard` · N18 `repeatable-populated` false-alarms on a young corpus · N19 ingest's weaker
  route · N20 positional-vs-flag inconsistency · N21 untested `relocate --format json` success arm · N22 mixed
  sha forms.

Plus **18 structural build facts S1–S18** (S17/S18 added at review): the two `finalize.yaml` files are
independently maintained and unfenced (S1); both stated-at fences are **presence-only and were broken live**
(S2); five verbatim-literal assertions have no regen path (S3); measured blast radii — dev finalize 156, meth
finalize 180, the validate-promise line **342**, `create-gates:` 324 (S4); **`cargo test` fails fast** (S5);
nothing consumes the wrong-shape/not-present distinction (S6); P1-7 is a seam plumb (S7); `error_code` is
log-only/additive and its membership check is a `debug_assert!` (S8); `--format` is global so all 44 leaf verbs
accept it, 43 honour it (S9); `task diff` needs no new data (S10); two tier-2 routes are `RouteKind::Human`
outside the argv fence (S11); the pack-load fence set (S12); `pinned_facts/` mechanics (S13); the door axis +
`CommitRejected` privacy (S14); a **masking test pins the H2 dead end as correct** (S15 — rewrite, don't
extend); the survivable frame's five parts (S16); **pack precedence is dev-highest, dev shadows methodology**
(S17); **the finding JSON emits keys alphabetically** (S18).

Four enumerations the decisions rest on: §4a the nine promise surfaces · §4b the **14 gate rows** finalize runs
that validate does not · §4c `owned_location_violation`'s **seven causes** (only the last consults `tracked`) ·
§4d the committing-door axis (**9 production doors + 1 excluded by design**) · §4e the write-verb × miss-shape
matrix (**7 cells, 2 correct**).

## 1.3 Scope tiers, as settled

* **Tier 1 — contract-touching** (pre-1.0-cheap, post-1.0-breaking): H1 the validate preview contract · P5-4
  the write-miss key/route axis · the `task diff --format json` hole · H2 the survivable hook-rejection frame
  swept over every committing door. (N5, the schema-hash narrowing, N1 and the `blocking_probes` envelope key
  joined tier 1 through the Settle.)
* **Tier 2 — the understandability batch**: ~16 verified wording/clarity items, each keyed to a
  findings-verification row; two are *currently pinned as lies by goldens* (the adapter render golden `H3`, the
  setup golden `P1-7`) so the fix must move the pin in the same motion.
* **Tier 3 — the pinning discharge**: every findings-verification repro block lands — confirmed blocks as the
  fixes' red tests, refuted blocks as `pinned_facts/`.
* **Plus the greenfield-path five** (N6–N10) and the ~15 non-fork doc/record repairs.

**Human scope call, up front and verbatim (narrative :5978):**

> 1. take all of it.
> 2. confirm they are in
> 3. the trail will tell us if we need to put more work into v1 or not. we will try to get ist as good as we can
>    and if need to be we will open an other milestone after this one

Recorded as: **no overload valve**; the greenfield-path five are in; **the trial is a report, not a second
close** (M47 closes once → rc.10 built + installed → the trial → then the 1.0.0 call or a *new* milestone).
Two decomposition calls were then made rather than re-asked: tier 2 ships **all ~16 declared items**, and the
wave ships a numbered **flow 47** acceptance suite like M41–M46.

## 1.4 The fourteen Settle decisions

Source: `DECISIONS.md` → *2026-07-26 — M47 planning: the Settle* (patch `0002`, byte-exact). Numbering is the
entry's own. The **three one-way doors (1, 2, 3) were each argued at full strength by an independent
`robust-advocate` before the human chose, and all resolved robust**; the advocates corrected the proposer twice.

**Decision 1 — H1: wire the previewable gates onto `jigc task validate`** (the carryover probe + the six
staging-independent `owner-artifact` causes), *in addition to* scoping the prose on all nine surfaces.
*Why:* M45 Decision 6 is engaged on its rationale and **preserved, not overturned** — its feasibility half is
live-proven, but it reaches "one of seven `owned_location_violation` causes and zero words of carryover", and
the trial's two blocks were causes 4 and 5 (`not-under-home`, `names-no-file`), neither of which consults
`tracked`. Deliberately **out**: `finalize.stage-failed`, hook rejection, `empty-commit`, `nothing-staged`,
cause 7 (untracked), and `promote-clobber`/`base-mismatch` (previewable but beyond the field-proven need).
Also fixed either way: *"`--dry-run` is information-free on this axis — its refusing-state and committing-state
JSON forecasts are byte-identical, and it labels entries `carried-over` when `--carry-staged` was never
declared."*

**Decision 2 — N5: fix the slug rule now, at `SLUG_RULE_VERSION` 3.** *Why:* the tokenizer splits a hyphenated
compound before the edge-stopword drop, so a leading **and trailing** component in `EDGE_STOPWORDS` is dropped;
**measured blast radius: zero divergent ids** across every keeper corpus, fixtures and goldens; and *"the
correct identity is unreachable through every shipped command in every sequence"* (`add-item` has no `--slug`,
`rename` is doc-level, `retitle-item` freezes the anchor). Constraints: the fix lives in `slugify`, **never**
`renormalize`; both manifests re-pin in one commit; the version bump is a *tested* obligation because the
version arm does not force it. Un-migratable by design — a corpus spanning the change carries two id
generations permanently.

**Decision 3 — narrow `schema_hash` to a presentation projection.** Three keys leave the hash —
`Schema.description`, `Schema.usage`, `Slot.hint`; the six semantics keys (`default` · `set` · `inverse` ·
`inverse-card` · `check` · `title-names-symbol`) **stay in**. The advocate argued *against* the wide
`erase_out_of_projection` reuse the planning first proposed. *The decisive argument, verbatim:* **"while prose
is in the hash, the freeze is unfenceable"** — proven live on rc.9: reword a hint and re-pin at the same
`schema-version` → green; widen the `adr.status` enum (a real structural change) and re-pin again at the same
version → **also green, `validate` clean exit 0**. Implemented + measured by the advocate: **2383/2**, zero
goldens, zero corpus impact, no bumps/snapshots/migration. `corpus-migration.md:180` engaged on its rationale
(*"out of scope here"* = scope, not principle). Rider 6 (the fence) was taken by the human here and **dropped
at review** (see §1.5).

**Decision 4 — N1: captured-pre-image rollback on rejection.** *Why:* the obvious fix is foreclosed by
`reconciliation.md:51` (*"durable file-state is persisted only by a landed finalize"*), which the code honours
exactly; rollback honours it untouched and joins the confidence-audit's captured-pre-image discipline as a
**fifth staged-path family**. It also clears the compounding defect (rejected runs leave record `.md` files
staged, tripping the next task's carryover gate). **Sequencing: N1 precedes H2 on 4 of 9 doors.**

**Decision 5 — N6: move the commit-doc writes from `step:implement` into `step:finalize`** (human call over the
smaller `author-commit` arm). *Re-settled at review* — see §1.5 — because the arm was structurally impossible.

**Decision 6 — H2: one error code per door** (human call over a shared door-agnostic code). *Why:* reusing
`finalize.commit-rejected` on eight non-finalize doors would put a lying code in the log. This **contradicts
`ERROR_CODE_REGISTRY`'s recorded "deliberately not a per-verb code mint"**, so that record is revised with its
rationale engaged. Mechanics: `CommitRejected` → `pub(crate)`; `render::commit_rejected` generalized off the
task idiom; `corpus_migration.rs:893` **rewritten, not extended**.

**Decision 7 — N8: filter the drift hook to blocking severity.** *Re-settled at review* (the stated mechanism
was falsified) — see §1.5.

**Decision 8 — P5-4: sweep the axis, not the instance.** 7 cells / 5 codes + `retitle-item` (N11) + a new class:
`<doctype>` ships **literal and unsubstituted** on four codes and `<address>`/`<task-id>` on three more. They
pass the route fence only because they are declared `DUMMY_SUBSTITUTIONS` members — **"the fence proves
parseability, never followability."** New property **P6**: *if a placeholder's value is derivable from the
finding's own `key.target`, it must be substituted.* Record cost accepted: an explicit authorization clause in
`command-output-contract.md` (the existing one authorizes a *target value* correction, not a `code` change) plus
the `validation.md:60` taxonomy revision.

**Decision 9 — N3/N4: guard both undeclared-address writes.** N3 reuses the guard shape already shipping on the
`--unset` sibling; N4 goes at the **write callers**, not `slot_span` (whose two consumers include conformance
adjudication), and `resolve_leaf` is duplicated in two places, both moving. The exit-0→exit-1 change is a
behaviour change M45 Decision 6 declined in an analogous case; *"it is taken here because this is a data-loss
class, which outranks."* Guarding the write **restores** `finding.rs:272`'s `conformance.*` route exemption to
its original scope.

**Decision 10 — N7: align the resume door to the overlap-aware rule, revising `storage.md:238`.** Three siblings
ship three different base-pin rules (`start --task` blanket-refuses, `finalize` is overlap-aware, `task
validate` is unguarded — verified in one repo state as resume exit 1 / validate exit 0 / **finalize landed**).
`finalize.md`'s M17 amendment rationale applies verbatim to the un-enumerated resume sibling. Also fixes
`form-vision` (which mandates the broken sequence) and the compose footer's parallelism claim.

**Decision 11 — `task diff --format json`:** `{op, task, base:{sha,short}, code_diff, staged_docs:[…],
findings:[]}` with a **non-empty empty state**. (Review changed `staged_docs` from `{id, body}` to identity +
address — see §1.5.) Settled at plan time rather than at build because there is no precedent shape (three
divergent shapes ship).

**Decision 12 — build the named-fact guard.** Both stated-at fences are presence-only and were broken live
(590 chars of copy-in/append contract prose deleted with the front-matter code kept → clean build, clean pack
load, contract gone; and deleting a `{{schema:}}` ref removes the obligation itself while the composed text
still says the payload "follows:" with nothing following). *Scope re-cut at review* to the whole fence family.

**Decision 13 — the demand-counter routes to M47, and is re-counted after this wave's scope.**
`decisions-pending.md:14` forces a thrice-demanded deferral onto *the next wave's* Settle agenda, and M47 is
that wave while capability entries 2/3/4 each say "Trigger: the M46 Settle" — the file contradicts itself. M47
formally re-Settles entries 2/3/4/8 (entry 8 **has fired**), and the re-count happens **after** M47's scope
lands, because this wave drains the evidence entries 3 and 8 are counted on.

**Decision 14 — N9: fix the step so the reason becomes true.** `record-changelog.yaml` is four lines naming only
`doc create` over a nested repeatable; the fix uses the `{{schema:changelog}}` seam, which **newly obligates**
the step to declare `create.singleton-copy-in`. Hand-writing the grammar instead would re-introduce what M43
Increment 3 deleted from twelve templates.

**Fifteen non-fork blocking gaps "settle by doing"**, headline: the validate≡finalize invariant is stated at
**eight** doc sites that already contradict each other, and `finalize.md:43`'s *"Same check, same report, later
position"* is false; **`VISION.md:120` is deliberately left unchanged.** Plus: `reconciliation.md`'s
"exactly three re-baselining sites" (N1 is a fourth) · `validation.md:60`'s over-universalized enrichment ·
`surface-contract.md:28` vs `finalize.md:99` · three false `DECISIONS` bases (`:667`, `:749`, `:2602`) ·
`decisions-pending.md:190`'s stale `--dry-run` entry · `pinning.md`'s 1500-vs-612 self-contradiction ·
**`implementation/roadmap.md` has no section for the confidence-audit wave at all** ·
`assistant-adapter.md:48`'s rotted "cannot rot" rationale · N10's second arm · N18's mirrored knob ·
N16's legend · `describe --format json` vs its own help.

## 1.5 The in-house pre-decompose review (what it changed)

`DECISIONS.md` → *2026-07-26 — M47 planning: the pre-decompose review* (patch `0007`): **10 blocking + 10
advisory**; it **refuted two Settle claims against the code**.

* **Refuted 1 — pack precedence was stated backwards.** `pack.rs:915` says *"dev first = dev-highest"*;
  a builder following the Settle would have edited the wrong `commit` schema.
* **Refuted 2 — the finding JSON's key order was stated backwards.** Keys are emitted **alphabetically**
  (`check, code, key, location, message, probe, route, severity`), so `severity` comes *after* `probe`.
  Decision 7's mechanism was falsified as written. (The false claim came from a gap-detector that had matched a
  synthetic fixture.)

**Four re-settles went back to the human** (their questions + answers are in §2):
1. **Decision 5 → split `step:finalize` into a solicit half and a commit half.** The "move the writes" arm was
   structurally impossible: `migration-finalize.yaml` in **both** packs ends with `{{ include: step:finalize }}`
   and `project-finalize.yaml` is *only* that include, so all **12** migrate workflows would inherit the doubled
   solicitation the decision forbade — and the dialect has no conditionals. Riders: the code-less count is
   **four** + `project-setup` by another path; methodology's `finalize.yaml` **already** solicits, so its 7
   migrate workflows double-solicit at HEAD today.
2. **Decision 3's rider-6 fence — dropped, chartered to M46.** No implementable mechanism (a pack-load assert
   cannot observe that a value *changed* without a recorded prior) and the narrowing itself needs **16 bare
   re-pins at unchanged versions**.
3. **Decision 7 re-settled entirely:** `jigc validate --format json` gains a top-level **`blocking_probes`**
   array; the hook matches on that. An additive envelope key is contract-touching → tier 1.
4. **Decision 6's release hole settled** as a **test iterating the registry against the producer set**.

**Six findings baked back without a re-decision:** the fence-vs-slug-re-pin collision resolved by the successor
rule *each hashed entry's hash may move only when its **co-located** version field moves* · `erase_presentation`
pinned as an **erase-list, not a complement**, with an exhaustive `Schema` destructure · **P6** gains its seam
(the **finding-serialization** seam, not `Route::mechanical`) + a per-placeholder derivability table + the
correction that **`<task-id>` is not derivable from `key.target`** · Decision 12's guard re-scoped to the fence
family (every step carrying a `states-constraints:` code, both packs) with the named-fact mechanism pinned as a
`constraint-code → required-token(s)` map checked at pack-load · **the baseline persisted as a committed
artifact**.

**Advisories accepted:** H1's blast radius is **342 of 612 goldens** plus four inline suites ·
`task diff`'s `staged_docs[].body` would mint a second unversioned content-read path → identity + address
instead · `findings: []` declared structurally-always-empty · Decision 10 gains the "it is *un*-refusing"
argument · Decision 13's re-count recorded with a trigger · `storage.md` gains the generation-3 row.

## 1.6 The cross-model (Codex) review — what it changed

Run on the human's explicit instruction ("run the codex review") against the settled design at `b28d31f`;
`codex-cli 0.134.0`, `codex exec --sandbox read-only`. Recorded as `DECISIONS.md` → *2026-07-26 — M47 planning:
the cross-model review* (patch `0015`). **3 blockers, 1 improvement, 4 clean bills.**

* **Blocker 1 — `blocking_probes` greppability: premise right, conclusion refuted on test.** `render::json` is
  `to_string_pretty` so the array is multi-line and a line-oriented grep cannot bind the key to a member. But
  after collapsing newlines, `"blocking_probes"\s*:\s*\[[^]]*"doc-code"` is bounded by the array's **own `]`**
  and **structurally cannot escape into `findings`**; both negative controls pass (advisory-only `doc-code`
  present in `findings` → no match; empty array → no match). The shape stands; **the hook's match is now
  specified in the roadmap** and its acceptance carries the two negative controls (roadmap patches `0012`,
  `0013`).
* **Blocker 2 — increment 1 forbade its own work.** The same paragraph re-pinned 16 hashes at unchanged
  versions and six lines later adopted the co-located-version rule. Fixed by **scoping the rule forward and
  declaring increment 1 the one genesis exemption**, both re-pins landing in one commit (roadmap patch `0010`).
* **Blocker 3 — "validate exits 3 only where finalize already refuses" was false for the consent flag.**
  `carry_staged` is declared on the `Finalize` variant alone, so after increment 4 a driver that always intends
  to carry would get a **permanently red preview**. Fixed both ways: the claim is scoped to *default* finalize,
  **and `--carry-staged` joins `task validate`** — *"the third and last deliberate bend of the wave's
  no-new-surfaces boundary"* (roadmap patch `0011`).
* **Improvement accepted — `2383/2` is a prediction, not a measurement.** Increment 1 **re-measures**:
  after `erase_presentation` lands, `cargo test --no-fail-fast` must fail **exactly twice**, at
  `crates/engine/src/manifest.rs:400` and `crates/cli/tests/freeze_enforcement.rs:232` — both of which use a
  `hint` reword as their stand-in for a shape change. **A third failure, or a different pair, is a halt.**
* **Clean bills (load-bearing):** the slug fix is confined correctly — Codex *looked for a path where an
  already-committed id is re-derived rather than recognized and found none*; the owner-artifact seven-cause
  split is sound; 612 goldens / 342 promise-carrying / 6+10 schema entries / 44 leaf verbs all independently
  re-counted; per-door error codes defensible **conditional on the producer-set test being generated from the
  same 9-door axis rather than hand-maintained** — a condition written into increment 3 (roadmap patch `0016`).

The human's instruction on how to fold it in, verbatim (narrative :3465):

> Include what is useful yes. Do not trust to 100% all findings since codex can also be wrong. Prose should be
> included when it is benefitial for other agents/session with no context kb as we have currently, so
> there understanding is the same as ours now. Also if the finding improves jigc or hardens it we should also
> include it

## 1.7 The greenfield acceptance (G1–G3)

Two acceptance bars (baseline §1, verbatim): *"(1) In-repo: **flow 47** through the real binary + per-fix axis
tests + a clean completion audit + the four-command gate. (2) Out-of-repo: **1.0.0-rc.10** built + installed,
then a **three-probe blind greenfield trial** — G1 cold start (rejecting hook under `core.hooksPath` +
pre-staged-before-mint plants) · G2 the design altitude from zero · G3 corpus accretion from nothing. **The
trial is a report, not a second close.**"*

**Strategic-claim check, verified rather than inherited:** the only greenfield trial on record is
`RC-greenfield` on **rc.2** (2026-07-06) and it was **human-driven from `jigc setup` onward, not blind** — so
G1–G3 would be **the first blind greenfield sessions ever run**. This is a *stronger* claim than the charter's
"second greenfield run", and it is why area F of the baseline drove the greenfield flow at plan time: five
defects (N6–N10) were found on the probe routes and pulled into scope so the trial would not burn its probes on
known breaks. Declared bound: area F was **single-operator and non-blind**.

G1's plan-time transcript (baseline area F) ran 27 steps: bare `jigc` → `start` pre-setup → `setup` →
orientation → router → previews → mint → validate → author → finalize → ADR via `record-decision` → plant (b)
pre-staged carryover → plant (a) `core.hooksPath` + rejecting hook → stamp below/above current → `doc list` /
`task list` / `task discard`. G2 = the design altitude from zero (`do-research` → `form-vision` → `park-idea`,
where the `resume:` dead end was found). G3 = corpus accretion from nothing (foreign drop → detect → migrate →
review hold → approve → retire), where the hook-drift and changelog-step breaks were found.

## 1.8 The gate-record and the decomposition

`completions/artifacts/M47/planning-gate-record.md` (survives byte-exact) fills the milestone-level gates
(strategic-claim-fresh · value-flow-exercised · cheap-vs-robust · foreclosed-by-doc · prior-art-reconciled ·
census · acceptance-spiked · deliverable-reachable · check-scope-pinned · integration-seam) and a **19-row
per-scope-item table** with PA/DC/RE/CS/IS columns. Three items are declared **owed at increment-plan time**:
P6's finding-id + severity · N4's multi-slot fixture *or* a re-declared bound · Decision 13's re-count.

**The decomposition: eleven increments**, cut "one-way doors first, N1 before H2 as a hard prerequisite, every
surface-changing cluster before any golden regenerates." The roadmap body itself is **lost**; what can be
reconstructed from surviving fragments, commit subjects and narrative references:

| # | Content (reconstructed) | Confidence |
|---|---|---|
| 1 | The two one-way doors in one coordinated commit: N5 slug → generation 3 (`SLUG_RULE_VERSION` 2→3, both manifests re-pinned) + `schema_hash` → presentation projection (`erase_presentation`), with the **exactly-two-failures halt condition** and the declared genesis exemption | high |
| 2 | N1 — the record-commit transaction: the four milestone record-only doors stop bricking; the appending doors become atomic; *Proves* "a rejecting hook on any of the four doors leaves the repo recoverable — fix the hook, re-run, it succeeds" + "no staged residue" | high |
| 3 | H2 — the survivable frame swept over the whole 9-door committing axis + per-door error codes + the registry-vs-producer-set test; **re-scoped mid-build with T0** (see §2) | high |
| 4 | H1 — validate runs the previewable gates (carryover + owner-artifact causes 1–6) + the nine-surface prose scoping + `--carry-staged` on validate + the `--dry-run` `carried-over` fix; **largest radius: 342 of 612 goldens**, regenerated in increment 11 | high |
| 5 | Not named anywhere in the extraction | **lost** |
| 6 | P5-4 / N11 / N3 / N4 / P6 — the write-miss axis, the undeclared-address guards, the `GenerateError::NotPresent` variant, route followability | high (named as "increment 6's recorded N4 decision") |
| 7 | N8 — `blocking_probes` + the rewritten drift hook (+ the 44-leaf-verb `--format json` sweep, inferred) | high |
| 8 | Not named anywhere in the extraction | **lost** |
| 9 | The named-fact guard (Decision 12) — commit `3a35430` "the schema seam and its copy-in declaration brick apart both ways" | medium |
| 10 | The ~16-item tier-2 wording batch (≈20 fix commits after `92e2d42 design(m47): increment 10 task decomposition`) | high |
| 11 | The golden generation, the pinning discharge, flow 47, the record-truth repairs, the M46 re-count, the rc.10 bump, the fold-back | high |

---

# 2. The mid-build (and mid-planning) re-Settles — every fork the human answered

`asks.md` holds **five `AskUserQuestion` blocks / sixteen questions**, each with the human's recorded answer.
Two blocks are planning-time, one is the post-review re-settle, two are mid-build. Plus three inline
human decisions in the narrative. All are decisions of record.

### Block A — 03:37:54 (the three one-way doors)

1. **H1 — how much of the validate preview contract does M47 build?**
   **ANSWER: "Carryover + the 6 staging-independent owner-artifact causes (Recommended)."**
   → Became Decision 1. (Rejected: "carryover probe only" and "prose-scope only + fix the `--dry-run` false
   green", the latter because it would have required writing a *new* clause into
   `command-output-contract.md` blessing a task-scope divergence no record ever decided.)
2. **N5 — the slug drops the leading (and trailing) component of a hyphenated compound; fixing it forces
   `SLUG_RULE_VERSION` 2→3, un-migratable by design.**
   **ANSWER: "Fix now, at generation 3 (Recommended)."** → Decision 2. (Rejected: fix-outside-M47's-tiers,
   declare-current-behaviour-correct, defer-with-a-trigger.)
3. **The schema hash freezes authored prose, so a typo fix costs a version bump + snapshot + a corpus migration
   whose entire diff is the stamp line.**
   **ANSWER: "Narrow, and also build the same-version-re-pin fence now."** → Decision 3 **plus rider 6**.
   (Rider 6 was later dropped at review — see Block C.)

### Block B — 03:37:54 (four more forks)

4. **N1 — the four milestone record-only doors brick permanently on one rejected commit; the obvious fix is
   foreclosed by `reconciliation.md:51`.**
   **ANSWER: "Captured-pre-image rollback on rejection (Recommended)."** → Decision 4.
5. **N6 — five code-less dev workflows compose a finalize their own gate refuses. Where does the fix go?**
   **ANSWER: "Move the writes from step:implement to step:finalize"** — i.e. the human **overruled** the
   assistant's recommended `author-commit`-variant arm. → Decision 5 (later re-settled, Block C).
6. **H2 — sweeping the survivable frame to 8 more doors needs an error code for each;
   `ERROR_CODE_REGISTRY` has 2 members and says "deliberately not a per-verb code mint".**
   **ANSWER: "Mint one code per door"** — again **overruling** the recommended shared door-agnostic code.
   → Decision 6, with the registry's rationale revised rather than silently overridden.
7. **N8 — the pre-commit hook cries drift on every commit forever; filtering to blocking severity would
   silence `doc-code.title-names-symbol`, which M40 deliberately demoted.**
   **ANSWER: "Filter to blocking severity (Recommended)."** → Decision 7 (mechanism later re-settled).

### Block C — 03:57:56 (the four post-review re-settles)

8. **Decision 5 is structurally impossible as settled** (both packs' `migration-finalize.yaml` include
   `step:finalize`; `project-finalize.yaml` is only that include; no conditionals in the dialect).
   **ANSWER: "Split step:finalize into a solicit half + a commit half (Recommended)."** — recorded as *"a
   re-decision on new facts, not a reversal"*.
9. **Decision 3's rider-6 fence has no implementable mechanism, and the narrowing needs 16 bare re-pins the
   fence would forbid.**
   **ANSWER: "Drop the fence; ship the narrowing alone, charter the fence to M46 (Recommended)."**
   → `decisions-pending.md` entry `(D) The manifest-hash-change fence`, trigger = the M46 Settle, with three
   candidate mechanisms recorded (append-only `(type, version) → hash` ledger · a CI git-diff check · the
   co-located-version successor rule).
10. **Decision 7's mechanism is falsified — the finding JSON emits keys alphabetically.**
    **ANSWER: "Re-settle the mechanism entirely."** → the `blocking_probes` top-level array.
11. **Decision 6 grows the registry 2 → ~10 but its membership check is a `debug_assert!`, compiled out in the
    release build the trial runs on.**
    **ANSWER: "Test iterating the registry against the producer set (Recommended)."** → later hardened by Codex
    to *derive the producer set from the same 9-door axis, never a hand-maintained second list*.

### Block D — 07:41:38 (the Increment 3 halt — mid-build)

12. **Increment 3 cannot give `jigc milestone finalize` a truthful survivability frame until the `squash:false`
    abort path stops destroying sub-task staged code. How should the increment resolve?**
    **ANSWER: "Insert T0 — fix it first (Recommended)."** — i.e. widen the approved increment rather than write
    an honest data-loss frame or drop the door from the sweep.
13. **Should `milestone finalize` REFUSE when a milestone recorded as provisioned has no live worktrees,
    instead of silently degrading to a docs-only commit at exit 0?**
    **ANSWER: "Refuse — block the degrade (Recommended)."** — taken to close the class, not just the one path.

The session then ran the **mandated halt-and-resume gates** (`increment-workflow.md` → Halt and resume): a
property census over the four `remove_worktrees` sites, then an independent `design-reviewer` adversarial pass —
because *"M31's only escaped defect entered at exactly this gate, on exactly this code path."* The pass returned
four blockers, which produced Block E.

### Block E — 08:17:37 (the halt resolution's own re-settle)

14. **Property B is unbuildable as settled (no discriminator exists; a `.jigc` marker fails open on the
    fresh-clone case it exists to catch — verified live: fresh clone → `finalize` → exit 0, zero work landed,
    record flipped to terminal `joined`). What replaces it?**
    **ANSWER: "Refusal + manifest statement (Recommended)."** → (i) refuse the zero-contribution shape (block
    when `has_diff` rests **solely** on `record_changed`) + (ii) the landing manifest names each sub-task with
    no provisioned worktree. The durable "was provisioned" fact → **M46** (`decisions-pending.md` entry 10),
    because its only fresh-clone-durable home is the committed `milestone-record`, a frozen doctype
    (schema-version 2→3 + corpus migration + manifest re-pin).
15. **New finding outside the original fork: the LANDED teardown destroys sub-agents' unstaged and untracked
    work at exit 0, silently. `discard` guards that set and demands `--force`; finalize guards nothing.**
    **ANSWER: "Law-1 visibility now, refusal → M46 (Recommended)."** → `remove_worktrees` warns on a non-empty
    `git status --porcelain` before removing + the landing manifest names what the teardown discarded, with the
    bound written down: **visible, not prevented**. The `discard`-style refusal + `--force` → M46
    (`decisions-pending.md` entry 11).
16. **Advisory A1: on the ff-refusal arm the milestone record is left STAGED in the live index holding the
    `joined` blob — a lying record a subsequent plain `git commit` would land.**
    **ANSWER: "Fold into T0 as a fifth capture axis (Recommended)."**

All three were recorded before the build resumed, in `931d7ef` (`DECISIONS.md` + `roadmap.md` increment 3
amended **in place** so the wave would not renumber + `decisions-pending.md` entries 10/11).

### Inline human decisions (narrative, not `asks.md`)

* **The scope call** — "take all of it" / "confirm they are in" / "the trial will tell us…" (quoted in §1.3).
* **"run the codex review"** — the cross-model pass was the human's call, per
  `milestone-planning-workflow.md:54` (suggested, never auto-run, specifically for one-way doors).
* **The Codex fold-in instruction** (quoted in §1.6).
* **"goahead"** — the build-session start.

### One boundary crossing left open at the crash

The `add-from-spec` fix (§4) **retired a bound `design/team-ready-state.md` had recorded** as *"a capability M47
does not charter."* The orchestrator surfaced it twice as a decision the human could overturn ("reverting is
cheap now, expensive after the trial") and **never received an answer**. It stands as shipped, with the override
recorded in `DECISIONS.md`.

---

# 3. Build state at the crash

## 3.1 Timeline

| Phase | Result |
|---|---|
| Planning (3 commits off `f4a6a2b`) | `443921c` the Settle · `b28d31f` review baked + baseline persisted + eleven increments cut · `504d43b` cross-model review baked. Tree clean. |
| Build run 1 — `Workflow(milestone-build, {milestone:"M47", base:"504d43b"})`, run `wf_f33675aa-72b` | Increments **1–2 landed and validated**; **halted at increment 3's plan** on a genuine fork. 7 commits, `504d43b` → `39117c0`, suite **2392/0**. |
| Halt resolution | property census + independent `design-reviewer` pass + 3 human forks → recorded in **`931d7ef`**. |
| Build run 2 — same workflow, `skipThrough: 2`, run `wf_28e41818-ebe` | **`status: built-and-audited`** — 87 agents, 85 done, 2 transient 529s on `exec:inc3:T4` (retried and landed). |
| Build end state | **76 commits, `504d43b` → `7e300e4`, tree clean, all eleven increments in, `1.0.0-rc.10` bumped, flow 47 landed.** |
| Completion audit | Code review: **deliverable holds, no HIGH, no MEDIUM — 4 LOW.** E2E: **7/7 flow-47 arms pass through the real binary; overall FAIL on 4 confirmed defects**, none data-loss, none regressions. |
| Post-audit triage + fixes | 8 findings → 1 closed as NOT-A-DEFECT, 1 sized, the rest dispatched to `build-fixer`s. **Twelve fix commits** landed, `7e300e4` → `b415c67`, suite 2385 → **2552/0**, tree clean at every step. |
| **Crash** | Immediately after dispatching the **thirteenth** fixer ("Close the git-path class", 4 remaining members, agent `a67f48149813fe1c5`, from HEAD `b415c67`). Its result never arrived. |

## 3.2 Commits recovered by hash

**Planning:** `443921c` · `b28d31f` · `504d43b` (+ `931d7ef` for the halt resolution).

**Increments 1–2 (from `git log 504d43b..39117c0`):**

```
5479347 design(m47): increment 1 task decomposition
32f5bbf feat(engine): the slug rule at generation 3 — the edge-stopword drop is word-aware
69223fa feat(engine): the schema-hash becomes a presentation projection
8b3e33c design(m47): increment 2 task decomposition
4b89339 feat(cli): the record-commit transaction — the record-only doors stop bricking
aae5431 feat(cli): the appending doors become atomic — the mint unwinds with its record
39117c0 fix(engine): the conflict route belongs to the caller, not the classifier
```

Validator verdicts: **Increment 1** deliverable_holds ✅ gate_green ✅, 0 blocking / 4 advisory (the
`words.len() == 1` clear drops a non-whole token · `SLUG_RULE_VERSION`'s rustdoc doesn't list generation 3 ·
`erase_presentation` is `pub` with no outside caller · the char-cap cell pinned at unit level only).
**Increment 2** deliverable_holds ✅ gate_green ✅, 0 blocking / 4 advisory (the conflict-block *message* still
names a nonexistent task · the sibling `file-state.hash-matches` route still inapplicable · the index-restore
axis correct but untested · the "no staged residue" clause tested by a porcelain proxy, never by driving the
gate). **Increment 1's halt condition hit exactly as forecast: 2390/2, the predicted pair, no third.**

**Increment 3 (partial hashes recovered from later references):** `0616516` (the N2 migrate-corpus
pending-landing fix, the premise a later finding threatened), `7159c36` (T2 — the zero-contribution refusal),
`5fba89e` (T3 — "the landed teardown names the work it discards"), `1b7c978` (T4 — the fifth capture axis).

**Increments 10–11 (top of `git log 504d43b..7e300e4`, verbatim):**

```
7e300e4 docs(m47): the fold-back — no doc still states the pre-M47 world
b5bd2a7 chore(release): 1.0.0-rc.10 — the bump and its twelve-golden blast radius
221a57a docs(m47): the two refusals a cold start meets get their recovery chapter
58a9afb docs(m47): the M46 demand count is re-counted over the post-M47 state
ae5f83b docs: the three false bases are annotated, and pinning.md counts its own tree
37474cb docs(m47): the confidence-audit wave gets the roadmap row it shipped without
24eb5f1 docs(m47): every verified fact names its standing test, or says why it has none
a08e4d0 test(cli): the three refuted trial claims become standing pinned facts
ad9253a test(cli): flow 47 — the wave's composite done-picture, seven axis-iterating arms
0f2380b design(m47): increment 11 task decomposition
974463b fix(cli): describe's help stops denying the version key it stamps
63a97d3 fix(pack): an author directive's pending token previews the address it becomes
eadf436 fix(cli): a blocking store row says whether anything stops on it
10fff50 fix(pack): a young changelog is not a hollow one
a7f84e5 fix(engine): the slug rule's one-liner states the rule it enforces
8e4ee2f fix(cli): `doc author --help` states the collision reject it actually performs
25ac653 fix(pack): "documented code" is defined where the router asks you to choose on it
9cbed2f fix(cli): the `doc schema` legend sits where its markers are
9c6e9f6 fix(cli): describe and orientation stop overclaiming the preview
bdcb437 fix(cli): the help stops speaking Rust, and the create doors name their form
057c489 fix(cli): `create-gates:` says what a create gate is
dee02f0 fix(cli): the read surfaces label what they print
2a5be42 fix(cli): the stale-copy note speaks to a reader who may be the staging task
5d313ef fix(cli): the five edit verbs acknowledge the copy-in they performed
86c78f3 fix(cli): ingest's needs-reconcile rows route as strongly as validate's
58ec293 fix(engine): the finalize refusals name every exit and speak one sha form
1290f7a fix(engine): the fragment miss teaches the address grammar it just refused
6367e8c fix(engine): `title-names-symbol` states its heuristic and routes mechanically
8057878 fix(cli): the setup summary names the hook file it installed
2a21e18 fix(cli): the AGENT.md preload states the store-scope exit rule the binary takes
92e2d42 design(m47): increment 10 task decomposition
3a35430 feat(cli): the schema seam and its copy-in declaration brick apart both ways
ce3c142 feat(cli): a decl… [log capture truncated here]
```

(The remaining ~43 commits of the 76 — increments 3 through 9 — are **not** recovered by subject line.)

**The twelve post-audit fix commits:**

```
5baddd4 fix(engine): every fragment miss teaches the grammar, at every hop
e2a319c fix(engine): an undeclared slot key routes at the schema, not the item ids
646372e fix(milestone): a fresh clone rebuilds the workbench, not just the cache
9a974ff fix(milestone): add-from-spec seeding resumes, so its re-run line is true
0a1acc5 fix(milestone): partition the teardown-loss warning into never- vs partly-staged
7cbf22d fix(cli): one porcelain parser — the task-seam path mangling, swept to its axis
ae2f61b (task::git_commit_files — the post-commit file-state advance)
c5cee76 (task::git_untracked_all — the owner-artifact gate's tracked predicate)
b415c67 fix(cli): the cross-worktree collision set names a followable path
```
Nine are recovered by hash; the orchestrator's own running total at 19:21 says **twelve** fix commits, so three
(the LOW hygiene items — see §3.4) are unrecovered by hash. Suite progression across the fix rounds:
**2530 → 2531 → 2535 → 2537 → 2547 → 2552**, gate (`fmt` / `clippy -D warnings` / `test --no-fail-fast` /
`build`) green at every step, tree clean at every step.

## 3.3 The completion audit, recovered verbatim in substance

**Code review summary** (from the workflow result JSON): *"The M47 deliverable holds."* It independently
verified the load-bearing claims rather than trusting them — the slug change is genuinely confined to `slugify`
(`renormalize` byte-unchanged, a `tokens_match_renormalize` proptest pins the shared separator map, and the
version bump is a **real tested obligation** via a `GENERATIONS` fingerprint table); `erase_presentation` is an
explicit erase-list with exhaustive destructures and a both-directions axis test (**5 presentation keys
invariant / 13 semantic-structural keys still move**) and `schema_hash` has no consumer outside the freeze
fence; **all nine committing doors are wired to `rejected_outcome`, bijective with `ERROR_CODE_REGISTRY`**, and
each has a recovery arm in `commit_rejection_axis.rs` that executes the *emitted* re-run line verbatim; the
fan-out abort teardown removal properly **inverts** its old pin with a recorded basis-has-changed rebuttal; the
**438 changed goldens** carry real new content (the reviewer read the `single-task` compose diff end to end);
and live probes confirmed the H1 preview (exit 3 on a pre-staged carryover, exit 0 under `--carry-staged`), the
refusing `--dry-run` envelope, the `blocking_probes` hook regex against positive and negative controls, and
panic-free rejection on seven hostile write addresses. **4 LOW findings.**

**E2E summary:** all **seven flow-47 arms** hold end-to-end through the real `target/debug/jigc` (1.0.0-rc.10)
in throwaway repos, as do the per-increment grouped-scope bullets it could drive: the validate/dry-run/finalize
preview agreement (incl. `--carry-staged` parity and the closed `--dry-run` false green); **7 of the 9
committing doors** under a rejecting `core.hooksPath` hook with verbatim re-run recovery and no staged residue;
the **8-cell addressed-item matrix** with byte-identical staged bytes across rejects; the leaf-verb
`--format json` sweep + `task diff` envelope + clap carve-out; the freeze fence's prose-vs-shape discrimination
over **all 16 doctypes in both packs**; the named-fact guard over every declared fact plus the `{{schema:}}`
biconditional and the unmapped-code arm; and the generation-3 slug rule across all four minting doors on both
edge shapes. **Fan-out determinism** witnessed by an N-process binary sim in three deliberately divergent
sub-task orders, all landing the identical commit sha and tree with byte-identical join/finalize output.
Increment 3 T0's abort arm held over both a hook rejection **and a hookless `git merge --ff-only` refusal**, and
the fresh-clone zero-contribution refusal blocked at exit 3 with the record left `active`.
**Verdict FAIL on four confirmed defects** (§4). Declared bound: headless, so the concurrent-spawn proof is the
N-process sim + verbatim spawn-template execution, not a genuine Task-tool spawn (the M39 bound, already
discharged live at project-alpha-3.0).

## 3.4 What was definitively NOT done

1. **The three remaining LOW hygiene items** were still queued at the crash: the `doc create --help` slug seam
   (the un-swept third consumer of increment 10's fix), the `brief`→`prd` role-rename note, and a test-only
   `pub` const. *(The `doc create --help` item is one of the four e2e defects — it was never dispatched.)*
2. **The final git-path fixer never returned.** Four verified members were dispatched and are unfixed:
   `rename.rs:474 scan_prose_mentions` · `milestone.rs:964 record_only_range` ·
   `milestone.rs:3248 milestone_landed_summary` · `migrate_corpus.rs:463`.
3. **rc.10 was never rebuilt or reinstalled.** The installed binary predated all twelve fix commits. The
   orchestrator had explicitly stated the next steps: rebuild + reinstall rc.10, re-run the full gate, and run
   **a fresh e2e pass** before the 1.0.0 call, "since the audit's green predates this entire sweep."
4. **The three-probe blind greenfield trial (G1–G3) was never run.**
5. **No milestone `VERDICT.md`** (`completions/artifacts/M47/VERDICT.md`) was written; the wave was never
   formally closed, no CLAUDE.md fold-back for the fix sweep, no `decisions-pending.md` graduation beyond the
   planning-time one.
6. **Two carried items with no home yet:** `doc set-slot` at a section with no slot exits via a plain CLI error
   outside the findings envelope (flagged out-of-scope by a fixer, the orchestrator said it would route it once
   the queue drained), and the two-hop undeclared-address `anyhow` fall-through the e2e noticed.
7. **The unrouted planning-process note** (six concurrent capability-auditors sharing one `target/` produced
   spurious `pack-load stated-at fence failed` errors — a harness artifact, not a product defect) was never
   added to `implementation/milestone-planning-workflow.md`.
8. **The `add-from-spec` boundary-crossing question** was never answered by the human (§2).

---

# 4. New defects found during the build

## 4.1 The Increment-3 halt cluster (found at plan time, mid-build)

| # | Defect | Verdict | Fix |
|---|---|---|---|
| a | **The `squash:false` chain-abort arm destroys every sub-task's staged code.** Live: hook-rejected chain finalize → `.jigc/worktrees/` gone; fix the hook, re-run → **exit 0**, commit carries the ADRs and *not* `src/low.rs`/`src/zed.rs`, manifest says "2 docs" per sub-task. The `squash:true` sibling recovers correctly. | **REAL, data-loss.** And it *enforces the design of record*, not revises it: `finalize.md:157` already promises "working area intact", `team-ready-state.md:93` justifies the force-removal by *"the commit lands first"* — false on the abort arm. | **LANDED** as Inc 3 T0(a): the `remove_worktrees` call at `milestone.rs:2064` deleted; the lying comment at `:2057-2062` corrected; the shipped test `milestone.rs:2313-2375` **inverted, not extended**, as a basis-has-changed rebuttal of M31 Inc 3 T4. |
| b | **The trigger set is wider than "hook rejected."** `:2063` catches *any* `Err` — including `git merge --ff-only` refusing over ordinary untracked WIP in the main checkout, **with no hook installed at all**. | **REAL** (found by the independent pass; the halt report understated the exposure). | **LANDED** — one call site fixes all of it, but the acceptance iterates **three rejection causes** (aggregate hook · per-sub-task hook · ff refusal). |
| c | **The LANDED teardown destroys unstaged and untracked sub-agent work at exit 0, silently, on the success path.** The chain/combine commit only the **staged** set; `git worktree remove --force` then deletes the whole checkout. `discard` guards exactly that set behind `--force`; finalize guards nothing. | **REAL** — and it refuted the orchestrator's own census row ("MUST NOT change… the commit landed, every byte is in git"). | **PARTIALLY LANDED** (Inc 3 T3, `5fba89e`): law-1 visibility — warn on non-empty porcelain, name the discarded paths in the landing manifest. Bound recorded: **visible, not prevented**. The refusal → M46 (`decisions-pending.md` entry 11). |
| d | **The fresh-clone zero-contribution finalize**: fresh clone → `finalize` → **exit 0**, zero work landed, record flipped to terminal `joined` so the milestone can never be re-finalized. | **REAL**; also proves Property B unbuildable in charter (a `.jigc` marker fails open by construction). | **LANDED** (Inc 3 T2, `7159c36`): refuse when `has_diff` rests solely on `record_changed` + the manifest names un-provisioned sub-tasks. The durable provisioning fact → M46 (entry 10). |
| e | **The ff-refusal arm leaves the milestone record STAGED** holding the `joined` blob — a lying record a subsequent plain `git commit` would land; a live counter-example to Increment 2's own "no staged residue" clause. | **REAL** (advisory A1). | **LANDED** (Inc 3 T4, `1b7c978`) as the **fifth capture axis** of the pre-image discipline. |
| f | Two further worktree destroyers: `provision_worktrees`' `remove_dir_all` on a non-registered leftover (T0 makes "a live worktree holding uncommitted code" *normal*, widening that path) · `jigc uninstall`'s unconditional `.jigc` wipe. | **REAL, declared bounds not fixed.** | **NOT FIXED** — recorded in `decisions-pending.md` entry 11 as riders. |

## 4.2 The completion-audit findings

| # | Finding | Verdict | Outcome |
|---|---|---|---|
| 1 | **`add-from-spec`'s mid-loop rejection tells two lies and names no recovery** — prints "so this command re-runs cleanly" and an argv that exits 1 with `milestone.sub-task-collision`, leaving the remaining criteria unseeded. Directly contradicts Increment 2's own *Proves*. | CONFIRMED (e2e, MEDIUM) | **FIXED `9a974ff`** — resumable seeding (`SeededFromSpec { added, already_seeded }`); the axis test iterates *k* ∈ 1..=N and lifts the **emitted** re-run bytes from the printed line. **Retires a recorded charter bound** — open for the human to overturn at the crash. |
| 2 | **Fresh-clone milestone continuation dead-ends** — `.jigc/tasks/<sub-task-id>/` is never rebuilt, so the adapter's own emitted `Spawn:` line fails with "no task"; six doors dead; the route is a **loop**. | CONFIRMED (e2e, MEDIUM-HIGH → sized MEDIUM). **Pre-existing since M39** (reseed fn byte-identical pre-M47), but M47's new zero-contribution refusal routes into it. Stayed green 20 days because `milestone_zero_contribution.rs:138` **hand-wrote the state the tool cannot rebuild**. | **FIXED `646372e`** — the shared reseed site rebuilds every sub-task area; the masking test dismantled (runs the emitted `provision` and `Spawn:` spans verbatim through a `jigc` shim). Declared bound: a `--workflow`-overridden sub-task's re-entry is not fresh-clone-durable (refused loudly, never mis-composed). |
| 3 | **`doc create --help` still publishes the caps-only slug sentence** — the un-swept third consumer of increment 10's fix, on the door a cold start uses first. | CONFIRMED (e2e) | **NOT FIXED** — still in the queue at the crash. |
| 4 | **`store.no-such-item` / `store.no-such-leaf` kept bare prose routes** when their `no-such-section` sibling was fixed. | CONFIRMED (e2e) | **FIXED `5baddd4` + `e2a319c`** — and the fixer enumerated the emitters rather than trusting the report: **nine** miss causes, not three (incl. the item hop into a section holding no items, and the leaf miss on a fields-only section). Fixed as a **seam**: `store::Miss` builds each route from the address truncated to the hops that *did* resolve. New `crates/cli/tests/doc_show_miss_axis.rs` iterates all nine × both formats × both arms and runs the emitted route argv verbatim requiring exit 0. |
| 5 | **The teardown-loss warning over-reports** — `never_staged_paths` collects `MM`/`AM` cells, so the one genuinely new law-1 surface the wave added overstates its own claim. | CONFIRMED (code review, LOW — "worth more than its LOW label") | **FIXED `0a1acc5`** — three-way partition keyed on the porcelain index column (wholly staged / never staged / partly staged), doc-comment brought into agreement, `--porcelain -z` adopted after **empirically disproving `core.quotePath=false`** on git 2.53. |
| 6 | **A refusing `--dry-run` drops the `carried-over` discriminator.** | **NOT-A-DEFECT — closed.** The M47 Settle names this exact behaviour as the defect it was fixing; four design docs and `carryover_gate.rs` already carry the semantic; the discriminator survives in `findings[0].key.target`. | Closed with evidence, no code change. |
| 7–8 | Two further code-review LOWs (the `brief`→`prd` role-rename note; a test-only `pub` const). | LOW | **NOT FIXED** — queued at the crash. |

## 4.3 The git-path class — the cascade nobody predicted

The most consequential discovery of the whole session, and it came out of the LOW at #5. The class:
**a git reader that parses display-quoted output and keys a decision on the resulting string.** Git quotes paths
containing spaces, quotes or non-ASCII **regardless of `core.quotePath`** (empirically disproved on git 2.53).
Reachability is ordinary: `docs-root` is operator-configurable, owner-artifact paths are author-chosen, fan-out
worktrees hold arbitrary user code — **a filename with a space is enough.**

**Fixed (`7cbf22d`)** — one porcelain parser at `crates/cli/src/git_status.rs` (`--porcelain -z`, verbatim
paths, `Untracked::{Normal,All}` knob); `milestone.rs`'s copy deleted, both `task.rs` seams rewired; the `XY`
fixture consolidated; two *partner* readers (`git_changed_paths`, `git_commit_name_status`) moved to `-z` in the
same commit because they are compared by exact string. Red-before evidence, all live:

* **`jigc rename` refused a clean tree** — `cannot rename with a dirty working tree: "notes, draft.md"` (one
  file split into two phantom paths).
* **The finalize base-overlap guard did not fire at all** on a dirty `src/has space.txt` the moved base also
  touched — silently re-pinned and fell through to validation.
* **The `--dry-run --format json` forecast** emitted quoted, octal-escaped non-paths.

**Then three siblings (`ae2f61b`, `c5cee76`, `b415c67`), two of them worse than reported:**

* `task::git_commit_files` — not "an advisory re-fires forever" but a **store-scope false green**: `post_commit`
  re-baselines from HEAD, `git show` fails on the escaped path, the record keeps the *worktree* hash which
  matches on disk, so **`jigc validate` returned an empty findings array on a managed doc whose HEAD and
  worktree genuinely diverged.** The *"out-of-band edits are detected and routed"* invariant silently off.
* `task::git_untracked_all` — **`jigc task finalize` exited 0 and landed a commit** over a gitignored
  owner-artifact where the ASCII sibling exits 3. The gate passed the state it exists to block.
* `combine::block_set` — the join collision set named a path the repo does not contain.

**Still open at the crash — four verified members, dispatched, never fixed:**

| Site | Consequence |
|---|---|
| `crates/cli/src/milestone.rs:964 record_only_range` | `is_record_path` fails on the quoted string → **finalize blocks where M39 says it should advance**; a wrong verdict on a legal repo state |
| `crates/cli/src/milestone.rs:3248 milestone_landed_summary` | the un-swept **milestone axis of the class `7cbf22d` already fixed on the task side**; a contract-pinned manifest names a path the repo does not contain |
| `crates/cli/src/migrate_corpus.rs:463` | `holds_unlanded_migration` flips — **the premise M47's own increment-3 fix (`0616516`) rests on** |
| `crates/cli/src/rename.rs:474 scan_prose_mentions` | the `jigc rename` prose-mention advisory silently under-reports |

Against **seven exemptions, each justified with a live probe** (incl. `git worktree list --porcelain`, verified
*not* to quote): `task::git_untracked` · `milestone::worktree_porcelain` · `milestone::registered_worktrees` ·
`worktrees_have_staged_code`/`worktree_staged_file_count` · `RecordPreImage::capture` ·
`ingest::git_candidates`/`orphan::committed_markdown` (already `-z`, marginal trimming hazard) ·
`setup.rs`'s emitted hook shell. The orchestrator's stated closure criterion: *"every git-path reader is now
either fixed, queued, or exempt-with-reason. After these four the grep is exhausted, so the class closes by
construction."*

The orchestrator's own summary of what this made the wave, worth preserving verbatim:

> the completion audit found 8 findings, 4 of them LOW. The *cascade* from one LOW — a warning that
> over-counted discarded files — has so far produced a store-scope false green on `jigc validate`, a `finalize`
> that exits 0 over an artifact its gate exists to block, a `rename` that refuses a clean tree, and a
> base-overlap guard that didn't fire at all. **None of those were in any audit report.** They came out of
> asking "what else is on this axis."

---

# 5. The test system

There was no single new "testing system"; there were **six named properties with per-cell acceptance, plus a
substrate of goldens/fences/pinned-facts inherited from M45 and extended here**. That combination is almost
certainly what is remembered as "a huge testing system."

## 5.1 The property census (the spine)

Produced as a *required output* of the capabilities gap-detector — "since M45's whole lesson was that a fix is
complete over its class's axis." Six properties, each written as *property → paths → acceptance*:

* **P1 — "every committing door survives a hook rejection."** Axis: **9 production doors + 1 excluded by
  recorded design** (`jigc setup`'s `--no-verify` install commit), derived from
  `crates/cli/tests/hook_output_axis.rs:11-40` — which turned out to be a **doc comment, not a machine-readable
  list**, so the const had to be minted in increment 3 ("one list, two consumers"). Only 2 of 9 doors were
  survivable at baseline. Per-door acceptance = a rejecting-hook arm; for the four bricking doors,
  rejecting-hook → retry → succeeds. **Un-enumerated sibling added:** *the door leaves no residue that poisons
  a sibling door* (two rejected `milestone create` runs leave two record `.md` files staged, which trip the next
  task's carryover gate). Shipped as `crates/cli/tests/commit_rejection_axis.rs`, whose recovery arms
  **execute the emitted re-run line verbatim**.
* **P2 — "every write verb that misses an item routes to a followable containing section."** 7 cells, 2 correct.
  Per-cell acceptance: assert the emitted `code`, and assert the route's argv contains the **real** address and
  **real** task id — *no `<…>` substring*.
* **P3 — "no surface promises a preview it does not deliver."** 9 surfaces, with blast radii attached
  (342 goldens on the promise line). Item 4 is the trap: the pack-load fence checks *presence of a constraint
  statement*, not its content.
* **P4 — "every leaf verb honours `--format json` on the success path."** 44 leaf verbs enumerated **from the
  clap tree, not a hand list**; 43 honour, `task diff` does not. `command-output-contract.md:300` already
  specifies the shape of the suite; it did not exist.
* **P5 — "a write at an undeclared address is rejected, not silently written."** 9 cells, **5 unguarded**; the
  correct guard shape already ships on the `--unset` siblings.
* **P6 — "a finding a driver reads back is followable" (new).** Route *parseability* is fenced
  (`route_fence.rs`, debug-only); route **followability** is not. Rule: *if a placeholder's value is derivable
  from the finding's own `key.target`, it must be substituted.* Seam pinned at review to the
  **finding-serialization** seam; `<task-id>` declared **not derivable** (it later became the discriminator a
  fixer used: `is_unenriched_not_present` keys on the `<task-id>` span).

## 5.2 The golden / snapshot substrate (measured, not assumed)

* **The compose-golden tree: 612 cells**, arithmetic independently re-derived —
  `(33 workflows × 2 surfaces + 15 doctypes × 2 + 4 front-door + describe + agent-md) × 6 fixture states =
  102 × 6 = 612`. Regen is one step (`UPDATE_GOLDENS=1 cargo test -p cli --test compose_goldens`, **8.7 s**,
  zero-diff verified); **CI refuses regen**. Critical operational fact: the suite is one `#[test]` per fixture
  state and each panics on its *first* differing cell, so **red-golden output does not reveal blast radius** —
  only `UPDATE_GOLDENS=1` + `git diff --name-only` does.
* **139 inline `insta` sites**; `cargo insta test --accept` regen exercised (with a noted wart: it rewrites
  `@r"` → `@"`).
* **Five verbatim-literal assertions with no regen path** + the whole-script `precommit_hook_body_golden`.
  Measured: one word changed in the dev finalize step ⇒ **156 goldens moved, 7 tests red**, `cargo insta` fixed
  only 2.
* **`cargo test` fails fast** — the same edit reported `1 failed` when the truth was 7. Every fixer in the
  session was instructed **"use `cargo test --no-fail-fast`, never bare `cargo test`."**
* The wave's own regen landed in increment 11: **438 changed goldens**, verified by the code reviewer to carry
  *real new content rather than widened admissions*, plus a **twelve-golden** blast radius for the rc.10 bump.

## 5.3 Instruction-drift / "does the prose still say the thing" checking

This is the part with genuinely new machinery:

* **The named-fact guard (Decision 12, commit `3a35430` — "the schema seam and its copy-in declaration brick
  apart both ways").** Both stated-at fences were **presence-only and broken live** in two directions: delete
  590 chars of copy-in/append contract prose but keep the front-matter code → clean build, clean pack load,
  contract gone; delete the `{{schema:}}` ref → the *obligation itself* disappears while the composed text still
  says the payload "follows:" with nothing following. The guard is a **`constraint-code → required-token(s)` map
  checked at pack-load**, scoped (at review) to *every step carrying a `states-constraints:` code, in both
  packs* — the only shape that stays inside the A-3 "presence never content" bound. The e2e drove "the named-fact
  guard over every declared fact plus the `{{schema:}}` biconditional and the unmapped-code arm."
* **The two-`finalize.yaml` parity problem (S1)** — independently maintained, substantively different, unfenced.
  The parity must be over **contract facts, never a field list** (the two `commit` schemas differ; dev declares
  `implements: ref → spec`).
* **The drift-hook acceptance** drives the **real rendered hook body** against **real mixed-severity validate
  reports**, with two negative controls — *"behaviour, not field order — so a renderer or serde change reddens
  it."*
* **The registry-vs-producer-set test**, derived from the same 9-door axis as the frame sweep, so the
  `debug_assert!` compiled out in release cannot hide a missing member.

## 5.4 Pinned facts and flow 47

* `crates/cli/tests/pinned_facts/` (5 modules at baseline: `address_grammar`,
  `file_state_transactional`, `finalize_json`, `finding_emission_order`, `form_vision_grounding`) — the M45
  repro-block conversions. Increment 11 added `a08e4d0 test(cli): the three refuted trial claims become
  standing pinned facts` and `24eb5f1 docs(m47): every verified fact names its standing test, or says why it
  has none`.
* **Flow 47** (`ad9253a`): *"the wave's composite done-picture, **seven axis-iterating arms**"* — all seven
  passed through the real binary at the audit.
* New test files created during the fix rounds: `crates/cli/tests/doc_show_miss_axis.rs` (9 causes × 2 formats ×
  2 arms), `crates/cli/tests/fanout_abort_recovery.rs` (3 rejection causes + the landed-teardown arms),
  `crates/cli/tests/porcelain_path_shapes.rs` (13 porcelain cells × 5 path shapes, asserting emitted paths
  **exist on disk** — "a claim about reality, not a byte pin"), and the shared `git_status::fixture`.

---

# 6. Rebuild recommendations

## 6.1 What can be re-planned mechanically (no human needed)

The planning phase is **fully recoverable from this extraction**, and should be restored *before* any code:

1. **Restore the two artifacts byte-exact** from `recovery/m47/files/` to
   `completions/artifacts/M47/{baseline.md,planning-gate-record.md}`. They are the durable ledger every decision
   cites (N-labels, S-facts, the five enumerations). Note baseline §7 records the `2383/2` prediction and its
   **halt condition** — keep it, it fired correctly.
2. **Restore the four `DECISIONS.md` entries byte-exact** from `patches/0002`, `0007`, `0015` (planning) and
   `0004` (the Inc-3 halt resolution). They are the record of *why*, written — per the human's explicit
   instruction — for a reader with no context.
3. **Restore the `decisions-pending.md` blocks** from `patches/0005` (entries 10 + 11, the two M46 deferrals)
   and `0009` (the graduated M47 charter + the manifest-hash-fence deferral + the demand-counter re-count).
4. **Re-cut the eleven-increment decomposition.** This is the one planning artifact that is *not* recoverable —
   only six fragments survive (`patches/0003`, `0010`–`0013`, `0016`). But it is **mechanically re-derivable**:
   the gate-record's 19-row scope table + the Settle's 14 decisions + §1.8's reconstruction table give the
   content; the ordering rules are recorded explicitly (one-way doors first · **N1 before H2 on 4 of 9 doors** ·
   every surface-changing cluster before any golden regen, which lands only in increment 11). **Increments 5 and
   8 are unrecovered** — re-derive them from the gate-record rows not otherwise assigned (candidates: N6's
   `step:finalize` solicit/commit split; N7's resume alignment; `task diff --format json`; the N10 trailer arms).
   Fold the Inc-3 T0 amendment (`patches/0003`) back **inside** increment 3's paragraph — the milestone-reader
   enumerates increments from headings, so a new bolded entry would renumber the wave.
5. **Re-apply the four cross-model corrections** verbatim (`patches/0010`–`0013`, `0016`): the scoped successor
   rule + genesis exemption; the `--carry-staged`-on-validate parity; the specified `blocking_probes` regex and
   its two negative controls; the producer-set-from-the-9-door-axis condition.

## 6.2 Build order

Rebuild in the original increment order — it was risk-first and its dependencies are real:

1. **Increment 1 first** (both one-way doors in one coordinated commit). Its halt condition is a genuine,
   already-validated oracle: **exactly two failures**, at `crates/engine/src/manifest.rs:400` and
   `crates/cli/tests/freeze_enforcement.rs:232`. A third failure or a different pair means the projection erased
   something the freeze needs — stop. It fired exactly as forecast last time (2390/2).
2. **Increment 2 (N1) before increment 3 (H2)** — hard prerequisite on 4 of 9 doors.
3. **Increment 3 with T0 already in scope** — do not re-discover the halt. The full T0 spec survives in
   `patches/0003` (four parts: the abort-arm teardown removal · the zero-contribution refusal + manifest
   statement · the landed-teardown law-1 warning · the record pathspec as the fifth capture axis), including the
   inverted test, the three rejection causes the acceptance must iterate, and the five doc revisions owed.
4. **Increments 4–10** as cut; **increment 11 last** (goldens regenerate only here — 438 files, so any earlier
   regen is churn, not blast radius).
5. **Then fold in the twelve post-audit fixes as first-class work, not as a fix round.** They are known-good,
   axis-swept, and two of them (the fresh-clone reseed; the git-path parser consolidation) closed defects that
   are *older than M47*. In particular: **build `crates/cli/src/git_status.rs` early** — a single
   `--porcelain -z` parser plus a shared `XY` × path-shape fixture, then rewire every reader onto it — because
   that consolidation is what made the sibling sweep mechanical last time. The fixers' own advice, verbatim:
   *"the reason this bug shipped twice is that the parse was written twice."*
6. **Finish what the crash interrupted:** the four remaining git-path members (`record_only_range` ·
   `milestone_landed_summary` · `migrate_corpus.rs:463` · `scan_prose_mentions`), the three LOW hygiene items
   (incl. `doc create --help`'s caps-only slug sentence), then **rebuild + reinstall rc.10, re-run the full gate
   and a fresh e2e**, write `completions/artifacts/M47/VERDICT.md`, and only then run G1–G3.

**Build-wide rules that will bite if missed** (all measured, all recorded): `cargo test --no-fail-fast` always ·
goldens regenerate only in increment 11 · the baseline is **2385 / 0** at `f4a6a2b` and was **2552 / 0** at the
crash · increment 4 carries the largest single radius (342 of 612 goldens) · a tier-2 finalize-prose fix must be
applied to **both** `finalize.yaml` files or it lands on half the surface · don't run six capability-auditors
concurrently against one `target/` (the `include_dir!` pack-baking artifact).

## 6.3 What genuinely needs the human

1. **The `add-from-spec` boundary crossing.** The fix (`9a974ff`) made `milestone add-from-spec` **resumable** —
   a change to what a verb *does*, retiring a bound `design/team-ready-state.md` recorded as *"a capability M47
   does not charter."* The orchestrator judged the override correct (the bound was argued against the charter
   and never against the bytes the door printed) and surfaced it twice; the human never answered. **This is the
   one open decision of record.** Reverting is cheap before the trial and expensive after.
2. **Whether the four unfixed git-path members and the three LOW items ship in rc.10 or route to M46.** The
   recorded stance is fix-now (two are gate false-greens), but the wave was never formally closed.
3. **Whether increments 5 and 8, once re-derived, match the lost cut** — a scope confirmation, not a design
   question.
4. **The 1.0.0 call itself**, unchanged: rc.10 → the blind greenfield trial → the call is the human's, and per
   the recorded answer, *"if need to be we will open an other milestone after this one."*
5. **The two M46 deferrals minted mid-build** (the durable "was provisioned" fact; the `discard`-style
   dirty-worktree guard on finalize's landed teardown) carry triggers keyed to the M46 Settle — they are
   recorded, not open, but they *presume* M47's substitutes shipped. If the rebuild drops either substitute,
   both entries need re-pricing.

## 6.4 Two things worth preserving from this arc regardless

* **The cross-model review earned its cost twice over** — two of its three blockers were self-contradictions the
  same-model pass wrote and then reviewed without noticing, in one paragraph six lines apart. The third was
  *refuted on test*, which is the discipline that makes the pass usable rather than authoritative.
* **The "what else is on this axis" cascade** turned one LOW warning-accuracy finding into four
  gate-level correctness fixes, none of which appeared in any audit report. If the rebuild reproduces only the
  reported defects, it reproduces the wave at half strength.
