# M52 gap probe — dimension: DECISIONS

**Probed 2026-09-17 at `HEAD = 85ad06c5`** against the installed release `~/.local/bin/jigc` =
`1.0.0-rc.15` (confirmed by `jigc --version`). Fixtures from `dev/jigc-rig <state> --binary
~/.local/bin/jigc`, two-step eval, roots under `mktemp -d`; nothing written into the working repo,
no cargo run. Every claim is marked **[driven]** (I ran it, argv inline) or **[read]** (a source or
doc read — a lead, not a finding).

Inputs read in order: `implementation/decisions-pending.md` §*The rc.16 wave (M52)* (lines 22–103) ·
`completions/artifacts/M52/baseline-ledger.md` (all) · `completions/artifacts/M51/per-axis-review/README.md`
headers + §A/§D · `completions/artifacts/M51/settle-record.md` D1–D6, D13–D15 + the 2026-09-11 review
amendments §1–§7 · `DECISIONS.md` (the 2026-08→09 entries the forks touch).

---

## §1 · BLOCKING — decisions the charter does not list that the baseline forces

### B1. No decision owns the **scope boundary between the 39 rows and the baseline's 20 latent defects** — 11 of which are the tier-0 shape
`baseline-ledger.md:272-310` lists **20 driven latent defects in no §A row**, of which **eleven are
exit-0 byte loss or repository harm** — the exact predicate the charter uses to define tier 0
(`decisions-pending.md:33`). Examples: `unwind_mint` `remove_dir_all`s a minted task area,
destroying third-party authored prose with no git copy and no note (`baseline-rollback` L6, called
*"the most severe cell in the rollback class"*); `jigc uninstall` destroys a plain file at any
`ENTRIES` name at exit 0 with no `--force` and no narration (`baseline-destroying` L-2);
`jigc milestone discard` fails **open** on an unreadable `.jigc/worktrees/` and settles the record
irreversibly at exit 0 while two live worktrees still hold uncommitted work (`baseline-destroying`
L-1). **[read]**

The charter's deliverable is *"all 39 rows closed as classes"* (`decisions-pending.md:30`). It was
written **before** the baseline existed and therefore cannot have disposed these. M50's own standing
rule is the precedent it breaks: *"every driven defect carries a disposition, and the carried ones
carry a trigger"* (`DECISIONS.md:1459`, M50 D14/T2). **The Settle must decide, in words, whether
tier 0 is the charter's 12 rows or the 12 ∪ the 11 driven exit-0 loss cells** — and if it is the 12,
each of the 11 needs a `decisions-pending.md` entry with a trigger, not silence. Leaving this
unstated is how a wave ships a tier called *exit-0 loss* over a baseline that drove eleven more.

*Why blocking:* the wave's own done-picture is *"no byte dies … at exit 0 behind a committing,
destroying or moving door"*. Three of the eleven are exactly that, behind doors already in
`DESTROYING_DOORS`. A tier-0 that excludes them makes the wave's headline claim false on its face.

### B2. Fork 1 has **no decision on what a rollback promises for a directory-tree population or for a gitignored one** — and M51 declared directories *out* with a stated reason
The charter's fork 1 asks *"one shared capture/CAS primitive every rollback path draws from, versus
per-site guards"* (`decisions-pending.md:94`). The baseline says the class is **nine populations**,
that **two of them restore a directory tree**, and that **four restore gitignored paths no object DB
holds** (`baseline-ledger.md:62-73`). **[read]**

M51's review amendment §6/A1 already took the directory question and took it the other way:
> *"The subject is `.jigc/.gitignore` and `.jigc/version` (A1) — the two files `finalize` rewrites —
> **not** the config *directory*. … a worktree pre-image family over a directory is a different
> shape (absent-means-delete over N files). **Named now so the build does not capture a directory.**"*
(`completions/artifacts/M51/settle-record.md`, amendment §6). **[read]**

So *"make them all CAS"* re-opens a decision M51 took deliberately, with a reason, four weeks ago —
and the charter does not say so. Two sub-decisions are missing outright:
- **(a) directory populations.** Absent-means-delete over N files is a different contract from
  absent-means-absent over one. Does a directory population get CAS, a door guard, or a stated
  `Exempt(reason)`? `baseline-rollback §3` also records that **two populations already restore
  correctly under a door guard instead of a CAS** (`rename.dirty-tree`; `setup`'s pre-write refusal)
  — so the evidence does **not** force one primitive, and the charter's binary framing hides a
  three-way fork.
- **(b) gitignored populations.** A CAS conflict's contract is *"do not overwrite — preserve **both**
  versions"* (amendment §6). For a tracked path git holds the second copy. For
  `.jigc/tasks/<id>/docs/*.md` **no object DB holds anything** — so *where* the preserved copy lives
  is undecided. The repo has a shipped precedent — `relocate` parks a conflicted pre-image in the
  gitignored `.jigc/displaced/` under a process-unique name, *"so a second refused finalize parks
  beside the first rather than overwriting the only copy of those bytes"* (`DECISIONS.md:944`) —
  but **nothing establishes that it transfers to a rollback population**, and `displaced/` is itself
  inside the tree `uninstall` and `milestone discard` destroy. Any plan sentence reading *"reuse the
  displaced park"* is `unverified-reuse` until spiked against a rollback conflict in a task area.

### B3. **Nobody owns "jigc's own writer set"** — fork 3's forced subject — and the baseline's own count of it is already short
`baseline-ledger.md:132-138` states fork 3's answer must be *"a complement of jigc's own writer set
(code-side derivable: `engine/src/state.rs`'s **seven** `*_FILE` constants + `cli/src/migrate.rs::SOURCE_FILE`)"*.
Grepped at HEAD, `crates/engine/src/state.rs` declares **nine** task-area file constants —
`BASE_PIN_FILE:32`, `ROLES_FILE:37`, `RENAMES_FILE:42`, `INTENT_FILE:51`, `WORKFLOW_FILE:60`,
`SOURCE_PATH_FILE:68`, `SLUG_OVERRIDE_FILE:77`, `PROVENANCE_FILE:215` (under `docs/`),
`STAGED_SNAPSHOT_FILE:788` — each driven onto `task_dir.join(...)`, plus
`crates/cli/src/migrate.rs:63::SOURCE_FILE` = **ten**. **[read]** The hand-count in the baseline is
**two short before the wave starts** — the exact shape (a hand-enumerated set that has already
rotted) this repo has recorded at M46 Inc 1 (*"found a fifth writer nobody had counted"*,
`DECISIONS.md:3908`) and at M49 (six fixers, all six found more, `DECISIONS.md:3772`).

**Missing decision:** who owns that set, where it lives, and what fences it. It is not `MINT_DOORS`
(that enumerates *doors*, `engine::state`), not `ENTRIES` (that enumerates `.jigc/.gitignore`
prefixes). A ⇔ fence has no obvious counterpart to biject against — unlike `VERB_KINDS`/`BEHALF_DOORS`
there is no clap tree to close the partition over. **So fork 3 cannot be settled as "the complement
of a registry" until someone decides what makes that registry complete.** Say so, or the destroying
guard's subject is a hand list guarding against hand-list rot.

### B4. `jigc rename` is a **committing mover that accepts a `<slug>` head naming nothing and rewrites the real document** — and fork 5's stated arms cannot reach it
**[driven]**, `dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc`:
```
jigc rename vision:bogus-head --to "Phantom" --slug bogus-head
→ exit 0
→ "renamed vision:bogus-head -> vision:bogus-head (VISION.md -> VISION.md), repointed 0 referrer(s)"
→ HEAD MOVED (e88b167 → 6195792); git status clean
→ head -6 VISION.md  ⇒  "# Phantom"        (the real committed document's H1, rewritten)
→ jigc doc list       ⇒  vision:vision  VISION.md  managed   (the identity never existed)
```
The contrast in the same corpus is the tell that the guard exists and is asked about the wrong
token: `jigc rename vision:vision --to "Phantom Title" --slug bogus-head` **correctly refuses** at
exit 1 with `blocking · write.identity-change — … a placement singleton's identity is fixed to its
type …`. So `--slug` is adjudicated and the `<slug>` **head** is not.

The charter's fork 5 offers three landing sites: *"parse boundary, write seam, or `finalize` refusing
rather than dropping payloads"* (`decisions-pending.md:101`). **None reaches this door** — `rename`
commits in the same invocation; there is no finalize to refuse at, and the write seam is a `git mv` +
H1 splice, not a doc-payload write. The baseline names `rename --slug` in the subject
(`baseline-ledger.md:150-151`, *6 write doors + 1 read door*) but the charter's fork text does not
carry the committing-mover cell at all. **The Settle owes a fourth arm for the committing doors, or
fork 5 ships a fix whose declared option set demonstrably misses the worst cell in its own class.**

### B5. **`task validate` has no posture row, and `GATE_COVERAGE`'s tiering decision cannot hold one** — M47 Settle Decision 1 forecloses the obvious fix
**[driven]**, `dev/jigc-rig fresh`, one live `quick-fix` task, a planted conflicted `git merge`
(`.git/MERGE_HEAD` present):
```
jigc task validate probe-posture-preview   → exit 3, prints schema-conformance.required-slot-present
                                             and says NOTHING about the posture
jigc task finalize  probe-posture-preview  → exit 1, blocking · repo.operation-in-progress
                                             — "a merge is in progress — the repository is not in a
                                             committable state", route `git merge --abort`
```
`crates/cli/src/gate_coverage.rs:119-215` — **twelve rows, none a posture**; every `Door::Previewed`
row is a *finding the gate computes* (`content-findings`, `carryover`, `owner-artifact-unstaged`,
`changelog-gate`). A posture is a **door refusal taken before the gate runs**, so there is no row
*kind* for it. **[read]**

And the obvious repair is foreclosed by a locked decision: the `base-pin` row carries, in code,
*"excluded from the preview by the M47 Settle, Decision 1 (previewing it would flip `task validate`
0 → 3 on a shipped state, against the pinned exit-code taxonomy)"* (`gate_coverage.rs:185-187`),
re-affirmed at M46 (`DECISIONS.md:4185`, `:4217`, and bound (3) at `:4238` — *"No exit code moves at
the default severity"*). Previewing posture flips `task validate` **0 → non-zero** on a clean task
under a live merge — identically. **Three sub-questions, none decided:** does `GATE_COVERAGE` model
*families* or only *findings*? does the preview flip an exit (re-opening M47 D1) or narrate without
flipping (a fourth `Tier`)? and if it flips, to **1** (the door's code) or **3** (the taxonomy's
task-scope-gate exit)? The *"previews what finalize gates on"* claim rides **eight** surfaces
(`baseline-posture` L1), so silence here ships a known-false claim on eight surfaces.

### B6. **Nothing owns dispatch order**, and `refuse_on_posture` has a *silent* bypass that falsifies M51 D2's claim
`crates/cli/src/cli.rs:540` — inside `refuse_on_posture`:
```rust
let cwd = std::env::current_dir().ok()?;          // ← `None` ⇒ the whole posture family is skipped
let repo_root = crate::repo::discover_repo_root(&cwd)?;
```
**[read]** A `current_dir()` fault returns `None` from the guard, so **every** commit-on-behalf door
proceeds with the posture family unasked. The baseline names the same seam as collateral of DEFECT D
(`baseline-contracts §2`, *"`refuse_on_posture` reads `current_dir().ok()?` and silently skips the
whole M51 posture family on that fault"*), and counts the family at **24 sites + LD-1's malformed
`packs.yaml`** = 26 pre-dispatch failure points, **2 of which bypass or corrupt both reject funnels**.

Two decisions are missing:
- **(a) a dispatch-order contract.** DEFECT D's fix wants one pre-dispatch funnel. The baseline
  measured it is armable — `--format` is fully parsed and validated before any `current_dir()`, and
  `Cli::dispatch()` (`cli.rs:568`) already holds `self.format` (`baseline-contracts §2`, driven).
  But *what is adjudicated before what* — format parse → pack load → cwd → repo discovery → posture
  → verb — is stated in no artifact. Without it the fix is a patch at one site and the next
  pre-dispatch fault repeats it.
- **(b) M51 D2's claim must be re-scoped or the bypass closed.** `design/worked-examples.md:3445`
  declares, as flow 52's honest bound, *"the posture family closes over **three of four** driven
  members — detached · unborn · operation-in-progress — with the **`GIT_DIR` redirect declared out**"*.
  The cwd-fault bypass is a **second** un-declared escape from the same family, in no bound.
  Under this wave's own anti-false-completeness purpose, that is not a code nit; it is the claim
  being wrong.

### B7. Fork 2's **consent arm is unavailable under the charter's own boundary**, and nobody has said so
The charter poses fork 2 as *"does the posture family gain cherry-pick and revert members, **or** does
`--carry-staged` refuse under any in-progress operation?"* (`decisions-pending.md:96`). The baseline's
measured constraint kills a third reading before it is offered: *"a `git commit` under
`CHERRY_PICK_HEAD`/`REVERT_HEAD`/`MERGE_HEAD` **is** that operation's conclusion to git … no `git
commit` argument makes it not conclude. A consent can only mean *consent to conclude the user's
operation under jigc's subject*, and today `--force`/`--carry-staged`/`--dry-run` are leaf-keyed and
carry no posture consent — **any consent flag is a new one**"* (`baseline-ledger.md:99-104`). **[read]**

A new flag is **new capability**, which the wave's boundary forbids (`decisions-pending.md:30`). And
M51 already withdrew `--force` for this family on the record: *"Posture refusals carry NO override
(S3) … `--force` at `jigc setup` carries exactly one meaning — D3's consent"* (settle amendment §3),
restated in `design/validation.md:653` and `design/finalize.md:31`. **So the only admissible arm is
refuse.** Say it in those words; otherwise a decomposition will reach for a consent flag and halt.

### B8. Fork 2 owes a **per-operation route table**, and the route rule in a locked doc forbids the cheap answer
`design/validation.md:663` registers `repo.operation-in-progress` with a **closed** marker set and a
**closed** route rule: *"(`MERGE_HEAD` · `rebase-merge`/`rebase-apply` · `BISECT_LOG`) … Route:
**`Human`, naming the command that concludes *this* operation** — `git merge --abort` /
`git rebase --abort` / `git bisect reset`, **never a menu of three**"*. **[read]** The baseline's
state axis is **8 un-concluded operations + 2 HEAD postures + a conflicted index with no marker**
(`baseline-ledger.md:88-98`), including two operations that **write no marker at all** (squash merge,
conflicted stash-pop) and a `REVERT_HEAD` git's own `determine_whence()` never reads.

So the fix needs a decision per state cell: (i) the **discriminator** (`rebase-apply/applying` ⇒ `am`,
`rebase-apply/onto` ⇒ rebase — driven by the baseline, and a property of **git 2.54.0** only,
`baseline-ledger.md:356-359`); (ii) the **correct abort argv per operation**, which is the very thing
D2's defect was (*jigc names a rebase and routes at `git rebase --abort`, which git refuses*) — a
route table built from a read rather than driven repeats it; and (iii) **what jigc does about a state
with no marker**, where refusing means detecting a conflicted index and *guessing* the operation, and
not refusing means a squash merge is still swallowed whole. **None of the three is in the charter.**

### B9. `validation.md:723`'s *"one predicate, the door and the store surface cannot diverge again"* has **no owner for the third door**
`design/validation.md:723` (the `ingest.unaddressable-identity` registration) states: *"**One
predicate, not two agreeing ones**: the refusal asks the same function that mints the identity every
other consumer reads, **so the door and the store surface cannot diverge again**"* — scoped to
`cli::ingest::classify_row`. **[read]** The baseline drove `jigc relocate adr --from notes` over
`notes/My Note.md` landing `adr:My Note` **managed at exit 0**, with `doc show` refusing it and the
route looping (`baseline-tokens §4 defect 5`). `relocate` asks no such predicate.

**Missing decision:** does `ingest.unaddressable-identity`'s predicate bind **every door that lands a
managed identity** (`relocate`, `migrate-corpus`'s relocation arm, `rename`), or only `ingest`? The
locked sentence's universal (*"cannot diverge again"*) is already false; the Settle either widens the
predicate to the door class or strikes the universal with its falsifying datum. A1-D2 (tier 0) and
A1-D3 (tier 1) both sit on `relocate` and neither fork names this.

### B10. Fork 3's rider — `task finalize` joining a destroying registry — collides with the `--force` single-consent mold, and no flag is available
**[driven]** `jigc task finalize --help` carries `--approve`, `--format`, `--dry-run`,
`--carry-staged`, `-h` — and **no `--force`**. `jigc milestone finalize --help` carries only
`--carry-staged`, `--format`, `-h`. `DESTROYING_DOORS` (`crates/cli/src/milestone.rs:2625`) has four
members; `FINALIZE_DOOR` is the one whose `code: None` — *"the one that never refuses"*
(`milestone.rs:2610-2619`) — because the table's subject was re-derived from *refusal* to
*destruction*. **[read]**

So if `jigc task finalize` joins, it can only take the **narrating** arm (`code: None`), since the
refusing arm needs a consent flag and a new flag is new capability. That makes the loss **visible,
not prevented** — the identical declared cost M46 took, on measured evidence, for the `--ignored`
axis (`DECISIONS.md:4434`, `:4442`, `:3914`). **That is a decision with a permanent cost, and the
charter files it as a rider.** It must be taken in words, with the cost restated as permanent and a
re-opening condition, exactly as M46's was.

### B11. Axis-7 `C-2`'s **declared residual 2 rests on a premise the baseline falsified**, and the charter asks for a restatement
The charter says this row *"owes a DISPOSITION, not a fix … The Settle either re-takes the residual
with that amplifier named, or restates it as still-declared with the amplifier written into the
declaration"* (`decisions-pending.md:88`). The baseline makes *restate* the harder arm: **"C-2 is not
one fixed cell: 2 doctypes at the pack default, **5 under `placement-root: .`** — and under that knob
with the pack dropped, **`jigc validate` exits 0 "validates clean" over three stamped orphans**"**
(`baseline-ledger.md:203-205`, `freeze §4 L-5`). **[read]** A residual declared as a bounded two-cell
gap is, under a knob jigc itself ships and M49 built, a **total false green over the whole placement
population**. The declaration's *basis has changed*; restating it unchanged would be a
basis-unchanged claim over a falsified basis — the shape `DECISIONS.md` requires be struck with its
datum, never carried.

### B12. **Nothing owns "project posture"** — LD-3: 6 of 9 `milestone` doors run at exit 0 over a repo with no `jigc setup`
`baseline-contracts §4 LD-3`: **6 of 9 `milestone` doors run at exit 0 over a repo with no
`jigc setup`**, including `provision` (which creates a real worktree) and `finalize` (exit 3, a real
adjudication), while **21 of 47 leaves refuse that state**; only `milestone execute` refuses. **[read]**
M51's posture family is *git* postures (`BEHALF_DOORS` × detached/unborn/operation-in-progress);
`design/finalize.md:31` and `validation.md:657` enumerate exactly those. **No decision anywhere says
whether "this project is not set up" is a posture a door adjudicates.** This is in no charter fork
and in no tier, and it is a door creating a worktree in a repo jigc does not manage.

---

## §2 · BLOCKING — decisions whose basis the baseline falsified

### S1. M47's `RecordPreImage` door enumeration is **four; the population is six** — and two of the two extra are `--force` doors that clobber silently
`design/finalize.md`'s M47 rollback row enumerates **four** record-only doors. The baseline drove
`task discard <sub> --force` and `milestone discard --force` as a **fifth and sixth** `RecordPreImage`
caller, clobbering silently, plus `RecordFlipGuard` clobbering identically at `squash: false`
(`baseline-ledger.md:282-284`, rollback L2/L3/L4). **[read]** The enumeration is the basis of the
row's completeness claim; it is stale and must be struck with the datum, not extended.

### S2. `design/finalize.md:174`'s rollback row **claims a behaviour the binary does not have**
The row states, verbatim: *"On failure, restore each pre-existing destination from its captured
bytes, delete only a copy with no pre-promote content, **rewrite each retire-deleted original**;
working area intact"*. **[read]** Axis-4 `DEFECT 2` drove that **retire never puts the retired
original back**, and that neither promote's nor retire's loss is named by any finding
(`decisions-pending.md:44`). A locked doc asserting a rollback leg the code does not run is exactly
razor leg 1 — but it also means fork 1's *cheap* arm ("keep the declared behaviour") is unavailable:
there is no declared behaviour to keep, only a declared behaviour to *build*. M51 D4's advocate made
this identical argument about `:178` (*"cheap is not keeping a declared decision; it is declaring a
new one"*) — it applies again, one row up, and the charter does not carry it.

### S3. `worked-examples.md:3445`'s *"three of four driven members"* bound is **a member count where the failure is a state count**
The bound reads: *"the posture family closes over **three of four** driven members — detached ·
unborn · operation-in-progress — with the `GIT_DIR` redirect declared out"*. **[read]** The baseline's
driven state axis is 8 un-concluded operations + 2 HEAD postures + a marker-less conflicted index
(`baseline-ledger.md:88-98`), of which the *operation-in-progress* member covers **3 operations over
4 markers**. So "three of four" is true of the **family's own vocabulary** and false of the
**repository states a caller can reach** — which is what the bound reads as. Plus B6's cwd bypass.
The bound must be restated over states, with the falsifying datum, in the same wave that widens it.

### S4. The charter's fork-4 premise — *"deciding `from` may move a `placement`/`location` home … price the migration"* — is **falsified**
`decisions-pending.md:99` warns fork 4 against the zero-schema-hash rule. The baseline: *"**Closing
D-2 moves no schema hash, bumps no `schema-version`, migrates no corpus**. The prior home is already
recorded in full … The fix is entirely `candidate_docs`' enumeration"* (`baseline-ledger.md:176-181`).
Confirmed by reading `crates/cli/src/migrate_corpus.rs:2075-2131`: the `to.location` branch loads no
snapshot at all, and the placement branch unions prior homes via `.filter_map(|prior| prior.location)`
— **prior `placement` is never consulted**, which is why `L→L`, `P→P` and `P→L` are all uncovered.
**[read]** The fork's hardest-looking constraint evaporates; what survives are two *different*
constraints the charter does not name (see S5 and B13 below).

### S5. Fork 4's real constraints are unnamed: `DoctypeMigration` cannot resolve a prior `placement.file`, and the walk widens the collision surface
`baseline-ledger.md:182-185`: a prior `placement.file` with a leading component needs the
`placement-root` knob, which `DoctypeMigration` (`migrate_corpus.rs:76-81`) **does not carry** (it
carries `docs_root` only); and widening the walk widens the destination-collision surface
`destination_collision_route` (`:1982`) exists for. **[read]** Neither is in the fork. A third,
unnamed: once the walk finds below-version instances that were previously invisible, an adopter's
`jigc validate` **flips exit 0 → 1** on a corpus that was green yesterday. That is the same
intended-exit-flip decision M42 took explicitly for `schema-conformance.schema-version-current`; it
must be taken here in the same words, not discovered at decompose.

### S6. M46's `--ignored` refusal transfers to the task area **and harder** — so fork 3's stated robust arm is refused by its own precedent
`DECISIONS.md:4450` records the refusal's measured ground: *"a refusal on that axis fires on the
ordinary fan-out **success** path and trains `--force` into reflex … the loss is **visible, not
prevented**; re-opening condition: an adopter reports work lost to a narrated ignored-path teardown."*
The baseline measured the transfer: jigc itself writes `base.json` · `intent` · `workflow` ·
`staged-snapshot.json` · `docs/provenance.json` into **every** task area from `jigc start` onward, so
*"any non-staged-`.md` byte blocks"* fires on **100 % of tasks at 100 % of doors**
(`baseline-ledger.md:132-138`). **[read]** So fork 3's robust arm **as the charter words it**
(*"widen the guard's subject to every byte under the task's area"*, `decisions-pending.md:97`) is not
available; only the complement-of-the-writer-set formulation is — which is B3's unowned registry.
The charter poses a fork whose robust arm does not exist as stated.

### S7. M50's *`--force` is the single consent* rule and M51's *posture refusals carry no override* are two rules; the baseline shows a cell where neither answers
M50/M48's destroying-door mold: `--force` is the single consent (`DECISIONS.md:2458`, `:2393`,
`:2465`). M51 amendment §3: posture refusals carry **no** override. **[read]** Fork 3's widened
task-area subject lands at `task finalize` and `milestone finalize`, **neither of which has a
`--force`** (**[driven]**, §B10). So the shipped consent vocabulary has a hole exactly where the
wave's widest subject lands. Decide it; do not inherit it.

---

## §3 · BLOCKING — prior-art contradictions between locked statements

> Per the brief, a disagreement between two locked statements is a **fork**, not a free choice.

### P1. `corpus-migration.md:68` + `:299` say the corpus walk keys on **`from`**; `migrate_corpus.rs:2075-2131` keys on **`to`** — and `:299` records it as *proven acceptance*
`design/corpus-migration.md:68`: *"The corpus walk keys on the **`from`** home (where the instances
actually sit), not the empty `to`."* `:299` (the *Relocation safety (M38 `relocated`)* acceptance
row): *"the corpus walk enumerates the **`from`** home"* — stated in the **Acceptance** column, i.e.
as a property something proves. **[read]** `candidate_docs` (`migrate_corpus.rs:2075`) returns
`to.location`-keyed pairs in its first branch and, in the placement branch, unions prior **`location`**
homes only — never a prior `placement` (`:2104` `.filter_map(|prior| prior.location)`). **[read]**
The doc-comment two lines above (`:2072-2074`) even asserts *"the walk is the union of all of them"*,
true only for the one cell M38 shipped (`L→P`).

**This settles fork 4 rather than leaving it open:** the direction is `from`, decided doc-side at
M38, and the code is the violation. What the Settle actually owes is S5's two mechanism constraints —
and a check on whether any test claims `:299`'s acceptance (a green suite over a false claim is the
`design/doc-read-surface.md` *"no shipped doctype declares a multi-slot repeatable"* failure again,
which is what cost M49 its centrepiece).

### P2. `storage.md`'s single-writer model is cited to **refuse** a race fix and to **build** one, in the same settle record
`design/storage.md:300-310` (*Concurrent writers*) scopes the concurrency carve-out to **the three
shared `.jigc/` caches** and says nothing about worktree bytes or about the other workbench files.
**[read]** M51 amendment §2 refuses Codex-1's descriptor-held traversal *"on the single-writer model
([storage.md] → Concurrent writers **bounds the one exception, and it is the shared `.jigc/` caches,
not the worktree**)"*. M51 D4 + amendment §6 **build** a compare-and-swap for `.jigc/.gitignore` and
`.jigc/version` precisely because *"an unconditional restore can destroy a concurrent edit"*. **[read]**
Both are locked, both are M51, and they read the same silence oppositely.

Fork 1 sits exactly on that fault line — it asks to extend CAS to nine populations, four of which are
gitignored workbench paths outside the three named caches. **The Settle must pick one reading of the
concurrency model and state it**, or it will justify each of the nine populations by whichever half
is convenient. (Note the asymmetry it must survive: if the worktree is single-writer, `.jigc/.gitignore`'s
CAS is gold-plating; if it is not, the descriptor-held refusal's warrant is gone.)

### P3. `reconciliation.md:186` — *"No silent discard, ever. Every block surfaces; every absorb surfaces"* — vs two driven absorbs
`design/reconciliation.md:186` is a **What reconciliation does NOT do** bullet, i.e. an invariant.
**[read]** Axis-7 `D-1`: `jigc ingest` durably absorbs a blocking `file-state.hash-matches` and
surfaces nothing (`"finding": null, "adopted": true`, later `validate` green). `baseline-freeze §4
L-1`: `jigc rename` over a drifted committed adr commits, re-keys the baseline to the post-drift
hash, and the blocking finding is **gone with no absorb line**. The baseline also records that the
**classifier is correct** and *"only the surface is missing"* — so the fork is narrow (which surface,
at which of the two doors), but the invariant's breach must be named as an invariant breach, not a
missing print.

### P4. `project-setup.md:160`'s outcome-keyed narration rule is declared *"complete over every cause by construction"* — and the **ack** beside it is a fixed string
`design/project-setup.md:160` states the narration is keyed on the removal's **outcome** — *"read
before the removal runs, printed after it, and listing exactly the items whose bytes are actually
gone … that binds every one of the **four** narrate-then-remove pairings … complete over every cause
by construction rather than over a list of shapes."* **[read]** Axis-3 `D-2`: `jigc milestone discard
--force` acks **`workbench removed`** at exit 0 over a teardown that failed, with two worktrees still
on disk and two `could not remove …` warnings in the same run; `baseline-destroying §3` pins the
class at **size 1** (`milestone.rs:3298`, one fixed string) and confirms the *narration* beside it is
correctly outcome-keyed. So the doc's universal is true of the narration and false of the **ack**,
which it does not distinguish. The repair is one string; the **decision** is whether
`project-setup.md:160`'s "by construction" claim extends to acks — i.e. whether the outcome-keying
rule has an axis nobody enumerated (every door that *acks* a removal, not every door that *narrates*
one). One string fixed without that axis is the reported-repro fix the wave's own standing rule bans.

### P5. `finalize.md:31` / `validation.md:663` enumerate the operation-in-progress markers **closed**; fork 2 opens them
Both locked registrations name exactly `MERGE_HEAD · rebase-merge/rebase-apply · BISECT_LOG` and a
three-command route menu. **[read]** Widening to cherry-pick/revert/am/sequencer/squash-merge/
stash-pop revises **three** locked statements in lockstep (`finalize.md:31`, `validation.md:663`,
`worked-examples.md:3445`) plus the `repo_posture.rs` declared-out row. Not a contradiction between
docs — a contradiction between the docs and what the fork proposes — but it means fork 2's cost is
*four doc revisions + a route table + a git-version bound*, which the charter's one-line framing
hides.

---

## §4 · Decisions the wave must **NOT** re-open (and the one that is *not* settled)

1. **The v1 freeze.** Zero schema-hash movement is the wave's default (`decisions-pending.md:91`).
   The baseline **discharges the one fork that looked like it needed a bump**: fork 4 moves no hash
   (S4). So the freeze is not in tension with any tier-0 fix, and any plan sentence proposing a bump
   should be read as a mis-scoped fix. The manifest CI fence's escape (`Manifest-Repin: <entity>`)
   is not an M52 instrument.
2. **`GIT_DIR` declared out** (M51 D2). Its rationale — `repo.rs:1-16`'s no-shell-out walk-up, *"so a
   fake `.git` can never walk up to, and bind against, a real ancestor repo"*, ~15 fixtures depending
   on it — is **not falsified by the baseline**; `dirname(common-dir) != toplevel` is explicitly *not*
   a discriminator (a legitimate linked worktree has the same asymmetry). **But its *scope* is
   recorded reopenable**, and the wave's done-picture (*"every posture a caller can reach answers
   with a code"*) does not hold over it. Do not reopen the mechanism; **do** decide whether the
   wave's claim is scoped in writing or the residual is re-taken. Silence here is the false-
   completeness shape the wave exists to correct. (See hunt (b).)
3. **The `Plain` family ships as a classified registry, not one shared predicate** (M51 D1 + amendment
   §2, and the honest bound at `worked-examples.md:3445`). Fork 5's `<slug>`-head question is a
   *different* token family; it must not be folded into `PATH_ARG_OCCURRENCES` as if it were the same
   rule.
4. **`store.malformed-slug` stays outside `Address::parse`** (M50, measured: it reddens
   `doc_read_surface`'s law-1 fence because `doc schema --format json` advertises type-level addresses
   carrying `<slug>`). Fork 5's head check is a **resolution** question, not a grammar one — the
   driven B4 token is a *well-formed slug naming nothing*. Re-posing it as "put it in `Address::parse`"
   re-opens a measured refusal.
5. **EC-9 / the publishing floor** stays on its trigger (M51 D6), which fires *after* the 1.0.0 call.
6. **Posture refusals carry no override** (M51 amendment §3) — see B7.
7. **The pre-pin window is OPEN, and M52 is the last wave before the call.** **[read]**
   `design/command-output-contract.md:476-488` authorizes, *while the window is open*: an **additive**
   key; the **removal** of an **undeclared** key (*"never one it did [declare]"*); and a **reshape of
   a door's declared arm** (M51 Inc 6's 25-door precedent), each *"declared in the same motion an
   addition would be"*. From the pin: *"a removal or a reshape is `2.0`"* (`:482`, `:488`). The
   charter states this correctly for fork 6 — **and nowhere else.** That is a gap in its own right
   (M2 below).

---

## §5 · Disposition of the 7 open leads against the baseline

| # | lead | disposition |
|---|---|---|
| 1 | axis-1 `C10`'s fence half (all `Route` constructors invoke the backticked-span fence in debug builds) | **TRIGGER MAY HAVE ALREADY FIRED, unnoticed.** Its stated trigger is *"the first wave that adds or moves a `Route` constructor"*. The M51 **completion audit** — landing *after* the per-axis review recorded this lead — *"added **two** fences, a **span** fence on **all three `Route` constructors**"* (CLAUDE.md M51 record; `engine::finding::shell_token`). So the constructors moved inside M51 and the lead was not re-posed. M52's tier 1 touches 17 route repairs and, per M51's audit, **22 route producers across four carriers**. **Recommend: re-pose at the Settle, in a debug-posture seam suite** (M43's declared proof split is the named precedent). **[read]** |
| 2 | axis-1 `O5` (tampered `.jigc/tasks/<id>/source-path`) | **TRIGGER LIKELY FIRES.** Stated trigger: *"the first wave that takes **workbench-state truth** as a subject."* `baseline-destroying §4 L-3` drove `staged_doc_ids` never asking `is_file()` — a **directory** `docs/fake:thing.md` is a staged doc at three doors and reads `managed` on the pinned `doc list --task`. That is jigc trusting a hand-shaped workbench entry, which is O5's own class. Fork 3 additionally makes the task area a subject. **Recommend: dispose explicitly** — admit with L-3, or carry with the trigger restated over the *measured* population. |
| 3 | axis-2 the fan-out `git merge --ff-only` re-probe (`task.rs:5614-5619`) | **DISCHARGED — built + proven.** Confirmed at `baseline-ledger.md:110-112` (`posture §3`): *"a bisect planted between the door and the ff was caught, `--ff-only` never ran, the index rolled back."* New fact carried with it: the dedicated worktree's posture is **per-worktree**, so the boundary commit **is created and orphaned** before the live refusal — that is a *new* open item, not part of the discharge, and it belongs in fork 2's scope. **[read]** |
| 4 | axis-6 `C5` (manifest-less project-local pack ships an off-catalog workflow with `router_hidden: null`) | **STAYS OPEN.** Trigger (*"a project-local pack becomes a supported adopter surface"*) does not fire; M52 mints no pack surface. Note the bound that rides it: `baseline-contracts §5` records **no project-pack composition was driven** at the baseline, so the wave should not claim anything over that axis. |
| 5 | axis-6 the JSON arm of every composed door carries **no** task-state lines | **TRIGGER LIKELY FIRES.** Stated trigger: *"any wave that **moves a task-state line**."* Tier-0 `A6-1` (a sub-task's composed surface byte-identical inside and outside its worktree) and tier-2 `A6-2`/`A6-3` all edit composed surfaces, and `baseline-surfaces §3` widens A6-1 to **2 doors × 4 states, all one byte-string**; discriminating them is a task-state line by definition. **Recommend: dispose at the Settle** — the declaration is re-posed, not inherited, or the text and JSON arms diverge by an edit nobody priced. |
| 6a | axis-6 driver lead: `task bind`'s two refusals carry no code and no route (bare `anyhow`) | **TRIGGER FIRES.** Stated trigger: *"the moment a wave sweeps the route floor's producers again — M50 left **60** countable in `UNSWEPT_PRODUCERS` and that table is the trigger's home."* M52 tier 1 is a 17-row route/posture dead-end sweep, and M51's audit already moved `migrate.rs` out of `UNSWEPT_PRODUCERS` into `GUARDED_SRC`. **Admit or refuse with a citation; do not inherit.** |
| 6b | axis-6 driver lead: `migrate` mints silently (a third work-starting door) | **TRIGGER MAY FIRE.** Re-posed *"at the wave that next touches the `MINT_DOORS` enumeration."* Fork 1's `unwind_mint` population (`baseline` §2 #1) and fork 3's task-area subject both sit on the mint path. Dispose. |
| 6c | axis-6 driver lead: `describe --commands --format json` carries no argv | **STAYS OPEN** — declared non-contractual (`introspection.md` → *Non-contractual by design*, re-declared at `command-output-contract.md:177`); trigger is *"the wave that takes `describe` into the pinned contract"*, which M52 is not. |
| 6d | axis-6 driver lead: the none-provisioned advisory silence is by design | **STAYS OPEN** — but check against A6-1, whose repro needs `git checkout main` (`baseline-surfaces §3`) and whose never-provisioned refusal is reused **verbatim** in the provisioned-wrong-checkout state, *"whose leading route `jigc milestone provision …` is a no-op in half the states that print it."* If A6-1's fix discriminates the states, this lead's premise moves. |

---

## §6 · The three cross-cutting hunts

### (a) Cheap-vs-robust, judged against the vision's committed trajectory

The trajectory is short and hard: **M52 is the last wave before the 1.0.0 call** (charter close:
*"Then the 1.0.0 call, which is the human's"*). That makes the one-way-door leg unusually live: every
wire-shape repair **not** taken here becomes a `2.0` act (`command-output-contract.md:482`, `:488`).

| fork | cheap cut | robust cut | verdict |
|---|---|---|---|
| **1 · rollbacks** | per-site guards; restore-what-we-captured | one captured-pre-image + CAS primitive with a stated per-population disposition (CAS · door guard · `Exempt(reason)`), incl. directories and gitignored homes | **`fork · cheap-vs-robust`. Robust, but not the charter's binary version.** The baseline's own evidence refuses *"make them all CAS"* (two populations already correct under a door guard). The debt in the cheap cut is not reversibility — it is that *"restore what we captured"* is **the exact framing this class has re-bought three times** (`decisions-pending.md:94` says so). One-way-door tell: **no** (no pin, no format). Declared-surface hole tell: **yes** — `finalize.md:174` declares a leg the code does not run (S2). |
| **2 · posture** | `--carry-staged` refuses under any in-progress operation | the family gains the missing operations, with a per-operation discriminator + route table | **Robust.** The cheap cut is a **regression-in-disguise**: it leaves `rename` concluding a cherry-pick and a revert concluded at exit 0 by **every** commit door (`baseline` §2 #8) while the wave *claims* the posture class closed. One-way door: **yes** — `finalize.md:31`/`validation.md:663` are the 1.0 statement of the family; shipping the pin over a 3-marker family makes widening it a doc-and-behaviour change after the pin. |
| **3 · destroying subject** | keep the `.md`-suffix predicate, add narration | subject = complement of jigc's own writer set, code-side derivable, at all five doors + the registry question | **Robust — but the charter's wording of it is unavailable (S6).** Cheap cut's cost: an ordinary `NOTES.md` at a task-area root dies at exit 0 (`baseline-ledger.md:50`) — a **declared-surface hole** in `project-setup.md:160`'s "refuses what it cannot prove is disposable, or names every byte it destroys; silence is the one arm no door may take" (`DECISIONS.md:4432`). |
| **4 · corpus walk key** | leave `to`, document the gap | walk `from` over every prior home of **both** kinds | **Not a fork — settled `from` doc-side (P1).** The cheap cut is a *known* total false green (`validate` green, `doc list` says *no committed docs*, over a below-version doc on disk). One-way door: **yes** — an adopter corpus that silently stays below-version across the 1.0 pin has no later run that will find it. |
| **5 · `<slug>` head** | guard the five write doors A1-D1 names | guard the resolution boundary for **all** identity-taking doors incl. the committing movers (`rename`, `relocate`, `milestone add-from-spec`) | **`fork · cheap-vs-robust`, and the cheap cut is a *demonstrated* hole**: B4 drives the worst cell (a committing door rewriting the real doc from an identity naming nothing) at a door the cheap cut does not touch. One-way door: **partly** — `doc set-slot vision:bogus-head#thesis --format json` returns `{"findings":[],"target":{"slug":"bogus-head"}}` on the **1.0-pinned** ack (`baseline-tokens §3`), so shipping the pin blesses a target key that names nothing. |
| **6 · third reject envelope** | declare a third arm | move both doors onto a declared arm (`render.rs:3383`'s one line, 2 call sites, 1 unit test — `baseline-contracts §3`) | **Robust, and cheap in cost.** Declaring a third arm is the *reversible-looking* option that is actually the sticky one: a declared arm is pinned for the life of `1.x`. Correction the Settle must carry: **`doc schema`'s `contract-version` is not involved** — the envelope version is `engine::result::SCHEMA_VERSION = 3` (`result.rs:27`), so the charter's stated price for this arm is wrong (`baseline-ledger.md:219-221`). |

**M2 (a missing decision the hunt surfaces).** The window's closing is a **one-time** event and the
charter prices it for fork 6 alone. The other pinned-surface contradictions on the table — `DEFECT B`'s
undeclared second array (and `baseline-contracts §3`'s widening to **17 addressable cells / 7 root
shapes**), `DEFECT C`'s three `ArmOutcome::Success` rows that ship non-zero by design, `DEFECT 1`'s
finding that reaches the machine surface **nowhere**, `LD-2`'s two pinned contracts disagreeing on
`milestone-record.base`'s type (`doc schema` says `string`; `doc show …#meta/base` returns
`{sha, short}`), and B4's bogus `target.slug` — **all** become `2.0` acts the day after. **One
decision is owed over that set, not six**: which pinned-surface contradictions close before the pin,
and which are *declared* (the reversible direction) with their reason. Taking them one fork at a time
is how a subset ships and the complement is blessed by omission — which is the precise failure
`command-output-contract.md:486` was written to prevent.

### (b) Foreclosed-by-doc

1. **`fork · foreclosed-by-doc` — M47 Settle Decision 1 forecloses the natural fix to the posture
   preview gap (B5).** Rationale, quoted from the code that carries it: *"excluded from the preview by
   the M47 Settle, Decision 1 (previewing it would flip `task validate` 0 → 3 on a shipped state,
   against the pinned exit-code taxonomy)"* (`gate_coverage.rs:185-187`), re-affirmed at
   `DECISIONS.md:4238` bound (3). **Predicate: an exit flip at `task validate` is inadmissible.**
   Does the baseline falsify it? **No** — the predicate holds. But the **basis has widened**: M50
   *did* flip `task validate` 0 → non-zero on a shipped state, deliberately and in writing (`jigc
   task validate ""`, `DECISIONS.md:2621` bound (iii): *"The exit-0 → non-zero flip … is intended …
   and is not a licence to move any other door's exit code"*). So the taxonomy tolerates a flip when
   the state was never legitimate. **A posture-under-a-live-merge is arguably that same kind of
   state.** This is a candidate **revise** — the better path (preview the posture) still honours M47
   D1's rationale if the flip is declared, as M50's was. Flag for the human; I settle nothing.
2. **`fork · foreclosed-by-doc` — M51 D2's `GIT_DIR` declared-out vs *this wave's own claim*.**
   Rationale quoted: `repo.rs:1-16`'s no-shell-out walk-up, *"so a fake `.git` can never walk up to,
   and bind against, a real ancestor repo"*; ~15 fixtures depend on it; `dirname(common-dir) !=
   toplevel` is explicitly **not** a discriminator. **The rationale is not falsified.** What has
   changed is the *claim it sits under*: M52's done-picture says *every* posture a caller can reach
   answers with a code. Under a `GIT_DIR` redirect, `milestone create` writes the record into repo A
   and lands the commit in repo B at exit 0 (M51's own baseline). So the wave must either **scope its
   claim in writing** or re-take the residual. **Stating one half and not the other is the
   false-completeness shape this wave exists to correct** — M51's amendment §3 says that sentence
   itself; it applies to M52's charter now.
3. **`fork · foreclosed-by-doc` — M46's `--ignored` refusal forecloses fork 3's robust arm as worded
   (S6).** Rationale quoted: *"a refusal on that axis fires on the ordinary fan-out **success** path
   and trains `--force` into reflex, **strictly worse than no guard**"* (`DECISIONS.md:3914`).
   Basis **unchanged and strengthened** — the baseline measured the same failure at 100 % of tasks ×
   100 % of doors. The better path (the writer-set complement) **honours** the rationale; it just
   needs B3's registry to exist. Not a conflict — a *mechanism* the charter left out.
4. **`fork · foreclosed-by-doc` — M48's own rule already answers half of fork 1's rider.** Fork 1
   asks *"whether `finalize.rollback-conflict` reaches the machine surface at all"*
   (`decisions-pending.md:95`). `command-output-contract.md:494` states the locked rule: *"a value the
   human/agent text already prints, but the `--format json` envelope withholds, **is a gap**"*
   (M48, `DECISIONS.md` → 2026-08-13). So *whether* is answered; only *how* is open, and the
   constraint is real — the reject envelope is single-key, so a second document on stderr breaks
   stream discipline (`DEFECT 1`) — which makes the answer a **reshape**, admissible only while the
   window is open (§4.7). The charter frames a settled question as free.
5. **Consistent prior art, not a conflict, worth carrying to fork 6:** `command-output-contract.md:181`
   refused reshaping `task list`'s top-level array — *"it is a breaking change to the surface a driver
   reads on every loop, the 1.0 pin is the next gate, and D5's asymmetry runs the other way here:
   declaring the array costs a paragraph, reshaping it costs every driver."* **`DEFECT B` is the same
   shape one door over** and should be argued against that paragraph explicitly rather than
   re-derived.

### (c) Prior-art-reconciled — every disagreeing pair of locked statements I found

Grepped across `design/` and `DECISIONS.md` for each class the wave touches
(`cherry-pick|revert|REVERT_HEAD`, `operation-in-progress`, `rollback|pre-image`, `corpus walk`,
`unaddressable`, `silent discard`, `Concurrent writers`, `--ignored`, `single consent`,
`DESTROYING_DOORS`, `GATE_COVERAGE`):

- **P1** `corpus-migration.md:68` + `:299` (**`from`**) vs `migrate_corpus.rs:2075-2131` (**`to`**) —
  and `:299` asserts it as *acceptance*. **Settled doc-side; the code is the violation.** BLOCKING.
- **P2** `storage.md:300-310` single-writer model read **two ways inside M51's own settle record**
  (amendment §2 refuses a race fix citing it; D4/§6 builds one). **BLOCKING for fork 1.**
- **P3** `reconciliation.md:186` *"No silent discard, ever"* vs `ingest`'s absorb (axis-7 `D-1`) and
  `rename`'s baseline re-key (`baseline-freeze §4 L-1`). Invariant vs behaviour.
- **P4** `project-setup.md:160` *"complete over every cause by construction"* (four narrate-then-remove
  pairings) vs `milestone.rs:3298`'s fixed ack string — the rule has an **ack axis** nobody enumerated.
- **P5** `finalize.md:31` + `validation.md:663`'s **closed** marker enumeration + `never a menu of
  three` route rule vs fork 2's widening vs `worked-examples.md:3445`'s "three of four" bound — three
  locked statements moving together, plus the git-2.54-only discriminator bound.
- **P6** `validation.md:723` *"the door and the store surface cannot diverge again"* vs `relocate`
  landing `adr:My Note` managed at exit 0 (B9).
- **P7** `design/finalize.md:174`'s *"rewrite each retire-deleted original"* vs axis-4 `DEFECT 2`
  (retire never restores). A locked doc asserting an unbuilt leg (S2).
- **P8** `design/finalize.md`'s M47 `RecordPreImage` row enumerates **four** doors; the driven
  population is **six** (S1).
- **P9** `corpus-migration.md:90` *"never a silent already-current"* vs the driven `Relocated`-only
  bump, where the doc lands in **no bucket at all** (`baseline-ledger.md:186-188`) — a third
  statement in the same doc as P1, falsified by the same defect.

---

## §7 · Major (not blocking) and minor

- **M3 · the invocation-log identity gap is a declared M50 bound the wave will widen.** M50 recorded
  that the malformed-id guard's identity *"reaches the printed surface and not yet the invocation
  log"* (CLAUDE.md M50 record), and M50's audit MEDIUM swept **31 production sites** dropping carried
  findings. Every new code M52 mints (posture members, the destroying-subject code, the identity
  check) joins that funnel. **Decide whether new codes must land in the log at mint**, or the bound
  grows silently by one per fix.
- **M4 · `--dry-run` and `task validate` are two preview surfaces with one claim.** `baseline-surfaces
  §3` records the dry-run text arm also drops `changelog-recording.gate-granted-unused` (a different
  family) and that D-1's producer half is structurally **9 `DocAck` arms, 1 driven**. Fork-adjacent to
  B5; decide whether the preview-coverage claim is one rule over both surfaces.
- **M5 · `staged_doc_ids` never asks `is_file()`** (`baseline-destroying §4 L-3`): a directory
  `docs/fake:thing.md` reads `managed` on the **pinned** `doc list --task`. That is a pinned-surface
  lie, so it also belongs in M2's one-decision set.
- **M6 · the `ENTRIES`-name shape check** (`baseline` §2 #6): `workbench_paths` excludes on
  `file_name() == prefix` — *a name match with no shape check* — **against its own doc-comment
  recording M49's opposite lesson** (`fanout_worktree_paths`' `is_dir()`, *a claim about shape where
  the door's question is about bytes*, `DECISIONS.md:3772`). The lesson is written **in the file that
  violates it**. Minor as a fix, notable as evidence that the standing rule needs a fence, not a
  comment.
- **M7 · `ROOT_KNOBS` has two members that fail differently** (`baseline-tokens §4` defects 4, 6, 7):
  `placement-root` over `NAME_MAX` → exit 0, `doc list` silently drops two managed docs, `validate`
  **exit 0**; both knobs accept `""` acked as `= .` (M50 records `""` and `.` as *unmixable at the
  type level*) and `x/../y` silently normalized. A1-D5 names one knob; the decision must name the
  registry.
- **Minor · counting.** The charter's own note (`decisions-pending.md:28`) already records the
  proposed-vs-final tier arithmetic honestly. Nothing to fix; noted so the Settle does not re-derive it.

---

## What I did **not** reach

- I re-drove **three** cells only (B4's `rename` head, B5's posture preview pair, B10's flag
  inventory). Everything else marked **[read]** is a source or doc read, or a relay of a baseline
  subagent's driven report — which, per the repo's own rule, is **a lead, not a measurement**.
- I did not run cargo, did not apply a mutant, and drove nothing on the debug binary — so every
  statement about a **fence** (the ⇔ fences, `debug_assert!` route spans, the pack-load stated-at
  fences) is a read.
- I did not drive any cherry-pick/revert/sequencer cell myself; §B8's state axis is relayed from
  `baseline-posture` and is **git 2.54.0-specific** by that companion's own bound.
- I did not check whether a test asserts `corpus-migration.md:299`'s *from-home walk* acceptance
  claim. **If one does, it is a false pin and is itself a finding** — that check is one `grep` and
  belongs in the Settle's re-drive.
