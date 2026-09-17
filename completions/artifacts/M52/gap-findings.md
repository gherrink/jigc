# M52 — the consolidated gap findings

**Four Opus `gap-detector` subagents, 2026-09-17, one per dimension**, each given the charter
([decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.16 wave (M52)*), the
driven [baseline-ledger.md](baseline-ledger.md) and the evidence base
([M51/per-axis-review](../M51/per-axis-review/README.md)). Their reports are archived verbatim beside
this file — [gap-decisions.md](gap-decisions.md) · [gap-docs.md](gap-docs.md) ·
[gap-doctypes.md](gap-doctypes.md) · [gap-capabilities.md](gap-capabilities.md) — and this file is the
**deduped, severity-ranked** consolidation. Citations are `[dec Bn]`, `[docs Bn]`, `[dt Bn]`,
`[cap Bn]` into those reports; every claim is *driven* or *read* as the report marks it. **Relayed,
not verified by the orchestrator** — the Settle re-drives the rows it turns on.

**Three headlines before the list.**

1. **No M52 fix needs a schema-hash to move, and no pack workflow/step edit *can* move one** —
   `schema_hash` digests the `Schema` value alone, exhaustively destructured
   (`crates/engine/src/manifest.rs:77-89`) [dt headline]. The snapshot store is fenced complete both
   directions over both manifests (`snapshot_store_two_snapshots.rs:168`), so *"closing D-2 needs no
   bump"* holds [dt positive]. The cost this wave carries instead is **read-contract and
   wire-shape** movement — `doc schema` `contract-version` 6→7 [dt B1] and the reject-envelope
   reshapes [dec M2] — **and that window closes at the 1.0 pin, with M52 the last wave before the
   call** [dec (c)].
2. **Two of the charter's six forks are mis-posed** [dec hunt (a)]: fork 1's *one primitive vs
   per-site* is false against the measured shapes — M51's CAS is `&'static str` → `Option<Vec<u8>>`
   and **structurally cannot take a directory tree** (`task.rs:4094-4098`), so the class is **three
   mechanisms** [cap B2]; fork 3's robust arm *as worded* (every byte under the task area) fires on
   **100 % of tasks at 100 % of doors** because jigc writes its own files there — refused by M46's own
   `--ignored` ground, which **argues for** the complement-of-writer-set subject rather than against
   it [dec S6, docs hunt (b)]. **Fork 2's consent arm is unavailable** under the wave's own boundary:
   any posture consent is a **new flag** = new capability, and M51 amendment §3 withdrew `--force` for
   the family [dec B7, cap B3]. **Fork 4 is settled doc-side, not open**: two locked docs and the
   code's own doc-comment say `from`; the code says `to`; the fix moves no hash [dec P1, cap hunt (c)].
3. **The pre-pin window's close is one event, and the charter prices it for fork 6 only** — DEFECT B
   (17 cells / 7 root shapes), DEFECT C, DEFECT 1, LD-2 (`doc schema` vs `doc show` on
   `milestone-record.base`), the bogus `target.slug` on the pinned write ack, and a directory as
   `managed` on the pinned `doc list --task` all freeze on the pin. **One decision over that set, not
   six** [dec M2]. And the charter's stated price for fork 6 is wrong: the envelope version is
   `engine::result::SCHEMA_VERSION = 3` (`result.rs:27`), not `doc schema`'s `contract-version`
   [docs B9, cap M2].

---

## BLOCKING

### The tier-0 scope itself

- **G-1 · Nothing bounds tier 0 against the baseline's 20 latent defects, 11 of them exit-0 loss or
  harm** — the charter's own tier-0 predicate admits them and no decision says whether tier 0 is
  *the 12 rows* or *12 ∪ 11*. Those inside the six classes ride the class fixes by construction;
  four sit in **no charted class** and need explicit admission or a written trigger: **LD-3** (6 of 9
  `milestone` doors run at exit 0 with no `jigc setup`, `provision` creating a real worktree —
  *"project posture"*, in no fork) · **L-1** (`milestone discard` fails **open** on an unreadable
  worktrees root and settles the record irreversibly) · **L-2** (`uninstall` destroys a plain file at
  any `ENTRIES` name) · **rename `--slug <bogus>`** on a placement head (commits a rewrite of the real
  `VISION.md` — in fork 5's class but reached by no arm the charter states). [dec B1, B4, B12; baseline §2]

### Fork 1 — the rollback family

- **G-2 · Three mechanisms, not one primitive** (see headline 2): (i) file-valued populations the
  M51 CAS can serve (promote · retire · `RecordPreImage` · `RecordFlipGuard::Drop` · `rollback_rename`
  · the record path at `milestone create`) but with the **wrong finding identity** —
  `finalize.rollback-conflict` at `task.rs:4301` is wrong for `rename`/`milestone create`/`config
  set`/`doc author`; (ii) **directory-tree** populations (`unwind_mint`, `unwind_unrecorded_seeds`)
  which `finalize.md:195`'s carve-out excludes from CAS *"absent-means-delete over N files, including
  files this run never saw"* — a rationale that **holds for a directory jigc did not create and does
  not transfer to `.jigc/tasks/<id>/`, which `unwind_mint` created in the same run** (candidate
  **revise**: delete only what this call minted); (iii) populations already correct under a **door
  guard** (`rename.dirty-tree`, `setup`'s pre-write refusal). **No rollback-population registry
  exists** (`grep ROLLBACK_|RESTORE_` → 0). [cap B2, docs hunt (b), dec B2]
- **G-3 · `finalize.md:200`'s all-or-nothing promise is scoped to the finalize transaction and 5 of 9
  populations sit outside it with no doc stating a rollback rule at all** — a doc gap, not a
  foreclosure; and `finalize.md:174` claims *"rewrite each retire-deleted original"* while retire
  **never restores**, so fork 1's cheap arm has no declared behaviour to keep. [cap hunt (b), dec S2]
- **G-4 · The `RecordPreImage` door count disagrees three ways, one doc-vs-doc at HEAD**:
  `finalize.md:193` + `reconciliation.md:149` = four; `team-ready-state.md:74` = five (*"a rule about
  doors, never a list of them"*); driven = **six**. `reconciliation.md:144`'s *"exactly four sites"*
  is falsified inside its own file by `:111` and `corpus-migration.md:68` (≥6). [docs B1, B2]
- **G-5 · DEFECT 1's machine-surface half is already answered by a locked rule**: `command-output-
  contract.md:494` — a withheld value *is* a gap; only the *how* (a reshape of the `{error}` arm,
  window-bound) is open. And a **second producer** prepends prose to the JSON reject stream (`config
  set`'s relocation lines) [baseline §1.1]. `storage.md:300-310`'s single-writer model is read two
  opposite ways inside M51's own settle record (§2 refuses a race fix citing it; D4/§6 builds one) —
  **blocking for fork 1** until the Settle states which reading binds. [dec hunt (b) 4, P2]

### Fork 2 — the posture family

- **G-6 · Consent is unavailable; only refuse is admissible — say so.** A `git commit` under
  `CHERRY_PICK_HEAD`/`REVERT_HEAD`/`MERGE_HEAD` **is** the operation's conclusion to git; no argument
  makes it not conclude; no shipped flag carries posture consent. [baseline §1.2, dec B7, cap B3]
- **G-7 · Family membership is new surface under the docs' closed enumeration** (`finalize.md:31`,
  `validation.md:663`: merge/rebase/bisect only) — the razor's leg 1 holds only for the **route
  promise** (*naming the git command that resolves the state*) and for **`--carry-staged`'s consent
  noun** (*merge* → any in-progress operation); membership must be argued on **leg 0** (a revert
  concluded at exit 0 by every commit door is repository harm at a committing door — tier 0 by the
  charter's own predicate). **A per-operation route table is owed** (`validation.md:663` forbids *"a
  menu of three"*). [docs B5, dec B8]
- **G-8 · `InProgress::markers()` cannot express two of the eight states** — presence-of-any over
  static filenames (`repo.rs:166-176`, `:640-650`); a squash merge is `SQUASH_MSG ∧ ¬MERGE_HEAD`, a
  conflicted index with no operation needs a git query. `repo.rs:1-16`'s no-shell-out rationale is
  **already spent** (`head_ref` `:617` and `classify_leftover` `milestone.rs:2639` shell out) and binds
  only `worktree_git_dir` (`:652`) — candidate **revise** of the doc-comment's scope. The
  `applying`/`onto` discriminator for `rebase-apply/` is git-2.54's on-disk contract. [cap B3, dec B8]
- **G-9 · `refuse_on_posture`'s `current_dir().ok()?` (`cli.rs:540`) silently skips the whole M51
  posture family** — a second undeclared escape beside `GIT_DIR`, in no bound; and **nothing owns
  dispatch order**. [dec B6, cap B7]
- **G-10 · `task validate` has no posture row and `GATE_COVERAGE` models findings, not door-refusal
  families**; the row alone would be a law-1 lie the fence blesses — row + probe must land together
  (`preview_gates` is CLI-side, `task.rs:1736`, reachable). The obvious fix flips an exit, the ground
  M47 Settle D1 used to exclude `base-pin` — **but M50 flipped `task validate` 0→non-zero
  deliberately** (`DECISIONS.md:2621` bound iii), so a *declared* flip is a candidate **revise**.
  [dec B5, hunt (b) 1; cap B4]
- **G-11 · The `GIT_DIR` declared-out must be scoped against the wave's own done-picture in writing**
  — stating one half is the shape the wave exists to correct. [dec hunt (b) 2]

### Fork 3 — the destroying-door subject

- **G-12 · The subject for non-`.md` task-area bytes is stated in NO locked doc** —
  `team-ready-state.md:94` scopes the rule to worktree-shaped paths, `:96` widens only to sub-task
  `docs/*.md`, `finalize.md:137` authorizes `Remove .jigc/tasks/<task-id>/` with no guard ⇒ **fork 3
  has no razor leg 1**; it argues leg 0 and the wave **writes** the rule. Three different subjects
  across `team-ready-state.md:94`, `setup.rs:3030`, `finalize.md:137`, none reconciled. [docs B3, B4, hunt (c) #10]
- **G-13 · Nobody owns jigc's own writer set, and the baseline's list is short.** 13 scattered
  consts, 11 private (`state.rs:32,37,42,51,60,68,77,215,788`, `validate.rs:66,71`, `migrate.rs:63`);
  **driven**: `renames.json` lands at the ordinary `doc rename --task` door — absent from the
  baseline's list, so a guard cut on that list blocks every task that renamed a doc. **No clap tree
  to ⇔-fence against** — the honest shape is a counted source-scan on the `repo_relative_paths.rs:849`
  mold. [cap B1, dec B3]
- **G-14 · `task finalize` has no `--force`** (driven `--help`), so under the consent mold it can only
  narrate — a permanent *visible-not-prevented* cost to be taken in words — **or** displace rather
  than destroy (`.jigc/displaced/`, `DECISIONS.md:944`, is `unverified-reuse` for this shape). And
  M50's `--force`-single-consent and M51's posture-no-override are two rules with a hole exactly
  where fork 3 lands. [dec B10, B2, S7]
- **G-15 · The classifier fails open**: `leftover_at` maps every `symlink_metadata` error to
  `Absent`, `probe_leftover` returns `None` for `Absent`, against its own doc-comment (`milestone.rs:2783-2794`);
  `leftover_probe_fail_closed.rs` is a **near-miss citation** (plants at the leaf, never an unreadable
  root) — do not classify L-1 test-fenced on the filename. `workbench_paths` (`setup.rs:3057-3071`)
  violates its own doc-comment (name-vs-shape). Sinks take the **registered set** while guards walk
  **on disk**; no sink registry. [cap M1, M5; docs B4]
- **G-16 · `project-setup.md:160` records the *"`git` on PATH"* route + unnamed `--force` as fixed at
  M50 while `setup.rs:3569` still emits it** — a locked doc asserting an incomplete repair; and
  `:160`'s *"complete by construction"* vs `milestone.rs:3298`'s fixed ack is an **ack axis** nobody
  enumerated. [docs B10, dec P4]

### Fork 4 — the corpus walk

- **G-17 · Settled doc-side (`from`), violated by the code, no hash moves; the riders are the real
  work**: `candidate_docs`' `.filter_map(|prior| prior.location)` (`migrate_corpus.rs:2104`) drops
  prior `placement`; the location branch (`:2080`) loads no snapshot; `DoctypeMigration:68-81` carries
  `docs_root` but no `placement_root` — both missing pieces are in hand (`migrate_in_repo:331` holds
  `resolved`; `start.rs:3414/:3430` are the extracted primitive). **A missing snapshot silently
  narrows the walk** (`.filter_map(|k| load_prior_schema(…).ok())` + `pack.rs:211`'s best-effort)
  against `corpus-migration.md:90`'s *"never a silent already-current"* — the fold is fail-closed
  (`migrate-corpus.missing-snapshot`), the enumeration feeding it is not. Widening the walk widens the
  collision surface and **flips adopter `validate` 0→1** on a doc it now sees (the intended
  tightening, to be stated). [cap B6, dt B4, dec S4, S5]
- **G-18 · C-2's declared residual rests on a falsified premise**: `validation.md:710` says *"that
  one cell"*; driven, **5 doctypes under `placement-root: .`** and `validate` exits 0 *"validates
  clean"* over three stamped orphans. That line **is** C-2's owed disposition; the cheap cut (restate
  as declared) is honest only if fork 4 lands without a bump — it does. `decisions-pending.md:309`'s
  namespaced-stamp trigger fires only on a home-moving answer, which this is not. [dec B11, docs M3, B6]
- **G-19 · `ValueRemapped` on an `id-from` enum re-mints the item id** (`### changed {#changed}`,
  driven) — the act `retitle-item` refuses and both manifests declare un-migratable; `transform.rs`
  has zero `id_from` hits. The correct arm is the plain-enum `fold-refused`, not `prose-needed`.
  [dt M3; baseline §1.5]
- **G-20 · `reconciliation.md:186`/`:47` (*every absorb surfaces*) is violated at `ingest` (D-1) and
  `rename` (baseline L-1) with no door exemption written**; the classifier is correct, only the
  surface is missing. `validation.md:723`'s one-predicate rule has no owner for `relocate`, which
  lands `adr:My Note` managed at exit 0. [docs B8, dec B9, dt M4]

### Fork 5 — the `<slug>` head

- **G-21 · `singleton` and `placement` are two predicates for one rule and diverge on a supported
  shape** — driven on a manufactured pack: `adr` with `location:` + `singleton: true` creates,
  authors, finalizes and promotes at exit 0 while `doc show adr:bogus` gives a generic
  `store.not-found` routed at a verb that refuses. `store.rs:254` and `doc.rs:6259-6262` key on
  `placement.is_some()`; `fixed_title`/mint/`projection_home_line`/the copy-in fence key on
  `Schema::singleton`. `storage.md:216` (*"nothing enforces placement ⇒ singleton… it needs no
  fence"*) is silent on the converse — candidate **revise**. [dt B2, hunt (b)]
- **G-22 · The landing site and its buried limitation**: `canonical_path` (`store.rs:49-55`) never
  reads `slug` on the placement branch; the shipped read-guard's own comment (`store.rs:241-248`)
  says *"read-scoped: the write/promote/reconcile callers use `canonical_path` directly and are
  unaffected"*. The predicate is one line (`:254`); `parse_verb_addr` (`doc.rs:6122`) already holds
  the pack; a guard inside `canonical_path` reaches ~25 production call sites; M50's `Address::parse`
  reddening does **not** transfer (absent for a well-formed token). `rename --slug` is a committing
  mover none of fork 5's three stated arms reaches. [cap B5, dec B4]
- **G-23 · The pinned schema projection teaches the exact bogus address** — `doc schema vision
  --format json` (contract-version 6) emits `"set-slot": "vision:<slug>#thesis"` and no
  `singleton`/`placement`/`location` key for any of 16 doctypes; the methodology pack's own
  `author-vision` step composes `jigc doc schema vision` as *"the authority"* two lines after saying
  *"the slug is fixed"*. `projection_home_line` (`compose.rs:1194-1215`) already answers for all three
  home shapes. **Contract-version 6→7 is a one-way door** (`doc-read-surface.md:108` forbids additive
  carve-outs; `:90` closes at the pin). Pool with **LD-2** (`base` as `string` vs `{sha, short}` —
  and the schema says `set: on-create` where three records name it the worked example of an
  `on-transition` absolute, `milestone-record.yaml:62`) into **one** bump. [dt B1, B3, M1, hunt (a)]

### Fork 6 — the reject envelope and the window-close set

- **G-24 · DEFECT A onto a declared arm is one line and two call sites, but the arm matters**: the
  bare `Finding` carries `key:{code,target}` while the declared `{error}` arm is single-key — routing
  there is a **regression in disguise**; only `{findings, schema_version}` or a declared third arm
  keeps the key, and neither moves a version (`result.rs:27`). [cap M2, baseline §1.6]
- **G-25 · DEFECT B costs the doc nothing and the registry everything**: `doc-read-surface.md:71-78`
  declares six fragment shapes and is **right**; `render.rs:5096`'s *"`task list` is the surface's one
  array"* is wrong; 17 cells / 7 shapes need **+3 rows and ≥1 new `ArmShape`** (the baseline's *"no
  new ArmShape"* was measured for DEFECT A only). `command-output-contract.md:181`'s refusal to reshape
  `task list` is the paragraph any reshape argues against. [docs M1, cap M3, dec hunt (b) 5]
- **G-26 · DEFECT D's seat exists, the class is 26 sites in 2 shapes, and one shape cannot ride it**:
  `Cli::dispatch` (`cli.rs:568`) holds `self.format`; the 24 `current_dir()` sites are per-verb
  `eprintln!`; `pack.rs:1786/:1793`'s warnings run **after** dispatch, print a **host absolute path**,
  and falsify `repo_relative_paths.rs:746`'s `pack.rs` row reason. [cap B7, baseline §1.6]
- **G-27 · `ArmOutcome` vs the exit taxonomy is a code-comment defect; no doc moves** —
  `command-output-contract.md:457` names the taxonomy the one-way door; do not reword it. [docs M2]

### Composed surfaces and adopter docs

- **G-28 · `surface-contract.md:202` claims pack `jigc` lines are fence-covered post-compose — false
  at HEAD** (13 workflows compose unrunnable `Run:` lines at exit 0); that sentence is A6-2/A6-3's
  leg-1 citation. The off-verb exit-4 promise is false because no `source-path` seam exists
  (`task.rs:2026`'s `is_migration` false) and **no fence protects the sentence** (`finalize.promote-
  clobber`'s tokens survive elsewhere in the body) — the fix belongs at the **compose door**, not the
  12 steps. `write-commands.md:185`'s *"the two forms that start work"* is itself false (`MINT_DOORS`
  = 5). [docs B7, M6; dt M2]
- **G-29 · The *one batch, one hash move* rule for guide bytes is written nowhere** (only
  `decisions-pending.md:387` as a past fact); home `assistant-adapter.md:31`/`:38`. Every guide byte
  moves `jigc-body-blake3`. [docs M7]
- **G-30 · No `design/` part-doc is inside `count_fences.rs` HOMES** (`count_fences.rs:93-121`), so
  every count M52 moves is unfenced prose — six enumerated in gap-docs, plus `UNSWEPT_PRODUCERS` at
  **79** (`dp:729`) vs **60** (`dp:83`). Rider: add the homes on the `doctype_map_versions` mold. [docs B11]

### Harness

- **G-31 · No git-in-progress fixture builder exists** in the rig or `trial_corpus.rs` (the posture
  auditor built eight states by hand) — size as its own increment; **no population was ever raced by
  a real process** (a hook cannot stand in for the fan-out third party, and `unwind_mint` runs before
  any commit); `chmod 000` arms pass silently on ubuntu CI. [cap M6]

## MAJOR (folded into the forks above where they belong; listed for the ledger)

`ENVELOPE_ARMS`' four `doc show` rows are `ArmOrigin::Dispatch` (D5-legal) [cap M3] · `DESTROYING_DOORS`
admits rows trivially but its vocabulary is worktree-shaped; `flow49_acceptance.rs:999-1051` reds
until both matrices grow [cap M4] · `corpus-migration.md:75-81` describes `relocate`'s freeze-exempt
path whose domain is **empty** on a stock corpus (A1-D3's home) [docs M5] · `storage.md` → Concurrent
writers' base-relative merge vs `rollback_rename`'s unconditional `file-state.json` rewrite [docs M8]
· `engine/ingest.rs:226-228` re-derives an identity `cli/ingest.rs:555` already computes [dt M4] ·
`worked-examples.md:3445`'s *"three of four members"* is a member count where the failure is a
**state** count [dec S3].

## Open leads, re-dispositioned

Lead 3 **discharged** (built + proven, with a new item: the boundary commit is created and orphaned
before the live refusal). Lead 1's trigger **likely already fired unnoticed** (M51's audit added a
span fence to all three `Route` constructors after the review recorded it). Lead 2 likely fires
(`staged_doc_ids` never asks `is_file()`). Lead 5 likely fires (A6-1/2/3 move composed lines). Lead 6a
**fires** (tier 1 *is* a route-floor sweep; `UNSWEPT_PRODUCERS` is its home). Leads 4, 6c, 6d stay
open. [dec open leads]

## Must NOT re-open [dec (c)]

The v1 freeze (no fix needs a hash) · `GIT_DIR`'s mechanism (rationale unfalsified — scope it in
writing, G-11) · the `Plain` registry as a classified list · `store.malformed-slug` outside
`Address::parse` (M50, measured — and non-transferring here, G-22) · EC-9/publishing floor ·
posture-no-override. **The pre-pin window is open** (`command-output-contract.md:476-488`: additive
keys, removal of *undeclared* keys, reshape of a declared arm, each declared in the same motion).

## What the detectors did not reach

Three cells driven by the decisions detector, the rest read or relayed; no cargo, no debug binary,
so every fence-quality claim is a read. The `from`-home acceptance sentence (`corpus-migration.md:299`)
is pinned by **no test** (grepped by the orchestrator: no hit), so it is a doc-vs-code contradiction
and not a false pin.
