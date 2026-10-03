<!-- Independent ADVERSARIAL re-drive of the trial finding L-22 (doors: `milestone provision` / `milestone finalize` / `milestone discard` / `uninstall`), made AFTER this record was assembled; copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24` (and, for the container cells, the registry build in the trial image), 2026-10-04, by an agent that did not write the row it verifies. The `l22-work/` scripts the body names are the verifier's work files; they are not committed. -->

# L-22 (variant) — independent adversarial verification

**Subject:** a foreign linked worktree (one jigc did not create) whose recorded path does not exist where jigc runs — git's `prunable` state — has its admin record under `.git/worktrees/<name>/` removed, silently, at exit 0, by the milestone doors.

**Binary:** the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (asserted before every cell), release posture. Host git 2.54.0 (Apple Git-157); container cells: git 2.47.3 and the registry `jigc 1.0.0-rc.24` in the trial image `jigc-gate:registry-1.0.0-rc.24`.
**Tree read:** `bffa6667`; `git diff --stat 91834b5e HEAD -- crates` is empty, so the source read is the rc.24 release tree.

## Verdict in one paragraph

**It survives. I could not refute it, and the two things its first verifier left as inference or argue-down both came out against the argue-down when driven.** (1) The precondition is not only "a directory someone moved or deleted": a **live, in-place, never-touched** host worktree is orphaned when jigc runs in a container that sees the repository but not the worktree's path (driven, O-1), and a worktree moved with plain `mv` that is **fully working at its new path** (`git status` exit 0) is orphaned the same way (driven). (2) The loss is not only "recoverable until gc": with git's defaults, a detached commit older than two weeks is **permanently deleted by the next plain `git gc`** once jigc has pruned the record, while the same gc on the control (no jigc) keeps it (driven). **Tier 1 by the predicate; low likelihood; small, keyed fix available.**

## 1. Re-drive — three doors, each on its own fresh rig

Common setup (bash, from the jigc checkout; my scripts are `l22-work/drive.sh <cell>`):

```
jigc --version                                             # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
T=$(mktemp -d "$SCRATCH/t.XXXXXX") || exit; M=foreign-worktree-probe
jigc milestone create "Foreign worktree probe"; jigc milestone add-task $M "Add alpha file"; jigc milestone add-task $M "Add beta file"
jigc milestone provision $M                                 # (not in the provision-first cell)
for s in add-alpha-file add-beta-file; do echo code > "$REPO/.jigc/worktrees/$s/$s.txt"; git -C "$REPO/.jigc/worktrees/$s" add "$s.txt"; done   # (not in the discard cell)

# plants — CI="-c user.name=probe -c user.email=probe@example.com"
Y=$T/y; git -C "$REPO" worktree add -q --detach "$Y" HEAD            # y: DETACHED
echo c > $Y/foreign-committed.txt; git -C $Y add .; git -C $Y $CI commit -qm "foreign detached commit"; FC=$(git -C $Y rev-parse HEAD)   # (i)
echo "staged-only bytes" > $Y/foreign-staged.txt; git -C $Y add foreign-staged.txt; SB=$(git -C $Y rev-parse :foreign-staged.txt)        # (iii)
git -C $Y update-ref refs/worktree/wip <a commit-tree child of FC>   # a per-worktree ref file under .git/worktrees/y/refs/worktree/
echo "unstaged edit" >> $Y/README.md
Z=$T/z; git -C "$REPO" worktree add -q -b side "$Z" HEAD             # z: ON A BRANCH
echo b > $Z/branch-committed.txt; git -C $Z add .; git -C $Z $CI commit -qm "foreign branch commit"; BC=$(git -C $Z rev-parse HEAD)     # (ii)
echo "z staged" > $Z/z-staged.txt; git -C $Z add z-staged.txt; ZB=$(git -C $Z rev-parse :z-staged.txt)
L=$T/live; git -C "$REPO" worktree add -q --detach "$L" HEAD; (untracked + staged file in it)   # the refuted half's control
mv $Y $T/y.away; mv $Z $T/z.away                                     # directory absent

# before-control (identical in every cell)
ls $REPO/.git/worktrees/{y,z}       # COMMIT_EDITMSG commondir gitdir HEAD index logs ORIG_HEAD refs   (y also refs/worktree/wip)
git -C $REPO worktree list          # … <tmp>/y (detached HEAD) prunable · <tmp>/z [side] prunable · <tmp>/live (detached HEAD)
git -C $REPO fsck --unreachable     # FC: 0 hits · SB: 0 · BC: 0 · ZB: 0     (all reachable — from y's HEAD, y's index, side, z's index)
command grep -rl $FC $REPO/.git/logs $REPO/.git/worktrees | wc -l    # 2  (y's HEAD reflog, y's ORIG_HEAD/HEAD)
```

| cell | argv | exit | stdout (whole) | stderr | `.git/worktrees/{y,z}` after |
|---|---|---|---|---|---|
| provision (re-run) | `jigc milestone provision $M` | **0** | `provisioned 2 worktree(s) for milestone:foreign-worktree-probe at base <sha> (add-alpha-file, add-beta-file)` + footer | empty | **both gone** |
| provision (first) | same, no earlier provision | **0** | same line | empty | **both gone** |
| finalize | `jigc milestone finalize $M` | **0** | `finalized <sha> — Finalize milestone foreign-worktree-probe (2 sub-tasks)` / `added add-alpha-file.txt` / `added add-beta-file.txt` / `modified docs/milestone-records/foreign-worktree-probe.md` / `3 files committed` / `sub-tasks: …` + footer | empty | **both gone** |
| discard | `jigc milestone discard $M` (no staged sub-task code) | **0** | `discarded milestone:foreign-worktree-probe (2 sub-task(s); workbench removed)` + footer | empty | **both gone** |

After, identical in all four: `git worktree list` no longer names y or z; `<tmp>/live` byte-identical (shasum of both files, `status --porcelain`, HEAD); no word about any worktree registration on either stream.

Controls, each on its own rig:

| cell | argv | exit | records after |
|---|---|---|---|
| none | no command | — | present; directories put back → `git status` exit 0 in both, `git worktree repair` exit 0 |
| start | `jigc start` | 0 | present |
| locked | `git worktree lock` y and z, then `jigc milestone provision $M` | 0 | present (`locked` file beside them) |
| discard, refused | `jigc milestone discard $M` over staged sub-task code | 1 (`milestone.dirty-worktree`) | present — a refused discard does not prune |
| plain git | `git -C $REPO worktree prune -v` | 0 | gone; stderr `Removing worktrees/y: gitdir file points to non-existent location` (and z) — the lines jigc's bare prune never asks for |
| plain git | `git -C $REPO gc` | 0 | **present** — git's own automatic prune does not fire on a fresh record |

Two cells the record did not have:

- **`jigc milestone finalize $M` on a never-provisioned milestone: exit 3, `blocking · milestone.zero-contribution`, and both foreign records gone.** A *refusing* run of the door mutates `.git/worktrees/`. (Read, not instrumented: `milestone_boundary_gate` builds a `DedicatedWorktree` before the zero-contribution check, and its `add` and `Drop` each run a bare prune.)
- **`jigc milestone discard $M` on a never-provisioned milestone: exit 0, both gone.** No fan-out worktree ever existed; the trailing prune in `remove_worktrees` is unconditional.

So the precondition on jigc's side is not "a provisioned fan-out" — it is "any run of one of these doors".

## 2. What is actually lost — per plant

| plant | before | after the prune | where it survives |
|---|---|---|---|
| **(i)** commit on y's detached HEAD | reachable (fsck 0 hits), 2 reflog/HEAD files name it | **unreachable** (fsck 1 hit, with and without reflogs); 0 files under `.git/logs` or `.git/worktrees` name it | the object itself, in the object database, until gc. `git fsck --lost-found` lists it as dangling. **Not for long by default:** see the gc cell below |
| **(ii)** branch `side` checked out in z | `refs/heads/side` → BC | `refs/heads/side` → BC, BC reachable | fully — branch refs are shared, not per-worktree. Survives `git gc --prune=now` |
| **(iii)** staged-only blobs (y's SB, z's ZB) | reachable from each worktree's `index` | objects exist, **unreachable/dangling**; the `index` that named them is deleted | the bytes survive twice — as a dangling blob (until gc) and as the working file in the returned directory. What is gone for good is the index: which paths were staged, and any *staged-then-edited-again* version exists only as the dangling blob, with no path attached |
| per-worktree ref `refs/worktree/wip` | file present under `.git/worktrees/y/refs/worktree/` | file gone | nowhere. (Note: `fsck` run from the main checkout already reported its target unreachable *before* — it does not walk other worktrees' private refs — so this plant shows a deleted ref file, not a reachability change) |
| y's HEAD reflog, `ORIG_HEAD` | present | gone | nowhere |
| **(iv)** the directory comes back | (control) `git status` exit 0, `git worktree repair` exit 0 | `ls` shows every working file intact, including the unstaged edit; `git -C <tmp>/y status` → `fatal: not a git repository: (null)` **exit 128**; `git worktree repair <tmp>/y <tmp>/z` → `error: unable to locate repository; .git file does not reference a repository` **exit 1** | git's own repair cannot mend it. Hand recovery works (driven): recreate `.git/worktrees/y/{gitdir,commondir,HEAD}` — HEAD from the fsck dangling sha, or `ref: refs/heads/side` for z — then `git reset`; the result is `?? foreign-staged.txt` + ` M README.md`: files back, **staged/unstaged distinction lost, reflog empty**. That recovery needs the dangling commit to still exist and needs a reader who knows git's admin layout |

Not driven: an in-progress operation in the worktree (rebase/merge state lives in the same admin directory and would go with it), `config.worktree`/sparse-checkout state.

**The gc cell — the loss becomes permanent with git's defaults.** On the pruned discard rig, object mtimes backdated 21 days (simulating work older than `gc.pruneExpire`'s default two weeks; no gc config set in the rig), then a plain `git gc`:

```
before gc: FC exists · SB exists
git gc                       [exit 0]
after:  FC GONE · SB GONE · BC (the branch commit) exists
```

Control — the no-jigc rig, same backdating, same plain `git gc`: `FC exists · SB exists`, records present, the returned worktree `git status` exit 0. (`git gc --prune=now` on a pruned rig deletes FC regardless of age.) `git commit` runs `git maintenance run --auto`, so in a large repository the gc need not be typed by anyone.

## 3. Is this git's behaviour that jigc merely triggers?

jigc runs a **bare, repository-wide `git worktree prune`** — no `--expire`, no `--verbose`, output discarded. Plain git does the same thing to the same entry, with three differences that are the whole finding:

- **When.** git's own automatic prune is `git gc` → `git worktree prune --expire 3.months.ago` (git-config, `gc.worktreePruneExpire`). Driven: plain `git gc` on a fresh prunable record keeps it. Without jigc the user has three months plus `gc.pruneExpire`; with jigc the record goes at the next milestone door, and an aged detached commit at the next gc.
- **Who asked.** A user typing `git worktree prune` asked for it. A user typing `jigc milestone provision` asked for worktrees under `.jigc/worktrees/`.
- **What is said.** `git worktree prune -v` names each record it removes; jigc prints nothing.

git documents the state and its guard: *"If a worktree is on a portable device or network share which is not always mounted, lock it to prevent its administrative files from being pruned automatically"* (git-worktree(1), `lock`). The locked control holds under jigc. **`git worktree add` does not prune**, so the provision door's prune is jigc's own choice, not a side effect.

## 4. Is it declared?

**No design doc, guide or `--help` declares it.** `command grep -rn -i prun design/ crates/cli/guides/` finds no sentence about pruning worktree registrations; `jigc milestone {provision,finalize,discard} --help` and `jigc uninstall --help` do not contain "prune". What exists:

- **`DECISIONS.md` → 2026-06-20/21, M31 planning (the origin):** *"Idempotency = prune + reuse-if-registered + clear-stale-leftover, not blind --force. `git worktree prune` first (drops admin records for dirs a crash deleted)…"* and, for the teardown, *"…the registered check skips all ids and only the **harmless** `git worktree prune` no-op runs"*. The prune is declared as a mechanism for jigc's *own* crashed-run records and asserted harmless; a foreign record is not considered.
- **`DECISIONS.md` → 2026-09-23, the confirmation pass, `uninstall`:** *"It prunes now, best-effort as they are, and **says so**: a destroying door narrating what it changed in the repository is law 1."* The three milestone doors run the same prune and say nothing.
- **`design/storage.md` → Repository layout, `worktrees/<sub-task-id>/`:** *"No door removes such a path silently: each probes it first and then either refuses what it cannot prove is disposable (`--force` the one consent) or names every byte it is about to destroy"* — scoped to jigc's own paths; the doors' declared reach is that set. The prune reaches past it, probes nothing, names nothing.
- **`jigc milestone provision --help`:** *"Idempotent — reuses a live worktree untouched, and clears an **empty** leftover directory."* Nothing about registrations elsewhere.
- **In-source, the project's own grading of this loss on its own worktrees:** `crates/cli/tests/leftover_operation_in_progress.rs` module doc records `.git/worktrees/<sub>/` going with a discard — reflog gone, a detached-HEAD commit dangling until gc — as the HIGH it was written to close.

So: undeclared, and it contradicts one stated rule (law 1 as the 2026-09-23 decision applies it to this exact operation).

## 5. Tier

**Tier 1 by the predicate — and I do not think it argues down to 3.**

*For tier 1:*
- Exit 0 through a committing door (`milestone finalize`) and two destroying doors (`provision`, `milestone discard`), silent on both streams; before/after controls on every plant; three independent controls (none, locked, refused discard) attribute it to the doors.
- Repository harm that git's own repair cannot undo: a worktree that worked before the door is `not a git repository` after it (`repair` exit 1).
- Permanent loss is demonstrated, not inferred: aged detached commit + plain `git gc` → gone; control keeps it.
- The precondition is **not** "the user abandoned it". Two driven states where the worktree is alive and in use: the container cell (worktree never moved, never deleted, `git status` exit 0 on the host before, exit 128 after), and the plain-`mv` cell (worktree working at its new path before, exit 128 after).
- Tier 3 is "a surface says something the binary does not do". Here no surface says anything and the binary destroys; that is not a wording defect.
- The project already rated the identical loss on its *own* worktrees HIGH and closed it.

*Against tier 1 (what the human should weigh):*
- Working files are never touched; in every cell they are all still on disk.
- Branch work — the common shape of a linked worktree — survives entirely (ii). The irreplaceable part is confined to detached-HEAD commits, the index and the reflog.
- Permanence needs a second event (gc) and age, or `--prune=now`; inside that window a hand recovery works.
- git calls the state `prunable`, documents `lock`, and would itself prune the record after three months; jigc shortens a fuse git lit.
- It did not occur in the trial. It needs a foreign worktree whose recorded path is absent *and* a milestone door — the single-task loop (`task finalize`) never prunes.

The against-list lowers **likelihood and magnitude**, not the predicate. If the exit rule is read literally, this is a tier-1 row.

## 6. How a real adopter or agent reaches it

| trigger | driven? | datum |
|---|---|---|
| **Repository visible in a container, linked worktree's path not** (devcontainer / bind mount of the repo only) | **yes** (`l22-work/container2.sh`; one volume, "host" sees `/host/proj` + `/host/proj-wt`, "devcontainer" sees only the repo at `/work`) | host before: `git -C /host/proj-wt status` exit 0, FC reachable. Container: `git worktree list` → `/host/proj-wt … prunable`; ordinary git there leaves the record; `jigc milestone provision` exit 0, one `provisioned 2 worktree(s)…` line. Host after: `.git/worktrees/proj-wt` gone, `status` → `fatal: not a git repository` exit 128, `repair` exit 1, FC unreachable. The worktree was never moved or deleted |
| **Worktree (or its parent directory) renamed with plain `mv`** | **yes** | on git 2.54 the moved worktree keeps working indefinitely (`status` exit 0, `log` exit 0) while the main checkout lists the old path `prunable`; `jigc milestone provision` exit 0 → `status` exit 128, `repair` exit 1. Control: without jigc, `git worktree repair <new path>` heals it (exit 0) |
| Worktree on a volume not mounted right now | by equivalence (path absent = the `mv`-away cells) | git's documented example |
| Directory deleted by hand / by an agent's `rm -r` without `git worktree remove` | by equivalence | here the prune is close to what the user meant; the cost is a detached commit losing its only anchor |
| Same repo at a different mount path with a worktree *inside* the repo directory (agent tooling that keeps worktrees under the repo) | **not driven** — inference from git's absolute `gitdir` path rule | would make every such worktree prunable from inside the container |

Likelihood: **low, not exotic.** It needs the fan-out milestone doors, which are exactly where agents and linked worktrees coexist; the container trigger needs nothing unusual from the user except running jigc inside a devcontainer while keeping worktrees on the host. A mirror of the same mechanism bites jigc itself: the container's own provisioned worktrees are `prunable` on the host afterwards.

## 7. Code (read-only; nothing edited)

**Sites — every bare prune:**

| # | site | reached by | introduced (read with `git log -S` / `git blame`) |
|---|---|---|---|
| 1 | `crates/cli/src/milestone.rs` · `provision_worktrees` — `git_worktree(jigc_home, &["worktree", "prune"])?` before the registered-set read | `jigc milestone provision` | `3c2a7bbb` 2026-06-21 *feat(cli): add milestone provision verb* (M31); line last re-homed `f919ea95` 2026-09-23 |
| 2 | `crates/cli/src/milestone.rs` · `remove_worktrees` — trailing `let _ = git_worktree(jigc_home, &["worktree", "prune"])`, unconditional | `run_milestone_finalize` (both landed arms), `run_discard` | `7ea15567` 2026-06-21 *tear down fan-out worktrees on milestone finalize* (M31) |
| 3, 4 | `crates/cli/src/task.rs` · `DedicatedWorktree::add` and its `Drop` | `chain_commit`, `commit_combined_tree_with_hooks` (the two fan-out commit arms) and `checkout_tree_worktree` ← `milestone_boundary_gate` — all `milestone finalize`, including refusing runs | `94f870cd` 2026-06-21 *run user hooks on the squash:true fan-out combine commit* (M31) |
| 5 | `crates/cli/src/setup.rs` · `prune_worktree_admin` (`--verbose`, measured) | `jigc uninstall` | `fba875c3` 2026-09-23 (confirmation pass) |

**Inside which code:** none of M54, M55 or the co-author trailer. All four silent sites are **M31 (2026-06-21)**; no prune line changed after 2026-09-23 (`git log -G'worktree", "prune' --since=2026-09-24 -- crates/cli/src` is empty). Every release candidate since M31 carries it.

**Class:** four doors — `milestone provision`, `milestone finalize` (landing *and* refusing), `milestone discard` (landing only), `uninstall`. Not in the class: `jigc start`, single-task `task finalize` (its stage policies never build a dedicated worktree), a refused `discard`.

**`uninstall` driven (the record's O-2):** fresh rig, no milestone, one prunable foreign worktree → exit 0, record gone, and the ack says `- pruned git's worktree registrations for the fan-out worktrees `.jigc/` held`. `.jigc/` held no fan-out worktree; the only record dropped was foreign. The one door that narrates the prune **misattributes** it — a tier-3 sentence on top of the same removal.

**Contract a fix must restore:** a jigc door's reach over git's worktree registrations is the set jigc created — `.jigc/worktrees/<sub-task-id>` and `.jigc/worktrees/.combine-*` under `jigc_home`. It removes no registration outside that set, in any state; what it does remove it names (the 2026-09-23 rule); a refusing run removes none.

**Cheap vs robust — a genuine fork:**
- *Cheap:* keep the repository-wide prune, add `--verbose`, print what was dropped (uninstall's shape, with its sentence corrected). The foreign record is still destroyed; the door merely says so. Restores law 1, not the reach.
- *Robust:* key the removal to the owned set and drop the bare prune. git has the path-scoped verb: **`git worktree remove --force <path>` on a registration whose directory is absent removes that one record and leaves a prunable sibling in place** — driven on git 2.54.0 and 2.47.3 (`ls .git/worktrees` → `foreign` after removing `own`). Every site already holds the paths it owns and the registered set, so this is the existing key applied to the one unkeyed operation — no new mechanism. It must also cover the stale-`.combine-*` crash case the prune was added for.

## 8. Pin

**UNPINNED.** No suite builds a prunable foreign registration and runs any of the four doors.

- The nearest pin points the wrong way: `cwd_verb_subject.rs` (uninstall) asserts `!listed.contains("prunable") && !listed.contains(".jigc/worktrees")` — "git's worktree admin must be clean after the teardown" — over a fixture with **no foreign worktree**, so a repository-wide prune and an owned-set removal pass it identically, and a fix that leaves a foreign prunable record in place would turn it red.
- `repo_posture.rs`, `worktree_jigc_home.rs`, `golden_harness.rs`, `setup.rs` plant *live* foreign worktrees (never absent). `provision_leftover_guard.rs` and `leftover_operation_in_progress.rs` plant at jigc's own sub-task paths. `milestone.rs` pins the leaked-worktree warning's `git worktree prune` remedy text.
- Why none caught it: every fixture's foreign worktree is live, and a live record is not `prunable`; the M31 decision recorded the teardown prune as a "harmless no-op" and no test was asked to disagree.

Tests were read, not run.

## Not covered

`--format json` on any cell; an in-progress operation or `config.worktree` in the foreign worktree; the in-repo-worktree-at-a-different-mount-path trigger; which of finalize's two commit arms (`Combine` / `ChainPerSubtask`) the default cell took; the refusing-finalize prune was attributed by reading, not by instrumenting. The gc cell simulates age by backdating object mtimes.
