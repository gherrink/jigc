# M52 gap probe — dimension: docs

HEAD `85ad06c5`, binary `~/.local/bin/jigc` = `1.0.0-rc.15`. *driven* = run on that binary via
`dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc` (two-step eval, `cd "$REPO"`, no cargo,
nothing written into the working repo). Everything else is *read*.

---

## BLOCKING

### B1 · The `RecordPreImage` door count disagrees three ways, and one of those ways is doc-vs-doc at HEAD
*read.* `design/finalize.md:193` — "the record-only commit `create` / `add-task` / `add-from-spec` /
`discard`" = **four**. `design/reconciliation.md:149` — the same four, with the M47 lesson quoted beside
it (*"a doc that under-counts its own re-baselining sites makes the next commit door undiscoverable"*).
`design/team-ready-state.md:74` — "the four milestone ops … **and, since M49, a sub-task's `jigc task
discard`**" = **five**. Baseline [rollback L2, L3] drives **six** (`task discard <sub> --force` and
`milestone discard --force`). So two locked docs are already one behind a third at HEAD, and all three
are behind the binary. **team-ready-state.md:74 is the one that settled it** ("a rule about doors, never
a list of them" — a door inherits by writing the record). `finalize.md:193`'s and
`reconciliation.md:149`'s enumerations must ride the increment that fixes C2/C3, not a later fold-back.
Blocks because fork 1 cannot name its axis while three docs disagree on the population size.

### B2 · `reconciliation.md:144`'s "exactly **four** sites" is falsified inside its own file
*read.* `:144` "The recorded `file-state` hash updates at exactly **four** sites". `:111` (same doc)
states `jigc rename` re-baselines (`forget(old)` + `record(new)` + re-record each rewritten referrer) —
not one of the four. `design/corpus-migration.md:68` states the relocation re-keys `from → to` — a
sixth. The count is ≥6. This is the *same failure the paragraph at `:149` names in its own text*, one
paragraph up. It is load-bearing for tier 0: baseline [freeze L-1] drives `rename` re-keying the
baseline **past a blocking `file-state.hash-matches`**, and the doc that would have made that door
visible under-counts itself.

### B3 · The destroying-door subject for **non-`.md` bytes under a task area** is stated in no locked doc
*read.* Fork 3's answer has no home to restore. `design/team-ready-state.md:94` derives the rule at the
scope it holds: *"**every door that removes a worktree-shaped path under `.jigc/worktrees/`** probes that
path first and answers for what it removes"*. `:96` widens it to sub-task **`docs/*.md`** only
(`milestone.staged-prose`). `design/finalize.md:137` authorizes the removal with **no guard at all** —
"**Remove `.jigc/tasks/<task-id>/`.** The staging area's job is done". `design/project-setup.md:160`
enumerates `uninstall`'s *set* and its three refusing states, not a subject rule.
⇒ For the axis-3 `C-1` class the razor's **leg 1 (stated and violated) is NOT available** — no doc
states it. It has to be argued on **leg 0** (a hole in a declared surface: `team-ready-state.md:94`'s own
*"Silence is the one arm no door may take"* is stated over worktree paths only). The wave **writes** the
rule; `team-ready-state.md` → the destroying-door rule is its home, and `finalize.md:137` takes the
qualifier in the same increment.

### B4 · `workbench_paths`' doc-comment contradicts its own code, its own cited lesson, and has no locked home
*read.* `crates/cli/src/setup.rs:3030-3051`: "*`tasks/` and `worktrees/` are inside `ENTRIES` … excluded
by construction and answered by the doors that own them*" and, two paragraphs later, "***Every child that
is not a directory**, symlinks included (M49's lesson at `fanout_worktree_paths`): … the shape of a path
is a reason to recurse into it, never a reason to drop it from the set*". The code at `:3065-3070`
excludes on `entry.file_name() == prefix` — **a name match with no shape check** — so a plain *file* named
`tasks`/`worktrees`/`displaced` at `.jigc/` leaves the subject and dies at exit 0 ([destroying L-2]).
That is M49's *claim-about-shape-where-the-question-is-about-bytes* verbatim, re-enacted inside the
comment that cites it. **No `design/` doc states the `ENTRIES`-complement subject**; the only statement
of it is this doc-comment. Candidate home: `project-setup.md` → Teardown (`:160`), which already owns
"exactly the enumerated repo-local setup-created set".

### B5 · The posture family is a **closed enumeration** in two locked docs — so fork 2's family arm is new surface, not a repair
*read.* `design/finalize.md:31` — "**Three** predicates … an operation in progress (`MERGE_HEAD` ·
`rebase-merge`/`rebase-apply` · `BISECT_LOG`)". `design/validation.md:663` — "*a merge, rebase or bisect
the user started and has not concluded*". Cherry-pick and revert appear in **neither**. Baseline
[posture §1] drives **8 un-concluded operations + 2 HEAD postures + a conflicted index with no marker**;
`InProgress::markers()` carries 4 markers over 3 operations; two operations write **no marker at all**.
⇒ **Adding members is new capability under the charter's own "no new capability" boundary**, and it
cannot claim leg 1. What *is* stated-and-violated, and therefore cleanly repairable:
  - `finalize.md:31` + `validation.md:653` — "`--carry-staged` … stops concluding **a merge** it never
    consented to conclude". The binary concludes a **cherry-pick/revert** under jigc's subject (D3). The
    literal noun holds; the stated *rationale* ("never *conclude someone else's* operation") is violated.
    Fork 2's `--carry-staged` arm therefore repairs a **rationale**, and the doc sentence must be
    re-scoped from *merge* to *any in-progress operation* in the increment that ships it.
  - `finalize.md:31` — "a **`Human` route naming the git command that resolves the state** … the command
    for *this* operation rather than a menu of three". D1/D2 drive routes git **refuses**
    (`git rebase --abort` under `git am`; `git switch` under a rebase). Stated and violated, cleanly.
  - `validation.md:663`'s route column carries the same three-command menu and takes the same repair.
Also: **the `am`/`rebase --apply` discriminator** the baseline drove (`rebase-apply/applying` ⇒ am,
`rebase-apply/onto` ⇒ rebase) is a **git-version-dependent on-disk contract** ([posture §5], git 2.54.0)
and belongs in `validation.md`'s row with that bound written, not as a bare marker name.

### B6 · The corpus walk's key: stated `from`, driven `to` — and closing it may fire a *different* doc's trigger
*read.* `design/corpus-migration.md:68` — "The corpus walk keys on the **`from`** home (where the
instances actually sit), not the empty `to`." `:299` repeats it in the property census ("the corpus walk
enumerates the **`from`** home"). `:218` — "M42 makes the two branches a **union** — walk the prior home
**and** the placement file". Driven class [freeze §3]: the union reads prior **`location:`** only
(`.filter_map(|p| p.location)`), never a prior **`placement`**, so **3 of 4 (from-home × to-home) cells
are uncovered**. Stated and violated — leg 1 clean, at three sentences.
Two further doc consequences the Settle must not discover at decompose:
  - `:90` — "*a **missing** snapshot **blocks** that doc with a route (never a silent `already-current`)*"
    is falsified in the other direction: an uncovered cell puts the doc in **no bucket at all** (exit 0,
    every array empty). `:90`'s own guarantee is the sentence to widen.
  - `design/storage.md:208`'s placement-census row for `candidate_docs` reads "**both** homes: the
    versioned snapshot stores the prior `location:` **declared**, so the walk re-applies the `docs-root`
    prefix" — it is **not** falsified (it describes exactly the location arm that exists), but it is the
    row that must gain the prior-**`placement`** arm, and `DoctypeMigration` (`migrate_corpus.rs:76-81`)
    carries `docs_root` only, so the `placement-root` knob has to join it.
  - **Trigger crossover:** if fork 4 is answered by moving a `placement`/`location` home, that is a
    `schema-version` bump — which **fires** `implementation/decisions-pending.md:309`'s namespaced-stamp
    trigger ("*the next `schema-version` bump of any doctype* — that bump already ships a corpus
    migration, which is the one vehicle the key change needs, so the two ride together or the key change
    is paying for a migration twice"). That decision is then **due at M52's Settle**, not deferrable.

### B7 · `surface-contract.md:202` claims post-compose fence coverage the binary does not have
*read.* "*~75 literal `jigc` command lines inside pack step prose … are **fence-covered only
post-compose** (the compose-scope `Run:` emission); pre-compose they are style-guide territory.*"
Baseline [surfaces §3]: **13** workflows (the 12 `migrate-*` plus `milestone-execution`) compose
unrunnable `Run:` lines at exit 0 with `{{ source }}` empty; **14 renders across 13 workflows** assert
lists that are not there. Either the post-compose fence does not exist or it cannot see this class —
either way `:202` is a locked-doc sentence that is false at HEAD. Leg 1 clean for A6-2/A6-3, and `:202`
is the sentence the fix makes true (or must be re-scoped in the same increment).

### B8 · `reconciliation.md:186`/`:47` — "every absorb surfaces" — violated at two doors, with no exemption written
*read.* `:186` "**No silent discard, ever.** Every block surfaces; every absorb surfaces." `:44-47` "Parse
succeeds + schema validates → **absorb** … surface *external edit absorbed: `<doc>`* to the agent."
Driven at the baseline: `jigc ingest` absorbs a blocking `file-state.hash-matches` with
`"finding": null, "adopted": true` (D-1), and `rename` over a drifted committed adr commits and re-keys
the baseline past the same block with **no absorb line** [freeze L-1]. The doc names **no** door
exemption. Leg 1 clean at both; the fix is a *surface*, the classifier being correct ([freeze §3]).

### B9 · `command-output-contract.md:195` declares exactly two reject arms — and the charter's fork-6 price is wrong
*read.* `:195` "**The two reject arms — the only rows no single leaf owns.**" `setup`/`uninstall`
serialize a bare `Finding` (8 top-level keys) — DEFECT A. Leg 1 clean, one sentence.
**Charter correction owed:** `implementation/decisions-pending.md:98` prices arm-declaration as
"`doc schema` `contract-version` movement". The baseline falsifies the premise [contracts §3]: the
envelope version is `engine::result::SCHEMA_VERSION = 3` (`result.rs:27`); `doc schema`'s
contract-version (6) is a *different* pin and is not involved. `design/doc-read-surface.md:136`'s
"bumps on **any** structural change with **no additive carve-out**" governs `doc schema` alone. The fork
text must be corrected before the Settle reasons off it.

### B10 · `project-setup.md:160` records a repair as *done* that survives at the cell M50 did not cover
*read.* `:160` — "*before M50 one leftover file made this door answer only for the file, drop the sibling
directory's good refusal on the floor, and **route at "make sure `git` is on PATH"** — a route that fits
nothing it had found, **with the `--force` consent unnamed***. Driven-at-HEAD source: `setup.rs:3569`
`unverified_worktrees_finding` still emits *"make sure `git` is on PATH and the repository is readable"*
with no `--force`, for a failure its **own doc-comment at `:3566` names as `read_dir` on
`.jigc/worktrees/`**. So a locked doc asserts a past-tense fix that is incomplete — a law-1 lie in the
doc, and the axis-3 `D-4` row's home. The doc sentence and the route move in the same increment.

### B11 · No `design/` part-doc is inside the count fence, so every count M52 moves is unfenced prose
*read.* `crates/cli/tests/count_fences.rs:93-121` — `HOMES` is `CLAUDE.md`,
`implementation/decisions-pending.md`, `design/worked-examples.md`, three test files, `MIGRATING.md`.
**No `design/` part-doc but `worked-examples.md`.** `implementation/decisions-pending.md:253` records the
intended mold ("Verified-accurate-and-fenced enumerations: … fenced by `doctype_map_versions.rs`"; and
"`STORE_EXIT_FLIPS` is the pattern to copy — `validation.md:380` **refuses** to restate the count,
recording that the hand-count went stale three times"). This is precisely why B1/B2 could ship.
M52 moves at least these hand-written counts and **none has a fence**:
  | count | home | driven truth |
  |---|---|---|
  | four `RecordPreImage` doors | `finalize.md:193`, `reconciliation.md:149` (five at `team-ready-state.md:74`) | six [rollback L2-L4] |
  | "exactly four" re-baselining sites | `reconciliation.md:144` | ≥6 (`:111`, `corpus-migration.md:68`) |
  | three posture markers / three operations | `finalize.md:31`, `validation.md:663` | 4 markers / 3 ops, 8 operations, 2 marker-less [posture §1] |
  | two reject arms | `command-output-contract.md:195` | 3 shapes shipped [contracts §3] |
  | four `doc show` projections (code-side) | `render.rs:5635-5679` vs `doc-read-surface.md:72-78`'s **six** | 7 root shapes / 17 cells [contracts §3] |
  | three destroying doors that refuse | `team-ready-state.md:94` | 4 in `DESTROYING_DOORS`; `task finalize` in **none** [destroying §3] |
  | `UNSWEPT_PRODUCERS` | **79** at `decisions-pending.md:729`, **60** at `:83` — one file, two counts | — |
  Recommendation (rider, not scope): add the six `design/` homes to `HOMES` in the increment that moves
  the first of these counts, on the `doctype_map_versions` mold `decisions-pending.md:253` names.

---

## MAJOR

### M1 · `doc-read-surface.md` is **right** and the code-side declaration is wrong — no doc move owed
*driven.* `jigc doc show 'roadmap:roadmap#milestones' --format json` → a **top-level JSON array** at
exit 0 (and `changelog:changelog#releases` likewise). `design/doc-read-surface.md:71-78` declares six
`#fragment` shapes (slot → string · **repeatable section → item array** · fields-only → object · leaf →
value · item → object · nested leaf → value), i.e. seven root shapes with the whole-doc form. Against
that, `crates/cli/src/render.rs:5096` — `ArmShape::ArrayOf`'s doc-comment — states "*`jigc task list` is
**the surface's one array***", and only four `doc show` arms exist (`render.rs:5635-5679`). **The locked
doc settled it; the registry is behind.** DEFECT B's fix adds rows and costs `doc-read-surface.md`
nothing; `command-output-contract.md:157` ("*This section is where every remaining row's keys are
declared, in `ENVELOPE_ARMS` order*") is the only home that gains text.

### M2 · `ArmOutcome::Success` vs the exit taxonomy — again the doc is right
*read.* `command-output-contract.md:459-465`'s taxonomy table declares exit **1** for
`STORE_EXIT_FLIPS` members on the store sweep and exit **3** for a blocking `task validate`.
`render.rs`'s `ArmOutcome::Success` is documented "*Exit 0, the document on stdout*". DEFECT C is a
**code-comment/registry** defect; **no doc sentence moves**. Worth saying explicitly so the increment
does not open the taxonomy, which `:457` marks as the wave's one-way door ("*The one-way door here is
the statement, not the codes*").

### M3 · `validation.md:710`'s residual 2 understates itself — and it is C-2's disposition home
*read.* "*(2) An orphan at a **root-level placement home** … goes **unflagged**, at exit 0, which is
**that one cell's** pre-M51 status quo.*" Baseline [freeze §4 L-5]: **2 doctypes at the pack default, 5
under `placement-root: .`**, and under that knob with the pack dropped `jigc validate` exits **0**
*"validates clean"* over **three** stamped orphans. "That one cell" is false. The charter says C-2
"owes a DISPOSITION, not a fix" — its disposition is a **rewrite of `validation.md:710`** carrying the
amplifier (it makes tier-0 `D-2`'s placement arm a *total* false green), plus the decision at
`decisions-pending.md:309` if fork 4 bumps anything.

### M4 · `validation.md:723` states the unaddressable-identity rule at **one door**; `relocate` is outside it
*read.* The row scopes the predicate to "*at `jigc ingest`'s classifier (`cli::ingest::classify_row`)*"
while its own rationale is door-independent — "*adopting it records a file-state baseline and an edge-index
entry **no door can name***" and "***One predicate, not two agreeing ones***". Driven [tokens §4 defect 5]:
`jigc relocate adr --from notes` over `notes/My Note.md` lands `adr:My Note` **managed** at exit 0,
`doc show` refuses it, the route loops — the exact state the row says the code exists to prevent.
The rule as written is **weaker than the fix needs**; widening it honors the row's own stated rationale
(a candidate revise, not a conflict). Home: `validation.md:723`, in the increment that adds the predicate.

### M5 · `corpus-migration.md:75-81` describes a live path whose domain is **empty** at HEAD
*read.* `:75` — "*A **freeze-exempt** doctype … relocates through a **parallel path** —
`jigc relocate <type> --from <prior-home>`*" — with `:77`/`:81` elaborating. But
`implementation/doctype-authoring.md:21` records the methodology pack as manifest-governed since M40
("*all eleven shipped schemas listed*"), and `team-ready-state.md:201` states the consequence for the one
worked case ("*since M40 it is a methodology-manifest member, so a home change version-gates like any
manifest member and **`jigc relocate` refuses it***"). ⇒ **freeze-exempt = ∅**, which is exactly A1-D3
("the `relocate`/`from` registry row's declared runnable argv cannot reach its own arm"). Two locked docs
disagree about whether this path has a subject; `team-ready-state.md:201` is the one that settled it, one
doctype at a time, and `corpus-migration.md:75-81` never took the general consequence. The A1-D3 fix
(and any `relocate` hardening for A1-D2 / axis-2 `codex-1`) must say which it is.

### M6 · `write-commands.md:185` — "The two forms that start work" — is itself a law-1 lie
*read.* The open lead at `decisions-pending.md:83` carries `migrate`'s silent mint as "**matching** a
stated record (`write-commands.md`:185)". Read: `:185` says "*The two forms that start work … it rides
only these two forms*", while `MINT_DOORS` (M49) enumerates **five** production mint doors and
`jigc migrate` is a third work-starting one. So the "stated record" the lead rests on is a doc sentence
the binary already falsifies — the lead is **not** properly closed, and the sentence is a tier-2 item
whether or not the mint behaviour changes.

### M7 · The adopter-facing homes, and the batching rule that exists only as a past fact
*driven.* In a fresh rig install the guide artifact carries `jigc-version: 1.0.0-rc.15` and
`jigc-body-blake3: c25024ac…` at `.claude/skills/jigc/SKILL.md:4-5`;
line **30** is `cargo install --path crates/cli` (CX-3) and line **71** is "*every `jigc setup` rewrites
it*" (CX-2). Their sources are `QUICKSTART.md:19` and `QUICKSTART.md:60`. `MIGRATING.md:49` is the third
guide sentence in the wave's blast radius if the same-path migration wording moves.
`design/assistant-adapter.md:31` states the generation rule ("*the body is the repo's own `QUICKSTART.md`
+ `MIGRATING.md`, embedded at compile time … no second authored copy*") and `:38` the refuse-to-clobber
consequence (`adapter-guide.user-modified`). **What is written nowhere is the batching rule**: that every
guide byte moves `jigc-body-blake3` and engages refuse-to-clobber, so guide edits belong in **one**
increment and **one** hash move. Its only trace is `implementation/decisions-pending.md:387` recording
M49 Inc 13 as a past fact. M52 touches ≥2 guide sentences ⇒ it needs the rule; home:
`assistant-adapter.md` → the adapter's owned artifact, beside `:31`/`:38`.
*(Also: the baseline's CX-3 bound stands — `:31`'s "*in-repo relative links are resolved away*" already
covers documents; `crates/cli` is the guide's only non-document repo path.)*

### M8 · `storage.md` → Concurrent writers vs `rollback_rename`
*read.* `design/storage.md` → Concurrent writers: "*Every writer of the three shared files persists
through `state::persist`*" and `file-state.json` is "**merged, base-relative**" with the per-key rule
spelled out. Baseline [rollback L5]: `rollback_rename` "*rewrites `file-state.json` unconditionally*"
(and removes the new path with a racer's bytes in it). If that write bypasses `save`'s merge it
contradicts the section directly; if it goes through `save` the section's guarantee does not extend to a
*rollback-time* rewrite and the section owes that carve-out. Either way the section is the home, and the
answer is fork 1's business.

---

## The three hunts

### (a) cheap-vs-robust — per sentence, which cut is the one-way door at the pin
- **`finalize.md:200` — "Before phase 6, all-or-nothing — this sentence is the promise (M51)."**
  Cheap: narrow the promise to the populations that already restore correctly. Robust: extend capture/CAS
  to the eight un-covered populations. **One-way door at the pin** — `:200` is the *contract row* the
  whole table serves, M51 built rather than struck it (with the reasoning written at `:200` itself), and
  retiring it days before 1.0 is a documented promise withdrawn on the strength of an implementation gap.
  `fork · cheap-vs-robust` — the robust cut is the minimal-correct one.
- **`finalize.md:137` — "Remove `.jigc/tasks/<task-id>/`."** Cheap: qualify the sentence ("removes
  everything under the area, staged or not"). Robust: guard the removal (fork 3). **One-way door**: 1.0
  pins the destroying-door consent model (`--force` the single consent at each door,
  `team-ready-state.md:94`), and blessing an unguarded destroyer in a locked doc makes adding the guard a
  *behaviour change on a documented surface* after the pin. `fork · cheap-vs-robust`.
- **`finalize.md:31` / `validation.md:663` — the three-marker family.** Cheap: reword the enumeration to
  "an operation git records as in progress" and leave the predicate at three markers (a doc-only repair
  that removes the lie and fixes nothing). Robust: fork 2. **Not a one-way door for the family**
  (membership is additive and reversible), **but it is one for `--carry-staged`**: the flag's documented
  consent is pinned prose an adopter reads, and re-scoping it later is a consent change. Recommend: take
  the `--carry-staged` half as the repair (leg 1 clean, B5) and Settle the family half explicitly against
  the "no new capability" boundary rather than letting it arrive as a side effect.
- **`validation.md:710` — residual 2.** Cheap: restate the residual with the amplifier (the charter
  **explicitly offers this**: "*or restates it as still-declared with the amplifier written into the
  declaration*"). Robust: the namespaced stamp key. **Conditionally a one-way door**: if fork 4 bumps a
  `schema-version`, `decisions-pending.md:309`'s own logic says declining then "*pays for a migration
  twice*" — so the cheap cut is honest only if fork 4 lands **without** a bump.
- **`command-output-contract.md:459-465` — the exit taxonomy.** Do **not** open it. `:457` records the
  one-way door explicitly ("*The one-way door here is the statement, not the codes*"); DEFECT C is a
  registry repair (M2). Flagging so the increment does not take the cheap "reword the taxonomy" route.

### (b) foreclosed-by-doc
- **`finalize.md:195`'s directory carve-out forecloses one shared capture/CAS primitive.** Quoted
  rationale: "***The subject is those two files and not the config directory***: `.jigc/config` is one of
  the three staged pathspecs and carries a stated disposition rather than a capture, because a worktree
  pre-image over a directory is a different shape — absent-means-delete over N files, **including files
  this run never saw**." Baseline [rollback §3]: **two of the nine populations restore a directory tree**
  — the exact shape this sentence cites to exclude. **Engaging it:** the rationale rests on the directory
  being one jigc *did not create*, so "files this run never saw" are third-party by default. It does
  **not** transfer to `.jigc/tasks/<id>/`, which `unwind_mint` created in the same run — there,
  absent-means-delete is correct for everything except bytes written *after* the mint, which is precisely
  the loss cell ([rollback L6], the class's most severe). So this is a **candidate revise**, not a
  conflict: the carve-out survives for pre-existing directories and must be re-scoped in those words, in
  the increment that gives `unwind_mint` a capture. `fork · foreclosed-by-doc`.
- **`storage.md` → Concurrent writers does *not* foreclose a shared primitive** (its subject is the three
  shared JSON caches), but it constrains where one may sit: "*the critical section **spawns no
  subprocess***", which is what keeps jigc's own `pre-commit` hook — a nested `jigc` process — from
  deadlocking. A capture/CAS primitive placed inside a save-scoped lock would violate it. Naming it so the
  Settle does not design into that wall.
- **The `--ignored` refusal's ground does not foreclose fork 3 — it argues *for* the complement rule.**
  `team-ready-state.md:94`'s recorded, measured ground: "*a provisioned worktree arrives tracked-only
  while the sub-task walk tells the agent to build the code and run the tests, so build output sits in
  every worktree that did its job (measured 1 → 4 → 43 entries) and a refusal on that axis fires on the
  ordinary fan-out **success** path and trains `--force` into reflex*". Baseline [destroying §3] measures
  the same objection *harder* in task areas (jigc itself writes `base.json` · `intent` · `workflow` ·
  `staged-snapshot.json` · `docs/provenance.json` into **every** area from `jigc start`, plus
  `roles.json` and `source`/`source-path`), so *"any non-staged-`.md` byte blocks"* fires on 100 % of
  tasks. The rationale therefore **selects** the derived answer rather than blocking it: the subject is
  the **complement of jigc's own writer set**, code-side derivable (`engine/src/state.rs`'s seven
  `*_FILE` constants + `cli/src/migrate.rs::SOURCE_FILE`), never a shape or suffix claim. Not a fork —
  recorded so the Settle can cite the ground instead of re-deriving it.

### (c) prior-art-reconciled — locked statements that disagree
Grepped `rollback` · `pre-image` · `restore` · `operation-in-progress` · `rebase-apply` · `cherry-pick` ·
`leftover` · `workbench_paths` · `placement` · `candidate` · `from home` · `orphan` · `territory` ·
`reject` · `envelope` · `projection` · `resume:` across all of `design/` and `implementation/`.

| # | side A | side B | which settled it |
|---|---|---|---|
| 1 | `finalize.md:193` + `reconciliation.md:149` — **four** record-pre-image doors | `team-ready-state.md:74` — **five**, "a rule about doors, never a list of them" | **B** (it states the rule; the other two enumerate) — code is six |
| 2 | `reconciliation.md:144` — re-baselining at "exactly **four** sites" | `reconciliation.md:111` (rename) + `corpus-migration.md:68` (relocation re-key) | **B** — same file contradicts itself |
| 3 | `corpus-migration.md:68`/`:299` — walk keys on **`from`** | `corpus-migration.md:218` — union of prior home **and** the placement file; `storage.md:208` — prior `location:` **declared**, re-rooted | **A** is the design intent, **the code implements neither fully** (fork 4) |
| 4 | `corpus-migration.md:75-81` — `jigc relocate` is the freeze-exempt relocation path | `doctype-authoring.md:21` + `team-ready-state.md:201` — every methodology schema is manifest-governed since M40, so `relocate` refuses | **B** — A's domain is empty (A1-D3) |
| 5 | `doc-read-surface.md:71-78` — **six** fragment shapes incl. a repeatable-section **array** | `render.rs:5096` — "`jigc task list` is the surface's one array"; four `doc show` arms | **A**, *driven* — DEFECT B |
| 6 | `command-output-contract.md:459-465` — exit 1 / 3 declared for two arms | `render.rs` `ArmOutcome::Success` — "Exit 0" | **A** — DEFECT C, no doc move |
| 7 | `command-output-contract.md:195` — **two** reject arms | `render::setup_block` (`render.rs:3383`) `Format::Json => json(finding)` | **A** — DEFECT A |
| 8 | `reconciliation.md:186`/`:47` — every absorb surfaces | `ingest` (D-1, driven) and `rename` ([freeze L-1]) | **A** |
| 9 | `project-setup.md:160` — the "`git` on PATH" route + unnamed `--force` recorded as **fixed at M50** | `setup.rs:3569` still emits it; `:3566`'s own comment names the real cause | **the doc is aspirational** — axis-3 `D-4` |
| 10 | `team-ready-state.md:94` — destroying-door subject is "*a worktree-shaped path under `.jigc/worktrees/`*" | `setup.rs:3030-3051`'s `ENTRIES`-complement comment (a *third* subject), and `finalize.md:137`'s unguarded removal (a *fourth*) | **none** — three subjects, no reconciliation (fork 3) |
| 11 | `validation.md:723` — the unaddressable-identity predicate at **`ingest`'s classifier** | its own "*One predicate, not two agreeing ones*" + `relocate`'s driven landing of `adr:My Note` | **the rationale**, not the scope (M4) |
| 12 | `validation.md:710` — residual 2 is "**that one cell**" | [freeze L-5] — 5 doctypes under `placement-root: .`, 3 stamped orphans, `validate` exit 0 | **the baseline** (M3) |
| 13 | `decisions-pending.md:729` — `UNSWEPT_PRODUCERS` = **79** across 11 files | `decisions-pending.md:83` — M50 left **60** countable | unreconciled, one file |
| 14 | `storage.md` → Concurrent writers — `file-state.json` is merged base-relative through `state::persist` | `rollback_rename` rewrites it unconditionally [rollback L5] | unreconciled (M8) |
| 15 | `surface-contract.md:202` — pack `jigc` lines are fence-covered post-compose | 13 workflows compose unrunnable `Run:` lines at exit 0 [surfaces §3] | **A is false at HEAD** (B7) |
| 16 | `write-commands.md:185` — "the **two** forms that start work" | `MINT_DOORS` = five; `jigc migrate` is a third | **B** (M6) |

**Nothing found disagreeing** on: `resume:` (the JSON-arm silence is declared once, at `render::composed`'s
`Format::Json` comment, and the charter's open lead 5 quotes it correctly) · `territory` (`validation.md:703`
/`:708` is the single home and `decisions-pending.md:309` cites it accurately) · `leftover`
(`LeftoverShape` is stated once, `project-setup.md:160`, consistently with `team-ready-state.md:94`).

---

## What I did not reach
- I read code only to locate the sentence a doc contradicts; **every behavioural claim above is the
  baseline's** (relayed) except the three marked *driven*.
- I drove `committed-singletons` only — the two drives were `doc show` over a repeatable section and the
  installed `SKILL.md` bytes. No posture, rollback, destroying or migration cell was driven here.
- I did not diff the twelve `migrate-*` step files themselves; A6-2/A6-3's **count** is the baseline's.
- `doctype-map.md` was checked for version claims only; under the zero-schema-hash default it moves
  nothing, but **fork 4 with a bump makes it a home** (it is fenced by `doctype_map_versions.rs`).
