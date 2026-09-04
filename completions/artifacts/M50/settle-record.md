# M50 — the Settle record

Settled 2026-09-04 with the human, against [baseline-ledger.md](baseline-ledger.md) (five
capability-auditors) and [gap-findings.md](gap-findings.md) (four gap-detectors), with three
independent `robust-advocate` runs, two driven spikes, and an independent `design-reviewer` pass
that returned **13 blocking findings**, all accepted. Every fork was decided by the human on two
honestly-argued cases; none was self-framed by the proposer.

**Posture.** Every fact this record turns on was driven against the **release** binary
`target/release/jigc` (`1.0.0-rc.13`) at `HEAD = d9e91f1`. Orchestrator-driven personally: W-13,
`task discard "../.."`, `rename --slug "../../src/pwned"`, `doc show 'research:../../../outside'`,
and `rename 'research:../../src/planted'`. All else relayed from an agent that drove it, marked as
such in the input artifacts.

---

## The claim, final

> **No caller-supplied token becomes a path component without the door validating it against the
> grammar, home or value rule that token's own family already declares — and `jigc start` tells the
> truth about task state on both its text and its versioned envelope.**

**Four families, two rule kinds.** A slug grammar governs families 1, 2 and 4; a home rule and a
value rule govern family 3. The one-predicate framing was conceded wrong by the robust-advocate
against its own interest: `trackable.rs:19-23` refuses to ask about gitignore **by design**, because
`relocate`'s squatter displacement depends on it, so family 3 cannot ride the trackability
predicate. Any wording claiming a single predicate is rejected.

| family | subject | scale | driven worst case |
|---|---|---|---|
| 1 · resolve | `--task` / `milestone_id` | 25 doors, 5 seams | `task discard "../.."` destroys the repository incl. `.git`, exit 0 |
| 2 · mint identity | `--slug` at `jigc rename` | 1 door (the 6th of 6) | a managed doc lands outside the docs root; `doc list` omits it, `doc show` serves it, `validate` says clean |
| 3 · root knobs | `docs-root`, `placement-root` | 2 knobs | `uninstall` destroys committed docs and an untracked file at exit 0; a file-shaped root lands the knob with every move failed |
| 4 · address slug head | the `<slug>` of `<type>:<slug>` | 9 `DoctypeArg::Address` doors + `old_slug` | `doc show` serves outside-the-repo bytes through the **pinned JSON**; `rename` moves an arbitrary in-repo file into the docs root and **commits it**, exit 0 |

---

## Decisions

### D1 — `jigc task discard <valid-id>` refuses over staged prose, with `--force`
Joins `uninstall`'s **staged-prose** guard, extracted to a shared home and applied at both doors, so
the contradiction is impossible rather than merely fixed. **Not** `DESTROYING_DOORS` membership —
its subject is worktree-shaped paths probed for git linkage, which would refuse every live task.
*Counter-evidence weighed and found weaker than first framed:* M46's `--ignored` measurement was
about build output on the ordinary success path; authored prose no commit holds is a different
population, and a task's ordinary success path is `finalize`, not `discard`.

### D2 — Tier 0's subject is families 1, 2 and 4; family 3 rides Tier 1

**Why these three together:** they share one predicate (`engine::slug::is_slug`), one refusal string
that already ships at five doors, and one axis. **Why family 3 is separate:** it needs *different*
rules, and folding unlike rules under one banner is the false-completeness shape that has bitten
four waves running.

**Why 2 and 4 cannot ride family 1's guard:** both are **argv-disjoint** from any resolve seam —
neither `jigc rename … --slug X` nor `jigc doc show '<type>:X'` carries a work-unit id — so the
chartered acceptance (walk 17's 25-door table) definitionally cannot reach them. That is M49's
MEDIUM shape restated: green because the shape space iterated the wrong dimension.

**The predicate.** `is_slug` (`slug.rs:193`), whose own doc-comment names it the **recognition**
predicate for *"an authored anchor, a ref body, a frozen id read back from disk."* The baseline's
*"three mint boundaries, zero resolve seams"* framing was wrong on the axis: `slugify` mints,
`is_slug` recognizes, and the resolve seams are its paradigm sites.

**Proven not to break anything shipped.** Every mint shape conforms (incl. `ÄÖÜ Straße` →
`aou-strasse` and M44's path-hash form); 46 `--task` literals across packs, tests, adapters and
guides are metavariables or conforming; one golden literal, a placeholder; `no_such_task_route.rs`
drives `nonexistent`, which `is_slug` accepts — suite run, **3 passed**. All five other `--slug`
doors already refuse the identical value byte-identically.

**The refusal, fully specified (design-review B7).** A malformed token is **not** `no task <id>` —
the token was never an id, so that route (`jigc task list`) would be a law-1 misdirection.
- **Code:** a new `*.malformed-id` member, blocking, one per family, minted at the **seam**, not at
  clap. A clap `value_parser` yields exit 2 with **no code and no route** — an M43 route-floor breach
  at 25 doors — so the guard sits below clap and above the filesystem op, and produces a `Finding`.
- **Route:** names the grammar and the discovery verb for that family (`jigc task list` only where a
  *well-formed but unknown* id is the likely intent; the malformed case states the grammar).
- **Stated behaviour change:** the four divergent `--task ""` texts (five, counting `task bind`'s,
  driven) **collapse to one**. This is intended, it is a change at 25 doors, and
  `no_such_task_route.rs` is its acceptance — proven by running the suite, not inferred.
- **Scope, stated:** family 1 fires at the five resolve seams; family 2 at `rename.rs`'s
  `slug_override`; family 4 at **address parse**, over *the slug head of every address at every
  `DoctypeArg::Address` door*.

**Registries — derived, not manufactured.** Family 4's is the strongest and already ships:
`DOCTYPE_DOORS` filtered to `DoctypeArg::Address`, bijectively fenced against the real clap tree by
`every_doctype_door_is_registered` (`cli.rs:1549`, `:2239`). Families 1 and 2 mint
`WORK_UNIT_ID_ARG_IDS = ["id","task","milestone_id"]` and `SLUG_ARG_IDS` by the same shipped
`DOCTYPE_ARG_IDS` pattern.

**Also in family 1's axis, both driven:** `post_commit`'s second `remove_dir_all` (`task.rs:3431`),
silent by design; and `milestone discard ".."` stopped only by an *incidental* missing sibling file,
code-less and route-less.

**Census gap recorded (A3):** `jigc config insert-step` / `replace-step` mint a step id from a
`file_stem` with no grammar check (`config.rs:1215` checks collision only). Not a traversal — a
`file_stem` carries no `/` — but it is a fourth mint site the *"the rule is already stated"* claim
must not overstate.

### D3 — one `ActiveTask` orientation variant, `SCHEMA_VERSION` 2→3
`jigc start --format json` emits `"state": "clean"` — declared at `result.rs:105-110` as *"no active
task"* — over a live task, and the `refs-post-hoc` golden **pins the falsehood as expected output**.
Law 1 on a versioned machine contract, true at N=∞, independent of the trial.

Ships: one variant carrying `id`, `workflow`, `intent`, `base`, `staged` and `findings`-as-data,
**collapsing `bootstrap.md`'s states 3 and 4** (they differ only by a findings count) so no second
bump is owed; four `Run:` directives delivering the routing *inside* the push channel; filled from
files already on disk via two enumerators that already ship.

**Settled by the design review:**
- **B8 — the selection rule is stated per `start` form**: bare (orientation) renders the variant;
  the router form and `--workflow` form **name the open task without suppressing a second mint**,
  because `render.rs:312` composes *"several open tasks are legal … run them in parallel"* into every
  task and `start_compose.rs:2153` pins it. **N17** — the silent second mint, the one door on F-5's
  axis that changes the repo — is admitted here rather than left undisposed.
- **B8 — the N>1 shape is settled**: the variant carries the active set, not one task, so two live
  tasks render as two rows under one tag. A singular variant would have forced the build to guess.
- **B9 — the `Run: jigc task discard <id>` directive names the consent** (`--force`) or is
  conditioned on `staged` being empty, since **D1 makes that route refuse in exactly the state D3
  renders**. A route the wave's own guard blocks is a route-floor defect the wave *creates*.
- **A5** — the golden count is *the goldens over fixture states with a live task* (2 today), not a
  literal.

*Rejected shape:* `active_tasks: [...]` on `Clean` — it retains the lie and makes it
self-contradicting inside one document.

*Why not the text-only cut:* it is **not** zero-contract. The `staged_listing_hint` stderr precedent
would make orientation's text say one thing and its JSON another in one invocation — the gap
`command-output-contract.md:393` names — and `text_json_parity_axis.rs:130` disposes `start` as
`DeclaredOut` on a rationale about the composed `{task, text}` arm, never `OrientationView`. It would
ship a divergence through the one blind spot in the wave's own fence. And its routing target is
demoted by a locked doc: `doc-read-surface.md:15` already superseded the `task diff` route, and
driven, `task diff` does not diff docs at all.

### D4 — fork 6 dissolves into D3's bump
`introspection.md:56` (*"no version governs it"*) is a **drifted restatement** of a fact homed at
`doc-read-surface.md:167`, whose `:175` already assigns `describe --format json` the result
contract's `schema_version`. It narrows to content. The `origin_pack` key on a definition bumps
`SCHEMA_VERSION` — **the same bump D3 takes**.

### D5 — fork 7: `setup` warns through its existing `findings` key
`setup --format json` already ships a `findings` array, empty. A warning naming what the next door
will refuse is an existing slot, not new contract surface. The declared bound (the bootstrap door
does not itself refuse) stands; only its silence goes. Home: `design/corpus-migration.md` → The
freeze, cross-referenced from `design/project-setup.md`.

### D6 — the nested migration locus, **full cut**
`MAX_NESTING_DEPTH` — **derived**, `(MAX_FRAGMENT_HOPS − 1) / 2 = 2` — gives three loci;
`migrate-corpus` iterates two. `schema_diff.rs:746-753` compares a nested `Leaf::Repeatable`
**wholesale** and pushes `Unclassified`, with no recursion. Driven six ways on real `changelog.yaml`.
**Any increment hardcoding "three loci" re-enacts M45's *statement == constant* failure.**

**Decided on leg 0's second disjunct — a hole in a declared surface — not on irreversibility.** The
advocate conceded the one-way-door tell does not fire. What fires: three locked statements assert a
closure false at locus 3 (`corpus-migration.md:197`, `:290`'s *"empty by construction"*, CLAUDE.md's
*"the migration class is closed"*); the deferral's **own written trigger describes HEAD** and has
stood fired since it was recorded; and the prior refusal's ground (`M49/settle-record.md:66`) is the
port-as-requirements rationale M49 retracted at `:35`. The escape hatch damns it: the stuck adopter
follows the pack-load fence's own advice, deletes their manifest, and `validate` then reports *"the
committed store validates clean"* at exit 0 over a stale corpus.

**Spiked and driven (the owed `acceptance-spiked` obligation, discharged before decompose).**
The cell the human bought sight-unseen is **cheaper than priced and not a new primitive**: it is
`insert_item_slot` generalized over a locus path, exactly as `set_item_slot → set_nested_item_slot`
already is — and that shipped generalization is the measurement (78 → 52 lines). **~25–35 new
lines**, landing as one function taking a locus with the flat form as the empty case. Every
navigation helper ships and is on the real `doc set-field` path; `render_item_at` is already
depth-parameterized.

**The reserved-depth interaction is clean** — sub-labels render at `#####`, `slot_ceiling` reserves 5
and first-allows 6, and the two seams that disagreed by one level in M49's HIGH **agree** here. Two
structural refusals remove the ambiguities: at most one nested repeatable per block, and a depth-3
nest is refused at schema load, so `{multi-slot, nested}` at depth 2 is unreachable.

**Three carry-forwards into the increment:**
1. **The `render_item` → `render_item_at(depth)` swap at the byte-fidelity pre-image compare**
   (`write.rs:3341`) is one word, and getting it wrong refuses **every** doc with `UnmodelledContent`
   — reads like a corpus problem, is a code problem. Explicit red test.
2. **`ItemSlotError::UnmodelledContent` needs the item chain**, not the bare anchor: same-anchor
   nested items under different parents are legal, so the refusal is otherwise unfollowable.
3. **N29 and N30 land in this increment.** **N29** — every conformance finding raised inside a nested
   item composes an address the tool's own grammar refuses (`validate` emits
   `#releases/1-0-0/added/bogus`, dropping the `changes` hop; `doc show` answers
   `store.no-such-section`), **reachable on the stock shipped `changelog` at HEAD with no reshape**;
   cause is one missing `prefix_hop` at `parse.rs:1287-1298` whose comment asserts the composition it
   does not do. By the wave's own lens that is an **axis**, not an instance. **N30** —
   `conformance.item-slot-label-missing` hardcodes `####` while the writer emits `#####` at locus 3,
   and authoring `####` there is refused by `write.slot-heading-depth`: a law-1 lie whose
   instruction the next door rejects, and it becomes reachable exactly when this cell ships.

**Registry (design-review B11).** `SchemaChange::ALL` cannot exist (every variant carries data). The
working shape is a discriminant enum + exhaustive `From<&SchemaChange>` — the house pattern
(`SetKind::ALL`, `RefusalKind::ALL`, `ConfigAckArm::ALL`). **The record carries the 18 × 3
disposition table** (variant → locus → verdict → code where it refuses), because counts are not a
set and the two sentences *"4 refuse, zero bytes"* and *"the locus closes with no refusal cells"*
cannot both stand. A locus-3 refusal must not silently inherit `migrate-corpus.fold-refused`'s
documented permanent dead end. The arm iterates `SchemaChangeKind::ALL × loci` with the locus count
**derived from `MAX_NESTING_DEPTH`, never a literal**, and no cell permitted to read `Unclassified`.

### D7 — the `adr → research` ref is **REFUSED**; the fence and the `to:` key ship
**Settled three times; refused on the third.** The dev-pack arm was refuted by the design review
(B2) after the methodology-shadow arm was refuted by a spike:

- **B2 — the decision contradicts its own prerequisite.** `compose-embedded-methodology: false` is a
  **reachable, unit-tested** composition (`pack.rs:3459`, driven) in which the dev pack loads and
  `research` does not exist. The dev-pack ref would then point at an absent doctype, and D7's own
  N13 fence — first subject site *pack-load* — must either **block every door at exit 1 for the
  embedded dev pack** or not fire, in which case it does not fence. Same for `JIGC_PACK_DIR`, and it
  collides with M49's PB-1 bound: a vendored `adr` carries a ref to a pack the fork cannot vendor.
- **B3 — foreclosed by a locked doc whose rationale was never engaged.**
  `design/methodology-docs.md:42` settles it the other way (*"a managed `ref: {to: adr}` would point
  at a doctype outside the composed schema universe"*), and the rationale is **symmetric**: the dev
  pack composes alone too.
- **The necessity leg fails.** An optional ref is `AddedOptionalField` — driven, one command, one
  auto-commit, one changed byte-line, zero authoring: **the cheapest migration jigc has.** F-6 is
  therefore *not* expensive after the pin, which is the human's own admitting criterion.

*Three shapes failed on one fact — the pack boundary is real in both directions.* F-6 stays open; an
ADR cites research in prose, as today.

**Ships regardless, on its own merits:**
1. **The pack-load fence that a `ref`'s `to:` resolves in the composed schema set** (N13), over three
   sites — pack-load, `index.rs:525-527`'s silent `continue`, `doc schema`'s projection. **Scope
   stated:** which pack-sets it fires over, and what it does with a `to:` resolvable in one
   composition and not another. A fence of this shape already ships —
   `workflow-refs.schema-ref-resolves` asks the composed-set-membership question for `{{schema:}}`
   placeholders; this is the same question never asked for a `ref`.
2. **A `to:` key on `doc schema`'s ref projection.** Today neither arm names a ref's target doctype,
   while `write.malformed-value`'s route says *"run `jigc doc schema adr` to see the field's declared
   type and members"* — **a route to a surface that cannot answer for the field kind that raised
   it.**

**Carried findings from the spike:** the freeze error names **neither the pack nor the manifest
path**; a *losing* shadow is still freeze-checked, so a drifted hash blocks every door over bytes
that reach no surface; and `workflow-refs.schema-ref-resolves` prints a **blocking** finding while
`validate` returns exit 0 at store scope.

### D8 — family 3: the home rule **and** the value rule, at both knobs
**The home rule.** A third predicate — *this path is jigc's own workbench* — refuses `.jigc` and its
children at **both** root knobs via a new `ROOT_KNOBS` registry replacing the two hand-written
`matches!` (`config.rs:319`, `:368`). It cannot key on gitignore or trackability
(`trackable.rs:19-23` requires a gitignored destination to stay permitted).

**The value rule (design-review B10).** The same registry carries a value predicate: a **file-shaped**
root is refused rather than landing the knob with every move failed — the state
`config.rs:350-363`'s own rationale says must not exist (N5); an **absolute** value is refused rather
than silently reinterpreted as repo-relative while the knob echoes back the absolute form (N6); a
**symlinked** root is refused or canonicalized so `doc list`'s path and git's recorded path cannot
disagree (N7).

**`uninstall`'s guard (design-review B4 — the correction is load-bearing).** The widened subject is an
**added third subject**, not a replacement: `worktrees/` and `tasks/` are *inside* `ENTRIES`, so the
literal reading would have re-opened the two defects M46 Inc 2, M47 and M49 built guards for. Its
subject is *paths under `.jigc/` that are **(a)** not under any `ENTRIES` prefix **and (b)**
untracked* — the untracked conjunct is mandatory, because every non-transient `.jigc/` path is
**tracked on a fresh `setup`**, so without it the guard fires on every install. A *tracked* one is
**narrated, not refused** — it is `git checkout`-recoverable, which is D8's own reasoning. And
`ENTRIES` is a 7-prefix `&str`: the membership test is a path-prefix computation plus a git query, so
the flow-51 arm states it as a **derivation**, not a registry.

The axis is **both knobs** (`docs-root` reaches the identical loss, driven) and the losing class is
the **untracked** one. `untrackable` has **0 hits in all of `design/`**; the rule gets a home at
`design/storage.md` → Placement.

### D9 — the adapter **denies** the two human-owned destroyers
**Re-settled after the review (B5, B6).** The profile has exactly two keys under
`deny_unknown_fields` (`adapter.rs:265-281`): **`deny` blocks, it does not prompt**, and `permit` is
a *prefix* pattern, so "leaving the permit" changes nothing unless the permit becomes an enumeration
of all 47 `VERB_KINDS` leaves. And `inject_permit` is purely **additive** (`adapter.rs:660-691`), so
narrowing a permit **reaches zero existing installs** without a prune of a user-owned file, which
M48's refuse-to-clobber posture refuses.

Ships: `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)` join the profile's **deny**
list — both are human actions, so blocking is the correct semantics rather than a prompt jigc cannot
express — and **deny entries are injected additively like permits, so this reaches installed repos.**

**`milestone provision` is dropped from the carve-out (B6)**, on two grounds: ordinary provision
destroys nothing, its destruction living behind `--force` and M48's fail-closed classifier — the
carve-out was aimed at the verb where the destruction is in the flag; and D11 makes the fan-out the
wave's own unattended Fix-phase mechanism, so prompting there would **park an unattended round on a
prompt nobody is watching**.

`design/assistant-adapter.md:89`'s *"No self-collision … The floor constrains the agent, not the
CLI"* is corrected: it reasons only about jigc's own `git` shell-outs and never contemplated jigc
performing an **agent-supplied** deletion.

### D10 — W-15: the route floor over the whole class, the declared bound intact
`design/validation.md:77` carries a **stale universal and a live bound** in one line. The universal
(*"true at every door"*) is driven false at `remove-item`/`retitle-item`; the bound is **correct on
its merits** — the single hop is a field-id search across every section, so minting
`write.unknown-section` there would be a law-1 lie on `#thesis`/`#meta`, which are declared sections.
**The charter's prescribed fix is refused by the razor at leg 1.**

**Enumerated, not counted (design-review B13).** *"~10 bare cells"* is not a set and *"a code"* is not
a code. The set exists: `write_miss_shape_axis.rs`'s `CELLS` (`:594`) and `NO_SECTION_HOP` (`:1311`),
with the fence at `:1330`. Each cell is enumerated in the decomposition as a `CELLS` row **with the
code it earns** — the `write.unknown-section` / `write.unknown-field` / `write.not-present` split is
`design/validation.md`'s own route split, and the route is a function of the code — and each new row
states whether it joins `CELLS` or `NO_SECTION_HOP`. `validation.md:77`'s universal is corrected in
the same edit.

### D11 — the fan-out Fix phase, partitioned by **projected write-set**
**Re-settled after the review (B12).** "One sub-task owns one file" is refuted by this repo's own
structure: since M47 `autotests = false` and a new suite file must be registered in exactly one of 12
group roots, so two sub-tasks adding tests both edit one file and `join` blocks with the very refusal
the rule exists to route around. Same for the 624 goldens, `Cargo.toml`, and any doc fold-back.

Ships: triage partitions by each fix's **projected write-set**, so a finding spanning two files is
one sub-task and two findings in one file are one sub-task. **Shared artifacts — the group roots, the
goldens, `Cargo.toml`, the doc fold-back — are excluded from the fan-out** and applied serially by
the orchestrator after the join, since no sub-task can know what the others will need until they are
done. A round whose write-sets cannot be made disjoint **falls back to serial** per `fix-gate.yaml`,
stated rather than discovered.

**Prerequisite, not a rider:** N12 — the `squash: false` ack is wrong on the **pinned** JSON
(`committed.hash` owns 2 of its own 4 `manifest` entries; the per-fix shas appear nowhere;
`sub_tasks[]` has no `hash`), and that envelope is exactly what a fix-round orchestrator reads.

### D12 — Tier 3: pin every row, its own increment if needed
All 21 `UNPINNED` rows of `RC-m50/findings-verification.md` are discharged, plus the two owed
regardless (`doc rename --task` preserving header fields and slot bodies; the milestone door's
finding codes). Rows whose defect an M50 increment closes are pinned by that increment's own test.

**Carried bound:** M48 deliberately left a row unpinned because *a standing test would pin data loss
as expected output*. "Pin every row" cannot mean encoding data loss as expected: such a row is pinned
**to the fixed behaviour** if M50 fixes it, and otherwise keeps a stated `UNPINNED: <why>`. The
decomposition names which, and the count is reported honestly.

### D13 — what N1 owes beyond the fix
- **A post-hoc sweep of the archived RC invocation logs** for degenerate-id calls at any door.
- **No adopter disclosure is owed — recorded as a decision, not by silence.** No third party holds
  the binary; no root `CHANGELOG.md` exists to land it on.
- The next trial's observer briefing discharges at `decisions-pending.md:190`, from the increment
  records rather than memory.

### D14 — every driven defect carries a disposition (design-review A1)
**19 of the 28 defects in the two input artifacts appeared in no decision and no refusal** — and two
of them (N3, N24) are family-1 members while N17 is F-5's, so silence would have dropped rows from
axes the wave claims. Each of N3, N5–N11, N14–N17, N19, N23–N28 takes a one-line disposition in the
decomposition: **admitted to a tier**, **refused with its leg and citation**, or **carried to a
triggered ledger entry**. The wave's own razor rule — *if the razor cannot refuse, the claim is
wrong* — applies to the undisposed set as much as to the chartered one.

---

## Ordering constraints the decomposition must respect

1. **D3 and D4 share one `SCHEMA_VERSION` bump.** Both are projection changes an external consumer
   must notice, and the integer is **global** — orientation, `describe`, the validation report and
   the block/error envelope. They land in the **same** increment or the second pays a second global
   bump for a local addition. Any other item that moves the integer joins them.
2. **D6's discriminant registry lands before its acceptance arm**, or the arm has no set to read and
   pins instances — which M45's contract forbids.
3. **D2's `WORK_UNIT_ID_ARG_IDS` / `SLUG_ARG_IDS` land with their guards**, for the same reason.
   Family 4's registry already ships and needs none.
4. **N12 lands before D11's pack work** — the fix-round orchestrator reads that envelope.
5. **D8's `ROOT_KNOBS` lands before its home and value predicates**, which both read it.

---

## Fired triggers disposed in this Settle

1. **`decisions-pending.md:510`** — the empty-`--task` admission, recorded 2026-08-31, trigger *"the
   task-door admission axis is next opened"*. Fired by D2; its *"deliberately prose, not a test"*
   clause retires as D2 converts it.
2. **Orientation states 3/4** — deferred in a doc-comment (`render.rs:94-96`) on the premise *"no
   task store yet"*, false since M1, carried in **no** ledger entry. Discharged by D3.
3. **The Nth-demand counter** (`decisions-pending.md:14`) — `ideas/state-aware-compose.md` records
   demands at 2026-07-12 and 2026-07-15; RC-m50 is the **third**, which by rule forces the item onto
   this agenda. The dated line is appended, and `VISION.md` → Open questions — stale by **four
   trials** — takes its batch lines, including the "no new parks" form where that is the truth.
4. **`decisions-pending.md:176`** — `AddedNestedRepeatable`'s own trigger, standing fired at HEAD.
   Discharged by D6.

## What the razor refused, with citations

- **W-15 as chartered** (D10) — `validation.md:77` states the bound and HEAD obeys it. Leg 1.
- **The `adr → research` ref** (D7) — `methodology-docs.md:42`'s universe rule, engaged rather than
  overridden, plus the necessity leg failing because `AddedOptionalField` makes it a one-byte-line
  1.1 addition.
- **`DESTROYING_DOORS` membership for `task discard`** (D1) — wrong instrument, not a wrong goal.
- **Family 3 inside Tier 0** (D2) — unlike rules under one banner is the false-completeness shape.
- **A doctype or schema change for fork 8** (D11) — `completion-record.findings` is a top-level
  repeatable, so a per-finding leaf is `AddedItemField`, cheap after the pin too.
- **`milestone provision` in the adapter carve-out** (D9) — M48's fail-closed classifier already
  guards it, and the carve-out was aimed at the verb where the destruction is in the flag.
- **`active_tasks: []` on `Clean`** (D3) — retains the lie and makes it self-contradicting.
- **A new part-doc for the id rule** — an existing concept's missing half, not a new concept.

*Two of these are product-shaped items refused with citations. The charter's exclusion list was four
entries and all four were build/trial infrastructure; the razor now demonstrably refuses.*

## Docs that move (the fold-back is scope, not tidying)

`design/structural-grammar.md` (the resolve half of `:98`'s discipline) · `design/finalize.md:29` and
`design/write-commands.md:182` (both specify *exists* where the rule is *a valid id that exists*;
`:182`'s orientation rationale is separately falsified) · `design/write-commands.md:167` (one of four
elements delivered) · `design/write-commands.md:305` (the M43 convergence rule was written for a
*wrong* id, not an *invalid* one) · `design/bootstrap.md` (four states → what ships, with a
*triggered* ledger entry for any remainder) · `design/validation.md:77` (D10) ·
`design/introspection.md:56` (D4) · `design/design-altitude-doctypes.md:43-47` (false since M40) ·
`design/methodology-docs.md:42` (**kept, and cited as D7's ground**) · `design/methodology-docs.md:40`
and `design/corpus-migration.md`'s two-loci census (D6) · `design/storage.md` → Placement (D8's home) ·
`design/assistant-adapter.md:89` (D9) · `design/team-ready-state.md` (D1's shared guard) ·
`implementation/pinning.md:15` (claims a prerequisite that shipped at M45 — uncorrected it buys a
phantom increment) · `implementation/milestone-completion-workflow.md` §3 (D11's loop) ·
`crates/cli/src/render.rs:94-96`, `milestone.rs:3737-3757`, `config.rs:350-363`, `parse.rs:1300-1301`
(doc-comments that state what the code does not do).

**Priced, not assumed:** `QUICKSTART.md:170-172` and `MIGRATING.md:40` both state the destroying door
as safe, and `setup.rs:69-70` `include_str!`s both into the shipped `.claude/skills/jigc/SKILL.md`.
Editing them moves `jigc-body-blake3`, engages M48's `adapter-guide.user-modified` refuse-to-clobber
path, and moves goldens. **A product surface change, not a docs edit.**

## Instrument note — this Settle's own failures, recorded

**Two decisions were settled on the orchestrator's reading and refuted by driving.** D7's
methodology-shadow arm fell to a spike: *"the methodology pack already shadows `commit`"* is true of
the files and false of the shipped composition, where dev wins and that shadow is **itself already
inert**. Then D7's replacement fell to the design review for the **same shape one level out** — the
spike checked the *shadow* and nobody drove the *marker-false composition* the dev-pack ref depended
on. The design review's own summary is the honest verdict: its four highest-value findings are all
that shape.

The rule, and it binds this session as much as any subagent: **a claim about how the composed
product behaves is not established by reading the files it is composed from.** The inputs to this
Settle were driven throughout; the two places the orchestrator substituted its own reading for a
driven fact are the two places the Settle was wrong — and both reached a *decision*, not a draft.

**What worked:** both were caught before decompose, by instruments the workflow mandates and the
orchestrator did not get to frame — a spike sent because the arm was flagged conditional, and an
independent review that did not author what it reviewed.
