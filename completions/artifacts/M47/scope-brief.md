# M47 — the rc.10 wave (the surface-fundament wave) — SCOPE BRIEF + VERIFIED BASELINE LEDGER

Repo: `/home/maurice/Projects/gherrink-jigc` · HEAD `f4a6a2b` · installed binary `jigc 1.0.0-rc.9`.
This brief is the output of the planning **Scope** phase. Everything below marked VERIFIED was
established by a capability-auditor **exercising the real binary** at this sha, not by reading docs.

---

## 1. The done-picture

**What M47 proves:** that jigc's own surfaces tell the truth about jigc — a blind agent driving the
tool from a cold start is carried by what the tool *says*, not by what a human already knows.

**The boundary (as chartered):** the wave changes *what existing surfaces say*, never *what surfaces
exist*. New verbs/flags/projections route to M46. **Human decision 2026-07-25: the boundary bends for
the acceptance path** — the five greenfield-path defects (§4) are IN even where they need step content
or a behaviour change, because leaving them means the acceptance trial burns its probes on known bugs.

**Human decisions at Scope (2026-07-25):**
- **Take all of it** — no overload valve. M47 carries the full corrected scope, charter + baseline.
- **The greenfield-path five are in.**
- **The trial is a report, not a second close.** M47 closes once → build + install `1.0.0-rc.10` →
  the three-probe blind greenfield trial → then either the 1.0.0 call or a *new* milestone. M47 does
  not hold open awaiting probe findings.
- Tier 2 ships **all ~16 declared items**. The wave ships a numbered **flow 47** acceptance suite.

**Acceptance, two bars:**
1. In-repo: flow 47 through the real binary + per-fix axis tests + clean completion audit + full gate.
2. Out-of-repo: rc.10 installed, then a **three-probe blind greenfield trial** — G1 cold start (with
   the rejecting-hook-under-`core.hooksPath` plant and the pre-staged-before-mint plant) · G2 the
   design altitude from zero (`do-research` → `form-vision` → roadmap → first milestone) · G3 corpus
   accretion from nothing (first changelog entry · spec → implement-from-spec · first arch-doc · a
   hand-authored foreign doc appearing mid-stream for detect-and-route).

**Strategic-claim check (VERIFIED):** the only greenfield trial on record is `RC-greenfield` on
**rc.2** (2026-07-06) and it was **human-driven from `jigc setup` onward, not blind**. So G1–G3 would
be the **first blind greenfield sessions ever run**. Every blind-session protocol and every finding it
produced came from adoption corpora.

---

## 2. Source documents

- Charter: `implementation/decisions-pending.md` → "The rc.10 wave (M47…)" (line ~32) — tiers 1/2/3 + acceptance G1–G3.
- Trial evidence: `completions/artifacts/RC-alpha4/findings-verification.md` (18 CONFIRMED · 6 PARTIAL · 4 REFUTED, every verdict with a repro block) and `trial-record.md` alongside.
- Charter decision: `DECISIONS.md` → 2026-07-25 "The rc.10 wave (M47) is chartered".
- Pinning discipline: `implementation/pinning.md` (§1 goldens · §3 repro-block → test conversion · §4 fixture substrate).
- Contract docs: `design/command-output-contract.md`, `design/surface-contract.md`, `design/validation.md`, `design/finalize.md`, `design/doc-read-surface.md`.

---

## 3. CHARTER PREMISES CORRECTED BY THE BASELINE (all VERIFIED)

1. **H1's fork is mis-posed — a third arm exists.** `jigc task finalize <id> --dry-run` already runs
   every *pre-stage* finalize gate read-only (base-mismatch, empty-commit, promote-clobber, the
   carryover *computation*), exits 3 with the full findings envelope, commits nothing. It is named by
   **no** promise-making surface. Call-path: the `dry_run` branch returns at `crates/cli/src/task.rs:1155-1166`
   before `try_execute_finalize_plan` (`task.rs:1238`), so it is blind to the post-stage
   `owner-artifact.present` gate only.
2. **H1's promise sweep is incomplete: 9 surfaces, not 4.** The charter's four
   (`crates/cli/pack/steps/finalize.yaml:12-13` · `packs/methodology/steps/finalize.yaml:36-37` ·
   `QUICKSTART.md:138` · `crates/cli/src/cli.rs:1315`) miss the **highest-traffic** one:
   `crates/cli/src/render.rs:266`, the `what's-left:` line rendered on **every id-carrying compose**.
   Plus `design/validation.md:576`, `design/finalize.md:41`, `design/write-commands.md:195`,
   `VISION.md:120`, `design/bootstrap.md:71`. Two of those design docs state it as an **invariant**
   ("what validate reports and what finalize blocks on can never diverge").
   Also: `crates/cli/pack/config/commands.yaml:37`'s hint is arguably already accurate.
3. **No arm-(1) construction can make the sentence literally true.** The diff is 14 gate rows; of them
   `finalize.stage-failed` + the commit rejection are unpreviewable in principle; `finalize.empty-commit`
   / `nothing-staged` would be **wrong** at validate (blocks an agent who validates before `git add`);
   and `owner-artifact.present` would **false-positive** — LIVE-PROVEN: an artifact at the correct
   owned home but not yet `git add`ed → validate-with-the-gate blocking on a state that **finalizes at
   exit 0** (finalize stages it in-transaction). ⇒ the prose must be scoped **regardless**; the fork is
   *how much of arm (1) to add in addition*, not arm 1 vs arm 2.
4. **P5-4's stated premise is wrong.** The findings-verification says the *field-group-absent* miss on
   `set-field` "DOES carry the good route". It is not a miss — it **succeeds** (`insert_item_field`
   materialises the `<!-- fields -->` sentinel + bullet, `write.rs:3215-3249`). The real split:
   `set-field <addr> --value` at a missing item → `write.wrong-shape` + type-level `doc schema` route;
   `set-field <addr> --unset` at the **identical** address → `write.not-present` with the route's
   **literal `<address>` / `<task-id>` placeholders unsubstituted** (enrichment is wired at only two
   call sites, `doc.rs:932` set-slot and `doc.rs:1376` remove-item).
5. **`task diff` falsifies a recorded M44 "clean confirmation."** `DECISIONS.md:749` logs
   *"every verb speaks `--format json` holds on the success path (N4)"*. False; `task diff` is the
   counterexample. It is **three** nonconformances: success path prints raw text under `--format json`;
   **`jigc task diff --help` advertises the format it ignores** (law-1 lie on a *generated* surface);
   and an empty task emits **0 bytes on stdout AND stderr at exit 0**. The claim shipped into three
   surfaces incl. the **AGENT.md preload paragraph** (`crates/cli/src/adapter.rs:1102`).
6. **Tier 3's framing is half wrong.** The non-empty-`left_out` **stdout** arm IS already pinned
   (`crates/cli/tests/finalize_manifest.rs:313`). The **stderr** half is what is unpinned (its
   carried-over sibling has coverage at `carryover_gate.rs:655`; the left-out one does not).
7. **P3-2 (append-vs-update) narrows.** Greenfield VERIFIED the append prose **accurate byte-wise** on
   both roadmap and changelog. The trial's real finding is the **collision** case, which neither
   shipped story covers (it hard-rejects atomically). Scope the item to the collision case.
8. **`completions/artifacts/M45/VERDICT.md:23` says 558 golden files; the tree holds 612.** Benign,
   un-narrated delta. Do not plan against 558.

---

## 4. NEW DEFECTS FOUND OUTSIDE THE CHARTER (all VERIFIED live)

### 4a. Integrity tier — wrong bytes, permanent dead ends, or wrong frozen identity

- **N1 · The four milestone record-only doors brick permanently on one rejected commit.**
  `milestone create` / `add-task` / `add-from-spec` / `discard` seed the file-state baseline **after**
  the commit (`milestone.rs:362`, `:750`, `:1257`) while every later milestone write runs
  `reconcile_record_preflight` (`milestone.rs:470`) **before** it → `reconciliation.conflict-block`.
  Every escape blocks: fixing the hook, `milestone discard --force`, `milestone finalize`, even a raw
  `git commit` of the record (the poison is the gitignored baseline, not the index). Recovery required
  **hand-editing `.jigc/state/file-state.json`**. `jigc validate` reports `file-state.hash-matches`
  with an inapplicable route and exit 0. **Zero hook-rejection test coverage on all four doors.**
- **N2 · H2's dead end splits per clone.** After the rejected `migrate-corpus` commit, the operator's
  `jigc validate` is **clean exit 0** (baseline advanced, worktree bytes stamped) while a **fresh clone
  blocks** (`schema-conformance.schema-version-current`, exit 1) — and the teammate's route points back
  at the door that dead-ends. Silent until someone clones.
- **N3 · `set-field` / `doc author` at an *undeclared item field*: exit 0 + positive ack, then the doc
  is dead.** `set_item_field_or_insert` (`write.rs:3266-3303`) and `set_nested_item_field_or_insert`
  (`:1784-1831`) call `item_field_schema(...)`, and on `None` **skip the type check and write anyway**
  — no `write.unknown-field` guard, no post-write reparse. Afterwards every read is `store.unparseable`
  with a *human* route the adapter rule forbids following; only `jigc task discard` (whole task)
  recovers. The `--unset` sibling HAS the guard (`:5726-5734`). Reproduced on top-level, nested, and
  through batch `doc author`.
- **N4 · `set-slot` at an undeclared *leaf* of a single-slot item silently writes to the real slot and
  the ack names the phantom leaf.** Exit 0. Root cause: `ParsedItem::slot_span`
  (`crates/engine/src/parse.rs:156-165`) ignores the leaf id when `self.slots.is_empty()`, so the guard
  at `write.rs:1511-1515` cannot fire. Law-1 lie on the **success** path. (Multi-slot arm UNVERIFIED —
  no reachable multi-slot repeatable in the dev pack.)
- **N5 · The slug drops the leading component of a hyphenated compound.**
  `"On-call handoff artifact"` → `adr:call-handoff-artifact`; `"In-flight state capture"` →
  `flight-state-capture`; `"By-task-id join order"` → `task-id-join-order`. Boundary: first word is
  hyphenated AND its leading component is in `EDGE_STOPWORDS` (`crates/engine/src/slug.rs:114`) — the
  tokenizer splits before the M41-F5 edge-stopword drop, so an intra-word component becomes an edge
  token. The module doc (`slug.rs:24-25`) **promises medial stopwords are untouched**. Mints a WRONG
  FROZEN cross-reference identity; only `jigc rename` recovers. NOTE: the slug rule is fixed-point
  fenced (`slug_rule_version` + manifest hash) — a fix here forces a `SLUG_RULE_VERSION` bump and its
  fence.

### 4b. Greenfield-path tier — sits directly on the G1/G2/G3 probe routes (human: IN SCOPE)

- **N6 · Five code-less dev-pack workflows compose a finalize their own gate refuses.**
  `plan` · `architecture-documentation` · `record-change` · `record-decision` · `project-setup` all
  block on `commit:<id>#header/type` + `#summary` — leaves their composed text never names. Root cause:
  the **dev** pack puts the commit-doc writes in `step:implement` / `step:implement-quick`
  (`crates/cli/pack/steps/implement.yaml:12-13`), i.e. the *code-writing* step; `crates/cli/pack/steps/finalize.yaml`
  has none. The **methodology** pack puts them in `packs/methodology/steps/finalize.yaml`, so all 8
  methodology workflows are covered. ⇒ exactly the dev workflows that write **no code** are uncovered.
  Blind operator hits a red gate on the **first ADR, first spec, first arch-doc, first changelog
  release, first project bootstrap**. Secondary lie in the same step text: it tells these code-less
  workflows *"Make sure your code edits are staged (`git add`) first"*.
- **N7 · The advertised `resume:` door dead-ends on any HEAD advance.**
  `jigc start --task <id>` (`crates/cli/src/start.rs:1706-1717`) applies a **blanket** base-equality
  refusal, while `finalize`'s guard is **overlap-aware** (blocks only when moved history touches the
  task's own paths) and `task validate` has **no** guard. So the **read-only** door is strictly
  stricter than the **commit** door. A plain human commit on an unrelated file triggers it. The route
  offers only destructive exits (`git checkout <sha>` or `task discard`); the correct exit ("carry on,
  every other verb works") is never named. **`form-vision` mandates the broken step** ("ground the
  vision in the *committed* research… now RE-COMPOSE"). The compose footer's *"you can run them in
  parallel"* is false the moment one lands.
- **N8 · The pre-commit hook reports drift on every commit, permanently, when none exists.**
  `crates/cli/src/setup.rs:194-196` greps the validate JSON for `"probe": "doc-code"` with **no
  severity filter**; its own comment says "Warn IFF … a doc-code CONTENT finding". Any **non-Rust**
  project that follows `implement-from-spec` as instructed acquires a permanent false
  "drift detected" on every commit (the advisory `doc-code.is-a-test-unverifiable` is enough to trip
  it). This is charter item P1-8 — greenfield shows it is *self-inflicted by the tool's own routed
  flow*, not merely noise.
- **N9 · `single-task`'s changelog step cannot record a change, and `describe` cites it falsely.**
  `crates/cli/pack/steps/record-changelog.yaml` is 4 lines and names only
  `jigc doc create changelog --title Changelog --task {{task.id}}`. `changelog` is a *nested
  repeatable*; `add-item` / `set-slot` are never named. Following it verbatim yields an empty changelog
  that **validates clean at exit 0**. Compounding: `jigc describe` justifies hiding `record-change`
  from the router because *"routine change recording already rides `single-task`'s record-changelog
  step, so a catalog line would duplicate it."* — **factually false**, and it implicates M43's own
  suppression-reason discipline (`suppressed: {reason, expires}`).
- **N10 · The store-scope trailer claims advisories gate.** `jigc validate` prints
  *"N finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` /
  `jigc task finalize`."* — but `task validate` reports **clean at exit 0** on the same findings
  (`title-names-symbol`, `is-a-test-unverifiable`, both advisory), and a task finalized exit 0 with
  `title-names-symbol` present. It classifies by **probe family**, not severity. Law-1.

### 4c. Surface tier

- **N11 · `retitle-item` at a nonexistent item is P5-4's un-named axis sibling** —
  `write.rs:2037-2051`, same `write.wrong-shape` + generic route. Pinned at
  `crates/cli/tests/exit_codes.rs:546`. Sweeping P5-4 without it repeats the incomplete-fix pattern.
- **N12 · `set-field --unset`'s route ships literal placeholders** (see §3.4). Also: this makes M44's
  recorded "Verified bases" (`DECISIONS.md:667`) under-counted — it claims `set-field` never emits
  `write.not-present`; it has since M41.
- **N13 · `jigc milestone create` commits silently** — ack names no sha, no path, no next step, while
  `jigc setup` DOES name its install commit. It also moves HEAD, compounding N7.
- **N14 · `<<author: brief#vision>>` in `project-setup`** (`crates/cli/pack/steps/author-prd.yaml:6-7`)
  renders a task *role* where every other workflow renders a doc address; `jigc doc schema brief` →
  `store.unknown-type`, exit 1. The step's prose papers over it rather than fixing it.
- **N15 · `Create::task` — a Rust type name — leaks into `jigc doc author --help`.**
- **N16 · `doc schema` prints a `* = author-required` legend with no `*` in the listing** for `adr`,
  `research`, `vision`, `arch-doc`.
- **N17 · `finalize.empty-commit`'s route omits the abandon exit** (`jigc task discard`).
- **N18 · `schema-conformance.repeatable-populated` false-alarms on a young corpus** — a greenfield
  changelog with unreleased changes and no cut release is *correct*.
- **N19 · `jigc ingest`'s `needs-reconcile` route is weaker than `validate`'s for the same file**
  (prose vs the mechanical `jigc migrate <path> --as adr`).
- **N20 · `jigc milestone create <TITLE>` positional vs `jigc doc create --title` flag** — two `create`
  verbs, two conventions.
- **N21 · `relocate --format json` success arm has no standing test** (reachable only over a
  manifest-less pack).
- **N22 · `finalize.base-mismatch` mixes short and full 40-char sha forms in one message.**

---

## 5. STRUCTURAL FACTS THAT SHAPE THE BUILD (all VERIFIED)

- **S1 · The two `finalize.yaml` files are independently maintained, substantively different, and
  unfenced against each other.** Include resolution is origin-pack-scoped, so a methodology workflow's
  `{{ include: step:finalize }}` gets the methodology copy. **Charter item P2-1 IS this bug.** Any
  finalize fix applied once lands on half the surface. No seam exists; cheapest guard is a test that
  both declare the same `states-constraints:` set and state the same named facts.
- **S2 · Both stated-at fences are PRESENCE-ONLY, and the code says so** (`crates/cli/src/pack.rs:378-380`,
  `:468-470`). A tier-2 edit can delete the prose that states a contract and keep the front-matter
  code — every fence stays green. **This is the axis a wording wave is most able to break invisibly.**
- **S3 · Five verbatim-literal test assertions have NO regen path** — `crates/cli/tests/config_fill.rs:200-215`,
  `crates/cli/tests/multi_pack_acceptance.rs:456-471` and `:515,549-555`,
  `crates/cli/tests/start_compose.rs:161-176` and `:733` — plus the whole-script
  `crates/cli/src/setup.rs:1624 precommit_hook_body_golden`. Measured: one word changed in the dev
  finalize step ⇒ **156 goldens moved, 7 tests red**, of which `cargo insta` fixed **2**.
- **S4 · Golden blast radii (measured, byte-exact grep validated against a live regen):**
  dev `finalize.yaml` **156** · methodology `finalize.yaml` **180** · the `jigc task validate` promise
  line **342** · the `create-gates:` footer **324** (largest) · `adapter.rs:1086` AGENT.md **6** +
  2 unit tests · `render.rs:1517` setup hook path **0 goldens, exactly 1 test** · a `when:` reword on a
  selectable work-workflow **30** · `usage:`/`description:` **6** each.
  Compose-golden regen: `UPDATE_GOLDENS=1 cargo test -p cli --test compose_goldens`, **8.7 s**,
  zero-diff verified; CI refuses regen (`support/goldens.rs:265`).
  **Red-golden output does NOT reveal blast radius** — the suite is 6 tests, each panicking on its
  first differing cell.
- **S5 · `cargo test` FAILS FAST.** With a pack-prose edit it reported `1 failed` and stopped while the
  true count was 7. **Any prose batch must run `cargo test --no-fail-fast`** (~197 s).
  Gate cost warm: build ~0.2–1.9 s · test **201 s** · clippy 5.2 s · fmt 1.2 s.
  **Baseline test count: 2385 passed / 0 failed** at HEAD, across 241 test binaries. No drift.
- **S6 · P5-4's flip is cheap and safe.** NOTHING consumes the `wrong-shape`-vs-`not-present`
  distinction — no pack YAML, no adapter, no `AMBUSH_CLASS_CODES`, no `CHECK_INVENTORY`, no severity
  knob, no error-code registry, no golden. Its only consumer is
  `crates/cli/src/doc.rs:673`'s own route-selection guard. Blast radius: **2 asserts + 1 prose block**
  in `crates/cli/tests/write_not_present_route.rs` (`:490-500`, `:20-22`), + `exit_codes.rs:546` if
  swept to `retitle-item`. `design/command-output-contract.md:337` already argues for exactly this
  correction: *"A correction does not get cheaper by waiting; it gets frozen."*
  BUT: `GenerateError` has **no `NotPresent` variant** — the flip needs a new variant + a
  `generate_error_finding` arm + `enrich_not_present_route` calls at `run_set_field` AND
  `run_unset_field`.
- **S7 · P1-7 is a seam plumb, not a reword.** `crates/cli/src/setup.rs:480 resolve_hooks_dir()`
  already computes the truth (honours `core.hooksPath`, worktree `.git`-file, linked-worktree common
  dir) and is already called at `setup.rs:271`. `SetupSummary` (`setup.rs:648-658`) carries no
  hook-path field. Plumbing it also fixes the worktree cases the hardcoded literal lies about.
- **S8 · H2's error code costs 4 mechanical touch points and NO contract-version bump.**
  `error_code` is log-only/additive by declaration (`invocation_log.rs:47-49`). Touch points: mint the
  const · add to `ERROR_CODE_REGISTRY` (`invocation_log.rs:71`) · update
  `registry_mirrors_the_declared_members` (`:314-321`, a literal-list equality that **hard-fails**) ·
  update the `design/surface-contract.md:76` prose mirror. NOTE the membership check at the
  constructors is a **`debug_assert!`** — compiled out in release.
- **S9 · The `--format json` fence is shape-limited.** `crates/cli/src/adapter.rs:1539-1551
  format_is_a_global_arg` asserts only that `--format` **is global** (acceptance). It stays green if
  every verb ignores the flag. `--format` is a global clap arg (`cli.rs:37-38`), so all **44** leaf
  verbs accept it; 43 honour it, `task diff` does not.
- **S10 · `task diff`'s JSON envelope needs NO new data.** Base pin (`task.rs:662-667`), diff string
  (`git_diff`, `task.rs:2978-2991`), staged docs (`task.rs:1694-1725`, already sorted) are all in hand
  at `run_diff` (`task.rs:326-351`). The bug is a dropped parameter at the dispatch arm `task.rs:255`.
- **S11 · Two tier-2 routes are `RouteKind::Human`, outside the argv parse fence**, though both quote
  real argvs: `store.no-such-section` (`crates/engine/src/store.rs:381-388`) and `title-names-symbol`
  (`crates/engine/src/target_surface.rs:465-478`). Converting the mechanical portion to
  `Route::mechanical` buys the fence with no new machinery. NOTE the route fence itself is
  **debug-only** (`crates/engine/src/finding.rs:508-540`).
- **S12 · Pack-load fences a wording edit CAN trip:** `when:` shape (one line, no trailing period,
  ≤120 chars — `pack.rs:524-532`) · both stated-at presence fences (`pack.rs:381`, `pack.rs:471`) ·
  `Route::mechanical` argv parse. Everything else (finding messages, route tails, acks, `--help`
  paragraphs, AGENT.md prose, `doc list` rows, setup summary) is **pure judgment, no fence**.
- **S13 · `pinned_facts/` mechanics:** 5 modules today; add a file under
  `crates/cli/tests/pinned_facts/`, register it in `crates/cli/tests/pinned_facts.rs` with an explicit
  `#[path]` attr (there is **no `mod.rs`**), add a row to that file's provenance ledger doc-comment,
  and fill `pinned-by:` in the source repro block. ~120–160 lines each if it rides an existing
  `trial_corpus.rs` `State` (Fresh · CommittedSingletons · Migrated · RefsPostHoc · ChattyHooks ·
  Vendored); a NEW state ripples into every state-iterating suite.
- **S14 · The gate census** (finalize runs 14 gates validate does not) and the **committing-door axis**
  (**9 production doors + 1 excluded-by-design**: `jigc setup`'s `--no-verify` install commit, with its
  rationale recorded at `setup.rs:983-987`). Only **2 of 9** doors are `SURVIVABLE` today (task
  finalize's two arms). `CommitRejected` is a **private** struct (`task.rs:2558`) — no module outside
  `task.rs` can downcast it; a sweep must address that first.
- **S15 · A masking test pins the H2 dead end as correct:**
  `crates/cli/tests/corpus_migration.rs:893 migrate_corpus_fails_loudly_when_a_pre_commit_hook_rejects`
  asserts only `!status.success()` + verbatim stderr + zero commits. It never asserts a route, an error
  code, or that a re-run notices the staged state. **The fix must rewrite this test, not extend it.**
- **S16 · The survivable-frame definition (the shape any sweep replicates), from `task.rs:1342-1345` +
  `render.rs:947-957`:** (1) git's stdout+stderr verbatim, prefixed
  `` `git commit` was rejected (no commit was made): `` (2) a blank line then a **state truth
  statement** (what survived, what did not land, what is still staged) (3) a **re-run instruction
  naming this door's exact argv** (4) a **log-only** `error_code` from the registry — deliberately not
  a `Finding`, so the M41 route floor never wraps the hook's verbatim bytes (5) under `--format json`
  the whole framed string in `{"error": …}`.

---

## 6. THE M45 DECISION 6 RE-SETTLE (the wave's main fork — do not decide it, characterise it)

`crates/engine/src/validate.rs:276-283` + `DECISIONS.md` → 2026-07-23 Decisions 5 and 6.
- The rationale is **(a) correctness/feasibility**, not UX or scope: the presence gate asserts the
  artifact is durably **tracked**, a state phase-5 staging satisfies only *after* the stage; rather
  than relax the `tracked` clause (which would stop it proving durability), the gate **moved after the
  stage**, preserving the check's meaning.
- **The feasibility half STILL HOLDS** — live-proven (§3.3).
- **The completeness half does NOT.** Decision 5 itself names *"`task validate` is exit 0 while
  `finalize` is exit 3, so the what's-left preview cannot show it"* as **part of the live failure being
  fixed** — and the fix made that sentence *more* true. Decision 6 then recorded the split as a
  deliberate property without revisiting the surfaces that promise the preview.
- **The trial's two blocks were the `not-under-home` cause, NOT the untracked cause** — and that cause
  is fully adjudicable read-only at validate; it never needed the `tracked` clause. So the rationale,
  sound for the clause it names, **over-covers**: it removed *all* causes from validate to protect one.
- **Two shipped tests pin validate's silence as correct** and would invert under arm (1):
  `crates/cli/tests/owner_artifact_gate.rs:286-292` and
  `crates/engine/src/validate.rs:5913 owner_artifact_gate_relocated_off_validate_task_to_post_stage`.
- **`plan_owner_artifacts` (`crates/engine/src/finalize.rs:281`) is private** — arm (1) needs it
  exposed or the owner-exemption is lost and completion-record tasks false-positive.

---

## 7. WHAT IS UNVERIFIED / BOUNDED

- Area F was **single-operator and non-blind** (the auditor knows the product). A real blind agent may
  stall where it recovered from pattern knowledge.
- `migrate-corpus`'s dead end on greenfield — no unmigrated corpus exists there. (Verified separately
  in area B on a constructed corpus.)
- Fan-out/join (`milestone provision/execute/join/finalize`) not driven on greenfield.
- `park-idea`, `decided-task`, `quick-fix`, `record-dogfood`, `completion`, `increment`, `sub-task`
  preview-scanned only, for the commit-doc axis.
- D4 (`milestone finalize` squash:false per-sub-task rejection) and D7 (`add-from-spec` mid-loop
  rejection) — structurally derived, `UNVERIFIED (cost)` live.
- N4's multi-slot arm — no reachable multi-slot repeatable in the dev pack.
- All behavioural evidence is from **debug** builds; debug/release byte parity is UNPINNED by
  declaration (`implementation/pinning.md:70`). Matters for the `debug_assert!` fences (S8, S11).
