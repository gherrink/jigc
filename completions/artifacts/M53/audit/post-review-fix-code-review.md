# Independent review — `986d5e0a` (the M53 post-review fan-out posture fix)

I read the whole diff, both design-doc deltas, the surrounding `repo.rs` / `cli.rs` / `milestone.rs` code in full, and drove the **debug binary** (`target/debug/jigc`, built bare) plus the installed `1.0.0-rc.17` against `dev/jigc-rig committed-singletons` corpora. Targeted suites run: `repo_posture::` (16/16), `posture_door_axis::` (7/7), `posture_member_inventory` (4/4). I did not run `dev/gate`.

**The core fix is real and it works.** Every claim in the fixer's list that I could reach, I reached; the load-bearing ones are DRIVEN below. The findings are siblings, a contract-registration gap, and a record count.

---

## What I verified holds

| claim | verdict | evidence |
|---|---|---|
| The worktree's **own** git dir is the marker subject | HOLDS · **DRIVEN** | `git bisect` started only in `.jigc/worktrees/area-low` (main `.git` marker-free) → `blocking · repo.operation-in-progress — a bisect is in progress in the fan-out worktree \`.jigc/worktrees/area-low\``, exit 3. `worktree_git_dir` (`crates/cli/src/repo.rs:1180-1206`) reads the `gitdir:` pointer, so `detect` resolves `.git/worktrees/<n>/`. |
| `UnmergedIndex` (the one member with **no** on-disk marker) runs `git ls-files -u` in the worktree cwd | HOLDS · **DRIVEN** | Hand-built stage-2/3 index entries in the worktree only, main `git ls-files -u` empty → refused naming the worktree. `index_has_unmerged_paths` uses `.current_dir(repo_root)` (`repo.rs:545-553`). |
| The `HeadDetached` exemption does **not** leak to the main checkout | HOLDS · **DRIVEN** | `git switch --detach` in the main checkout → `blocking · repo.head-detached … exit 1`, byte-identical `Here` text. The `owes: \|_\| true` is applied to `posture_subject(worktree)`, never to the cwd's subject (`milestone.rs:3797-3805`); `BEHALF_DOORS`' `milestone finalize` row is `CommitsOnBehalf { exempt: &[] }` (`cli.rs:2246-2252`). |
| `HeadUnborn` is unreachable **and stated on the production side** | HOLDS · READ | `PostureSubject::adjudicates`' doc: *"an unborn HEAD cannot occur in a checkout git created at a commit"* (`repo.rs:793`). The test-support refusal cites it accurately. |
| `Here` bytes unchanged at all 12 acting `BEHALF_DOORS` rows, `location: None` preserved | HOLDS · **DRIVEN + test** | `posture_door_axis::` 7/7 and `repo_posture::` 16/16 green (incl. `SHIPPED_ROUTE_LINES`); `Finding::graded(Blocking, code, msg, None, Some(route))` is field-for-field `Finding::block` (`engine/src/finding.rs:1382-1425`), and `fence_addressed_token_is_quoted` early-returns on a `None` address (`:1448-1456`). Driven main-checkout refusals above reproduce the shipped text verbatim. |
| Nothing durable is written on the refused path | HOLDS · READ + driven | The call sits at `milestone.rs:4994`, immediately after the worktree set and **before** `milestone_boundary_gate`, `flip_record_for_finalize` (the `RecordFlipGuard`) and the planner. Everything above it writes only gitignored workbench state (`reseed_cache`, `materialize`). Driven: repeated refusals left HEAD and the record `active` (a later `milestone discard` still worked, which a flipped record refuses). |
| The probed set **is** the committed set | HOLDS · READ | One `worktrees` binding at `milestone.rs:4984` feeds the probe (`:4994`), the gate (`:5039`), `worktrees_have_staged_code` (`:5085`), `detect_code_collision` (`:5215`), `subtask_patches_and_messages` (`:5229`) and `StagePolicy::Combine` (`:5413`). No parallel derivation. |
| A worktree git cannot vouch for is excluded from **both** | HOLDS · **DRIVEN** | Corrupted the worktree's `.git` pointer with staged bytes present → `milestone.zero-contribution` exit 3, no panic, `x.txt` **not** in any commit. |
| `milestone finalize --dry-run` does not exist | HOLDS · **DRIVEN** | clap: `error: unexpected argument '--dry-run' found`. |
| The `git ` prefix fence is derived and real | HOLDS · READ | `repo_posture.rs::every_routed_command_is_a_git_invocation` iterates `InProgress::ALL`; a member routed at `rm …` reddens. |
| The new arms iterate a **derived** axis | HOLDS · READ | They iterate `GitState::ALL`, and `repo_posture.rs:529-537` is a standing surjectivity fence — *"every member of `InProgress::ALL` must be produced by at least one `GitState`"*. |
| The residue assertion covers marker **bytes**, not presence | HOLDS · READ | `operation_residue` reads `fs::read(...)` over `MARKER_UNIVERSE`, which includes `MERGE_MSG` and `SQUASH_MSG` (`git_state.rs:314-328`). |
| The new pre-`TaskArea::resolve` call does not regress the malformed-id guard | HOLDS · **DRIVEN** | `jigc task validate "../.."` and `""` still answer `work-unit.malformed-id`. `owning_milestone` compares against the recorded list and builds no path from the raw token (`engine/src/milestone.rs:1006-1021`); `worktree_path` is reached only after it returns `Some`. |

---

## Findings

### HIGH — `jigc milestone discard` destroys a fan-out worktree's live git operation at exit 0, no consent, no narration; the carried disposition's warrant is driven false

**Location:** `crates/cli/src/milestone.rs:4461-4485` (`dirty_worktrees`), reached from `classify_leftover` at `:3582`; disposition recorded in `DECISIONS.md` (2026-09-22 entry, *"Out of the class"*) and the commit message.

**Evidence — DRIVEN.** A one-sub-task milestone, its worktree paused mid-`rebase -i` at a `break`, holding a commit created during the rebase and reachable only from the worktree's detached HEAD; `git status --porcelain` in that worktree is **empty**:

```
$ jigc milestone finalize second-wave
blocking · repo.operation-in-progress — a rebase is in progress in the fan-out worktree
  `.jigc/worktrees/beta-area` …                                            EXIT=3   ← the fix working

$ jigc milestone discard second-wave            # no --force
discarded milestone:second-wave (1 sub-task(s); workbench removed)          EXIT=0
$ ls -d .jigc/worktrees/beta-area  →  No such file or directory
$ git rev-list --all | grep -c 1c39ef2b…  →  0      # reachable from no ref
$ git fsck --lost-found | grep 1c39ef2b… →  dangling commit 1c39ef2b…
```

The `rebase-merge/` todo, `ORIG_HEAD` and the worktree's own reflog are gone with the registration; the tip commit survives only as a dangling object until `git gc` prunes it. Nothing was printed about the rebase.

**Why I call this tier-1 rather than a defensible carry.** The recorded warrant is *"a destroying door whose guard refuses over content and whose `--force` is its consent"*. That is true only where the operation leaves content. `dirty_worktrees` asks `git status --porcelain` — a question about *working-tree bytes* — where the door's exposure is *repository state git holds*. In the clean-tree cell (`Bisect`, a `break`-paused rebase, a `--quit`-able sequencer) **no guard fires at all**, so the consent the disposition points at is never asked for and `--force` is never reached. That is exactly the shape M50's audit adjudicated at this same door for staged prose ("a basis-has-changed reconciliation, not an override").

**Class bound — derived.** `DESTROYING_DOORS` has six members (`milestone.rs:3338`); its own comment at `:3346-3349` names the **four** worktree-shaped ones, and all four reach the same `git status --porcelain` probe through `classify_leftover` / `probe_leftover` (2 call sites: `milestone.rs:3582`, `setup.rs:2730`). I drove **two** of the four: `milestone discard` (above) and `jigc uninstall`, which also removed the worktree directory at exit 0 with no mention of the paused rebase — weaker there, because the stale `.git/worktrees/<n>` registration survives, so the commit stays ref-reachable until `git worktree prune`. I did not drive `milestone provision` or `task discard`.

**Smallest correction:** give the leftover classifier a second leg — `cli::repo::posture(path)` non-empty, or the presence of any `MARKER_UNIVERSE` entry in the worktree's git dir — so a clean-tree live operation reaches the same collect-then-refuse narration the dirty cell already has, with `--force` as its consent. The probe already exists and is now shared; this is one disjunct, not a new mechanism.

---

### MEDIUM — the new finding leaks an absolute host path into its message, its pinned `(code, target)` key and its route when the command is run from inside a linked worktree

**Location:** `crates/cli/src/milestone.rs:3801` — `let at = render::repo_relative(repo_root, worktree);`

**Evidence — DRIVEN.** Two sub-tasks, `zed-two` mid-bisect, `jigc milestone finalize zeta-probe` run from **inside** `zed-one`'s worktree:

```
blocking · repo.operation-in-progress — a bisect is in progress in the fan-out worktree
  `/private/var/folders/nj/…/repo/.jigc/worktrees/zed-two` — …
  at: /private/var/folders/nj/…/repo/.jigc/worktrees/zed-two
  route: … abandon it with `git -C /private/var/folders/nj/…/repo/.jigc/worktrees/zed-two bisect reset`
```

Same leak from the preview: `jigc task validate zed-two` inside `zed-one`. Cause: `repo_root` here is `discover_repo_root(cwd)` (`milestone.rs:4906`, and `:3826` in the preview) — the *worktree* — while every path in `worktrees` is rooted at `jigc_home` (`subtask_worktrees` joins `canonical_home`, `:3737`), so the prefix strip fails. This is a **law-1** violation (`design/surface-contract.md`; `render::repo_relative` is its one home since M50 Inc 10), and it puts a machine-specific string in the `(code, target)` key the `--format json` `Blocked` arm pins.

**Class bound — derived by grep, partly driven.** `grep -c "repo_relative(&\?repo_root" crates/cli/src/milestone.rs` → **20** call sites, most of which render a path under `jigc_home`. I drove **two**: the new one above, and the pre-existing `milestone.dirty-worktree` producer (`:4575`), which leaks identically and renders the cwd's own worktree as `at: .`. So the new site **joins** a pre-existing class rather than creating it — but it is a new member, and the only one whose leaked string becomes a pinned finding key.

**Smallest correction:** `fan_out_posture_findings` and `sub_task_fan_out_refusal` already have `jigc_home` in hand at both call sites; render against it (`render::repo_relative(&jigc_home, worktree)`), since the worktree set is rooted there by construction.

---

### MEDIUM — the same code now answers on two different declared envelope arms, two streams and two exit codes at the door and at its own preview, and the new target form is registered in neither contract home

**Locations:** `crates/cli/src/milestone.rs:3838-3845` (preview → `invocation_log::operational_failure`, flattened) vs `:4995-4997` (`blocked` → `render::validation`); `design/command-output-contract.md:246` (the filesystem-path form's member list); `crates/cli/src/render.rs:5451-5456` (`ENVELOPE_OWED_CODES`).

**Evidence — DRIVEN.** One state, two doors:

```
$ jigc --format json milestone finalize <id>
  stdout 728 bytes: {"schema_version":3,"findings":[{… "key":{"code":"repo.operation-in-progress",
                                                     "target":".jigc/worktrees/area-low"} …}]}
  stderr 0 bytes                                                              EXIT=3

$ jigc --format json task validate <sub>
  stderr: {"error":"blocking · repo.operation-in-progress — …"}   stdout empty  EXIT=1
```

`repo.operation-in-progress` is now a finding that **projects a key** (`command-output-contract.md`'s membership predicate: *"a finding is in the envelope-projecting set iff it is serialized as a `Finding`"*), and the target it projects is a filesystem path. But: it appears in **none** of the six declared target forms' Members lists, it is not in `is_declared_singleton` (`engine/src/finding.rs:251-260`), and it is not in `ENVELOPE_OWED_CODES`. The doc's closure claim — *"Every finding that projects a `key` lands in exactly one of those six forms, or in one of the named exceptions below"* — is falsified at HEAD by this commit, and nothing reddens: `debug_assert_targets_declared` checks *presence and uniqueness*, never form membership.

This is the divergence M52's completion audit closed one family over (five doors flattening `store.unknown-type` while others enveloped), re-opened.

**Note on the fix shape:** naively adding `repo.operation-in-progress` to `ENVELOPE_OWED_CODES` would **panic the debug binary**. `carrier` can only widen (`render.rs:5504-5511`), so every `Here` refusal at the 12 acting doors would take the findings arm carrying `location: None`, and `Finding::serialize`'s `debug_assert!(self.carries_declared_target())` (`finding.rs:1223-1224`) fires.

**Smallest correction:** declare the split rather than papering it — register `repo.operation-in-progress` under the filesystem-path form **at `BreachSite::FanOutWorktree`**, and as an `is_declared_singleton` exception at `BreachSite::Here` (the guard takes the **first** breach, so `Here` is fail-fast one-per-invocation — precisely the rationale the list already uses for `setup.*` / `uninstall.*`). Then decide whether the preview owes the envelope arm; it costs a `ENVELOPE_OWED_CODES` entry only once `Here` is exempt.

---

### MEDIUM — "nine of the ten `InProgress` members" is a count with a stated reason that names a non-member, landed in six homes, one of which is a `COUNT_HOMES` file whose own fence states the rule it breaks

**Locations (derived: `git show 986d5e0a | grep -n nine` → 6 file homes + the commit message):** `DECISIONS.md:9` · `crates/cli/src/repo.rs:47-48` · `crates/cli/src/milestone.rs:3779` · `crates/cli/tests/commit_seam_posture.rs:743` · `design/finalize.md:35` · `design/validation.md:660`.

**Evidence.**
- READ: `InProgress::ALL` has **ten** members (`repo.rs:289-298`) and `Unborn` is **not** one of them — it is a `PostureMember`. `GitState::worktree_refusal` excludes exactly one `GitState`, `Unborn`, whose `in_progress()` is `None` (`git_state.rs:258, 265-274`). With `repo_posture.rs:529-537`'s standing surjectivity fence (*every* `InProgress` member is produced by some `GitState`), the fan-out axis reaches **ten of ten** — which is what the new acceptance asserts (`refused + excluded + 1 == 17`, i.e. 15 refusing cells).
- **DRIVEN** on the installed `1.0.0-rc.17` (the pre-fix binary), the member I suspected might be the outlier: an unmerged-index worktree → `finalized 9141092 … 2 files committed`, **exit 0**, worktree torn down. So the pre-fix landing was over ten, not nine.
- The test file's variant is wrong differently: *"nine of these cells landed at exit 0"* — there are **fifteen** refusing cells.

**Why this is not just a typo.** `crates/cli/tests/posture_member_inventory.rs`'s arm 4 — `no_home_of_the_family_states_a_count_it_can_move` — declares the rule *"no home of the family states a count of `cli::repo::InProgress::ALL`"* over `COUNT_HOMES = ["crates/cli/src/repo.rs", "crates/cli/tests/repo_posture.rs"]` (`:105, :317-340`), and its `STRUCK_COUNTS` carries the literal `"nine of the ten members"` struck at M53 Increment 4 with its falsifying datum. This commit re-introduced that exact shape into `crates/cli/src/repo.rs`, and the fence stayed green **only because the doc-comment wraps between `**nine**` and `of the ten`** and inserts `` [`InProgress`] `` — a literal mismatch, not a satisfied rule. Arm 3's design/ scan (`:274-299`) keys on a different struck literal, so the two design docs pass too.

**Smallest correction:** strike the numeral in all six homes with the datum (the surjectivity fence + `Unborn ∉ InProgress::ALL`) — state *every member of `InProgress::ALL` buildable in a linked worktree*, which is what is true and what the acceptance actually iterates.

---

### LOW — the site clause lands after a predicate that already ends in a prepositional phrase

**Location:** `crates/cli/src/repo.rs:585-590` / `:702-710`. **DRIVEN:** `a conflict left unmerged paths in the index in the fan-out worktree \`…\``; the `Sequencer` member composes `left a queue of commits in \`sequencer/\` in the fan-out worktree \`…\``. Two of ten members read badly because `subject()` is appended after `predicate()`. Smallest correction: put the site before the predicate for the two members whose predicate ends in a phrase, or lead with it (`in the fan-out worktree \`X\`, a conflict left …`).

### LOW — the acceptance's refusing cells run against an **attached** worktree HEAD; production worktrees are detached

**Location:** `crates/cli/tests/support/git_state.rs:930-938` — `overlay_worktree` does `git switch -q -c posture-wt-<id>` before `drive`. So every refusing cell probes a subject where `posture()` returns a **single** breach, while production always returns `[OperationInProgress, HeadDetached]` and depends on probe order plus the `Dedicated` exemption to pick the right one. The adaptation is honestly documented and the two proceeding cells cover the detached subject; I **drove** the production topology myself (all my rig probes ran on genuinely `--detach`ed worktrees and refused correctly), so this is a fixture-fidelity narrowing, not a live defect. Smallest correction: one cell that leaves HEAD detached while carrying a marker-only operation (`Bisect` needs no branch).

### LOW — `overlay_worktree`'s one-worktree-per-repository bound is stated in prose only

**Location:** `crates/cli/tests/support/git_state.rs:909-914`. The per-worktree branch is parameterised (`posture-wt-<id>`), but `drive`'s internal `THEIRS_BRANCH` / `CLEAN_BRANCH` (`:548, :551`) are not, and nothing asserts the single-worktree precondition. A second fixture worktree would fail at git rather than at a stated assertion. Smallest correction: assert the caller's worktree is the only one under `.jigc/worktrees/` in a driven state, or parameterise the two constants.

---

## Verdict

**HOLDS WITH FINDINGS.**

The deliverable genuinely holds: the boundary's probe subject is now every checkout it commits from, the widened subject resolves each worktree's own git dir for marker-keyed **and** index-keyed members (both DRIVEN), the `Here` path is byte-identical across all 12 acting doors (DRIVEN + two suites), the refusal is placed before anything durable, the probed set and the committed set are one value, and the preview no longer answers *validates clean* over a state the door refuses. `join` / `execute` / `provision` are correctly out of class, and a worktree git cannot vouch for is excluded from both the probe and the commit (DRIVEN, no panic, no false green).

What the commit did **not** close is the sibling it carried: the same boundary's *destroying* door tears down the same worktree's live operation at exit 0 with no consent and no narration, on a warrant (`--force` is its consent) that the clean-tree cell falsifies — that is the HIGH, and it is the one I would not ship past 1.0 without a decision. The other three are the fix's own residue: a law-1 host-path leak in the new finding's key, an unregistered target form that re-opens the door-divergence M52 closed, and a wrong count that slipped past the fence built to stop exactly it.
