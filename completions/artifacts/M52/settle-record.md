# M52 — the Settle record

Settled with the human on **2026-09-17**, against the charter
([decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.16 wave (M52)*),
[baseline-ledger.md](baseline-ledger.md) (seven Opus capability-auditors driving the installed
release `1.0.0-rc.15`, none running cargo) and [gap-findings.md](gap-findings.md) (four
gap-detectors, 31 blocking gaps). **Every fork was decided on two honestly-argued cases; none was
self-framed by the proposer** — six independent `robust-advocate` runs argued the robust side (F1,
F2, F3, F5, F6, and one over the five smaller forks), each read-only, each settling nothing, each
driving its own decisive cells on the release binary. **Their cases are archived verbatim beside this
file** — [advocates/](advocates/) — because they were written in the planning scratchpad.

**Every fork resolved robust, and two of the charter's six forks were re-posed before they were
decided** because the baseline falsified their framing: fork 1's *one primitive vs per-site* is
false against the measured shapes (M51's CAS cannot take a directory tree — three mechanisms), and
fork 3's robust arm *as worded* (every byte under the task area) is refused by M46's own `--ignored`
ground, which — measured on a task area — argues *for* the complement-of-writer-set subject. Fork 2's
consent arm was unavailable under the wave's own boundary (any posture consent flag is new
capability), so it was decided as scope. Fork 4 **dissolved into a repair**: two locked docs and the
code's own doc-comment already say the walk keys on `from`; the code keys on `to`; no hash moves.

**Posture.** Every fact this record turns on was driven against `~/.local/bin/jigc` =
`1.0.0-rc.15` at `HEAD = 85ad06c5` (HEAD's production code is byte-identical to that sha). The
advocates drove their own spikes (F1: `unwind_mint`'s loss, the mint's three-file set, the
leading/trailing DEFECT-1 stream at `task finalize`; F2: the revert cell and the **clean**
`merge --squash` cell; F3: the nine-file task area and `displaced/`'s dying at `uninstall`; F5: the
one-alias commit and the `rename --slug` route that lands the harm; F6: the raced JSON stream, the
third reject root shape, the 24 `current_dir()` signatures; small: the `git mv` false green on the
fresh-clone shape, LD-3's uncommitted milestone, `task validate` vs `--dry-run` under a merge, the
`--preview` refusal routing into the defect). Everything else is relayed from the baseline or a
gap-detector and is marked as such in those artifacts. **An agent's report is a lead, not a
measurement** — the rows this Settle turns on were driven by at least one independent agent, and
the gate-record records which.

---

## The claim

> **No byte dies and no third party's bytes are silently discarded at exit 0 behind a committing,
> destroying or moving door; every repository posture and every route a caller can reach answers
> with a code and a followable route; and every surface 1.0.0 pins says what the binary does — with
> each fix complete over its class's axis, machine-checkable.**

**Honest bounds taken at this Settle, stated in the wave's own terms** *(re-labelled bound-or-deferral at [Review amendments](#review-amendments-2026-09-17) §16)*. (1) The posture family closes
over the operations **git 2.54.0** can leave un-concluded as this git writes them; the `REVERT_HEAD`
blindness of `determine_whence()` and the `rebase-apply/applying` vs `/onto` discriminator are that
git's on-disk contract, **declared as a bound, not fenced** against a git we have not run. (2) The
`GIT_DIR` redirect stays declared out (M51 §3), and it is now the **one** declared posture residual —
the `current_dir()` skip is closed (D2), so the claim carries one exemption with a reopening
condition rather than two, one of them an accident. (3) The **departed-doctype** half of C-2 (a
stamped orphan whose doctype is gone from the composition) stays a declared residual — no home
declares it once the doctype is gone, and the honest fix is the namespaced stamp on its existing
trigger (D7). (4) The **behaviour** half of D3's displacement is reversible after 1.0; only the
declared envelope key is one-way, and it is spent deliberately. (5) **Zero schema-hash movement,
zero `schema-version` bumps, zero corpus migrations** — confirmed by the gap pass: `schema_hash`
digests the `Schema` value alone (`manifest.rs:77-89`), no fix below touches a schema, and the one
tempting bump (`milestone-record.base`'s `set:` kind) is a **record correction** (three locked docs
drifted from a true schema), never a bump. What moves instead is **read-contract and wire shape**:
`doc schema` `contract-version` 6→7 (once, pooled), and additive/declared reshapes of the reject
envelope inside the pre-pin window this wave is the last to hold open.

---

## Decisions

### D1 — fork 1: the rollback family — the registry, three disciplines, the door-keyed code

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §1 · §2 · §14]*

**The fork.** *Cheap:* per-site CAS at the six file populations, directory trees left on
`finalize.md:195`'s carve-out, `finalize.md:174` struck to match the binary, one conflict code,
DEFECT 1 suppressed under `--format json`. *Robust:* a rollback-population registry with a stated
discipline per population; the shipped CAS generalized; the conflict code takes its door; the
promise extended to every jigc transaction; the finding on the reject envelope.

**The advocate's decisive argument** ([advocates/F1.md](advocates/F1.md) §1, §4, §5): driven,
`unwind_mint` destroys third-party authored prose in the gitignored task area behind a frame that
says *nothing was committed … the milestone is unchanged* — and `finalize.md:195`'s warrant (*files
this run never saw*) is false verbatim there, because the directory was created three lines up by
the same call. *Remove only what this call minted* needs **no capture**: a mint writes exactly three
constant files (`state.rs:633-649`), so remove those and non-recursive `remove_dir`; `ENOTEMPTY`
means a third party is present — leave the bytes, name them, route (the idiom at `task.rs:4897`).
And the cheap arm's DEFECT-1 fix leaves DEFECT 1 reproducing at the identical argv: four producers
route to stderr on the premise *"the structured envelope owns stdout"*, true on success and false at
reject, and a grep for format-blind sites cannot see them.

**Decided (the human, 2026-09-17) — the robust arm:**

1. **`ROLLBACK_POPULATIONS`**, a code-side registry, one row per population (nine worktree-restore
   populations + the rollback-less `config set <root-knob>` relocation, [baseline-rollback.md](baseline-rollback.md) §1),
   each carrying its discipline: **`FileCas`** (the M51 restore, generalized) · **`MintedSet(<the
   mint's own constant list>)`** (`unwind_mint`, `unwind_unrecorded_seeds`) · **`DoorGuard(<the
   named guard>)`** (`setup`'s pre-write refusal; `rollback_rename`'s HEAD-sourced arm under
   `rename.dirty-tree`) · **`Declared(<reason>)`** (`CreatedDoc::rollback`, whose window is
   intra-process). Fence: a **counted source scan** on the `repo_relative_paths.rs:849` mold over the
   production restore functions **plus the two non-function restores** (`Drop for RecordFlipGuard`,
   `rollback_promotions`' inline `fs::write`) — no clap tree bijects this class, and the record says so.
2. **The shipped CAS is generalized, not rewritten**: `spec: &'static str` → a runtime identity used
   for the family key and the park filename; `ConfigLayerHome` → an absolute path; the ~150-line
   `restore` body (`task.rs:4190-4243`) unchanged. Applied to promote · retire (both arms — the
   already-absent arm stops swallowing a re-created path) · `RecordPreImage` (all **six** doors, G-4)
   · `RecordFlipGuard::Drop` (both `squash` arms) · `rollback_rename`'s two unguarded arms · the
   record path at `milestone create` (the absent-pre-image arm stops deleting a racer's file).
3. **The conflict code takes its door**: `<door>.rollback-conflict` — `finalize` keeps
   `finalize.rollback-conflict`; `rename`, `milestone` (create/add-task/add-from-spec/finalize/discard
   and `task discard`'s record arm), `config`, `doc-author` mint theirs on the same mold. Scope: fires
   at the door's reject path, one blocking finding per conflicting path, carried on the reject
   envelope (D6). The stable `(code, target)` key discriminates a raced rename from a raced finalize.
4. **`MintedSet` unwind** at both `unwind_mint` sites and `unwind_unrecorded_seeds`: remove the
   mint's own files, then non-recursive `remove_dir`; `ENOTEMPTY` ⇒ the area survives, one blocking
   finding names it with a `Human` route. Honest cost: a re-opened `serial-collision` on the raced
   re-run — a named block replacing silent destruction. `finalize.md:195`'s carve-out is **re-scoped**
   to *a directory jigc did not create in this transaction*, with the falsifying datum quoted.
5. **`config set <root-knob>`'s relocation gains a rollback** (population 10, baseline L8): the
   staged `git mv` + file-state re-key are undone when the knob write fails.
6. **C4's class — every door that amends a jigc-owned file before its own validation can fail**
   (`milestone provision`, `milestone create '<existing title>'`): the amend moves **after** the
   refusal point, or the failed run acks it. Decided: **after** — an amend that only a success
   needs is made only by a success.
7. **DEFECT 1 as a class**: the stream rule is stated once — *a finding produced on a reject path
   folds into the reject document* — and applied at all four `Format`-routing producers
   (`emit_left_out_advisory`, `emit_carried_advisory`, `relay_hook_output`,
   `carry_rollback_conflicts`) plus `config set`'s relocation prose. Rides D6's envelope.
8. **Docs, riding their increments:** `finalize.md:200`'s all-or-nothing promise reads *every jigc
   transaction*; `:174` states the CAS rule for retire (built, not struck); `:193`'s door count
   **becomes the registry** — G-4's three-way disagreement (four / five / six) dies with it;
   `reconciliation.md:144`'s *"exactly four sites"* struck with its datum; `storage.md` → Concurrent
   writers gains one sentence stating what it forbids (a subprocess inside the save-lock critical
   section) and what it does not (capture-and-compare before a restore) — the reading M51 §2 took
   is about jigc processes, and its own reopening condition is satisfied at HEAD by the hook and the
   fan-out.

### D2 — fork 2: the posture family — refuse under any un-concluded operation, in full

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §4 · §16]*

**The fork.** Consent unavailable; scope only. *Cheap:* the closed family kept; D1's mask order,
D2's `am` discriminator, D3b's cause vocabulary, `--carry-staged`'s noun. *Robust:* every acting
door refuses under **any** un-concluded git operation; a per-operation route table; the ordering
fix; the `current_dir()` skip closed; `relocate.rs:314` re-probed; `task validate` gains the posture
row **and** probe.

**The advocate's decisive argument** ([advocates/F2.md](advocates/F2.md) §1, §2, §5): the cheap
arm's premise that membership is *new surface* is false — M51 Increment 2's locked deliverable
(`roadmap.md:2548`) is universally quantified over *repository posture*; `finalize.md:31` and
`validation.md:663` are inventory rows one member short of their own rule. Driven: a **clean,
non-conflicting `git merge --squash`** followed by `jigc milestone create` exits 0 and deletes
`SQUASH_MSG` — the ordinary path, damaged by a pathspec door. The widening lands inside the existing
enum with **zero call-site changes at 12 doors** because M51 built the seam correctly; and M51's own
D2 says refuse→consent is additive after the pin while narrate→refuse is breaking.

**Decided (the human, 2026-09-17) — the robust arm, in full:**

1. **`InProgress` widens** to the operations git can leave un-concluded — merge · rebase (merge
   backend) · rebase (`--apply` backend) · `am` · cherry-pick · revert · a multi-commit `sequencer/`
   · squash merge (`SQUASH_MSG ∧ ¬MERGE_HEAD`, pure filesystem) · a conflicted index with no
   operation (`git ls-files -u`, driven 13 ms) — each variant with its own `detect`, `noun` and
   concluding/abandoning command, `markers()` generalized to a per-variant predicate. The
   `rebase-apply/applying` vs `/onto` discriminator names `am` vs `rebase`. **Every acting door**
   (`BEHALF_DOORS`'s commit-on-behalf and move-on-behalf members) refuses `repo.operation-in-progress`
   under every member; the route names the operation's own git command (`validation.md:663`'s *never
   a menu*). `repo.rs:1-16`'s no-shell-out doc-comment is **revised to the scope it binds**
   (`worktree_git_dir`'s walk-up) with the two shipped shell-outs cited.
2. **`posture()` reports the operation before the detached HEAD it causes** (D1's three masked cells,
   incl. `rebase --apply`); `HeadDetached ∧ OperationInProgress` renders the operation.
3. **D3b's cause vocabulary**: the `CommitRejected` frame says *hook* only when a hook ran; git's
   own refusals (partial commit during a pick, unmerged index, `gpgsign` failure) name git, with the
   stdout/stderr separator (`task.rs:5433`) fixed.
4. **`refuse_on_posture`'s `current_dir().ok()?` (`cli.rs:540`) becomes a refusal** — a door cannot
   act on behalf of a repository it cannot name — inside D6's pre-dispatch funnel. **`GIT_DIR` is
   stated as the one declared residual** with its reopening condition, in the claim's bound.
5. **`relocate.rs:314`'s `git rm --cached` re-probes** (`SeamAct::Move`) — the last of 11
   index-mutating sites.
6. **`task validate` previews posture**: one `GATE_COVERAGE` family row + `preview_gates`
   (`task.rs:1736`) calling the shipped `repo::posture`. This is a **declared exit flip** on the
   precedent of M50's own deliberate flip (`DECISIONS.md:2621` bound iii) — a **revise** of M47 D1's
   ground, not an override, and the datum that decides it: `task finalize --dry-run` already refuses
   under a merge, so the product ships two previews of one door that disagree.
7. **Docs, riding their increments:** `finalize.md:31` and `validation.md:663` restate the family as
   *any un-concluded operation* with the member inventory generated from `InProgress::ALL`;
   `worked-examples.md:3445`'s *"three of four members"* struck (a member count where the failure is
   a state count); the git-2.54 bound written at both homes.

### D3 — fork 3: the destroying-door subject — the writer-set complement, and finalize displaces

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §6 · §7 · §8 · §14]*

**The fork.** Both charter arms refused by measurement. *Cheap:* the complement subject at the three
consenting doors; `task finalize` narrates what it destroyed. *Robust:* the same, plus `task finalize`
**displaces** the complement instead of destroying it, names the move on text and envelope,
`uninstall` gains a `displaced/` row, and `task finalize` + `task discard` join `DESTROYING_DOORS`.

**The advocate's decisive argument** ([advocates/F3.md](advocates/F3.md) §2, §3, §5): driven, a
fresh task area after `start` + create + rename + validate holds exactly nine files, every one
jigc's — the complement's false-fire rate on the ordinary path is **zero**, so M46's `--ignored`
warrant (a worktree that did its job holds build output) does not transfer; the landed-boundary
warrant for narrating (*the staged set is already in git*) is structurally unavailable because the
commit takes nothing from the area; the cheap arm ships the asymmetry the human adjudicated
unacceptable at M50 (`discard` refuses over bytes `finalize` takes) one predicate over; and either
arm spends the pre-pin window on a key — `discarded:` pins destruction, `displaced:` pins keeping.

**Decided (the human, 2026-09-17) — the robust arm:**

1. **`TASK_AREA_FILES`** — jigc's own task-area writer set as one public registry (the 12 scattered
   consts: `state.rs:32,37,42,51,60,68,77,215,788`, `validate.rs:66,71`, `migrate.rs:63`, incl.
   `renames.json` and the two probe snapshots the baseline's list missed, G-13), with a counted
   source-scan fence over `<dir>.join(<literal>)` in both crates. **The destroying-door subject in a
   task area is the complement of that set.** Written into `design/team-ready-state.md` as the rule,
   with `design/finalize.md:137` taking the qualifier — the rule is stated in no locked doc today
   (G-12), so the wave writes it, arguing leg 0.
2. **The three consenting doors** (`task discard`, `milestone discard`, `uninstall`) refuse over a
   foreign byte without `--force` — `<door>.foreign-bytes` on the `<door>.staged-prose` mold, one
   blocking finding naming every path, `--force` the single consent (M48) — and narrate every path
   destroyed with it through the outcome-keyed loss emitter.
3. **`task finalize` never destroys a foreign byte**: the complement moves to
   `.jigc/displaced/<task-id>/` (relative paths preserved; the shipped `fs::rename` idiom of
   `relocate::displace_foreign_squatter` / `task::park_pre_image`) before the teardown, named through
   the loss emitter **and** as one declared additive key `displaced` on the landed envelope — the
   act the pre-pin window permits. Skipping the teardown is refused (a left-over area makes
   `task list` report an active task, driven).
4. **`uninstall` gains a `displaced/` row** — today a file there dies at exit 0 by declared design
   (`setup.rs:3038-3046`); it becomes a narrated destruction under `--force` and a refusal without.
5. **`task finalize` and `task discard` join `DESTROYING_DOORS`** with M46's refuse-vs-narrate
   discriminator; the registry's subject is generalized from *worktree-shaped path* to *destroyed
   path* (priced: `flow49_acceptance.rs:999-1051` grows with it). **The two sinks
   (`remove_worktrees`, `cleanup_subtask_areas`) take the same on-disk subject as the guards**, not
   the registered set (G-15).
6. **The classifier fails closed** (L-1): `leftover_at`'s error arm returns the fail-closed verdict
   its doc-comment claims, so `milestone discard`/`provision`/`uninstall` hold over an unreadable
   root; `workbench_paths`' name match gains the shape check (L-2); `staged_doc_ids` asks
   `is_file()` (L-3).
7. **The four siblings**: D-1's route names `task discard <id> --force` over a milestone sub-task
   and states that it settles the record; D-2's `workbench removed` ack keys on the outcome; D-3's
   host path goes through `render::repo_relative` and the `UNSWEPT_PRODUCERS` reason is corrected;
   D-4's route names `read_dir` on `.jigc/worktrees/` and `--force`, `project-setup.md:160` struck
   with its datum.

### D4 — the corpus walk: a repair, keyed on `from` through the snapshot store

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §14 · §17]*

**Not a fork.** `corpus-migration.md:68` and `:299` state the walk keys on the `from` home, the
code's own doc-comment (`migrate_corpus.rs:2072-2074`) says *the union of every home the doctype has
ever declared*, and `candidate_docs` unions prior `location:` homes only (`:2104`,
`.filter_map(|p| p.location)`), loading no snapshot at all on the location branch (`:2080`). The
prior home is already recorded in full — `schema-snapshots/<ty>.v<k>.yaml` is the whole prior
`Schema`, fenced complete in both directions (`snapshot_store_two_snapshots.rs:168`). **No hash
moves.**

**Decided:**

1. `candidate_docs` walks **every prior home of every kind** — prior `location:` *and* prior
   `placement:` (rerooted through `placement-root`, so `DoctypeMigration` gains `placement_root`
   beside `docs_root`; the primitive is `start.rs:3414/:3430`) — so all four `{L,P}²` cells are
   covered and the bump kind is irrelevant.
2. **A missing snapshot fails closed at the enumeration**, not only at the fold: the
   `.filter_map(|k| load_prior_schema(…).ok())` becomes `migrate-corpus.missing-snapshot`
   (blocking) — `corpus-migration.md:90`'s *never a silent already-current* made true in both
   directions.
3. **The adopter-visible consequence is stated**: a below-version doc at a prior home that the walk
   now sees flips `validate` 0→1 via the existing `schema-version-current` — the intended
   tightening, written in MIGRATING.md's batch.
4. **D-3's arm**: `ValueRemapped` on an `id-from` enum takes the plain-enum `fold-refused` arm, never
   `prose-needed`, and **never re-mints the item id** (G-19: the act `retitle-item` refuses).
5. **`relocate`'s refusal route** stops naming `migrate-corpus` for a state `migrate-corpus` did not
   answer — it answers now, so the route becomes true rather than repaired.
6. **The relocation-route family** (every route naming `migrate-corpus`) is swept: each fires in a
   state the walk now answers, or is re-routed.

### D5 — fork 5: one fixed-identity predicate at the parse boundary, and the projection tells it

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §9 · §10 · §15]*

**The fork.** *Cheap:* eight door guards keyed on `placement.is_some()`; the projection stays at 6.
*Robust:* one predicate `placement.is_some() || singleton` at the parse boundary + three siblings;
`rename --slug` refuses; `doc schema` projects the home fact (`contract-version` 6→7, pooled).

**The advocate's decisive argument** ([advocates/F5.md](advocates/F5.md) §1, §2, §5): between a
bogus-head write and finalize **no address reaches the staged doc**, so M48's read-back fence is
defeated structurally; `rename vision:alpha --to Phantom` refuses and prints a route composing the
caller's bogus slug, and that route commits a rewrite of the real `VISION.md`; `parse_verb_addr`
already holds the pack and is the one funnel for all nine `doc.rs` doors, so robust is **four homes
against the cheap arm's eight edits**; and `doc schema` is the only channel that can tell a driver
`vision:alpha` is not an address — `doc-read-surface.md:108` forbids additive carve-outs, `:90`
closes at the pin.

**Decided (the human, 2026-09-17) — the robust arm:**

1. **One fixed-identity predicate** — `placement.is_some() || singleton` — in one engine home,
   shared by the shipped read guard (`store.rs:254`) and the door guard; `cli::doc::is_singleton_type`
   (which never reads `Schema::singleton`) retired into it. `storage.md:216` **revised** with the
   driven converse quoted (a `location:` + `singleton: true` doctype creates and promotes at exit 0
   while its read refuses).
2. **Landed at the parse boundary** — `parse_verb_addr` (`doc.rs:6120`, nine call sites) plus the
   three doors that resolve schema separately (`rename.rs:986`, `milestone.rs:1410`, `task.rs:2687`)
   — **not** inside `canonical_path` (the question is a door question; the resolver has legitimate
   internal callers). A well-formed head that is not the doctype's fixed identity refuses
   **`store.fixed-identity`** (blocking; route: the canonical address), at read and write doors alike;
   `store.malformed-slug` stays where M50 put it.
3. **`rename --slug` refuses on the same predicate**, and `write.identity-change`'s route stops
   composing the caller's token into the escape hatch.
4. **`doc schema` projects the identity/home fact** from the shipped `projection_home_line`
   (`compose.rs:1194-1215`) — `contract-version` 6→7, **once**, pooled with LD-2's `base`
   projection (D6.5). Not a schema-hash move (`manifest.rs:77-89`).
5. **The five methodology author steps** (`author-vision`, `author-roadmap`, `author-decisions`,
   `author-decision`, `author-ledger`) stop calling `doc schema <ty>` *"the authority … every address
   a write can take"* on a fixed-identity doctype; the sentence renders the fixed address.
6. **A1-D4 rides the same seam**: the OS name ceiling at the address head refuses with
   `SLUG_NAME_CEILING_CODE`'s mold at the parse boundary, so the six write doors stop leaking a bare
   OS error. **A1-D5 both knobs**: `ROOT_KNOBS` gain the nameability leg, and `placement-root`'s
   per-doc move failures stop landing inside a success ack (the knob refuses, `config.unusable-root`).
7. **`milestone-record.base`'s `set:` kind is a record correction**: the schema (`set: on-create`,
   `milestone-record.yaml:62`) is true; `doc-read-surface.md`'s settability table,
   `write-commands.md:84` and `DECISIONS.md:6397` are struck with the datum. **No bump.**

### D6 — fork 6: the reject envelope carries findings, one pre-dispatch funnel, the arms declared

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §3 · §12]*

**The fork.** *Cheap:* declare what exists; DEFECT 1 suppressed under json; `pack.rs` silenced.
*Robust:* `{error}` gains optional `findings`; the pre-dispatch funnel over 26 sites; `setup`/
`uninstall` onto the findings arm; `doc show`'s rows + `ArmShape`; LD-2 rides D5's bump.

**The advocate's decisive argument** ([advocates/F6.md](advocates/F6.md) spikes 1–4): a raced
`task finalize --format json` leaves a driver crashing or silently losing the only line naming the
parked pre-image; the 24 `current_dir()` sites all sit in functions already holding `format`, so the
fix is 24 one-line substitutions onto the shipped `operational_failure(format, &err)`; `SCHEMA_VERSION`
is read by nothing that checks it and `findings` is additive so no bump is owed; the 21 `{error}`
rows per cell are bare `anyhow!` with no finding behind them — `{error}` is correct there.

**Decided (the human, 2026-09-17) — the robust arm:**

1. **The `{error}` reject arm gains a declared optional `findings` (+ `schema_version`)** —
   additive, declared in the same motion under `command-output-contract.md:476-494`'s pre-pin rule,
   no `SCHEMA_VERSION` bump. `carry_rollback_conflicts` and `config set`'s relocation prose fold into
   the document (D1.7).
2. **`setup`/`uninstall` reject on the `{findings, schema_version}` arm** — `render::setup_block`
   (one function), two call sites, one unit test; the `(code, target)` key kept.
3. **One pre-dispatch funnel**: the 24 `current_dir()` sites substitute onto `operational_failure`;
   `refuse_on_posture`'s fault becomes a refusal (D2.4); `pack.rs:1786/:1793`'s warnings ride a
   `OnceLock<Format>` seat set in `main` after `try_parse()`, print a repo-relative path, and
   `repo_relative_paths.rs:746`'s falsified reason is struck. Fence: a counted source scan on the
   `UNSWEPT_PRODUCERS` mold. Scope: every leaf, every pre-dispatch failure point, one document.
4. **DEFECT B**: +3 `ENVELOPE_ARMS` rows and the new `ArmShape` member(s) for `doc show`'s
   array-of-items, array-of-strings and object-leaf projections; `ArmShape::ArrayOf`'s and the four
   rows' false doc-comments rewritten; `doc-read-surface.md:71-78` unchanged (it is right).
   **`task list` stays an array** (`:181`'s reason holds).
5. **LD-2**: the projection marks `base` as the compound it is (`{sha, short}`) in D5's 6→7 move.
6. **DEFECT C** is a comment fix on `ArmOutcome::Success`; the taxonomy (`:457`) is not reworded.

### D7 — C-2 split on its seam: the declared-home emptiness check; the departed half declared

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §5]*

**The fork.** *Restate as declared* vs *re-take via `Territory`*. The advocate found neither right
([advocates/small.md](advocates/small.md) (i)): driven on the shipped pack, `git mv CHANGELOG.md
HISTORY.md` + commit and the fresh-clone shape (`.jigc/state/` gitignored) → `validate` exit 0,
`doc list` drops `changelog` entirely; and D4's walk repair does not reach it (the stranded file is at
no resolved home). Widening `Territory` to exact root files misses `HISTORY.md` too.

**Decided (the human, 2026-09-17) — split on the seam:**

1. **The declared-home half is re-taken**: a new store-scope check, **`schema-conformance.home-vacated`**
   — at each resolved doctype home whose committed **history** is non-empty (`git log HEAD -1 --
   <path>`, the shipped M45 history-gate predicate) while the index holds nothing there — blocking,
   joining `STORE_EXIT_FLIPS` as its **seventh** member (an adopter-visible exit change, stated in
   MIGRATING.md's batch). It names one exact path per declared home; no team document can be its
   subject; `Territory` moves by zero bytes, so M51's HIGH narrowing is not re-opened. Route: `jigc
   ingest`/`jigc rename` as the state indicates, or `jigc unmanage` for a genuine retirement.
2. **The departed-doctype half stays declared** (`validation.md:710` rewritten to state the half it
   covers and the half it does not, with L-5's datum), on the namespaced-stamp trigger at
   `decisions-pending.md:309`.

### D8 — tier 0 admits by the predicate: the four un-charted exit-0 cells join

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §13]*

**Decided (the human, 2026-09-17):** the tier-0 predicate binds on the **condition**, not on which
instrument found the cell. **LD-3** — `milestone create`/`add-task`/`add-from-spec`/`provision`/
`join`/`finalize`/`discard`/`list-tasks` refuse over a repository with no `jigc setup` with the
shipped `locate::not_set_up()` answer `milestone execute` already gives (the fence: every leaf that
reads or writes the workbench refuses that state, derived from `VERB_KINDS`). **L-1**, **L-2** ride
D3.6. **`rename --slug`** rides D5.3. The **surface-tier latents** go to tier 2 where they are
one-liners (the `""`-acked-`= .` root knobs · `x/../y` normalized silently · an unreadable `.jigc/`
called *not set up* · the fixed `workbench removed` string) and otherwise defer with triggers.

### D9 — the composed off-verb lies: refuse where unreachable-as-composed, state the empty case

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §11]*

**Decided (the human, 2026-09-17) — two rules:** (1) **the two compose doors** (`start --workflow`,
`workflow --preview`) refuse a workflow the pack declares verb-routed — the declaration exists
(`suppressed.reason: verb-routed — reached only through …` on all 12 `migrate-*` workflows; the
`sub-task` and `milestone-execution` members are declared the same way in this wave) and becomes a
**structured key on the `suppressed` block** (name owed to the plan; fenced at pack-load beside
`suppressed`'s own fence) whose value is the real door's argv, rendered as the refusal's
`Mechanical` route — **`workflow.verb-routed`**, blocking, exit 1; the `--preview` refusal that today
routes *into* the defect is re-aimed the same way. (2) **The legitimately-empty renders** (A6-3's
class: `implement-from-spec` over a spec-less corpus, and every `{{@…}}` enumeration the surfaces
auditor listed) state the empty case on the pack's own shipped shape (`single-task`'s supersedes
slice, `form-vision`'s grounding). (3) **`is_migration` gains its off-verb arm** so plain
`task finalize` on a migrate-shaped task without a staged source does what the composed body says
(the exit-4 hold, never a promotion). `surface-contract.md:202` moves in the same increment.

### D10 — the absorb surface, the orphan message, the placement identity, `relocate`'s predicate

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §14]*

**Repairs, not forks, each under a locked rule:**

1. **`ingest` and `rename` say what they absorbed** (`reconciliation.md:186`): one advisory
   `file-state.absorbed` at both doors, on text and envelope, naming the finding it retired.
2. **D-4's message** says what the producer computed — *no resolved doctype claims this path* — and
   routes as `ingest` already does; the family's seven producers across four doors are swept.
3. **C-1**: `ingest`, `unmanage`, `doc show`, `rename` and the edge index agree on a placement
   doc's identity — the canonical id is the doctype id (`stem(placement.file) != ty` for `vision`
   and `changelog`); `edges.json` holds one identity; `unmanage` drops the edge it owns.
4. **`relocate` asks the unaddressable-identity predicate** `ingest` asks (`validation.md:723`'s
   *one predicate, not two agreeing ones*) — `ingest.unaddressable-identity`'s mold at the relocation
   door.
5. **A1-D2/A1-D3**: `relocate --from` refuses the workbench and adapter roots with the sibling
   doors' codes; the `relocate`/`from` registry row states its reachability (a manufactured-pack arm
   is its only witness on a stock corpus), and `corpus-migration.md:75-81`'s freeze-exempt path
   states its empty domain.

### D11 — the surface tier, and the counts

The verified tier-1/tier-2 rows not decided above land as one batch each, keyed to their review
row: CX-1's two unqualified `--dry-run` claims (+ `validate --help`'s *read-only*) qualified against
the invocation log; CX-2's three sentences in two homes (one generated at `setup.rs:80`) made true;
CX-3's install line scoped to the source tree; D-1's text manifest arm renders **every** advisory
(the class, not F-9's two); D-2's four committing doors say so in `--help` and the two silent acks
print their sha; A6-1's sub-task `resume:` line discriminates provisioned-in-worktree from
provisioned-in-shared-checkout and the boundary's loss narration sweeps the shared-checkout-staged
axis; the phantom `reconciliation.rename` at `task validate` in the shared checkout (baseline
surfaces defect 3) keyed on the record commit. **The guide bytes land in one batch and one hash
move**, and that rule is **written** at `assistant-adapter.md` (G-29). **The design homes join
`count_fences.rs`** on the `doctype_map_versions` mold (G-30): every count this wave moves is
fenced or generated.

### D12 — acceptance: flow 53 arms per class, the per-axis review re-run, the fixture builder

*[Amended 2026-09-17: see [Review amendments](#review-amendments-2026-09-17) §7 · §12 · §15]*

**Decided (confirmed at Scope):** [acceptance-design.md](acceptance-design.md) — seven arms, each
iterating its class's axis from a code-side registry (`ROLLBACK_POPULATIONS` · `InProgress::ALL` ×
`BEHALF_DOORS` · `TASK_AREA_FILES`'s complement × `DESTROYING_DOORS` · the `{L,P}²` home-pair set ·
the fixed-identity doctype set × the address doors · the pre-dispatch failure-point registry ×
`VERB_KINDS` · the verb-routed workflow set × the compose doors), **plus the per-axis review re-run
on the installed `1.0.0-rc.16`** with M51's preserved instrument, compared row by row. **A
git-in-progress fixture builder** joins `trial_corpus.rs` / `dev/jigc-rig` as its own increment
(the posture auditor built eight states by hand). `chmod 000` arms declare the CI platform bound.

### D13 — open leads, dispositioned

Lead 3 **discharged** (built + proven; the boundary commit created-and-orphaned before the live
refusal is recorded as a bound). Lead 1's trigger **fired** at M51's audit (the span fence on all
three `Route` constructors) — the debug-posture seam suite is owed in D12's arm 6. Lead 2 fires
(`staged_doc_ids` — D3.6). Lead 5 fires (A6-1/2/3 move composed lines — D9/D11 re-pose the JSON-arm
declaration rather than inherit it: the task-state lines stay text-arm-only, **stated**). Lead 6a
fires (`task bind`'s two bare-`anyhow` refusals join the route floor in D11's batch). Leads 4, 6b,
6c, 6d stay open with their triggers.

---

## What the Settle refused, with the citation

- **A posture consent flag** — new capability (the boundary); M51 §3 withdrew `--force` for the
  family; and a `git commit` under a pick/revert/merge *is* the conclusion, so consent could only mean
  *conclude the user's operation*, which no flag should say.
- **A `--force` on `task finalize`** — new capability; D3 displaces instead.
- **A guard inside `canonical_path`** — wrong altitude (a door question), 25–35 call sites on pinned
  paths with no measurement behind changing them.
- **Widening `orphan::Territory`** — re-opens M51's HIGH and misses the moved file anyway (D7).
- **Reshaping `task list`'s array** — `command-output-contract.md:181`'s reason holds.
- **Any schema-hash move**, incl. the tempting `milestone-record.base` bump — a record correction.
- **Rewording the 14 verb-routed step texts** — structure delegated to prose across 14 surfaces,
  branching on a gitignored file the reader cannot see (D9).
- **A `pinned-by:` parser** — `pinning.md` §3, unchanged.

## Owed at decompose

- Risk-first: D1 and D3's loss cells and D2's exit-0 conclusions first; D8's LD-3 with them.
- **Every doc move rides the increment that makes it true** (M51's rule); D11's guide batch is one
  increment and one hash move.
- **The gate-record's rows are diffed against the roadmap's Grouped-scope lines before the cut is
  called done** (the M51 §21 lesson, now in the planning workflow).
- `foldback_truth.rs` re-aimed to `**M52 —` at the close, inverted at completion.
- The bump to `1.0.0-rc.16` after the audit's fixes, never before.

---

## Review amendments (2026-09-17)

Two independent readers — an Opus `design-reviewer` driving the release binary
([design-review.md](design-review.md): 4 blocking · 7 significant · 10 advisory) and an unseeded
Codex source review ([codex-design-review.md](codex-design-review.md), prompt at
[codex-design-prompt.md](codex-design-prompt.md): 4 · 8 · 6) — both returned **not ready to
decompose**. What held under both: the zero-schema-hash claim (`manifest.rs:52-89`), `parse_verb_addr`
as the one funnel for the nine `doc.rs` doors, `projection_home_line`'s three home shapes, the 24
`current_dir()` sites already holding `format`, the M45 history-gate predicate's transfer, and — against
the charter's own worry — **D5 does not redden the `doc_read_surface` law-1 fence** (that fence drives
advertised addresses through `Address::parse`, which D5 leaves untouched). **The human accepted every
finding on 2026-09-17**; the corrections below ride a dated bracket at each decision they touch, and no
decision above was rewritten in place. **Read a decision and its bracket together.**

### §1 — D1.2: the CAS body cannot be unchanged (Codex B1 · Opus A1)

`ConfigLayerWorktree::restore` is typed over `ConfigLayerPreImage` (`task.rs:4092`, `:4123`), calls a
helper that hard-codes `finalize.rollback-conflict` (`:4301`) with *"this finalize"* in its route
(`:4286`, `:4297`), and parks by the **last path component** (`:4253`) — so two populations sharing a
basename would collide. **Struck:** *"the ~150-line restore body unchanged"* (it is 51 lines and cannot
be). **Decided instead:** a **generic pre-image entry** — `{identity, absolute path, pre-image bytes,
post-write bytes}` — with **injected door metadata** (the door's code, the door noun for the route
prose, a collision-safe park identity `.jigc/displaced/<door>/<identity>.pre-image.<nanos>`); the
compare-and-swap logic (absent-pre-image · unreadable · `pre == post`) is what transfers. **Rows the
generic entry serves:** promote · retire · `RecordPreImage` (six doors) · `RecordFlipGuard::Drop` ·
`rollback_rename`'s two arms · the `milestone create` record path · the config layer (the shipped
population, re-keyed onto the generic entry).

### §2 — D1.4: `MintedSet` is the door's jigc-written area set, and every unwind branch is dispositioned (Opus B1 · Codex S3)

Driven by the reviewer with a listing hook: at `milestone create`'s unwind the area holds **four** files
— `base.json` + `tasks.json` (the mint, `engine/milestone.rs:283-296`) **plus** `staged-snapshot.json`
(`cli/milestone.rs:562-568`) and `record-commit-msg.txt` (`:737`), written by the door **after** the mint
— so *"the mint's own constant list"* would report `ENOTEMPTY` over jigc's own bytes on every rejected
run and the re-run would block on `milestone.serial-collision` forever. The `add-task` sibling transfers
as stated (driven: exactly `intent` · `workflow` · `base.json`). **Decided:** the `MintedSet` row names
**the door's jigc-written area set**, read from the same registry D3.1 mints (the task-area set for
`add-task`/`add-from-spec`'s sub-task areas; the milestone-area set — `base.json` · `tasks.json` ·
`staged-snapshot.json` · `record-commit-msg.txt` — for `milestone create`), never a hand-list. **Every
unwind error branch is dispositioned:** `ENOTEMPTY` ⇒ the area survives and **`<door>.foreign-bytes`**
(D3.2's code, target the area path, `Human` route naming it, findings arm, exit 1); any other
`remove_file`/`remove_dir` error (permission, a file where a directory was, one owned file already gone)
⇒ `operational_failure` naming the path, the area left as found. `unwind_unrecorded_seeds`' set is the
N sub-task areas, each unwound by the same rule.

### §3 — D6.1: `{error}` stays unchanged; a reject carrying findings takes the findings arm (Codex B2)

`Reject::Error` is an **ad-hoc root** that carries no `schema_version` because that contract does not
version it (`render.rs:5157`, `:6001`); `Reject::Findings` is the result-contract root (`:6014`). Adding
`findings` + `schema_version` to the error arm would make it half-impersonate the findings arm and put a
result-contract version on a non-result root — not an addition the pre-pin rule cures. **Decided
instead:** `{error}` is untouched. **A reject that carries a finding is emitted on the
`{findings, schema_version}` arm, with the operational error itself rendered as a finding** — for the
hook-rejected commit that is the shipped **`finalize.commit-rejected`** identity (the survivable frame's
own code, already in the invocation log), so the document is `{findings: [commit-rejected,
rollback-conflict…], schema_version}`; for `config set`'s relocation prose the same shape under its own
door code. The four `Format`-routing producers (D1.7) fold into that document. `ENVELOPE_ARMS` records
the doors that move from `Reject::Error` to `Reject::Findings` in the same motion (the M51 Increment 6
act). **No `SCHEMA_VERSION` move.**

### §4 — D2.6: posture at `task validate` — invoked separately, finalize's exit, the preview boundary revised (Opus B2 · Codex S1 · Opus A10)

`Door::Previewed`'s doc-comment promises *"same check, same severity, same exit code"*
(`gate_coverage.rs:91-94`); posture refuses at `finalize` through `operational_failure` at **exit 1**
while a blocking preview is **exit 3** — the record named no exit, and three different builds follow.
And `command-output-contract.md:470` / `finalize.md:55` declare the preview boundary as *"up to the
commit"* / *the shared validation phase, not the phases around it* — posture is a pre-commit phase those
sentences exclude. **Decided:** posture is **not** folded into `preview_gates`; `task validate` invokes
`repo::posture` **separately** at the door, renders the identical `repo.operation-in-progress` /
`repo.head-detached` finding, and exits **1** exactly as `finalize` would — *same check, same severity,
same exit*. The `GATE_COVERAGE` row is added with `Door::Previewed` and a `tier` note naming the
separate invocation. `command-output-contract.md:470` and `finalize.md:55` are **revised as a declared
spend**: the preview reports the one pre-commit phase a caller can resolve before finalizing — posture —
and no other. The adopter-facing sentence lands in D11's guide batch.

### §5 — D7: `home-vacated` fires on exact declared paths only, never a directory (Opus B3 · Codex S7 · Codex A2)

*"Each resolved doctype home"* read as a directory would flip `validate` red on a corpus that
legitimately retired its last ADR. **Decided:** the subject is the set of **exact declared paths** — a
placement doctype's resolved `placement.file`, and a `location:` singleton's `<location>/<ty>.md` —
i.e. the fixed-identity homes D5's predicate names; a `location:` collection directory is never a
subject. Git-command failure inherits the shipped consumers' conservative reading (*history present* ⇒
blocking, `task.rs:1835`). Target: the path; route: `Human`, naming `jigc ingest <path>` (the file moved
and is adoptable), `jigc rename` (the identity moved) or `jigc unmanage` (a genuine retirement) by
state; findings arm; exit 1 via `STORE_EXIT_FLIPS`. The distinction from `orphaned-instance` is written
at `validation.md`: one concerns a still-resolved declared home, the other a stamped file no resolved
doctype claims. Arm 4 iterates that home set (§15).

### §6 — D3.1: the subject rule is over the tree, with a `docs/` cell (Opus B4)

The 12 registry consts are filenames at the area root; the record had no cell for the `docs/` subtree,
so read literally it flags `docs/` on every task and leaves C-1's own cell (`docs/notes.txt`; a
`docs/*.md` that is no staged doc id) unadjudicated. **Decided:** `TASK_AREA_FILES` names the root files
**and** the `docs/` directory with its own rule: inside `docs/`, jigc's set is `provenance.json` plus the
**staged doc ids** (`staged_doc_ids`, now asking `is_file()`) and nothing else — a `docs/*.md` that is
no staged id and any non-`.md` under `docs/` are foreign. The milestone-area set (§2) is its sibling
registry row, and the fan-out worktree keeps M48's `git status --ignored` adjudication.

### §7 — D3.5 and arm 3: `DESTROYING_DOORS` gains a disposition axis; the arm splits (Opus S4 · Codex B3)

`DestroyingDoor{verb, code: Option<&str>}` carries one code and a refuse/narrate axis; D3 gives `task
discard` two refusal causes (staged prose · foreign bytes) and `task finalize` a third disposition.
**Decided:** the registry's subject generalizes to *destroyed path* and gains
`disposition: Disposition` — `Refuse{consent: "--force"}` · `Narrate` · `Displace` — and `codes: &[&str]`;
M46's refuse-vs-narrate discriminator becomes the first two members. `flow49_acceptance.rs:999-1051`
grows with it, priced. **Arm 3 splits**: consenting doors × `{without --force, with --force}`, and the
displacing door (`task finalize`) × its one mode — the `task finalize --force` cell the record refuses
is never enumerated.

### §8 — D3.3/D3.4: displacement is a new tree-preserving primitive; `uninstall`'s `displaced/` row (Codex S2 · Opus A2 · Opus A3)

`displace_foreign_squatter` classifies one destination by the file-state baseline, frees an index slot
with `git rm --cached`, and parks by **basename** into a flat directory (`relocate.rs:283-328`);
`park_pre_image` likewise preserves no relative path. Only the same-filesystem `fs::rename` technique
transfers. **Decided:** a **new tree-preserving displacement primitive** — every foreign path moved to
`.jigc/displaced/<task-id>/<relative path>`, directories created as needed, no index manipulation —
and the **`displaced` key** on the landed envelope is `[{from: "<relative>", to: "<relative>"}]`,
sorted by `from`, **always present** (empty array on the ordinary path). `uninstall`'s `displaced/` row:
**refuse** without `--force` naming every entry, **narrate** with it; clearing it is the human's `rm`
named in the route — **no new verb**.

### §9 — D5.2: the three sibling doors resolve schema before the predicate (Opus S1 · Codex S4)

`rename::parse_addr` takes no pack; `milestone.rs:1410` runs ahead of `jigc_home`/the cascade and
resolves schemas at `:1431`; `task bind` parses at `task.rs:2682` and loads schemas at `:2694`. The
predicate needs a `Schema`. **Decided:** at each sibling the predicate is applied **after** that door's
existing schema resolution, **without moving any mutation and without changing the unknown-doctype
precedence** (`store.unknown-type` still answers first). The nine `doc.rs` doors are the one-line case.

### §10 — D5.4: the projection's keys, from one structured primitive (Opus S5 · Codex S5)

`projection_home_line` is a private **prose** renderer; `doc schema`'s JSON is a separate struct
(`doc.rs:4569-4794`). **Decided:** a structured engine primitive — `identity: {kind: "fixed" |
"slugged", address: "<the canonical address, or the `<ty>:<slug>` pattern>"}` and `home: {kind:
"placement" | "location", path: "<resolved>"}` — from which **both** the prose line and the
`contract-version` 7 JSON keys derive; the keys are named here so arm 5's assertion is writable.
`base`'s compound rides the same bump as `{sha: string, short: string}`.

### §11 — D9: the `suppressed.door` key, specified (Codex S6 · Opus A8)

`Suppressed` has exactly `reason` and `expires`, both required, fenced at pack-load
(`compose.rs:77`, `:173`). **Decided:** a third field **`door: <argv string>`** — optional; when
present, the compose doors (`start --workflow`, `workflow --preview`) refuse **`workflow.verb-routed`**
(blocking, exit 1) with `door` rendered as the `Mechanical` route; the pack-load fence checks its shape
exactly as a `Mechanical` route's argv is checked (parses against the real CLI) and refuses a malformed
value with the existing `suppressed` fence's code; absent ⇒ no refusal (the workflow composes as
today); project-layer workflows obey the same rule; unknown keys stay refused by the existing
deny-unknown-fields posture. The 14 members (`migrate-*` × 12 · `sub-task` · `milestone-execution`)
declare it; `reason`'s prose loses no words.

### §12 — Arm 6: a `PRE_DISPATCH_FAULTS` registry with an applicability relation (Codex B4 · Codex A1)

*"Every failure point × every leaf"* is not a product: a deleted cwd precedes repository discovery, a
malformed `packs.yaml` needs a door that loads packs, an unreadable `.jigc/` needs setup state, and some
leaves load no pack at all. **Decided:** a code-side **`PRE_DISPATCH_FAULTS`** registry, one row per
fault carrying its **phase** (before-discovery · after-discovery · after-pack-load) and its fixture
constructor; arm 6 iterates `PRE_DISPATCH_FAULTS × VERB_KINDS` **filtered by the leaf's phase reach**
(a leaf that never loads a pack has no after-pack-load cell, and the table says so) with an expected
`(fault, verb) → (arm, exit)` column. `refuse_on_posture` returning `Option<Outcome>` means closing its
`current_dir()` skip is a control-flow change, not one expression — noted, and it is why D6's funnel
precedes D2.4 (§14).

### §13 — D8: LD-3 answers on the `{error}` arm, deliberately (Opus S6 · Codex A3)

`locate::not_set_up()` is a bare `anyhow` with a mechanically rendered `jigc setup` route
(`locate.rs:45-52`), the answer 21 leaves already give for that state. **Decided:** the eight milestone
doors give **the same answer** — consistent with the shipped surface, on `Reject::Error`, **stated as a
deliberate consistency** rather than an omission; LD-4's *"not set up"* lie over an unreadable `.jigc/`
is fixed at the same producer (it names the read fault). Re-coding not-set-up at all 47 leaves is scope
this wave does not carry.

### §14 — The under-specified codes, registered (Codex S8 · Opus A9)

- **`migrate-corpus.missing-snapshot` at the enumeration** — target `<ty>@v<k>`; `Human` route naming
  the snapshot path to author (`doctype-authoring.md:21`); findings arm; exit 1. The fail-closed site is
  `migrate_corpus.rs:2103` **only** — `pack.rs:199-212`'s best-effort load feeding the read-only sweep
  stays declared (a manifest-less pack means *unchecked*, M51's `unversioned-doctype` posture), so a
  class sweep does **not** redden `validate` on manifest-less packs (Opus S3).
- **`file-state.absorbed`** — advisory; target the doc address; `Informational` route naming the finding
  it retired; on the door's success envelope `findings`; exit unchanged; at `ingest` and `rename`.
- **`<door>.foreign-bytes`** — also the `MintedSet` conflict finding (§2).
- **`displaced`** — §8.
- **`<door>.rollback-conflict`** — per §1's injected metadata; the family of doors: `finalize` ·
  `rename` · `milestone` · `task-discard` · `config` · `doc-author`.

### §15 — Arms 4 and 5: the home set and the manufactured `singleton`-only cell (Codex S7 · Opus S7)

Arm 4's `home-vacated` assertion iterates **the fixed-identity home set** (placement root file ·
placement file rerooted through `placement-root` · location singleton · a never-had-history control ·
a home whose file moved and is adoptable at its new path) rather than one `git mv`. Arm 5 adds a
**manufactured** `location:` + `singleton: true` doctype (via `--pack-from-dev`) so the predicate's
`singleton`-only disjunct is exercised — every shipped `singleton: true` doctype is also `placement:`,
so the shipped set cannot reach it.

### §16 — The bounds re-labelled, the order written, the negative fence (Opus A5–A7 · Codex A4–A6)

- **Bound:** `GIT_DIR` (a deliberate operator act that moves the subject; reopening condition as M51
  §3). **Bound:** `setup`'s unborn-HEAD `PostureExemption` stays — D2.1's universal quantifier does not
  cross it. **Deferral, labelled:** git 2.54.0's marker contract — reopening trigger: *the first CI or
  adopter report on another git major/minor where a marker cell diverges*. **Deferral, labelled:** the
  departed-doctype orphan half — its trigger (*the next `schema-version` bump of any doctype*) **cannot
  fire inside M52**, so the un-namespaced stamp key **crosses the 1.0 pin**, and the record says so.
  **Posture, not bound:** D3's displacement behaviour is reversible after 1.0; its key is one-way and
  is spent only once its value shape (§8) is fixed.
- **The decompose order that must hold:** D6.3's funnel + §3's arm move → D2.4 and D1.7 · D1.1's
  registry → D1.2/D1.3 · D3.1's registry (+ §6's tree rule) → D3.2/D3.3/§2 · D5.1's predicate → D5.2
  → D5.3/D8's `rename --slug` · D4.1's walk → D4.5/D4.6 · §11's key + fence → D9's refusal · the
  registries and applicability relations → the arms.
- **A negative fence at the close:** `git diff <base>..HEAD --stat -- crates/cli/pack/schemas
  packs/methodology/schemas crates/cli/pack/config/schema-manifest.yaml
  packs/methodology/config/schema-manifest.yaml '**/schema-snapshots/**'` is **empty**; the one
  permitted doc correction (`milestone-record.base`'s `set:` kind in three records) touches no file
  under those paths.
- **Count slips fixed in place:** 12 registry consts (not 13); nine address doors + `SLUG_DOORS`'
  `rename` row (not "ten").

### §17 — D4.2's exemption, stated (Opus S3)

Recorded in §14's first bullet: fail-closed at `candidate_docs`' enumeration only; `pack.rs`'s
best-effort sweep load stays declared.
