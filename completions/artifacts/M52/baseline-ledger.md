# M52 — the baseline ledger

**Driven 2026-09-16/17, at `HEAD = 7637a46f`** (branch `main`, clean tree), against the **installed
release** binary `~/.local/bin/jigc` = **`1.0.0-rc.15`** (sha256 `126f1584…`, built from `577a0099`;
HEAD's production code is byte-identical to that sha — only `Cargo.toml`'s dev profile and one test
file changed since). Fixtures from `dev/jigc-rig <state> --binary ~/.local/bin/jigc` (two-step eval,
`cd "$REPO"`, roots under the session scratchpad). **Debug posture was used nowhere**; **no auditor ran
cargo; none wrote into the working repository** (`git status` clean after all seven).

**What this baseline is for.** The evidence base — [the M51 per-axis review](../M51/per-axis-review/README.md)
— already verified all 39 §A rows with repro blocks on this exact binary. So this baseline **did not
re-verify a single row**. It drove the **classes** behind them: the axis each fix must iterate, because
M51's audit widened every class it was handed and the review's reconciliation widened two more. **A row
in the review is a verified instance; the class is what the wave ships** — and every class below came
back wider than its row states.

**Provenance.** Seven **Opus `capability-auditor` subagents**, one per class, each driving the binary
and returning argv + observed lines per verdict. Their ledgers are archived beside this file,
verbatim, each with its own provenance and *honest bounds* section:

| companion | class | review rows it drives behind |
|---|---|---|
| [baseline-rollback.md](baseline-rollback.md) | every capture/restore population in jigc's transactions | axis 4: C1 · C2 · C3 · C4 · DEFECT 1 · DEFECT 2 |
| [baseline-posture.md](baseline-posture.md) | the posture family × every acting door | axis 2: D1 · D2 · D3 · D3b · codex-1 · open lead 3 |
| [baseline-destroying.md](baseline-destroying.md) | the destroying-door subject × every byte shape in the workbench | axis 3: C-1 · D-1 · D-2 · D-3 · D-4 |
| [baseline-tokens.md](baseline-tokens.md) | the `<slug>` head, the OS ceiling, `relocate --from` and the root knobs | axis 1: A1-D1 · A1-D2 · A1-D3 · A1-D4 · A1-D5 |
| [baseline-freeze.md](baseline-freeze.md) | the corpus walk's key, `ingest` over file-state, orphan territory, the relocation route | axis 7: D-1 · D-2 · D-3 · D-4 · C-1 · C-2 |
| [baseline-contracts.md](baseline-contracts.md) | the reject envelope at every leaf, pre-dispatch failures, `doc show` projections, `ArmOutcome` | axis 5: DEFECT A · B · C · D |
| [baseline-surfaces.md](baseline-surfaces.md) | composed surfaces × posture, composed argv, empty enumerations, adopter docs, help | axes 6 + 8: A6-1 · A6-2 · A6-3 · CX-1 · CX-2 · CX-3 · D-1 · D-2 |

Citations below are `[rollback §n]`, `[posture §n]`, … and point at the companion's own section (§1
enumeration · §2 drives · §3 what changed against the rows · §4 latent defects · §5 bounds).

**A map, not gospel — and relayed, not verified by the orchestrator.** Every fact here is a subagent's
driven report. The Settle re-drives the rows it turns on, and the gate-record records which
(`claim-driven`). Nothing here is settled.

---

## 0 · The headline — every class is wider than its row, and the widening has one shape

Seven classes driven; **seven came back wider**. The shape is the one M45 named and M49–M51 kept
re-finding: *a fix scoped to the reported repro's door set, suffix, noun or locus misses the sibling
cell where the identical mechanism fires*. Concretely:

| review row | the row's class | driven class | companion |
|---|---|---|---|
| axis 4 R7 | **4** worktree-restore populations (config-layer CAS · promote · retire · record) | **9**, exactly one of them CAS; **5 sit outside the finalize executor** and are reached by 3 doors (`rename` · `doc author` · `config set`) **not in axis 4's 13-door set** | [rollback §1, §3] |
| axis 2 D1/D2/D3 | rebase/bisect masked; `rebase-apply` = `am`; cherry-pick + revert unhandled | the state axis is **8 un-concluded operations + 2 HEAD postures + a conflicted index with no marker**; `InProgress::markers()` carries **4 markers over 3 operations**; **two operations write no marker at all** (squash merge, conflicted stash-pop) | [posture §1, §3] |
| axis 3 C-1 | *a non-`.md` file* under the task area, 5 doors | **predicate misnamed**: an ordinary `NOTES.md` at the area root dies identically — the class is *everything in `.jigc/tasks/<id>/` that is not a `docs/`-level `.md`*; the 5 doors hold | [destroying §3] |
| axis 1 A1-D1 | a bogus `<slug>` head on a **singleton**, 5 write doors | **the boundary is `placement:`, not `singleton:`** (a manufactured `singleton: true` + `location:` doctype refuses); subject = the 5 shipped placement doctypes with a committed instance; **6 write doors + 1 read door** (`rename --slug` — a committing mover — and `milestone add-from-spec`); and **one** alias is a *working write to the real file under an identity every read refuses*, not only ≥2 aliases ⇒ loss | [tokens §3] |
| axis 7 D-2 | a `Relocated`-only bump invisible on both home kinds | **the bump kind is irrelevant**; the class is the **(from-home × to-home) pair** and **3 of 4 cells are uncovered** (`L→L`, `P→P`, `P→L`) — only the one M38 shipped (`L→P`) works, because `candidate_docs` unions prior **`location:`** homes and never reads a prior **`placement`** | [freeze §3] |
| axis 5 DEFECT B | six `doc show` projections where four are declared | **17 addressable cells / 7 distinct root shapes**; `ArmShape::ArrayOf`'s *"the surface's one array"* falsified **twice** (array-of-items **and** array-of-strings on a `ref` leaf) plus an object-valued single field leaf | [contracts §3] |
| axis 6 A6-2 | one workflow composes unrunnable `Run:` lines | **13** — the 12 `migrate-*` workflows compose identically at exit 0 with `{{ source }}` empty | [surfaces §3] |
| axis 6 A6-3 | one empty render | **14 renders across 13 workflows** | [surfaces §3] |
| axis 8 D-2 | `task discard --help` never says it commits | **4 of 10 `COMMITTING_DOORS`** rows, **2 silent in the ack too** | [surfaces §3] |

## 1 · What the baseline changed in the charter — per class

### 1.1 The rollback family (fork 1) — nine populations, three door guards, one CAS

- **Nine worktree-restore populations, one CAS** [rollback §1]: the config-layer CAS (the control) ·
  promote · retire · `RecordPreImage` · `RecordFlipGuard::Drop` · `rollback_rename` · `CreatedDoc::rollback`
  · `unwind_mint` · `unwind_unrecorded_seeds` — plus the rollback-**less** `config set <root>`
  relocation. Greps recorded with what they miss (the two restores that are not functions — `Drop for
  RecordFlipGuard` and the inline `fs::write` in `rollback_promotions`). Five index-restore populations
  exist beside them, none defective, none driven.
- **Three doors outside axis 4's set reach the class**: `rename`, `doc author`, `config set` [rollback §3].
- **Two populations already restore correctly under a door guard instead of a CAS** (`rename.dirty-tree`;
  `setup`'s pre-write refusal) — so *"make them all CAS"* is not forced by the evidence; **two
  populations restore a directory tree**, the exact shape `design/finalize.md` cites to exclude
  `.jigc/config` from the CAS set; and **trackedness changes a conflict's severity, not its existence**
  (four populations restore gitignored paths no object DB holds) [rollback §3].
- **`retire`'s already-absent arm** reaches one door only (`task finalize --approve`) and is silent at
  every cell there [rollback §3].
- **DEFECT 1 has a second producer** [rollback §4 L9]: `config set docs-root --format json` prepends
  relocation prose to a stderr-borne `{"error": …}` (leading garbage, where `carry_rollback_conflicts` is
  trailing); the other five reject-class doors parse clean.

### 1.2 The posture family (fork 2) — the state axis, the commit model, and the consent constraint

- **State axis** [posture §1]: merge · rebase (merge backend, HEAD detached) · rebase (`--apply`
  backend, HEAD detached — **a third masked cell**) · `git am` · cherry-pick (single) · revert (single) ·
  multi-commit pick/revert (`sequencer/`) · bisect (before the first good/bad HEAD is **attached** and
  the door is **correct** at all 12; after, detached and masked) · detached · unborn · **conflicted index
  with no marker** (squash merge, stash-pop). git **2.54.0** throughout — every marker fact is a property
  of this git.
- **D1's discriminator is `HeadDetached ∧ OperationInProgress`**, not "rebase or bisect" [posture §3].
- **D2 cannot be fixed by renaming the noun**: `rebase-apply/` is written by `git am` **and** `git rebase
  --apply`, and for the latter `git rebase --abort` is correct. git's own discriminator, driven both
  ways: `rebase-apply/applying` ⇒ am, `rebase-apply/onto` ⇒ rebase [posture §3].
- **D3 is decided by the commit model**: whole-index `git commit -F` (`task finalize`, `rename`)
  **consumes** the operation; pathspec `git commit -F -- <p>` is stopped **by git**, not by jigc — and
  only under a cherry-pick, because git's `determine_whence()` reads `CHERRY_PICK_HEAD`/`MERGE_HEAD` and
  **never `REVERT_HEAD`**. So **a revert is concluded at exit 0 by every commit door incl. the pathspec
  ones** [posture §4 L2], `rename` concludes a cherry-pick [L4], a squash merge (no marker) is swallowed
  whole [L3], and a multi-commit pick leaves `.git/sequencer/` dangling so git's advertised `cherry-pick
  --abort` then **destroys jigc's commit and files** while jigc records the task finalized [L5].
- **The constraint a consent answer inherits** [posture §3]: a `git commit` under
  `CHERRY_PICK_HEAD`/`REVERT_HEAD`/`MERGE_HEAD` **is** that operation's conclusion to git — it clears the
  marker, consumes `MERGE_MSG`, records merge parents; no `git commit` argument makes it not conclude.
  A consent can only mean *"consent to conclude the user's operation under jigc's subject"*, and today
  `--force`/`--carry-staged`/`--dry-run` are leaf-keyed and carry no posture consent — any consent flag
  is a **new** one.
- **D3b is a cause axis** (≥4 causes rendered as *"Fix the hook's complaint"*: hook · partial commit
  during cherry-pick, driven at 4 doors · unmerged index · gpg signing) [posture §3, §4 L6, L8].
- **codex-1 is one site, not a door class**: 11 production index-mutating sites — 1 probed immediately
  (`git mv`), 9 covered by the commit seam's re-probe (driven, index rollback observed), **1 uncovered**
  (`relocate.rs:314 git rm --cached`) [posture §3].
- **Open lead 3 is discharged as built + proven**: a bisect planted between the door and the ff was
  caught, `--ff-only` never ran, the index rolled back. New fact: the dedicated worktree's posture is
  per-worktree, so the boundary commit **is created and orphaned** before the live refusal [posture §3].
- **Movers act inside a sequencer index** [posture §4 L7]: `config set docs-root` git-mv's four docs at
  exit 0 under a cherry-pick; `git cherry-pick --abort` undoes them and leaves `validate` red with four
  blocking `reconciliation.rename` routed at `jigc unmanage`.
- **`task validate` exits 0 where `task finalize` exits 1 on `repo.operation-in-progress`** — `GATE_COVERAGE`
  has **no posture row**, so the *"previews what finalize gates on"* claim on 8 surfaces is one family
  short [posture §4 L1].

### 1.3 The destroying-door subject (fork 3) — the subject rule the evidence forces

- **C-1's predicate is misnamed** (see §0); the five doors hold; `task finalize`'s commit held only the
  README and 5/5 marker strings were in **no git object**; symlinks never traversed — no out-of-repo
  loss on any door [destroying §2, §3].
- **Two subject derivations, and two adjudications** [destroying §3]: worktrees are derived **on disk**
  and adjudicated by `git status --porcelain --ignored` — every planted shape refuses un-forced and is
  narrated forced, **gitignored bytes included**; task areas are derived on disk but adjudicated by a
  **filename suffix**. In one `milestone discard --force` run the identical ten shapes were fully
  narrated in the worktree and wholly silent in the task area. The two *sinks* (`remove_worktrees`,
  `cleanup_subtask_areas`) take the **registered set**, so a sink can be narrower than the guard that
  cleared it.
- **M46's `--ignored` argument transfers to `.jigc/tasks/<id>/`, and harder** (measured): jigc itself
  writes `base.json` · `intent` · `workflow` · `staged-snapshot.json` · `docs/provenance.json` into
  **every** task area from `jigc start` onward, plus `roles.json` (refs-post-hoc) and `source` +
  `source-path` (migrate). A rule *"any non-staged-`.md` byte blocks"* fires on **100 % of tasks at 100 %
  of doors**. **The forced shape is a complement of jigc's own writer set** (code-side derivable:
  `engine/src/state.rs`'s seven `*_FILE` constants + `cli/src/migrate.rs::SOURCE_FILE`), never a shape
  or suffix claim [destroying §3].
- **D-1 is exactly 2 route producers** (`task.rs:799`, `setup.rs::staged_prose_finding`) of 6 that name
  `jigc task finalize`; both run verbatim → exit 3. Unreported second leg: the other exit both offer,
  `task discard <sub> --force`, **settles the sub-task in the committed milestone record** as a
  side-effect the route never states [destroying §3].
- **D-2 is class size 1** (one fixed string, `milestone.rs:3298`); the `milestone finalize` sibling's
  ack is honest. **D-3 is 3 doors, not 2** (`milestone.staged-prose` prints the same host path from the
  same producer `task.rs:697`); the `UNSWEPT_PRODUCERS` count is right and its reason is false for 1 of 3.
  **D-4 confirmed**, and widened into L-1 below [destroying §3].

### 1.4 The `<slug>` head and the root knobs (fork 5)

- **Boundary `placement:`, subject 5 doctypes, 6 write + 1 read doors, two consequences** (see §0)
  [tokens §3]. The pinned contract carries the bogus identity: `doc set-slot vision:bogus-head#thesis
  --format json` → `{"findings":[],"target":{"slug":"bogus-head"}}`; `finalize --dry-run` shows the
  staged-copy advisories not at all; `doc schema --format json` (contract-version 6) projects **no**
  `singleton`/`placement`/`location` over all 16 doctypes — a driver cannot learn the rule from the
  pinned surface. `task bind`'s singleton cell is unreachable (one `reads` role in both packs, typed
  `spec`).
- **A1-D5 names one knob; `ROOT_KNOBS` has two and they fail differently**: `config set placement-root
  <300×a>` → **exit 0**, per-doc move failures narrated *inside the success ack*, `doc list` silently
  drops two managed docs (no `orphaned` key), `doc show` not-found, **`validate` exit 0** — a false green
  where the `docs-root` sibling at least exits 1 [tokens §4 defect 4]. Both knobs accept `""` at exit 0
  acked as `= .` (M50 records `""` and `.` as *unmixable at the type level*) [defect 6] and `x/../y`
  silently normalized [defect 7].
- **A1-D3's class is 2, not 1**: `relocate` (hard-unreachable) plus `SLUG_DOORS`' `rename` row, whose
  slug guard is reached only when `adr:keeper` exists [tokens §3]. **`relocate --from`**: only
  `.jigc`/`.claude` move (prefix containment over `git ls-files`); `.`, `..`, absolute, symlink, `.git`
  reach nothing; and the residue of A1-D2 is an **unaddressable managed identity** — `relocate adr --from
  notes` over `notes/My Note.md` lands `adr:My Note` **managed** at exit 0, `doc show` refuses it, the
  route loops — exactly the state `design/validation.md:723` says `ingest.unaddressable-identity` exists
  to prevent, and `relocate` asks no predicate [tokens §4 defect 5].
- **Corrections to the rows**: A1-D4's `rename` cell (`rename.in-flight`) is a rig artifact — with no
  task open it is `write.identity-change`/`store.not-found`; the count of 5 stands, the reason does not.
  A1-D5 understates the second knob's failure (a validate false green, not "validate cannot run").

### 1.5 The corpus walk, `ingest`, and orphan territory (fork 4)

- **Closing D-2 moves no schema hash, bumps no `schema-version`, migrates no corpus** [freeze §3]. The
  prior home is **already recorded in full**: `schema-snapshots/<ty>.v<k>.yaml` is the whole prior
  `Schema` (`location:` *and* `placement:`), `pack::prior_doctype_schemas` already loads every prior
  shape and is consumed at **7 sites**; the manifest carries no history; `doctype-authoring.md:21`
  already obliges the snapshot on a home change. The fix is entirely `candidate_docs`' enumeration
  (`migrate_corpus.rs:2075-2131`: the `to.location` branch loads no snapshot; the `to.placement` branch
  unions prior `location:` homes via `.filter_map(|p| p.location)` at line 2104 and never a prior
  `placement`). **Two constraints**: a prior `placement.file` with a leading component needs the
  `placement-root` knob, which `DoctypeMigration` (`:76-81`) does **not** carry (it carries `docs_root`
  only); and widening the walk widens the destination-collision surface `destination_collision_route`
  (`:1982`) exists for. **Sentences contradicted**: `design/corpus-migration.md:68` and `:299` (*"the
  corpus walk keys on the `from` home"*), the doc-comment at `migrate_corpus.rs:2072-2074`, and `:90`
  (*"never a silent already-current"* — the doc is in **no** bucket). `design/storage.md:208`'s census
  row is **not** falsified.
- **D-3's axis is the `id-from` role, not the locus**: `id-from` enum at locus 2 *and* 3 both dead-end
  at `prose-needed`; the plain-enum cells at the same loci take the correct `fold-refused` arm; a nested
  `category` rename (`releases/1-0-0/changes/changed`) is the same dead end [freeze §3, §4 L-4].
- **D-1's door set is `{ingest, rename}`** of 6 candidate durable-baseline doors; `migrate-corpus`
  driven *not* to absorb; the classifier is correct (a non-conformant drift refuses `needs-reconcile`) —
  only the *surface* is missing. **`rename` over a drifted committed adr** → exit 0, commits, re-keys
  the baseline to the post-drift hash, the blocking `file-state.hash-matches` gone, no absorb line
  [freeze §4 L-1].
- **D-4's family is 7 producers / 4 doors**; the sharper pair is `ingest` (blocking) vs `validate`
  (silent, exit 0) at a root-placement home [freeze §3]. **C-1's class is `stem(placement.file) != ty`** =
  2 of 5 placement doctypes (`vision`, `changelog`), only `vision` carrying a `ref`; `ingest` on a
  healthy `refs-post-hoc` corpus already holds **both** `vision:VISION` and `vision:vision` in
  `edges.json` [freeze §4 L-2], and `unmanage VISION.md` acks *not managed (nothing to drop)* while the
  edge survives [L-3]. **C-2 is not one fixed cell**: 2 doctypes at the pack default, **5 under
  `placement-root: .`** — and under that knob with the pack dropped, **`validate` exits 0 "validates
  clean" over three stamped orphans** [freeze §4 L-5].
- **`conformance.section-renamed`'s route-exemption reason** (*"no address to route at"*) is falsified
  by `ingest`, which prints a followable route for that code at the same commit [freeze §4 O-1].

### 1.6 The pinned contracts (fork 6)

- **DEFECT A's door bound of 2 holds** across 141 leaf-cells (47 leaves × {outside a git repo · a git
  repo with **no `jigc setup`** — the cell the review never entered · unknown id/address}), **zero new
  offenders** — but its **code** bound does not: **6 driven codes, not 4**, out of ≈30 `setup.*`/
  `uninstall.*` codes reaching the same producer [contracts §2, §3].
- **The two fix shapes, measured** [contracts §3]: onto a declared arm = **1 production function**
  (`render::setup_block`, `render.rs:3383`, the line `Format::Json => json(finding)`) · **2 call sites**
  (`cli.rs:738`, `:766`) · **1 in-crate unit test** pinning the bare shape (`render.rs:6085`); both doors
  keep `Result<_, Finding>`, `setup.rs` does not change, and reusing the `{findings, schema_version}` arm
  moves **no version**. Declaring a third arm = +1/+2 `ENVELOPE_ARMS` rows, no new `ArmShape`/`ArmOutcome`
  member. **Correction to the charter's premise**: `doc schema`'s `contract-version` (6) is **not**
  involved — the envelope version is `engine::result::SCHEMA_VERSION = 3` (`result.rs:27`).
- **DEFECT D — 11 pre-dispatch failure points; 2 bypass or corrupt both funnels**: the `current_dir()`
  family (**24 sites**: `cli.rs:540` + 23 match arms) and **LD-1** — a malformed project `packs.yaml`
  prepends 2–4 plain `warning:` lines to stderr under `--format json` (producers `pack.rs:1786`, `:1793`,
  format-blind), a **one-character typo in a file the operator is invited to edit**, which overtakes
  DEFECT D's *"exotic"* bound and makes the class **26 sites** [contracts §2, §4 LD-1]. **A single
  pre-dispatch funnel is armable with the format in hand**: `--format` is fully parsed and validated
  before any `current_dir()` (driven from a deleted cwd: `--no-such-flag` → clap exit 2; `--format zzz`
  → exit 2; `--version` → exit 0), and `Cli::dispatch()` (`cli.rs:568`) already holds `self.format`.
  Collateral: `refuse_on_posture` reads `current_dir().ok()?` and **silently skips the whole M51 posture
  family** on that fault.
- **`ArmOutcome` row count 3 holds**, but `validate | StoreSweep` flips on **4 of 6** `STORE_EXIT_FLIPS`
  members driven, `migrate-corpus | Report` on 1, `task validate` → 3 re-driven; the class cannot widen
  further — `validation_store_exit_flips` has one production call site (`cli.rs:1094`) [contracts §3].
- **Three more latent** [contracts §4]: **LD-2** two pinned contracts disagree on one field's type
  (`doc schema milestone-record` says `base: string`; `doc show …#meta/base` returns `{sha, short}`) ·
  **LD-3** **6 of 9 `milestone` doors run at exit 0 over a repo with no `jigc setup`**, incl. `provision`
  (creates a real worktree) and `finalize` (exit 3, a real adjudication), while 21 of 47 leaves refuse
  that state — only `milestone execute` refuses · **LD-4** an unreadable `.jigc/` (`chmod 000`) is
  reported as *"this project isn't set up — run `jigc setup`"*.

### 1.7 Composed surfaces and adopter docs (tier 2 + A6-1)

- **A6-1 is 2 doors × 4 states, all one byte-string** (`start --task` undiscriminated too; the
  provisioned-wrong-checkout state reuses the never-provisioned refusal **verbatim**, whose leading route
  `jigc milestone provision …` is a no-op in half the states that print it). **A loss narration exists**
  at the boundary and sweeps the **worktree** axis, not the **shared-checkout-staged** axis (1 of 2).
  **The row's repro needs a correction**: state (c) needs a detached HEAD (the record commit always
  moves `main` past the pin) and `milestone finalize` exits 1 there on `repo.head-detached`, so the
  repro needs a `git checkout main` in between [surfaces §3].
- **Two off-verb composed lies, driven to their consequence** [surfaces §4]: `jigc start --workflow
  sub-task "…"` mints a milestone-less sub-task whose body says *"never `jigc task finalize` here"* and
  `task finalize` then lands it at exit 0 (defect 1); `jigc start --workflow migrate-adr …` then plain
  `task finalize` → **exit 0, promoted the doc** — against the composed body's *"a plain finalize commits
  NOTHING … holds (exit 4) … `--approve` is the one destructive gate"*, which 12 workflows carry
  (defect 2).
- **`task validate` in the shared checkout at the base pin** reports `reconciliation.rename …
  milestone-record:<id> … is missing` routed at `jigc unmanage` — the record is not missing, the checkout
  predates it; clean inside the worktree [surfaces §4 defect 3].
- **CX-1 is 2 unqualified claims** (+ `jigc validate --help`'s *"read-only sweep"*), both falsified by
  the same invocation-log write; the 3 *qualified* siblings are true. **CX-2 is 3 sentences in 2 homes,
  one generated by the binary** (`setup.rs:80` → installed `SKILL.md` line 8). **CX-3 is bounded**: the
  preamble disclaims *documents*, and `crates/cli` is the guide's only non-document repo path. **Premise
  corrected**: `SKILL.md` is **not** a verbatim copy of the two guides (link transform; QUICKSTART
  matches 9 766/14 978 chars) [surfaces §3].
- **D-1 confirmed and the dry-run text arm also drops `changelog-recording.gate-granted-unused`** (a
  different family); the producer half is structurally **9 `DocAck` arms**, 1 driven [surfaces §3].
  **D-2 is 4 of 10 `COMMITTING_DOORS`** (`task discard` · `milestone create` · `add-task` ·
  `add-from-spec`), and `add-task`/`add-from-spec` move HEAD with **neither help nor ack** naming a
  commit [surfaces §3, §4 defect 4].

## 2 · Latent defects across the seven classes — the exit-0 loss and harm cells first

Every entry is driven and in no §A row; argv in the companion.

**Byte loss or repository harm at exit 0 (the tier-0 shape):**
1. **`unwind_mint` `remove_dir_all`s the minted area**, destroying third-party authored prose in
   gitignored `.jigc/tasks/<id>/docs/` — no git copy, no note [rollback §4 L6]. *The most severe cell
   in the rollback class.*
2. **`milestone create`'s absent-pre-image arm deletes a file a racer created at the record path**, no
   finding; population 1's CAS answers the same cell correctly in the same binary [rollback L1].
3. **`task discard <sub> --force` and `milestone discard --force` are a fifth and sixth
   `RecordPreImage` caller** clobbering silently; `design/finalize.md`'s M47 row enumerates four doors
   [rollback L2, L3]. **`RecordFlipGuard` clobbers identically at `squash: false`** [L4].
4. **`rename`'s `rollback_rename` removes the new path unconditionally with the racer's bytes in it**
   and rewrites `file-state.json` unconditionally [rollback L5].
5. **`jigc milestone discard` fails OPEN on an unreadable `.jigc/worktrees/`** and settles the record
   irreversibly at exit 0 (no `--force`): both live worktrees still hold uncommitted work, records
   pruned, `status: discarded`, re-entry refused `milestone.terminal`. Root cause: `leftover_at` maps
   every `symlink_metadata` error to `Absent` and `probe_leftover` returns `None` for `Absent` —
   contradicting its own doc-comment and `held_subtask_worktrees`' stated fail-closed rule
   [destroying §4 L-1].
6. **`jigc uninstall` destroys a plain file at any `ENTRIES` name at exit 0**, no `--force`, no
   narration — `workbench_paths` excludes on `file_name() == prefix`, a name match with no shape check,
   against its own doc-comment recording M49's opposite lesson [destroying L-2].
7. **`jigc rename vision:alpha --to Phantom --slug alpha` → exit 0, commits, rewrites the real
   `VISION.md` H1**; the no-`--slug` refusal's own route composes the caller's bogus slug into the escape
   hatch that lands it [tokens §4 defect 1]; sibling: `--slug vision` rewrites a `display-title:`
   singleton's schema-supplied H1 while `doc rename` refuses the identical act [defect 2].
8. **A revert is concluded at exit 0 by every commit door**; a cherry-pick by `rename`; a squash merge
   swallowed whole; a multi-commit pick leaves `sequencer/` dangling so `git cherry-pick --abort`
   destroys jigc's commit [posture L2–L5]. **Movers act inside a sequencer index** [L7].
9. **`jigc rename` over a drifted committed adr commits and re-keys the baseline past a blocking
   `file-state.hash-matches`**, no absorb line [freeze L-1].
10. **Plain `task finalize` on an off-verb `migrate-*` task promotes the doc at exit 0** against the
    composed body's exit-4 hold [surfaces defect 2]; a milestone-less `sub-task` lands at exit 0 against
    its body's *"never finalize here"* [defect 1].
11. **`chmod -w .jigc/config; config set docs-root papers`** → staged `git mv` + file-state re-key
    land, the knob write fails, **no rollback** [rollback L8]; **`config set placement-root <300×a>`**
    → exit 0, `doc list` drops two managed docs, `validate` exit 0 [tokens defect 4].

**False greens and silent state (the store lies):**
12. **`validate` exit 0 "validates clean" over three stamped orphans** under `placement-root: .` with
    the pack dropped [freeze L-5]; a `CHANGELOG.md`→`HISTORY.md` move plus content change makes even
    `ingest` file it as *"fine to stay plain"* [L-6].
13. **6 of 9 `milestone` doors run at exit 0 over a repo with no `jigc setup`** [contracts LD-3].
14. **`relocate` lands an unaddressable managed identity** (`adr:My Note`) at exit 0 [tokens defect 5].
15. **`task validate` exits 0 under `repo.operation-in-progress`** where finalize exits 1 —
    `GATE_COVERAGE` has no posture row [posture L1]; **`task validate` in the shared checkout at the
    base pin** reports a phantom missing record routed at `unmanage` [surfaces defect 3].
16. **`staged_doc_ids` never asks `is_file()`** — a directory `docs/fake:thing.md` is a staged doc at
    three doors and `managed` on the pinned `doc list --task` [destroying L-3].
17. **`ingest` on a healthy corpus holds both `vision:VISION` and `vision:vision`** in `edges.json`;
    `unmanage VISION.md` acks *not managed* while the edge survives [freeze L-2, L-3].

**Stream discipline and machine surface:**
18. **A malformed `packs.yaml` breaks the JSON reject stream at every leaf** (LD-1); **`config set`'s
    relocation prose prepends to `{"error": …}`** (rollback L9).
19. **`doc schema` and `doc show` disagree on `milestone-record`'s `base` type** [contracts LD-2].

**Surface (law 1):**
20. `milestone finalize` over an unwritable `.jigc/worktrees/` exits 1 with raw git stderr, no code
    [destroying L-4] · an unreadable `.jigc/` reported as *"not set up"* [contracts LD-4] · `commit.gpgsign`
    failure and an unmerged index framed as a hook rejection, the latter with git's stdout/stderr
    concatenated with no separator (`task.rs:5433`) [posture L6, L8] · `jigc validate --help`'s
    *"read-only sweep"* falsified by the log write [surfaces defect 5] · `milestone create '<existing
    title>'` amends `.jigc/.gitignore` then refuses `milestone.record-exists` with zero notice — C4 is
    **two doors** [rollback L7] · `docs-root`/`placement-root` accept `""` (acked `= .`) and `x/../y`
    silently [tokens defects 6, 7].

## 3 · Confirmed as the rows state them (drive-once, do not re-derive)

Every §A row the classes were driven behind held at its base cell; **none was refuted**. The
corrections are all of the widening or re-attribution kind and are in §1: A1-D4's `rename` cell reason
(rig artifact) · A1-D5's second knob (false green, not "cannot run") · A6-1's repro (needs `git checkout
main`) · DEFECT A's code bound (6, not 4) · the charter's `doc schema contract-version` premise for
fork 6 (wrong home — `engine::result::SCHEMA_VERSION`) · CX-3's premise that `SKILL.md` is a verbatim
copy of the guides (it is a link-transformed one).

**Open leads re-posed by the baseline.** Lead 3 (the fan-out `--ff-only` re-probe) is **discharged:
built + proven** [posture §3]. The other six stand as the charter disposes them.

## 4 · What the baseline did **not** reach

Each companion's §5 is the bound; the ones the Settle must not read past:

- **No cargo run, no mutation applied** — every fence-quality statement is a read plus a drive.
- **One git version** (2.54.0); the `REVERT_HEAD` blindness in `determine_whence()` and the
  `applying`/`onto` discriminator are this git's on-disk contract [posture §5].
- **`revert × sequencer`, the fan-out boundary under a revert, a mover under a revert, `setup`'s
  install commit and `migrate-corpus`'s self-commit under any sequencer state** — not driven
  [posture §5]. The mid-run plant is a **bisect** (git refuses to start a merge over a dirty index), so
  the seam re-probe is proven for one family member and inferred for two.
- **No population raced by a genuine concurrent process** — only by the hook the design doc names;
  `milestone add-from-spec` not driven at any rollback cell; `unwind_mint`'s two `note:` lines (a third
  DEFECT-1 candidate) left as a lead [rollback §5].
- **`unwind_mint` under a racing writer** not driven from the destroying side; sites #11–#16 (the
  merged-docs rebuild, `index::invalidate`, `CreatedDoc::rollback`, config swap rollback, message
  temps, install footprint) take no user path — stated, not verified; macOS only [destroying §5].
- **`task bind` × singleton unreachable**; `deferral-ledger` driven only uncommitted;
  `WORK_UNIT_ID_DOORS` ceiling sampled 5/25 [tokens §5].
- **`setup`'s `corpus_stale` route undriven** (needs a second binary version); `ValueRemapped` ×
  locus-1 × `id-from` unconstructible; the methodology pack's own manifest never independently
  drifted [freeze §5].
- **`--format agent`/`human` not driven on the contracts axis**; a blocked `milestone join` not driven;
  ≈24 of DEFECT A's ≈30 codes bounded by grep; no project-pack composition driven [contracts §5].
- **29 of 34 composed workflows verified mechanically, not followed to a landed outcome**; 8 of 9
  `DocAck` arms in D-1's class structurally, not by drive; `migrate-corpus --dry-run` measured on the
  no-op path only [surfaces §5].
