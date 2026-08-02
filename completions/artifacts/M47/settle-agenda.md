# M47 — consolidated gap list + SETTLE AGENDA

Deduped from four gap-detectors (decisions · docs · doctypes · capabilities), each measured against the
verified baseline in `M47-scope-brief.md`. Everything here was **exercised**, not inferred.
Companion: `M47-scope-brief.md` (done-picture · 8 corrected charter premises · N1–N22 · S1–S16).

**Human Scope decisions already taken (2026-07-25):** take all of it (no overload valve) · the
greenfield-path five are IN · the trial is a report, not a second close · tier 2 ships all ~16 items ·
the wave ships flow 47.

---

## PART A — THE FORKS (the human gate). Ordered by consequence.

### FORK 1 — H1: how much of the validate preview contract does M47 build?
**Robust case argued independently by a spawned advocate (do not let the orchestrator frame it alone).**

- **Forced regardless (not a fork):** the prose must be scoped on **9 surfaces**. No construction makes
  the sentence literally true — 2 of 14 gate rows are unpreviewable in principle, 2 are *wrong* at
  validate, 1 false-positives.
- **The charter's "third arm" is DEAD.** `finalize --dry-run` aborts at `finalize.empty-commit` in the
  exact state an agent previews from (commit doc filled, nothing staged) — `plan_finalize` is a
  sequential abort-chain (`crates/engine/src/finalize.rs:186-270`), not a gate collector.
- **New defect, must be fixed either way:** `--dry-run` exits **0** on a state finalize refuses at **3**
  (carryover verdict discarded; refusal at `task.rs:1173` sits after the dry-run return at `:1155-1166`)
  and its JSON carries **no `findings` key**. Lands on G1's second plant.
- **Arm (1) is buildable by hand-assembly, not reuse:** `owner_artifacts_gate` (`validate.rs:1803`),
  `decide_carryover` (`finalize.rs:674`), `read_staged_snapshot` (`state.rs:671`) are all pub;
  `plan_owner_artifacts` (`finalize.rs:281`) is private and needed for the owner-exemption.
- **Costs:** inverts 2 shipped tests (`owner_artifact_gate.rs:286-292`, `validate.rs:5913`);
  ~342 goldens on the promise line; reconciles 3 contradicting design docs.
- **M45 Decision 6 is mis-scoped** — declares *store*-scope blindness, cites a *task*-scope case;
  Decision 5 in the same entry calls that split part of the failure being fixed. Feasibility half holds
  (the `tracked` clause genuinely cannot be satisfied pre-stage); completeness half does not. The
  trial's two blocks were the **not-under-home** cause, which needs no `tracked` clause.
- **Arms:** (a) prose-scope only + fix the dry-run false green · (b) + wire the carryover probe onto
  validate · (c) + the read-only-adjudicable owner-artifact causes.

### FORK 2 — N5: does M47 fix the slug hyphenated-compound bug? **(one-way door)**
**Robust case argued independently by a spawned advocate.**
- Verified: `"On-call handoff artifact"` → `adr:call-handoff-artifact`; the module doc (`slug.rs:23-25`)
  **promises** medial stopwords are untouched. Shipped surface contradicts its own stated rule.
- **Any** correct fix moves `rule_fingerprint()` (the census generates `a-b`, structurally identical) →
  `SlugRuleHashMismatch` against **both** manifests → `SLUG_RULE_VERSION` 2→3 + re-pin both, one commit.
- **No migration exists or can exist** (`manifest.rs:161`, `storage.md:82`) — two permanent id generations.
- Constraints: fix in `slugify`, **never** `renormalize`; the version arm does NOT force the bump
  (same author edits both) so it must be a stated, tested obligation; `slug::mint_statement`
  (`slug.rs:99-110`) renders into all 12 migrate steps and changes with the fix.
- **Charter interaction:** tier-2 item P5-3 currently ships the **documentation** of the rule. If N5 is
  not fixed, M47 deliberately documents a rule the code does not implement — a law-1 lie by omission.
- **Arms:** (a) fix + generation 3 · (b) declare current behaviour correct, fix the doc promise, fold
  into P5-3 · (c) defer with trigger.

### FORK 3 — the schema hash eats authored prose. Narrow it, or declare prose out of scope? **(one-way door)**
**Robust case argued independently by a spawned advocate.**
- Measured: a one-word `description:`/`usage:`/slot-`hint:` reword **moves `schema_hash`** → pack load
  blocks at every verb → bump + re-pin + snapshot + `migrate-corpus` run; and since M42 an unmigrated
  corpus **fails `validate` at exit 1**.
- Those are surfaces M47 most wants: `describe` renders `description`/`usage` verbatim; the
  `{{schema:<T>}}` projection renders every slot `hint` **twice** across the 12 migrate steps.
- **The fix already ships:** `erase_out_of_projection` (`schema_diff.rs:389-402`). Re-defining the hash
  over it costs a one-commit re-pin of 16 hashes, **no bumps, no snapshots, no migration** (stamps don't move).
- Foreclosed by `design/corpus-migration.md:180` — *"narrowing the hash … would be a freeze-semantics
  change, out of scope here"*. Rationale is **scope (M42-local), not principle**.
- Honest counter: the full projection would stop the stamp tracking `set:`/`default:`/`inverse-card:`
  semantics. A **presentation-only** narrowing (`description`/`usage`/`hint`) may be the correct shape.
- **Arms:** (a) narrow (which projection?) · (b) declare "no doctype prose touched at M47" as an
  explicit checkable scope line and budget a bump per exception.

### FORK 4 — N1: the milestone record-only doors brick. Which fix shape?
- Live: one rejected commit → every escape blocks (`reconciliation.conflict-block`), incl.
  `discard --force` and a raw `git commit`. Only hand-editing `.jigc/state/file-state.json` recovers.
- **Foreclosed by `design/reconciliation.md:51`** — *"the **durable** file-state is persisted **only by a
  landed `finalize`**"*. The code honours it exactly. The bug is that rule applied to a door its
  rationale (*read verbs don't write state*) never contemplated.
- Naive fix (baseline before commit) **reproduces N2's class** (advance over bytes that never committed).
- **SEQUENCING: N1's state fix is a hard prerequisite of H2 on 4 of 9 doors** — the survivable frame's
  "what survived" clause cannot be stated truthfully today; it would be a bricking notice.
- **Un-briefed compounding:** rejected `milestone create` runs leave record `.md` files **staged**, which
  then trip the carryover gate on the next unrelated task — jigc's own bytes poisoning jigc's own gate.
- Undecided underneath: **jigc's own uncommitted write is indistinguishable from a human's OOB edit.**
  No record adjudicates this.
- **Arms:** (a) captured-pre-image rollback on rejection (honours `:51`; joins the confidence-audit
  discipline as a 5th staged-path family) · (b) advance baseline on the worktree write (revises `:51`) ·
  (c) frame + resume-notices-uncommitted only.

### FORK 5 — N6: where does the commit-doc instruction for the five code-less dev workflows live?
- Census verified by driving every workflow: **5 dev workflows name neither write** (`plan`,
  `architecture-documentation`, `record-change`, `record-decision`, `project-setup`); 3 dev workflows
  name both; `sub-task` names both **twice (pre-existing double-instruct)**; 5 migrate workflows are
  **exempt, verified** (mint pre-fills); 3 are inert; all 16 methodology workflows are covered.
- **Arm 3 is NOT free reuse:** `crates/cli/pack/steps/author-commit.yaml` carries the right triple but
  its prose is sub-task-specific and wrong ("never `git commit`… this worktree's detached HEAD").
- **Arm 1** (dev `step:finalize`) walks into **S1** (two unfenced `finalize.yaml`, 156 + 180 goldens).
- `design/workflow-dialect.md:186` reads *for* composing an existing unit over overloading one step.
- **Foreclosed-by-doc:** `design/finalize.md:123` declares the `git add` contract an obligation on *the
  family* — and that is exactly what makes the dev finalize step tell five code-less workflows to stage
  their code. Engage its rationale (silent partial commits), which is sound for code-writing workflows
  and over-covers the code-less family.

### FORK 6 — N8: the drift hook's severity filter is a semantics fork, not a grep clause
- POSIX `grep` is line-oriented → severity and probe never match together. Working form needs
  `tr -d '\n'` + a single-line ERE, which works **only** because serde emits `severity` immediately
  before `probe` — an **unfenced coupling between a shell script and a Rust struct's field order**.
- **The semantics fork:** filtering to `blocking` silences `doc-code.title-names-symbol`, which M40
  **deliberately demoted to advisory** — i.e. it removes the M19 drift signal the hook exists for. The
  alternative (exclude `is-a-test-unverifiable` by code) is a different rule with a different fence.
- `precommit_hook_body_golden` pins the *script text*; nothing pins its *behaviour over a real report*;
  `precommit_hook_acceptance.rs` has 8 arms, none with mixed severities.

### FORK 7 — H2: the error-code naming for 8 more doors
- `ERROR_CODE_REGISTRY` has 2 members and its doc says *"deliberately **not** a per-verb code mint."*
  Reusing `finalize.commit-rejected` on `rename`/`milestone create`/`migrate-corpus` puts a **lying
  code** in the log — a law-1 violation on the surface M42 built to stop the log lying. Minting
  per-door codes contradicts the registry's recorded rationale.
- **Release-fence gap:** the membership check is a `debug_assert!` (`invocation_log.rs:96`), compiled
  out in release — and **rc.10 ships as a release build for the trial**.
- Mechanics: `CommitRejected` is private (`task.rs:2558`) → `pub(crate)`; `render::commit_rejected`
  (`render.rs:945`) hardcodes the task idiom → must take a state-truth statement + the door's argv.
- **Strong prior art:** `crates/cli/tests/hook_output_axis.rs:1-46` already enumerates the identical
  9-door axis with the identical exclusion, for the non-blocking case, with a working harness.
- **`corpus_migration.rs:893` pins the dead end as correct — rewrite, don't extend (S15).**

### FORK 8 — P5-4's real axis, and route *followability*
- The matrix is **7 cells / 5 codes**, 2 correct today. Chartered as a single code flip it fixes 2 of 7.
- **New class:** `<doctype>` ships **literal, unsubstituted** on 4 codes, plus `<address>`/`<task-id>`
  on 3 more outside the write family (`write.non-reparseable`, `finalize.migration-no-replacement`,
  `reconciliation.conflict-block`). It passes the route fence only because these are declared members of
  `DUMMY_SUBSTITUTIONS` (`route_fence.rs:32`) — **the fence proves parseability, never followability.**
  A mechanical check exists: *if the placeholder's value appears in `key.target`, it must be substituted.*
- **Record cost the charter didn't budget:** `command-output-contract.md:337` authorizes a *target value*
  correction, **not** a `code` change — the doc exists precisely because M42 caught itself reasoning by
  analogy there. So the flip needs an explicit new authorization clause + a revision of
  `design/validation.md:60`'s taxonomy.

### FORK 9 — N3/N4: guard the undeclared-address writes? (and the route exemption they break)
- **N3** (`write.rs:3275-3279`): undeclared item field → exit 0 + positive ack → doc dead; only
  `task discard` recovers. The `--unset` sibling has both the guard and the reparse. **The correct guard
  shape already ships on the sibling — genuine reuse.** No schema change; no conformant corpus can
  contain the state, so nothing reclassifies.
- **N4** (`parse.rs:156-165`): `slot_span` ignores `leaf_id` when `slots.is_empty()`. **Root cause is in
  a shared read path with 2 consumers** — the fix belongs at the write callers (`write.rs:1511`, `:1746`),
  not in `slot_span`, or conformance adjudication changes too. `resolve_leaf` is **duplicated**
  (`store.rs:612-635` and `doc.rs:3169`). Multi-slot arm **UNVERIFIED** (no reachable multi-slot
  repeatable in the dev pack) — needs a synthetic fixture or a re-declared bound.
- **Foreclosed-by-doc:** `conformance.*` is route-exempt by declaration (`finding.rs:272`), rationale
  *"the located message is the repair… no CLI verb repairs a hand-broken byte."* N3 makes the byte arrive
  **through jigc's own write verb at exit 0** → a **blocking, route-less finding produced by the tool's
  own success path**. The better path (guard the write) *honours* the rationale by restoring its scope.
- **The decision:** the exit-code change (0 + ack → 1 + block) is a behaviour change on a public surface,
  and M45 Decision 6 declined an analogous alignment on exactly that reasoning. Counter: today's
  behaviour destroys the doc — a data-loss class, which outranks.
- **P5-4/N11/N3/N4 are one axis.** Sweeping any without the others repeats the M45 pattern.

### FORK 10 — N7: three siblings, three base-pin rules
- `start --task` blanket-refuses; `finalize` is overlap-aware; `task validate` is unguarded. Live-verified
  in one repo state: resume exit 1, validate exit 0, **finalize landed**.
- `design/storage.md:238` records the blanket rule; `design/finalize.md`'s M17 amendment revised the same
  rule for one sibling with the rationale *"the unconditional rejection proved wrong for serial-task
  shapes the methodology itself mandates"* — which applies verbatim to resume. The amendment's scope
  bound was drawn against the **milestone** sibling; the **resume** sibling was never enumerated (M31 shape).
- On the G2 route: `form-vision` mandates the broken step; the compose footer's *"you can run them in
  parallel"* is false the moment one lands.

### FORK 11 — `task diff --format json` has no precedent shape
- Three divergent shapes already ship (bare array · `{"docs":[…]}` · `{"dry_run":…}` with no `findings`).
  `design/doc-read-surface.md:15` records `task diff` shows *a changeset, not a doc at an address*, so
  the `DocAck` target decomposition doesn't apply. **A 1.0-pinned read contract with no precedent** —
  picking it silently at build time is the cheap path.
- Empty state must be non-empty: today `task diff --format json` on an empty task emits **0 bytes on both
  streams at exit 0**.

### FORK 12 — the tier-2 wave's own footgun: presence-only fences (S2), demonstrated
- Live: deleted 590 chars of copy-in/append contract prose, kept `states-constraints:` → **clean build,
  clean pack load, contract gone.** Deleted the `{{schema:}}` ref → the obligation itself disappears AND
  the composed text says *"the batch payload … follows:"* with **nothing following** — a law-1 lie
  manufacturable with every fence green.
- **Arms:** trust review + the completion audit · build the S1-shaped guard (both `finalize.yaml` declare
  the same `states-constraints:` set and state the same **named facts** — presence-of-a-named-fact stays
  inside the A-3 "presence never content" bound).

### FORK 13 — governance: the demand-counter now points at M47
- `decisions-pending.md:14` — *"the third independent demand forces it onto the **next wave's** Settle
  agenda"*. M47 executes before M46, so M47 **is** the next wave — but capability entries 2/3/4 each
  carry a dated 2026-07-25 demand and each says *"Trigger: the M46 Settle."* The file contradicts itself.
- **Entry 8 has FIRED, un-noted:** N1's recovery required reading/editing the recorded file-state set.
- **And M47's scope DRAINS evidence entries 3 and 8 are counted on** (P1-8's hook fix + P1-6's message
  fix + N1's fix). Re-count **after** M47's scope is fixed, or M46 inherits a paid-down count.
- **Arms:** (a) M47's Settle formally re-Settles 2/3/4/8 and records the outcome · (b) the human declares
  the exception and it is **written into `:14`**.

### FORK 14 — N9: `describe`'s false suppression reason
- `record-change.yaml:8`'s reason is factually false; `expires: never`; `describe` projects it (law 1 on
  a generated surface). M43's fence is **presence-only by design** and cannot catch it.
- **Arms:** (a) correct the reason · (b) un-suppress `record-change` (touches "what surfaces exist") ·
  (c) fix `record-changelog.yaml` so the reason becomes true (closes both; already ruled IN as a
  greenfield-path item).
- **Fence interaction:** using `{{schema:changelog}}` in the fix **newly obligates** the step to declare
  `create.singleton-copy-in` + carry the copy-in prose. Hand-writing the grammar instead re-introduces
  exactly what M43 Inc 3 deleted from 12 templates (a template that understates the schema).

---

## PART B — BLOCKING GAPS THAT ARE NOT FORKS (settle by doing, record the decision)

- **B1 · The validate≡finalize invariant is stated in 8 doc sites and the docs contradict *each other*.**
  `validation.md:576` and `write-commands.md:195` state it **absolute**; `finalize.md:41` carries a
  declared exception two lines later at `:43`; `command-output-contract.md:325` states the counter-position
  but **scopes it to store-scope**, the one scope where the divergence isn't. Also
  `reconciliation.md:131` (a table row claiming the *whole* preflight is "same as validate") and
  `write-commands.md:32`. **`finalize.md:43`'s "Same check, same report, later position" is false** —
  validate never reports it — and it is the sentence that lets a reader conclude no scoping is owed.
  **`VISION.md:120` — recommended NO change** (asserts no invariant; a VISION edit stays a flagged event).
- **B2 · `design/reconciliation.md` has never heard of the record-only door.** It pins re-baselining at
  *"exactly three sites"*; N1 is a fourth. The bug had no design of record to violate.
- **B3 · `design/validation.md:60` universalizes a 2-site enrichment** ("overridden at the CLI write-verb
  dispatch"). P5-4 adds sites; revise in the same motion or ship the over-statement wider.
- **B4 · `design/surface-contract.md:28` re-affirms a rationale `finalize.md:99` refuted one wave earlier**
  (the hook-rejection route exemption). Reconcilable — the exemption is about the route floor not wrapping
  git's bytes, and the frame wraps *around* them — but the reconciliation is written nowhere.
- **B5 · `DECISIONS.md` carries two false "verified bases"** the wave would build on: `:667`
  (`set-field` never emits `write.not-present` — false since M41) and `:749` (every verb speaks
  `--format json` on the success path — falsified by `task diff`). Plus `:2602` (*"`--dry-run` dropped —
  validate is the preview"*), superseded at M30 and never marked.
- **B6 · `decisions-pending.md:190` still lists `finalize --dry-run` as an open hardening item** —
  resolved at M30. A planner consulting the file reads it as unbuilt.
- **B7 · `implementation/pinning.md` contradicts itself on the golden count** (~1500 at `:17`, 612 at
  `:23`; disk says 612; `M45/VERDICT.md:23` says 558). And its §2 JSON-purity fence rides the *compose*
  golden sweep, which structurally cannot see `task diff` — fixing the verb leaves the fence blind to the
  next one. `command-output-contract.md:300` already specifies the clap-tree-enumerating purity suite; it
  does not exist.
- **B8 · `implementation/roadmap.md` has NO section for the confidence-audit wave** (zero hits). A fully
  built wave with a persisted VERDICT and no roadmap row.
- **B9 · `design/assistant-adapter.md:48`'s "cannot rot" rationale rotted** — quoted verbatim into
  `adapter.rs:1102`. It predicted enumeration drift; the actual rot was a verb never honouring the global
  flag. Patch the sentence without correcting the *rationale* and it re-authorizes the reasoning.
- **B10 · N10 has TWO arms**, not one: the severity misclassification **and** an un-enumerated door arm —
  the store trailer names only the two *task* doors while the milestone doors gate on the same findings.
- **B11 · Two `commit` schemas exist** with different shapes (dev has `implements: ref → spec`), and
  methodology **shadows** dev in a composed cascade. An S1 parity test must assert *contract facts*, not
  a field list.
- **B12 · N18's fix is a one-token knob** (prior art settled it — `validation.md:527` rejected a schema
  `min-items` knob as a frozen trap), but the default is **duplicated** in both `knobs.yaml` with
  methodology whole-file-shadowing and **no test enforcing the mirror**. A one-file fix is inert.
- **B13 · N16 was imprecise:** `arch-doc` *does* print `*`; only `adr` prints none. The defect is that the
  legend says `fields (…)` while markers land on item leaves under `sections:`.
- **B14 · `describe --format json` emits valid JSON while its own long help says "don't parse it."**
  Law-1 tension on a generated surface: refuse the flag or fix the help.
- **B15 · M47's charter carries ZERO `(D)`-tagged entries** while every prior wave's does. Under the M45
  precedent, an untagged fork is what ambushes a build.

---

## PART C — THE PROPERTY CENSUS (required Settle artifact; `property → paths → acceptance`)

- **P1 · every committing door survives a hook rejection** → **9 doors + 1 excluded** (the axis is already
  enumerated at `crates/cli/tests/hook_output_axis.rs:11-40` for the *non-blocking* case, same exclusion,
  working harness). 2 survivable today. **Plus an un-enumerated sibling: the door leaves no staged residue
  that poisons a sibling door** (verified: rejected `milestone create` runs leave staged records that trip
  the next task's carryover gate).
- **P2 · every write verb that misses an item routes to a followable containing section** → 7 cells,
  2 correct. Acceptance asserts the code **and** that the route contains no `<…>` substring.
- **P3 · no surface promises a preview it does not deliver** → 9 surfaces (+2 design-doc invariants).
- **P4 · every leaf verb honours `--format json` on the success path** → 44 leaf verbs, 43 honour.
  Acceptance = the clap-tree-enumerating purity suite `command-output-contract.md:300` already specifies.
- **P5 · a write at an undeclared address is rejected, not silently written** → 9 cells, **5 unguarded**.
  The correct guard shape already ships on the `--unset` siblings.
- **P6 · a finding a driver reads back is followable** (NEW) → every route whose placeholder value is
  derivable from `key.target` must be substituted. 6 sites found.

---

## PART D — SEQUENCING CONSTRAINTS FOR DECOMPOSITION

1. **N1's state fix precedes H2's frame** on 4 of 9 doors (the frame cannot state the truth until then).
2. **N5 precedes P5-3's slug documentation** (tier 2 documents the rule; the rule may change).
3. **Fork 3 (schema-hash) precedes any doctype-prose edit** — it decides whether such edits are free.
4. **Fork 9's guards precede or accompany P5-4** — one axis; sweeping separately repeats the M45 pattern.
5. **The goldens regenerate last per increment**; `cargo test --no-fail-fast` (~201 s), never `cargo test`.
6. **M46 sequencing hazard:** if any doctype must bump at M47 for prose, decide whether to batch M46's
   `deferral-ledger` shape change (P2-8) into the same bump, or adopters pay two migrations in two waves.
