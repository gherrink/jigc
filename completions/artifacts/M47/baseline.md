# M47 planning — the verified baseline (2026-07-26)

The **Scope-phase capability ledger and gap census** for the rc.10 wave, established by six
`capability-auditor`s + four `gap-detector`s + three independent `robust-advocate`s **exercising the
real binary** at HEAD `f4a6a2b` / installed `jigc 1.0.0-rc.9`. Persisted per the pre-decompose
review's B1: the Settle's decisions cite these labels, so the labels must be durable and checkable.

The decisions this baseline fed are in [DECISIONS.md](../../../DECISIONS.md) → 2026-07-26 *M47
planning: the Settle* (+ its review addendum). The field evidence upstream of it is
[RC-alpha4/findings-verification.md](../RC-alpha4/findings-verification.md) (`P*`/`H*` labels).

> **Corrections applied after the independent pre-decompose review** are marked **[R]**. Two of them
> reverse a claim the first pass asserted; they are corrected in place rather than appended, and
> listed together in §6 so the correction is auditable.

---

## 1. The done-picture

**What M47 proves:** that jigc's own surfaces tell the truth about jigc — a blind agent driving the
tool from a cold start is carried by what the tool *says*, not by what a human already knows.

**Boundary:** the wave changes *what existing surfaces say*, never *what surfaces exist* — bent by
explicit human decision for the greenfield-path five, because leaving them means the acceptance trial
burns its probes on known defects.

**Acceptance, two bars.** (1) In-repo: **flow 47** through the real binary + per-fix axis tests + a
clean completion audit + the four-command gate. (2) Out-of-repo: **1.0.0-rc.10** built + installed,
then a **three-probe blind greenfield trial** — G1 cold start (rejecting hook under `core.hooksPath`
+ pre-staged-before-mint plants) · G2 the design altitude from zero · G3 corpus accretion from
nothing. **The trial is a report, not a second close.**

**Strategic-claim check (verified).** The only greenfield trial on record is `RC-greenfield` on
**rc.2** (2026-07-06), and it was **human-driven from `jigc setup` onward, not blind**. G1–G3 would be
the **first blind greenfield sessions ever run**; every blind-session protocol and finding to date
came from adoption corpora.

---

## 2. Charter premises corrected by the baseline

1. **H1's "third arm" is dead.** `finalize --dry-run` halts at `finalize.empty-commit` in the exact
   state an agent previews from (commit doc filled, nothing staged) — `plan_finalize`
   (`crates/engine/src/finalize.rs:186-270`) is a sequential abort-chain, not a gate collector.
2. **The promise is on nine surfaces, not four** — the charter's list missed the highest-traffic one,
   `crates/cli/src/render.rs:266` (the `what's-left:` line on every id-carrying compose). Full list §4.
3. **No construction makes the promise literally true** — 2 of 14 gate rows are unpreviewable in
   principle, 2 are *wrong* at validate, 1 false-positives. Prose-scoping is forced in every arm.
4. **P5-4's stated premise is false.** The "field-group-absent miss gets the good route" case is not a
   miss — it **succeeds** (`insert_item_field` materialises the sentinel + bullet,
   `crates/engine/src/write.rs:3215-3249`). The real split is `--value` vs `--unset` on one address.
5. **`task diff` falsifies a recorded clean confirmation** and is three nonconformances, incl. a lie in
   its own generated `--help` and a zero-byte-both-streams empty state at exit 0.
6. **Tier 3's framing is half wrong** — the non-empty-`left_out` **stdout** arm is pinned
   (`crates/cli/tests/finalize_manifest.rs:313`); the **stderr** arm is not.
7. **P3-2 narrows** — the append prose is byte-verified **accurate**; the defect is the *collision* case.
8. **Golden count: 612 on disk.** `completions/artifacts/M45/VERDICT.md:23` says 558;
   `implementation/pinning.md:17` says ~1500 and `:23` says 612 — the doc contradicts itself.

---

## 3. The defect census (N1–N22)

### 3a · Integrity tier

- **N1 · The four milestone record-only doors brick permanently on one rejected commit.**
  `milestone create` / `add-task` / `add-from-spec` / `discard` seed the file-state baseline **after**
  the commit (`crates/cli/src/milestone.rs:362`, `:750`, `:1257`) while every later milestone write
  runs `reconcile_record_preflight` (`milestone.rs:470`) **before** it → `reconciliation.conflict-block`.
  Every escape blocks, incl. `discard --force` and a raw `git commit` (the poison is the gitignored
  baseline, not the index). Recovery required hand-editing `.jigc/state/file-state.json`.
  **Compounding:** rejected runs leave record `.md` files **staged**, which then trip the *next*
  unrelated task's carryover gate. **Zero hook-rejection coverage on all four doors.**
- **N2 · H2's dead end splits per clone.** After a rejected `migrate-corpus` commit the operator's
  `jigc validate` is clean exit 0 while a **fresh clone blocks** — and the teammate's route points back
  at the door that dead-ends. Silent until someone clones.
- **N3 · `set-field` / `doc author` at an undeclared *item field*: exit 0 + positive ack, then the doc
  is dead.** `set_item_field_or_insert` (`crates/engine/src/write.rs:3275-3279`) short-circuits on
  `None` and writes anyway — no `write.unknown-field` guard, no reparse. Afterwards every read is
  `store.unparseable` with a *human* route the adapter rule forbids following; only `jigc task discard`
  (the whole task) recovers. The `--unset` sibling has **both** guard and reparse (`:5726-5734`).
  Reproduced top-level, nested, and through batch `doc author`; **re-reproduced end-to-end at review**.
- **N4 · `set-slot` at an undeclared *leaf* of a single-slot item writes to the real slot and the ack
  names the phantom leaf**, exit 0. `ParsedItem::slot_span` (`crates/engine/src/parse.rs:156-165`)
  ignores `leaf_id` when `slots.is_empty()`. Law-1 lie on the **success** path. Multi-slot arm
  **UNVERIFIED** — no reachable multi-slot repeatable in the dev pack.
- **N5 · The slug drops the leading *and trailing* component of a hyphenated compound.**
  `"On-call handoff artifact"` → `call-handoff-artifact`; `"Telemetry consent opt-in"` →
  `telemetry-consent-opt`. `crates/engine/src/slug.rs:114` `EDGE_STOPWORDS`; the module doc at `:23-25`
  **promises** medial stopwords are untouched. Mints a **wrong frozen identity**.

### 3b · Greenfield-path tier (on the G1/G2/G3 routes; human ruled IN)

- **N6 · Code-less dev workflows compose a finalize their own gate refuses.** They block on
  `commit:<id>#header/type` + `#summary` — leaves their composed text never names. Root cause: the
  **dev** pack sites the commit-doc writes in `step:implement` (`crates/cli/pack/steps/implement.yaml:12-27`),
  the *code-writing* step; the **methodology** pack sites them in `packs/methodology/steps/finalize.yaml`.
  **[R] The count is four, not five** — `architecture-documentation`, `plan`, `record-change`,
  `record-decision` reach `step:finalize` with no code-writing step; `project-setup` reaches it through
  `project-finalize.yaml` (which is *only* that include) and is code-less too, so it is a fifth by a
  different path. **[R] The include graph:** `migration-finalize.yaml` in **both** packs ends with
  `{{ include: step:finalize }}`, so all **12** migrate workflows inherit whatever `step:finalize`
  solicits. **[R] Pre-existing:** methodology's `finalize.yaml` already solicits, so its 7 migrate
  workflows double-solicit at HEAD today.
- **N7 · The advertised `resume:` door dead-ends on any HEAD advance.** `jigc start --task`
  (`crates/cli/src/start.rs:1706-1717`) blanket-refuses base inequality; `finalize`'s guard is
  **overlap-aware**; `task validate` has **none**. Verified in one repo state: resume exit 1, validate
  exit 0, **finalize landed**. The read-only door is stricter than the commit door. Route offers only
  destructive exits. `form-vision` **mandates** the broken step; the compose footer's *"you can run
  them in parallel"* is false the moment one lands.
- **N8 · The pre-commit hook reports drift on every commit, forever, when none exists.**
  `crates/cli/src/setup.rs:194-196` greps for `"probe": "doc-code"` with **no severity filter**; its own
  comment says *"Warn IFF … a doc-code CONTENT finding"*. Any non-Rust project following
  `implement-from-spec` as instructed acquires a permanent false "drift detected".
- **N9 · `single-task`'s changelog step cannot record a change, and `describe` cites it falsely.**
  `crates/cli/pack/steps/record-changelog.yaml` is four lines naming only `doc create`, over a
  **nested repeatable**; following it verbatim yields an empty changelog that **validates clean**.
  `record-change.yaml:8`'s suppression reason (projected by `describe`) claims routine recording
  *"already rides"* that step.
- **N10 · The store-scope trailer claims advisories gate.** It says *"these gate at `jigc task validate`
  / `jigc task finalize`"* while `task validate` reports clean at exit 0 on the same findings; it
  classifies by **probe family**, not severity. **Second arm:** it names only the two *task* doors while
  the milestone doors gate on the same findings.

### 3c · Surface tier

- **N11 ·** `retitle-item` at a nonexistent item is P5-4's **un-named axis sibling** (`write.rs:2037-2051`,
  same `write.wrong-shape` + generic route; pinned at `crates/cli/tests/exit_codes.rs:546`).
- **N12 ·** `set-field --unset` emits `write.not-present` with the route's **literal `<address>` /
  `<task-id>`** — enrichment is wired at only two call sites (`crates/cli/src/doc.rs:932`, `:1376`).
  This makes `DECISIONS.md`'s recorded "verified base" (`set-field` never emits `not-present`)
  under-counted — it has since M41.
- **N13 ·** `jigc milestone create` commits silently — no sha, no path, no next step, while `jigc setup`
  names its install commit. It also moves HEAD, compounding N7.
- **N14 ·** `<<author: brief#vision>>` in `project-setup` (`crates/cli/pack/steps/author-prd.yaml:6-7`)
  renders a task *role* where every other workflow renders a doc address; `jigc doc schema brief` →
  `store.unknown-type`, exit 1.
- **N15 ·** `Create::task`, a Rust type name, leaks into `jigc doc author --help`.
- **N16 ·** **[R]** `arch-doc` *does* print `*` markers (as do `spec`, `prd`, `changelog`, `commit`);
  only `adr` prints none. The defect is that the legend header says `fields (…)` while the markers land
  on **item leaves** under `sections:`.
- **N17 ·** `finalize.empty-commit`'s route omits the abandon exit (`jigc task discard`).
- **N18 ·** `schema-conformance.repeatable-populated` false-alarms on a young corpus (a greenfield
  changelog with unreleased changes and no cut release is *correct*). Fix is a one-token knob addition
  — prior art settled it (`design/validation.md:527` rejected a schema `min-items` knob as a frozen
  trap) — but the default is **duplicated** in both `knobs.yaml` with **no test enforcing the mirror**.
- **N19 ·** `jigc ingest`'s `needs-reconcile` route is weaker than `validate`'s for the same file
  (prose vs the mechanical `jigc migrate <path> --as adr`).
- **N20 ·** `jigc milestone create <TITLE>` is positional while `jigc doc create --title` is a flag.
- **N21 ·** `relocate --format json`'s success arm has no standing test (reachable only over a
  manifest-less pack).
- **N22 ·** `finalize.base-mismatch` mixes short and full 40-char sha forms in one message.

---

## 4. The enumerations the decisions rest on

### 4a · The nine promise-making surfaces (H1)

1. `crates/cli/src/render.rs:266` — the composed `what's-left:` line (**every** id-carrying compose)
2. `crates/cli/pack/steps/finalize.yaml:12-13` (dev)
3. `packs/methodology/steps/finalize.yaml:36-37`
4. `QUICKSTART.md:138`
5. `crates/cli/src/cli.rs:1315` (the `task status` unknown-subcommand tip)
6. `design/validation.md:576` — stated as an **invariant** ("can never diverge")
7. `design/write-commands.md:195` — stated as an **invariant**
8. `design/finalize.md:41` — invariant + a declared owner-artifact exception at `:43`
9. `design/command-output-contract.md:325` — the counter-statement, **scoped to store scope**

Adjacent, adjudicated separately: `VISION.md:120` (**no change** — asserts no invariant; a VISION edit
stays a flagged event) · `design/bootstrap.md:71` + `crates/cli/pack/config/commands.yaml:37` +
`design/command-catalog.md:65` (illustrative/hint mirrors) · `design/reconciliation.md:131` and
`design/write-commands.md:32` (the two extra doc sites the first pass missed) ·
`crates/cli/src/task.rs:354-355` (a doc comment, not user-facing).

### 4b · The 14 gate rows finalize runs that validate does not

`finalize.milestone-sub-task` · `finalize.base-mismatch` · `changelog-recording.gate-granted-unused` ·
`finalize.empty-commit` / `nothing-staged` · `finalize.render-io` · `finalize.promote-io` ·
`finalize.promote-clobber` (+ `provenance-io`, `source-path-io`) · `finalize.migration-no-replacement` ·
**`finalize.carried-staged`** · the migration review hold (exit 4) · the left-out/carried advisories ·
**`owner-artifact.present`** · `finalize.stage-failed` · the commit/hook rejection.

**Settled disposition:** carryover **in**; owner-artifact causes 1–6 **in**; cause 7 (untracked)
**stays post-stage**; `stage-failed` + hook rejection **unpreviewable in principle**; `empty-commit` /
`nothing-staged` **wrong at validate**; `promote-clobber` + `base-mismatch` previewable but
**deliberately out of scope** (beyond the field-proven need).

### 4c · `owned_location_violation`'s seven causes (`crates/engine/src/validate.rs:1927-1988`)

empty · absolute · contains `..` · **not-under-home** · **names-no-file** · symlink-escape ·
**untracked**. Only the last consults `tracked`. The trial's two blocks were causes **4 and 5**.
Already pinned as correct under `tracked = true` by `validate.rs:5696` and `:5849`.

### 4d · The committing-door axis — 9 production doors + 1 excluded

`task finalize` (IndexHonoring) · `task finalize` (MigrationFixed) · `milestone finalize` (combine) ·
`milestone finalize` (chain, per sub-task) · `milestone create` · `milestone add-task` ·
`milestone add-from-spec` · `milestone discard` · `migrate-corpus`. **Excluded with reason:**
`jigc setup`'s install commit is `--no-verify` by recorded design (`crates/cli/src/setup.rs:983-987`).
**Only 2 of 9 are survivable today.** The same axis is already enumerated for the *non-blocking* case
at `crates/cli/tests/hook_output_axis.rs:11-40`, with a working harness.

### 4e · The write-verb × miss-shape matrix (P5-4) — 7 cells, 2 correct

| verb | miss | code | route |
|---|---|---|---|
| `set-slot` | missing item | `write.not-present` | ✅ enriched |
| `remove-item` | missing item | `write.not-present` | ✅ enriched |
| `set-field --value` | missing item | `write.wrong-shape` | ❌ literal `<doctype>` |
| `set-field --unset` | missing item, declared field | `write.not-present` | ❌ literal `<address>`/`<task-id>` |
| `set-field --unset` | missing item, undeclared field | `write.unknown-field` | ❌ `<doctype>`; also a **wrong diagnosis** |
| `retitle-item` | missing item | `write.wrong-shape` | ❌ `<doctype>` |
| `add-item` | missing section | `write.unknown-section` | ❌ `<doctype>` |

Placeholder siblings outside the write family: `write.non-reparseable`,
`finalize.migration-no-replacement`, `reconciliation.conflict-block`. They pass the route fence only
because these are declared `DUMMY_SUBSTITUTIONS` members (`crates/cli/src/route_fence.rs:32`) — **the
fence proves parseability, never followability.**

---

## 5. Structural build facts (S1–S16)

- **S1 ·** The two `finalize.yaml` files are independently maintained, substantively different, and
  **unfenced against each other**; include resolution is origin-pack-scoped. Charter item P2-1 **is**
  this bug.
- **S2 ·** Both stated-at fences are **presence-only, and the code says so**
  (`crates/cli/src/pack.rs:378-380`, `:468-470`). **Broken live:** 590 chars of copy-in/append contract
  prose deleted with the front-matter code kept → clean build, clean pack load, contract gone; and
  deleting a `{{schema:}}` ref removes the obligation *itself* while the composed text still says the
  payload "follows:" with nothing following.
- **S3 ·** Five verbatim-literal assertions have **no regen path** (`crates/cli/tests/config_fill.rs:200-215`,
  `multi_pack_acceptance.rs:456-471` and `:515,549-555`, `start_compose.rs:161-176` and `:733`) plus the
  whole-script `crates/cli/src/setup.rs:1624 precommit_hook_body_golden`. Measured: one word changed in
  the dev finalize step ⇒ **156 goldens moved, 7 tests red**, `cargo insta` fixed 2.
- **S4 ·** Blast radii (measured): dev `finalize.yaml` **156** · methodology `finalize.yaml` **180** ·
  the validate-promise line **342** · `create-gates:` footer **324** · `adapter.rs:1086` **6** + 2 unit
  tests · `render.rs:1517` **0 goldens / exactly 1 test** · a `when:` reword **30** ·
  `usage:`/`description:` **6** each. Compose-golden regen: one step, **8.7 s**, zero-diff verified;
  CI refuses regen. **Red-golden output does not reveal blast radius** (6 tests, each panicking on its
  first differing cell).
- **S5 ·** **`cargo test` FAILS FAST** — a pack-prose edit reported `1 failed` while the true count was
  7. Prose batches must run `--no-fail-fast` (~201 s). Gate warm: build ~0.2–1.9 s · test **201 s** ·
  clippy 5.2 s · fmt 1.2 s. **Baseline 2385 passed / 0 failed** across 241 test binaries.
- **S6 ·** **Nothing consumes the `wrong-shape`-vs-`not-present` distinction** except the route-selection
  guard at `crates/cli/src/doc.rs:673` and the tests that pin it — no pack YAML, no adapter, no
  `AMBUSH_CLASS_CODES`, no `CHECK_INVENTORY`, no severity knob, no registry, no golden. But
  `GenerateError` has **no `NotPresent` variant**, so the flip needs a new variant + a
  `generate_error_finding` arm + enrichment at `run_set_field` **and** `run_unset_field`.
- **S7 ·** P1-7 is a **seam plumb, not a reword**: `crates/cli/src/setup.rs:480 resolve_hooks_dir()`
  already computes the truth and is already called at `:271`; `SetupSummary` (`:648-658`) carries no
  hook-path field.
- **S8 ·** An `error_code` is **log-only/additive by declaration** (`crates/cli/src/invocation_log.rs:47-49`)
  — no envelope change, no contract-version bump. Four touch points, one of which
  (`registry_mirrors_the_declared_members`, `:314-321`) hard-fails. The constructor membership check is
  a **`debug_assert!`**, compiled out in release.
- **S9 ·** `--format` is a **global** clap arg (`crates/cli/src/cli.rs:37-38`), so all **44** leaf verbs
  accept it; 43 honour it, `task diff` does not. The fence `adapter.rs:1539-1551` asserts only that it
  **is global** — it stays green if every verb ignores it.
- **S10 ·** `task diff`'s envelope needs **no new data**: base pin (`crates/cli/src/task.rs:662-667`),
  diff string (`:2978-2991`), staged docs (`:1694-1725`, already sorted) are all in hand at `run_diff`
  (`:326-351`). The bug is a dropped parameter at the dispatch arm `:255`.
- **S11 ·** Two tier-2 routes are `RouteKind::Human` and sit **outside** the argv fence though both
  quote real argvs: `crates/engine/src/store.rs:381-388`, `crates/engine/src/target_surface.rs:465-478`.
  The route fence itself is **debug-only** (`crates/engine/src/finding.rs:508-540`).
- **S12 ·** Pack-load fences a wording edit can trip: `when:` shape (one line, no trailing period,
  ≤120 chars — `crates/cli/src/pack.rs:524-532`) · both stated-at presence fences (`:381`, `:471`) ·
  `Route::mechanical` argv parse. Everything else in tier 2 is **unfenced judgment**.
- **S13 ·** `pinned_facts/` mechanics: add a file under `crates/cli/tests/pinned_facts/`, register it in
  `crates/cli/tests/pinned_facts.rs` with an explicit `#[path]` attr (**no `mod.rs`**), add a row to
  that file's provenance ledger, fill `pinned-by:` in the source repro block. ~120–160 lines if it rides
  an existing `trial_corpus.rs` `State`; a new state ripples into every state-iterating suite.
- **S14 ·** The gate census (§4b) and the committing-door axis (§4d). `CommitRejected` is **private**
  (`crates/cli/src/task.rs:2558`); `render::commit_rejected` (`crates/cli/src/render.rs:945`) hardcodes
  the task idiom.
- **S15 ·** **A masking test pins the H2 dead end as correct** —
  `crates/cli/tests/corpus_migration.rs:893` asserts only `!status.success()` + verbatim stderr + zero
  commits. **Rewrite, don't extend.**
- **S16 ·** The survivable frame is five parts: git's stdout+stderr **verbatim**, prefixed
  `` `git commit` was rejected (no commit was made): `` · a blank line then a **state-truth statement** ·
  a **re-run instruction naming this door's exact argv** · a **log-only** `error_code` (deliberately not
  a `Finding`, so the route floor never wraps git's bytes) · under `--format json` the whole framed
  string in `{"error": …}`.
- **[R] S17 · Pack precedence is `[dev ▸ methodology]`, dev-highest** (`crates/cli/src/pack.rs:915-919`
  *"dev first = dev-highest"*; `crates/cli/src/pack.rs:1084`; `implementation/pinning.md:17`
  *"`commit` ships in both packs and **dev wins by precedence**"*). **Dev shadows methodology.** The two
  `commit` schemas differ (dev declares `implements: ref → spec`), so a parity guard must assert
  **contract facts**, never a field list.
- **[R] S18 · The finding JSON emits keys alphabetically** — `check, code, key, location, message,
  probe, route, severity` — so `severity` comes **after** `probe`, and the envelope is pretty-printed
  one key per line.

---

## 6. Corrections applied at the pre-decompose review **[R]**

1. **Pack precedence was stated backwards.** Dev shadows methodology, not the reverse (S17).
2. **The finding JSON key order was stated backwards.** Alphabetical: `probe` precedes `severity` (S18)
   — verified by running the real binary. The first pass's claim rested on a synthetic fixture.
3. **N6's count is four-by-one-path plus one-by-another**, and the migrate include graph makes a
   `step:finalize`-only fix structurally impossible without splitting the step (§3b).
4. **N16 was imprecise** — only `adr` prints no `*`; the defect is the legend's placement (§3c).
5. **The baseline itself was not durable** — it lived in a temp scratchpad that was cleaned mid-planning.
   This file is the fix.

---

## 7. Declared bounds

- Area F (greenfield) was **single-operator and non-blind** — the auditor knows the product; a genuinely
  blind agent may stall where it recovered from pattern knowledge.
- Fan-out/join was not driven on greenfield; `park-idea`, `decided-task`, `quick-fix`, `record-dogfood`,
  `completion`, `increment`, `sub-task` were preview-scanned only, for the commit-doc axis.
- `milestone finalize` (chain, per-sub-task) and `add-from-spec` mid-loop rejection are
  **structurally derived, `UNVERIFIED (cost)`** live.
- N4's multi-slot arm is **UNVERIFIED** — no reachable multi-slot repeatable in the dev pack.
- All behavioural evidence is from **debug** builds; debug/release byte parity is UNPINNED by
  declaration (`implementation/pinning.md:70`). This matters for the `debug_assert!` fences (S8, S11).
- `2383/2` (the schema-hash narrowing spike) was reproduced by its author only; it is arithmetically
  consistent with the 2385/0 baseline, and `cargo test` is green at HEAD. **The cross-model review
  correctly declined to treat it as independently verified** (it could confirm every *static* count but
  not a test result). It is a **prediction, not a measurement**, and increment 1 re-measures it rather
  than inheriting it: run `cargo test --no-fail-fast` after `erase_presentation` lands and expect
  **exactly two** failures — `crates/engine/src/manifest.rs:400`
  (`shape_mutated_without_bump_is_hash_mismatch`) and `crates/cli/tests/freeze_enforcement.rs:232`
  (`methodology_schema_shape_drift_without_manifest_bump_is_blocked`), **both of which use a `hint`
  reword as their stand-in for a "shape change"** and are repointed at a real shape drift per the
  `drift_adr_schema` (`location:`) mold eight lines above one of them. **A third failure, or a different
  pair, is a halt** — it means the projection erases something the freeze needs.
