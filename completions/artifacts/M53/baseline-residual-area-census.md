---

# M53 · `(3, A3-2)` sibling-cell enumeration — the residual task area

**Binary:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`. **Repo HEAD:** `155054cc` (worktree clean but for untracked `completions/artifacts/M53/`). Every rig: `rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit 1; eval "$rig"` — two-step, `mktemp -d` roots, no teardown, no edits, no cargo. Under `.jigc/` every scan uses `command grep` (the shell's `grep` is `ugrep --ignore-files`, which honours `.gitignore`).

Every claim is marked **DRIVEN** (run on that binary) or **READ** (source at HEAD). **This is a map at one sha, not gospel. I settled nothing.**

---

## 1 · The census — who enumerates, who resolves, where membership is decided

### 1.1 The greps, with counts and misses

| # | command (verbatim) | hits | what it is |
|---|---|---|---|
| P1 | `command grep -rn "list_active_task_ids" crates/engine/src crates/cli/src` | **14** | 1 definition + 6 doc-comment mentions + **7 production call sites** |
| P2 | `command grep -rn 'join("tasks")' crates/engine/src crates/cli/src` | **133** | ~107 are `#[cfg(test)]` fixtures |
| P3 | `command grep -rn 'join("tasks")' crates/cli/src` | **26** | 14 production, 12 test |
| P4 | `command grep -rn "no_such_task" crates/cli/src` | **11** | **4 production refusal sites** + 1 helper + 6 doc-comments |
| P5 | `command grep -rn "could not read the base pin" crates/cli/src crates/engine/src` | **3** | the code-less dead-end's **3 producers**, 2 spellings |

**What P2/P3 would MISS** (the honest bound): every consumer that receives an **already-resolved** `&Path` to an area and never joins the literal `"tasks"` — `engine::state::unwind_area(area, kind)`, `engine::state::foreign_area_paths(area)`, `cli::task::displace_foreign_area(… area …)`, `cli::task::staged_doc_ids(&dir.join("docs"))`, `engine::state::read_base_pin(task_dir)`. Those are *consumers*, not membership-deciders, so the census below is complete for the question asked — but a grep on `"tasks"` alone is not a sweep of "what touches a task area."

### 1.2 The enumerators — (a) who reads `.jigc/tasks/` as a set

| # | site | fn | reached by | what it requires of the dir |
|---|---|---|---|---|
| **E1** | `crates/engine/src/state.rs:1030` | **`list_active_task_ids`** | everything in E2–E7 | `read_dir(tasks)` + **`entry.path().is_dir()`** — nothing more |
| E2 | `crates/cli/src/task.rs:399` | `run_list` | `jigc task list` (+ `--format json`) | E1 |
| E3 | `crates/cli/src/orient.rs:160` | `active_tasks` | `jigc start` (orient, text + json), the `SessionStart` hook | E1 |
| E4 | `crates/cli/src/start.rs:818` | the `also_open` footer | `jigc start "<intent>"`, `jigc start --workflow X "<intent>"` | E1 |
| E5 | `crates/cli/src/doc.rs:6014` | `ActiveTask::resolve`, **enumerated branch** | every `jigc doc` write/read verb **without** `--task` (0 → "start one"; 1 → *implicitly picks it*; >1 → ambiguity list) | E1 |
| E6 | `crates/cli/src/doc.rs:4005` / `:4535` | `stale_read_hint` / `staged_listing_hint` | `jigc doc show` / `doc list` (task-less) stderr hints | E1 + `instance_path(...).is_file()` / `staged_doc_ids` non-empty |
| E7 | `crates/cli/src/rename.rs:1076` | `mid_fan_out_marker` | **`jigc rename`** | E1 — first id ⇒ **refuse `rename.in-flight`** |
| **E8** | `crates/cli/src/task.rs:772` | **`staged_task_prose`** — *its own* `read_dir(tasks_root)` + `path.is_dir()`, **not E1** | `uninstall.staged-prose`, `milestone.staged-prose`, `task-discard.staged-prose` | dir exists; then `docs/` entries matching `staged_doc_id` |

**Record-driven (not disk-enumerated), but they probe each area:**

| # | site | fn | reached by | requires |
|---|---|---|---|---|
| E9 | `crates/engine/src/milestone.rs:1099` | **`reseed_sub_task_areas`** | `milestone add-task`/`add-from-spec`/`provision`/`execute`/`join`/`finalize`/`discard` (not `list-tasks` — `VerbKind::Read`, M49 Inc 2 T4) | **`.exists()` ⇒ SKIP** (plus `item_is_settled`) |
| E10 | `crates/cli/src/milestone.rs:5516` | `cleanup_subtask_areas` | `milestone finalize` (both arms), `milestone discard` | `area.exists()` |
| E11 | `crates/cli/src/milestone.rs:4065` | `discard_foreign_subject` | `milestone discard` guards | recorded ids → `foreign_area_paths` |
| E12 | `crates/cli/src/milestone.rs:6062` | `subtask_patches_and_messages` | `milestone finalize` (`squash: false`) | recorded ids |
| E13 | `crates/cli/src/setup.rs:3578` | `PendingProse::narrate_taken` | the three destroying doors' loss narration | per-path `symlink_metadata` |

**Milestone-area enumerators (2, both mere-dir-existence):**

| # | site | fn | reached by | requires |
|---|---|---|---|---|
| M1 | `crates/engine/src/milestone.rs:848` | `owning_milestone` | orientation's `milestone:` key, `start --task`, `task finalize`'s sub-task refusal, `repo.rs:519`, `setup.rs:2757` | `is_dir()` **and** `read_task_list` OK — so a residual is **invisible** here |
| M2 | `crates/cli/src/rename.rs:1090` | `first_dir_name` | **`jigc rename`** | `is_dir()` alone — a residual **blocks** |

### 1.3 The resolve seams — (b) one id → one area

Four for tasks (the fifth M50 seam is the milestone door-top, `milestone_id()`), **all four the identical predicate**:

| # | site | fn | verbs reaching it | predicate |
|---|---|---|---|---|
| **R1** | `crates/cli/src/task.rs:1892` | `TaskArea::resolve` | `task validate` · `task finalize` · `task diff` · `task discard` · `task bind` · `sweep_for_orientation` | `if !dir.is_dir() { no_such_task(id) }` |
| **R2** | `crates/cli/src/start.rs:2199` | `resume_in_repo` | `jigc start --task <id>` | same |
| **R3** | `crates/cli/src/start.rs:2327` | the workflow re-entry | `jigc workflow <W> --task <id>` | same (its own route: `milestone list-tasks`) |
| **R4** | `crates/cli/src/doc.rs:6001` | `ActiveTask::resolve`, explicit branch | every `jigc doc` verb with `--task <id>` | same |

### 1.4 Where membership is DECIDED

**Two predicates, both `is_dir()`, and only one of them is shared.**

- **`engine::state::list_active_task_ids` (state.rs:1030) is the single enumeration source** — its own doc-comment claims exactly that: *"This is the single enumeration source of truth the active-task resolution (`jigc doc`), the `jigc task list` roster, and the ambiguous-task error all read, so they never disagree on which tasks are live."* (READ). E2–E7 all read it. **One line changed there moves seven doors.**
- **E8 `staged_task_prose` is a second, independent `read_dir` of the same root** (READ, `task.rs:772`) — deliberately (it is scoped by `only:` and fails closed), but it is *not* behind E1.
- **The four resolve seams R1–R4 each carry their own `!dir.is_dir()`** — four copies of one predicate, converging only on the shared *message* (`no_such_task`), not on a shared predicate. A truthful predicate would have to land at all four, or at a new shared one they call.
- **E9/E10 key on `.exists()`**, not `is_dir()` — a *file* named `.jigc/tasks/<id>` would be "present" to them and "not a task" to E1–E7 (READ, unexercised).

So: **there is one shared enumerator and four independent by-id predicates.** The truthful predicate has to live in **two** homes (`list_active_task_ids` + one shared resolve predicate the four seams call) — or the seams keep answering differently from the roster, which is the state today.

---

## 2 · What a legitimate task area is guaranteed to contain — per mint door, DRIVEN

`engine::state::MINT_DOORS` (state.rs:1234) has 5 rows. Each built through the real binary and `ls -la`'d immediately after the mint.

| `MINT_DOORS` row | command driven | files present at cold start | `base.json`? |
|---|---|---|---|
| `jigc start "<intent>"` (`start.rs::mint_in_repo`, `Snapshot::Written`) | `jigc start --workflow quick-fix "tidy the readme"` | `base.json` · `docs/` (`commit:<id>.md`, `provenance.json`) · `intent` · `staged-snapshot.json` · `workflow` | **yes** |
| `jigc migrate <path> --as <ty>` (`start.rs::mint_migration_in_repo`, `Written`) | `jigc migrate OLD-DECISION.md --as adr` | the above **+ `source` + `source-path`** | **yes** |
| `jigc milestone create` (`milestone.rs::run_create`, `Written`) — **milestone area** | `jigc milestone create "axis three probe"` | `.jigc/milestones/<id>/`: `base.json` · `record-commit-msg.txt` · `staged-snapshot.json` · `tasks.json` | **yes** |
| `jigc milestone add-task` (`engine::milestone::add_task`, **`Snapshot::Exempt`**) | `jigc milestone add-task axis-three-probe "first sub"` | `base.json` · `intent` · `workflow` — **no `docs/`, no `staged-snapshot.json`** | **yes** |
| the record-driven **re-seed on a fresh clone** (`engine::milestone::reseed_sub_task_areas`, **`Exempt`**) | `git clone` → `jigc milestone provision axis-three-probe` in the clone | `base.json` · `intent` · `workflow` | **yes** |

```
# the re-seed cell, verbatim (DRIVEN)
$ C=$(mktemp -d …); git clone -q "$REPO" "$C/repo"; cd "$C/repo"
$ ls .jigc                      ->  .gitignore AGENT.md config version      # no tasks/
$ jigc milestone list-tasks axis-three-probe
  milestone:axis-three-probe tasks (2): first-sub, second-sub               # a READ — reseeds NOTHING
$ find .jigc/tasks               ->  No such file or directory
$ jigc milestone provision axis-three-probe
  provisioned 2 worktree(s) … at base 51c5f06 (first-sub, second-sub)
$ find .jigc/tasks -mindepth 1 | sort
  .jigc/tasks/first-sub/{base.json,intent,workflow}
  .jigc/tasks/second-sub/{base.json,intent,workflow}
```

### 2.1 The discriminator question, answered

**`base.json` is present in every legit area produced by every one of the five mint doors (DRIVEN, all five), it is written FIRST by the mint (READ), and it is removed by `unwind_area` (READ).**

`mint_task`'s write order — `crates/engine/src/state.rs:997-1021` (READ):

```rust
let dir = jigc_root.join("tasks").join(&id);
if dir.exists() { return Err(collision_finding(&id)); }         // :1001
std::fs::create_dir_all(&dir)…?;                                 // :1005
let pin_path = dir.join(BASE_PIN_FILE);
std::fs::write(&pin_path, body)…?;                               // :1009   ← FIRST content write
std::fs::write(dir.join(INTENT_FILE), intent)…?;                 // :1013
std::fs::write(dir.join(WORKFLOW_FILE), workflow_id)…?;          // :1018
```

`mint_milestone` (`engine/milestone.rs:296-309`, READ) is the same shape: `create_dir_all` → `base.json` → `tasks.json`.

`unwind_area` (`engine/src/state.rs:385`) removes every `WorkArea::jigc_written()` member; `BASE_PIN_FILE` is `TASK_AREA_FILES[0]` (`state.rs:129-143`) **and** `MILESTONE_AREA_FILES[0]` (`state.rs:168-174`). So the M52 mint-unwind residual and the proposed A3-2 residual both lack it by construction (READ).

**The window** — two syscalls wide, `create_dir_all` → `fs::write(base.json)`. A `list_active_task_ids` racing a mint can see a dir with no pin. **This window exists today too** (a racer sees no `workflow`/`intent` and E2/E3 render blank/`null` — DRIVEN below as the residual's own rendering), so a base-pin-keyed predicate does not create a flicker class, it **narrows** the existing one from three writes to one.

**One asymmetry worth naming (READ, not driven):** `mint_task` writes `base.json` with plain `std::fs::write` (`state.rs:1009`), while `mint_milestone` uses the atomic `crate::state::persist` (`milestone.rs:301`). A predicate keyed on base.json **existence** is unaffected; one keyed on base.json **parsing** has a torn-read window at the task door and not at the milestone door.

**Cheaper shape-hole (READ, unexercised):** `unwind_area`'s member removal is `else if shape.is_file()`, so a *foreign directory* named `base.json` survives the unwind — an existence-keyed predicate would call that residual a task. A `symlink_metadata(…).is_file()` leg closes it, and `unwind_area` itself already states *"Shape is part of membership."*

---

## 3 · Today's behaviour — door × shape, DRIVEN

Shapes: **A** = `mkdir .jigc/tasks/<id>` (empty) · **B** = dir holding one foreign file (`notes.txt`) · **C** = dir holding only `docs/adr:something.md` (looks like staged prose).

### 3.1 Plain task id (rig `fresh`, ids `ghost-a`/`ghost-b`/`ghost-c`)

| door | A (empty) | B (foreign file) | C (foreign `docs/*.md`) |
|---|---|---|---|
| `task list` | **0** — `jigc task list — 3 active task(s)` / `ghost-a` (no workflow/intent columns) | same | same |
| `task list --format json` | **0** — `{"id":"ghost-a","workflow":null,"intent":""}` | same | same |
| `start` (orient, text) | **0** — `Active task: ghost-a` / `workflow: none recorded` / `staged: nothing staged yet` / `findings: none`, + 4 `Run:` routes incl. `task finalize ghost-a` | same | **0** — `staged: adr:something`, **`findings: 4 blocking, 1 advisory`**, discard route carries `--force` |
| `start --format json` | **0** — `"state":"active-task"`, `"base": null`, `"staged": []`, `"findings": []` | same | **0** — `"staged":["adr:something"]` + 5 findings incl. 4 `conformance.section-missing` |
| `start "<new intent>"` — `also open:` | **0** — listed: `` - `ghost-a` — resume it with `jigc start --task ghost-a` `` (no `(workflow …)` parenthetical) | same | same |
| `start --task <id>` | **1** — **code-less**, route-less: `could not read the base pin for task \`ghost-a\`: No such file or directory (os error 2)` | same | same |
| `task validate <id>` | **0** — `no findings — the task validates clean` | **0** — same | **3** — 4 blocking `conformance.section-missing` over the foreign `.md` |
| `task validate --format json` | **0** — `{"schema_version":3,"findings":[]}` | same | findings array |
| `task finalize <id>` | **1** — **code-less**, **host-absolute path**: `could not read the base pin at "/private/var/folders/…/repo/.jigc/tasks/ghost-a/base.json"` | same | same |
| `task finalize --format json` | **1** — flattened `{"error":"could not read the base pin at \"/private/var/…\"…"}` | same | same |
| `task diff <id>` (+ json) | **1** — same code-less + host path, same flattened `{"error":…}` | same | same |
| `workflow <W> --task <id>` | **1** — code-less base-pin | same | same |
| `task bind <role> <addr> <id>` | **1** — code-less, **host-absolute**: `task at "/private/var/…/ghost-a" has no recorded workflow — discard it and re-start with \`jigc start\`` | same | same |
| `doc list --task <id>` | **0** — `jigc doc list — no docs staged in task ghost-a` | same | **0** — `adr:something  docs/decisions/something.md  managed` |
| `doc show <addr> --task <id>` | **1** — `store.not-staged` (correct shape) | same | **1** — `store.unparseable` over the foreign file, route *"fix the staged working copy"* |
| **any `doc` write verb** `--task <id>` | **1** — `the active task has no recorded workflow — discard it with \`jigc task discard ghost-a --force\` and re-start with \`jigc start\`` (code-less; **no resurrection** — `find .jigc/tasks/ghost-a` is still empty after) | same | same |
| `uninstall` | not named by any guard | **1** — **`uninstall.foreign-bytes`**, names `.jigc/tasks/ghost-b/notes.txt` | **1** — **`uninstall.staged-prose`**, names `ghost-c: adr:something` |
| `task discard <id>` | **0** — `discarded task ghost-a`, area gone | **1** — `task-discard.foreign-bytes` + route; `--force` → **0**, warns, removes | **1** — `task-discard.staged-prose` + route; `--force` → **0** `discarded task ghost-c — dropped staged edits to: adr:something` |
| **`jigc start "<intent slugging to the id>"`** | **1** — `blocking · task.serial-collision — task \`ghost-c\` is already active` / `route: resume with \`jigc start --task ghost-c\` or abandon with \`jigc task discard ghost-c --force\`` | same | same |
| `jigc rename <addr> --to "…"` | **1** — `blocking · rename.in-flight — cannot rename while task \`first-sub\` is in flight` | same | same |

### 3.2 Sub-task id of a **JOINED** milestone (`joined-probe`, landed boundary `ad17ec4`, then `mkdir .jigc/tasks/first-sub`)

| door | result |
|---|---|
| `task list` / `start` | **0** — `2 active task(s): first-sub, second-sub`; orientation shows `milestone: null` (M1 `owning_milestone` reads the erased `.jigc/milestones/` cache) and offers `Run: jigc task finalize first-sub` |
| `task finalize first-sub` | **1** — **code-less base pin + host path**. The `finalize.milestone-sub-task` refusal **does not fire**, because it too keys on M1 |
| `milestone list-tasks` · `provision` · `execute` · `join` · `finalize` · `discard` (all 6) | **1** each — `blocking · milestone.terminal — milestone \`joined-probe\` is \`joined\`…`, route `jigc doc show milestone-record:joined-probe`. **Nothing re-seeds.** |
| **`task discard first-sub`** (shape A, **no `--force`**) | **0** — `discarded task first-sub` / `record commit: 5531638 — this sub-task's milestone record, settled to \`discarded\` and committed on its own`. Record now reads `status: joined` (header) with `- status: discarded` on a sub-task whose work landed in `ad17ec4`. **A permanently committed lie through a committing door at exit 0.** |
| `task discard second-sub` (shape B) | **1** `task-discard.foreign-bytes` → `--force` → **0**, `record commit: 03fd473` — same lie, `--force` is not a barrier to it |
| `task discard first-sub` (shape C, re-planted, item **already** `discarded`) | **1** — and the frame is a **lie**: `` `git commit` was rejected (no commit was made): / On branch main / nothing to commit, working tree clean / … Fix the hook's complaint, then re-run … `` — no hook rejected anything; the flip is a byte no-op so git had nothing to commit. Area survives, now holding jigc's own `record-commit-msg.txt` |

**The two-resolver disagreement that makes this reachable (READ):** `engine::milestone::owning_milestone` (`milestone.rs:846`) reads the `.jigc/milestones/` **cache** — erased by a landed boundary → `None`. `cli::milestone::recording_milestone` (`milestone.rs:2359`) reads the **committed record** → finds `joined-probe`. So every *guard* keyed on the cache goes quiet while the *committing* door keyed on the record proceeds. And `engine::milestone::discard_sub_task_item`'s doc-comment (`engine/milestone.rs:1551-1555`, READ) states the premise: *"this arm is reachable only while the sub-task has a live working area, and a `joined` item's area no longer exists … **A guard here would be code no state can reach.**"* — **driven false above by a bare `mkdir`, on today's binary, with no fix applied.**

### 3.3 Sub-task id of a still-OPEN milestone, legit sibling present (`open-probe`: `alpha-sub` residual, `beta-sub` legit)

| door | result |
|---|---|
| `task list` | **0** — `alpha-sub` (bare) and `beta-sub  [sub-task]  beta sub` — the residual renders **without** the `[sub-task]` tag or intent |
| `milestone list-tasks open-probe` | **0** — `tasks (2): alpha-sub, beta-sub` (from the record) |
| **`milestone provision open-probe`** | **0** — `provisioned 2 worktree(s)` — **and the residual is NOT re-seeded**: `find .jigc/tasks` still shows only `alpha-sub/leftover.txt`. E9's `.exists() ⇒ skip` |
| `start --task alpha-sub` / `workflow sub-task --task alpha-sub` | **1** — code-less base pin |
| `task validate alpha-sub` | **0** — *validates clean* |
| `task finalize alpha-sub` | **3** — `finalize.milestone-sub-task` **correctly** (the cache is live here, so M1 resolves) |
| `milestone execute open-probe` | **0** — composes the fan-out walk, `Spawn:` line for `alpha-sub` included |
| `milestone join` | **0** — `joined milestone:open-probe — 0 doc(s) merged` |
| `milestone finalize` | **3** — `milestone.zero-contribution` (a consequence of the sibling not having worked, not of the residual) |
| `milestone discard open-probe` | **1** — `milestone.foreign-bytes`, names `.jigc/tasks/alpha-sub/leftover.txt` |
| `task discard alpha-sub --force` | **0** — `record commit: 98918af`. Here the flip is **legitimate** (the item really was `active`) |

**So (4c) is answered DRIVEN: no door re-seeds into a residual, and nothing mixes.** The cost is the mirror image — **a genuinely active sub-task whose area became a residual is unrecoverable by any milestone door**: `provision` gives it a worktree, `execute` names it in a `Spawn:` line, and its own `start --task` dead-ends code-lessly, forever, in that checkout. A fresh clone heals it (the residual is gitignored and not cloned — DRIVEN, §2).

### 3.4 The milestone-area twin

| cell | result |
|---|---|
| residual `.jigc/milestones/joined-probe/leftover.txt` (record **terminal**) — `task list` / `start` | **0** — **silent**. No enumerator lists milestone areas (M1 requires a readable `tasks.json`) |
| all 6 milestone doors | **1** — `milestone.terminal`, from the record |
| `milestone create "joined probe"` | **1** — `milestone.record-exists` (from the record, not the dir) |
| `uninstall` | **1** — `uninstall.foreign-bytes`, names `.jigc/milestones/joined-probe/leftover.txt` |
| residual `.jigc/milestones/never-existed/` with **no record** — `milestone create "never existed"` | **1** — `blocking · milestone.serial-collision — milestone \`never-existed\` already exists` / route `jigc milestone add-task never-existed "<intent>"` — **which dead-ends**: `milestone add-task never-existed "a task"` → **1** `milestone.area-io` |
| `milestone list-tasks never-existed` | **1** — code-less `could not read the task list for milestone \`never-existed\`` |
| **`jigc rename <addr> --to "…"`** over a **terminal**-record milestone-area residual | **1** — `rename.in-flight — cannot rename while milestone \`joined-probe\` is in flight` / route `jigc milestone finalize joined-probe` … `jigc milestone discard joined-probe` — **both refuse `milestone.terminal`. The route is unsatisfiable and no jigc verb clears the directory.** `jigc rename` is permanently wedged until a human `rm -r`s it |

That last row **corrects the advocate's §1(d) concession in one cell**: the advocate drove the six milestone doors and the two task surfaces and concluded *"the milestone-area residual creates zero lying state."* It does — but M2 (`rename.rs:1090 first_dir_name`) is a **seventh** reader neither list covered, and it is the one that wedges.

---

## 4 · The sibling cells a truthful predicate would create

### (a) If the enumerators skip a residual, what still names it?

**Driven today — the by-id doors do NOT go through E1.** R1–R4 each test `dir.is_dir()` directly. So a fix confined to `list_active_task_ids` leaves every clearing route alive:

| clearing route | today | after an enumerator-only fix |
|---|---|---|
| `jigc task discard <id>` | R1, `is_dir()` — resolves all three shapes | **unchanged** — still resolves |
| `uninstall.foreign-bytes` | names shape B/its bytes (DRIVEN) | unchanged (E8 is its own `read_dir`) |
| `uninstall.staged-prose` | names shape C's `.md` (DRIVEN) | unchanged |
| `milestone.foreign-bytes` at `milestone discard` | names a sub residual's bytes (DRIVEN) | unchanged |
| `task.serial-collision` at the mint | names the id (DRIVEN) | see (b) |
| `rename.in-flight` | names the id (DRIVEN) | **would stop naming it** if E7 reads the truthful E1 |

**Shape A is the hole.** An empty residual is named by *no* guard (nothing to lose) — only by `task list` / `start` / `also open:`. Take those away and a shape-A residual becomes invisible to every surface, while still refusing the mint at `task.serial-collision` and (unless E7 changes too) wedging `jigc rename`. **That is a real regression candidate: an id that refuses a mint and is listed nowhere.** The cheapest answer already exists — keep the id in the roster with a truthful label rather than dropping it — but that is the human's fork, not mine.

### (b) A mint whose slug equals the residual's id — DRIVEN today

```
$ jigc start --workflow quick-fix "ghost c"
blocking · task.serial-collision — task `ghost-c` is already active
  at: task:ghost-c
  route: resume with `jigc start --task ghost-c` or abandon with `jigc task discard ghost-c --force`
EXIT=1
```
Both route arms behave differently from what the message implies: `start --task ghost-c` **dead-ends code-lessly at exit 1**, and `task discard --force` works. Once a residual is "not a task", `mint_task`'s `if dir.exists()` (`state.rs:1001`) either (i) keeps refusing — then the message *"task `ghost-c` is already active"* becomes a **law-1 lie**, and the route's first arm stays a dead end; or (ii) adopts the directory and mints into it — **which mixes jigc's files with foreign bytes at a door that previously could not**, and re-opens the class `unwind_area` exists to close. **This is the sharpest new fork the fix creates.** (iii) — a third identity, "the id is taken by a leftover, here is how to clear it" — is expressible with today's `task.serial-collision` code and a rewritten message + route.

### (c) `reseed_sub_task_areas` re-seeding into a residual — **DRIVEN, refuted**

`engine/milestone.rs:1099`: `if jigc_root.join("tasks").join(&item.id).exists() { continue; }` runs **before** `item_is_settled`. Driven in §3.3: `milestone provision` over a residual `alpha-sub` re-seeded nothing and mixed nothing. On a fresh clone the residual does not exist (gitignored), so the re-seed is normal (DRIVEN, §2). **No mixing cell exists — but its mirror does** (§3.3: the residual is a permanent local dead end for a genuinely active sub-task, and E9's skip is what makes it permanent). If the truthful predicate is applied at E9 (`.exists()` → `has_base_pin()`), that dead end **self-heals** — which is an argument *for* touching E9, and also a new cell: the re-seed would then write `base.json`/`intent`/`workflow` **beside** the foreign bytes, which is the mixing (c) asked about, arriving through the fix rather than through today's code.

### (d) Milestone-area residuals — **driven above**, §3.4. Two enumerators (M1 cache-keyed and silent; M2 `rename.rs:1090` dir-keyed and blocking). The terminal case wedges `jigc rename` with an unsatisfiable route; the record-less case wedges `milestone create` with a dead-end route.

### (e) `LeftoverShape` / the destroying doors' classifier over a residual

**READ:** `LeftoverShape` / `leftover_at` (`milestone.rs:3336`, `:3462`) has `.jigc/worktrees/<id>` as its subject, **not** task areas — `discard_foreign_subject`'s own doc-comment says so: *"The fan-out worktrees are **not** here: they are `probe_leftover`'s subject."* A residual task area is classified instead by `engine::state::foreign_area_paths` (the writer-registry complement). **DRIVEN:** that complement answers correctly over all three shapes — B's `notes.txt` is foreign, C's `docs/adr:something.md` is **jigc's own** by the `staged_doc_id` name rule (the deliberate keep-too-much choice, `state.rs:145-160`), which is why C lands on `*.staged-prose` and not `*.foreign-bytes`. `DESTROYING_DOORS` rows are unmoved by a residual: `TASK_DISCARD_DOOR`/`UNINSTALL_DOOR`/`DISCARD_DOOR` all `Refuse`, `TASK_FINALIZE_DOOR`/`FINALIZE_DOOR` are `Displace` with `codes: &[]`.

### (f) Concurrency — `task list` racing a mint

**READ**, §2: the window is `create_dir_all` → `fs::write(base.json)`, two syscalls. Today's `is_dir()` predicate admits a **three-write** window (a racer sees a dir with no `workflow`/`intent` and E2/E3 render it exactly as they render a residual — `workflow: null`, `intent: ""` — which is precisely why a residual is indistinguishable from a mid-mint task on today's surfaces). **A base-pin-keyed predicate narrows the flicker rather than creating it**, and flickers in the safe direction: a just-minted task is briefly absent from the roster instead of a finalized one being briefly present. No concurrent racer is driven here — jigc's own model (`design/storage.md` → Concurrent writers) names the hook as the designed racer, and no hook runs inside `mint_task`.

---

## 5 · Shipped precedent for the identity a by-id door should answer with

**DRIVEN — the unknown-id cell, on this binary, at seven doors:**

```
$ jigc task validate nope-nope
blocking · finalize.no-task — no task `nope-nope`
  at: task:nope-nope
  route: `jigc task list` lists the live tasks
EXIT=1
```
Identical bytes at `task finalize` · `task diff` · `task discard` · `start --task` · `doc list --task`. `workflow <W> --task` carries the same code with its own route (`jigc milestone list-tasks <milestone-id> lists a milestone's sub-tasks`). The `--format json` arm:

```json
{"schema_version":3,"findings":[{"severity":"blocking","probe":"finalize","check":"no-task",
 "code":"finalize.no-task","key":{"code":"finalize.no-task","target":"task:nope-nope"},
 "message":"no task `nope-nope`","location":{"address":"task:nope-nope","line":1,"col":1},
 "route":"`jigc task list` lists the live tasks"}]}
```

**READ:** one producer — `engine::finalize::no_such_task_finding(id, route)` (`finalize.rs:1290`), wrapped by `cli::task::no_such_task` (`task.rs:1445`) with a `Route::mechanical(["jigc","task","list"], …)`, called at R1/R2/R4, and constructed directly at R3 with its own route. It is the M51 Increment 6 sweep's output across all 25 `WORK_UNIT_ID_DOORS` (`cli.rs:2589`).

**Codes that mean "this id names no live task":** exactly one — **`finalize.no-task`**. No `work-unit.*` member covers it (`work-unit.malformed-id` is the *not-an-id* family, `task.rs:1467`). So:

> **"residual ⇒ unknown task, with a route naming the leftover path" is expressible with a shipped code and needs no new one** — the finding takes an arbitrary `Route` as a parameter, so a residual-specific route (`jigc task discard <id>` / `--force`, naming the path) drops in without minting an identity. What it does need is an answer to law 1: `finalize.no-task`'s message is *"no task `<id>`"*, and the directory is right there — so the **message** changes, which is a wire-visible change to a shipped code's text (not its key) at 25 doors. That is the human's call, not mine.

**Countervailing datum:** three doors do **not** answer `finalize.no-task` over a residual today — they answer the flattened, code-less `{"error": "could not read the base pin at \"<host absolute path>\""}` (`task finalize`, `task diff`) / `{"error": "could not read the base pin for task \`<id>\`"}` (`start --task`, `workflow --task`). **Three producers, two spellings, both law-1 host-path violations, none in `ERROR_CODE_REGISTRY`:** `start.rs:2220`, `start.rs:2345`, `task.rs:1928`. A truthful predicate at the resolve seams **deletes this class outright** — the residual never reaches `base()`.

---

## 6 · Standing fences and doc homes the fix must keep true or revise

### 6.1 Tests

| fence | what it pins | effect of a truthful predicate |
|---|---|---|
| **`crates/cli/tests/flow37_rename.rs:815` `mid_fan_out_active_task_blocks`** | plants `fs::create_dir_all(".jigc/tasks/some-task")` — **shape A verbatim** — and asserts `jigc rename` **blocks** | **the one standing test that pins "area exists ⇒ active".** It reddens if E7 reads a truthful E1. Its sibling at `:1746` plants the same shape |
| `crates/cli/tests/task_list.rs` | drives `jigc task list` with 0/1/2 **real** tasks; ids render, empty roster clean at exit 0, `--format json` array | no residual arm exists — the class is untested |
| `crates/cli/tests/orientation_active_task.rs` | M50 Inc 5 T2 — the `active-task` tag's seven keys, the four `Run:` directives, two tasks as two rows, and a task-less project rendering `Clean` **byte-identical** to a discarded-task corpus | the byte-identity arm is the one that would move: a residual-holding corpus would have to render `Clean` too |
| `crates/cli/tests/work_unit_unknown_envelope.rs` | the unknown-id cell over `WORK_UNIT_ID_DOORS` — `finalize.no-task` as a **key**, not a flattened string | the residual cell would join this axis; the suite's subject is already registry-derived |
| `crates/cli/tests/no_such_task_route.rs` | M43 Inc 1 T5 — the three refusals are **byte-identical** and the extracted route is **run verbatim** | a residual-specific route would have to keep the byte-identity claim or restate it |
| `crates/cli/tests/subtask_discard_record.rs` | M49 Inc 2 T3 — the record write **and** the reseed skip, with `DOOR_ARMS` bijected against the reseed call sites in `milestone.rs` | the §3.2 false flip is this suite's exact subject, one state it never plants |
| `crates/cli/tests/task_area_writer_registry.rs` | count-fences `TASK_AREA_FILES` (13) / `MILESTONE_AREA_FILES` (5) against production `join(<name>)` sites in **both** crates | untouched — a base-pin predicate adds no writer |
| `crates/cli/tests/work_unit_id_axis.rs` | 25 doors × 4 token cells | the residual is a fifth cell of the same axis |
| `crates/cli/tests/malformed_work_unit_id.rs`, `route_followability.rs`, `compose_goldens.rs` | carry `active task(s)` / `Active task:` strings | goldens to re-bless if the roster line moves |

### 6.2 Doc homes — what "an active task IS", quoted

| home | sentence |
|---|---|
| **`design/storage.md:291`** | *"The **teardown itself is not skippable** — a left-over area makes `jigc task list` report an active task that finalized — so a move that cannot be made is said out loud rather than turned into a silent retry."* — **the premise is driven true throughout §3, and the fix's whole content is making the teardown skippable** |
| `design/storage.md:9` | *"The committed Markdown at type locations is the only source of truth. Every other artifact — the staging working area, the edge index, the file↔state hashes — is either a rebuildable cache or a transient working copy."* |
| `design/storage.md:121` | `tasks/<task-id>/   #   gitignored — per-task working area (staging)` |
| `design/storage.md:289` | *"the working area itself is never committed (it persists on disk across sessions, so work resumes, but is lost on a clean/clone — acceptable for staging)"* |
| `design/finalize.md:157` | *"The removal itself is **never skipped** — a left-over area makes `jigc task list` report an active task that finalized."* |
| `design/finalize.md:159` | *"a stale `.jigc/tasks/<id>/` gets cleaned up by `jigc task discard <id>` or the next `finalize` reusing the slot"* — **first clause driven TRUE (§3.1); second clause driven FALSE** (re-finalize exits 1 on the missing pin; the mint refuses `task.serial-collision`) |
| **`design/write-commands.md:180`** | bare `jigc start` *"**orients**: … *either* the workflow catalog … *or* the **active set** — every live task with its workflow, intent, base pin, staged docs and findings, and the routes that resume, preview, commit or abandon it"* — **the definitional sentence; a residual has none of the five and gets all four routes** |
| `design/write-commands.md:33` | *"a task is born at `jigc start` … and then managed with `jigc task <verb> <id>`"* — birth is the mint, and the mint writes the pin |
| `design/team-ready-state.md:84-88` | *"The working area's two populations"* — jigc's writer registry and its complement. **The membership home; a residual is an area whose jigc population is empty** |
| `crates/engine/src/state.rs:1024-1029` (doc) | *"the **single enumeration source of truth** the active-task resolution (`jigc doc`), the `jigc task list` roster, and the ambiguous-task error all read, so they never disagree on which tasks are live"* |
| `crates/engine/src/milestone.rs:1551-1555` (doc) | *"this arm is reachable only while the sub-task has a live working area, and a `joined` item's area no longer exists … **A guard here would be code no state can reach.**"* — **driven false at HEAD, §3.2, by `mkdir`** |
| `crates/cli/src/milestone.rs:3097` (doc) | *"Every door **answers for what it removes** … a door that destroys what it never named is the law-1 half-truth"* |

---

## 7 · Ledger summary

| capability | status | evidence | gap |
|---|---|---|---|
| `engine::state::list_active_task_ids` is the one enumerator 7 production sites read | **built + proven** | READ `state.rs:1030`, P1 = 14 hits / 7 call sites; DRIVEN — `task list`, `start`, `also open:` all reported the same 3 residuals | — |
| "is this a task" is **one** shared predicate | **shape-limited** | READ — E1 is shared; R1–R4 each carry their own `!dir.is_dir()`; E8 its own `read_dir` | a truthful predicate needs **2 homes** (enumerator + a shared resolve predicate), or the roster and the seams diverge |
| every legit task area carries `base.json` from the first instant, at all 5 `MINT_DOORS` | **built + proven** | DRIVEN §2, all five doors incl. the `Exempt` `add-task` and the fresh-clone re-seed; READ `state.rs:1005-1019` — written first, one syscall after `create_dir_all` | write window = 2 syscalls; `mint_task` uses non-atomic `fs::write` while `mint_milestone` uses `persist` (READ) |
| `base.json` is removed by `unwind_area` | **built + proven** | READ `state.rs:385` + `TASK_AREA_FILES[0]`/`MILESTONE_AREA_FILES[0]` | `else if shape.is_file()` ⇒ a foreign **directory** named `base.json` survives (READ, unexercised) |
| a residual is reported as an **active task** by `task list` / `start` / `also open:` / `doc`'s implicit resolution | **latent defect** | DRIVEN §3.1–§3.3 — all four, all shapes, exit 0 | the class the fix must close |
| a residual **validates clean** at `jigc task validate` | **latent defect** | DRIVEN §3.1 (A/B) exit 0 *"the task validates clean"* | validate never reads the pin |
| four by-id doors dead-end **code-lessly** with a **host-absolute path**, incl. `--format json` | **latent defect** | DRIVEN §3.1 + §3.5 json arms; READ — 3 producers, 2 spellings (`start.rs:2220`/`:2345`, `task.rs:1928`), none in `ERROR_CODE_REGISTRY` | law 1 + the flattened-`{"error"}` shape M51 Inc 6 removed at 25 doors, surviving on this path |
| `jigc task discard <sub>` commits a **false `discarded`** on a **joined** sub-task, exit 0, **no `--force`** | **latent defect (tier 1)** | DRIVEN §3.2 — `mkdir` → `record commit: 5531638`, record `status: joined` + `- status: discarded`; root cause READ (`owning_milestone` cache-keyed vs `recording_milestone` record-keyed) | the advocate's §2 cell, reproduced on the **unfixed** binary |
| a second `task discard` over an already-settled item dresses git's empty commit as a **hook rejection** | **latent defect (new — in no ledger, brief or advocate)** | DRIVEN §3.2 — *"`git commit` was rejected (no commit was made) / nothing to commit, working tree clean / Fix the hook's complaint"*, no hook involved | `task discard` is a `COMMITTING_DOORS` member (READ, `invocation_log.rs:171`); M48 swept the empty-commit outcome at `rename` — this door's record-only commit is not covered |
| `reseed_sub_task_areas` re-seeds INTO a residual, mixing jigc files with foreign bytes | **refuted** | DRIVEN §3.3 — `provision` over a residual `alpha-sub` seeded nothing; READ `engine/milestone.rs:1099` `.exists() ⇒ continue` | the mirror is real: a genuinely active sub-task's residual is a **permanent local dead end**, healed only by a fresh clone |
| the milestone-area residual creates **zero** lying state | **refuted in one cell** | DRIVEN §3.4 — the advocate's six doors + two task surfaces re-confirmed, but `jigc rename` blocks `rename.in-flight` with a route (`milestone finalize`/`discard`) both arms of which refuse `milestone.terminal`; **no jigc verb clears it** | a **seventh** reader, `rename.rs:1090 first_dir_name`, in neither list |
| a record-less milestone-area residual refuses `milestone create` with a dead-end route | **latent defect** | DRIVEN §3.4 — `milestone.serial-collision` → route `milestone add-task` → `milestone.area-io` | |
| the mint refuses over a residual with `task.serial-collision`, route arm 1 a dead end | **latent defect** | DRIVEN §3.1 — `jigc start --workflow quick-fix "ghost c"` exit 1 | **the sharpest fork the fix creates** — refuse-and-lie, adopt-and-mix, or a third message |
| a `doc` **write** verb resurrects a residual into a half-task | **refuted** | DRIVEN §3.1 — `doc create adr --task ghost-a` exit 1, `find` still empty afterwards; the refusal even names the right route (`task discard … --force`) | its message is code-less and `task bind`'s carries a host-absolute path |
| every residual is clearable by a shipped door with a shipped finding and route | **built + proven** | DRIVEN §3.1 — A: `task discard` exit 0; B: `task-discard.foreign-bytes` → `--force`; C: `task-discard.staged-prose` → `--force`; `uninstall` names B and C through its own two guards | **shape A is named by no guard** — if the enumerators stop listing it, it is invisible to every surface while still refusing the mint |
| `finalize.no-task` is the one shipped identity for "this id names no live task", key `(code, target)`, at all 25 doors | **built + proven** | DRIVEN §5, seven doors + the json envelope; READ one producer, `finalize.rs:1290`, route as a parameter | expressible without a new code; its **message** would change at 25 doors |
| a standing test pins "area exists ⇒ active" | **built + proven (the fence exists)** | READ `crates/cli/tests/flow37_rename.rs:815` — `create_dir_all(".jigc/tasks/some-task")` then asserts `jigc rename` blocks | **it reddens** under a truthful E1; its sibling at `:1746` too |
| `design/storage.md:291` / `design/finalize.md:157` state the rule the fix inverts | **built + proven (the statements)** | READ, quoted §6.2; premise DRIVEN true | 2 design homes + `task.rs:5839-5843` + `post_commit`'s doc + `finalize_displacement.rs`'s module doc; **`finalize.md:159`'s second clause is driven FALSE already** |
| `engine::milestone::discard_sub_task_item`'s *"a guard here would be code no state can reach"* | **latent defect (the comment)** | DRIVEN §3.2 on the **unfixed** binary | the premise is already false at HEAD; the fix does not falsify it, it is falsified |

**Honest bounds on this report.** No concurrent racer was driven (§4f is READ). The foreign-directory-named-`base.json` shape and the `.exists()`-vs-`is_dir()` divergence at E9/E10 are READ, unexercised. `add-from-spec` was not driven as a mint door (it is not a `MINT_DOORS` row; it reaches `add_task`). The `--strict-permissions` and non-`fresh` rig states were not exercised. Every table row above is a fact about `jigc 1.0.0-rc.16` at `155054cc` and nothing else.
