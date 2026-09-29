# M52 gap detection — dimension: capabilities

**HEAD `7637a46f`; binary `~/.local/bin/jigc` = 1.0.0-rc.15.** Every claim is marked **[driven]**
(rig + release binary, two-step eval, `cd "$REPO"`, no cargo, nothing written into the working
repo), **[read]** (source read at a cited line — a lead, not a measurement, per M46's rule), or
**[baseline]** (relayed from a M52 companion ledger; not re-driven here).

Rigs used: `dev/jigc-rig {fresh, committed-singletons} --binary ~/.local/bin/jigc`. `git status`
in the working repo is clean after every drive.

---

## BLOCKING

### B1 · The task-area writer set has no code-side enumeration, and the baseline's own allowlist is short by at least 4 names

Fork 3's answer is *forced* to be a complement of jigc's writer set (baseline-destroying §3.7:
a suffix/shape rule fires on 100 % of tasks at 100 % of doors). That complement needs a registry,
and **there is none**. The names are **13 scattered `const`s across 3 files, 11 of them private**:

- `crates/engine/src/state.rs:32,37,42,51,60` — `BASE_PIN_FILE` `ROLES_FILE` `RENAMES_FILE`
  `INTENT_FILE` `WORKFLOW_FILE` are **private** (`const`, not `pub const`) **[read]**
- `crates/engine/src/state.rs:68,77` — `SOURCE_PATH_FILE` `SLUG_OVERRIDE_FILE` are `pub` **[read]**
- `crates/engine/src/state.rs:215,788` — `PROVENANCE_FILE` `STAGED_SNAPSHOT_FILE` private **[read]**
- `crates/engine/src/validate.rs:66,71` — `SNAPSHOT_FILE` (`probe-snapshot.json`),
  `BASE_SNAPSHOT_FILE` (`base-probe-snapshot.json`), private; their doc-comments say *"under the
  task working area"* / *"Same gitignored working area"* **[read]** — so the destroying companion's
  §5 bound (*"may be short by up to three names"*) resolves: short by **exactly two** here.
  `STORE_SNAPSHOT_FILE` (`validate.rs:372`) is store scratch, **not** a task-area name **[read]**
- `crates/cli/src/migrate.rs:63` — `SOURCE_FILE`, `pub(crate)` **[read]**

**Driven widening the baseline missed:** `renames.json` lands in a task area at the ordinary
`jigc doc rename <addr> --task <id>` door — an M51 Inc 11 surface, shipped in this binary:

```
rig=committed-singletons; jigc start --workflow record-decision "probe rename state"
jigc doc create adr --title "Alpha Note" --task probe-rename-state --slug alpha-note
  → .jigc/tasks/probe-rename-state/: base.json docs intent roles.json staged-snapshot.json workflow
jigc doc rename adr:alpha-note --to "Beta Note" --task probe-rename-state
  → .jigc/tasks/probe-rename-state/: ... renames.json ...          [driven]
```

So the honest set is **12 names (10 driven-or-read confirmed + `source` + `slug-override`)**, not
the baseline's 8. A guard cut on the baseline's list **blocks `task discard` / `task finalize` on
any task that used `doc rename --task`** — a real, ordinary door.

**What must be minted:** a `TASK_AREA_FILES` registry, spanning two crates (7 private `engine`
consts promoted to `pub`, 2 `engine::validate` consts promoted, 1 `cli` const). **The ⇔ fence**
that ties it to real code does **not** exist in the `VERB_KINDS`/`BEHALF_DOORS` totality mold and
cannot be minted in it — there is no clap tree to biject against. The only honest fence is a
**source-scan** over `task_dir.join(<literal>)` / `minted.dir.join(…)` / `self.dir.join(…)` call
sites in both crates, asserting every joined name is a registry member (the
`repo_relative_paths.rs:849` `the_unswept_remainder_is_counted_not_described` mold — a counted
scan, not a bijection). **State that as the fence's shape, and state its bound**, or the Settle
will reach for a bijection that has no second side.

### B2 · The config-layer CAS (M51 D4) does not fit 5 of the 8 populations fork 1 would extend it over

Read at `crates/cli/src/task.rs:4092-4241` (`ConfigLayerPreImage` / `ConfigLayerWorktree`) **[read]**:

| property of the shipped primitive | site | consequence for fork 1 |
|---|---|---|
| key is `spec: &'static str` from `CONFIG_LAYER_SPECS` | `task.rs:4094` | promote/retire/`rollback_rename` destinations are **runtime** paths; the key type has to change |
| home resolves through a 2-variant `ConfigLayerHome` enum | `task.rs:4143-4149` | a promote destination is neither `JigcRoot` nor `RepoRoot`-relative-to-a-static-spec |
| value is `Option<Vec<u8>>` — **a byte string** | `task.rs:4098-4101` | **cannot take a directory tree.** Populations 8/9 (`unwind_mint`, `unwind_unrecorded_seeds`) `remove_dir_all` a tree — the CAS is structurally inapplicable |
| park is `park_pre_image(jigc_root, spec, bytes)` → one file in `.jigc/displaced/` | `task.rs:4250-4262` | a tree has no single-file park; a per-doc park needs a runtime name |
| the conflict finding is `finalize.rollback-conflict` | `task.rs:4301-4312` | `rename`, `milestone create/add-task/add-from-spec/discard`, `task discard`, `doc author`, `config set` are **not finalize doors** — the stable `(code, target)` key would lie |

Gitignored/untracked paths are **not** a blocker: the park home `.jigc/displaced/` is itself
gitignored (`gitignore.rs:38` `ENTRIES`) and `restore` never consults git **[read]**, so the
baseline's *"trackedness changes severity, not existence"* holds at the primitive level.

**So there is no shared primitive today, and none of the three obvious extensions is free.** The
honest decomposition is **three** mechanisms, not one: (i) a runtime-path-keyed byte CAS for
populations 2/3/4/5/6 (promote · retire · `RecordPreImage` · `RecordFlipGuard` · `rollback_rename`)
with a **new** door-agnostic finding code; (ii) **no** CAS for 8/9 — the tree shape needs a
different answer (a pre-mint emptiness precondition, or a park-the-tree move); (iii) `config set`'s
relocation (`config.rs:942`/`:1053`) has **no rollback of any kind** — it is a *new* mechanism,
not an extension. Fork 1 as posed (*"one shared capture/CAS primitive every rollback path draws
from"* vs *per-site guards*) is a **false binary** against the measured shapes.

**Registry:** there is **no registry of rollback populations** —
`grep -rn "ROLLBACK_\|RollbackPopulation\|RESTORE_" crates/{cli,engine}/src` → **0 hits** **[driven]**.
One must be minted, and its ⇔ fence has the same no-second-side problem as B1: the closest honest
shape is a source-scan over the two restore idioms `grep -rnE 'fn [a-z_]*(rollback|restore|unwind)'`
(29 defs, 10 production) **plus** the two restores that are *not* functions (`Drop for
RecordFlipGuard` at `milestone.rs:4449`; the inline `fs::write` inside `rollback_promotions`) —
which a fn-name grep **cannot** reach (baseline-rollback §1 G2) **[baseline]**.

### B3 · `InProgress::markers()`'s shape cannot express two of the eight git states — and the "no shell-out" rationale that would block the probe is **already spent**

`crates/cli/src/repo.rs:166-176` **[read]**: `InProgress::markers() -> &'static [&'static str]`, and
`operations_in_progress` (`repo.rs:640-650`) is `markers().iter().any(|m| git_dir.join(m).exists())`
— a **presence-of-any** test over a static filename list.

- A **squash merge** writes `MERGE_MSG + SQUASH_MSG` and **no** `MERGE_HEAD` (baseline-posture
  §1.1 row 2) **[baseline]**. Reaching it needs `SQUASH_MSG ∧ ¬MERGE_HEAD` — a **conjunction with
  a negation**, which `&[&str] + any()` cannot express. A new predicate shape is required.
- A **conflicted index with no marker** (conflicted stash-pop; conflicted `--squash`) writes no
  file at all (§1.1 row 14) **[baseline]**. No marker-set widening reaches it. The probe has to be
  a **git query** — `git diff --name-only --diff-filter=U` or `git ls-files -u`.

**The foreclosure that is not one.** `repo.rs:1-16`'s no-shell-out rationale is about **`jigc_home`
resolution** (*"so a fake `.git` can never walk up to, and bind against, a real ancestor repo"*),
and `posture()` **already shells out**: `head_ref` runs `git symbolic-ref -q HEAD` with
`current_dir(repo_root)` (`repo.rs:617-635`), and `classify_leftover` runs `git rev-parse
--show-toplevel` (`milestone.rs:2639`) **[read]**. So an unmerged-index probe in `repo.rs` is
admissible; the rationale still holds for `worktree_git_dir`, which is explicitly the walk-up half
(`repo.rs:652-661`). **Cite this at the Settle** — the module header reads like a ban and is not one.

**Second constraint on the same fork, measured:** `git commit` under `CHERRY_PICK_HEAD` /
`REVERT_HEAD` / `MERGE_HEAD` **is** the conclusion of that operation to git; no argument makes it
not conclude (baseline-posture §3) **[baseline]**. And `--force` / `--carry-staged` / `--dry-run`
are **leaf-keyed** and carry no posture consent today, so **any** consent flag is a new one — which
is new *capability*, and M52's boundary says *no new capability*. The Settle must either take the
refuse-only arm or explicitly re-open the boundary; it cannot have both silently.

### B4 · `GATE_COVERAGE` admits a posture row, but the row alone does not make `task validate` see it

`crates/cli/src/gate_coverage.rs:99-114` **[read]**: `GateCoverage { id, door, tiers, fragment,
token }` — all `&'static str` / enums, so a **family** row is structurally admissible (a `posture`
id with token `"repository posture"` costs nothing). But the doc-comment at `:44-48` states
plainly: *"The **membership** of `Tier::Previewed` is owned elsewhere — the checks
`TaskArea::preview_gates` actually runs."* So a row without a probe is a **law-1 lie the fence
would bless**.

Capability answer: the probe is reachable — **`preview_gates` is CLI-side**
(`crates/cli/src/task.rs:1736`), not engine-side, so `crate::repo::posture(&repo_root)` can be
called there **[read]**. It is **not** reachable through `refuse_on_posture`:
`BEHALF_DOORS`' `["task","validate"]` row is `ActsOnBehalf::Neither` (`cli.rs:1980-1983`) and that
is **correct** — `task validate` commits nothing **[read]**. So the fix lands inside
`preview_gates`, and the row and the probe must land in **one** increment.

The fence that would catch a row-without-probe does not exist: `gate_coverage_fence.rs` checks
**tokens on surfaces**, not that a `Tier::Previewed` member is actually run. Worth minting, or
worth saying out loud that it is not.

### B5 · `parse_verb_addr` already holds the pack — so fork 5's placement guard is placeable — but `reject_malformed_slug_head` is the wrong thing to widen, and a guard inside `canonical_path` has ~25 call sites

The A1-D1 mechanism is exact: `crates/engine/src/store.rs:49-55` — `canonical_path` returns
`repo_root.join(&placement.file)` and **never reads `slug`** on the placement branch **[read]**.
That is why the boundary is `placement:` and not `singleton:` (baseline-tokens §3.4) **[baseline]**.

**The buried limitation a prior wave already found and deferred** —
`crates/engine/src/store.rs:241-248`, the shipped read-side guard's own comment:

> "A placement doctype is a singleton homed at its one literal `placement.file`, so
> `canonical_path` resolves it regardless of `slug`. … **This guard is read-scoped: the
> write/promote/reconcile callers use `canonical_path` directly and are unaffected.**"

A wave stated the hole in a comment and shipped it. The predicate is one line
(`schema.placement.is_some() && slug != schema.ty`, `store.rs:254`) — real reuse, **not** analogy.

**Where it can land, measured:**
- `crates/cli/src/doc.rs:6122` `parse_verb_addr(pack: &dyn PackSource, project_config: &Path, addr)`
  — the shared write-door parse boundary, and it **already holds the pack**, so the schema (and
  `placement`) is in hand with no new plumbing **[read]**. It calls `expand_bare_singleton` (which
  already resolves the pack) then `reject_malformed_slug_head`.
- `reject_malformed_slug_head` (`task.rs:1291`) is `(addr, slug) -> Result<()>` with **no schema**
  **[read]**. The placement guard is a **sibling** at the same boundary, never a widening of it.
- The other three boundaries each resolve the schema **separately or later**: `task bind`
  (`task.rs:2687`, schema at `:2695`), `milestone add-from-spec` (`milestone.rs:1410`),
  `rename` (`rename.rs:986`). Three extra sites, each needing its own schema lookup.
- Landing it **inside** `canonical_path` costs **~25 production call sites** across both crates
  (`task.rs`×2, `doc.rs`×3, `milestone.rs`×10, `engine/{index,state,store,validate,milestone,
  target_surface}.rs`×9) **[read]** — and would change the 1.0-pinned behaviour of every consumer.

**The M50 `Address::parse` measurement does not transfer and must not be cited as if it did.**
M50 kept `store.malformed-slug` out of `Address::parse` because `doc schema --format json`
advertises **type-level** addresses carrying a literal `<slug>` placeholder, which `doc_read_surface`'s
law-1 fence round-trips. A1-D1's token is a **well-formed** slug naming a non-existent singleton —
`doc schema` advertises no such thing, so the reddening mechanism is absent. **Different question,
same blast radius** (the charter says this; the *reason* it differs is the above).

### B6 · A `migrate-corpus` `from`-keyed walk is fully available at the seam — one field short

`crates/cli/src/migrate_corpus.rs:2075-2131` **[read]**:
- `candidate_docs(pack: &dyn PackSource, …)` **already has the pack** and already calls
  `crate::pack::load_prior_schema` (`:2103`). `prior_doctype_schemas` (`pack.rs:199-213`) is not
  needed — the loop is there.
- The whole defect is `.filter_map(|prior| prior.location)` at `:2104` (prior `placement` dropped)
  plus the `to.location` branch (`:2080-2088`) returning before any snapshot is loaded.
- `DoctypeMigration` (`:68-81`) carries **`docs_root` only, no `placement_root`** — so a prior
  `placement.file` with a leading component (`docs/roadmap.md`) cannot be re-rooted.

**Both missing pieces are in hand at the construction site.** `migrate_in_repo` (`:327-375`) holds
`resolved` (`:331`) and already derives `docs_root` via `crate::start::docs_root_prefix(&resolved)`
(`:340`); the placement twin `crate::start::placement_root(&resolved)` and the extracted
re-root primitive `crate::start::reroot_placement_file` (`start.rs:3414-3434`, already shared by
two callers) exist **[read]**. So the fix is: one field on `DoctypeMigration`, one `filter_map`
widened, one branch un-short-circuited. **No schema hash moves, no `schema-version` bumps, no
corpus migrates** (baseline-freeze §3) **[baseline]**.

Two constraints the Settle must carry: widening the walk widens the surface
`destination_collision_route` (`:1982`) exists for; and `dt.to.placement.file` **is** already
placement-root-resolved (via `all_schemas` → `apply_placement_root`, `start.rs:3631-3634`), so the
prior side must be re-rooted through the **same** primitive or the two halves will disagree **[read]**.

### B7 · Pre-dispatch: the funnel site exists, the format is in hand — and the class is 26 sites in 2 shapes, not one

`Cli::dispatch(self)` (`cli.rs:568`) holds `self.format` and **already runs a pre-dispatch guard**
there (`refuse_on_posture(&self.command, self.format)`, `cli.rs:571-573`) **[read]**. So a
`current_dir()` funnel has a proven seat.

**But the 24 `current_dir()` sites are not at the seat** — each `run_*` fn does its own
`std::env::current_dir()` with a **plain `eprintln!`** and no `format` consultation
(`cli.rs:735-741`, `:753-759`, and 22 siblings) **[read]**. Closing them means either 24 signatures
gain a `&Path`, or `dispatch` resolves once and every `run_*` takes it — a mechanical but
**24-site** refactor, and `refuse_on_posture` itself reads `current_dir().ok()?` (`cli.rs:540`),
which **silently skips the entire M51 posture family** on that fault **[read]** — a second-order
cell nobody has counted.

**And the second shape cannot ride that funnel at all.** `pack.rs:1786` / `:1793` are
`eprintln!("warning: {err:#}")` **inside `make_pack()`**, which runs *after* dispatch, per-verb,
up to 4× **[read]**. Driven on rc.15:

```
printf 'packs: [unclosed\n  : :\n' > .jigc/config/packs.yaml
jigc --format json doc show adr:nope           → exit 1
  stderr: warning: /private/var/folders/.../repo/.jigc/config/packs.yaml is not a valid pack-set list: …
          (×4) then { … findings envelope … }
  json.loads(stderr) → JSONDecodeError: Expecting value: line 1 column 1     [driven]
```

Two riders the baseline does not name: the warning **prints a host absolute path**
(law 1's `render::repo_relative` rule), and `crates/cli/src/pack.rs` sits in
`crates/cli/tests/repo_relative_paths.rs:746` `UNSWEPT_PRODUCERS` at **count 9** with the reason
*"pack-load has no repo-root subject to be relative to"* — **falsified** for these two sites, whose
subject is a file under the repo the walk just found **[read]**. A fix that touches them must move
that count (`the_unswept_remainder_is_counted_not_described`, `:849`, checks it against source).

**Registry:** no registry of pre-dispatch failure points exists. The honest fence for the
`current_dir()` half is a **source scan** (`grep -c 'current_dir()' crates/cli/src/cli.rs` == a
declared number) — the `UNSWEPT_PRODUCERS` mold again, not a bijection.

---

## MAJOR

### M1 · `leftover_probe_fail_closed.rs` is a near-miss citation — the acceptance derivation must not count it

Applying `implementation/pinning.md` §3 addendum (*a citation is verified by what a test asserts*):
the suite's axis is **(refusing door) × (leftover shape planted at the leaf path) × (consent)**
(module doc, `crates/cli/tests/leftover_probe_fail_closed.rs:1-45`) **[read]**, and it *asserts* the
doors refuse over an unreadable **leftover**. The baseline's L-1 cell is an unreadable **worktrees
root** (`chmod 000 .jigc/worktrees`), which the suite's `Shape` axis never plants.

The code confirms the gap is real and is a *stated-rule violation*:
`crates/cli/src/milestone.rs:2783-2789` — `leftover_at` maps **every** `symlink_metadata` error to
`LeftoverAt::Absent`, and `probe_leftover` (`:2811-2812`) returns `None` for `Absent` = *provably
safe to delete* — while the same function's doc-comment two lines above (`:2791-2794`) states:
*"`None` when `path` is **provably safe** to delete … **including** the case where the probe could
not read the path at all, which is a hold like any other"* **[read]**. A rule stated in the code and
violated in the function it annotates — razor legs 1 and 3, in one file.

**A suite whose name says `fail_closed` while the property fails at the root is exactly the
`unverified-reuse` trap.** Name it in the acceptance derivation, or the class gets classified
*test-fenced* on the strength of its filename.

### M2 · `render::setup_block`'s JSON arm is one line — and the two candidate targets are not equivalent

`crates/cli/src/render.rs:3383-3396`: `pub fn setup_block(format: Format, finding: &Finding) ->
String`, `Format::Json => json(finding)` — **one line**; call sites `cli.rs:738` and `:766`, both
passing `&Finding` from `Result<_, Finding>` doors whose signatures do not change **[read]**.

Driven on rc.15, the third shape is real and **richer than the declared reject arm**:

```
jigc --format json setup   (dirty install path)  → exit 1, stderr:
  { "severity":…, "probe":"setup", "check":"dirty-install-path", "code":"setup.dirty-install-path",
    "key": { "code":"setup.dirty-install-path", "target": null }, "message":…, "location":… }
jigc --format json doc show adr:nope  → { "error": "this project isn't set up — …" }   [driven]
```

**So the two fix targets differ in kind, and the fork as posed hides that.** Routing these doors to
the single-key `{"error": …}` arm **destroys the stable `(code, target)` key** M42 built and M51
D7 made true at 25 doors — a regression-in-disguise. Routing them to the `{findings,
schema_version}` arm preserves it and moves **no version** (`engine::result::SCHEMA_VERSION = 3`,
`result.rs:27`; the charter's `doc schema contract-version` premise is the wrong home —
baseline-contracts §3.2 **[baseline]**). Declaring a **third** arm preserves it too and breaks no
wire. The Settle's real question is *preserve the key or not*, and only two of the three arms do.

Cost of the wire change is real but bounded: 1 in-crate unit test pins the bare shape
(`render.rs:6085`) **[read]**, and 2 `ENVELOPE_ARMS` rows carry `path: &[]` today.

### M3 · `ENVELOPE_ARMS` admits the `doc show` widening, but needs 2 new `ArmShape` members

The four shipped `doc show` rows (`render.rs:5619, 5638, 5658, 5670`) each carry
`ArmOrigin::Dispatch("the address's DEPTH picks the projection … no result enum could model the
set")` — i.e. **hand-enumerated with a stated reason**, which is exactly what D5 permits **[read]**.
`ArmShape` has 4 members: `Object(&[&str])`, `ArrayOf(&[&str])`, `Scalar`, `DataKeyed`
(`render.rs:5094-5108`) **[read]**.

The driven space is **17 addressable cells / 7 root shapes**, and `ArmShape::ArrayOf`'s *"the
surface's one array"* is falsified twice (array-of-item-objects **and** array-of-strings on a `ref`
leaf), plus an object-valued single field leaf (`#meta/base` → `{sha, short}`) (baseline-contracts
§3.3) **[baseline]**. So the fix is **+3 rows and ≥1 new `ArmShape` member** (an array-of-scalars
shape), **not** row edits — contradicting the baseline's own *"no new `ArmShape`/`ArmOutcome`
member is needed"*, which was measured for **DEFECT A**, not for DEFECT B. Do not carry that
sentence across the two defects.

Rider, unsolved by any of this: `doc schema milestone-record` says `base: string` while
`doc show …#meta/base` returns an object (baseline-contracts LD-2) **[baseline]** — two 1.0-pinned
surfaces disagreeing about one field's type. No registry spans both; the only mechanism that could
is the `doc_read_surface.rs` parity assertion (pinning.md §2), which today asserts *address
round-trip*, not *type agreement*. Naming that is cheaper than discovering it at decompose.

### M4 · `DESTROYING_DOORS` admits `task finalize` / `task discard` structurally — but its consumers key on a worktree-shaped subject

`DestroyingDoor { verb, code: Option<&str> }` and `DESTROYING_DOORS: [&DestroyingDoor; 4]`
(`crates/cli/src/milestone.rs:2603-2630`) **[read]** — adding rows is trivial. The obstruction is
downstream:

- `flow49_acceptance.rs:999-1051` asserts **every** member has a cell in the refusal/narration
  matrix and that `0 < refusing < len` **[read]** — so two new rows red the acceptance until both
  matrices grow.
- `design/finalize.md:206` states the teardown door's subject is *"the worktrees git has registered
  here, which is narrower than theirs"* — the table's whole vocabulary (`LeftoverVerdict`,
  `LeftoverShape`, `classify_leftover`'s `git rev-parse --show-toplevel` **inside** the path) is
  **worktree**-shaped **[read]**. A task area is not a worktree; `classify_leftover` returns
  `Unverifiable` for it by construction.

So membership is not a one-line addition: either the table's subject generalizes from *worktree-
shaped path* to *destroyed path*, or the task-area class gets its own registry. **Decide it in
those words**, because the charter's fork-3 rider (*"`jigc task finalize` is in no destroying
registry at all, so membership is part of the question"*) reads as cheaper than it measures.

### M5 · The sinks are narrower than the guards, and no registry spans the two derivations

`remove_worktrees` (`milestone.rs:4581`) and `cleanup_subtask_areas` (`:4520`) take the
**registered set**; `probe_leftover`/`held_subtask_worktrees`/`fanout_worktree_paths`/
`workbench_paths` walk **on disk** (baseline-destroying §1.3) **[baseline]**. So a sink can destroy
less than the guard cleared, and — the sharper direction — a guard can clear a set the sink then
misses. There is **no registry of destroying *sinks***; a fix that only teaches the guards leaves
the asymmetry. Name the sink axis explicitly in the acceptance.

Driven-adjacent confirmation of the same shape one function over: `workbench_paths`
(`setup.rs:3057-3071`) excludes a child when `entry.file_name() == prefix` — a **name** match with
**no shape check** — while its own doc-comment three paragraphs up says *"**Every child that is not
a directory**, symlinks included (M49's lesson at `fanout_worktree_paths`): the shape of a path is
a reason to recurse into it, never a reason to drop it from the set"* **[read]**. Stated rule,
violated in the function it annotates — the L-2 root cause, pinned to a line.

### M6 · Harness capability: there is no git-in-progress-state fixture builder, and no genuine in-transaction racer

- `dev/jigc-rig --list-states` and `crates/cli/tests/support/trial_corpus.rs` build **adopted
  corpora**; neither has a git-posture state (a merge, a rebase-apply, a sequencer, a conflicted
  stash-pop). The posture auditor built all 14 by hand in throwaway repos (baseline-posture §1.1)
  **[baseline]**. M52's tier-0 posture arms need that builder, and it is **new test substrate**
  spanning ≥8 states × 2 backends — size it as its own increment, on the `trial_corpus.rs`
  precedent (pinning.md §4: *"a new load-bearing corpus state is added here, one place"*).
- **No rollback population has ever been raced by a genuine concurrent process** — every racer in
  the whole class is the user's own `pre-commit` hook (baseline-rollback §5.4) **[baseline]**. That
  matters for `unwind_mint`: it runs on `milestone create` / `add-task`'s **record-commit rejection
  path**, i.e. a hook *is* reachable there — but the third party the fix exists to protect is
  *another jigc agent writing into `.jigc/tasks/<id>/docs/` in a fan-out* (baseline-rollback §3.9),
  which a hook cannot simulate. The N-process binary sim (M51's precedent) is the only instrument;
  say so rather than letting the hook stand in for it.
- A **manufactured `packs.yaml` typo** needs no builder (driven above, 1 `printf`).
- A **deleted cwd** and `chmod 000` are macOS-driven only. `.github/workflows/` runs the gate on
  **ubuntu-latest** — `chmod 000` as root is a no-op on Linux CI, and a `chmod`-based fixture will
  silently pass. Any `chmod 000`-dependent arm needs a CI-visible skip with a stated reason, or it
  is a test that never executes (the rc.14 trial found two of those in its own instrument).

---

## MINOR

### N1 · `apply_placement_root` / `reroot_placement_file` is the shared primitive B6 needs, and it is already shared by two callers — verify a third does not drift

`start.rs:3414-3428` + `:3430-3434` **[read]**. `config.rs:1062-1063` is the second caller
(`placement_root` / `normalize_placement_root`). A `migrate_corpus` third caller is a
one-line reuse. Cheap; flagged only so the Settle does not mint a fourth spelling.

### N2 · Open lead 6's trigger fires if A6-2's fix lands as a task-state line

`render.rs:322-326` declares the composed JSON pinned at exactly `{task, text}` per
`design/command-output-contract.md` §1, and the charter's open lead 6 carries the trigger *"any
wave that moves a task-state line"* **[read]**. A6-2's fix (name `jigc milestone execute <id>` at
the door that composes it) lands either in the **pack step body** (rides `text`, trigger does not
fire) or as a **task-state/affordance line** (trigger fires, and the pre-pin window is the only
place to add the key). Decide which at the Settle rather than at decompose.

### N3 · `rename`'s address boundary and `SLUG_DOORS` are separate guards that both need the placement predicate

`rename.rs:986` calls `reject_malformed_slug_head`; the `--slug` half is `SLUG_DOORS`' sixth row
**[read]**. A1-D1 at `rename` is driven as a **committing** landing (baseline-tokens §4.1)
**[baseline]**, so the fix must reach *both* the address head and `--slug`, and `rename` is in
`COMMITTING_DOORS` — the registry M51's claim clause binds. Two sites, one predicate.

### N4 · `migrate-corpus`'s prior-home union comment is already the doc that will need striking

`migrate_corpus.rs:2072-2074` states *"a claim over **every** home the doctype has ever declared,
so the walk is the union of all of them"* — driven false for 3 of 4 (from × to) cells
(baseline-freeze §3) **[baseline]**. It sits beside `design/corpus-migration.md:68` and `:299`.
Three homes, one strike each, each with its falsifying datum.

### N5 · `format_json_success_axis.rs` / `text_json_parity_axis.rs` do not reach the reject class

Both suites' subject is the **success** arm (`pinning.md` §2's corrected note: *"the success half
is swept by its own suite over the clap-tree verb axis"*) **[read]**. DEFECT A/B/D are all
**reject**-class or **projection**-shape cells, so neither suite fences them; `machine_output.rs`
asserts *stdout empty / stderr the sole channel*, which LD-1 violates by **prepending**, not by
using the wrong stream — a cell its predicate may pass. Read the predicate before classifying
either as coverage.

---

## The three cross-cutting hunts

### (a) cheap-vs-robust — `fork · cheap-vs-robust`

1. **Fork 1 (rollback).** Cheap = per-site guards. Robust = one shared primitive. **Both are
   wrong as posed** (B2): the measured shapes force **three** mechanisms. The *one-way-door* leg
   fires on the **finding identity**, not the code: whichever code the non-finalize populations
   emit becomes a 1.0-pinned `(code, target)` key at the stable-key seam. Picking
   `finalize.rollback-conflict` for `jigc rename` would pin a lie; picking a per-door code pins
   six. **A door-agnostic `write.rollback-conflict` is the minimal-correct one**, and it is cheap
   only while the pre-pin window is open.
2. **Fork 3 (destroying subject).** Cheap = widen the suffix to *any file*. Measured, that fires
   on **100 % of tasks at 100 % of doors** (baseline-destroying §3.7) — a *regression-in-disguise*
   that trains `--force` into reflex, which is precisely why M46 refused the `--ignored` arm.
   Robust = the complement of a **registry** (B1). There is no third option; the cheap arm is not
   cheap, it is broken.
3. **Fork 5 (`<slug>` head).** Cheap = a per-door check at the 5 write doors the row names. The
   driven door set is **6 write + 1 read + `rename --slug`** (baseline-tokens §3.1) **[baseline]**,
   and the read door (`milestone add-from-spec`) is the one the row's framing says cannot exist.
   Robust = the predicate at `parse_verb_addr` + the three sibling boundaries (B5). Cheap here is
   a *hole in a declared surface*: `doc schema --format json` (contract-v6) projects **no**
   `singleton` / `placement` / `location` over all 16 doctypes, so a driver has **no machine-readable
   way to learn the rule** (baseline-tokens §3.7) **[baseline]** — the agent lands in the
   well-formed cell by construction.
4. **Fork 6 (third envelope).** Cheap = `{"error": …}`. That **drops the `(code, target)` key** —
   a regression-in-disguise against M42 + M51 D7 (M2). The minimal-correct arm is the one that
   keeps the key.
5. **B7's funnel.** Cheap = fix the one `current_dir()` site the review drove. Robust = the funnel
   at `dispatch` + a counted scan. The cheap arm leaves `refuse_on_posture`'s `current_dir().ok()?`
   silently skipping the **whole M51 posture family** (`cli.rs:540`) — the wave's own tier-0 work
   disabled by the wave's own un-swept sibling.

### (b) foreclosed-by-doc — `fork · foreclosed-by-doc`

- **`repo.rs:1-16`'s no-shell-out rationale reads as a ban on the unmerged-index probe and is
  not one.** Its scope is the `jigc_home` walk-up (*"so a fake `.git` can never walk up to, and
  bind against, a real ancestor repo"*); `head_ref` (`repo.rs:617`) and `classify_leftover`
  (`milestone.rs:2639`) already shell to git inside the same module family **[read]**. The
  rationale **still holds** for `worktree_git_dir` — this is a candidate **revise-the-scope**, not
  a conflict. (B3)
- **`design/finalize.md:195`'s directory-tree exclusion** (*"a worktree pre-image over a directory
  is a different shape — absent-means-delete over N files, including files this run never saw"*)
  is cited to exclude `.jigc/config` from the CAS set. Driven, the **identical** shape applies to
  populations 8/9 (`unwind_mint`, `unwind_unrecorded_seeds`) — so the doc's predicate **still
  holds** and *correctly* forecloses the CAS there. **Not a revise.** But `finalize.md:200`'s
  *"Before phase 6, all-or-nothing — this sentence is the promise"* is scoped to the **finalize
  transaction**, and **5 of the 9 populations sit outside it** (`rename`, `doc author`,
  `config set`, and the two `unwind_*`). **No doc states a rollback discipline for a non-finalize
  door** — that is a doc *gap*, not a foreclosure, and it is where the fix's rule has to be
  written. Route it to the docs dimension.
- **`render.rs:322-326` / `command-output-contract.md` §1** pin composed JSON at `{task, text}` —
  declared, and open lead 6's trigger rides it (N2). The declaration's rationale (*presentation
  lines add no key to the contract*) still holds; the fork is only whether M52 moves a line.
- **M50's `Address::parse` measurement** (guard kept out of the engine parser because
  `doc_read_surface`'s law-1 fence reddens over advertised type-level addresses) **does not
  transfer** to A1-D1's well-formed token (B5). Citing it as if it did would foreclose the right
  landing site for the wrong reason.
- **`repo_relative_paths.rs:746`'s `pack.rs` row reason** (*"pack-load has no repo-root subject to
  be relative to"*) is **falsified** for `pack.rs:1786/:1793`, whose subject is a repo file
  (B7) **[driven]**. Basis has changed; the row needs its datum, not a silent rewrite.
- **`storage.md:300-314` Concurrent writers** does **not** foreclose fork 1 — it is scoped to the
  three shared `.jigc/` caches and *supports* the robust arm by establishing that another jigc
  agent is a legitimate third party **[read]**.

### (c) prior-art-reconciled — one `blocking · prior-art-contradiction`, three clean

- **CONTRADICTION (already settled one way, framed the other in the charter).** The corpus walk:
  `design/corpus-migration.md:68` and `:299` (the acceptance row) state the walk keys on **`from`**;
  `crates/cli/src/migrate_corpus.rs:2072-2074`'s doc-comment states the union is over **every home
  ever declared**; `design/storage.md:208`'s M49 census row states only that a placement branch
  **exists** — and is **not** falsified **[read]** + **[baseline]**. Three artifacts, two of which
  the code contradicts and one of which it does not. Charter fork 4 poses it as a *choice* (*"does
  the corpus walk key on `from` or on `to`?"*); **two locked docs already settled it as `from`**,
  and the code is the violator. Pose it as a repair, not a fork — and B6 shows the repair costs no
  schema-hash movement, which removes the only reason the charter had to treat it as open.
- **CLEAN — the destroying-door subject.** `design/finalize.md:206` states the teardown's subject
  is the registered set and says so *is narrower than theirs*; `milestone.rs`'s
  `DESTROYING_DOORS`/`probe_leftover` doc-comments agree; `design/project-setup.md:160` records
  the M50 fix at the same door. No doc disagrees; the **code** disagrees with itself at
  `leftover_at` (M1) and `workbench_paths` (M5).
- **CLEAN — the posture family.** `design/finalize.md:31` (*"No in-progress merge/rebase/bisect"*),
  `repo.rs`'s module header, and M51 D2 all say the same three members. Nothing claims cherry-pick,
  revert or squash-merge membership; the gap is a **silence**, not a contradiction.
- **CLEAN — the rollback discipline.** `design/finalize.md:187-195` is the only home and it is
  internally consistent after M51's `:200` reconciliation. The gap is that **no doc covers the five
  non-finalize populations** (see hunt b).
