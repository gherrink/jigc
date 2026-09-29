# M52 baseline — area `destroying` (the destroying-door subject over every byte shape in the workbench)

**Provenance.** Binary `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`, sha256
`126f1584f183636bb6cd9e782b1dc26aca28dd1afaf2fb83da1fd0e5febc8fa9` (asserted, verbatim, before the first
drive). Repo HEAD `7637a46f1908af7cffd83cc2e5e96dc2365744a0`, tree clean. Date 2026-09-16/17.
**No cargo was run.** Every fixture from `dev/jigc-rig` (`fresh`, `committed-singletons`, `refs-post-hoc`),
two-step eval, `cd "$REPO"` before every drive; nothing was written into the working repository.
Rig scripts live at `<scratch>/dest/{mkrig.sh,mkfanout.sh,mkleftover.sh,plant.sh}`.
Assigned rows: C-1 class · D-1 · D-2 · D-3 · D-4 (`completions/artifacts/M51/per-axis-review/axis-3.md`).
**Verified at this sha — a map, not gospel.**

---

## §1 · The class enumeration

### 1.1 The greps, their hit counts, and what they miss

| pattern | scope | raw hits | production hits |
|---|---|---|---|
| `remove_dir_all` | `crates/cli/src` + `crates/engine/src` | **97** | 12 (the rest are `#[cfg(test)]` `Drop` guards of the shape `let _ = std::fs::remove_dir_all(&self.0)`) |
| `remove_file` | same | **38** | 12 |
| `remove_dir\b` (non-`all`) | same | **3** | 2 (`task.rs:4900`, `setup.rs:2896` — both empty-dir-only prunes) |
| `"worktree"` (argv literal) | same | **14** | 5 (`add` ×3, `remove --force` ×2) |
| `"prune"` | same | **4** | 4 |
| `git clean` | same | **0** | 0 |
| `"reset"` / `"--hard"` / `"checkout"` + `-f` | same | 1 test-only (`start.rs:4044`) | **0** |

**What the pattern would miss, measured rather than assumed:** a removal performed by a *subprocess* with
non-`remove` argv. Driven the grep for the three candidates (`git clean`, `git reset --hard`,
`git checkout -f`) — **zero production occurrences**, so the subprocess arm of this class is exactly
`git worktree remove --force` (2 sites) and `git worktree prune` (4 sites). It would also miss a removal
inside a vendored crate; not audited (bound §5).

### 1.2 Production sites that remove a tree or a file **under `.jigc/`**, mapped to their door(s)

| # | site | what it removes | door(s) that reach it |
|---|---|---|---|
| 1 | `cli/task.rs:553` `remove_dir_all(&task.dir)` | `.jigc/tasks/<id>/` **whole tree** | `jigc task discard` |
| 2 | `cli/task.rs:4948` `remove_dir_all(cleanup_dir)` (`post_commit`) | `.jigc/tasks/<id>/` **whole tree** | `jigc task finalize` |
| 3 | `cli/milestone.rs:4537` `cleanup_subtask_areas` | each `.jigc/tasks/<sub-id>/` **whole tree** | `milestone finalize` (both landed arms) · `milestone discard` |
| 4 | `cli/milestone.rs:3625` `remove_milestone_area` | `.jigc/milestones/<id>/` | `milestone discard` |
| 5 | `cli/milestone.rs:854` `unwind_mint` | the area *this call* minted | `milestone create` / `milestone add-task` rollback |
| 6 | `cli/milestone.rs:2383/2387` `remove_leftover` | `.jigc/worktrees/<id>` (dir **or** leaf) | `milestone provision --force` |
| 7 | `cli/milestone.rs:4603` `git worktree remove --force` | `.jigc/worktrees/<id>` | `milestone finalize` · `milestone discard` |
| 8 | `cli/milestone.rs:2274, 4617` `git worktree prune` | git admin records | `provision` · `finalize` · `discard` |
| 9 | `cli/task.rs:5774/5796/5799` `DedicatedWorktree` | `.jigc/worktrees/.combine-<pid>-<nanos>` | `milestone finalize` (combine overlay) |
| 10 | `cli/setup.rs:2745` `remove_dir_all(<repo>/.jigc)` | the **whole workbench** | `jigc uninstall` |
| 11 | `engine/milestone.rs:1879` `remove_dir_all(<merged>/docs)` | `.jigc/milestones/<id>/merged/docs/` | `milestone finalize` (materialize) |
| 12 | `engine/index.rs:179` `remove_file` | `.jigc/index/edges.json` | `post_commit` invalidate |
| 13 | `engine/state.rs:1180` `CreatedDoc::rollback` | one `.jigc/tasks/<id>/docs/<addr>.md` | `doc author` / `doc create` rollback |
| 14 | `cli/task.rs:4226` swap rollback | config-layer pre-image paths under `.jigc/config/` | finalize rollback |
| 15 | `cli/task.rs:3352`, `cli/task.rs:5674` | `finalize-message.tmp` / the sub-task message temp | task/milestone finalize |
| 16 | `cli/setup.rs:1986` | the install footprint file | `setup` / `uninstall` |

Removers **outside** `.jigc/` (named so the boundary of this area is explicit, not driven here):
`migrate_corpus.rs:953`, `task.rs:3704` (the `ValidatedRetirement` sink), `task.rs:4893/4900` (promote
rollback), `adapter.rs:811`, `rename.rs:793/827/836`, `setup.rs:786/2835/2896`.

### 1.3 The registries and the two subject derivations

| registry / probe | file:symbol | count | subject it walks |
|---|---|---|---|
| `DESTROYING_DOORS` | `cli/milestone.rs:2625` | **4** (`[&DestroyingDoor; 4]`) | `provision` · `discard` · `uninstall` · `finalize` — **`task discard` and `task finalize` are not members** |
| `LEFTOVER_VERDICTS` | `cli/milestone.rs:2510` | 3 | `Unverifiable` · `OwnWorktree` · `NoOwnLinkage` |
| `LeftoverShape` | `cli/milestone.rs` | 3 | `Directory` · `File` · `Unreadable(String)` |
| `probe_leftover` / `leftover_at` | `cli/milestone.rs` | — | **on-disk walk** of one worktree path; `symlink_metadata` |
| `held_subtask_worktrees` | `cli/milestone.rs:3531` | — | **on-disk walk** (`list.enumerate()` × `probe_leftover`); `registered` is a *label*, not the subject |
| `remove_worktrees` | `cli/milestone.rs:4581` | — | **registered set** — `registered_worktrees(repo_root)`; an unregistered path is skipped |
| `cleanup_subtask_areas` | `cli/milestone.rs:4520` | — | **registered set** (the milestone's `TaskList`), removes the whole area |
| `fanout_worktree_paths` | `cli/setup.rs:3004` | — | **on-disk walk** of `.jigc/worktrees/` children, every shape (M49's fix) |
| `workbench_paths` | `cli/setup.rs:3053` | — | **on-disk walk** of `.jigc/`, minus the seven `gitignore::ENTRIES` names |
| `staged_task_prose` → `staged_doc_ids` | `cli/task.rs:671 / 614` | — | **`docs/*.md` only**, per task area |
| `gitignore::ENTRIES` | `cli/gitignore.rs:38` | 7 | `tasks/ index/ state/ milestones/ worktrees/ logs/ displaced/` |

**The two derivations, stated as the plan needs them:** the *worktree* half is derived **on disk** and
answered through git (`git status --porcelain --ignored`); the *task-area* half is derived **on disk** but
its predicate is a **filename-suffix claim** (`*.md` under `docs/`). `remove_worktrees` and
`cleanup_subtask_areas` — the two **sinks** — take the **registered set**, which is why a sink can be
narrower than the guard that cleared it.

### 1.4 The shape space planted (`<scratch>/dest/plant.sh`)

S-a non-`.md` at area root · S-b non-`.md` under `docs/` · S-c a **`.md` outside `docs/`** · S-d nested
dir with a file · S-e empty dir · S-f dotfile · S-g symlink → file outside the repo · S-h symlink → dir
outside the repo · S-i symlink into the repo · S-j file with mode `000`. Worktree-only additions:
untracked · staged · unstaged · gitignored (via `.git/info/exclude`, so HEAD never moves).

---

## §2 · The drives

### 2.1 `jigc task discard` — the whole shape space

```
setup:  . mkrig.sh ; . plant.sh .jigc/tasks/tidy-the-readme "$OUT"
argv:   jigc task discard tidy-the-readme
exit:   1
obs:    blocking · task-discard.staged-prose — task `tidy-the-readme` stages 1 doc(s) … : commit:tidy-the-readme
        (the guard names ONE doc; S-c `NOTES.md` at the area root is NOT among them)

argv:   find .jigc/tasks/tidy-the-readme/docs -name '*.md' -delete ; jigc task discard tidy-the-readme
exit:   0
obs:    stdout: "discarded task tidy-the-readme"   stderr: EMPTY
after:  .jigc/tasks/tidy-the-readme -> gone. S-a … S-j all destroyed, named by nothing.
        Symlink TARGETS survive: $OUT/outside.txt, $OUT/outsidedir/inner.txt, README.md all intact.
```
**Classification: latent defect (confirms C-1, wider).** All ten shapes destroyed at exit 0. The class is
**not "non-`.md`"** — S-c, an ordinary `.md` file one directory above `docs/`, is destroyed identically.
The predicate is `docs/*.md`, so the honest statement of the class is *everything in the task area that is
not a `docs/`-level `.md`*.
**Positive result:** `remove_dir_all` does not traverse symlinks — S-g/S-h/S-i delete the link only. No
out-of-repo destruction on any door driven.

```
argv:   jigc task discard tidy-the-readme --force      (same plant set)
exit:   0
obs:    "discarded task tidy-the-readme — dropped staged edits to: commit:tidy-the-readme (transient)"
```
**Even the consented path never names a plant** — the `dropped` ack is built from `dropped_staged_docs`,
the same `docs/*.md` set.

### 2.2 `jigc task finalize` — the door in no destroying registry

```
setup:  . mkrig.sh ; . plant.sh .jigc/tasks/tidy-the-readme "$OUT"
        printf 'a change\n' >> README.md ; git add README.md
        jigc doc set-field commit:tidy-the-readme#header/type --task tidy-the-readme --value docs
        printf 'tidy the readme\n' | jigc doc set-slot commit:tidy-the-readme#summary --task tidy-the-readme --from-file -
argv:   jigc task finalize tidy-the-readme
exit:   0
obs:    "finalized 3cddebe — docs: tidy the readme / modified README.md / 1 file committed"  stderr EMPTY
after:  .jigc/tasks/tidy-the-readme gone; all ten shapes gone.
        commit contains README.md ONLY (git show --stat).
        `git rev-list --all --objects | git cat-file --batch | grep` for PRECIOUS / ATTACHED / DEEP /
        DOTFILE / "loose markdown" → **not in any object** (5/5).
```
**Classification: latent defect (C-1's fifth door, confirmed).** `task finalize` reaches removal site #2
(`post_commit`'s `remove_dir_all(cleanup_dir)`), carries **no** guard, **no** narration, and is in neither
`DESTROYING_DOORS` nor the leftover classifier. It is the **ordinary success path**, not an abandon path —
which makes it the highest-traffic member of the class.

### 2.3 `jigc milestone discard`

```
setup:  . mkfanout.sh (milestone + 2 provisioned, authored sub-tasks) ; plant into .jigc/tasks/first-sub
argv:   jigc milestone discard axis-three-probe                       (no --force)
exit:   1 · milestone.dirty-worktree   → names every WORKTREE path and its git entries
argv:   jigc milestone discard axis-three-probe --force
exit:   0
obs(stdout): "discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)"
obs(stderr): warning: … discards the staged docs of 2 open task(s) …  first-sub: commit:first-sub …
             warning: removing the fan-out worktree .jigc/worktrees/second-sub discards work that is not
                      in git:  .hidden / NOTES.md / docs/attachment.txt / link-dir / link-in / link-out /
                      nested/deep/file.txt / noread.txt / notes.txt
after:  the TASK-AREA plants under .jigc/tasks/first-sub — all ten — gone, named by nothing.
```
**Classification: latent defect (task-area arm) + built + proven (worktree arm).** The identical ten
shapes are **fully narrated** when they sit in a worktree and **wholly silent** when they sit in the task
area, in the same run, on the same screen. That contrast is the sharpest statement of the class available.

### 2.4 `jigc milestone finalize`

```
setup:  . mkfanout.sh ; plant into .jigc/tasks/first-sub
argv:   jigc milestone finalize axis-three-probe
exit:   0
obs:    finalized d88efe4 — … 3 files committed (record + first-sub.txt + second-sub.txt)   stderr EMPTY
after:  every plant gone; 5/5 marker strings not in any git object.
```
**Classification: latent defect (C-1's fourth door).** A `DESTROYING_DOORS` member whose declared
`code: None` ("the declared narrate-only member") narrates the worktree half and is silent on the task half.

### 2.5 `jigc uninstall`

```
setup:  . mkfanout.sh ; plant into .jigc/tasks/first-sub ; clear both worktrees + all staged *.md
argv:   jigc uninstall                                              (no --force)
exit:   0
obs(stdout): "jigc uninstall — repo-local install removed …"
obs(stderr): warning: removing `.jigc/` also removes 5 tracked file(s) under it: … (only tracked files)
after:  every plant under .jigc/tasks/first-sub gone, named by nothing.
```
**Classification: latent defect (C-1's third door).** All three `uninstall` guards miss it by construction:
`dirty_fanout_worktrees` walks `.jigc/worktrees/`, `staged_task_prose` walks `docs/*.md`, and
`workbench_paths` **excludes the `tasks/` prefix outright** — its own doc-comment says `tasks/` and
`worktrees/` are *"answered by the doors that own them."* Driven, those doors answer for `docs/*.md`
and nothing else.

### 2.6 `jigc milestone provision` — the leftover shape space (the worktree-path half)

```
setup:  . mkleftover.sh (7 sub-tasks, 7 shapes at .jigc/worktrees/sub-*)
argv:   jigc milestone provision leftover-probe              (no --force)
exit:   1 · milestone.leftover-holds-work — 6 path(s) named:
        sub-five (symlink→dir)  "the file itself — it is a file, not a worktree"
        sub-four (symlink→file) same
        sub-seven (dir)         ".hidden, N.md, nested"
        sub-six  (chmod 000)    "unknown — could not read the leftover directory … Permission denied"
        sub-three (plain file)  "the file itself"
        sub-two  (dir + a.txt)  "a.txt"
        sub-one  (EMPTY dir)    — correctly absent from the listing (no bytes)
argv:   jigc milestone provision leftover-probe --force
exit:   1 · milestone.provision-failed at sub-six, "4 of 7 worktree(s) landed"
obs:    three `warning: removing the leftover …` narrations, each naming what it took;
        outside symlink targets survive ($OUT/outside.txt, $OUT/outsidedir/inner.txt).
```
**Classification: built + proven** for the shapes reachable through the classifier. Every byte-bearing
shape refuses; an empty directory is correctly silent; a symlink is named as `File` and only the link dies.

### 2.7 D-1 — the route that names a verb the state refuses

Producers of a route naming `jigc task finalize <id>`, by grep (`grep -rn "jigc task finalize" crates/*/src`
→ 87 raw hits; the *route-text* producers with an interpolated or placeholder id are **6**):
`cli/task.rs:799` · `cli/setup.rs::staged_prose_finding` · `engine/finalize.rs:1466` (task arm of
`empty_commit_finding`, `Unit::Task` only) · `cli/render.rs:131/134` (already discriminates sub-task) ·
`cli/render.rs:4604` (`migrate --approve`) · `cli/task.rs:2480` (survivable-frame re-run argv).
**Exactly 2 can be served over a milestone sub-task**, and both were driven:

```
setup:  . mkfanout.sh
argv:   jigc task discard first-sub
exit:   1 · task-discard.staged-prose
route:  "… or land them with `jigc task finalize first-sub` (which refuses while a required slot is empty) …"
argv:   jigc task finalize first-sub                 ← the route, run verbatim
exit:   3 · finalize.milestone-sub-task — "the parent milestone's finalize is the only commit boundary"
        (identical from inside .jigc/worktrees/first-sub — driven, exit 3)

argv:   jigc uninstall   (worktrees cleared so the staged-prose arm is reached)
exit:   1 · uninstall.staged-prose — message lists `first-sub: commit:first-sub`, `second-sub: …`
route:  "… land it with `jigc task finalize <task-id>` (which refuses while a required slot is empty) …"
argv:   jigc task finalize first-sub                 ← exit 3 · finalize.milestone-sub-task

control (the third family member, driven, CORRECT):
argv:   jigc milestone discard axis-three-probe
exit:   1 · milestone.staged-prose
route:  "… land the milestone with `jigc milestone finalize axis-three-probe` …"   ✓ names the right door
```
**Classification: latent defect, class size 2** (the review's row says "two destroying doors" — confirmed
exactly, neither wider nor narrower). Measured constraint for the Settle: the fix needs the sub-task
discriminator **at the finding's construction site**, because both producers compose the route from an id
they already hold and neither consults the milestone record; `milestone.staged-prose` proves the
discriminator exists and is reachable from the same layer.

**A second leg of the same two routes, driven, that the row does not mention:** the *other* exit both
offer — `jigc task discard <task-id> --force` — **does** work over a sub-task (exit 0) and, as a
side-effect the route never states, **settles that sub-task in the committed milestone record**:
```
argv:  jigc task discard first-sub --force
exit:  0
obs:   "discarded task first-sub — dropped staged edits to: commit:first-sub (transient)"
       "record commit: f255208   — this sub-task's milestone record, settled to `discarded` and committed on its own"
```
Not a lie (the ack names it), but the route offers it as the cheap exit from a `uninstall` refusal.

### 2.8 D-2 — the ack that claims a removal that did not happen

```
setup:  . mkfanout.sh ; chmod a-w .jigc/worktrees
argv:   jigc milestone discard axis-three-probe --force
exit:   0
obs(stdout): "discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)"
obs(stderr): warning: could not remove the fan-out worktree …/first-sub: `git worktree remove --force …`
                      failed: error: failed to delete '…/first-sub': Permission denied
             remedy: run `git worktree prune`, then `git worktree remove --force …`        (×2)
truth:  .jigc/worktrees/{first-sub,second-sub} STILL ON DISK.
```
**Classification: latent defect, class size 1 — narrower than a reader would assume.** The ack literal is
`cli/milestone.rs:3298`. The sibling doors were driven on the identical failure and are **clean**:
```
setup:  . mkfanout.sh ; mkdir .jigc/worktrees/first-sub/keep ; chflags uchg …/keep
argv:   jigc milestone finalize axis-three-probe
exit:   0 — ack names only what committed; the warning names the leaked worktree. NO false removal claim.
```
So D-2 is one fixed string at one door, not a family. The **loss narration** beside it is outcome-keyed and
correct at every door driven (M50's fix holds).

### 2.9 D-3 — the fail-closed staged-prose refusal prints the host path

```
setup:  . mkrig.sh ; chmod 000 .jigc/tasks/tidy-the-readme/docs
argv:   jigc task discard tidy-the-readme
exit:   1 · task-discard.staged-prose — "cannot check task `tidy-the-readme`'s working area … :
        /private/var/folders/nj/…/repo/.jigc/tasks/tidy-the-readme/docs: Permission denied (os error 13)"
argv:   jigc uninstall
exit:   1 · uninstall.staged-prose — same absolute host path in the message
setup:  chmod 000 .jigc/tasks/first-sub/docs
argv:   jigc milestone discard axis-three-probe
exit:   1 · milestone.staged-prose — "cannot check milestone:axis-three-probe's sub-task working areas
        … : /private/var/folders/nj/…/repo/.jigc/tasks/first-sub/docs: Permission denied (os error 13)"
```
**Classification: latent defect — the class is THREE doors, not the two the row states.** One producer
(`cli/task.rs:697`, `path.join("docs").display()` inside `staged_task_prose`) feeds three per-door
fail-closed findings. The row's second half is also confirmed: `UNSWEPT_PRODUCERS`
(`crates/cli/tests/repo_relative_paths.rs:746`) carries
`("crates/cli/src/task.rs", 3, "two `with_context` promote/probe faults and one `git archive --prefix=`")`
— the three actual sites are `task.rs:697` (a **blocking finding surface**), `task.rs:1861`
(`--prefix=`) and `task.rs:3498` (`promotion.source`, one fault, not two). **The count is right and the
reason is false for 1 of 3.**

### 2.10 D-4 — the worktrees-root fail-closed route

```
setup:  . mkrig.sh ; mkdir -p .jigc/worktrees ; chmod 000 .jigc/worktrees
argv:   jigc uninstall
exit:   1 · uninstall.dirty-worktree — "cannot check `.jigc/worktrees/` … : Permission denied (os error 13)"
route:  "make sure `git` is on PATH and the repository is readable, then re-run `jigc uninstall` — or …
         remove them yourself (`git worktree list`, then `git worktree remove`) and re-run"
        → blames `git` on PATH for a `read_dir` EACCES, and never names `--force`,
          while the SAME door's two sibling fail-closed refusals both name it (2.9, driven).
```
**Classification: latent defect, confirmed; producer `cli/setup.rs:3569` `unverified_worktrees_finding`.**
Driving the same state at the sibling doors is where the row's class **widens into §4 defect L-1** below:
`uninstall` fails closed here, `milestone discard` does **not**.

---

## §3 · What changed against the review's rows

1. **C-1's predicate is misnamed.** The row says *"a non-`.md` file under a task's working area."* Driven,
   the guard's subject is `docs/*.md`, so **S-c — an ordinary `.md` file at the task-area root — is
   destroyed identically at all five doors.** A fix cut on "non-`.md`" would ship the reported repro and
   leave a sibling cell open. The honest class: *everything in `.jigc/tasks/<id>/` that is not a
   `docs/`-level `.md`*.
2. **C-1's door count is 5 and holds** — `task discard` · `task finalize` · `milestone discard` ·
   `milestone finalize` · `uninstall`, each driven independently. `milestone provision` is **not** a member
   (it never touches a task area) and `milestone create`/`add-task`'s `unwind_mint` is not either (it
   removes only an area the same call minted; not driven, see §5).
3. **D-3's class is 3 doors, not 2** — `milestone.staged-prose` prints the same host path from the same
   producer (driven, 2.9).
4. **D-2's class is 1, and that is a finding in itself** — driven at `milestone finalize`, the sibling ack
   is honest. The plan should not widen this one; it should widen C-1 and narrow this.
5. **D-1's class is exactly 2 route producers** out of 6 that name `jigc task finalize` — measured by grep
   and then by driving each candidate's reachability. The third family member is already correct, which is
   what makes this an un-swept axis rather than a design choice.
6. **The subject-derivation asymmetry is the plan's real constraint.** Worktrees are derived on disk and
   adjudicated by `git status --porcelain --ignored` — every planted shape is refused (un-forced) and
   narrated (forced), **including gitignored bytes** (driven, 2.3/§2 ignored cell). Task areas are derived
   on disk but adjudicated by a **filename suffix**. The two halves of one screen answer opposite ways
   about identical bytes.
7. **The M46 `--ignored` argument DOES transfer to `.jigc/tasks/<id>/`, and harder.** Measured: the
   non-`.md` files jigc itself writes into a task area are
   `base.json` · `intent` · `workflow` · `staged-snapshot.json` · `docs/provenance.json` (every task, from
   `jigc start` onward — driven), `roles.json` (driven on `refs-post-hoc`), `source` + `source-path`
   (driven on a `jigc migrate … --as adr` task). A byte-level subject rule of the form *"any file in the
   task area that is not a staged `.md` blocks"* therefore fires on **100 % of tasks at 100 % of doors** —
   strictly worse than the worktree `--ignored` case M46 refused, where the false-fire rate was only the
   build-output-producing subset. **The forced shape of any fix is a complement of jigc's own writer
   set**, i.e. an allowlist of the eight names above (plus whatever `engine/src/validate.rs`'s three
   `*-snapshot.json` constants can land there — home not confirmed, §5), not a shape or suffix claim. That
   list is code-side derivable: `engine/src/state.rs` declares `BASE_PIN_FILE` `ROLES_FILE` `INTENT_FILE`
   `WORKFLOW_FILE` `SOURCE_PATH_FILE` `PROVENANCE_FILE` `STAGED_SNAPSHOT_FILE`, `cli/src/migrate.rs`
   declares `SOURCE_FILE`.
8. **`.md`-suffix blindness cuts the other way too** (2.11 below / §4 L-3): `staged_doc_ids` never asks
   `is_file()`.

---

## §4 · Latent defects — driven, in no §A row

### L-1 · `jigc milestone discard` fails **open** on an unreadable `.jigc/worktrees/`, settles the record irreversibly at exit 0
```
control:  . mkfanout.sh ; find .jigc/tasks -name '*.md' -delete        (worktrees root READABLE)
argv:     jigc milestone discard axis-three-probe
exit:     1 · milestone.dirty-worktree — names both live worktrees and their staged entries

defect:   . mkfanout.sh ; find .jigc/tasks -name '*.md' -delete ; chmod 000 .jigc/worktrees
argv:     jigc milestone discard axis-three-probe                       (NO --force)
exit:     0
obs:      stdout "discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)"
          stderr EMPTY — no finding, no warning, no consent asked
truth:    both worktrees still on disk holding uncommitted `first-sub.txt` / `second-sub.txt`;
          `git worktree list` → ONLY the main checkout (the admin records were pruned away);
          `jigc doc show milestone-record:axis-three-probe` → status: discarded, both sub-tasks discarded;
          re-entry refused: `milestone.terminal` ("a settled milestone is over and has no workbench").
```
**Root cause, pinned in the source:** `leftover_at` (`cli/milestone.rs`) maps **every**
`symlink_metadata` error to `LeftoverAt::Absent`, and `probe_leftover` returns `None` for `Absent` — i.e.
*"provably safe to delete."* An `EACCES` on the parent is not absence. `probe_leftover`'s own doc-comment
states the contrary rule it no longer keeps: *"`None` when `path` is **provably safe** to delete (absent,
empty, or a clean worktree of its own) … **including** the case where the probe could not read the path at
all, which is a hold like any other."* `held_subtask_worktrees`' doc-comment repeats it: *"Fail-closed: an
unreadable directory or `git status` is a `LeftoverShape::Unreadable` hold rather than an empty set."*
**A rule stated in the code and violated at HEAD.**
`jigc uninstall` survives the same state only because `fanout_worktree_paths`' own `read_dir` returns
`Err` **before** `probe_leftover` is reached — i.e. the fail-closed behaviour at the one door that has it
comes from a different function.
**Why this is the harm and not merely a missing warning:** the door commits the milestone record as
`discarded`, which `milestone.terminal` makes unreachable afterwards — an irreversible settle taken over
worktrees nothing could vouch for, which is exactly what the guard exists to prevent.
**Same root, second door, no loss:** `jigc milestone provision leftover-probe` over an unreadable root
also fails open (no `milestone.leftover-holds-work`) but its sink fails for the same permission reason, so
`PRECIOUS LEFTOVER` survived — driven, exit 1 with `milestone.provision-failed` carrying raw `git` stderr.

### L-2 · `jigc uninstall` destroys a plain file at any `ENTRIES` name at exit 0, with no `--force` and no narration
```
setup:  rig fresh (no tasks, no worktrees, nothing dirty)
        for n in tasks index logs milestones worktrees displaced; do
            printf 'PRECIOUS %s BYTES\n' "$n" > ".jigc/$n"; done       # six ENTRIES names as PLAIN FILES
argv:   jigc uninstall                                                 (NO --force)
exit:   0
obs:    stdout: "jigc uninstall — repo-local install removed …"
        stderr: only "removing `.jigc/` also removes 5 tracked file(s) under it: …"
after:  .jigc gone; all six files destroyed, named by nothing, in no index.
```
`workbench_paths` excludes an entry when `entry.file_name() == prefix` — a **name** match with **no shape
check** — while its own doc-comment records the opposite lesson three paragraphs down: *"**Every child that
is not a directory**, symlinks included (M49's lesson at `fanout_worktree_paths`): the shape of a path is a
reason to recurse into it, never a reason to drop it from the set."* This is M49's
`path.is_dir()` shape-vs-bytes error, re-appearing as a **name-vs-shape** error one function over, in the
guard M48 built to close it. The same file is invisible to `fanout_worktree_paths` (`!is_dir()` → empty
set) and to `staged_task_prose` (`!tasks_root.is_dir()` → empty set), so **all three** `uninstall` guards
and **all three** narrations miss it in one stroke.

### L-3 · `staged_doc_ids` has no `is_file()` check — a directory named `*.md` becomes a staged doc at four surfaces
```
setup:  rig fresh ; jigc start --workflow quick-fix "tidy the readme"
        find .jigc/tasks/tidy-the-readme/docs -name '*.md' -delete
        mkdir -p '.jigc/tasks/tidy-the-readme/docs/fake:thing.md'
argv:   jigc task discard tidy-the-readme
exit:   1 · task-discard.staged-prose — "stages 1 doc(s) … : fake:thing"
route:  "read what is in them with `jigc doc show <address> --task tidy-the-readme`"
argv:   jigc doc show fake:thing --task tidy-the-readme      ← the route, run
exit:   1 · store.unknown-type — unknown doctype `fake` … route: `jigc describe`     (dead end)
argv:   jigc doc list --task tidy-the-readme
exit:   0 · "fake:thing  fake:thing  managed"                ← the pinned index read reports it as managed
```
Reaches `task discard`, `uninstall` and `milestone discard` (the three guards), the `dropped` ack, and the
1.0-pinned `doc list --task` contract. Low severity (the door **over**-refuses, so no bytes die) but it is
a law-1 claim the bytes do not support, on a pinned surface.

### L-4 · `jigc milestone finalize` over an unwritable `.jigc/worktrees/` exits 1 with raw `git` stderr, no code, no route
```
setup:  . mkfanout.sh ; chmod a-w .jigc/worktrees
argv:   jigc milestone finalize axis-three-probe
exit:   1
obs:    "`git worktree add --detach …/.jigc/worktrees/.combine-61644-1789595881589488000 b3bde53` failed:
         Preparing worktree (detached HEAD b3bde53)
         fatal: could not create leading directories of '…/.combine-…/.git': Permission denied"
        — no finding code, no route, no `jigc ·` footer.
```
The combine overlay's `DedicatedWorktree::add` (`cli/task.rs:5779`) propagates a bare `anyhow`. A
committing door answering a recoverable environment fault with unframed git stderr.

---

## §5 · Honest bounds

- **Not driven: `unwind_mint` (site #5)** — reaching it needs a hook that rejects the record commit of
  `milestone create`/`add-task`. Its subject is by construction an area the same call minted, so a
  pre-existing plant cannot be inside it; I did not verify that claim under a racing writer.
- **Not driven: sites #11–#16** (`materialize`'s merged `docs/` rebuild, `index::invalidate`,
  `CreatedDoc::rollback`, the config-layer swap rollback, the message temps, the install footprint). Each
  removes a path jigc itself wrote in the same or a prior call; none takes a user-supplied path. Untested
  premise, stated rather than assumed.
- **`engine/src/validate.rs`'s `probe-snapshot.json` / `base-probe-snapshot.json` /
  `store-probe-snapshot.json`** are declared constants I did not manage to land in a task area during any
  driven lifecycle — their home is unconfirmed, so §3.7's allowlist may be short by up to three names.
- **The `--ignored` measurement at worktrees** was produced with `.git/info/exclude` (deliberately, so HEAD
  never moved); a committed `.gitignore` in the sub-task's base was not separately driven.
- **Concurrency** is untouched: every drive is single-process. L-1's fail-open was produced by a
  permission bit, not a race; whether a racing writer produces the same `Absent` collapse is unmeasured.
- **Empty directories** carry no bytes and are invisible to `git status`; I recorded that they are
  destroyed unnarrated at every door but did not treat it as a defect.
- **macOS only.** `chflags uchg` (D-2's finalize control) and mode-`000` semantics are platform behaviour;
  the same cells on Linux were not driven.
- **Vendored crates** were not searched for removal calls (§1.1).
- **The `uninstall` partial-teardown cell** (an undeletable path inside `.jigc/`, `--force`) was driven:
  exit 1 · `uninstall.remove-jigc`, the loss narration correctly named only what actually died, and steps
  2–6 of the teardown never ran, leaving a half-uninstalled repo. I did not classify it as a defect —
  the narration is honest and the route ("ensure `.jigc/` is writable, then re-run") completes — but the
  half-state is on the record here rather than unsaid.
