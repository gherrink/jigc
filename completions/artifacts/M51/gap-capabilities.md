<!-- 2026-09-10 · Opus gap-detector · dimension: capabilities · HEAD bd348a83 · target/release/jigc 1.0.0-rc.14 · no cargo, no repo edits · copied verbatim -->

# M51 gap probe — dimension: CAPABILITIES (engine/CLI surface)

Verified at HEAD `bd348a83` against the release binary
`/Users/maurice/projects/gherrink-jigc/target/release/jigc` (`jigc 1.0.0-rc.14`). Spikes drove
that binary on `dev/jigc-rig` corpora (two-step eval, roots under the session scratchpad);
no cargo run, no repo file edited. Every reuse claim below is marked **verified-reuse** (I
exercised the exact new shape) or **unverified-reuse** (analogy only → a gap).

---

## G1 — `blocking · unverified-reuse` — EC-1's sink guard has no engine-side predicate, and `untrackable_reason` structurally cannot be it

**The claim to test** (charter F1 robust arm, baseline1 §4): "re-validate the recorded
`source-path` at the destructive sink (`task.rs::retire`)", with `trackable::untrackable_reason`
as the shared predicate.

**Half of it is verified-reuse. The other half is impossible as stated.**

*The door leg — verified-reuse, driven.* `crate::trackable::untrackable_reason`
(`crates/cli/src/trackable.rs:55`) takes a caller-typed `&str` and answers all four EC-1 shapes.
I exercised it on its one live caller that takes a **raw caller string**, `core.hooksPath` →
`setup::committable_hook_path` (`crates/cli/src/setup.rs:1499`), which is the same shape
`migrate`'s `<PATH>` is:

| `core.hooksPath` | binary's answer |
|---|---|
| absolute outside the repo | *"git cannot track this path"* — refused for the commit |
| `../sibling-hooks` | *"git cannot track this path"* — refused |
| in-repo symlink → outside | *"git cannot track this path"* — refused (the `resolve` canonicalize at `trackable.rs:159` follows the link) |
| `.git/hooks` | *"git cannot track this path"* — refused (component leg, `trackable.rs:78`) |
| `myhooks` (legitimate) | committed, no warning |

So one predicate discriminates all four cells and does not over-refuse. `migrate.rs`
(`crates/cli/src/migrate.rs:254` `repo_root.join(path)`) simply never calls it.

*The sink leg — unverified-reuse, refuted.* `plan_retirements` is
`crates/engine/src/finalize.rs:400`, in the **engine**. Driven greps at HEAD:
`grep -c "Command::new" crates/engine/src/` → **0**; `crates/engine/Cargo.toml` has no git or
process dependency. `untrackable_reason`'s legs 3/4/5 (`--git-dir`/`--git-common-dir`, ownership,
index gitlink) are all `git_output(...)` shell-outs. **The engine cannot host this predicate
without breaking its no-subprocess property.** And `grep -c "symlink_metadata|read_link|is_symlink"
crates/engine/src/` → **0**: *no engine-side symlink check exists at all*. The only engine-side
symlink-adjacent syscall is `canonicalize` in `engine::path::repo_relative`
(`crates/engine/src/path.rs:48`), whose own module doc says it "reads no filesystem content".

**What actually closes the sink, and what it costs.** Two shapes, and the plan must pick one:

- **(a) A lexical engine predicate** at `plan_retirements`: refuse a retirement that is absolute,
  carries any `..` component, or carries any `.git` component (case-insensitively).
  `engine::store::lexical_normalize` (`crates/engine/src/store.rs:66`) already supplies the
  normalizer; the three tests are pure `Path::components()` work. **It cannot answer the symlink
  cell** — that needs `symlink_metadata`, a capability the engine has never had.
- **(b) The CLI validates `plan.retirements` before `retire` runs.** `FinalizePlan.retirements`
  is `pub Vec<PathBuf>` (`crates/engine/src/finalize.rs:131`) and is in scope in the CLI, so
  `untrackable_reason` can be asked there over the full five legs including the symlink cell.
  Cost: the refusal is a CLI-side finding, so the engine's planner still hands back a plan it
  cannot vouch for.

**The census this fix must sweep is five raw consumers, not two.** `state::read_source_path` is
read raw at `crates/cli/src/task.rs:1296`, `:1426`, `:1794`, `:2491` **and**
`crates/engine/src/finalize.rs:401`. Two of those (`:1426` `task validate`, `:1794`
`task finalize`) pass the token to `decide_carryover` as `retire_exempt`
(`crates/engine/src/finalize.rs:738`), so a tampered `source-path` also **buys a carryover-gate
exemption for an arbitrary path** — the baseline's amendment, and a second exit-0 consequence of
the same untrusted byte. A guard placed only at `plan_retirements` leaves four sites reading raw.

**Adjacent, same predicate, and it *is* verified-reuse:** `jigc config insert-step … <file>` /
`replace-step <target> <file>` (baseline1 §2d) read an arbitrary host path at exit 0 and copy it
into a committable in-repo step file that then **composes into `jigc start`'s step text**. Asked
of the *source*, `untrackable_reason` answers exactly the question that door needs ("is this
inside the repository git can record?"), and it is CLI-side already (`config.rs`). No new
capability; one call.

---

## G2 — `blocking` — EC-2: there is no "before work starts" shared seam, and one `symbolic-ref` probe answers **one of four** posture cells

### G2a — the seam does not exist (`blocking`)

The charter asks for "one probe at the shared seam". Driven source read: **there is no shared
pre-work function.** `discover_repo_root` — the walk-up every committing door runs first — is
**copy-pasted into six modules**: `crates/cli/src/locate.rs:165`, `crates/cli/src/repo.rs:41`,
`crates/cli/src/milestone.rs:5411`, `crates/cli/src/task.rs:4911`,
`crates/cli/src/start.rs:3826`, `crates/cli/src/ingest.rs:632`. `locate::locate` is *not*
universal either — the six milestone doors call their own `discover_repo_root` directly
(`milestone.rs:469`, `:2061`, `:3014`, `:3512`, `:3622`, `:3728`).

The one function 9 of 10 `COMMITTING_DOORS` share is `task::git_commit_capture`
(`crates/cli/src/task.rs:4255`), and the repo already has the right *precedent* for a probe
there: `task::nothing_staged` (`crates/cli/src/task.rs:4290`) is a pre-flight predicate placed
"beside the commit seam the axis shares so a door inherits it rather than re-deriving it", with
`hook_output_axis.rs` fencing the producer axis. So the shape is proven — but at that seam the
probe fires **after** the door has done its work, which is right for `task finalize` (the working
area is destroyed after the commit) and wrong for a door that wants to refuse before it writes.

**`jigc setup` is outside the seam entirely** — `crates/cli/src/setup.rs:1709` runs
`git commit --no-verify … -- <pathspec>` through `git_output`, a documented exclusion at
`task.rs:4253`. Any seam-placed probe misses setup by construction, and setup is one of the two
doors EC-2 names as losing bytes (silent un-install on the next checkout).

### G2b — the probe answers one cell; three of the four need different predicates (`blocking`)

Driven, on throwaway git repos:

| posture | `git symbolic-ref -q HEAD` | `git rev-parse --verify HEAD` | detected by the charter's probe? |
|---|---|---|---|
| detached HEAD | **rc=1** | rc=0 | **yes** |
| unborn HEAD (`git init`, `--orphan`) | rc=0, prints `refs/heads/main` | **rc=128** | **no — false green** |
| `GIT_DIR=<other repo>/.git` | rc=0, prints `refs/heads/main` | rc=0 | **no — false green** |
| linked worktree on a branch | rc=0, prints `refs/heads/feat` | rc=0 | **no — and the defect there is not HEAD posture** |

- **Unborn HEAD** needs `git rev-parse --verify HEAD` (or `--quiet`), a *second* predicate.
  Baseline2 §2c shows the consequence: `milestone create` on an orphan branch writes git's
  **empty-tree hash** `4b825dc…` into the *committed* record as `base:`, `provision` then fails
  (`object … is a tree, not a commit`) and `milestone finalize` dies on a bare `anyhow`.
- **`GIT_DIR` redirect** is detectable in principle — `rev-parse --git-dir` returned B's git dir
  while `--show-toplevel` returned A — **but the same asymmetry is true of a legitimate linked
  worktree** (toplevel `wtA`, common-dir `A/.git`), so `dirname(common-dir) != toplevel` is not a
  discriminator. The discriminator that does work — *`repo_root/.git` exists on disk and is
  neither the resolved git-dir nor a `gitdir:` pointer to it* — **collides with a stated design
  constraint**: `crates/cli/src/repo.rs:1-16` deliberately refuses to shell out to git when
  `.git` is a *directory*, so that "a fake `.git` can never walk up to, and bind against, a real
  ancestor repo" (~15 unit fixtures depend on it). Closing the GIT_DIR cell means re-opening that
  layering. **Flag it as a fork the Settle must take, not as a probe.**
- **Linked worktree**: HEAD is on a branch and the probe is silent, but baseline2 §2b shows
  `milestone create`/`add-task` writing the record into the **main checkout** and committing onto
  **`main`** while the operator stands in the worktree on `feat`. That is a `repo_root` vs
  `jigc_home` mix-up (`crates/cli/src/repo.rs:26` `jigc_home`), a **different capability** from
  HEAD posture. EC-2's axis as written does not reach it.
- **No repository-operation-in-progress probe exists at all**:
  `grep -rn "MERGE_HEAD|rebase-merge|CHERRY_PICK" crates/` → **0 hits**. That is the capability
  behind the baseline amendment *"`--carry-staged` concludes a merge"* (baseline2 §2e: exit 0,
  HEAD gains two parents, MERGE_HEAD consumed, the ack says `1 file committed` and never mentions
  a merge). The carryover gate catches it only *incidentally*, via `finalize.carried-staged` on
  the merge's resolution — and `--carry-staged` is precisely the flag that turns that off.

**Capability statement:** the honest subject is **repository posture**, a predicate family of
four (`HEAD detached` · `HEAD unborn` · `operation in progress` · `git-dir redirect`), not one
`symbolic-ref`. Only one member exists as a one-liner today; a second is a one-liner
(`rev-parse --verify HEAD`); a third is a one-liner (`rev-parse -q --verify MERGE_HEAD` +
`.git/rebase-merge` / `CHERRY_PICK_HEAD`); the fourth is blocked by `repo.rs`'s stated layering.

### G2c — the exemption: `DedicatedWorktree` cannot be inferred at the seam (`blocking`)

`DedicatedWorktree` (`crates/cli/src/task.rs:4573`) is a typed value, but the boundary passes
only `wt.path()` into `git_commit` (`task.rs:4496`, `:4225`), so `git_commit_capture` receives a
bare `&Path` and **cannot tell a dedicated worktree from the live checkout without a path-shape
guess** (`.jigc/worktrees/.combine-<pid>-<nanos>`). A shape guess here is the exact class M50's
audit condemned (`fanout_worktree_paths`' `path.is_dir()` — a claim about *shape* where the
question is about *bytes*). **The declared exemption has to be a typed argument at the seam**, so
every call site states its posture and a new one cannot compile without deciding. Sibling to
enumerate: under `finalize.fan-out.squash: false`, `chain_commit` (`task.rs:4477`) reaches the
seam **N+1 times** through one `DedicatedWorktree` — a per-call exemption, not a per-boundary one.

### G2d — the landing act **can** carry the probe (verified-reuse)

`overlay_docs_commit_and_ff` (`crates/cli/src/task.rs:4407`) is one function, called with the
**live checkout** `repo_root` (`task.rs:4381`: `overlay_docs_commit_and_ff(repo_root, &code_tree,
&head, …)` where `repo_root` is the operator's checkout), and it is where `git merge --ff-only`
runs. Both boundary arms funnel through it. **Yes — the ff can carry the same probe, and it is
the correct home for the boundary's cell**: the dedicated-worktree commit is exempt by design,
the *landing* is not.

---

## G3 — `blocking · fork · cheap-vs-robust` — EC-3/EC-5/EC-8/EC-13 all need one primitive that does not exist: an enumeration of the **pinned** envelopes

**The recipe table can be the fence. It cannot be the declaration — and using it as one is a
one-way door at the pin.**

*What exists.* `crates/cli/tests/format_json_success_axis.rs:389` `struct Recipe { path, base,
drive }`, 47 rows, bijected against the clap leaf tree at `:884`
(`the_success_recipes_are_a_bijection_with_the_clap_leaf_verbs`). It drives every leaf verb to a
**genuine success through the real binary**, so it sees renderer-added keys, array envelopes and
arm splits that a rendered witness cannot. Adding `keys: &'static [&'static str]` and one
`assert_eq!(envelope.keys(), keys)` is mechanically the smallest complete fence — **verified: I
read the driver and the bijection; it already produces the exact object the assertion needs.**

*Two structural costs the charter does not price.*

1. **The bijection is `one recipe per leaf verb`** — `:918`: *"a duplicated path would hide an
   unswept verb"*. `task finalize --dry-run` vs landed, and `milestone finalize` landed vs
   blocked, are **arm** splits. Making the table verb×arm requires reshaping that assertion
   (distinct-paths bijection + an `arm` label that must be unique per path), or the duplicate-path
   guard silently weakens. Small, but it is the load-bearing half of that test.
2. **A `keys:` column pins what ships, not what is declared.** Baseline3 §1 counted **17 further
   envelope keys no design document names** beyond EC-3's four — `uninstalled`, `rows`, `summary`,
   `identity`, `new_path`, `old_path`, `prose_mentions`, `referrers`, `moved`, `displaced`,
   `reslugged`, `layer`, `rejected`, `knobs`, `anchor`, `side`, `step`, `overlay` — and the root
   cause is that **`command-output-contract.md` names three pinned surfaces while ~14 envelopes
   ride in none of them**. A `keys:` table written from the driven output blesses all 17 on the
   day 1.0.0 lands, and `command-output-contract.md:446`'s own rule
   (*"an undeclared key on a pinned envelope is a defect, not an addition"*) then makes each of
   them a versioned-extension problem instead of a delete.

**Cheap:** add `keys:` and declare what is there. **Robust:** mint the missing primitive first — a
code-side `PINNED_ENVELOPES` (or a `Tier::Pinned` on `text_json_parity_axis::REGISTRY`, which
already bijects the verb tree but classifies *fenceability*, not *pinned-ness*) that says, per
verb×arm, **pinned / unpinned-by-declaration**, so `keys:` fences the pinned set and the unpinned
set is a decision on the record. **Long-run cost of the cheap cut:** every key we would have
deleted becomes a v2 contract to delete it. This is a one-way door at the pin by the charter's
own leg-0 test.

**The same missing primitive is the reason EC-5, EC-8 and EC-13 cannot be closed cheaply:**
- **EC-5** (`schema_version` on an unstated subset — driven at 7 of 48 arms, baseline3 §1) wants a
  *partition* over the pinned set. No pinned set, no partition.
- **EC-8** (no post-1.0 evolution rule per surface, charter fork 6) — the fence shape exists
  (`gate_coverage_fence.rs`'s named-token guard over 8 enumerating sites;
  `cli::pack::CONSTRAINT_REQUIRED_TOKENS` at `crates/cli/src/pack.rs:1242`), but a
  *"each pinned surface names its evolution rule"* fence needs the same enumeration as its
  subject. **This is the answer to question 10: the home is a fence of `gate_coverage_fence.rs`'s
  shape, over a registry that has to be minted first.**
- **EC-13** (a hand-enumerated witness) reuses `doctype_map_versions.rs` — but only once the shape
  it witnesses is a code-side value rather than an ad-hoc `json!` (see G8b).

**Additive-window status — verified, and it is open.** `design/doc-read-surface.md:88-92` and
`design/command-output-contract.md:446` both state the close is keyed to **the 1.0 pin, not a wave
name**, with the required form: *every addition is declared here, in its own paragraph, as it
ships*. So every additive key this wave wants is permitted **now** and costs a versioned extension
the day after. That is the pricing instrument for G4, G5 and G9 below.

---

## G4 — `blocking` — EC-1's review hold: naming the retire target is free, and the data is already in scope (verified-reuse)

`render::migration_review` (`crates/cli/src/render.rs:4176`) emits
`json!({"review","task","source","rewrites"})` at `:4183-4192` — an **ad-hoc `json!`**, and
`grep` at HEAD finds **no test and no golden asserting its key set** (`render.rs:7484…7633` drive
the `Format::Agent` arm; `invocation_log.rs:462` asserts the hold's log identity;
`flow44_acceptance.rs:601-615` asserts the *fidelity diff text*). So:

- **The text arm is free** — no contract, no fence.
- **The JSON arm takes one additive key** (`retires`), inside the open window (G3).
- **The data needs no new computation.** `plan.retirements: Vec<PathBuf>`
  (`crates/engine/src/finalize.rs:131`) is in scope at the render site: `task.rs:1869-1885` builds
  `rewrites` from `plan.promotions` three lines above the `migration_review(...)` call.
  `FinalizePlan` is explicitly *"never serialized to disk or the wire"*
  (`crates/engine/src/finalize.rs:135`), so surfacing `retirements` here is a render decision, not
  a contract move.

**verified-reuse.** The one honest caveat: the hold is currently in **none** of
`command-output-contract.md`'s three declared surfaces, so declaring `retires` on it also settles
whether that envelope is pinned — i.e. it lands on G3's missing primitive.

---

## G5 — `high` — EC-26: `decide_carryover` is reusable in-memory; `MINT_DOORS` / `write_staged_snapshot` are **not** (unverified-reuse)

**The reusable half — verified.** `engine::finalize::decide_carryover`
(`crates/engine/src/finalize.rs:735`) takes `snapshot: Option<&StagedSnapshot>` and
`current: &StagedSnapshot` — **plain in-memory values**, not a task dir. `setup` can build both
in-process with `task::git_staged_snapshot(repo_root)` (`crates/cli/src/task.rs:3925`) and call it
directly. No persistence needed, because `setup` mints and commits in **one invocation**.

**The unreusable half — refuted.** `MINT_DOORS` (`crates/engine/src/state.rs:808`) is fenced as
*"`mint_task` ∪ `mint_milestone` must equal the `site` set"* — its subject is a **working-area
mint**, and `write_staged_snapshot` (`state.rs:864`) writes `staged-snapshot.json` **into that
area**. `jigc setup` mints no working area and has no dir to write into. **Answer to question 4:
no — `MINT_DOORS`' snapshot cannot be taken by a door that mints no task, and reusing that
registry is an unverified-reuse claim.** The registry that has to widen is `COMMITTING_DOORS`
(the charter says so; the note *"this is the same registry as EC-2"* is right), not `MINT_DOORS`.

**Two capabilities the fix genuinely needs:**

1. **A third `CarryoverBoundary` variant.** `crates/engine/src/finalize.rs:678` is a 3-variant
   enum with `is_task_boundary()` at `:697`; the finding text and the owner-artifact exemption key
   on it. A `Setup` variant is compiler-forced everywhere the enum is matched — good — but the
   finding code (`finalize.carried-staged`) and its route are task-shaped and would lie at setup.
2. **A pathspec-scoped snapshot, which no primitive supplies.** `decide_carryover` compares whole
   `StagedSnapshot`s; `setup`'s commit is pathspec-limited (`setup.rs:1709`) and baseline2 §4
   drove that an unrelated staged `feature.txt` is correctly untouched. Applying the gate
   unscoped would block `setup` on every unrelated staged file. Either `git_staged_snapshot` gains
   a pathspec argument, or the caller filters. Small, but it is a real new capability and it is the
   difference between the fix landing and `setup` becoming unusable in a dirty tree.

---

## G6 — `high` — EC-17 `LeftoverShape`: three derivations, not two; the collapse is feasible with one widening (verified-reuse, qualified)

Read at HEAD:

- `probe_leftover` (`crates/cli/src/milestone.rs:2645`) — `symlink_metadata(path)` then
  `!meta.is_dir()` ⇒ `LeftoverShape::File`. **Correct for a symlink-to-directory.**
- `doomed_at` (`crates/cli/src/milestone.rs:5091`) — `path.exists()` (follows) then `!path.is_dir()`
  (follows) ⇒ a symlink-to-dir takes the *directory* branch and `child_names` enumerates **through**
  the link.
- `PendingLoss::narrate_taken` — filters lines by `symlink_metadata(&line.at).is_err()`, so after
  the link is removed every enumerated child reads as *taken*. **A third derivation**, and it is
  what turns the wrong enumeration into the "these bytes are not recoverable" lie.

**Answer to question 5 (collapse `doomed_at` onto `probe_leftover`): yes, and it needs exactly one
widening.** `probe_leftover` already returns `LeftoverHold { verdict, shape, entries }` — the
shape/verdict half of what a narration needs. But `doomed_at`'s `OwnWorktree` branch uses
`discarded_work(path)` (per-entry `path` + `state.label()`) while `probe_leftover`'s uses
`dirty_worktrees` (names only). So the collapse is *one probe returning the richer per-entry form,
with the refusal rendering a subset* — not a drop-in swap. That is the property `hold_line`'s own
doc-comment already claims for the three refusing doors, made structural.

**The two subject derivations (question 5b) are real and the registry hides them.**
`DESTROYING_DOORS` (`crates/cli/src/milestone.rs:2514`, 4 members) crosses **two** subject
derivations: an on-disk walk (`provision`, `uninstall`) vs `git worktree list`'s registered set
(`remove_worktrees`, `crates/cli/src/milestone.rs:4337`, shared by `discard` and `finalize`, which
`continue` past an unregistered path). **A symlink can never be a registered worktree**, so two of
the four cells are structurally unreachable — a fix scoped to the 4-member registry will iterate a
cell that cannot exist and call the class swept. The axis is
`{shape} × {on-disk walk, registered set}`, and the registry must say which derivation each door
runs or the fence is vacuous for half its rows.

---

## G7 — `high` — EC-15: the join's per-sub-task staged set exists at the derivation site and is thrown away (verified-reuse)

**Answer to question 6: yes, and the input is already read.** `engine::milestone::join` loads
`crate::state::ProvenanceRecord::load(&sub_dir)` per sub-task at
`crates/engine/src/milestone.rs:1947` and builds `staged` groups carrying `source_task`
(`:2002`). The clash arm `continue`s at `:2094-2096` **before** anything is `claim`ed into the
overlay. `JoinOutcome` (`crates/engine/src/milestone.rs:1701`) carries only
`{overlay, findings}`, so `render::doc_less_sub_tasks` (`crates/cli/src/render.rs:3844`) has
nothing but `outcome.overlay.values().map(|d| d.source_task)` to derive from — which is why the
text and the wire agree with each other and both disagree with the disk.

**Capability needed:** one field on `JoinOutcome` (`staged_by_task: BTreeMap<String, BTreeSet<String>>`
or similar), populated from the provenance load that already runs. The wire key `no_docs_from` is
**declared** and its *shape* does not move — only its values become true. So this is a value
correction, not a contract spend. The one design call: whether `overlay` stays `{}` on a block
(today's behaviour) or gains the blocked group — the fix should not silently change `overlay`
while fixing `no_docs_from`.

---

## G8 — `high` — the truth-fence menu: two of EC-11's three cases have **no code-side set to generate from**

### G8a — the seam that works, and where it stops

`engine::slug::mint_statement` (`crates/engine/src/slug.rs:139`) interpolated into
`doc::create_long_about` (`crates/cli/src/doc.rs:169`) and byte-asserted by
`help_truth.rs::doc_create_help_states_the_slug_caps` is the proven mold. **Answer to question 9:
yes, a `long_about()` can be generated the same way — for the one case that has a registry.**

| EC-11 case | the set the sentence names | exists at HEAD? |
|---|---|---|
| `migrate-corpus --help` *"v0→v1 transform"* | `engine::schema_diff::SchemaChangeKind::ALL` (`crates/engine/src/schema_diff.rs:757`, 18 members) | **yes — verified-reuse** |
| `validate --help` *"code anchors"* | the **five store probe families** | **NO.** The families exist only as numbered comments inside `validate_store` (`crates/engine/src/validate.rs:477`, `:496`, `:516`, `:580`, `:607`) and as prose in `design/validation.md`. `grep -n "FAMILIES"` over `probe.rs`/`validate.rs` → 0. `STORE_EXIT_FLIPS` (`crates/cli/src/render.rs:909`) is the wrong set — it enumerates *exit flips*, not families. **A registry must be minted first.** |
| `doc show --help` four-key shape | the pinned key list | **NO.** The whole-doc serve is an ad-hoc `json!` at `crates/cli/src/doc.rs:5070-5073`. A `const` would have to be minted *and made load-bearing at the render site*, or it is a second home for one fact. The whole-output goldens (`doc_show.rs:314-383`) would catch a divergence, which makes the mint safe. |

`long_about` seams available to carry a generated statement:
`crates/cli/src/doc.rs:73/100/153/169/204` and `crates/cli/src/milestone.rs:67`. Everything else
is 981 `///` doc-comments with no registry (baseline4 §2).

### G8b — the count fences (question 8), each mapped to the mechanism it reuses

| item | mechanism to reuse | verdict |
|---|---|---|
| `ManifestKind::ALL` (absent — EC-22) | `SetKind::ALL` / `SchemaChangeKind::ALL` / `ConfigAckArm::ALL` (`crates/cli/src/render.rs:2589`) — a `const ALL` + a compiler-checked exhaustive `match`. `ManifestKind` is `pub` in `pub mod render` (`crates/cli/src/lib.rs:41`), so a test can read it. | **verified-reuse** — mint the const, then a `foldback_truth`-shaped arm over the guide |
| `migrate-corpus` triage keys (EC-23) | `CorpusMigrationReport` (`crates/cli/src/migrate_corpus.rs:120`) is a `pub struct` with named fields ⇒ the `text_json_parity_axis` exhaustive-destructure idiom enumerates them with the compiler | **verified-reuse with a caveat**: field names ≠ wire keys (`no_commit`/`unlanded` are `#[serde(skip)]`), so the fence needs a stated per-field disposition — exactly what `text_json_parity_axis`'s `Disposition` already does |
| `COMMITTING_DOORS` count in prose (EC-12/EC-14) | `foldback_truth.rs`'s per-prose-unit arm over a **derived** guide set (`setup.rs`'s `include_str!` sites), plus `doctype_map_versions.rs`'s registry→rows shape; `invocation_log::COMMITTING_DOORS` is `pub` | **verified-reuse**, with one required capability the plan must name: **historical** counts (both manifest headers' *"all 16 doctype hashes"*, `decisions-pending.md:320`) must be *dated-bracketed*, not re-pinned — `foldback_truth.rs::dated_correction_spans` is the shipped discipline |
| `validate --help` / `doc show --help` | see G8a | **needs a new registry first** |

---

## G9 — `medium` — EC-20: `task finalize --dry-run` can gain `findings` additively (verified-reuse)

`render::finalize_manifest` (`crates/cli/src/render.rs:1887`) emits
`json!({"dry_run","subject","manifest","left_out"})` at `:1894-1899` — ad-hoc, fenced
presence-only (`finalize_dry_run_subject_close`). The findings are **already computed above the
branch**: `crates/cli/src/task.rs:1827`'s `if dry_run` arm sits below the full `report` (the
changelog advisory merged in at `:1677`) and returns the manifest alone. So **yes — an additive
key, inside the open window (G3), with the value already in scope.** The behavioural fence EC-20
wants (drive each `Tier::Previewed` member and assert the emitted `findings[].code` set is equal
across `task validate` / `--dry-run` / landed) **depends on** that key existing, so the order is
key-then-fence. `GATE_COVERAGE` (`crates/cli/src/gate_coverage.rs:43-49`) says in its own words
that it fences what surfaces *say*, never what a door *emits* — so the registry exists and the
behavioural arm over it does not.

---

## G10 — `medium` — EC-18 `gitignore::ensure`: no line-merge primitive exists, and the amend has four callers

`crates/cli/src/gitignore.rs:32` — on any missing `ENTRIES` line it does
`std::fs::write(&path, ENTRIES)`, a whole-file replace, against its own doc-comment at `:28`
(*"amended once to the union"*). **Answer to question 7: there is no line-merge primitive.** The
repo's three amend-or-refuse precedents are all *other* shapes: the `SKILL.md` blake3 body stamp
(`setup.rs:63` `GUIDE_HASH_KEY`, `:243` `GUIDE_MODIFIED_CODE` — refuse-once-edited), the
`settings.json` key merge, and the `pre-commit` verbatim wrap. A line union is ~6 lines and needs
no primitive — but three properties must hold and none is free:

1. **Idempotent across four production callers** — `adapter.rs:1108` (setup), `task.rs:2778`
   (finalize), `milestone.rs:472` (create), `milestone.rs:2066` (provision). An append-missing
   implementation must produce byte-identical output on a second call or finalize starts
   committing a churning file.
2. **Ordering.** Today the file is exactly `ENTRIES` in order; a union appends. Any golden or
   fixture asserting the exact bytes must be re-derived.
3. **The `SKILL.md` question applied here** — amend, or refuse-and-route? `.gitignore` is a file a
   user legitimately co-owns, so *amend* is right; but the ack currently names no content change
   at all, which is the law-1 half of EC-18 and is independent of the merge.

---

## G11 — `medium` — N20: the frame is built and discarded at exactly one branch (verified-reuse), but the `survived:` clause is the real work

`task::surface_commit_rejection` (`crates/cli/src/task.rs:3533`) downcasts `CommitRejected`; **every
other `anyhow` falls to `invocation_log::operational_failure(format, err)` and "carries no
identity"** — while a fully-populated `RejectionFrame` (`task.rs:3502`) is already in the caller's
hand at all seven construction sites (`cli.rs:796`, `migrate_corpus.rs:303`, `milestone.rs:408`,
`:4094`, `:4166`, `task.rs:554`, `:2063`). Routing the else-branch through `frame.code` is a
two-line change: **verified-reuse.**

**The honest bound the plan must carry (an M31 sibling census):** `RejectionFrame.survived` is
written for the *commit-rejected* case — "nothing was committed, the merged docs were rolled back,
and every provisioned sub-task worktree still holds its staged code". A **non-hook** failure can
occur earlier or later in the same door, where that clause may be false. So the axis is
`COMMITTING_DOORS × {hook rejection, non-hook failure} × {is the survived clause true?}` — ten
doors × two, and a naive reuse of one clause for both cells ships a law-1 lie at the exact moment
a user is recovering. Baseline2 §2c already has the shape: `milestone finalize` on an unborn HEAD
dies with a bare `anyhow` whose truth about state nobody has written down.

---

## G12 — `medium` — `jigc ingest`'s identity leg: the predicate exists, the route does not (question 12)

`engine::validate::unadopted_cause` asks the identity leg at
`crates/engine/src/validate.rs:909-918` — *"jigc's own writer names every doc it wrote
`<slug>.md` … the **path** is [the evidence], and the path says no"* — via
`crate::slug::is_slug(slug)`. **`engine::ingest::adopt` never asks it**: it derives the slug from
the filename stem at `crates/engine/src/ingest.rs:226-229` and adopts. That is the whole reason
`ingest` bypasses the discriminator — the leg lives in the *classifier*, not in the *gate*, and
the two run in different modules over the identical derivation.

**Capability needed:** one `is_slug` call at the ingest gate (engine-side, no new capability) plus
a finding code and **a route with no verb behind it**. `jigc doc rename` requires the doc be
managed; the file in question is by definition not. The only honest route is a `Human` route
naming `git mv` — for which M45's owner-artifact route (a `Human` route naming `git add`) is the
precedent. Driven consequence today (baseline1 §5): `ingest` says *adopted*, `doc list` says
*unregistered*, `doc show` says `store.malformed-slug`, and `validate` exits 1 routed at
**`jigc ingest`** — a route that, followed exactly, changes nothing. That loop is a
`route-floor` defect on top of the registration one and is not what the ledger entry at
`decisions-pending.md:593` describes.

---

## G13 — `medium` — question 11: the `resume:` line — **refuted by drive**

The charter's fork asks whether the composed footer can name `jigc workflow <id> --task <id>`
when the sub-task's docs are unprovisioned. **Driven on a real milestone sub-task (rig
`committed-singletons`, `milestone create` + `add-task`, no `provision`):**

```
$ jigc start --task tighten-the-cache        → rc=1
$ jigc workflow single-task --task tighten-the-cache → rc=1
```

**Byte-identical refusal from both**: *"task `tighten-the-cache` is pinned to base a2728c0 but
you're on … a sub-task's work happens in its own worktree at `.jigc/worktrees/tighten-the-cache`
… run `jigc milestone provision <m>` … then re-run this from that worktree."* The guard is
upstream of both doors (a base-pin check, not a docs-area check), so **naming `workflow` in the
footer buys nothing** — the reuse claim is refuted. The task dir at that point holds
`base.json / intent / workflow` and no `docs/`. The refusal is honest and routed; the residual
gap is only that `render.rs:480`'s `resume:` line and `:119`'s `Run:` line promise a resume the
state cannot serve without a prior `provision`, i.e. a **law-1 wording** item, not a capability.
(`--format json` on that refusal is the flattened `{"error": …}` envelope — the known deferred
shape at `decisions-pending.md:605`, exit 1.)

---

## G14 — `medium` — EC-4/EC-16: the relocation is not a value of the acked enum, and a test pins the false phrase

`config::route_docs_root_repoint_orphans` (`crates/cli/src/config.rs:751`, `placement-root`
sibling at `:846`) returns `()` and narrates via two `eprintln!`s (`:781`, `:800-804`);
`run_set` (`:317`) calls it for effect and returns `ConfigAck::Set { key, value }`
(`render.rs:2541`). `config_ack_parity` destructures that enum **exhaustively** — the strongest
form of the parity rule — over exactly `{key, value}`, so it is structurally blind. **Capability:
`route_docs_root_repoint_orphans` returns `Vec<Relocation{from,to}>`; `ConfigAck::Set` gains a
`relocated` field; the exhaustive destructure then forces the key.** One unresolved design call
the mechanism does not settle: the door also stages an *index* change, so the honest key set may be
`{relocated, staged}`.

EC-16's over-claim has **five homes, one of them a test that pins the lie**:
`config.rs:782`, `config.rs:66` (the `--help` doc-comment), `config.rs:878`, `render.rs:3351`
(`unmanage`'s restatement) and `crates/cli/tests/unmanage.rs:291-293`, which **asserts the phrase
is present**. The set the code walks is `stranded_under_docs_root`, filtered to `location:`
doctypes — the placement branch `CLAUDE.md` names as the cross-cutting gotcha. So the fix is a
five-site sweep including a red-then-green on a passing test.

---

## G15 — `low/medium` — the remaining capability cells, each with its predicate status

| item | capability | at HEAD |
|---|---|---|
| **EC-27** whitespace `docs-root` | a *"reads back as itself"* predicate on the root value | `config::unusable_root_reason` (`crates/cli/src/config.rs:677`) asks absolute/symlink/file-shaped; the control-char rule is elsewhere. **Neither asks about whitespace.** New predicate, trivial; the existing `ROOT_KNOBS` loop (`root_knob_rules.rs:368/:470`) is the axis |
| **EC-28** `SLUG_DOORS` × OS name ceiling | a filesystem-length refusal with a code and a route at each door | **absent.** Driven by baseline1: `doc rename` bare `os error 63`; `start --slug` and `doc create --slug` answer `task.working-area-io` with a route blaming *"a disk or permissions problem"* — **a false route**, and EC-28's claim that `doc create --slug` "refuses correctly" is refuted. `SLUG_DOORS` (`cli.rs:1978`, 6 rows) is the registry; the ceiling is a filesystem fact the slug grammar does not encode, so it needs its own predicate |
| **N15** `--task` miss says *committed* | a `--task`-aware `store.not-found` arm | **the correct shape is three inches away**: the sibling `store.not-staged` names its copy and routes with the right door. One resolver arm, no new capability |
| **EC-25** echoed vs parsed token | the refusal re-prints the *typed* token | the `at:` field already carries the typed form; only the message renders the normalized one. ≥3 doors (`doc retitle-item`, `doc add-item`, `doc set-slot`), not 1 |
| **N23/EC-38** retired doctype orphans its corpus | detect+route on **deregistration** (the M38/M39 relocation floor applied to a doctype leaving the resolved set) | `orphan.rs` was generalized at M39 for *relocation*; the deregistration axis is new. Note the reachability bound: only a pack that **replaces** the dev pack (`JIGC_PACK_DIR`) or a project pack dropping a doctype it owns reproduces it |
| **`task discard` silent commit ack** | the ack names the sha every other committing door prints | the sha is produced by the same `git_commit_pathspec` chain (`milestone.rs:820` → `git_commit_capture`); the door discards it. Same shape as N20 — a value computed and dropped |

---

## The three hunts

### `fork · cheap-vs-robust` — H1: `keys:` on the recipe table (see G3)

**Cheap:** a `keys:` column derived from what the binary emits today. **Robust:** mint the
pinned-envelope enumeration first, so `keys:` fences a *declared* set. **Tell (1) — one-way door
at the pin:** the cheap cut converts 17 undeclared keys from "delete them" into "v2 to delete
them", by the contract's own rule at `command-output-contract.md:446`. **Tell (2):** the wave's
vision-purpose here *is* a complete/declared surface (the charter's claim: *"the pin closes over
declared keys"*), and the cheap cut leaves a known hole. Do not bless it by omission.

### `fork · cheap-vs-robust` — H2: EC-1 at the door only

**Cheap** (charter F1 cheap arm): guard `migrate`'s `path`. **Robust:** door + sink + the four
other raw `read_source_path` consumers. The cheap cut is **not** a one-way door at the pin — but
baseline1's tamper repro drives byte loss at exit 0 through a door whose *argument was benign*, so
the cheap cut ships a known data-loss cell. Leg 0 of the razor ("a hole in a declared surface")
admits the robust arm on the evidence, not on preference.

### `fork · cheap-vs-robust` — H3: EC-2 as "one probe"

**Cheap:** `git symbolic-ref -q HEAD` at `git_commit_capture`. Driven, that answers **1 of the 4
posture cells** (G2b) and misses `setup` entirely (G2a). Shipping it and calling the exemption
class closed would put a *false* completeness claim on the record — the exact failure the wave
exists to correct. **Robust:** a posture predicate family, with the GIT_DIR member declared out
with its `repo.rs` blocker quoted rather than silently omitted.

### `fork · foreclosed-by-doc` — H4: `repo.rs`'s deliberate no-shell-out layering forecloses the GIT_DIR cell

`crates/cli/src/repo.rs:1-16` states the rationale verbatim: it shells to git **only** when `.git`
is a *file*, "so a fake `.git` can never walk up to, and bind against, a real ancestor repo" — the
~15 fake-`.git` unit fixtures depend on it. Detecting a `GIT_DIR` redirect requires asking git in
exactly the case that layering refuses to. **The rationale still holds** (test isolation is real),
but its *scope* is reopenable: the fixtures could be discriminated by a marker rather than by
"never ask git when `.git` is a directory". Flagging for the human, not overturning: the plan
should either close the cell by re-scoping that rationale, or declare the GIT_DIR cell out **with
this quote** rather than leaving it unmentioned.

### `fork · foreclosed-by-doc` — H5: `command-output-contract.md`'s three-surface framing

*"The three surfaces this pins"* — composed output, write-acks, the findings envelope. Fourteen
shipped envelopes are in none of them (baseline3 §1), including `setup` and `upgrade`, whose keys
EC-3 calls defects. So `guide_file` is "a defect" only because `hook_file` beside it happened to
get a paragraph, while `identity` on `unmanage` is unremarked. **The doc's framing, not the
binary, is what makes EC-3 a four-key finding instead of a twenty-one-key one.** The better path
(enumerate the pinned set) honours the doc's stated rationale — *an undeclared key on a pinned
envelope is a defect* — and is a candidate **revise**, not a conflict. Additionally, driven at
HEAD: `jigc migrate` and `jigc milestone execute` are a **fourth** producer of the composed
`{task, text}` shape against that section's *"Three verbs"* (baseline3 F-new-1), and neither
carries a closed-key assertion.

### `blocking · prior-art-contradiction` — H6: `design/finalize.md` contradicts itself on the transaction (EC-29 / fork 3)

Its config-layer rollback row says *"worktree untouched"* **on purpose** while the section header
two rows above reads *"Before phase 6, all-or-nothing"*. Two statements in **one locked doc**
about the same act, and the plan cannot pick freely: the row is the one that was *decided* (it
names the behaviour and its reason), the header is the one that generalized. The rollback matrix's
un-swept dimension is the **worktree**, over
`{promotions, retirements, owner-artifact, config-layer, milestone-record} × {worktree, index}` —
the four index families are covered, the worktree is not. Note the capability asymmetry: index
rollback rides captured pre-images (M45/M46 discipline, four aligned families); a **worktree**
rollback for the config layer has no capture at all today, so "robust" here is a new capture
family, not a reuse.

### `advisory · prior-art-reconciled` — H7: what a `grep` across `design/` agrees and disagrees on

- **`COMMITTING_DOORS` = 10** — the code registry (`crates/cli/src/invocation_log.rs:130`) and
  `crates/cli/tests/flow47_acceptance.rs:18` agree; `MIGRATING.md:39`, `QUICKSTART.md:180`,
  `CLAUDE.md:7`, `design/worked-examples.md:3005`, `flow47_acceptance.rs:21` (four lines from
  `:18`) and `decisions-pending.md:320` all say nine. Not a contradiction to adjudicate — a
  stale count with one authority. **`jigc setup` is a committing door that is not a
  `COMMITTING_DOORS` member**, which is a genuine definitional disagreement the plan must settle
  before EC-2 and EC-26 both widen "the registry".
- **`design/validation.md` declares five store families** (`:324`, `:354`, `:426`, `:435`) and
  `crates/engine/src/validate.rs` implements five (`:477`, `:496`, `:516`, `:580`, `:607`) —
  they agree; `jigc validate --help` names one. One authority, one liar.
- **`design/measurement.md`** calls the invocation log *"an independently-versioned surface"* and
  `invocation_log.rs:378-380` repeats it, while no version integer exists
  (`grep "log_version|LOG_VERSION|record_version"` → 0) and `decisions-pending.md:455` defers
  `task_id` *on that very premise*. The deferral's ground and EC-6's finding are the same claim,
  read opposite ways. **A fork, not a free choice.**

---

## Bounds of this probe

- I did **not** run cargo; every fence-quality grade is from reading the test body plus driving
  the binary. No mutation was applied to any fence.
- The `core.hooksPath` spike exercises `untrackable_reason` through one caller. I did not build a
  harness to call the function directly, so the mapping from "the hook is not committable" to
  "the predicate returned `Some`" is inferred from `setup.rs:1499` + the message text.
- The migration review-hold JSON key set is **baseline1's** drive plus my source read of
  `render.rs:4183-4192`; my own attempt to reach the hold in a fresh rig stopped at
  `finalize.migration-no-replacement` (a singleton copy-in the payload did not satisfy), so I did
  not re-drive the exit-4 envelope.
- `LeftoverShape` was reasoned from source over the symlink-to-dir cell only; the other four
  shapes were not driven here (baseline4 §10 carries the same bound).
- I did not drive `milestone finalize` on a detached HEAD to confirm the ff-merge cell; the
  `overlay_docs_commit_and_ff` conclusion is a source read of `task.rs:4381`/`:4407`.
