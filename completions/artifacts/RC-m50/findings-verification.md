# Findings verification — every claim driven, every verdict with a repro block

**Binary: `1.0.0-rc.13` from `979baca`** (the container image; host repros use `target/release/jigc`
built from the same sha — never `~/.local/bin/jigc`, which predates `1799a2d`). Every claim in
[session-findings.md](session-findings.md) that names the product is adjudicated here under
[protocol.md](protocol.md) §1, **class from evidence first, consequence looked up second**, and
carries a `pinned-by:` citation *verified by reading what the cited test asserts* or a stated
`UNPINNED: <why>` ([pinning.md](../../../implementation/pinning.md) §3). The **M50 rider** marks
each SHIPS-RECORDED row cheap-now / expensive-after.

Verdict counts: **8 CONFIRMED · 0 PARTIAL · 0 REFUTED** on product claims (the headless sessions
produce no worker claims to refute; B1/B2's feedback, when the interactive arms run, joins this
file), plus **4 instrument findings, all fixed**.

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

`UNPINNED`: no standing test drives an empty id at any door. Nearest:
`crates/cli/tests/no_such_task_route.rs::task_verb_with_unknown_id_routes_to_task_list` (line 141)
asserts `exit 1` + the byte-equal converged message for `"nonexistent"` — a non-existent
*directory*, so the guard fires; `""` is an existing one. `clap_error_kind_axis.rs` carries no
empty token among its 17 kinds; `not_in_repo_axis.rs:152` drives `task discard` with `"a-task"`.
The fix's test must iterate walk 17's table.

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

`UNPINNED`: the route fence's tests (`crates/cli/src/route_fence.rs:175/231/251`) construct
synthetic argvs in-process and cannot exercise release posture; `no_such_task_route.rs` as above.

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
`config.rs:365-378` says a gitignored root *"keeps working"*). **Half B is UNPINNED:**
`narrate_teardown` (`setup.rs:2211`) iterates `fanout_worktree_paths` + staged task prose only;
`flow48_acceptance.rs::every_destroying_door_names_the_bytes_it_is_about_to_destroy` (line 743)
plants under a worktree and never a tracked file directly under `.jigc/`. M50's choice: refuse
`.jigc` as a placement root, or make `uninstall` narrate tracked children — either way the axis
is *tracked files under `.jigc/`*.

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

`UNPINNED` at the cell: `crates/cli/tests/write_miss_shape_axis.rs` `CELLS` (lines 854-885) carry
`set-slot` at `#no-such-section` but both `set-field` rows at `#no-such-section/link` (a leaf);
the completeness fence at `:1330` requires *some* `write.unknown-section` row per verb and is
satisfied by the leaf-bearing ones. The fix's test adds the section-only row for `--value` and
`--unset`.

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

`UNPINNED`: `crates/cli/tests/milestone_boundary_gate.rs::assert_unfilled_commit_leaf_blocks`
(line 545) drives `--format json` only and asserts exit 3 + `contains("commit:code-area")` +
no-commit + teardown — no code, no text render. `located_finding_text.rs:299` registers the site
as `Carries` and checks only the `at:` locus. `text_json_parity_axis.rs:379` fences the *landed*
envelope. The `HANDLED_COMMIT_LEAVES` axis (line 461) pins *which* leaves gate, not the render.

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

`pinned-by: crates/cli/tests/leftover_probe_fail_closed.rs::every_refusing_door_answers_an_unprobeable_leftover_with_a_code_and_a_route`
(line 178) — plants exactly this subject (`fs::write(&leftover, PRECIOUS)` at
`.jigc/worktrees/area-zed`) and asserts non-success, `blocking · <code>`, `route:` present,
`at:` naming the path, bytes intact — **the class is pinned; the route's content, the sibling
enumeration and the `--force` mention are not** (it asserts `contains("route:")` only).

**M50 rider:** cheap.

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

`UNPINNED`: `project_pack_composition.rs` (7 tests) and flow50 arm 6 seed complete packs only;
`unknown_doctype_axis.rs:368` fenced this leak out of the *doctype* doors (`DOCTYPE_DOORS` ×
`nosuch`) and never reaches a config resource. No test can assert the right pack because the
message structurally cannot name it.

**M50 rider:** cheap; the first door a pack author meets.

## W-6 · `--explain` labels one pack `house/vfs-local` and `house/fs-local` — CONFIRMED (walk 20 capture) · surface · SHIPS RECORDED

The workflow line prefixes every version with `v`, so the `fs-local` sentinel renders as a
non-version. `UNPINNED` (no `--explain` test drives a local pack's label). Cheap.

## W-7 · `describe` carries no origin pack per definition — CONFIRMED · capability gap · SHIPS RECORDED

`kind,id,prose,router_hidden` on the definitions arm; origin is on `--explain` only. Not a lie —
`describe` never claimed origin. The commands arm *does* carry `pack`:
`pinned-by: crates/cli/tests/describe.rs::describe_commands_carry_the_union_of_every_declaring_pack`
(line 499 — `c["pack"].as_str().unwrap_or_else(|| panic!(…))` per entry, membership equal to the
union of both `config/commands.yaml`, 16 + 15 = 31 derived).

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

## Not pinned, and said so — the conversion ledger

| row | status |
|---|---|
| W-13 | `UNPINNED: no test drives an empty id at any door` — owed by the fix wave, iterating walk 17's table |
| W-16 / PT-1 | `UNPINNED: the route fence is debug-only and synthetic` — same test as W-13 |
| W-14 | half A `pinned-by: untrackable_home_axis.rs:272`; half B `UNPINNED: no destroying-door test plants a tracked file directly under .jigc/` |
| W-15 | `UNPINNED: write_miss_shape_axis.rs carries no section-only set-field row` |
| W-1 | `UNPINNED: milestone_boundary_gate.rs drives JSON only; located_finding_text.rs checks the locus, not the prefix` |
| W-2 | class `pinned-by: leftover_probe_fail_closed.rs:178`; the three sub-claims `UNPINNED` |
| W-5 | `UNPINNED: no listed-pack-missing-resource test exists` |
| W-6 | `UNPINNED` |
| W-7 | `pinned-by: describe.rs:499` (the commands arm) |
| plant E's rename preserving status/date/slots (R3, B3) | `UNPINNED: doc_rename_in_task.rs asserts identity movement and the old id's absence, never the header fields' or slot bodies' survival` — worth a row in M50's test batch |

**Every row above carries a citation or a stated reason. The human's gate — no 1.0.0 call until
the ledger is closed — is met by this file only if the M50 wave pins the UNPINNED rows as it
fixes them.**
