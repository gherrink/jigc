# M53 — the settle record (the rc.17 fix pass)

**Settled 2026-09-21**, human-led, from the main session. Charter:
[decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)* ·
baseline: [baseline-ledger.md](baseline-ledger.md) with five companions · advocates:
[advocates/](advocates/) (two forks, both cheap-vs-robust, neither framed by the proposer).

**Posture.** Every driven fact below was executed on the installed release `1.0.0-rc.16`
(code-identical to `HEAD = 155054cc`) on `dev/jigc-rig` corpora. *An agent's report is a lead*: the
claims each fork turned on were **re-driven by the orchestrator** before the fork went to the human
([baseline-ledger.md](baseline-ledger.md) §5, plus the three drives recorded at D2/D3 below); everything
else is **relayed** from a companion, which marks its own claims DRIVEN or READ.

**The shape of this Settle.** A fix pass: no gap fan-out. The charter carried a fix shape per row; the
baseline tested each against the binary. **One of the four survived as written** (A3-1's, and it left a
residue that became a fork). Three were corrected on a driven datum, and one of those corrections — the
human's — added a fifth scope item. Five decisions, every fork decided by the human, **one against the
orchestrator's recommendation** (D3, recorded as such, and the census then moved the orchestrator).

**Amended at review** — [Review amendments](#review-amendments-2026-09-21) §1–§14, every finding accepted;
where an amendment contradicts a sentence in D1–D5, the amendment is the design.

---

## The boundary, as it now stands

The charter's: *no new mechanism — a row in a registry that already exists, or a condition on a guard that
already exists; no new disposition, family, registry; no new finding code unless the row's door already
carries none; zero schema-hash movement; a fix that needs new mechanism is a halt to the human with its
sibling cells enumerated.*

**What M53 mints, in full** (the ledger the completion audit and the re-review grade against):

| kind | count | what |
|---|---|---|
| finding codes | **1** | `finalize.foreign-bytes` (D2) — admitted by the charter's own clause: both `Displace` doors carry `codes: &[]` |
| enum variants | **1** | the tenth `InProgress` member (D4) — the charter's own prescription |
| registries, dispositions, families, gates, phases, envelope keys, `contract-version`s | **0** | |
| schema-hashes, `schema-version`s, corpora | **0** | the negative fence is asserted at the close |
| **new mechanism, by the human's halt** | **1** | **D3 — the residual rule**: *a task is an area that carries its base pin*. Named, its sibling cells enumerated ([baseline-residual-area-census.md](baseline-residual-area-census.md)), admitted by the human 2026-09-21 |
| a widened walk, by the human's halt | **1** | **D1 — `merged/` is walked**: a second walk depth in the shipped complement probe. The F2 advocate named it as *the one item honestly owed as a halt*; admitted by the human |

---

## D1 · `(3, A3-1)` — the milestone area answers for what it removes, `merged/` included

**Decided (human, F2 → *walk it*).**

1. **The milestone area joins the shipped displacement.** `post_commit`'s `displace` parameter
   (`task.rs:5956`; mirrored at `try_execute_finalize_plan`, `:3856`) carries the **kind** beside the unit
   id, because `WorkArea::Task` is hard-coded at `:5976`. Both milestone call sites (`milestone.rs:5098`,
   `:5228`) pass `(WorkArea::Milestone, <milestone-id>, sink)` instead of `None`, and their comment —
   *"this door must not answer for a subject it was not given"* — is struck with its datum. Destination:
   `.jigc/displaced/<milestone-id>/<relative>`, the primitive's own computation. The pairs join the
   existing `committed.displaced` union (sorted by `from`, present always); `render.rs:4836` / `:1933`'s
   *"a sub-task's working area"* doc-comments are widened to say what the key now carries.
2. **`merged/` is walked.** `engine::state::foreign_area_paths`' tree branch becomes a match on `kind`:
   `Task` keeps its one-level `docs/` walk; `Milestone` walks `merged/` — under `merged/docs/` an entry
   the shipped `staged_doc_id` predicate recognises is jigc's, **every other entry under `merged/` is
   foreign and returned whole** (a foreign directory is one entry, the shipped unit rule). All four doors
   read this one probe, so `milestone finalize`, `milestone discard` (± `--force`) and `uninstall` answer
   at their **existing** identities.
3. **`materialize` clears what it wrote.** `engine::milestone::materialize`'s unconditional
   `remove_dir_all(merged/docs)` (`engine/milestone.rs:1890`) removes only entries `staged_doc_id`
   recognises. Safe because a left-behind foreign file reaches nothing (READ by the advocate:
   `plan_promotions` `finalize.rs:1141` and the gate copy loop `milestone.rs:5843` both skip a stem with
   no `<ty>:<slug>` split) — **a claim the increment's first task spikes before building on it.**
4. **`unwind_area`'s milestone arm honours the same walk** (`engine/state.rs:415`, a bare
   `remove_dir_all(merged/)`). The advocate conceded this arm *unreachable* — true at HEAD, **false the
   moment D2 lands**, because D2 calls `unwind_area(area, WorkArea::Milestone)` at the landed boundary
   over an area where `merged/` exists. **A predicted hole is a named acceptance cell, not settle prose**
   (the M52 retrospective's rule): *a foreign byte under `merged/` whose move failed survives a landed
   `milestone finalize`* — cell **D1×D2** in the acceptance design.

**The foreclosing doc, engaged at its predicate.** `team-ready-state.md:92` — *"the join's staging area,
jigc's wholesale, never walked"* — is an exclusion resting on the predicate *nothing but jigc's bytes are
in `merged/`*. Driven false four ways (advocate §1): a blocked finalize leaves materialized bodies there
at exit 3; a succeeding `pre-commit` hook writes there (the writer M52 itself recorded at
`engine/state.rs:214`); `materialize` took an editor `.swp` on a finalize that committed nothing; and the
carve-out **defeats the consent gate** at the two doors that have one — **re-driven by the orchestrator**:
`milestone discard` with no `--force` over `merged/top.txt` + `merged/docs/deep.txt` → exit 0,
*"workbench removed"*, both gone, while the identical byte at the area root refuses with
`milestone.foreign-bytes`. The sentence is rewritten with those data; `engine/state.rs:165-167` with it.

**Why the cheap arm was not available.** After the charter's fix alone, `milestone finalize` would print
`"displaced": []` on a 1.0-pinned envelope while destroying bytes under `merged/` — silent today, *false*
after. And the instrument that is the human's gate lists the `materialize` site already
(`M52/per-axis-review/codex/axis-3-source-pass.md:55`) and mis-graded it by inheriting the doc's phrase.

**Check scope.** No new check. `milestone.foreign-bytes` / `uninstall.foreign-bytes` fire on the widened
probe's output at their shipped surfaces (the door's own area, before anything is removed).

**Declared bounds.** A foreign directory inside `merged/` moves whole, never merged · a milestone id and a
task id can collide under one `.jigc/displaced/<id>/` (driven; suffix-resolved by
`free_displacement_path`, no loss — a provenance ambiguity, stated in `storage.md`) · the
undecodable-filename bound at `task.rs:5845` is unchanged.

**Acceptance axis.** `{area root · merged/ top · merged/docs/ beside a materialized body · merged/<dir>/**}
× {materialize on a blocked finalize (exit 3) · landed finalize, both squash arms · milestone discard ±
--force · uninstall}` — plus the zero-false-fire control: an ordinary post-join area displaces **nothing**
and `displaced` is `[]`.

---

## D2 · `(3, A3-2)` — the removal is conditioned on the move

**Decided (human, F1).** The charter's shape could not be built as written — three premises falsified
([baseline-ledger.md](baseline-ledger.md) §3). The advocate withdrew its own pre-commit arm on a driven
datum (a succeeding hook writes into the area *during* the commit, where no probe can see it; and
displacing before the commit falsifies the survivable frame's state-truth clause on a hook rejection).

1. `displace_foreign_area` reports the entries it **could not** move beside the ones it moved.
2. `narrate_displacement` names both sets and counts the **complement**, never `moved.len()` — the
   partial cell's *"held 1 entry"* over an area that held two is a law-1 lie at HEAD (driven, baseline
   §4.1).
3. `post_commit` (`task.rs:5981`) and `cleanup_subtask_areas` (`milestone.rs:5538`) replace
   `remove_dir_all` with the shipped `engine::state::unwind_area(area, kind)`: jigc's registry members
   go, the directory goes through non-recursive `remove_dir`, and `AreaUnwind::Foreign` is the typed
   *left standing*. The same discipline M52 Increment 5 already applied one door over
   (`finalize.md:215`).
4. On `Foreign`: **one advisory `finalize.foreign-bytes` per area** — located at the area, message *kept,
   not taken* (the N paths, the failing move's reason, *`.jigc/` is gitignored, so nothing else has a
   copy*), route `Human` naming the repo-relative path to move out and delete (D3 decides that no jigc
   verb resolves a residual). **Exit stays 0**: the commit is truth, and the landed arm is exit 0 by the
   taxonomy (`command-output-contract.md`).
5. **Where the finding rides.** `task finalize`'s landed envelope carries `findings` (driven) — it rides
   there. `milestone finalize`'s landed arm is **pinned at `Object(&["committed"])` with no `findings`
   key** (driven by the advocate; the additive-key window closed at M48) — there it is **stderr-only, and
   the record says so**. The safety of this decision does not rest on being heard: D3 makes the residual
   inert at every door.
6. `engine::milestone::discard_sub_task_item` asks the shipped `item_is_settled(item)` before the splice
   and **refuses with the shipped `milestone.terminal`**, its route naming the leftover area to clear by
   hand. Its doc-comment — *"a guard here would be code no state can reach"* — is rewritten: the premise
   is **driven false at HEAD, with no fix applied** (orchestrator: a bare `mkdir .jigc/tasks/first-sub`
   after a landed boundary, then `jigc task discard first-sub` → exit 0, `record commit: d53299d`, the
   record reading `status: joined` over `- status: discarded`). After D3 a residual no longer reaches this
   function; the guard covers what still can — a **whole** leftover area (its pin intact) after an
   `unwind_area` I/O fault.

**Check scope.** `finalize.foreign-bytes`: advisory · fires at the **landed** arm of `jigc task finalize`
and `jigc milestone finalize` only, post-commit, once per working area left standing · target = the area
(`task:<id>` / `milestone:<id>`) · `Human` route · joins `FINALIZE_FAMILY` and `ERROR_CODE_REGISTRY`.
`DESTROYING_DOORS`' `codes` stay `&[]` at both doors — that field is *door-scoped blocking codes the door
refuses with*, and a kept-bytes advisory is not a refusal; the ⇔ stays true and the plan says so where
the row is.

**Docs that move, each strike carrying its datum.** `storage.md:291` and `finalize.md:157` (*the teardown
is never skipped*) — restated as *never skipped for what jigc wrote; never taken for what it could not
move* · `finalize.md:159`'s second clause (*"or the next finalize reusing the slot"*) — **driven false**
(re-finalize exits 1 on the missing pin; the mint refuses `task.serial-collision`) · `task.rs:5839-5843`,
`post_commit`'s doc (`:5942`), `finalize_displacement.rs`'s module doc ·
`command-output-contract.md`'s `finalize.*` sub-table gains the row.

**Acceptance axis — 18 cells, not the charter's 2×2.** `{all move · some move · none move} × {unwind ok ·
unwind faults} × {task area · sub-task areas · milestone area}`, through the real binary at both doors,
both squash arms where the door has them. *Some move* is manufactured without permission games: occupy
`.jigc/displaced/<id>/docs` with a file. Plus cell **D1×D2** (above) and the discard cell (D2.6).

**Declared bounds.** Stderr-only at `milestone finalize` until 2.0 · `free_displacement_path`'s unbounded
`loop` untouched (READ, not driven) · an `unwind_area` I/O fault can leave a whole area standing with its
pin — listed as active, truthfully a task jigc could not tear down, cleared by `task discard --force`
(plain) or by hand (sub-task, D2.6).

---

## D3 · the residual rule — a task is an area that carries its base pin

**Decided (human, against the orchestrator's first recommendation; the census then moved the
orchestrator).** The recommendation was to declare the lying roster as a bound. The census
([baseline-residual-area-census.md](baseline-residual-area-census.md)) showed that was never really
available: the tier-1 cell is reachable on **today's** binary by a bare `mkdir`, and the predicate closes
four cells where the guard alone closed one.

**The predicate.** `symlink_metadata(<area>/base.json).is_file()`. Existence, never a parse
(`mint_task` writes the pin with a plain `fs::write`, so a parse-keyed predicate would have a torn-read
window the milestone side lacks). **Its load-bearing property, driven at all five `MINT_DOORS`** —
including the `Snapshot::Exempt` `add-task` (re-driven by the orchestrator: `base.json · intent ·
workflow`) and the fresh-clone re-seed: every legitimate area carries the pin, it is the **first** content
write after `create_dir_all` (`engine/state.rs:1005-1009`), and `unwind_area` removes it
(`TASK_AREA_FILES[0]`). The mid-mint window **narrows** from three writes to one and flickers in the safe
direction.

**Its homes — where membership is decided, not a list of sites.**

| home | what changes | doors it moves |
|---|---|---|
| `engine::state::list_active_task_ids` (`state.rs:1030`) | `is_dir()` → the predicate | `task list` · `start` (orient) · `also open:` · `doc`'s implicit task · the two stderr hints · `rename.in-flight` |
| **one** shared resolve predicate, called by R1–R4 (`task.rs:1892` · `start.rs:2199` · `:2327` · `doc.rs:6001`) | four copies of `!dir.is_dir()` → one call | every by-id door — all 25 `WORK_UNIT_ID_DOORS` rows on the new cell |
| `rename.rs:1090` `first_dir_name` | the milestone twin takes the predicate over `MILESTONE_AREA_FILES[0]` | `jigc rename` (the wedge over a terminal milestone-area residual — *relayed*, not re-driven: the orchestrator's corpus held singletons only; the increment's first task drives it) |

**What a by-id door answers over a residual.** The shipped **`finalize.no-task`**, key unchanged
(`(finalize.no-task, task:<id>)`), with a **residual sentence** — *no task `<id>`: `.jigc/tasks/<id>` is a
leftover directory holding N path(s) jigc did not write* — and a `Human` route naming the repo-relative
path. One producer (`engine::finalize::no_such_task_finding`, route already a parameter). This **deletes**
the three code-less, host-absolute-path producers (`start.rs:2220`, `:2345`, `task.rs:1928`) — a residual
never reaches the pin read. **`jigc task discard` is not an exception**: a residual is a task at **no**
door, so no door answers differently from the other 24 on this cell, and the false-flip path (D2.6) is
closed at resolution.

**What the mint answers.** `mint_task`'s `dir.exists()` (`state.rs:1001`) keeps refusing — adopting the
directory would mix jigc's files with foreign bytes at a door that could not before. Under the shipped
`task.serial-collision`, a residual gets its own sentence and the same `Human` route (today's *"task X is
already active … resume with `start --task`"* would become a lie with a dead-end first arm). The
milestone twin: `milestone.serial-collision` over a record-less residual, whose route today dead-ends at
`milestone.area-io`.

**Left alone, declared.** `reseed_sub_task_areas`' `.exists() ⇒ skip` (`engine/milestone.rs:1099`): a
residual under an **open** milestone stays a local dead end until the human removes the path the refusal
names — after which the re-seed heals it (driven: no door re-seeds *into* a residual). Touching it would
write jigc's files beside foreign bytes through the fix · `staged_task_prose`'s own `read_dir`
(`task.rs:772`): a residual holding a `docs/<ty>:<slug>.md` is still named by the three `*.staged-prose`
guards, the deliberate keep-too-much rule · `E9/E10`'s `.exists()` vs `is_dir()` divergence (READ,
unexercised).

**Check scope.** No new code. `finalize.no-task` and the two `serial-collision` codes fire where they
fire today; only the residual cell's sentence and route are new.

**Fences and docs that move.** `flow37_rename.rs:815` / `:1746` plant a bare `create_dir_all` and assert
`rename` blocks — the one standing test pinning *directory ⇒ active*; its fixtures move to a minted task,
and a residual arm asserts `rename` **proceeds** · `orientation_active_task.rs`'s byte-identity arm gains
the residual corpus (renders `Clean`) · `work_unit_unknown_envelope.rs` and `work_unit_id_axis.rs` gain
the residual cell · `no_such_task_route.rs`'s byte-identity claim is restated for the two sentences ·
`write-commands.md:180` (the active set's definition gains *carries its base pin*) ·
`engine/state.rs:1024`'s doc-comment.

**Acceptance axis.** `{empty dir · foreign file · foreign docs/<ty>:<slug>.md} × {plain id · sub-task of
a joined milestone · sub-task of an open milestone} × WORK_UNIT_ID_DOORS ∪ the enumerating doors ∪ the
same-slug mint`, plus the milestone-area twin × `rename` and `milestone create`.

---

## D4 · `(2, DEFECT A)` — the tenth `InProgress` member

**Decided (human, F4 → *as corrected*).**

- **Predicate:** `MERGE_MSG ∧ ¬(MERGE_HEAD ∨ CHERRY_PICK_HEAD ∨ REVERT_HEAD ∨ SQUASH_MSG ∨ rebase-merge ∨
  rebase-apply)` — the charter's, plus the two rebase conjuncts: a paused `rebase-merge` leaves
  `MERGE_MSG` with none of the four (**re-driven**). Disjoint **by predicate**, the style `Am`/`Rebase`
  already use.
- **Position:** after every marker-keyed member, **before `UnmergedIndex`** — so a conflicted
  `cherry-pick -n` stops being *"a conflict"* routed at `git reset --merge` (which destroys the picked
  bytes) and gets this member's noun and route.
- **Route:** conclude `git commit` (*once its conflicts are resolved*, the shipped mold); abandon
  **`git reset`** — git-native, worktree-correct, clears `MERGE_MSG`, keeps the picked bytes as unstaged
  changes (**re-driven**; rc 0 on the conflicted cell too, relayed). The charter's `rm .git/MERGE_MSG` is
  **false in a linked worktree** (`MERGE_MSG` is per-worktree — and `milestone finalize` runs there) and is
  not a git command; `cherry-pick --abort` / `merge --abort` exit 128. The route says the pick is
  unstaged, not lost.
- **Noun:** distinct from `CherryPick`'s *a cherry-pick* — *an uncommitted cherry-pick*; the inventory
  fence keys rows on `(noun, abandon)`.
- **The fixture substrate gains four states**, all driven byte-identical or sibling by the auditor: clean
  one-commit `-n` · clean multi-commit range `-n` (no `sequencer/`) · conflicted `-n` · conflicted `-n`
  after `git add` — as `GitState` variants mapping to the one member, in `git_state.rs`, `dev/jigc-rig`
  and `dev_rig_parity.rs`. `AUTO_MERGE` stays outside `MARKER_UNIVERSE` (written by clean `stash apply`
  and a concluded rebase too).
- **Homes kept true:** `finalize.md:37` and `validation.md:776` by the shipped generation fence
  (`posture_member_inventory.rs`); the enum's doc-comment and the seven prose *nine*s by hand — reworded
  **without a numeral**, since nothing fences them.

**The axis is wider than the row.** Three damage shapes across the 12 acting `BEHALF_DOORS` rows (driven):
*swallow* (`task finalize`), **message-only kill** (the four milestone record doors), **index
contamination** (`config set docs-root`, a mover). The family already refuses at every acting door by
construction; `flow53` arm 2's *"the sweep consumes no marker"* would be **red today** under this state
and is the regression fence.

**Check scope.** `repo.operation-in-progress`, unchanged: every acting door and `task validate`'s
preview. **No false positive** in 12 driven concluded states (both hook-rejection shapes, ff merge,
concluded squash, `merge --abort`, concluded rebase). **Bound:** GUI clients and a crashed git, undriven
— carried under the family's existing git-2.54.0 deferral.

---

## D5 · `(5, DEFECT 1)` — a title that yields no id is refused where the id is derived

**Decided (human, F3 → *unslugable-title, engine seam, and converge `jigc start`*).**

- **Identity: `write.unslugable-title`**, a third producer with this door's own sentence. The charter's
  `work-unit.malformed-id` is **inert** on the derived id (`mint_id("") == "milestone"`;
  `is_slug("milestone")` is true) and **false** on the title (an id grammar as the repair for free
  prose). The mint doors cannot be `WORK_UNIT_ID_DOORS` rows — that registry is derived ⇔ from clap's
  id arguments.
- **Seams:** `run_create`'s top (`cli/milestone.rs:629`) — before `guard_record_free`,
  `gitignore::ensure` and `mint_milestone`, so **no write precedes it**; and
  **`engine::milestone::add_task`** before `mint_sub_id` (`engine/milestone.rs:371`) — one site covering
  `milestone add-task` **and `milestone add-from-spec`**, the third committing door the charter did not
  name (driven: `task:task` committed from a spec criterion titled `日本語`).
- **`jigc start` converges**: `start.rs:92`'s bare `anyhow` — code-less, and false for non-Latin input
  (*"must contain at least one letter or digit (got "日本語")"*) — takes the same code and a true
  sentence.
- **The sentence says what slugs**: ids are built from ASCII letters and digits, so a title in another
  script, or of stopwords only, yields none. **The cell is ordinary, not adversarial** — every non-Latin
  title and `"the"` trip it (driven, nine titles).
- **Target:** the bare work-unit type token (`milestone` / `task`), mirroring `create.empty-title`'s
  bare-doctype-id rule (`command-output-contract.md:237`) — there is no `milestone:<?>` to address.
- **Which reject arm it rides** at the milestone doors is **relayed both ways** (the auditor observed the
  flattened `{error}`; M52 Increment 1 moved finding-carrying rejects onto the findings arm) — the
  increment's first task drives it and the finding rides whichever arm a finding at that door rides
  today. No arm moves.

**Tests that pin the defect and must move.** `engine/milestone.rs:4801`
`empty_title_falls_back_to_type_name` stays green (the milestone guard is CLI-side); the fallback
expressions stay — `mint_migration_in_repo` relies on the task-side one by design.

**Acceptance axis.** `MINT_DOORS` — in the suite that already iterates it,
`work_unit_id_axis.rs::every_mint_door_produces_an_id_every_door_accepts` — crossed with `{"" ·
whitespace · punctuation · non-Latin · stopword-only}`, plus `add-from-spec` with a degenerate criterion.
The new predicate the fence asserts: *the id is derived from the caller's prose, never fabricated*.

---

## Rows the baseline surfaced that M53 does **not** fix — to the 1.x ledger, by the exit rule

Tier-2/3, each with the charter's trigger (*the first adopter finding on that surface, or the wave that
next touches its producer*): `doc add-item --title ""` answers under a third code for the same condition ·
`create.empty-title`'s route says `--title` at `doc author` · `task bind`'s and the `doc` write verbs'
code-less *"no recorded workflow"* (the residual half is closed by D3; a genuinely workflow-less area is
not) · `free_displacement_path`'s unbounded loop · `mint_task`'s non-atomic pin write vs
`mint_milestone`'s `persist` · the M52 review's R-I evidence shape (`grep -rl` under a gitignored tree).

## Instrument notes for the re-review

Drivers use `command grep` with a before-control under `.jigc/` · the axis-3 driver is handed D1's and
D2's declared bounds so it grades **against** them · the three-axis re-review's tier-1 definition is the
charter's, unchanged.

---

# Review amendments (2026-09-21)

The pre-decompose review ([design-review.md](design-review.md) — Opus, driving the release binary and
reading at the line) returned **NOT READY: five blocking, nine should-fix**, every one the M52 signature —
*a claim that a shipped predicate, producer or registry transfers, false when read at the line.* **Every
finding is accepted**; the orchestrator re-read all five blocking data at their lines before accepting
(`milestone.rs:5855` · the three `cleanup_subtask_areas` callers · `invocation_log.rs:602-625` ·
`engine/milestone.rs:1176` · `repo.rs:417`). The decisions above stand as amended here; where an
amendment contradicts a sentence above, **the amendment is the design**. What the reviewer checked and
found sound (eight reuse claims, listed at the end of its report) is not re-litigated.

**§1 (B1 → D1.3) · the gate copy loop asks the predicate its two siblings ask.** D1.3's premise named two
consumers with one predicate; there are **three with two**. `plan_promotions` (`finalize.rs:1142`) and
`staged_doc_id` use `split_once(':')` and skip a colon-less stem; the milestone gate's copy loop
(`cli/milestone.rs:5855`) uses `split(':').next()`, so a foreign `merged/docs/adr.md` resolves to doctype
`adr`, is **copied into the gate staging area**, and blocks the boundary at exit 3 with
`schema-conformance.unknown-type` (driven by the reviewer on the identical seam at the task door) —
**before** the commit, so phase 7's displacement never runs and every re-run blocks. Today `materialize`'s
wholesale clear hides the cell; D1.3 would expose it. **The copy loop takes `split_once(':')`** — one
condition on an existing guard. New acceptance cell, named: *a foreign `<doctype>.md` under `merged/docs/`
survives `materialize`, is not gated, and is displaced at phase 7.* `materialize`'s own doc-comment
(*"truncated before writing"*, `engine/milestone.rs:1855`) joins D1's docs that move (N2).

**§2 (S7 → D1.2) · the milestone rule is `staged_doc_id` alone.** The Task branch's predicate is
`TASK_DOCS_FILES.contains(name) || staged_doc_id(name)`; `materialize` writes **only** `<ty>:<slug>.md`
bodies, so inheriting `TASK_DOCS_FILES` would call a foreign `merged/docs/provenance.json` jigc's and
destroy it. Cell named in the axis.

**§3 (B2 → D2.3) · the sink is conditioned on the caller's disposition.** `cleanup_subtask_areas` has
**three** callers, and `run_discard` (`milestone.rs:4009`) passes `SubtaskComplement::Take` — where
`--force` *is* the consent. Its own doc-comment: *"A disposition read off the function instead of the call
would silently re-decide `jigc milestone discard`."* So: **`Displace ⇒ unwind_area`, `Take ⇒
remove_dir_all`, unchanged.** New acceptance cell: *`milestone discard --force` over a sub-task area
holding a foreign byte still removes it and still acks `workbench removed`.*

**§4 (B3 → D2 check scope) · `finalize.foreign-bytes` joins `FINALIZE_FAMILY` only.**
`ERROR_CODE_REGISTRY` is the committing doors' commit-phase **outcome** vocabulary, asserted equal to the
`COMMITTING_DOORS`-derived set with a hard `== 11`; `task.rs:830-833` already states the rule for this
family (*a door refusal … joins neither `CHECK_INVENTORY` nor `ERROR_CODE_REGISTRY`*). The finding reaches
the invocation log through the findings channel, the path `DISCARD_FOREIGN_BYTES` takes. `FinalizeCode`'s
doc-comment (*"a blocked-finalize finding code"*) gains the landed-advisory sentence (N3).

**§5 (S6 → D2.4) · the engagement with the two rules against a shared foreign-bytes code, written.**
`task.rs:826-828` — *"its own, never a sibling door's … the two other doors … destroy different things and
their routes lead different ways"*; and `mint_foreign_bytes_finding` is deliberately
`milestone.foreign-bytes`' third producer because *"a second spelling … would make one state answer two
ways."* Both rules key the code on **the state and its route**. At the two `Displace` doors the state is
one state — *a landed commit, an area kept because a byte could not be moved aside* — reached through one
seam (`unwind_area` on the `Displace` arm), with one route (move the path out by hand). It is **not** the
mint-unwind state (nothing was committed there, and that route says so) and **not** the discard state (a
refusal before anything is removed). One state, one code; two would be the second spelling the rule bars.

**§6 (S1 → D2.4) · the finding's subject is a post-unwind re-read, and the `Err` arm has a disposition.**
`AreaUnwind::Foreign` is a unit variant — it carries no paths, and the shipped precedent names the area
only. And the motivating cell cannot be described from the move's failure set: a succeeding hook writes
into the area *during* the commit, the displacement reports **zero** failures, and the unwind still
returns `Foreign`. So the message's path set comes from **`foreign_area_paths` re-read after the unwind**
(a call to a shipped probe — counted in the ledger below), and the failing move's reason is named **where
one exists**. **The `Err` arm:** the finding fires too, with the fault named — the shipped
*"self-heals"* note is false for an area D3 may have made a residual (§7).

**§7 (S2 → D2 bounds) · two unwind-fault cells, not one.** `unwind_area` removes members in registry
order and the pin is member 0, returning `Err` *at the point of failure, leaving the rest as found*. Fault
**on the pin** → the area stands whole with its pin: listed as active, cleared by `task discard --force`
(plain) or by hand (sub-task, §8). Fault on **any later member** → the pin is already gone: after D3 the
area is a residual at every door, `task discard` does not resolve it, recovery is **by hand**, and §6's
finding is the surface that names it. Both cells are in the axis.

**§8 (B4 → D2.6) · a second producer under `milestone.terminal`, not the shipped one.**
`terminal_milestone_finding` (`engine/milestone.rs:1176`) composes message **and** route inside the
producer; called for a settled sub-task it would say *"milestone `first-sub` is `joined`"* at target
`milestone:first-sub`, routed at a `milestone-record:first-sub` that does not exist. D2.6 keeps the
**code** and mints a **producer** for the condition: target `task:<sub-id>`, message *sub-task `<sub>` of
milestone `<m>` is already `<status>` on the committed record — its working area is a leftover, not live
work*, route `Human` naming the area path to clear by hand. **Reachability after D3, stated:** only §7's
fault-on-the-pin cell reaches it; without the guard that cell commits the false flip, with it the area is
cleared by hand — a route, not a dead end.

**§9 (S3 → D3) · the pin-read producers are kept, and one host path is fixed.** Two of the three
(`start.rs:2220`, `:2345`) carry **no** host path — only `task.rs:1928` and its `malformed base pin`
sibling at `:1929` do. And D3's own predicate is *existence, never a parse*, so a **legitimate** area with
a torn, corrupt or unreadable pin passes it and still fails the read — deleting the arms would drop the
handling for the exact window D3 argues exists. **Narrowed:** a residual no longer *reaches* them;
`task.rs:1928`/`:1929` render through `render::repo_relative` (law 1). Nothing is deleted.

**§10 (B5, N1 → D4) · the conclude phrase is the member's own.** The mold at `repo.rs:417` hard-codes
*"once its conflicts are resolved"* — true for the five shipped members that supply a conclude command,
because each is detected only in a stopped state; **false on the clean pick, the charter's own cell.** The
member supplies its own conclude phrase — *conclude it with `git commit`, which uses the pick's own
message (after resolving any conflicts)* — carried as a per-member qualifier beside `conclude_command()`,
the five shipped members' rendered bytes **unchanged** (asserted). The abandon half is worded to be true
in both cells: on the clean cell `git reset` leaves the picked changes unstaged; on the conflicted cell it
leaves the conflict markers in the working tree (driven by the reviewer) — *keeps the picked changes in
your working tree, unstaged*. Both cells are named in the acceptance.

**§11 (S4, S5 → D5) · the fallbacks' stated reason was false; the fence gets its exemption rows.**
`mint_migration_in_repo` passes `Some(&id)` and never reaches `mint_id` (`start.rs:212`), so it does not
rely on the fallback. After D5's guards, both empty→type-name fallbacks are **production-dead**. They
stay, with their doc-comments saying so and why (a defensive floor under a guarded door; removing them
changes `mint_id`'s total-function contract, pinned by a proptest, for no behavioural gain) — a decision,
not an oversight. The acceptance predicate — *the id is derived from the caller's prose, never
fabricated* — holds over three of `MINT_DOORS`' five rows; the `migrate` row (a path hash) and the re-seed
row (a recorded id) are **`Exempt(reason)` rows**, on the registry's own mold. `start.rs:4409`'s assertion
on the replaced literal joins the tests that move.

**§12 (S8 → D5) · the milestone guard sits after the two reads.** `run_create` opens with
`discover_repo_root` and `jigc_home_or_repo` — both reads. Placed literally at the top, `milestone create
""` outside a repository would answer `write.unslugable-title` instead of M49's one not-in-repo answer.
The guard sits **after those two reads and before the first write**; the stated requirement (*no write
precedes it*) holds.

**§13 (S9 → D2 axis) · the zero-false-fire controls, for both kinds.** Every cell of D2's axis presupposes
foreign bytes; its most dangerous regression is `unwind_area` answering `Foreign` over an **ordinary**
area, which would leave every finalized task standing and mint an advisory on every landed finalize.
Controls, named: an ordinary `task finalize` lifecycle (`start` → `doc create` → `doc rename --task` →
`task validate` → finalize), a **migrate** task (`source`, `source-path`), and an ordinary post-join
milestone area **including `merged/` after D1.4** each unwind to `Removed`, mint nothing, and leave
nothing on disk. The reviewer drove the first by inspection; the increment drives all three before
building on them.

**§14 (N4, N5, N6) · the census, the route token, the ledger.** Three more production `join("tasks")`
sites (`milestone.rs:2563`, `:4714`, `:6000`) are already-resolved-path **consumers**, not
membership-deciders — named so the reader's obvious question is closed. Any path a route embeds inside a
command span renders through `engine::finding::shell_token`; a path in prose is quoted. **The ledger,
corrected — producers were uncounted:**

| kind | count | what |
|---|---|---|
| finding codes | **1** | `finalize.foreign-bytes` |
| enum variants | **1** | the tenth `InProgress` member |
| **finding producers under shipped codes** | **4** | `milestone.terminal` for a settled sub-task item (§8) · `write.unslugable-title` at the milestone/sub-task mint and at `jigc start` (D5) · the residual sentence under `finalize.no-task` · the residual sentence under the two `serial-collision` codes (D3) |
| per-member qualifiers | **1** | the conclude phrase (§10) |
| calls to a shipped probe from a new site | **1** | `foreign_area_paths` re-read after the unwind (§6) |
| new mechanism / widened walk, by the human's halt | **2** | D3 the residual rule · D1 the `merged/` walk |
| registries, dispositions, families, gates, envelope keys, `contract-version`s, schema-hashes, `schema-version`s, corpora | **0** | |
