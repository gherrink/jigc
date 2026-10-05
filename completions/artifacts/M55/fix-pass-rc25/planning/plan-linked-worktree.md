## 0. Bound on this report: nothing was driven

I did not drive the rc.24 binary. My operating mode forbids creating files anywhere, temp directories included, and a rig is a new repository. Every claim about today's behaviour is therefore marked:

- **[read]** — source, with file and function. `git diff --stat jigc-v1.0.0-rc.24 HEAD -- crates` is recorded empty in R3-F7, so a read of `crates/` is a read of rc.24 (other agents' `setup.rs` churn excepted; I did not read that file).
- **[pinned]** — an existing test in the tree asserts it; I did not run it.
- **[brief]** — handed to me as driven in scratch; not re-driven.
- **[owed-drive]** — the fixer must drive this (argv given) before relying on it.

Paths are rooted at `$REPO`, the repository root.

## 1. The predicate

Not a path-shape test. The class is: **the task's commit boundary lands in a checkout other than the one the doc store binds to.**

- `task finalize` promotes into the standing checkout but reads the store from `jigc_home` **[read]** `task.rs::TaskArea::finalize` passes `&self.jigc_home` to `plan_finalize` and `&self.repo_root` to `try_execute_finalize_plan` → `promote`; `validate` reads `&self.jigc_home`; file-state is one shared record under `jigc_root`.
- A sub-task's boundary is `milestone finalize`, whose whole subject is `jigc_home` **[read]** `milestone.rs::run_create`; **[pinned]** `cwd_verb_subject::a_milestone_created_in_a_linked_worktree_finalizes`.

So the guard fires when the task is **ordinary** (`engine::milestone::owning_milestone` is `None`) and `render::CommitSite::differing(jigc_home, standing)` is `Some`. Both pieces exist; `TaskArea::commit_site()` is already "where will this commit land".

| Layout | Answer |
|---|---|
| (a) main checkout | `differing` is `None`; never fires |
| (b) fan-out worktree, its own sub-task | `owning_milestone` is `Some`; never fires. Sub-agent doc writes keep working |
| (b') ordinary task minted inside a fan-out worktree | fires, correctly: `repo.rs::posture_subject` would exempt it, but its finalize commits there once the user follows the `git switch -c` route |
| (c) squash-combine dedicated worktree | no `doc` verb or `task finalize` runs there; its commits take `SeamSubject::dedicated`. Unreachable |
| (d) bare repository | `repo.rs::git_common_dir_parent` yields a non-checkout with no `.jigc/config`; every door answers not-set-up first **[read]**, **[owed-drive]** one cell |

`doc.rs::ActiveTask` keeps only `jigc_home` today; it needs the standing root (`discover_repo_root(cwd)`).

## 2. Doors and docs

**Which docs:** `engine::finalize::promote_destination(schema, slug)` is `Some`. This is the promote plan's own predicate and the one the amend sibling keys on. `commit` is the only shipped doctype it answers `None` for **[read]** both packs' `schemas/commit.yaml` declare neither `location:` nor `placement:`.

**In the class (refuse):**

- The eight `doc` write leaves (`VERB_KINDS`, doc × Write; nine arms, as `set-field` has `--unset`). All pass `ActiveTask::resolve`. Seven take `staged_path` then `read_or_copy_in`; `create` and `author` take `create_admission` then `title_pre_check` then `create_gated`.
  - Rank: after argument shape, `machine_maintained_guard` and create admission; before any read of the committed store.
  - Fires whether or not the doc is already staged.
  - Today **[brief]** C1–C5, R3-F7 §3 last row.
- `task finalize`, `finalize --dry-run`, `task validate`, and orientation's task rows: the backstop at the destroying site. Needed for docs staged from the main checkout and finalized from the worktree, for tasks staged by rc.24 before upgrade, and for hand-placed files in the area.
- `jigc migrate`: **a new finding, [read] only.** `migrate.rs::migrate_in_repo` uses `ctx.repo_root` and `start.rs::mint_migration_in_repo` mints under `<standing>/.jigc/tasks/`, while every other task door reads `<jigc_home>/.jigc/tasks/`. From `<wt>` the task is invisible to `doc author`, `task list` and `finalize`. **[owed-drive]** from `<wt>`: `jigc migrate <path> --as adr`, then `jigc task list`. Its product is always a persisted doc, so it should refuse before the mint.

**Not this class** — single-homed at `jigc_home` **[read]** `require_project_layer` in each: `jigc rename`, `relocate`, `unmanage`, `ingest`, `migrate-corpus`, `config set`, `milestone create/add-task/add-from-spec/discard`, `setup`/`uninstall`.

- They read and write the main checkout, so no bytes cross.
- Their open question is law 1: does the ack say it acted in the main checkout? `setup`/`uninstall` do (`render::InstallSite`). For `rename`, `migrate-corpus` and the milestone record commits, **[owed-drive]** — a separate lead.
- `task bind`, `task discard`, `task amend`: no promoting bytes.

## 3. Earliest door

- **Mint today** from `<wt>`: silent about the checkout **[read]** `start.rs`, only `task amend`'s mint passes `AmendTarget::checkout`. The mint provisions only the transient commit doc **[read]** `start.rs::provisions_at_compose`, so it is not a doc-write door.
- **Orientation today**: no checkout statement; rows carry only the standing checkout's posture **[read]** `orient.rs::active_tasks`.

Recommendation: **state at mint, refuse at the first persisted write.**

- Law 3 has a seam-generated tier (`surface-contract.md` → The stated-at fence). A text-only line from the code-owned predicate, emitted only when it holds, satisfies it and moves no golden. `render::commit_site_line` is the precedent.
- Mint and resume, roughly: "checkout: the linked worktree at `<wt>` on branch `<b>` — this task commits here, code only; managed docs live in the main checkout at `<main>` and a doc write from here is refused".
- Orientation gets the same sentence; an already-staged promoting doc shows as the backstop finding in `tasks[].findings` (existing array, via `task.rs::sweep_for_orientation`).
- JSON gets no key: a declared bound, as for `CommitSite`.

Refusing at mint is mechanically justified only where a mint cannot land: the doc-only commit model (`task.rs::commits_doc_only`; four methodology workflows) and `jigc migrate`. `single-task` must never be refused. This is fork F1.

## 4. The route

**Write-door refusal** (task T minted in `<wt>`, base pin is the linked HEAD **[read]** `start.rs::mint_in_repo`):

- **Print:** `cd <abs main>`, then `jigc start "<intent>"` for the doc. T stays usable here for code, or is abandoned with `jigc task discard <T>`.
  - No `--force` in our route: discard's own guard names what is staged (`task.rs::refuse_over_staged_prose`).
  - Shape precedent: `start.rs::blanket_base_pin_refusal` (`cd {shell_operand}` plus a mechanical span).
- (a) Docs-only: a new task from main is an ordinary run. Exit 0 expected, **[owed-drive]**.
- (b) Mixed: T finalizes code-only in `<wt>` **[pinned]** `cwd_verb_subject::task_finalize_names_the_checkout_it_commits_in…` (quick-fix), **[brief]** single-task. The doc lands from main as a second commit.
- A later `git merge` is within the rule: the branch never touched a managed doc, so git reconciles nothing (`storage.md`, third combine bullet).
- **Do not print** "re-run this with `--task T` from main". It depends on the base pin: main refuses `finalize.base-mismatch` when the branches' docs differ **[brief]**.

**Dead-end cell [read], [owed-drive]:** staged code in `<wt>` dangles a committed anchor.

- The finalize floor blocks `doc-code.symbol-exists` (`engine/validate.rs`, blast radius). The repair is a doc write, which is now refused there.
- Honest exits: land the code from the main checkout on that branch, or merge first and repair from main. The route needs a sentence for this, or (B) must own it.

**Backstop refusal** (docs already staged): `cd <abs main>`, then `jigc task finalize <T>`.

- It lands docs only and leaves code staged in `<wt>` **[brief]**.
- It does not work when T's pin is the linked HEAD and the branches' docs overlap. So choose the route by asking `engine::finalize::decide_base_repin` over `jigc_home`'s facts.
- On overlap, print: read back with `jigc doc show <addr> --task <T>`, re-author in a fresh task from main, then discard.
- The fast-forward path that deletes branch wording **[brief]** is closed by the (R3, F7) copy-in baseline, not here. Test the two together.

## 5. Read side

Coherent under the rule: the store is the main checkout's working tree. One stderr note on `stale_read_hint`'s mold, only when `differing`, for `doc show`, `doc list` and `validate`: "served from the main checkout at `<main>` — jigc's doc store has one home, not the linked worktree you are standing in". Stdout stays byte-identical. `(R1, F5)`, the hook asking about main from a worktree, is the same fact on a recorded row.

## 6. Finding code

New: **`finalize.linked-worktree-doc`**.

- **Why not reuse:** the only code with this meaning is `finalize.amend-staged-doc`, whose message, routes and stated-at tokens are amend-specific.
- **Shape:** blocking, no knob. Target is the file path (promote destination), one finding per doc. Human route, since its first act is a `cd`.
- **Raised at two positions** on the sibling's mold: `task.rs::amend_staged_doc_finding` with `AmendDocDoor`.

Registration homes:

1. `render.rs::FINALIZE_FAMILY`, sorted; `tests/finalize_family_registry.rs` scans producers and reddens otherwise.
2. `design/command-output-contract.md`, the `finalize.*` sub-table (same fence).
3. `pack.rs::AMBUSH_CONTRACTS`, an `Exempt` row. Reason: the contract binds only in a checkout state no step can know, and is stated by the seam. Fenced by `tests/stated_at_fence.rs`.
4. `design/surface-contract.md`, the exempt-row prose.
5. `design/validation.md`, a registration section.
6. `design/finalize.md` and `design/storage.md`.

Not touched: `GATE_COVERAGE` (it rides `content-findings`, as the amend gate does in `preview_gates`), `ENVELOPE_ARMS`, `ENVELOPE_OWED_CODES`, `ERROR_CODE_REGISTRY`, `CONSTRAINT_REQUIRED_TOKENS`, `PostureMember`.

## 7. What must not move

- Schemas, manifests, snapshots, `schema_version`, contract versions, every envelope key set.
- `VERB_KINDS`, `BEHALF_DOORS`, `COMMITTING_DOORS`.
- Compose goldens stay put only if all four hold:
  - no step or workflow text changes;
  - no `GATE_COVERAGE` row (it generates `what's-left:`);
  - `adapter.rs::BOOTSTRAP_PATHS_AND_CWD` is untouched (six `agent-md--*` composite goldens carry it);
  - the new lines print only when `differing` is `Some`.
- The two existing linked-worktree tests (`cwd_verb_subject` C2-09, `task_amend`) stay green unedited. They are the code-only control.

## 8. Tests, red first

`$REPO/crates/cli/tests/linked_worktree_doc_home.rs`, registered in `groups/g_finalize.rs` beside `task_amend`. Use a real-setup corpus built fresh per arm: `mint_project_layer` leaves `.jigc/config` untracked, so the worktree has no checked-out `.jigc/`, unlike an adopter's.

- `every_doc_write_leaf_refuses_a_promoting_doc_from_a_linked_worktree`
  - Iterates `doc::doc_write_verbs()` (plus `--unset`) × {location doctype, placement singleton, transient `commit`} × {user worktree, main control, fan-out worktree with its sub-task}.
  - Panics on a leaf with no cell.
  - Asserts code, key, exit, and nothing staged.
- `an_ordinary_task_inside_a_fan_out_worktree_is_refused_too` (b').
- `a_code_only_task_lands_from_a_linked_worktree`, for `single-task` and `quick-fix`: zero promotions, no managed path touched.
- `the_backstop_refuses_docs_staged_from_the_main_checkout` at `task validate`, `--dry-run`, `finalize`, and orientation: same key at all four.
- `the_write_door_route_reaches_exit_0` and `the_backstop_route_reaches_exit_0`, plus the overlap arm's refusal and alternative route.
- The four advocate cells C1–C4 as regression arms. C5 is unreachable once the first lands.
- `migrate_from_a_linked_worktree_refuses_before_the_mint`.
- `the_mint_and_orientation_state_the_checkout`, with main-checkout bytes unchanged.
- Bare-layout and dangling-anchor cells, pinned as driven.

## 9. Docs, in the same commits

- `design/storage.md` → CLI and git: the rule is now enforced, by which predicate; why a one-sided merge is inside it; the dangling-anchor bound.
- `design/finalize.md` → 1. Preflight: beside the `SeamSubject::live` sentence, the standing checkout commits code only.
- `design/command-output-contract.md`, `surface-contract.md`, `validation.md`, per §6.
- `QUICKSTART.md` and `MIGRATING.md`: one paragraph each. This changes the installed skill's bytes, so a `jigc setup` re-run applies.
- `task.rs::finalize_long_about` and the `doc` write helps: one sentence. Run `help_truth`.
- `.jigc/AGENT.md`: leave it (fork F3).

## 10. Commits

1. `test`: the red suite and group registration.
2. `fix(doc)`: the predicate home, the write-door guard, the code with all §6 registrations, the `storage.md` sentence.
3. `fix(task)`: the backstop at the committing door and in `preview_gates`, route arms, `finalize.md`.
4. `fix(migrate)`: refuse before the mint.
5. `fix(start)`: mint, resume and orientation statement; the read-side note; guides and help.
6. `docs`: the `DECISIONS.md` entry, the `decisions-pending.md` row, and the `ideas/` parking file for (B).

## 11. (B), for the parking file

**Direction:** the task door reads and writes the checkout it commits in. A linked worktree becomes a real home for doc work on its branch.

**Reverses:**

- `storage.md`: "only code ever rides a worktree" and "one canonical home (the main checkout)".
- The read half of M31 Inc 2 / WF3 (`repo.rs` module header).
- It extends C2-09 from "commits where you stand" to "reads where you commit".

**Open questions:**

- File-state and the edge index are one cache per repository, keyed by path: per-checkout or branch-keyed?
- The project config layer is read from main while the branch carries its own copy.
- Which checkout does the pre-commit hook's nested `validate` ask about (`(R1, F5)`)?
- How do two branches' docs meet at `git merge` without text-merging anchors?
- Are the task roster and the base-pin rule per checkout?
- Do fan-out worktrees keep the directory join?
- Submodule and `--separate-git-dir` layouts also keep `.git` as a file (already parked in `ideas/monorepo-submodule-support.md`).

## Forks for the human

- **F1, mint.** State only, or also refuse where the mint cannot land (doc-only commit model, `jigc migrate`)? Recommend: state everywhere, refuse those two.
- **F2, `jigc migrate` in this pass?** Recommend yes: same class, and it is a dead end today. It is read-only evidence until driven.
- **F3, AGENT.md.** A sentence moves six compose goldens. Recommend no.
- **F4, code name and `Exempt` row.** `finalize.linked-worktree-doc` or `finalize.worktree-staged-doc`; an `Exempt` row or none (as `finalize.milestone-sub-task` has none). Recommend the first of each.
- **F5, the dangling-anchor cell.** Accept as a stated bound of (A), or let the code floor's route name the main-checkout exit?

## Critical files

- `$REPO/crates/cli/src/doc.rs`
- `$REPO/crates/cli/src/task.rs`
- `$REPO/crates/cli/src/render.rs`
- `$REPO/crates/cli/src/start.rs`
- `$REPO/crates/cli/src/migrate.rs`