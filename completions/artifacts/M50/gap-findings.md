# M50 — consolidated gap findings

Four `gap-detector` subagents (decisions · docs · doctypes · capabilities), each driving
`target/release/jigc` (`1.0.0-rc.13`) at `HEAD = cdce9fc`. Deduped and severity-ranked.
Baseline: [baseline-ledger.md](baseline-ledger.md).

**Provenance.** Orchestrator-driven personally: W-13, `task discard "../.."` (repository
destruction), `rename --slug "../../src/pwned"` (docs-root escape). Everything else is **relayed
from a detector that drove it**, with the command and output in its report.

---

## A · Tier 0's subject is wrong in the charter — three families, not one

The charter scopes T0 to *the empty-id class over `TaskArea::resolve`*. Driven, the real subject is
**every caller-supplied token that becomes a path component**, in three unrelated families:

| family | doors | driven worst case |
|---|---|---|
| **resolve** (`--task`/`milestone_id`) | 25, across **5** seams | `task discard "../.."` destroys the repository incl. `.git`, exit 0 |
| **mint/identity** (`--slug` at `jigc rename`) | 1 | `rename --slug "../../src/pwned"` lands a managed doc outside the docs root, drops it from `doc list`, `validate` says clean, exit 0 |
| **knobs** (`docs-root`, `placement-root`) | 2 | either knob set to `.jigc` → `uninstall` destroys committed managed docs at exit 0, narrating neither |

A guard scoped to the resolve seams closes **one of three**.

### A1 · The predicate already exists and the record says it belongs at resolve
`engine::slug::is_slug` (`slug.rs:172-186`) is named in its own doc-comment as **the recognition
predicate**, whose sites are *"an authored anchor, a ref body, **a frozen id read back from disk**"*
— a `--task <id>` is exactly that. So the ledger's *"applied at three mint boundaries, zero resolve
seams"* framing under-states it: `slugify` is the mint rule; `is_slug` is the **recognition** rule,
and the resolve seams are its paradigm sites. `design/structural-grammar.md:98` already asserts the
discipline for work-units.

### A2 · Guarding breaks nothing shipped (the `reuse-exercised` gate, exercised)
Driven: every shipped mint shape satisfies `is_slug` — `start` (incl. `ÄÖÜ Straße` → `aou-strasse`),
`migrate` incl. M44's path-hash form, `milestone create` (`../../canary` → `canary`), `add-task`,
the collision suffix. No `--task` literal in `packs/`, `crates/cli/pack/`, the 624 goldens or
`crates/cli/tests/` is non-`is_slug`. The guard **already ships one seam away**: the record-driven
reseed refuses `{#../../../canary}` with *"must be a slug `[a-z0-9-]`"* and a route.
**Residual, to be proven not inferred:** a grammar guard above the seams collapses the four
different `--task ""` texts to one — a behaviour change at 25 doors that `no_such_task_route.rs`
must be re-run against.

### A3 · The destroying axis at the resolve seam is two sites, and the milestone seam is safe by accident
`task.rs:3431` — a second `remove_dir_all` inside `post_commit`, fed by the same unvalidated
resolve, whose failure is logged as *"self-heals"* and never raised. `milestone.rs` holds five more
`remove_dir_all` sites; `milestone discard ".."` is stopped **only** because `tasks.json` is absent
at the traversal target — an incidental guard, code-less and route-less. Any refactor tolerating an
empty task list re-opens repository destruction at a second door.

---

## B · The adapter's deny floor guarantees what N1 falsifies

`crates/cli/adapters/claude-code.yaml:10` permits `Bash(jigc:*)` — every invocation, every flag,
unprompted. The floor's first deny member is `Bash(rm -rf:*)`.
`design/assistant-adapter.md:89`: *"**No self-collision.** The floor's … `rm -rf` denies target the
*agent's* Bash tool; `jigc` itself shells `git add`/`git commit`/`mv` as a binary it invokes
directly … **The floor constrains the agent, not the CLI.**"* That paragraph reasons only about
jigc's own git shell-outs and never contemplates jigc performing an **agent-supplied** deletion.
With N1 the allowlisted binary is an unprompted `rm -rf` under another name.

**Consequence for fork 1:** `--force` is not a consent model for an actor that already holds the
permit. Three arms, not two: refuse at admission with no flag past it (the id is *malformed*, not a
risky-but-legal request) · `--force` plus a narrowed permit shape · `--force` plus a written
admission that the self-collision paragraph is false. Silence picks the third.

**And `task discard` is not silent** — it narrates dropped edits **after** the destruction, in the
past tense. `team-ready-state.md:94` admits two arms (refuse · name the bytes **before**) and says
*"Silence is the one arm no door may take"*, so fork 1's valid-id half has **three** candidate
answers. Note that rule's stated scope is `.jigc/worktrees/`, **not** `.jigc/tasks/` — citing it for
`task discard` is citing outside its scope; the leg-1 argument for the valid-id half rests instead
on `project-setup.md:150` (`uninstall` refuses over bytes *"in no object DB at all"* and routes the
reader to a door that deletes them at exit 0).

---

## C · Fired triggers, due at this Settle, cited nowhere

1. **`decisions-pending.md:510`** — *"An empty `--task ""` is admitted once a live task exists, and
   the ack's route is unrunnable"*, recorded **2026-08-31**, four days before the trial. Trigger:
   *"the task-door admission axis is next opened."* Tier 0 **is** that wave. Its *"deliberately
   prose, not a test"* clause retires when M50 converts it.
2. **Orientation states 3/4** — deferred in a **doc-comment** (`render.rs:94-96`) on the premise
   *"no task store yet"*, false since M1, with **no ledger entry at all**.
3. **The Nth-demand counter** (`decisions-pending.md:14`) — a field-demand deferral takes a dated
   line *"in the same motion as each new datum"*, and **the third forces it onto the Settle
   agenda**. `ideas/state-aware-compose.md` records 2026-07-12 and 2026-07-15; RC-m50 is the third
   and was never appended. `VISION.md` → Open questions is stale by **four trials**.
4. **M45 retracted a guard as dead code** on the recorded belief that *"`TaskArea::resolve` bails
   before `run_discard`'s guard ever runs"* — the exact false belief that left five seams open.

---

## D · Locked docs that specify the defect, or forbid the fix

| doc | what it says | status |
|---|---|---|
| `design/finalize.md:29` | *"**Task exists.** `.jigc/tasks/<id>/` must exist; otherwise reject"* | the design of record **is** the defective predicate — `..`, `/etc`, `""` all exist |
| `design/write-commands.md:182` | same *nonexistent*-id predicate, justified because *"the in-progress catalog lives in bare `jigc start` orientation output"* | driven false — F-5 is the report of this sentence |
| `design/bootstrap.md:29,59` | *"the **four** orientation states"*, with state 3's shape spelled out incl. `staged:` and `Run: jigc task discard` | **two** renderers exist; F-5 and fork 2 are an undelivered 2026-05 design, not new features |
| `design/validation.md:77` | *"it is now true at every door"* **and** the single-hop declared bound | universal driven **false** at `remove-item`/`retitle-item`; the bound is live and W-15 contradicts it |
| `design/introspection.md:56` | *"no version governs"* `describe`'s json | a drifted **restatement** — `doc-read-surface.md:167` is the declared home and `:175` assigns it the result contract's `schema_version`. **Fork 6 shrinks to: does an `origin_pack` key move the integer.** |
| `design/design-altitude-doctypes.md:43-47` | the trio *"sit outside the frozen-v1 gate … a later shape change needs no corpus migration"* | **false since M40**; a planner prices fork 5's `research` side as free |
| `design/methodology-docs.md:40` | shipping without `AddedNestedRepeatable` *"would freeze that block at its birth shape"* | **false as written** — *with* the arm it is still frozen (§E) |
| `design/corpus-migration.md` | the loci census is *"two loci"* | the format permits **three**; this is the doc a builder enumerates from |
| `implementation/pinning.md:15` | `lib.rs` exposes *neither* the pack registry nor the clap tree, *"a prerequisite"* | **shipped at M45**; both are `pub`. `CLAUDE.md` routes planners here and T0's fence is a clap-tree enumeration — uncorrected it buys a phantom increment |
| `crates/cli/src/trackable.rs:19-23` | a gitignored destination **must stay permitted** — `relocate`'s squatter displacement depends on it | forecloses fork 3's cheap predicate; a **third** predicate (*this is jigc's own workbench*) is needed, and `untrackable` has **0 hits in all of `design/`** |
| `QUICKSTART.md:170-172`, `MIGRATING.md:40` | `task discard` *"removes **only** the working area"*, *"**touches nothing else**"* | false at HEAD — and `setup.rs:69-70` `include_str!`s both into the shipped `SKILL.md`, so an edit moves `jigc-body-blake3`, engages M48's refuse-to-clobber path and moves goldens. **A product surface change, unpriced.** |
| `milestone.rs:3737-3757` | its arm catches *"EVERY way the boundary refuses"* | `task.rs:3146` frames only a `CommitRejected` downcast |
| `config.rs:350-363` | *"a per-doc skip that still lands the knob leaves the store pointing at a home no doc is at"* | the same function then does exactly that (N5) |

---

## E · Fork 4 is ~10 missing cells at a third locus, not one kind

`schema_diff.rs:746-753` compares a nested `Leaf::Repeatable` **wholesale** and pushes
`Unclassified` on any inequality — **no recursion**. Driven, one edit at a time against real
`changelog.yaml`: nested add-optional-field · add-required-field · add-slot · widen-enum ·
remove-leaf · drop-block → **all six `[Unclassified]`**; the same edits one level up classify
correctly. `MAX_NESTING_DEPTH = 2` ⇒ exactly **three loci**; locus 3 carries **zero** of the ~10
kinds locus 2 has.

The most plausible real evolution is already a mutual dead end, driven end-to-end on a committed
`CHANGELOG.md`: adding a category to the shared `change-group` fragment (`include`d at both the
staging repeatable and the nested one) yields `[EnumWidened, Unclassified]` — the classified sibling
does not save it.

**The charter narrowed its own ledger entry**: `decisions-pending.md:176` says *"`AddedNestedRepeatable`
**+ a nested arm for the existing transforms**"*; the charter dropped the second clause.
**Cost correction:** the ~0.6 s / 3 commands figure is the **adopter's** run cost and is right; the
**build** cost is ~10 kinds at a third locus, or a locus-path refactor through both exhaustive
consumer matches. **Registry shape:** `SchemaChange::ALL` will not work (every variant carries
data) — the working shape is a discriminant enum + exhaustive `From<&SchemaChange>`, crossed with
the derived locus count 3.

**Argue it on the razor's second tell, not the first.** The kinds are internal, so building them
later is not format-irreversible. What the pin changes is **who is bricked**: today the only nested
block is ours; after 1.0 an adopter evolving a nested project-pack doctype sits at `validate` exit 1
with a route into `crates/engine/src/transform.rs` until we cut a release.

---

## F · Fork 5's prerequisite, and its foreclosing rationale

**N13 is a prerequisite, not a rider.** Driven with `research` absent from the loaded set:
pack-load, `doc schema`, `set-field` and `validate` all exit 0; `task validate` blocks with the
route *"…, **create the target in this task**, or drop the field"* — and `doc create research`
answers `create.unknown-doctype`. **An unfollowable arm on a blocking route, inside M43's route
floor.** The fence's subject is three sites (pack-load; `index.rs:525-527`'s silent `continue`;
`doc schema`'s projection). The trigger is live in the shipped topology: `JIGC_PACK_DIR`
**supersedes** the compose-methodology marker.

**The foreclosing rationale is direction-symmetric.** `design/methodology-docs.md:42` refuses the
mirror edge (`research → adr`) because it *"would point at a doctype **outside the composed schema
universe**"*. `design-altitude-doctypes.md:41-43` homed the trio in methodology *"off the recorded
unexercised cross-pack surface"*. Both become false statements if fork 5 lands, and owe a revision
in the same wave. Also: `vision.grounded-in` already declares `inverse: grounds` on `research`; a
second forward edge with the same inverse name is **accepted at pack-load at exit 0** — the inverse
namespace has no uniqueness fence.

**Declared unexercised:** fork 5 was **not** driven in the real shipped topology (marker set, both
packs, `adr` bumped with a manifest re-pin) — `JIGC_PACK_DIR` supersedes the marker, so the rig
cannot produce a modified dev pack *and* a composed methodology pack. Honest name: `unverified-reuse`.

---

## G · Fork 8's fit question, unasked

The primitive is built; it may not **fit**. Driven, two sub-tasks both staging `src/pad.ts`:
`milestone join` prints a success-shaped line, then blocks with *"the combine disjoint-applies code
and never text-merges a shared file — route: have the contending sub-tasks touch distinct files, or
combine their overlapping changes **by hand**"*. **Every recent completion audit fixed ≥2 findings
inside one module** (M49: `doc.rs`, `milestone.rs`; M46: `state::persist`'s five writers). A fix
round over such a set is refused at the join and degrades to the serial rule `fix-gate.yaml` already
prescribes.

**N12 is worse than the ledger said — it is wrong on the pinned JSON, not only the text.** Driven:
`committed.hash` owns 2 of its own 4 `manifest` entries, `files: 4` is false about that sha, and the
per-fix shas — *the entire product of `squash:false`* — appear nowhere; `sub_tasks[]` has no `hash`.
After 1.0, adding `sub_tasks[].hash` is additive but **correcting `files`/`manifest` is a break**.

Fork 8 needs **no** doctype change (`completion-record.findings` is top-level ⇒ a per-finding leaf
is `AddedItemField`, a byte no-op) — so it does not qualify under the human's criterion and must not
ride in on it.

---

## H · New defects beyond the baseline's N1–N20

| # | defect |
|---|---|
| N21 | `jigc rename --slug <traversal>` escapes the docs root at exit 0; `doc list` omits the doc, `doc show` serves it, `validate` says clean (orchestrator-driven) |
| N22 | `docs-root .jigc` reaches the identical `uninstall` loss as `placement-root` — fork 3's axis is **both** root knobs |
| N23 | **retiring or removing a doctype silently unmanages a committed, tracked corpus at exit 0** — `doc list` drops it, `validate` exits 0, `migrate-corpus` reports `0 blocked`, the file is still in `git ls-files`. The **staged** arm (`doc.rs:4073-4076`) explicitly contemplates *"a type the resolved cascade no longer defines"*; the committed arm does not |
| N24 | `post_commit`'s `remove_dir_all` (`task.rs:3431`) is a second destroying consumer of the unvalidated resolve, silent by design |
| N25 | absolute host paths leak into finding loci and error text at four driven doors, against M45's repo-real-path rule |
| N26 | a manifest-less project pack (PB-1, shipped M49) stamps nothing, so `schema-version-current` can never fire and a shape change silently makes the corpus non-conformant with **no** `migrate-corpus` route — the pre-M42 false-green class, reopened for the population PB-1 created |
| N27 | `task diff`'s **empty form** prints no `# code changes` header and no frame — no task id, workflow, intent, roles or provenance. Fork 2's cheap arm cannot be *"point at `task diff`"* until its cold-start form answers *what is here* |
| N28 | the inverse-relation namespace has no uniqueness fence (two forward edges may declare the same `inverse:` at exit 0) |

## I · Corrections to my own baseline ledger

1. **Fork 6 is not undecidable** — `doc-read-surface.md:167/175` is the declared home and settles it.
2. **Fork 3's cheap predicate is foreclosed by design**, and the test I called confused is defending
   a real rule (`trackable.rs:19-23`).
3. **`task discard` is not silent** — it narrates after the fact, which is a third arm, not zero.
4. **The `is_slug` framing was mint-vs-resolve; the code says recognition-vs-mint** — the razor is
   stronger than I wrote it.
5. **The missing-snapshot path is well-routed** (driven: a real code + followable route), and a
   **two-version skip migrates in one hop** — both listed as undriven in the ledger.
6. `DOCTYPE_ARG_IDS` is `cli.rs:1531`, not `:1530`.
7. **T1-a's *"panics at exit 101"* is fixture-dependent**, like `42 of 50` — the panic fires only
   when a run produces a finding whose route builds `["jigc","task","discard",""]`, and it is
   `#[cfg(debug_assertions)]`-gated so the **release** symptom is an unrunnable route. Stated as a
   property of the verb it will mis-target the increment's test.

---

## J · Dispositions — appended 2026-09-07 (M50 Increment 13 / T2)

**Appended, not merged**, for the reason §7 of [baseline-ledger.md](baseline-ledger.md) states: §§A–I
are a dated record of what was driven at `cdce9fc`. Same posture — every verdict below **driven at
`HEAD = 4080fa6`** against the release `target/release/jigc` on `dev/jigc-rig` corpora unless a cell
says *source read*. N1–N20 are disposed in that file; N31 is a row that did not exist when either
artifact closed and is disposed here so the wave leaves one ledger, not two.

| # | verdict | disposition |
|---|---|---|
| N21 | ADMITTED | Tier 0 (D2), family 2 — **Increment 2 / T2**. Driven: `jigc rename 'vision:vision' --to "Renamed vision" --slug "../../src/pwned"` exits 1 with `write.malformed-slug`, located on the doc, routed at the grammar. |
| N22 | ADMITTED | Tier 1 (D8), the home rule at **both** knobs — **Increment 4 / T2**. Driven: `jigc config set docs-root .jigc` exits 1 with `config.workbench-root`, naming *"the tree `jigc uninstall` removes whole"*. |
| N23 | CARRIED | **Driven, and it reproduces exactly.** With `arch-doc` and its two workflows removed from a loaded pack over a committed, tracked `docs/architecture/padding-layer.md` (the workflows matter only to isolate the answer — leave them in and `validate` reports a `workflow-refs.schema-ref-resolves` finding about the *definition*, still exit 0, and still not one word about the orphaned doc): `jigc validate` prints *"no findings — the committed store validates clean"* at **exit 0**, `jigc doc list` drops the row **entirely** (not even `unregistered`), `jigc migrate-corpus` reports `0 migrated, 0 already current, 0 blocked`, and `git ls-files` still carries the file. Only `jigc doc show arch-doc:padding-layer` tells the truth (`store.unknown-type`, exit 1) — and only if you already know the address the store surface no longer shows you. The staged arm contemplates this state by name ([doc-read-surface.md](../../../design/doc-read-surface.md):167, *"a type the resolved cascade no longer defines"*); the committed arm's enumerator simply skips it. Not a data-loss cell — the bytes are tracked and `git`-recoverable — and no tier is open to admit it. → `decisions-pending.md`, with a trigger. |
| N24 | ADMITTED | Tier 0 (D2), family 1 — **Increment 1 / T1**, whose entry records it *disposed by driving*: `post_commit`'s `remove_dir_all` has exactly two production sources (`TaskArea.dir`, `milestone_dir(…)`), both now downstream of the grammar guard, and both a real `jigc task finalize` and a real fan-out boundary land green. |
| N25 | ADMITTED | **Increment 12 / T1** — `crate::render::repo_relative` is the one home, with five `DeclaredAbsolute` rows each carrying a reason. **Its declared bound is live and is why N31 exists**: the class's domain is *what the destroying and provisioning doors print*, and the standing source fence reads `.display()` over `crates/cli/src/milestone.rs` only. |
| N26 | CARRIED | **Driven as a real listed project pack** (PB-1's actual population — a `packs:` entry beside the shipped marker, not a `JIGC_PACK_DIR` stand-in), and it holds, with two corrections and one new fact. A project pack shipping `note.yaml` and **no** `config/schema-manifest.yaml` loads at exit 0, `doc schema note` declares no `schema-version`, and the ingested instance's front-matter is empty — so `schema-conformance.schema-version-current`, the intrinsic whose entire job (M42) is to flip `jigc validate`'s exit code over an unmigrated managed corpus, **can never fire for that pack's doctypes**. After a shape change (a new required section): `migrate-corpus` reports `0 blocked` and offers **no route**, and `doc show` blocks `store.unparseable` routed at *"fix the committed file so it conforms to its schema"* — hand-repair, not a migration. **Correction 1:** *"silently"* is too strong — `jigc validate` **does** report `conformance.section-missing`; what is missing is the exit flip and the migration route, not the report. **Correction 2:** the fence half is a **stated** design (`schema-manifest.yaml`'s own header: *"an absent manifest = unchecked"*), so the row's defect is the consequence, not the absence. **New, driven, and it is the load-bearing half:** the escape hatch does not open. Adding a correct manifest to that same project pack (slug-rule block, right hash) makes **every door** exit 1 at the pack-load *stated-at* fence — *"no step of this pack declares `states-constraints:` for the ambush-class code(s) `finalize.promote-clobber`, `finalize.left-out`, `finalize.nothing-staged`, `finalize.carried-staged`"* — A/B'd in one run against the same tree. A schema-only project pack therefore cannot buy the stamp without also shipping workflow steps it has no reason to own. → `decisions-pending.md`, with a trigger. |
| N27 | CARRIED | **Driven.** `jigc task diff <id>` over a cold task prints `# staged docs` and the commit doc's skeleton — no `# code changes` header, and no task id, workflow, intent, bound roles or provenance; `--format json` carries `task` and `base` but none of the rest. D3 landed fork 2's **orientation** half (Increment 5), not a `task diff` frame, so the charter's second-half claim — *no read verb answers "what is here"* — is only partly answered at the door the baseline's claim 4 nominated. → `decisions-pending.md`, keyed to the duress-cell re-measure the next trial owes. |
| N28 | CARRIED | **Driven, and it is inert today for a stated reason.** A second forward edge declaring an inverse name another edge already declares (`arch-doc.also-derived-from → prd, inverse: has-specs` beside `spec.derived-from`) is accepted at exit 0 by pack-load, `describe`, `doc schema prd` and `validate`. The reason it costs nothing yet: the inverse **name** reaches exactly one surface — the `schema-completeness.inverse-cardinality` advisory's prose phrase *"(its `has-specs` inverse)"* — while that finding's `(code, target)` key is built from the **forward** relation (`index.rs`'s `{target}#{relation}`), so two colliding edges still discriminate. The read-view injection the format's own doc describes (`document-type-schema.md:81`, *"the derived back-edge in the target type's read view"*) is **unbuilt**, and that is exactly what would make the namespace readable and the fence necessary. The razor's necessity leg fails **today**; the condition that flips it is the trigger. → `decisions-pending.md`, with a trigger. |
| N31 | CARRIED | **A row neither artifact carries — found at Increment 13 planning while driving N15, re-driven and widened at T2.** `jigc doc show vision` over a repo with no committed `VISION.md` prints a `store.not-found` block reading *could not read `vision:vision` at `/private/var/folders/…/repo/VISION.md`* — an **absolute host path on a read door**. Driven at T2 the class is **three** doors, not one: `store.not-found` (`engine/src/store.rs:151`), `store.unparseable` (`:275`) and `finalize.render-io`'s missing-commit-doc locus at the milestone boundary, each interpolating `path.display()`. It is **not** a regression of Increment 12 / T1: that task's domain is declared as what the destroying and provisioning doors print, and its standing source fence reads only `crates/cli/src/milestone.rs`. The repair is a **seam** question rather than a relativization — two of the three producers live in `crates/engine`, which ships filesystem-free and holds no repository root, the same structural reason T1 declared the orientation header's cascade homes absolute — so the choice is *give the engine a root* or *relativize on the CLI's way out*. → `decisions-pending.md`, with a trigger, on the `decisions-pending.md` → *`engine::validate::adoption_route`* precedent (a defect found during an M50 task, routed to the completion audit's triage rather than fixed under a task that carries no mandate for it). |
