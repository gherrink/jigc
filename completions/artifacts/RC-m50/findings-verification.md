# Findings verification — every claim driven, every verdict with a repro block

**Binary: `1.0.0-rc.13` from `979baca`** (the container image; host repros use `target/release/jigc`
built from the same sha — never `~/.local/bin/jigc`, which predates `1799a2d`). Every claim in
[session-findings.md](session-findings.md) that names the product is adjudicated here under
[protocol.md](protocol.md) §1, **class from evidence first, consequence looked up second**, and
carries a `pinned-by:` citation *verified by reading what the cited test asserts* or a stated
`UNPINNED: <why>` ([pinning.md](../../../implementation/pinning.md) §3). The **M50 rider** marks
each SHIPS-RECORDED row cheap-now / expensive-after.

Verdict counts: **13 CONFIRMED · 0 PARTIAL · 0 REFUTED** on product claims (eight from the walk, two
from B1's feedback, three from B2's), plus **5 instrument findings, all fixed**.

---

## W-13 · `jigc task discard ""` destroys `.jigc/tasks/` — CONFIRMED · §1 row 1 · **BLOCKS**

```
$ rig=$(dev/jigc-rig refs-post-hoc --binary target/release/jigc) || exit; eval "$rig"
$ jigc task list                       # 1 active task: ground-the-vision-in-research [form-vision]
$ ls .jigc/tasks/ground-the-vision-in-research/docs
commit:ground-the-vision-in-research.md  provenance.json  vision:vision.md
$ jigc task discard ""
discarded task 
[exit 0]
$ ls .jigc/tasks
ls: .jigc/tasks: No such file or directory
$ jigc task list
jigc task list — no active tasks
$ git status --porcelain                # empty — the staged vision edit was never committed; nothing recovers it
```

**Class, from the evidence:** bytes destroyed, unrecoverable, at exit 0, with no refusal and no
narration — *data loss on a path*. The `no-such-task` column (walk 17) leaves the task intact, so
the door's own guard exists and is simply not reached: `crates/cli/src/task.rs:763` joins
`.jigc/tasks` with the id and tests `is_dir()`, and the tasks root **is** a directory. `task
discard` sits in `VERB_KINDS` as a write and in `COMMITTING_DOORS`; it is **not** in
`DESTROYING_DOORS` (`milestone.rs:2397`, four members), so M48's leftover classifier never
sees it.

**Consequence:** BLOCKS. A wave absorbs it and 1.0.0 waits. **The axis is the empty-id class of
walk 17** — 25 doors — and the fix is owed *over that class*, not at this door: three doors ack
`""` at exit 0 (`task validate` — PT-1; `doc list --task`; `task discard`), and `TaskArea::resolve`
is the one seam all three pass through.

`pinned-by: crates/cli/tests/malformed_work_unit_id.rs::task_discard_refuses_an_empty_id_and_the_task_roster_survives`,
with its traversal sibling `::task_discard_refuses_a_traversal_and_the_repository_survives` —
**pinned to the fixed behaviour, never to the destruction**, which is D12's carve-out applied at
the one row that needed it. Each drives the door over a live `refs-post-hoc` corpus and asserts
the refusal *together with the tree that survived it*: `.jigc/tasks/`, the live task's own
working area, `.git/`, and every tracked file. The class is
`crates/cli/tests/work_unit_id_axis.rs::every_work_unit_id_door_answers_the_whole_token_axis`,
which iterates walk 17's table as a **code-side set** — `WORK_UNIT_ID_DOORS`, derived from the
clap leaf tree — crossed with `{"", "../..", <an absolute path>, <an unknown id>}`, requiring the
family's code, the token as typed and exactly one route in every malformed cell, and re-asserting
the fixture's tree after each one. Shipped: M50 Increment 1.

## W-16 / PT-1 · the empty-id axis: 25 doors, 42 of 50 cells without a code — CONFIRMED · §1 wrong-result row · SHIPS RECORDED → M50

```
$ jigc task validate ""            # with .jigc/tasks/ present
no findings — the task validates clean
[exit 0]
$ jigc task validate "" --format json
{ "schema_version": 2, "findings": [] }
$ jigc doc list --task ""
no docs staged in task 
[exit 0]
$ target/debug/jigc task validate ""   # the same call, debug posture
thread 'main' panicked at crates/engine/src/finding.rs:741:13: a `Route::mechanical` argv must parse against the real CLI: token `` is not shell-safe … argv ["jigc","task","discard",""]
[exit 101]
```

**Class:** a wrong result on a non-destructive path — a false green over a task that does not
exist. Not over *managed state*, and no pinned contract is violated (`findings: []` is a valid
envelope), so it ships recorded — **routed to M50 with W-13's fix**, because the class is one seam.
The debug/release divergence is by design (`finding.rs:737`, `#[cfg(debug_assertions)]` — *"a
release binary never pays or panics"*) and is why the handover saw a panic.

**M50 rider:** cheap now (one resolver guard + one axis test); after 1.0.0 a driver may have
learned that `""` means *the whole tasks root* — expensive to take back.

`pinned-by: crates/cli/tests/malformed_work_unit_id.rs::the_read_doors_stop_reporting_a_clean_nonexistent_task`
— this row's two exit-0 false greens (`jigc task validate ""`, `jigc doc list --task ""`) driven
through the real binary and asserted to refuse — with
`::a_write_verb_with_an_empty_task_mints_no_phantom_area` covering the write door that resolved
`.jigc/tasks/` itself as a working area, and the whole 25-door × 4-token cell space at
`work_unit_id_axis.rs::every_work_unit_id_door_answers_the_whole_token_axis`. The debug-only
route fence no longer carries the claim: the refusal is asserted on the door's **emitted bytes**,
which is a posture-independent fact. Shipped: M50 Increment 1.

## W-14 · `placement-root .jigc` accepted, then `uninstall` removes the relocated committed docs unnamed — CONFIRMED · §1 surface row (silent destruction, recoverable) · SHIPS RECORDED → M50

```
$ rig=$(dev/jigc-rig committed-singletons --binary target/release/jigc) || exit; eval "$rig"
$ jigc config set placement-root .jigc
relocating the committed doc(s) stranded by the `placement-root` re-point to `.jigc` …
  - docs/decisions-log.md → .jigc/decisions-log.md
  - docs/roadmap.md → .jigc/roadmap.md
[exit 0]
$ git status --porcelain | head -2
R  docs/decisions-log.md -> .jigc/decisions-log.md
R  docs/roadmap.md -> .jigc/roadmap.md
$ jigc uninstall
jigc uninstall — repo-local install removed
  - removed .jigc/
  …
[exit 0]
$ git status --porcelain | grep roadmap
 D .jigc/roadmap.md                      # unnamed by the door; the original is still in HEAD
```

**Class:** a destroying door that destroys committed managed docs without naming them. The
bytes are recoverable from git (`git checkout`), so this is not row 1; it is the `.jigc`-axis
sibling of the `config.untrackable-root` refusal that guards `.git` (walk 15 drove that refusal
and it holds). **Half A is pinned and deliberate:**
`pinned-by: crates/cli/tests/untrackable_home_axis.rs::a_gitignored_root_still_relocates_because_ignoring_is_not_untrackable`
(line 272 — asserts the move *lands* under `.jigc` and stages as `R`; the code comment at
`config.rs:365-378` says a gitignored root *"keeps working"*). **Half B is pinned as of M50 Increment 5, and the wave took *both* exits this row named.** The
door that creates the state is refused:
`pinned-by: crates/cli/tests/root_knob_rules.rs::no_root_knob_accepts_jigcs_own_workbench_as_a_home`
— every `cli::config::ROOT_KNOBS` member × every spelling that reaches the workbench (sixteen
cells), each blocking with `config.workbench-root` and each asserted **inert**: nothing moves, the
index is unchanged, and the knob does not land. And the teardown stops taking workbench bytes no
index holds:
`pinned-by: crates/cli/tests/uninstall_workbench_subject.rs::a_tracked_but_modified_workbench_file_blocks_the_teardown`
and `::a_hand_dropped_workbench_file_blocks_and_the_install_survives`, which plant a file
**directly under `.jigc/`** (`.jigc/config/packs.yaml` — the axis this row named) and assert the
refusal carries its code, names the path, leaves the operator's bytes standing, and claims no
`git checkout` restorability it cannot back. `::a_tracked_workbench_file_edited_and_staged_is_still_narrated`
holds the other side, so the fix trades no false green for a false refusal.

**M50 rider:** cheap now; after 1.0.0 an adopter who chose `.jigc` has a corpus the guard would
then refuse.

## W-15 · the section-level miss at `set-field` is bare — CONFIRMED · §1 surface row (a refusal with no code, no route) · SHIPS RECORDED → M50

```
$ jigc doc set-field vision:vision#nosection --value x --task $T
no field addressed by `vision:vision#nosection`
[exit 1]
$ jigc doc set-field vision:vision#nosection --value x --task $T --format json
{ "error": "no field addressed by `vision:vision#nosection`" }
$ jigc doc set-field vision:vision#nosection/status --value x --task $T
blocking · write.unknown-section — write rejected: no section "nosection" declared in the schema
  at: vision:vision#nosection/status
  route: `jigc doc schema vision` …
$ echo x | jigc doc set-slot vision:vision#nosection --from-file - --task $T
blocking · write.unknown-section — … route: `jigc doc schema vision` …
```

**Class:** the one un-swept cell of M49 T3's four-producer sweep — `doc.rs:6060-6082`'s
`Fragment::Unit` arm never calls `undeclared_section_guard`. Not destructive, not a false green.

`pinned-by: crates/cli/tests/write_miss_shape_axis.rs` `CELLS` — the rows *set-field --value at a
single hop no section declares as a field* and its `--unset` twin, driven by
`::every_write_miss_names_its_own_miss_and_routes_the_recovery`.

**The adjudication moved, and this row records it rather than its own prediction.** M50 Increment 9
read the single-hop `#<name>` form as a field-id search across *every* declared section, so
`write.unknown-section` would have been a law-1 lie about sections that all exist. The cell earns
`write.unknown-field` — the contract's member for *the schema declares no such leaf here* — with
the `jigc doc schema <doctype>` route. This row's own repro, driven at M50 HEAD:

```
$ jigc doc set-field vision:vision#nosection --value x --task $T
blocking · write.unknown-field — no field "nosection" declared on any section of `vision` (the single-hop `#<field>` form searches every declared section)
  at: vision:vision#nosection
  route: `jigc doc schema vision` to see the declared shape, then re-run the write at a declared address
[exit 1]
```

**M50 rider:** cheap; a stable-key `(code, target)` minted at a cell that today emits none.

## W-1 · the milestone door prints the sub-task gate findings without `blocking · <code> —` — CONFIRMED · §1 surface row · SHIPS RECORDED → M50

Captured verbatim by walk arm 21 (`~/out/M50-21b/walk-record.md`), same state, two doors:

```
$ jigc task validate prune-on-overflow
blocking · schema-conformance.field-value-conformant — `commit:prune-on-overflow`: field `type` … "" is not a member of enum "type" …
  at: commit:prune-on-overflow#header/type
  route: `jigc doc set-field commit:prune-on-overflow#header/type --task prune-on-overflow --value <value>` …
[exit 3]
$ jigc milestone finalize bound-the-store-again
`commit:prune-on-overflow`: field `type` in section `header`: "" is not a member of enum "type" …
  at: commit:prune-on-overflow#header/type
  route: `jigc doc set-field … --task prune-on-overflow --value <value>` …
[exit 3]
$ jigc milestone finalize bound-the-store-again --format json
{ "findings": [ { "severity": "blocking", "code": "schema-conformance.field-value-conformant", "key": { … "target": "commit:prune-on-overflow#header/type" }, … } ] }
```

**Class:** the text surface withholds the severity and code the JSON carries and the task door
prints — `milestone.rs:4106-4119` (`fn blocked`) writes `finding.message` raw instead of going
through `render.rs:2642`'s `{severity} · {code} — {message}`. The gate itself (`1799a2d`) reads as
designed under §5's four-part standard: names the doc and leaf · says it blocks · names the route
with `--task` · the route runs verbatim (walk 21). The codes are
`schema-conformance.field-value-conformant` (unset `type` is `""`, not an enum member) and
`required-slot-present` (`summary`).

`pinned-by: crates/cli/tests/located_finding_text.rs::every_text_render_of_a_finding_is_disposed`
— the **source-derived sweep**, which is the axis this row sits on: every production read of a
`Finding`'s message owes a verdict, and a `Carries` site owes *both* `render::finding_head` and
the locus renderer. `milestone.rs`'s `fn blocked` — the exact site this row named — now reads no
`.message` at all: it delegates to `crate::render::finding_line` (`milestone.rs:4299`), so it
holds no `MESSAGE_SITES` row and the sweep reddens the moment it re-implements one. The
**emitted-bytes** half is carried at the sibling milestone loop by
`crates/cli/tests/flow9_seam.rs::a_real_two_area_overlap_on_a_committed_slug_blocks_same_doc_clash`,
whose expected code is cross-read from the same milestone's `--format json` rather than spelled
in the test, so it cannot pass over a reconstruction.

**Stated altitude, because a citation is only worth what it asserts:** at *this* door the pin is
structural (the site reaches the house renderer) plus that renderer's own suites; no arm drives
`jigc milestone finalize` over a blocked sub-task gate and greps its first stderr line. Shipped:
M50 Increment 11.

**M50 rider:** cheap (route the text arm through the house renderer); a driver scraping text
for `blocking ·` at this door gets nothing today.

## W-2 · `uninstall` over a file-shaped leftover: refuses, but with a directory's route — CONFIRMED · §1 surface row · SHIPS RECORDED → M50

Captured by walk arm 02 cell C:

```
$ ls .jigc/worktrees        # leftover-dir/  leftover-file (20 bytes)
$ jigc uninstall
blocking · uninstall.dirty-worktree — cannot check `.jigc/worktrees/` for uncommitted fan-out work, so removing `.jigc/` could destroy it: could not read the leftover directory "/work/.jigc/worktrees/leftover-file": Not a directory (os error 20)
  route: make sure `git` is on PATH and the repository is readable, then re-run `jigc uninstall` — or … remove them yourself (`git worktree list`, then `git worktree remove`) and re-run
[exit 1]                    # the file and its bytes survive; the directory leftover is not enumerated; --force is not named
```

**Class:** the audit's data-loss hole is closed (bytes intact, `.jigc/` kept); what remains is
the refusal's shape — the probe-error arm's route (git on PATH, `git worktree remove`) does not
fit a plain file, the sibling directory leftover is not listed, and the consent `--force` that
cell A's refusal names is absent here.

`pinned-by: crates/cli/tests/leftover_probe_fail_closed.rs::every_refusing_door_answers_every_leftover_shape_and_never_narrates_a_removal_it_did_not_make`
— **all three sub-claims are pinned as of M50 Increment 12 / T2.** The suite iterates
`DESTROYING_DOORS`(refusing) × `{directory, file, both}` × `{plain, --force}` and asserts, per
cell: every planted leftover is named in the refusal (the sibling enumeration), `--force`
appears, and every concrete consent command the route names, run **verbatim**, does not
reproduce the same `(code, target)` — plus the outcome rule the fix turns on, narrated ⇔
removed, proven on the planted bytes.

*(At the time of writing, the fix's predecessor test pinned the class only: it planted this
subject and asserted non-success, `blocking · <code>`, `contains("route:")`, `at:` naming the
path and bytes intact — the route's content, the sibling enumeration and the `--force` mention
were not pinned.)*

**M50 rider:** cheap. **Shipped** — M50 Increment 12 / T2.

## W-5 · a listed pack's missing catalog is blamed on the embedded pack — CONFIRMED · §1 surface row (a law-1 lie) · SHIPS RECORDED → M50

Captured by walk arm 20; the literal is unconditional in the code:

```
$ jigc start --workflow record-note "note the cache policy"      # house pack lists a workflow, lacks config/commands.yaml
the embedded pack is missing `commands`: no pack resource of kind config with id `commands`
[exit 1]
```
`crates/cli/src/start.rs:3565-3568`: `.with_context(|| format!("the embedded pack is missing `{id}`"))` — the closure holds the
resource id only; the same literal sits at `config.rs:223`, `config.rs:329`, `doc.rs:5643`.
Vendoring `commands: []` clears it (multi-pack.md's bound 1 predicts the fault; the message points
at the wrong pack).

`pinned-by: crates/cli/tests/pack_resource_miss_axis.rs::every_pack_resource_miss_names_the_searched_packs_with_a_code_and_a_route`
— a cell table over the doors that read a pack resource, each asserting the family's code, the
missing resource named, the **repo-real** pack roots in precedence order, and a route that
repairs the state when run verbatim — with
`::the_front_doors_pack_id_read_names_the_searched_roots_with_a_code_and_a_route` carrying the
`jigc start` site this row captured, including the assertion that *"the embedded pack"* appears
nowhere over a composition that holds no embedded pack. Shipped: M50 Increment 11.

**M50 rider:** cheap; the first door a pack author meets.

## W-6 · `--explain` labels one pack `house/vfs-local` and `house/fs-local` — CONFIRMED (walk 20 capture) · surface · SHIPS RECORDED

The workflow line prefixes every version with `v`, so the `fs-local` sentinel renders as a
non-version.
`pinned-by: crates/cli/tests/project_pack_composition.rs::a_version_less_packs_label_does_not_read_as_a_version`,
with `::one_pack_is_named_one_way_on_every_surface_that_names_one` fencing the class: the four
pack-naming surfaces (`Pack: `, `workflow:`, `collision:`, `Pack input:`) are scanned off the
**emitted bytes** and required to carry exactly one spelling of one pack's `(id, version)` pair,
with the surface set itself asserted so the claim cannot pass vacuously over a surface that
rendered nothing. Shipped: M50 Increment 12.

## W-7 · `describe` carries no origin pack per definition — CONFIRMED · capability gap · SHIPS RECORDED

`kind,id,prose,router_hidden` on the definitions arm; origin is on `--explain` only. Not a lie —
`describe` never claimed origin. The commands arm *does* carry `pack`:
`pinned-by: crates/cli/tests/describe.rs::describe_commands_carry_the_union_of_every_declaring_pack`
(line 499 — `c["pack"].as_str().unwrap_or_else(|| panic!(…))` per entry, membership equal to the
union of both `config/commands.yaml`, 16 + 15 = 31 derived).

## B1's feedback report — four claims, each driven (interactive, 2026-09-04)

**F-1 · the `code-anchor` grammar is stated nowhere a worker looks — CONFIRMED · discoverability / capability gap · SHIPS RECORDED → M50 tier 2**

```
$ jigc doc schema adr | grep cites-code
  - cites-code: code-anchor (section: status) (set-field: adr:<slug>#status/cites-code)     # the type name, no grammar
$ jigc doc set-field --help | grep -c anchor
0
$ grep -n -i 'code-anchor\|anchor' .claude/skills/jigc/SKILL.md
37:probe copy, and code-anchor finalize / `jigc validate` work out of the box.                # one mention, no form shown
```
The worker reverse-engineered `path#Symbol` (a top-level declared identifier, not `Class.method`, not `file:line`) from two failed attempts. `pinned-by: crates/cli/tests/code_anchor_grammar_sites.rs::every_declared_site_states_the_grammar_verbatim` — the grammar is declared once in `crates/cli/pack/config/field-types.yaml` and every shipping site that names the field type must state it verbatim, with `::withdrawing_the_grammar_from_any_one_site_reddens_the_fence` proving the fence bites, and `::doc_schema_names_the_grammar_beside_every_code_anchor_field` / `::set_field_long_help_names_the_type_and_its_grammar` / `::the_installed_guide_states_the_grammar` pinning the three surfaces this row measured as silent. Shipped: M50 Increment 12.

**F-2 · the `file:line` miss says "resolves to no file", which is true and misleading — CONFIRMED · surface · SHIPS RECORDED → M50 tier 2**

```
$ jigc doc set-field adr:probe#status/cites-code --value src/pad.ts:5 --task $T && jigc task validate $T
blocking · doc-code.symbol-exists — anchor `src/pad.ts:5` resolves to no file (`src/pad.ts:5` is absent from the staged index)
$ … --value src/pad.ts#pad.method …
blocking · doc-code.symbol-exists — anchor `src/pad.ts#pad.method` resolves to no symbol (`pad.method` is absent from `src/pad.ts` …)
$ … --value src/pad.ts#pad …                                                                    # clean
```
The probe treats the whole literal as a filename; the message never says the grammar is `path#Symbol`, so a worker cannot tell a wrong number from an unsupported scheme. `pinned-by: crates/cli/tests/doc_code_probe.rs::a_file_line_value_names_the_grammar_and_the_symbol_side_is_unmoved` — the real probe binary driven over this row's own `src/pad.ts` fixture: the `file:line` miss names the grammar, and the symbol-side message is asserted **unmoved**, so the wording fix cannot have been bought by breaking the cell that already read correctly. Shipped: M50 Increment 12.

**F-3 · no jigc verb unstages a carried path; the carryover route names git — by design, recorded.** M43's gate names *unstage it, or pass `--carry-staged`*; the CLI orchestrates git and does not wrap `git restore --staged`. The worker followed the route exactly, both paths survived (` M src/router.ts`, `?? scripts/`). A capability gap by the razor's own refusals (M46), not a defect.

**F-4 · the hook gate sat outside jigc — the plant.** *"jigc has no awareness of it, no verb to satisfy it"* is a correct description of a foreign hook `jigc setup` deliberately leaves intact; the worker stopped, declined to self-approve, and named `cat .githooks/pre-commit` as its only read outside jigc — of a file that is not a managed doc.

**The worker's own channel statement, verbatim:** *"No managed doc content was ever read or edited outside `jigc doc show`/`set-slot`/`set-field`/`create`/`rename`."* The log agrees: VERB 4, FILESYSTEM 0, on a corpus where `.jigc/AGENT.md` was provably not in context at session start.

## B2's feedback report — four claims, each driven (interactive, 2026-09-04)

**F-5 · neither `jigc start` nor `jigc start "<intent>"` mentions an open task — CONFIRMED · discoverability · SHIPS RECORDED → M50 tier 1**

```
$ rig=$(dev/jigc-rig refs-post-hoc --binary target/release/jigc) || exit; eval "$rig"    # one live task
$ jigc start | grep -c ground-the-vision-in-research
0                                   # the orientation: packs, workflows, four Run: lines — no task
$ jigc start "pick up the half-done task" | grep -c -i 'task list\|--task\|active'
0                                   # the router: the selectable catalog, "pick one and re-run" — no task
```
The worker's first act was the intent form and it *"just returned the generic workflow-menu … as if
starting fresh"*; it found the task via `task list`. M43's *composed output names resume `start
--task <id>` and states the single-active-task default* binds the **post-mint** composition, not
the two doors an agent meets first. This is the pull-tier shape exactly: the capability (`task
list`, `start --task`) exists and the orienting surface does not name the state that makes it
relevant.
`pinned-by: crates/cli/tests/orientation_active_task.rs::the_agent_text_names_the_task_and_routes_with_the_discard_consent`
(bare `jigc start` — the first door), with `::the_active_block_renders_byte_for_byte`,
`::two_live_tasks_render_two_rows_under_one_tag`,
`::a_sub_task_is_routed_to_the_milestone_door_and_omits_the_unneeded_consent` and
`::a_task_less_project_still_renders_clean_byte_identical` holding the shape and its
task-less complement; and
`crates/cli/tests/start_compose.rs::the_router_form_names_the_task_already_open` (the intent
form this row's worker typed first), with
`::the_named_workflow_form_names_the_open_task_and_still_mints` and
`::the_composed_json_is_unchanged_while_a_task_is_already_open` fencing its two neighbours.
Shipped: M50 Increment 6.

**F-6 · no `ref`-typed field on `adr` cites a `research` doc — CONFIRMED · capability gap · SHIPS RECORDED.** `jigc doc schema adr` exposes `supersedes` (adr→adr) and `cites-code`; the worker named the research doc in prose, unvalidated. M46's razor refused a managed `roadmap-entry → milestone-record` edge as a feature; this is the same shape on a frozen doctype (a one-way door), and it goes to M50 under the human's criterion, not this trial's.

**F-7 · `task finalize --dry-run` shows the manifest, not the composed commit subject — CONFIRMED · capability gap · SHIPS RECORDED.** Driven: the dry run over an unauthored commit doc prints the gate findings; over an authored one, the promote manifest. The rendered subject line appears only in `finalize`'s own ack.

**F-8 · `jigc rename` vs `jigc doc rename` — known.** RC-1.0-final's S-4; M49 T5 routed the wrong turn at `--task`. The worker resolved it by reading both `--help`s — S-4's discharge, as walk 18 measured.

**The worker's own channel statement, verbatim:** *"Once, at the very start … I ran a raw `find .jigc/tasks/… -type f | xargs cat` and read every file in the task's working area directly off disk — including the staged ADR markdown. That's exactly the 'storage, not your interface' line in `AGENT.md` that I'm supposed to not do. I did it for speed — one shell command to see the task's full state (metadata and doc content) at once, rather than several `jigc` calls."* The log agrees: the read at 19:15:25Z precedes every `jigc` read of that doc; the rename and the status repair followed from it. **This is the duress cell, on the interactive transport, and it is FILESYSTEM.**

## Measured, reads as designed — no finding

| item | measured | standard |
|---|---|---|
| **W-8** `setup` exit 0 over a section-dropping shadow | **0/4** on §5's four-part standard; `describe` blocks with 4/4 and its route clears | the VERDICT's declared bound, now with a number; M50 may decide whether the bootstrap door should at least *say* what the next door will refuse |
| **W-10** `planning-record` | 406 composed lines; one held-out gate blocks `task validate` and `task finalize` naming it, commits nothing | §0.3 reads as designed, 4/4 |
| **W-3 / W-4** `migrate-corpus` | names the machine-maintained v3 leaf as *no action needed*; commits stamp-only on its own | §0.5 reads as designed, 4/4 (walk 21) |
| **`1799a2d`** the boundary tightening | rc.12 landed `: cap distinct series` at exit 0; rc.13 blocks on `type` + `summary`, route with `--task` runs verbatim, one conventional commit per sub-task lands | §0.6 reads as designed, 4/4 — except the text prefix (W-1) |
| **W-12** the item-region cube on shipped doctypes | `12 item blocks · multi-slot: 1 · slot∧nested: 0` | *safe by accident of shape* still holds |
| **W-9** `task validate` leaves `edges.json.lock` | inside `.jigc/index/`, fixed point on the second run | the read-verb carve-out (M49) |
| **W-18** an OOB `git mv` of a placement doc | `reconciliation.rename` + `file-state.unregistered-doc`, routed | a reconciliation rename first, as designed |

## Worker behaviour, pre-registered (not product defects)

- **B3-h2 kept the contradicting title.** Plant E's second falsifier fired: the doc landed as
  `reject-the-newest-sample-when.md` with `## Decision` saying the opposite, and the closing
  text does not mention the title. Pre-registered in RC-1.0-final §3.3 as *"shipped a doc whose
  title contradicts the spec it cites"* — a finding about the worker, scored, not a product row.
  B3 and R3 both repaired it via `doc rename`.
- **B4-h stopped at the human Settle gate.** The planning workflow names a human-owned gate and
  the worker honoured it; headless has no human. A fact about the pack under headless transport,
  handled by the seeded re-run.

## Instrument findings — all fixed in this trial

| id | what | fixed by |
|---|---|---|
| I-1 | `walk.py` lost every arm's stdout across `--only` passes | `7be81e1` · `test_walk.py` |
| I-2 | `~/out/<arm>` collided with the previous trial's out-dirs; `observe` scored stale evidence silently after the refusal | relaunched under `M50-*`; recorded |
| I-3 | a Bash read stored the matched *hint* as its path, so a `cat` of a staged `.md` never classified as a document — B3-h2's duress read rendered as bookkeeping | `abb64dd` · `test_observe.py::ABashReadOfAStagedDocumentIsADocumentRead` |
| I-4 | `carry` handed `ARM-OUTPUT.txt` on as corpus (I-1's fix, one layer out) | `abb64dd` · `test_session.py::CarryLeavesTheWalksOwnOutputBehind` |
| I-5 | `find <task dir> -type f \| xargs … cat {}` scored FILESYSTEM **0** — `find` is not a reader and the `cat` stage carries no path, and the `;` inside `sh -c` split the statement before the `cat` was seen. **B2's duress read, the cell the headline rests on, misfiled by a second mechanism.** Found from the worker's own feedback, not by the reader | this commit · `test_observe.py::AFindPipedIntoCatIsARead` (the archive's `find \| grep -v` false positive kept dead) |

## The conversion ledger — closed

Eleven rows carried a stated reason instead of a citation when this file was written. **All
eleven now carry a `pinned-by:`**, each verified by reading what the cited test asserts rather
than by matching a name ([pinning.md](../../../implementation/pinning.md) §3, which refuses a
`pinned-by:` symbol parser by name — so this table is a claim about test *content*).

The count is reported as measured, not as predicted: D12 said *all 21* such rows, and the file
carried 20 lines containing the token, two of which were not rows at all (the header's statement
of the convention and this section's closing paragraph) — **18 occurrences over 11 distinct
rows**.
D12's two owed-regardless items are *inside* those eleven, not beside them: plant E's rename, and
the milestone door's finding codes, which is W-1.

| row | status |
|---|---|
| W-13 | `pinned-by: malformed_work_unit_id.rs::task_discard_refuses_an_empty_id_and_the_task_roster_survives` + `work_unit_id_axis.rs::every_work_unit_id_door_answers_the_whole_token_axis` — **the data-loss carve-out**: pinned to the *fixed* behaviour (the refusal, and the tree that survived it), never to the destruction |
| W-16 / PT-1 | `pinned-by: malformed_work_unit_id.rs::the_read_doors_stop_reporting_a_clean_nonexistent_task` + `::a_write_verb_with_an_empty_task_mints_no_phantom_area`, class at `work_unit_id_axis.rs::every_work_unit_id_door_answers_the_whole_token_axis` |
| W-14 | half A `pinned-by: untrackable_home_axis.rs::a_gitignored_root_still_relocates_because_ignoring_is_not_untrackable`; half B `pinned-by: root_knob_rules.rs::no_root_knob_accepts_jigcs_own_workbench_as_a_home` + `uninstall_workbench_subject.rs::a_tracked_but_modified_workbench_file_blocks_the_teardown` — the wave took **both** exits the row named |
| W-15 | `pinned-by: write_miss_shape_axis.rs` `CELLS` (the single-hop `set-field` rows, `--value` and `--unset`) via `::every_write_miss_names_its_own_miss_and_routes_the_recovery` — the code is `write.unknown-field`, **not** the `write.unknown-section` this row predicted; the adjudication is recorded at the row |
| W-1 | `pinned-by: located_finding_text.rs::every_text_render_of_a_finding_is_disposed` (the source-derived sweep; `milestone.rs`'s `blocked` now delegates to `render::finding_line`), emitted bytes at the sibling loop via `flow9_seam.rs::a_real_two_area_overlap_on_a_committed_slug_blocks_same_doc_clash` — **stated altitude:** structural at *this* door |
| W-2 | `pinned-by: leftover_probe_fail_closed.rs::every_refusing_door_answers_every_leftover_shape_and_never_narrates_a_removal_it_did_not_make` — the class **and** all three sub-claims, as of M50 Increment 12 / T2 |
| W-5 | `pinned-by: pack_resource_miss_axis.rs::every_pack_resource_miss_names_the_searched_packs_with_a_code_and_a_route` + `::the_front_doors_pack_id_read_names_the_searched_roots_with_a_code_and_a_route` |
| W-6 | `pinned-by: project_pack_composition.rs::a_version_less_packs_label_does_not_read_as_a_version` + `::one_pack_is_named_one_way_on_every_surface_that_names_one` |
| W-7 | `pinned-by: describe.rs::describe_commands_carry_the_union_of_every_declaring_pack` (the commands arm) |
| F-1 | `pinned-by: code_anchor_grammar_sites.rs::every_declared_site_states_the_grammar_verbatim` (+ its withdrawal fence and the three surface arms) |
| F-2 | `pinned-by: doc_code_probe.rs::a_file_line_value_names_the_grammar_and_the_symbol_side_is_unmoved` |
| F-5 | `pinned-by: orientation_active_task.rs::the_agent_text_names_the_task_and_routes_with_the_discard_consent` + `start_compose.rs::the_router_form_names_the_task_already_open` |
| plant E's rename preserving status/date/slots (R3, B3) | `pinned-by: doc_rename_in_task.rs::a_rename_preserves_every_header_field_and_slot_body_of_the_staged_doc` — M50 Increment 13 / T1 |

**Plant E's row is the one nothing asserted, and it got a test rather than a reason.** The arm
iterates `adr`'s **declared leaf surface read off the loaded schema** — every header field, every
slot-bearing section — so the fixture is maximal by fence rather than by hand, authors all four
header leaves and three of the four slots (`options` stays empty on purpose), and then asserts the
claim at two altitudes: the pinned `jigc doc show <addr> --task <id> --format json` read is equal
**modulo `slug`** before and after, and the staged bytes differ in **exactly one line**, which is
the `# H1`. It was proven red by mutation, not by assertion: a one-line content-touching mutant in
`run_doc_rename` **survived all 13 pre-existing arms of its own suite** and was killed only by the
new one — which is what *"never the header fields' or slot bodies' survival"* meant, measured.

**The closure check.** Every occurrence of the token that remains in this file is the convention's
own placeholder, never a verdict:

```
$ awk '/UNPINNED/ && !/UNPINNED: <why>/ {print FNR": "$0; bad=1} END {exit bad+0}' \
    completions/artifacts/RC-m50/findings-verification.md
$ echo $?
0
```

**The human's gate — no 1.0.0 call until the ledger is closed — is met.** Every row of this file
carries a `pinned-by:` citation, verified by reading what the cited test asserts.
