# Fixers' reports of the rc.24 fix pass, rounds 1 and 2 — verbatim structured returns

Each section is one fixer's own report. They are leads for the record, not verified facts: check a claim against the tree before writing it into a log.

## 12398ddc fix(uninstall): the teardown answers for every byte inside a transient directory

### status

fixed

### gate

dev/gate (full, shared target, foreground, exit 0) on the committed tree: `tests   passed=4408 failed=0  (over 17 test binaries)` · `GATE: PASS`. A first full run was red at 4405/3: my new help fence misread clap's one-line option layout, and two source-scan fences (migrate_route_family, task_area_writer_registry) counted the new code. All three were fixed before the passing run; nothing was committed past a red gate.

### class

Axis as derived: every byte `remove_dir_all(<repo>/.jigc)` takes that no guard refuses over and no narration names, plus the two mechanisms the decision attached (a `git add` route over an ignored path; the log writer re-creating `.jigc/`). Handed: 5 positions (logs/, state/, index/, non-directory children of tasks/ and milestones/) plus the log. Found: 12 members converted, 6 dispositioned out.

Doors: one. `uninstall` is the only production sink that removes index/, state/, logs/ or the tasks/ and milestones/ roots (checked by enumerating every production `remove_dir_all`: the others take one worktree path, one task area or one milestone area).

Converted (12):
1-3. A byte jigc did not write anywhere under `.jigc/logs/`, `.jigc/state/`, `.jigc/index/` (nested and symlink included) blocks with the existing `uninstall.foreign-bytes`.
4-5. A non-directory child of `.jigc/tasks/` or `.jigc/milestones/` blocks with the same code.
6. `.jigc/logs/invocations.jsonl` blocks with the existing `uninstall.untracked-workbench-file`. It is asked that subject's own index question, so a force-added, unmodified log is narrated as recoverable instead.
7. jigc's own rebuildable files (file-state record, edge index, the save lock beside each, the install-footprint record, a killed save's `<name>.<pid>.<nanos>.tmp`) go with the tree and are now named on stderr. The footprint record and the temp are beyond the four files the decision listed; they are in the row because leaving them out would have the guard call jigc's own file foreign.
8. The tracked-then-edited leg inside those directories blocks (the subject is the path, not the index).
9. `--force` names what it took from all of those positions.
10. Any future `ENTRIES` prefix: the guard iterates the constant and an unowned prefix refuses over every leaf (it used to fail open). The acceptance arm iterates `cli::gitignore::ENTRIES`, planting inside each prefix at depth 1 and depth 3.
11. The `git add` route: keyed on `git check-ignore` per listed path, so it is true for any ignored path (an operator's own ignore rule too), not only the log. The all-not-ignored route is byte-identical to before.
12. The cause of (R9, F3): the log writer creates `logs/` only, never the `.jigc/` above it.

Out, with reason (6):
- `worktrees/`: already owned by `uninstall.dirty-worktree` over every child; the gitignored-build-output bound inside a clean live worktree is declared (visible, not prevented).
- Directory children of tasks/ and milestones/ (working areas and residuals): already owned by the registry complement and staged-prose guards.
- `displaced/`: already owned.
- A force-added, unmodified foreign file inside a transient directory refuses under foreign-bytes although the index has a copy. This is the working areas' existing precedent, a safe-side over-refusal with a working route.
- A refused `uninstall` with the knob on is still recorded, and creates the log if it was absent. Stated in design/measurement.md item 7; not special-cased because any intermediate jigc command would create it anyway.
- `jigc migrate`'s own `git add -- <recorded source>` route (migrate.rs): a different door and code, not driven here.

### pinned_surfaces_moved

No finding code added or removed (`UNINSTALL_DOOR.codes` stays four, `DESTROYING_DOORS` six), no `--format json` key, no `contract-version`, no frozen schema hash, no compose golden.

What did move, none of it pinned by a literal:
1. QUICKSTART.md bytes move the installed guide's `jigc-body-blake3` digest. It is computed at install; no test or golden pins a literal. This spends a guide-byte move in this commit rather than in the pass's single guide batch, because the task said docs go in the same commit and the fix made the guide's "every jigc run appends one record" false.
2. `uninstall --help` prose.
3. The `uninstall.untracked-workbench-file` route text, only when a listed path is gitignored.
4. One new stderr narration block on a teardown that takes jigc's own cache files.
5. The exit of `jigc uninstall` at the decided cells (0 -> 1).
6. A successful `jigc uninstall` with the knob on no longer writes its own invocation record.
7. One counted row in tests/task_area_writer_registry.rs (task.rs complement joins 3 -> 4, reason stated in the row).
8. `jigc-engine` gains one additive pub fn, `state::is_temp_sibling`.

### notes

(R9, F5) is fixed over its class, with the cause of (R9, F3) folded in, as one commit on `fix/rc24-tier1`: `12398ddc fix(uninstall): the teardown answers for every byte inside a transient directory`. Tree clean; nothing pushed, no branch switched.

Gate: `tests   passed=4408 failed=0  (over 17 test binaries)` · `GATE: PASS`.

What changed in behaviour:
- **Foreign bytes:** the foreign-byte guard used to list tasks/, milestones/ and displaced/ by name. It now iterates `gitignore::ENTRIES` and resolves each prefix to an owner (`PrefixOwner` in crates/cli/src/setup.rs). The default row — index/, state/, logs/ and any future prefix — refuses over every leaf jigc did not write.
- **Strays beside areas:** two new `AreaKind` rows in crates/cli/src/task.rs cover non-directory children of tasks/ and milestones/ (`UnitRoot`) and the cache/log directories (`Transient`).
- **jigc's own files:** `own_transient_paths` builds the row from the writers' own path functions (`FileStateRecord::path_in`, `EdgeIndex::path_in`, `lock_sibling`, `INSTALL_FOOTPRINT_PATH`, the new `invocation_log::log_path`), plus the new `engine::state::is_temp_sibling` for a killed save's temp. Rebuildable ones go with the tree and are named by a new narration.
- **The log:** it joins the third subject and blocks under `uninstall.untracked-workbench-file`.
- **The route:** that refusal asks `git check-ignore -z --stdin` and names `git add` only for paths git will take.
- **The writer:** `append_record` uses `create_dir(logs)` instead of `create_dir_all`, so a successful teardown leaves no `.jigc/` and the second run is a no-op.

Tests (red first: 10 new arms failed on the unfixed tree for the stated reasons):
- crates/cli/tests/uninstall_workbench_subject.rs gains 11 arms, already registered in groups/g_milestone.rs. The class arm iterates `ENTRIES`; the own-file completeness arm iterates `State::ALL` of the trial substrate and reads state/ and index/ off the disk; the log and ignored-path arms follow the printed route.
- crates/cli/tests/help_truth.rs gains a fence keyed on `UNINSTALL_DOOR.codes`.
- Unit tests cover the prefix rows, the log writer and `is_temp_sibling`.

Three things you may want to rule on, all listed in `left_open`:
- I changed one QUICKSTART sentence in this commit because the fix made it false; that moves the installed guide's body digest outside the pass's guide batch.
- The install-footprint record and killed-save temps are treated as jigc's own disposable state, beyond the four files the decision named.
- A refused uninstall is still logged.

All routes were re-driven on a release binary built from the final tree; exits are in `routes_driven`.

### routes_driven

- Built target/release/jigc (jigc 1.0.0-rc.24) from the committed tree; every cell below ran in a fresh `dev/jigc-rig fresh --binary target/release/jigc` rig.
- LOG, refusal: `jigc uninstall` with the knob on -> exit 1 · `blocking · uninstall.untracked-workbench-file` listing `.jigc/logs/invocations.jsonl` · route verbatim: "git ignores every path above, so `git add` cannot keep a copy of them: move what you need out of `.jigc/`, or delete what you do not, then re-run `jigc uninstall`; `jigc uninstall --force` deletes them with the install" · `.jigc/` still present, 4 earlier records intact plus the refusal's own
- LOG, move arm: `mv .jigc/logs/invocations.jsonl <outside>` -> exit 0; `jigc uninstall` -> exit 0, `.jigc` absent afterwards (not re-created), moved log keeps its 5 records; second `jigc uninstall` -> exit 0 "(nothing to remove — no repo-local jigc install was present)"
- LOG, delete arm: `rm .jigc/logs/invocations.jsonl` -> exit 0; `jigc uninstall` -> exit 0, `.jigc` absent; second run -> nothing to remove
- LOG, consent arm: `jigc uninstall --force` -> exit 0 · stderr "warning: removing `.jigc/` destroys 1 file(s) under it that no index has a copy of: .jigc/logs/invocations.jsonl … not recoverable" · `.jigc` absent
- LOG, `jigc uninstall --format json` -> exit 1 · one finding, code `uninstall.untracked-workbench-file`, envelope keys unchanged (schema_version 3, key.target null), same route text
- FOREIGN, refusal: `jigc uninstall` over `.jigc/state/notes.txt` -> exit 1 · `blocking · uninstall.foreign-bytes` · route verbatim: "move what you need out of the paths above, or delete the ones you do not (`rm -r` takes them — jigc has no verb that clears `.jigc/displaced/`), then re-run `jigc uninstall`; or, once you have confirmed they hold nothing you need, `jigc uninstall --force` deletes them with the install"
- FOREIGN, per position (.jigc/logs/notes.txt, state/notes.txt, index/notes.txt, tasks/notes.txt, milestones/notes.txt, logs/sub/deep/notes.txt): `jigc uninstall` -> exit 1, `uninstall.foreign-bytes`, path named, bytes intact; move the file out, then `jigc uninstall` -> exit 0, `.jigc` absent — all six cells
- FOREIGN, delete arm: `rm -r .jigc/state/notes.txt` -> exit 0; `jigc uninstall` -> exit 0
- FOREIGN, consent arm: `jigc uninstall --force` over `.jigc/logs/notes.txt` and `.jigc/tasks/notes.txt` -> exit 0, both named on stderr ("removing the workbench directory .jigc/logs …", "removing the work-unit directory .jigc/tasks …")
- MIXED listing (`.jigc/notes.md` plus the log): `jigc uninstall` -> exit 1; the route's `git -C <REPO> add -- .jigc/notes.md`, run from `/` -> exit 0; control: the same command on the ignored log -> exit 1; move the log out, then `jigc uninstall` -> exit 0, `.jigc` absent
- OWN STATE (committed-singletons rig, debug binary of the same code): `jigc uninstall` -> exit 0, stderr names `.jigc/index/edges.json.lock`, `.jigc/state/file-state.json`, `.jigc/state/file-state.json.lock`

### docs_changed

- design/project-setup.md — Teardown / cleanup (G5): (c) gains the invocation log and the per-path route rule; (d) states the prefix-iterating derivation, corrects the "two subtrees" miscount, and adds the own-file row and the named-as-they-go rule
- design/measurement.md — The in-repo invocation log, new item 7: the log blocks the teardown; the writer never re-creates `.jigc/`; a refused teardown is still recorded
- design/worked-examples.md — flow 51 arm 6: "narrows the subject automatically" is qualified as safe only because every excluded prefix now has an owner
- crates/cli/src/cli.rs — `jigc uninstall --help` about (the log, the five directories, the ignored-path exit, the named caches) and the `--force` option ("all three guards" -> "all four guards")
- crates/cli/guides/QUICKSTART.md — the invocation-log sentence: one run appends nothing (a successful uninstall), and the log blocks the teardown
- Doc-comments in crates/cli/src/setup.rs (run_uninstall, uninstall, workbench_paths, workbench_foreign_subject, INSTALL_FOOTPRINT_PATH), crates/cli/src/task.rs (AreaKind) and crates/cli/src/invocation_log.rs (append_record)

### left_open

- DECISIONS.md and decisions-pending.md were not touched — they are the orchestrator's shared logs. Owed there: the fork-3 ruling (the log blocks; own caches go, named), and the robust alternative left for 1.x (a writer registry inverted over the whole tree with a source-scan fence).
- The pack-default comments in crates/cli/packs/dev/config/knobs.yaml and crates/cli/packs/methodology/config/knobs.yaml (and the fixture copy in pack.rs) still say one record is appended "per `jigc` invocation" and list five of the eight record keys. Left alone because they are pack files; the design of record (measurement.md item 7) states the exception.
- The guide batch: this commit already moved one QUICKSTART sentence. If the pass's rule is strictly one guide commit, the remaining guide sentences from the other families join a second commit in the same release — still one digest move per release.
- A force-added, unmodified foreign file inside logs/, state/ or index/ refuses with a message saying nothing else has a copy, which overclaims for that cell. Same precedent as the working areas; not changed.
- With the knob on, a refused `uninstall` still appends its record, so a log deleted before a refusal for another reason reappears and blocks the next run once. It converges in one extra move or delete. A human call if "never trips over a log the first one wrote" is meant to cover refused runs too.
- tests/flow53_acceptance.rs `every_destroying_door_answers_for_the_bytes_it_did_not_write` still drives `uninstall` over one subject; the class axis is pinned in tests/uninstall_workbench_subject.rs instead.
- decisions-pending (I), the footprint subtraction at `uninstall`, is unchanged and still owed at M57.

## dc0d7586 fix(setup): a file the install would replace refuses on an unborn HEAD too

### status

fixed

### gate

tests   passed=4419 failed=0  (over 17 test binaries) — GATE: PASS (full `dev/gate`, second run, on the tree committed as dc0d7586; fmt/clippy/build/test/doctest all exit 0). The first run was GATE: FAIL (step: test), passed=4417 failed=2: `git_span_aim::every_production_caller_of_the_one_home_is_a_row` (the new route's `git rm --cached` span needed its registry row) and `setup::setup_writes_compose_embedded_methodology_marker` (a pinned cell that seeded an untracked packs.yaml on an unborn HEAD — see notes). Both fixed before the passing run; nothing was committed past a red gate.

### class

Mechanism: the unborn-HEAD exemption lived in the QUERY (`dirty_against_head` asked `git status --untracked-files=no` when HEAD was unborn), so no untracked path could reach any consumer. It is now a property of the MEMBER: exempt only when untracked AND unborn AND the member's writer preserves (`DirtyPaths::rides_the_first_commit`).

HANDED: 4 paths (`.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/version`, `packs.yaml` comments) + `--force` naming nothing + S22's re-arm + the dead `git stash` route. REAL, by axis:

A. Install members (10, from the production enumeration with every conditional on) — 4 converted, 6 out:
- `.jigc/AGENT.md` — Replaces — CONVERTED (refuses).
- `.jigc/version` — Replaces + ownership oracle — CONVERTED (prose refuses; a one-line `jigc-version:` stamp stays exempt, driven).
- `.jigc/config/.gitkeep` — Replaces — CONVERTED.
- `.jigc/config/packs.yaml` — declared Replaces — CONVERTED (see notes for the choice).
- `CLAUDE.md`, `.claude/settings.json`, `.jigc/.gitignore`, root `.gitignore`, foreign `pre-commit` (in-worktree hooks dir) — Preserves — OUT: the declared on-ramp; each driven through the binary with the adopter's mark found in the first commit.
- guide `SKILL.md` — OUT: a copy that is not jigc's is dropped from the install before the gate (M48), left byte-identical, advisory raised; driven.

B. Consumers of the query (5, all converted by the one predicate): pre-write gate; `--force` advisory (`setup.forced-install-path` now names the replaced path and not the merged-into one); commit-time backstop; `record_failed_install` before/now pair; `commit_install`'s own-footprint record.

C. HEAD shapes (2, both converted): zero-commit repository; orphan branch with an emptied index.

D. Occupant shapes (5): regular untracked file — converted; live untracked symlink — converted (target untouched); DANGLING untracked symlink — converted, and NOT in the handed list: the pre-write filter asked `exists()`, which follows the link, so it dropped out of the gate and the writer created the link's target before the backstop refused; the filter now uses `symlink_metadata` (this also moves the born-HEAD dangling-link refusal from the backstop to the gate, same code, nothing written); staged file on unborn — already refused, route fixed; a directory at the path — OUT, the writer errors and destroys nothing.

E. Route arms (2, both converted): pre-write and backstop wording on an unborn HEAD.

Difference from handed: +1 occupant shape (dangling symlink), +1 route trap (committing only the named path births the repo and the re-run then refuses over an untracked `CLAUDE.md` — the route now names what that commit must carry), +1 backstop wording arm, and 2 sibling cells found OUTSIDE this mechanism and left open (gitignored replacing path; `task finalize`'s stamp refresh).

### pinned_surfaces_moved

No frozen schema hash, manifest, pinned `--format json` key, `contract-version`, compose golden or finding code moved. What did move: (1) the installed guide's body digest `jigc-body-blake3`, because QUICKSTART.md and MIGRATING.md bytes changed — computed at build, no golden pins it; (2) one pinned test cell, `setup::setup_writes_compose_embedded_methodology_marker` (iii), which seeded an untracked `packs:` list on an unborn HEAD and expected exit 0 — it now commits the seed first and still pins that the list survives beside the marker; (3) `GIT_SPAN_SITES` gained one row for the new route's `git -C <home> rm --cached` span; (4) internal API: `engine::finalize::setup_dirty_install_finding` takes a `SetupHead`, `cli::setup::install_path_dispositions` returns `Vec<InstallMember>` (path, disposition, writer). Born-HEAD message and route bytes are unchanged.

### notes

COMMIT: dc0d7586 `fix(setup): a file the install would replace refuses on an unborn HEAD too`, on `fix/rc24-tier1` (confirmed before starting and before committing). Tree clean. Not pushed, no branch switched. 14 files, staged by name.

GATE: `tests   passed=4419 failed=0  (over 17 test binaries)` · `GATE: PASS`.

WHAT WAS FIXED: on an unborn HEAD `jigc setup` no longer destroys an untracked file at a path its writer replaces. It refuses before the first write with the existing `setup.dirty-install-path`, names the path, and prints a route that runs with no commit yet. The on-ramp (untracked `CLAUDE.md`, `.gitignore`, `.claude/settings.json`, `.jigc/.gitignore`, foreign hook) still installs at exit 0 with those bytes in the first commit.

RED FIRST: with the new plumbing but the old rule (every untracked path exempt on unborn), 6 new tests failed at exit 0 "adapter installed" — the defect — while the controls stayed green; with the real predicate all 42 tests in the two suites pass.

HOW IT IS BUILT:
- The preserves/replaces fact lives in ONE place: `install_tracked_paths` in crates/cli/src/setup.rs now returns `InstallMember { path, disposition, writer }`. The struct has no default, so a new member does not compile without both facts. The old path-name lookup function is gone.
- Two fences for the next member: cell 22 pins the full table (a new member reddens it); cell 23 (`every_install_member_keeps_its_declared_promise_on_an_unborn_head`) iterates the production table through the real binary — `Preserves` must leave the adopter's mark in the first commit, `Replaces` must refuse with bytes intact — and panics on a member with no plant. A member falsely declared `Preserves` fails there.
- `packs.yaml`: I chose REPLACING for the guard's purpose. Why: the writer round-trips through the YAML value model, a textual comment-preserving writer cannot be total (flow-style root mapping, a marker present with another value), so "preserves" would become a per-file answer needing an oracle shared by writer and guard — new mechanism in a fix pass. It also makes unborn equal born, where a commented `packs.yaml` already refuses. See left_open for the consequence I want confirmed.
- The engine words the refusal per HEAD (`SetupHead::Born | Unborn { home, riding }`) in crates/engine/src/finalize.rs. Born bytes are unchanged.

TESTS ADDED: crates/cli/tests/setup_install_pathspec_guard.rs cells 23–29 (class axis, stamp-oracle control, orphan branch, live and dangling symlink, `--force` naming, on-ramp control, every route arm followed verbatim including the staged cell); crates/cli/tests/setup_failed_first_run.rs arms k–m (re-arm after unstage, its control, a write-span failure on unborn); one engine unit test for the unborn wording. No new suite file, so no group registration.

FOR THE RECORD (the orchestrator's logs — I wrote neither): the `packs.yaml` disposition above, and the three sibling cells in left_open, two of which I drove and none of which I built.

### routes_driven

- Built `target/release/jigc` (1.0.0-rc.24 + this tree) after the passing gate; all cells below in `dev/jigc-rig bare --binary target/release/jigc` roots, one `mktemp -d` repo per cell.
- REFUSAL (unborn, untracked `.jigc/AGENT.md` + commented `packs.yaml` + `CLAUDE.md`): `jigc setup` -> exit 1, `setup.dirty-install-path`, 2 paths named, `rev-list --count --all` = 0, all three files still `??`, all marks on disk. Route printed: "move the file(s) out of those path(s) — `git -C <abs repo> rm --cached -- <path>` first where you had staged one — then re-run `jigc setup`; or commit them in one commit with `CLAUDE.md` — untracked at an install path too: the install merges into such a file today, and `jigc setup` asks about it as well once the repository has a commit — and re-run, so git holds your copy before the install runs over the path; `jigc setup --force` is the single consent, ..."
- ARM 1 as printed — move both files out, then `jigc setup` -> exit 0 in one run; install commit minted; `CLAUDE.md` mark in HEAD; moved files intact.
- ARM 2 as printed — `git add -- .jigc/AGENT.md CLAUDE.md && git commit -q -m "our notes"` -> exit 0; `jigc setup` -> exit 0 in one run; `git log --all -S` finds the adopter's AGENT.md prose; `git status` empty.
- ARM 3 as printed — `jigc setup --force` -> exit 0; advisory `setup.forced-install-path` "over 1 install path(s)" naming `.jigc/AGENT.md` only.
- STAGED cell (refused before this fix, with the dead stash route) — `jigc setup` -> exit 1; printed unstage run verbatim with the path substituted: `git -C <abs repo> rm --cached -- .jigc/AGENT.md` -> exit 0; file moved out; `jigc setup` -> exit 0.
- Why the old route was dead: `git stash -u` on an unborn HEAD -> exit 1, "You do not have the initial commit yet".
- Repro B arm 2b — failed first run (no identity) -> set identity -> `git reset -q` (exit 0) -> append to `.jigc/AGENT.md` -> `jigc setup` -> exit 1 naming exactly `.jigc/AGENT.md`, mark still on disk. Control (reset, no edit): `jigc setup` -> exit 0, status empty.
- Orphan branch + `git rm -r --cached .` + untracked `.jigc/AGENT.md`: `jigc setup` -> exit 1, mark on disk, commit count still 1.
- Untracked symlink `.jigc/AGENT.md -> ../notes/agent.md`: `jigc setup` -> exit 1, target mark intact, link still a link.
- On-ramp control — untracked `CLAUDE.md`, `.gitignore`, `.claude/settings.json`: `jigc setup --format json` -> exit 0, `findings: []`, all three marks in HEAD.
- `jigc setup --format json` on the refusal -> exit 1; envelope keys unchanged (`schema_version: 3`, `key.target: null`, `location: null`).
- Honest bound: one line of my drive script read the first-run exit of repro B through a pipe and printed `first exit=0`; the control cell beside it reads it directly (exit 1) and the integration test asserts exit 1 with `setup.install-commit`.

### docs_changed

- design/validation.md — the `setup.dirty-install-path` row: `packs.yaml` moved from merged-into to replaced; the unborn sentence rewritten as a per-member rule with the rc.24 datum; the unborn route and state clause stated
- design/project-setup.md — Idempotency & irreversibility: S22 holds with no commit yet and after an unstage; `merged, never clobbered` scoped to merged-into files
- design/assistant-adapter.md — The install commit's pathspec: the one per-member exemption
- design/surface-contract.md — The carryover gate, second member: the same exemption and the unborn route
- crates/cli/guides/QUICKSTART.md — 1. `jigc setup`: a new paragraph carrying the no-commit carve-out and what refuses there
- crates/cli/guides/MIGRATING.md — gate 4: the carve-out in one parenthesis
- crates/cli/src/cli.rs — `jigc setup --help` long description
- Doc-comments in crates/cli/src/setup.rs and crates/engine/src/finalize.rs, and the headers of the two suites
- NOT touched: DECISIONS.md and implementation/project-history.md (the orchestrator's shared logs).

### left_open

- DECISION FOR THE HUMAN TO CONFIRM — a pre-seeded `packs.yaml` on an unborn HEAD now refuses, comments or not. This is the direct consequence of declaring the member replacing, which the brief allowed, but it reversed a previously pinned exit 0 (the compose-marker test above). The alternative — a comment-preserving writer (no-op when the marker is already true, textual append verified by re-parse, value-model fallback) plus a per-file oracle for the guard — would let a seeded list install at exit 0 and would also stop a clean re-run regenerating committed comments away. Moving to it later only turns refusals into successes, so it is non-breaking; I did not build it.
- SIBLING, not this mechanism, driven on the release binary: a GITIGNORED file at a replacing member is destroyed at exit 0 at EITHER HEAD. Born repo, `.gitignore` listing `.jigc/AGENT.md`, human prose there -> `jigc setup --format json` exit 0, `findings: []`, mark gone from disk and in no git object. `git_path_ignored` drops the path from the candidates on the reasoning that it cannot be swept into the commit — true for a merged-into file, false for a replaced one. Closing it would make a repository that ignores a jigc install path start refusing, so it is the human's call.
- SIBLING at another door, driven: `jigc task finalize` rewrites an uncommitted hand-edited `.jigc/version` at exit 0 with no finding about it (`fresh` rig, prose in `.jigc/version`, code staged -> finalize exit 0, stamp regenerated, mark in no object). Same replacing writer (`write_version_stamp`), no ownership question at that door. Very low reach; not built.
- OPEN FORK (as instructed, not built, not decided): a committed symlink at a replacing member on a BORN HEAD is written through. My derivation reaches it by route: following the unborn refusal's COMMIT arm over an untracked symlink commits the link, not its target, and the re-run then writes through it. The born route's commit arm has the same hazard today. The unborn route leads with move-the-file-out, which is safe under either arm of the fork.
- Declared and unchanged: on an unborn HEAD an untracked merged-into file rides the first commit under `--no-verify`, past a preserved user hook. No byte is lost.
- Route bound: the commit arm's list of paths that commit must carry is built from the pre-write candidates, which exclude the hook (its home is resolved by the install). An unborn repo with an in-worktree `core.hooksPath`, an untracked foreign `pre-commit` AND a refused path would meet a second, correctly routed refusal after committing. Not driven.
- Pre-existing, seen and not touched: the born message's first sentence says the install would *sweep* work into the commit even at a path it would replace; the commit-time backstop's route over a staged-then-recreated path takes two runs at either HEAD; `design/worked-examples.md` still says this suite has eighteen cells (it had 22, now 29).
- `(R1, F2)` — `--force` over a dirty hook names nothing — untouched.

## c0c4d88c fix(create): a create under a new: true entry never reaches the copy-in

### status

fixed

### gate

dev/gate (full, shared target, foreground-read exit 0) on the committed tree: `tests   passed=4424 failed=0  (over 17 test binaries)` · `GATE: PASS` (fmt ok · clippy ok · build exit=0 · test exit=0 · doctest exit=0). An earlier full gate on the round-1 tree also read `passed=4424 failed=0` / `GATE: PASS`; the committed tree is the one the second gate ran on (no edit between gate start and commit; `git status --porcelain` empty after the commit).

### class

Under an `allows-create` entry carrying `new: true`, a minting door looks at the minted identity's home more than once, and the refusal was decided on the first look only while later looks acted or answered on their own observation. Handed: 2 doors (`doc create`, `doc author`), 1 site (pre-check rank 0 vs `state::create` step 4 copy-in). Found: 3 looks at the home per door (pre-check rank 0 `create_occupied` · pre-check incumbent probe `create_incumbent` · the engine create's probe) and, past the first look, 4 engine/CLI outcomes that were not the gate's answer — all reached by both doors:
1. the engine copy-in (`create` step 4) — the reported instance — CONVERTED;
2. `create_gated`'s staged-copy hand-back + role bind over an occupied home (pin P3, guarded only by rank 0) — CONVERTED (the engine refuses before that branch);
3. the migration blank-seed over an in-location squatter under `new: true` (a write over an occupied home, guarded only by rank 0) — CONVERTED;
4. the pre-check's own second look: an occupant with a different title landing between rank 0 and the incumbent probe was answered `write.title-ignored` ("would be copied in for update"), the code findings-channel.md §4 says cannot arise under `new: true` — driven on the pre-fix tree (exit 1, nothing staged) — CONVERTED: under `new: true` a committed incumbent is `create.already-exists`, ranked above identity-change (P4 kept).
Mechanism after the fix: `occupied_home` is the one predicate; `create_gated` probes once ahead of both branches and that observation decides refusal and copy-in; the refusal is the typed `CreateRefusal::AlreadyExists` (route is the caller's, so a door cannot map the create's error without supplying it); both doors and rank 0 build the finding through one `already_exists_refusal`. id-source axis covered: title · different title onto the same slug · `--slug` · fixed-identity singleton · migration recorded source; entry forms: object (`as:`) and bare.
OUT, with reason: (a) the six edit verbs' copy-on-first-touch (`read_or_copy_in`, 7 call sites, the CLI's one `copy_in` caller) under a `new: true` entry — not a create, a NON-race member, converting it would refuse a previously-succeeding command: the human's call (see left_open); (b) plain-entry incumbent-probe→create window (title dropped, copy-in) and file-lands-after-mint / finalize plan→promote — ruled tier 2 for M57; (c) pub `state::create` (ungated, no entry, no production caller) keeps create-or-update.

### pinned_surfaces_moved

None. No finding code added or renamed (reuses `create.already-exists`, same `(code, target)` key and route builders), no `--format json` key, no `contract-version`, no frozen schema hash or manifest, no compose golden, no pack file, no help text. One behaviour delta, refusal→refusal only: under `new: true`, an occupant with a different title that lands between the pre-check's two looks is now `create.already-exists` instead of `write.title-ignored` (same exit 1, same route text, nothing staged). No command that succeeded sequentially now refuses. Engine API (not a pinned surface, crate is 0.1.0-rc): `jigc_engine::state::create_gated` now returns `Result<CreatedDoc, CreateRefusal>` (new pub enum `CreateRefusal`); six existing engine tests adapted via a test helper, no assertion weakened.

### notes

COMMIT: c0c4d88c `fix(create): a create under a new: true entry never reaches the copy-in` on `fix/rc24-tier1` (verified with `git branch --show-current` before committing; six paths staged by name; tree clean after; nothing pushed, no branch switched).

GATE: `tests   passed=4424 failed=0  (over 17 test binaries)` · `GATE: PASS`, exit 0, full `dev/gate`, never --quick.

WHAT WAS FIXED: the create-only gate was the title pre-check's first look at the home and nothing else — `state::create_gated` took the gate entry and never read `new`, so its own probe ~1 ms later copied in whatever had landed. Now `create_gated` probes the home once (`occupied_home`), ahead of the staged-copy hand-back, the copy-in, the migration blank-seed and the role bind, and that one observation decides both outcomes; under `new: true` it returns the typed `CreateRefusal::AlreadyExists { address }`, which both doors (`run_create`, `run_author`) answer through the same `already_exists_refusal` builder rank 0 uses (same code, key, route). Applied as decided (the one-probe fix); the pre-check's rank 0 stays as the ranked early refusal.

BEYOND THE HANDED INSTANCE (same class, same commit): the pre-check's incumbent probe is a second look at the same home; under `new: true` it now answers `create.already-exists` too (6-line guard in `title_pre_check`, above rank 2). Before it, a different-titled arrival between the two pre-check looks got `write.title-ignored`, which findings-channel.md §4 says cannot arise in such a task. This is a refusal→refusal change only.

TESTS: (1) engine unit `create_gated_under_a_new_entry_refuses_an_occupied_home_and_copies_nothing_in` — 6 arms × 2 entry forms, working area held byte-identical; control `create_gated_under_a_plain_entry_still_copies_in_and_a_free_home_still_mints`. (2) `crates/cli/tests/create_only_gate.rs` (already registered in groups/g_doc.rs; no new suite file): `a_home_occupied_after_the_create_only_ask_is_still_refused_and_never_copied_in` — 2 door states × 5 arms (both doors × title / different title / `--slug`), each refusal's route run as emitted and `task finalize` landing beside an untouched occupant; plus two controls that prove the harness is not vacuous (`the_window_harness_acts_after_the_pre_checks_ask`, `without_new_the_same_plants_get_create_or_updates_answers`). The harness is deterministic, not timed: a FIFO stands where a file the door reads between two looks would be (`roles.json`; `source-path`, second read for `doc author`), so the plant lands with the door blocked in that read. Both halves proven red in isolation (see routes_driven).

WHAT I DID NOT VERIFY: the FIFO harness on Linux (ran on macOS only); the `jigc-feedback` doctype at the binary. The harness's positions depend on the doors' read order, which the two controls check on every run rather than assume.

FOR THE ORCHESTRATOR: `jigc-engine`'s public `create_gated` signature changed (new pub enum `CreateRefusal`) — a breaking API change in the 0.1.0-rc engine crate that release-plz's semver check may surface on the release PR. One non-race sibling is the human's call (edit verbs by address inside a report task — first left_open item); I recorded the fact in the design doc as not ruled on and touched neither DECISIONS.md, project-history.md nor decisions-pending.md.

### routes_driven

- Binary: target/release/jigc built from the final tree (jigc 1.0.0-rc.24, sha256 prefix 680dc59a2d923685), rig `dev/jigc-rig fresh --binary target/release/jigc`, shipped `report-inconsistency` (entry `new: true`), two-step eval, stdout only.
- SEQUENTIAL (pre-check rank 0): `jigc doc create inconsistency --title "Seed" --task re-file-the-seed` → exit 1 create.already-exists at inconsistency:seed, route: "choose a distinct `--title`, or keep this one and pass `--slug <slug>` to mint beside the existing doc: `jigc doc create inconsistency --title <title> --slug <slug> --task re-file-the-seed`"; nothing staged, no roles.json.
- WINDOW, door `doc create` (home free at the pre-check, victim planted with the door stopped at its first roles.json read, i.e. before the create's probe): `jigc doc create inconsistency --title "Race" --task file-the-race-finding` → exit 1 create.already-exists at inconsistency:race (pre-fix: exit 0 copy-in); nothing staged, roles.json absent, RACE-VICTIM-MARKER at home 1. Emitted route run verbatim with its two fills: `jigc doc create inconsistency --title 'Race' --slug race-reported --task file-the-race-finding` → exit 0 `inconsistency:race-reported`; set-field/set-slot 0; `jigc task validate` 0; `jigc task finalize file-the-race-finding` → exit 0, 1 file committed; AFTER victim marker 1 → 1, reporter marker 1 in race-reported.md.
- WINDOW, door `doc author` (same plant): `jigc doc author inconsistency --from-file <payload> --task author-the-race-finding` → exit 1 create.already-exists at inconsistency:authored, route "set the payload's `title:` to a distinct title (its slug becomes the doc id) and re-run the same `jigc doc author inconsistency --from-file <payload> --task author-the-race-finding`"; nothing staged, victim marker 1, payload marker 0 at home. Route run as printed (payload title changed to `Authored Report`, same command) → exit 0 `inconsistency:authored-report`; `task finalize` exit 0; victim marker 1 → 1.
- WINDOW inside the pre-check (victim with a DIFFERENT title planted at the pre-check's `source-path` read, between rank 0 and the incumbent probe): `jigc doc create inconsistency --title "Mid  Check!" --task file-the-mid-check-finding` → exit 1 create.already-exists at inconsistency:mid-check (pre-fix tree: exit 1 write.title-ignored); route run with fills `jigc doc create inconsistency --title 'Mid  Check!' --slug mid-check-reported --task file-the-mid-check-finding` → exit 0; `task finalize` exit 0; victim marker 1 → 1.
- REAL RACE (python toggler renaming a conformant file into/out of the home, a fresh task per call, N=60 per door): final tree → create: 39 refused · 0 copied-in · 21 fresh; author: 35 refused · 0 copied-in · 25 fresh. Before-control with the SAME harness on the installed registry rc.24 (~/.local/bin/jigc): create 30 refused · 9 copied in at exit 0 · 21 fresh; author 26 · 3 · 31.
- RED proofs (temporary neutralisation, restored byte-identical, verified by cmp): engine refusal off → `state::tests::create_gated_under_a_new_entry_refuses_an_occupied_home_and_copies_nothing_in` FAILED (got Ok existed:true) and `create_only_gate::a_home_occupied_after_the_create_only_ask_is_still_refused_and_never_copied_in` FAILED at BetweenThePreCheckAndTheCreate (exit 0) while the other 24 suite tests passed; pre-check guard off → the same suite test FAILED at BetweenThePreChecksLooks with write.title-ignored.

### docs_changed

- design/findings-channel.md — §4: new paragraph 'The refusal is the create's own — one probe decides both outcomes' (incl. what the gate does not claim: edit verbs by address, a file landing after the mint, pointer to decisions-pending M57); Precedence sentence extended (every look of the pre-check answers create.already-exists under new: true)
- design/write-commands.md — The create-gate, Create-only paragraph: 'The refusal is the create's own' sentence, cross-referencing findings-channel §4
- design/worked-examples.md — flow 57 'What it adds over create_only_gate': the suite now also proves the window
- No `--help` text changed: `doc create --help` / `doc author --help` already state 'refused before anything is copied in'; it is now true by construction. No guide sentence states the rule.
- Rustdoc/comments in crates/engine/src/state.rs (create, create_gated, occupied_home, stage_minted, CreateRefusal, create_occupied, already_exists_finding) and crates/cli/src/doc.rs (title_pre_check, already_exists_refusal, create_refused)

### left_open

- HUMAN'S CALL — NON-race member, driven on the fixed binary: an EDIT verb addressed at an existing finding inside a report task (entry `new: true`) still copies it in. `jigc doc set-slot inconsistency:seed#description --from-file - --task <report task>` → exit 0 '(copied in for update …)', role `inconsistency` bound to inconsistency:seed, `task finalize` exit 0, the earlier finding's description replaced (git still holds the old bytes: SEED-MARKER in 2 commits). Design §4 Scope takes this copy-in as its premise (P3) and 'by either path' means the two create doors, so it is not a breach by the letter; refusing it would make a previously-succeeding command refuse, so I did not pick. I stated the fact in findings-channel.md §4 and flagged it as not ruled on. Owed if the human wants it tracked: a decisions-pending.md entry.
- LEAD for the (R3, F7)/O-2 fixer — NON-race, driven on the fixed binary: same edit door over an UNTRACKED conformant file at a home inside a report task: `set-slot inconsistency:squat#description` exit 0, `task validate` exit 0 (advisories file-state.staged-copy + baseline-adopt), `task finalize` exit 0; SQUAT-MARKER 1 → 0, nowhere under the repo, in 0 commits. This is the 'untracked file read as committed / no baseline' loss class, reachable under a `new: true` entry without any race; not this finding's mechanism, not touched.
- RULED OUT (M57, tier 2) and unchanged, driven once for the record: under a PLAIN entry (`park-idea`) an occupant with a different title landing between the incumbent probe and the create is copied in at exit 0 with the supplied title dropped (`"existed": true`, staged H1 is the occupant's). The suite does not pin this cell.
- RULED OUT (M57) and not driven by me: a file landing at the home after the create has minted (met by finalize.promote-clobber per the verifier's late-landing row) and `task finalize`'s plan→promote window.
- Not driven: `jigc-feedback` doctype at the binary (same entry shape and code path; the suite drives `idea` on a `new: true` shadow, the rig drove `inconsistency` on the shipped workflow); a case-sensitive filesystem; Linux (the FIFO harness uses the `mkfifo` binary and blocking FIFO opens, POSIX on both, but it ran only on macOS here — CI will be its first Linux run).
- Wording nit seen, not mine, not touched: the doc-only finalize's left-out narration calls an UNTRACKED file at a home 'a staged path stays staged' (`left-out … docs/inconsistencies/race.md`).
- OWED in the orchestrator's logs (not touched): DECISIONS.md — one entry for the rc.24 fix pass, (R6, K-1): the one-probe fix as built (the engine refuses on its own single probe; typed `CreateRefusal::AlreadyExists`; the pre-check's incumbent probe answers as the gate under `new: true` — a fourth member nobody had listed); and the review record's (R6, K-1) entry / UNPINNED note can be marked pinned by `state::tests::create_gated_under_a_new_entry_refuses_an_occupied_home_and_copies_nothing_in` and `create_only_gate::a_home_occupied_after_the_create_only_ask_is_still_refused_and_never_copied_in`. implementation/project-history.md: nothing owed by this commit beyond the pass's own fold-back.

## 3f3a724b fix(milestone): a created doc never promotes over a file at its home at the milestone boundary

### status

fixed

### gate

dev/gate (full, foreground, on the final tree, branch fix/rc24-tier1): fmt ok · clippy ok · build ok · test ok (469s) · doctest ok — `tests   passed=4440 failed=0  (over 17 test binaries)` — `GATE: PASS` — exit 0 read directly, not through a pipe. An earlier full run on the pre-reword tree also read `passed=4440 failed=0`, `GATE: PASS`.

### class

A committing door's promote planner that runs the shared promote sweep without the clobber guard. Handed: one site (`plan_milestone_finalize`) reached through a join suffix. Real: `plan_promotions` has THREE consumers, not two — (1) `plan_finalize`, guarded since M25, refactored onto the shared predicate, behaviour and bytes unchanged; (2) `plan_milestone_finalize`, CONVERTED — one planner serving both `finalize.fan-out.squash` modes, every doctype with a home, suffixed and unsuffixed landings, committed / untracked-conformant / untracked-foreign / staged-uncommitted occupants; (3) `decide_base_repin`, OUT — it reads promote destinations for a footprint and promotes nothing. Sibling writers checked: the milestone-record mint is already guarded (driven: `milestone.record-exists`, exit 1, occupant bytes intact); `jigc rename`, `jigc doc rename`, both create doors and relocate were already guarded per the verification. NOT converted, with reason: the join's suffix assigner (`fold_areas`/`suffix_resolve`) still mints onto an occupied id and the join preview still prints a plain suffix line — making it store-aware reverses storage.md rule 4's stated purity, so it is flagged open rather than picked. One cell of the matrix does not exist and is named in the test: a committed occupant at an unsuffixed home (a create over a committed doc copies it in, so the doc is never `created`; driven — a worktree sub-task's create DOES probe the main checkout's home).

### pinned_surfaces_moved

None. No frozen schema hash, no `schema-manifest.yaml`, no pinned `--format json` key, no `contract-version`, no compose golden, no pack file. The finding code is the existing `finalize.promote-clobber` with its existing `(code, destination path)` key and the door's existing exit-3 findings envelope. `AMBUSH_CONTRACTS`, `GATE_COVERAGE`, `CONSTRAINT_REQUIRED_TOKENS` and the `FinalizeCode` registry are untouched (the code is still minted in `clobber_finding`). Engine-internal signatures changed: `plan_milestone_finalize` gained `repo_root` and `origins`; `MaterializeOutcome` gained `origins` (in process only, never written into `merged/`). Two help texts gained sentences; the full gate, which includes `help_truth.rs`, is green.

### notes

COMMIT: 3f3a724b `fix(milestone): a created doc never promotes over a file at its home at the milestone boundary`, on `fix/rc24-tier1`, 11 paths staged by name, tree clean afterwards, nothing pushed.

WHAT WAS FIXED: `plan_milestone_finalize` now runs the promote clobber guard after the shared promote sweep, over the same predicate as `plan_finalize`. A sub-task's `created` doc whose destination in the main checkout already holds a file blocks with `finalize.promote-clobber` at exit 3, the record flip is restored, nothing is committed. `materialize` hands each merged body's origin (provenance, the address it is staged under in its sub-area, its collision group from its position on) to the planner in process, because `merged/` has no provenance manifest and a suffixed address is one no sub-area recorded.

RED FIRST: the new suite ran against the unfixed tree — 10 of 11 failed, each at the exit-3 assertion with the door printing a landed commit at exit 0 (the defect itself); the one that passed was the free-suffix control. One control had a wrong premise in the test and was corrected before the fix. Also reproduced by hand on the pre-fix release binary: both occupants' markers 1 -> 0 at exit 0.

THE ROUTE, and one thing the plan's version got wrong: a single `jigc doc rename` of the blocked doc does not land the milestone when the collision group has a later member — driven on rc.24, renaming bravo's doc moved charlie's onto the same occupied `-2`. So the finding names the rename for every sub-task of the group from the blocked suffix on, and the milestone lands on the first re-run (pinned by `a_blocked_suffix_names_every_later_sub_task_of_its_group_so_one_pass_lands`). The route prints no adopt/migrate command; it says the occupant waits until the milestone has landed and why.

EMITTED ROUTE AS PRINTED (final binary): `nothing was committed and every sub-task's staged work is intact: give the doc a title that slugs to an id nothing holds (`jigc doc rename inconsistency:stage --to "<title>" --task charlie-reports-the-stage`), then re-run `jigc milestone finalize stage-reports`. The file at `docs/inconsistencies/stage-3.md` is left exactly as it is — deal with it after the milestone has landed, not before: a commit made first moves `HEAD` off this milestone's base and blocks it on `finalize.base-mismatch``. Exits when run: rename 0, rename 0, finalize 0.

TESTS: crates/cli/tests/milestone_promote_guards.rs (new, 11 tests, registered in crates/cli/tests/groups/g_milestone.rs); crates/engine/src/finalize.rs unit tests `milestone_plan_blocks_a_created_doc_over_an_occupied_home`, `milestone_clobber_route_names_every_sub_task_from_the_blocked_suffix_on`, `milestone_plan_lets_an_edited_from_base_doc_re_promote`, `milestone_plan_passes_a_created_doc_at_a_free_home`; crates/engine/src/milestone.rs `materialize_reports_each_bodys_origin`.

PROCESS NOTE: one of my turns ended in text while the first gate was still running and the harness prompted for the structured result; I did not return then — the tree was uncommitted — and finished the gate and the commit first. The route wording was changed after the first green gate (a lowercase word after a full stop), so the gate was re-run in full on the final tree; the numbers above are that run.

FILES: crates/engine/src/finalize.rs · crates/engine/src/milestone.rs · crates/cli/src/milestone.rs · crates/cli/tests/milestone_promote_guards.rs · design/finalize.md · design/storage.md

### routes_driven

- Built `target/release/jigc` from the final tree; rig `dev/jigc-rig fresh --binary target/release/jigc`, two-step eval, toplevel checked outside the working repo. State: committed `stage-2.md`, untracked `stage-3.md`, three `report-inconsistency` sub-tasks in provisioned worktrees each minting `inconsistency:stage`.
- `jigc milestone finalize stage-reports` -> exit 3, two `blocking · finalize.promote-clobber` (at `docs/inconsistencies/stage-2.md` and `stage-3.md`), HEAD unchanged, both occupants' shas unchanged, record still `active`.
- Emitted route span, `"<title>"` filled: `jigc doc rename inconsistency:stage --to "Stage as bravo saw it" --task bravo-reports-the-stage` -> exit 0
- Emitted route span, `"<title>"` filled: `jigc doc rename inconsistency:stage --to "Stage as charlie saw it" --task charlie-reports-the-stage` -> exit 0
- Emitted route span verbatim: `jigc milestone finalize stage-reports` -> exit 0, `finalized c7030db`, all three sub-tasks' docs in HEAD (`git grep` for each reporter's prose), both occupants byte-identical (shas 68fc73dbde94 / c28aca8c2015 before and after), `stage-3.md` still untracked.
- Unsuffixed case (one sub-task, untracked file appearing at `docs/inconsistencies/drift.md` after the create), `--format json`: exit 3, envelope `schema_version` 3, one finding keyed `(finalize.promote-clobber, docs/inconsistencies/drift.md)`; emitted `jigc doc rename inconsistency:drift --to "<title>" --task alpha-reports-the-stage` (title filled) -> exit 0; `jigc milestone finalize drift-reports` -> exit 0; occupant marker 1 -> 1.
- The walk the route warns against, driven as a control: `git add` + `git commit` of the occupant before the boundary, then `jigc milestone finalize` -> exit 3 `finalize.base-mismatch` — the warning is true.
- `finalize.fan-out.squash = false` on the release binary: same two refusals, exit 3, nothing committed.
- In the suite (`crates/cli/tests/milestone_promote_guards.rs`, registered in `g_milestone`): every refused cell extracts the emitted `jigc doc rename` and `jigc milestone finalize` spans, runs them through a real `sh` word split, and asserts exit 0, every sub-task's doc in HEAD and the occupant byte-identical — 14 landing x occupant x squash cells, 4 doctype cells (`adr`, `inconsistency`), the three-reporter repro, the one-pass group case, and a provisioned-worktree case with staged code.

### docs_changed

- design/finalize.md — 4. Promote: the clobber guard stated for both committing doors, decided at the boundary, with the milestone route's three rules; `fan-out` finalize step 3 points at it
- design/storage.md — Placement: the 'guard is inherited' sentence now true at both doors; The by-task-id join: the suffix does not consult the store, a landing ON a doc is refused at the boundary; Open questions: a store-aware join suffix, marked unsettled
- design/auto-migration.md — Honest bounds: the guard's reachable set gains the join suffix landing on an occupied id
- design/findings-channel.md — section 4: `new: true` suffixing lands beside an existing doc, never on one
- design/write-commands.md — Task origination, the fan-out collision bullet: pointer to the boundary refusal
- `jigc milestone finalize --help` and `jigc milestone join --help` (doc-comments in crates/cli/src/milestone.rs): the refusal, its code and its route; the join says its suffix does not look at the store
- crates/cli/tests/git_span_aim.rs — the `clobber_finding` registry row's reason, kept true now that the finding has a third arm

### left_open

- HUMAN'S CALL — a store-aware join suffix (the join skips an occupied `<slug>-N`). Not built: it reverses storage.md rule 4's stated purity. The boundary refusal is complete without it. Flagged in design/storage.md -> Open questions as unsettled; a `decisions-pending.md` entry is owed if it is deferred.
- HUMAN'S CALL — whether `jigc milestone join` should preview the refusal. Today it exits 0 and prints a plain `suffixed -2 on collision` line over an occupied id; making it report or exit non-zero is a new refusal at a previously-succeeding command, beyond what was decided. Parity note: `task validate` does not preview `promote-clobber` either (`GATE_COVERAGE`: promotion is `FinalizeOnly`). The join's help now states the bound.
- LEAD for the (R3, F7) fixer, driven on this commit's binary: a COMMITTED occupant whose file was deleted from the worktree (unstaged), with NO file-state baseline (`.jigc/state/file-state.json` absent, a clone's state) — `milestone finalize` exits 0 and the suffixed doc replaces the committed doc under its id (`stage-2.md | 6 +++---`). With a baseline the same state blocks on `reconciliation.rename`. The clobber guard reads the worktree, like every other occupancy probe in jigc, so it does not see it; this is the baseline-less reconcile hole. Extending the guard to read the index would also change `task finalize` and collide with `jigc unmanage`'s sanctioned exit — not picked.
- For the (R6, D-7) fixer: the entry shapes `is_file()` does not see (a directory, a dangling symlink at a promote home) are untouched here. Both doors now ask one predicate, `home_occupied` in crates/engine/src/finalize.rs, so the no-follow change is one site.
- HUMAN'S CALL — the fan-out step prose (a compose golden) does not state the milestone boundary's `promote-clobber` refusal; the stated-at fence is satisfied by the `migration-finalize` declarers and the two helps now state it. Adding it to the step moves a golden.
- Not driven: two `created` instances of a placement/singleton doctype in one fan-out; a promotion in `merged/docs/` with no join origin (possible only if a non-jigc writer plants one between materialize and the plan) is treated as not-created, the task arm's 'unrecorded' rule; the plan-to-promote window against a non-jigc writer.
- OWED in the orchestrator's logs (not touched): a DECISIONS.md line for (R6, D-1) — the milestone planner runs the task door's clobber guard, decided at the boundary, route = in-task rename of every group member from the blocked suffix on, never adopt-first; and the project-history.md fix-pass span.

## 9465f9b6 fix(finalize): a managed doc lands as a regular file at its home, never through a link

### status

fixed

### gate

dev/gate (full, shared target, foreground-read exit 0) on the exact tree committed: fmt exit=0 · clippy exit=0 · build exit=0 · test exit=0 · doctest exit=0 · `tests   passed=4461 failed=0  (over 17 test binaries)` · `GATE: PASS`. No edit was made between the gate's start and the commit. An earlier gate run was killed by me (exit 144) because I edited a route string after starting it; it is not the evidence.

### class

Mechanism: an occupancy probe of a managed doc's home that FOLLOWS links (`is_file()` / `exists()`), guarding a write that follows them too, so an entry that is not a regular file is read as free (dangling link) or as the doc (live link) and written through. Handed: 3 probe sites + 1 sink on one door pair. Found: 9 members across 4 doors; 8 converted, 1 out, plus 3 sibling writers dispositioned.

CONVERTED (all on one predicate, `engine::store::home_entry`, symlink_metadata, no-follow):
1. `engine::state::occupied_home` -> create-only refusal (`create_gated`, `new: true`): any entry refuses, `create.already-exists`, message names the shape.
2. `engine::state::create_occupied` (the CLI pre-check's ranked ask) — same.
3. `engine::finalize` planner, task door (`plan_clobber_guard`): ANY promotion (created, edited-from-base, unrecorded; in-place migration carve-out included) over a non-regular entry -> `finalize.promote-clobber`, same key. Covers `task finalize`, `--dry-run`, and the migration landing (refuses before the exit-4 review hold).
4. Same planner arm at the milestone boundary (`plan_milestone_clobber_guard`), both squash models, incl. bodies with no join origin.
5. The promote sink `cli::task::promote` (one function, shared by every stage arm): re-reads every destination before the first copy (whole transaction refuses, nothing written), and the copy is now `copy_regular` — `O_NOFOLLOW|O_NONBLOCK` open, fstat on the handle, in-place rewrite, source mode kept.
6. Rollback path: `rollback_promotions`' "still there?" probe was `dest.exists()`; a dangling link appearing at a created destination was replaced by the HEAD blob. Now asked of the entry. (Test proved red under the old probe.)
7. In-task re-slug guard `cli::doc::free_destination`: a rename onto an id whose home is a dangling link/directory acked at exit 0 over an unfinalizable state; now `write.already-present`.
8. NOT IN THE BRIEF — `jigc milestone create` (`guard_record_free` + `materialize_and_commit_record`): driven on the pre-fix tree, a dangling link at the record home gave exit 0, a record commit holding the LINK, the record body in an untracked file. Same mechanism at the one other door that mints a managed doc at its home. Now refuses under the existing `milestone.record-exists` and writes with `create_new` (O_EXCL).

OUT, with reason:
9. `engine::state::bound_instance_present` and `create_incumbent`/plain-entry copy-in: deliberately still read THROUGH a live link (truth table unchanged) — reading is not the harm, and refusing the read turns an edit into a blank mint. Routed through the same observation so they cannot disagree with the copy-in.

LIVE-LINK EDIT PATH (asked for): driven pre-fix = edit written into the link's target, commit rejected as "a hook's complaint", exit 1, undeliverable. Now refused at the planner by name; exit is the regular file put where the link was; driven end to end.

Special files (FIFO/device node at a home): converted at planner and sink (unit tests plant a FIFO) but NOT reachable at a committing door — see left_open.

### pinned_surfaces_moved

None. No new finding code (reused finalize.promote-clobber, create.already-exists, write.already-present, milestone.record-exists, each with unchanged `(code, target)` key), no new store/knob/doctype, no frozen schema hash, no pinned `--format json` key, no contract-version, no compose golden. Engine pub API changed shape (pre-1.0, `engine` is not a contract): `state::create_occupied` now returns `Option<OccupiedIdentity>`; `CreateRefusal::AlreadyExists` gained `foreign`; `state::already_exists_finding` gained a `foreign` argument; added `store::{HomeEntry, ForeignEntry, home_entry}`, `state::foreign_home_entry`, `finalize::promote_sink_refusal`, `milestone::record_home_taken_finding`. One existing route string gained a variant: the create-only route says 'beside the link that is there' instead of 'beside the existing doc' when the occupant is not a doc; the doc-occupant wording is byte-unchanged.

### notes

(1) COMMIT: 9465f9b6 `fix(finalize): a managed doc lands as a regular file at its home, never through a link` on fix/rc24-tier1 (confirmed by `git branch --show-current` before committing; started from a clean tree; staged 18 paths by name; tree clean after; nothing pushed, no branch switched).

(2) GATE: dev/gate full — `tests   passed=4461 failed=0  (over 17 test binaries)`, `GATE: PASS`, exit 0, on the committed tree.

(3) WHAT WAS FIXED: (R6, D-7). Every decision about a managed doc's home now reads the home's own directory entry without following a link, and an entry that is not a regular file is refused and named as an occupant — at the create-only gate, the planner on both committing doors, the promote sink (which also writes with O_NOFOLLOW), the in-task re-slug guard, the rollback's occupancy probe, and `jigc milestone create`.

RED FIRST: the new suite ran 7 of 8 red on the unfixed tree, each for the defect (doors exiting 0, 1 or the exit-4 review hold where they must refuse); the base cell and the live-link edit were also reproduced by hand on the release binary before any change.

THINGS THE ORCHESTRATOR SHOULD KNOW BEFORE RESUMING:
- I went one door past the brief. `jigc milestone create` had the identical defect (driven: exit 0, link committed, record in an untracked file). I converted it in the same commit because it is the same mechanism and the decided contract's wording ('a link … at a home is … refused and named like any other occupant') covers it, reusing `milestone.record-exists`. If you read that as a new refusal beyond the decision, it is isolated: `guard_record_free` and the `create_new` write in crates/cli/src/milestone.rs, `record_home_taken_finding` in crates/engine/src/milestone.rs, one test, one paragraph in team-ready-state.md.
- The same reasoning applied to the in-task re-slug guard (`write.already-present`): its own doc comment claims parity with the committing door, which my planner change would otherwise have made false.
- I did NOT extend refusals to plain creates, copy-on-first-touch, or `jigc rename`; those are listed as the human's.
- Two existing unit tests (`mid_promote_failure_restores_…`, `rollback_restores_a_tracked_destinations_…` in crates/cli/src/task.rs) seeded their mid-promote failure with a directory at the second destination. The sink now refuses that before the first copy, which would have left both tests passing vacuously, so I re-seeded them with a missing staged source.
- The FIFO cell hung my first test run for 10 minutes (the validate read seam above); I removed it from the door-level axis and say so in the suite header rather than leaving a cell that cannot assert anything.

TESTS ADDED: crates/cli/tests/promote_destination_shape.rs (new, registered in g_finalize; 9 tests: entry-shape × exit axis at task finalize incl. --dry-run, out-of-repository, tracked link with/without staged code, live-link edit, placement singleton, migration landing with and without --approve, milestone boundary × both squash models × both exits, milestone create, controls); create_only_gate::an_entry_that_is_not_a_regular_file_is_an_occupied_home (5 shapes × both minting doors); doc_rename_in_task::the_reslug_destination_guard_refuses_a_home_that_is_not_a_regular_file; engine unit axes in finalize.rs (5 shapes × 3 provenances at the task planner, × 4 landings at the milestone planner, fixed identity, migration incl. in-place, sink identity) and state.rs (probe and gated-create axes); cli::task unit tests for the sink's refuse-before-anything-is-written, `copy_regular`, and the rollback probe.

Scratch drive scripts and logs are under <scratch>/d7/ (routes.sh, sib.sh, base.sh); they are not committed.

### routes_driven

- All on target/release/jigc built from 9465f9b6, in `dev/jigc-rig fresh --binary` rigs, each emitted span run as printed (only `<title>`/`<slug>` filled).
- task finalize (created adr, dangling link at home): `jigc task finalize record-the-cache-decision --dry-run` -> exit 3 · `jigc task finalize record-the-cache-decision` -> exit 3, finalize.promote-clobber; emitted `jigc doc rename adr:cache-strategy --to "<title>" --task record-the-cache-decision` (title filled) -> exit 0; emitted `jigc task finalize record-the-cache-decision` -> exit 0, HEAD holds 100644 docs/decisions/cache-strategy-revised.md, the link untouched and in no commit.
- task finalize (live-link edit path, edited-from-base): `jigc task finalize revise-the-decision` -> exit 3; route's exit taken by hand (copy of the target in the link's place); emitted `jigc task finalize revise-the-decision` -> exit 0, 100644 blob carries the edit, old target unchanged.
- task finalize (placement singleton, VISION.md a dangling link): -> exit 3, route offers no rename; link moved out; emitted `jigc task finalize form-the-project-vision` -> exit 0, 100644 VISION.md.
- milestone finalize: `jigc milestone finalize record-decisions` -> exit 3; emitted `jigc doc rename adr:cache-strategy --to "<title>" --task alpha-records-a-decision` -> exit 0; emitted `jigc milestone finalize record-decisions` -> exit 0, both sub-tasks' docs landed as 100644.
- create-only gate: `jigc doc create inconsistency --title "Dangling home" --task edge-finalize-dangling-home` -> exit 1, create.already-exists; emitted `jigc doc create inconsistency --title <title> --slug <slug> --task edge-finalize-dangling-home` (filled) -> exit 0.
- in-task rename: `jigc doc rename adr:pick-a-broker --to "Taken Home" --task pick-the-broker` -> exit 1, write.already-present; emitted `jigc doc rename adr:pick-a-broker --to 'Taken Home' --slug <other-slug>` (filled, plus --task) -> exit 0.
- milestone create: `jigc milestone create "ship it"` -> exit 1, milestone.record-exists, nothing written, no area minted; link moved out; same command -> exit 0, 100644 docs/milestone-records/ship-it.md.
- Migration landing (`task finalize` and `--approve` both exit 3; emitted `jigc task finalize <id> --approve` lands after the entry is moved out) and both squash models were driven as printed by the suite on the debug binary, not separately on release.

### docs_changed

- design/finalize.md — §4 Promote: the contract, where it is asked (planner both doors, sink, create gate, re-slug), the per-provenance exits, the live-link edit path, the rollback, four declared bounds
- design/findings-channel.md — §4 Scope: 'already exists' is any directory entry, read without following a link
- design/write-commands.md — `jigc doc rename` Bounds; the create-gate's `new: true` paragraph
- design/team-ready-state.md — The lifecycle, 'the id belongs to the record': the record home's entry, exclusive create
- design/auto-migration.md — Honest bounds: the shape arm is not narrowed by provenance or the same-path carve-out
- design/command-output-contract.md — the `promote-clobber` subject row
- --help: `jigc task finalize --dry-run` (crates/cli/src/task.rs), `jigc milestone finalize` (crates/cli/src/milestone.rs), `jigc doc create` (crates/cli/src/doc.rs); the FinalizeCode subject_note in crates/cli/src/render.rs
- Two stale doc comments corrected (crates/engine/src/state.rs MigrationSource, crates/cli/src/task.rs ValidatedRetirement): both claimed the engine contains no `symlink_metadata`, false since M53
- NOT touched, owed by the orchestrator: DECISIONS.md and implementation/project-history.md

### left_open

- HUMAN'S CALL — `jigc rename` of a committed doc whose home is a LIVE link: driven on the fixed tree, exit 0; `git mv` moves the link, the retitle is written through it into the target, which is left modified and uncommitted (`M elsewhere/cache.md`), and the rename commit holds only the link move. A write-through at a door the decision does not name (crates/cli/src/rename.rs, the `std::fs::write` sites); refusing it is a new refusal of a previously-succeeding command. Not changed.
- `jigc rename` ONTO an id whose home is a dangling link: driven, exit 1, `git mv … fatal: destination exists`, rolled back — git's own lstat holds, but the refusal is code-less and route-less where a regular-file occupant gets `write.already-present`. Its probe is still `new_abs.is_file()`. Cheap follow-up, not done (different door, different sink).
- HUMAN'S CALL — a plain (non-`new: true`) create over a body-less entry (dangling link, directory) still mints fresh at exit 0 and is refused only at finalize. No existing create-side code says this honestly (`create.already-exists`' sentence is about `new: true`); refusing at the create needs a code or a reworded one.
- HUMAN'S CALL — copy-on-first-touch (set-slot/set-field/…) of a doc whose home is a live link still acks 'copied in for update … re-promoted at finalize'; that finalize now refuses until the link is replaced by the regular file. An earlier refusal at the first write would be a new refusal on six write verbs.
- Milestone record's LATER per-op writers (add-task append, status flips: `std::fs::write` on the record path in crates/cli/src/milestone.rs) over a record somebody replaced with a link: not driven, not converted. Reasoned: write-through into the target, then a record commit with nothing in it, exit 1 and rollback. Only `milestone create` was converted.
- READ SEAM, separate fix — a FIFO (or other blocking special file) anywhere under a doctype's directory parks `jigc task finalize`/`task validate` forever in `engine::target_surface::enumerate_target_surface` (`fs::read`), before any planner runs. Driven (sampled stack). So the special-file cell of this class is pinned at planner and sink by unit tests only, and is declared out in the suite's header.
- READ SEAM, pre-existing (the record's L-3) — `jigc doc list` answers a code-less exit 1 over one unreadable entry (e.g. an untracked dangling link at a home); `jigc doc show` serves a doc through a live link. Not touched.
- External-writer window (ruled tier 2, M57): between the sink's re-read and its copy, `PreImage::capture` still reads through a link. Nothing is written (the open fails on the link) and the rollback's pre==post rule leaves it alone, but the capture's follow semantics are unchanged for all five rollback doors.
- A symlinked PARENT directory of a home is followed as before (declared bound in finalize.md).
- corpus migration (`migrate-corpus`) rewrites by temp+rename, so it replaces a link with a regular file rather than writing through it — a different outcome (takes the user's link), read from code, not driven.
- Not driven on Linux: every drive and the gate ran on macOS. `O_NOFOLLOW`/`O_NONBLOCK`/`O_EXCL` are POSIX and the tests are `#[cfg(unix)]`, but CI is the first Linux run.
- Edge, not reached by any shipped workflow: under a `new: true` entry on a fixed-identity doctype whose home is a link, the route still says 'changing the existing one is the work of a workflow…', which names a doc that is not there.
- OWED IN THE SHARED LOGS (orchestrator): a DECISIONS.md line for the contract and for folding `milestone create` into this fix; the project-history fold-back; and the review README's `(R6, D-7)` entry, whose 'not driven' cells (milestone finalize, migrate landing, tracked link, live-link edit) are now driven.

## eb18e5fe fix(reconcile): a task records what it copies in, and a staged doc with no record is decided by its base pin

### status

fixed

### gate

dev/gate (full, shared target, run on the exact tree committed): fmt ok · clippy ok · build ok · test ok (481s) · doctest ok — `tests   passed=4481 failed=0  (over 17 test binaries)` — `GATE: PASS`. An earlier full run of the same tree was red on clippy only (`enum_variant_names` in the restaged contrast suite, tests 4481/0); the variants were renamed and the whole gate re-run before the commit.

### class

A committing door replaces on-disk bytes of a managed doc that the task never compared against, because nothing remembers what the task copied in. Axis as derived: (a) copy-in sites — 2 production sites behind 8 `doc` write leaves (6 edit leaves through `read_or_copy_in`, with `set-field` on both arms = 7 call sites; `doc create` and `doc author` through the engine's `create_gated` → `stage_minted`), plus the public but production-uncalled `engine::state::create`; all converted through one function, `engine::file_state::read_for_copy_in`. (b) sweep doors — `task finalize`, its two previews (`task validate`, `finalize --dry-run`), `start --task` orientation and `milestone finalize` all share `validate_task` → `reconcile_committed`; all converted by the one UNKNOWN-arm backstop. (c) ways the key is absent — clone, `jigc unmanage` before or after the block, deleted cache, untracked doc, older-binary copy-in, non-conformant at copy-in, second task over an unrecorded first: all covered by record or backstop, except the two residuals in left_open. (d) doctype kinds — placement and location, both driven. Difference from the brief: the brief named 2 doors and 2 copy-in sites; the real count is 2 copy-in sites / 8 leaves / 9 call paths, 5 sweep surfaces, AND a third door family the brief and the plan both excluded on a wrong premise — the milestone-record door (8 call sites of `reconcile_record_preflight`), which is the same UNKNOWN arm, driven as an exit-0 loss, and is OUT (undecided fork, see left_open). Also found: `flow49_acceptance` pinned the same silent merge as a second standing test nobody had listed; restaged.

### pinned_surfaces_moved

None. No frozen schema hash, schema-manifest, pinned `--format json` key, `contract-version`, compose golden, or registry row (STORE_EXIT_FLIPS, COMMITTING_DOORS, VERB_KINDS, GATE_COVERAGE, AMBUSH_CONTRACTS) moved; no new finding code, store, knob or doctype. Additive and stated: the existing `findings[]` array of a `doc` write ack can now carry the existing `file-state.baseline-adopt` code (path-keyed), which the contract doc previously scoped to `surplus-sections-absent`. Text pins that moved: `reconciliation_baseline_contrast` (restaged, test renamed `the_baseline_decides_between_a_block_and_the_declared_merge`), `flow49_acceptance` arm 1 (renamed `a_lost_baseline_no_longer_switches_off_the_never_silently_merged_guarantee`), engine unit test renamed `a_none_lookup_keeps_every_arm_and_only_a_touched_path_asks`. Not pinned but changed: `engine::state::create` and `create_gated` take a new `jigc_root: &Path` argument; `CreatedDoc` gains `adopted_baseline`.

### notes

COMMIT: eb18e5fe `fix(reconcile): a task records what it copies in, and a staged doc with no record is decided by its base pin`, on fix/rc24-tier1, one commit, 17 paths staged by name, tree clean, nothing pushed.

GATE: `tests   passed=4481 failed=0  (over 17 test binaries)` · `GATE: PASS`.

WHAT WAS FIXED: (R3, F7). A hand edit made after a task's first write to a managed doc no longer gets overwritten at exit 0 when the checkout has no file-state key for that doc. Both committing doors now block with the existing `reconciliation.conflict-block`, commit nothing, and leave the bytes on disk.

HOW, as decided (robust arm):
1. Copy-in door. Both sites read through `engine::file_state::read_for_copy_in` (wrapped by `engine::state::read_for_copy_in`, which supplies the 'another open task holds it' fact). Key absent → records the hash of the raw bytes read, under `promote_destination`'s key, inside the record's save lock, before staging. Record-if-absent; never moves a held key. Not recorded: a non-conformant doc; a doc another open task already has staged (cell 8). Failure (lock timeout, unreadable record) fails the write with nothing staged.
2. Backstop. `reconcile_committed` UNKNOWN arm: touched + pin has a blob + on-disk differs → conflict-block, nothing recorded. Skips the caller's path-keyed migration source. `git_blob_at` now reads `git cat-file --filters`.
3. The merge order (edit before the first write) still lands. 4. Migration `unmanage` exit, L1 absorb and untouched first-encounter adoption stay green (`pre_guard_repair_route`, `flow59_branch_and_pull`, `l1_pull_absorption` all passed in the gate).

RED FIRST: the new suite ran against the unfixed tree with 14 of 15 tests failing on the defect (the reported instance: `expected exit [3], the process ended with exit 0`, `promoted VISION.md`). The 15th, the CRLF test, was vacuous before the backstop existed; after the fix it was mutation-checked — reverting `--filters` to a raw `blob` read makes it fail at exit 3 — and the real code restored.

THE NINE CELLS, each a test in crates/cli/tests/copy_in_baseline.rs unless noted: (1) raw-bytes hash — `the_recorded_hash_is_of_the_raw_bytes_not_the_canonicalized_copy` + engine unit; (2) the sweep's key — engine `the_copy_in_key_is_the_key_the_sweep_reads` (location and placement) + every e2e arm; (3) lock timeout fails closed at both sites; (4) concurrency — seven real processes on one file-state.json, plus engine cell `concurrent_copy_ins_lose_no_key_and_adopt_a_shared_doc_once` in file_state_concurrency.rs; (5) non-conformant never baselined, never overwritten; (6) adoption stated on the write that made it, once, both surfaces; (7) store sweep reads as a baselined checkout; (8) second task's copy-in records nothing, first task blocks; (9) staged doc with no record decided by the pin, both orders, both doors, plus `unmanage` after the block.

THINGS THE ORCHESTRATOR SHOULD KNOW:
- The class is larger than handed: the milestone-record door is a third door family with the same driven loss, left open as the human's call (first left_open item). This is the one item most likely to come back at the re-review.
- I reworded one clause of the Detection-timing write row instead of building a write-door block; see left_open.
- A new behaviour adopters will see: in a clone, the first write's ack now prints a `file-state.baseline-adopt` advisory line, and `jigc validate` / the pre-commit hook report a later hand edit as `file-state.hash-matches` drift instead of the `un-baselined` advisory. Both are stated in the design docs.
- With the key lost after a copy-in, an edit made before the first write now blocks too (previously merged). This is the decided 'block both orders' cost and is stated in storage.md.

Work files (not committed): <scratch>/f7/ — drive-task.sh, drive-milestone.sh, record-door.sh, repro.sh, gate2.log.

### routes_driven

- Built `target/release/jigc` from the fixed tree; rig `dev/jigc-rig committed-singletons --binary target/release/jigc`, then a real `git clone` of $REPO.
- TASK DOOR — clone, `jigc start --workflow single-task`, `jigc doc set-slot vision:vision#open-questions` (exit 0, ack prints `advisory · file-state.baseline-adopt — baseline adopted: \`VISION.md\``; file-state.json now holds VISION.md), hand line appended: `jigc task validate` exit 3, `jigc task finalize --dry-run` exit 3, `jigc task finalize` exit 3 with `blocking · reconciliation.conflict-block — conflict on \`VISION.md\`…`; HEAD unchanged, hand line still on disk.
- Emitted route span run verbatim: `jigc task discard sharpen-the-open-questions --force` → exit 0 (`discarded task … dropped staged edits to: commit:… (transient), vision:vision`), hand line still on disk, 0 tasks left.
- Route's second exit followed as printed (revert the external edit on disk): `git checkout -- VISION.md` exit 0, then `jigc task finalize sharpen-the-open-questions` → exit 0, task prose at HEAD.
- MILESTONE DOOR — clone, `jigc milestone create`, `jigc milestone add-task`, sub-task `doc set-slot` (adoption stated on the ack), hand line: `jigc milestone finalize sharpen-the-docs` → exit 3, `reconciliation.conflict-block` at VISION.md, HEAD unchanged, hand line on disk.
- Emitted route span run verbatim: `jigc task list` → exit 0, names `alpha-sharpens-a-doc [sub-task]`; then revert + `jigc milestone finalize sharpen-the-docs` → exit 0, `promoted VISION.md`, task prose at HEAD.
- SAVE-LOCK TIMEOUT (in `copy_in_baseline::a_save_lock_timeout_fails_the_write_door_closed_at_both_copy_in_sites`, real binary): `jigc doc set-slot …` and `jigc doc create adr …` behind a held lock → non-zero after the 10 s budget, stderr names `file-state.json.lock` and `retry`, nothing staged, no key; the same argv re-run after release → exit 0 and the key recorded.
- STORE SCOPE in the clone: before any write `jigc validate` exit 0 with four `file-state.un-baselined` advisories; after the copy-in and the hand edit `jigc validate` exit 0 with `blocking (gates at finalize) · file-state.hash-matches` at VISION.md — the same row and exit a baselined checkout reports (asserted equal to a control in the suite).

### docs_changed

- design/reconciliation.md — state-machine table (UNKNOWN split by touched), Baseline adoption (the copy-in record, its four bounds, the base-pin backstop), Persistence of the shifted baseline (amendment), Conflict (the unmanage exit stays an exit), Detection timing (write row + what each row persists), Hash re-baselining, What reconciliation does NOT do (no cache exemption), Open questions (two)
- design/storage.md — Concurrent writers: the new sub-agent-time writer, fail-closed at the copy-in door, and a rewritten 'What none of this buys'
- design/validation.md — the un-baselined route is now true; what the store sweep and pre-commit hook report in a clone after a copy-in
- design/command-output-contract.md — `file-state.baseline-adopt` as a write-ack finding, with its stated bound
- design/write-commands.md — copy-on-first-touch records the baseline
- design/worked-examples.md — Flow 49's contrast sentence
- crates/cli/guides/MIGRATING.md — item 8
- No `--help` text states the old behaviour; none changed.
- OWED BY THE ORCHESTRATOR (not touched): DECISIONS.md — one line that (R3, F7) landed as eb18e5fe, that the Detection-timing write row's second clause was reworded rather than built, and that `engine::state::create`/`create_gated` gained a `jigc_root` parameter (a breaking signature change in the published `jigc-engine` rc API). implementation/project-history.md — the fix-pass span. implementation/decisions-pending.md — trigger rows for the left_open items the human defers.

### left_open

- HUMAN'S CALL — the milestone-record door, a driven exit-0 loss in the same UNKNOWN arm. In a clone (no key for the record), a hand line appended to `docs/milestone-records/<id>.md` then `jigc milestone add-task` → exit 0, record commit landed, hand line in worktree 0 / HEAD 0. Driven on rc.24 and again on the fixed binary (unchanged by this fix: that door passes no pin, so the backstop never reaches it). It contradicts design/team-ready-state.md → No-silent-overwrite discipline. Reaches all 8 call sites of `reconcile_record_preflight` (the record-only doors and milestone finalize's status flip). The plan excluded it as 'splices in place, does not promote'; driven, it re-renders over the edit. Not converted because it is a new refusal at a door family the fork did not decide. Recommendation: same backstop, keyed on the record's blob at HEAD read with `--filters`, raising the door's existing `record_conflict_block`; needs a second seam so the L1 absorb stays off at that door. Flagged in design/reconciliation.md → Open questions.
- HUMAN'S CALL — Detection timing's write row, second clause. 'Re-probe before staging' is now true as written. 'Block if the doc has become conflicted since the task started' was NOT built at the write door: it would be a new refusal on eight write leaves, which the decision does not list. The row was reworded to say the block is raised by the next sweep, and the question is recorded in Open questions. If the human wants the row literally true, that is a follow-up build.
- RESIDUAL, stated in reconciliation.md and storage.md — a staged doc with neither a record nor a blob at the pin still adopts: a doc untracked at the task's base whose key was lost after the copy-in, or never written (non-conformant at copy-in; cell 8's second task). Cell 8 is closed for a pin-tracked doc (tested, both kinds); for an untracked doc the backstop has nothing to compare, so two tasks plus a hand edit between their copy-ins can still lose it. Closing it needs a per-task record of what was copied in (the verifier's stronger design, a working-area format change, 1.x). Reasoned, not driven.
- TIER 2, external-writer race class (ruling 4), unchanged — a non-jigc writer landing between one copy-in's read and another task's, or between the sweep and the promote. The copy-in read sits inside the save lock, which closes the jigc-versus-jigc half only.
- TIER 3 surfacing bound, stated in command-output-contract.md — a write refused after its copy-in ran (e.g. `write.non-reparseable`) prints only the refusal; the doc is staged and its baseline recorded, and neither is stated. The copy-in note already had this gap.
- TIER 3 wording lead, not changed — the milestone-boundary conflict route ends '…then re-run the join' at the `milestone finalize` door. Driven: `jigc task list` exit 0 and revert + `milestone finalize` lands, so it works; the last word names the wrong verb.
- Arm A unchanged by decision — with a baseline present, a hand edit made BEFORE the task's first write still blocks at finalize, and the route's revert exit then lands a staged copy that carries the edit (the advocacy's point 5). Not in scope of this fork.
- The linked-worktree sibling of (R3, F7) is its own decided fork (A) and was not touched here.

## 8c159622 fix(milestone): a door removes a git worktree registration only at a path it created, by path

### status

fixed

### gate

dev/gate (full, unscoped, foreground-waited): `tests   passed=4489 failed=0  (over 17 test binaries)` · `GATE: PASS` (fmt/clippy/build/test/doctest all exit=0; log <tmp>). The tree was byte-identical between gate start and commit. A first gate run was red on three source fences my own change tripped (a wrapped message literal, two `{path:?}` renders, an unlisted `Drop` removal); all three were fixed before the passing run. After the commit, `dev/runner-faithful --commit HEAD cargo test -p jigc --test g_milestone -- worktree_registration_reach:: milestone::milestone_finalize_warns` on the runner image (git 2.43.0): 9 passed, 0 failed, exit 0.

### class

A jigc door mutating git's worktree registry (`.git/worktrees/`) outside the set jigc created. Handed: 4 doors / 5 bare-prune argv sites (+ uninstall's misattribution). Found: 5 argv sites + 1 printed remedy (the A2 leaked-worktree warning told the reader to run `git worktree prune`) + 2 mirrors of the same mechanism that were not in the finding's door list: (a) a REFUSING `milestone provision` dropped registrations before its phase-1 refusal ("nothing was removed"), and (b) the prune destroyed jigc's OWN records in a moved repository / other mount, the ones `git worktree repair` needs. Dispositions — CONVERTED: `provision_worktrees` (prune removed; reuse decided from git's own `prunable` line; stale own record dropped by path in phase 2, after the clear), `remove_worktrees` (trailing prune removed, nothing replaces it), `DedicatedWorktree::add` and `::drop` (both prunes removed; drop falls back to removing its own throwaway checkout and retrying the keyed removal), `setup::prune_worktree_admin` → `drop_workbench_registrations` (keyed to direct children of `<jigc_home>/.jigc/worktrees/`, so the ack line prints only when an own record was dropped), the remedy text (→ `leaked_worktree_remedy`, keyed on locked / prunable / other). All removals now go through ONE function, `milestone::remove_owned_registration`, which refuses any path that is not a direct child of `<root>/.jigc/worktrees/` before git is asked. OUT with reason: `git worktree add` (2 sites — cannot touch another path's record), `git worktree list` (read-only), test-module `worktree add` in repo.rs/setup.rs/combine.rs (not production), `uninstall.dirty-worktree`'s route text `git worktree list, then git worktree remove` (already keyed), historical mentions in roadmap.md:717 and decisions-pending.md:695 (charters/records, not current-truth sentences), packs/adapters/guides (no mention — now fenced). Not in the class, confirmed by reading: `jigc start`, single-task `task finalize`, no `git gc`/`maintenance` call anywhere in production.

### pinned_surfaces_moved

No frozen schema hash, `--format json` key, `contract-version`, compose golden, finding code or exit code moved. `UninstallSummary::pruned_worktrees` keeps its name, type and text-only status; the uninstall ack sentence is unchanged (it is now absent in the one cell where it was false). Unpinned text that moved: the leaked-worktree warning's remedy sentence. Existing tests edited: `milestone.rs::milestone_finalize_warns_on_a_leaked_worktree_but_still_succeeds` (assertion flipped from `worktree prune` to `worktree unlock` + `worktree remove --force` and no prune); `git_span_aim.rs` (`GIT_SPAN_SITES` row re-keyed `remove_worktrees` → `leaked_worktree_remedy`, census note, one predicate example); `repo_relative_paths.rs` (row reason text); `rollback_population_registry.rs` (one `NOT_A_POPULATION` row for `DedicatedWorktree::drop`); comment-only edits in `cwd_verb_subject.rs` and `provision_leftover_guard.rs`.

### notes

(1) COMMIT: 8c159622 `fix(milestone): a door removes a git worktree registration only at a path it created, by path` on `fix/rc24-tier1` (confirmed by `git branch --show-current` before committing; started from a clean tree; 15 paths staged by name; tree clean after; nothing pushed, no branch switched).

(2) GATE: `dev/gate` → `tests   passed=4489 failed=0  (over 17 test binaries)` · `GATE: PASS`. Runner-image check on the committed revision (git 2.43.0): 9 passed, exit 0.

(3) WHAT WAS FIXED: no repository-wide `git worktree prune` remains in production code. Every registry removal goes through `milestone::remove_owned_registration` (`git worktree remove --force <path>`), which refuses a path that is not a direct child of `<root>/.jigc/worktrees/`. A refusing run touches nothing in `.git/worktrees/`. `uninstall` says it dropped registrations only when it dropped its own. The leaked-worktree remedy names the one registration instead of the prune.

RED EVIDENCE: the new suite `crates/cli/tests/worktree_registration_reach.rs` (registered in `crates/cli/tests/groups/g_milestone.rs`) was 8 of 8 red on the unfixed tree, each for the defect — e.g. `provision × bare`: `foreign-detached`'s admin directory empty where `[COMMIT_EDITMSG, HEAD, ORIG_HEAD, commondir, gitdir, index, logs/HEAD]` was planted; the moved repository: `[]` vs `[area-low, area-zed]`; the fence: `milestone.rs::provision_worktrees runs git worktree prune`. It is 8 of 8 green after.

GIT PRIMITIVE, confirmed on this machine (2.54.0) and the runner image (2.43.0): `git worktree remove --force <absent path>` exits 0, drops that one record and leaves a `prunable` sibling. Three properties that shaped the code:
- it also works when the path's parents are gone, but only with the canonical path git recorded — so `uninstall` passes the listed path verbatim;
- it refuses (exit 128) when a directory without a `.git` link stands at the path — so `provision` removes the stale record after the leftover clear, never before;
- it refuses a locked record.

DECISIONS I MADE THAT THE ORCHESTRATOR SHOULD KNOW:
- Plan row P was not taken. It would turn an exit-0 `provision` into a refusal, which the brief reserves for the human. I kept today's reuse by reading git's own `prunable` verdict, which is exactly the set the old prune dropped. It is listed in left_open with both options.
- jigc's own stale registrations are still dropped silently, as instructed; the commit message, rustdoc and design doc say so. I added no narration there, to leave the refuse-versus-name choice whole for the next fixer.
- One commit rather than the plan's five: it is one finding and one rule, and the fence is only true once every site is converted.
- The plan suggested keeping the remedy as a single generic sentence. I keyed it on git's verdict instead (locked / prunable / other), so the locked case's two printed spans are the complete repair as printed.
- `DedicatedWorktree::drop` gained a fallback (remove its own throwaway checkout, retry the keyed removal). It replaces what the prune did for a checkout git will not remove, and it cost one `NOT_A_POPULATION` row in `rollback_population_registry.rs`.
- The source fence carries one stated false positive: `engine/src/finding.rs`'s `GIT_COMMAND_GROUPS` word list, where `worktree` and `stash` are neighbours. It is a table row with its reason, not a filter.

FILES: crates/cli/src/milestone.rs · crates/cli/src/task.rs · crates/cli/src/setup.rs · crates/cli/tests/worktree_registration_reach.rs · design/team-ready-state.md · design/storage.md. The drive script (uncommitted work file) is <scratch>/l22-drive.sh, run as `bash l22-drive.sh <cell>`.

### routes_driven

- BINARY: target/release/jigc built from this tree (`jigc 1.0.0-rc.24`), host git 2.54.0, each cell on its own `dev/jigc-rig fresh --binary target/release/jigc` rig ([dev ▸ methodology]); foreign plant = y detached with a commit only its HEAD reaches + a staged-only file, z on branch `side`, both directories moved away (2 `prunable` records before every cell). The release binary was rebuilt after the last source edit and the remedy/finalize cells re-driven on it; the tree then went unchanged into the passing gate and the commit.
- `jigc milestone provision foreign-worktree-probe` (first, and re-run) → exit 0; `.git/worktrees` still holds y z; fsck-unreachable hits for the detached commit 0 → 0; directories returned → `git status` exit 0 in both (y shows `A  foreign-staged.txt`), `git worktree repair` exit 0
- `jigc milestone finalize foreign-worktree-probe` landing (README §7's repro verbatim) → exit 0, `finalized … (2 sub-tasks)`; y z intact, own records gone, no combine record left
- `jigc milestone finalize foreign-worktree-probe` on a never-provisioned milestone → exit 3 `milestone.zero-contribution`; `.git/worktrees` = y z before and after (the refusing run no longer mutates)
- `jigc milestone discard foreign-worktree-probe` never-provisioned → exit 0; provisioned → exit 0; refused over staged code → exit 1 `milestone.dirty-worktree`: y z intact in all three
- `jigc uninstall` over a foreign-only workbench → exit 0, y z intact, and the ack does NOT print `pruned git's worktree registrations…` (the misattribution cell); `jigc uninstall --force` over fan-out + foreign → exit 0, own records gone, y z intact, ack line printed; then `jigc setup` exit 0 and `jigc milestone provision` with the same sub-task ids exit 0, no stale-name collision
- EMITTED REMEDY, locked own worktree at a landed finalize (exit 0) — both printed spans run verbatim via `sh -c` from a cwd outside the repository: `git -C <repo> worktree unlock <repo>/.jigc/worktrees/add-alpha-file` → exit 0; `git -C <repo> worktree remove --force <repo>/.jigc/worktrees/add-alpha-file` → exit 0; record gone, y z intact
- EMITTED REMEDY, own checkout that lost its `.git` link, at `discard --force` (exit 0, ack `workbench NOT fully removed`) and at a landed finalize (exit 0, ack `unreadable worktree …, no code counted`; nothing narrated as lost, bytes still on disk): after the reader's stated step (move the directory aside) the printed `git -C <repo> worktree remove --force <path>` run verbatim from outside the repo → exit 0; record gone, y z intact
- `jigc milestone provision` refused over a registered-but-unlinked own path → exit 1 `milestone.leftover-holds-work`, own record AND y z still registered; its printed route `jigc milestone provision foreign-worktree-probe --force` run verbatim → exit 0, worktree re-added live (`git status` exit 0)
- Moved repository (`mv` after provision with staged code): `provision` → exit 1 `milestone.leftover-holds-work` and both own records survive; `git worktree repair <both paths>` exit 0, file still staged (`A  add-alpha-file.txt`); `provision` exit 0; `finalize` exit 0 landing both files
- Stale `.combine-1-2` record (directory hand-deleted): a later `finalize` lands at exit 0 and leaves it; `uninstall --force` drops it
- Not driven: the trial's container cell O-1 on the new build (covered by equivalence — path absent — and by the runner-image suite run, not by a two-mount container); `jigc milestone execute` advisories after a refused provision (read: they classify the path, not the registered set)

### docs_changed

- design/team-ready-state.md — `jigc milestone discard <id>` → *Abandon refuses on a dirty worktree* (the door rule's home): the registration-reach rule, the driven datum, and the stated bound for jigc's own stale registrations
- design/storage.md — Repository layout, the `worktrees/<sub-task-id>/` row: one pointer clause to that rule
- design/surface-contract.md — the operand-less span example `git worktree prune` → `git worktree list` (no producer emits the former any more)
- rustdoc at every site: `provision_worktrees`, `worktree_registrations`, `is_owned_worktree_path`, `remove_owned_registration`, `remove_worktrees`, `leaked_worktree_remedy` (crates/cli/src/milestone.rs); `DedicatedWorktree` (crates/cli/src/task.rs); `UninstallSummary::pruned_worktrees`, `drop_workbench_registrations` (crates/cli/src/setup.rs); two example spans in crates/engine/src/finding.rs
- `--help` and the guides: unchanged — none mentions registrations or a prune, before or after (now fenced for packs/adapters/guides)
- OWED in the orchestrator's logs (not touched): DECISIONS.md — one dated entry correcting M31's "harmless `git worktree prune` no-op" and the 2026-09-23 "It prunes now … and says so" with the driven datum (foreign records, the moved repository, the refusing runs) and the keyed rule. project-history.md — the fix-pass span. completions/artifacts/RC-rc24/README.md §7 L-22 — a fixed-at annotation (8c159622).

### left_open

- HUMAN'S CALL / the next fixer's (not built, as instructed): jigc's OWN stale registration — a sub-task worktree whose directory is gone still has its HEAD, index and reflog dropped with the record, unnamed, at `provision` (phase 2) and at the teardown. Behaviour kept exactly as under the prune and stated in the rustdoc and in team-ready-state.md. The single seam for it is `milestone::remove_owned_registration` plus `WorktreeRegistration::prunable`.
- HUMAN'S CALL (plan row P, deliberately NOT taken because it would make a previously exit-0 command refuse): a jigc sub-task worktree that is `git worktree lock`ed and whose directory is then deleted — `jigc milestone provision` still exits 0 and prints `provisioned 2 worktree(s)` although one is not on disk (driven on the new build: on disk `add-beta-file`, registered both). Pre-existing false success, preserved because git never calls a locked record `prunable`. Options: (1) keep; (2) treat registered-but-absent as not reusable, which surfaces `milestone.provision-failed` with git's lock sentence.
- Not closed, residue only: a stale `.combine-*` record whose directory was hand-deleted after a crashed finalize now lingers as `prunable` until `jigc uninstall` or git's own 3-month expiry (driven). No milestone door sweeps it, because a sweep in `DedicatedWorktree::add` would mutate the registry on a refusing run. A crashed finalize's LIVE `.combine-*` worktree is cleaned by no door — pre-existing.
- Not closed, residue only: jigc's own records at a previous home path (moved repository) linger as `prunable` after the user re-provisions with `--force` instead of repairing; jigc cannot prove a record outside `jigc_home` is its own, which is the fix.
- Seen, not changed (pre-existing text, tier-3 shape): `milestone.dirty-worktree`'s per-path line "registered here, so the teardown removes it and this content is destroyed" is false for a registered-but-unlinked directory (the removal fails and the bytes stay); the leftover refusal's route does not name `git worktree repair` for a moved repository although it now works.
- Bound introduced and stated in rustdoc: staleness is read from git's `prunable` porcelain line (git ≥ 2.31, 2021-03). On an older git a stale own registration reads as live and `provision` reuses it. Verified on 2.54.0 and 2.43.0 only.
- Test-coverage caveats: the hook-rejected-finalize cell of `a_refusing_run_leaves_git_worktree_admin_as_it_found_it` was added after the red run, so it was never observed red (same mechanism as the bare-finalize cell, which was). Row P and the stale-`.combine-*` lingering are driven, not pinned by a test. `--format json` is iterated on the door axis but not on the remedy cells.

## ebfc79fb fix(milestone): a worktree door answers for what git's registration of the path holds

### status

fixed

### gate

dev/gate (full, never --quick, unscoped, run in the background and read from its own summary) on the exact tree committed: fmt exit=0 · clippy exit=0 · build exit=0 · test exit=0 · doctest exit=0 · `tests   passed=4497 failed=0  (over 17 test binaries)` · `GATE: PASS` (log <tmp>). An earlier full gate on the first complete tree also passed (passed=4496). Red first: the new suite's then-6 tests all failed on the pre-fix tree, each on the defect (e.g. `jigc milestone provision × Commit × Stale × un-forced … must refuse — got exit 0`). Note: the gate's test step took 497 s and 504 s on this machine (it includes recompiling the test targets), close to the 10-minute foreground ceiling and well above the ~5 min the fixer brief quotes.

### class

Class: a jigc door drops one of its OWN worktree registrations whose HEAD or index holds work no ref reaches. Handed: provision + un-forced discard over a STALE registration, finalize's false line, and discard/finalize/uninstall over a LIVE committed worktree. Real count: 5 dropping sites, 4 stating surfaces and 2 members nobody had counted.

CONVERTED
1. `provision_worktrees` phase 2 (stale drop): refuses via the probe's new third leg; `--force` names what it took.
2. `remove_worktrees` from `discard`: refuses via `held_subtask_worktrees`; `--force` names.
3. `remove_worktrees` from landed `finalize`, chain arm: names.
4. `remove_worktrees` from landed `finalize`, combine arm: names.
5. `drop_workbench_registrations` from `uninstall`: refuses; `--force` names, after the drop.
6. NOT COUNTED BEFORE — `setup::fanout_worktree_paths`: uninstall enumerated its worktree subjects from the children on disk, so a registration with no directory was not a subject at all. Registered own paths now join the set (read from git's admin files, no process).
7. NOT COUNTED BEFORE — the `--force` narration read the landed boundary's partition, where a wholly staged path is deliberately unreported. Driven on the pre-fix binary: `milestone discard --force` and `uninstall --force` over a live worktree holding one `git add`-ed file printed nothing. Fixed with a per-worktree `StagedSet` (also covers the landed teardown over a sub-task settled by `task discard`).
8. The landed contribution label: a registered path with no directory reads `its worktree directory is gone, no code counted — git's registration of it held …, which did not land`; `nothing staged` is withheld there.
9. `milestone execute`'s `milestone.worktrees-partial` advisory: its tail no longer promises `provision` "adds the missing ones" over a registration it will refuse on.
10. `--help` for `milestone provision`, `milestone discard`, `uninstall`.

OUT, with reason
- `DedicatedWorktree::drop` and a crashed `.combine-*` at uninstall: its HEAD/index are jigc's own synthesis of the sub-task worktrees' staged sets, whose sources are probed at their own paths. Pinned as a control.
- `provision` over a LIVE registered worktree: it reuses it untouched and drops nothing, so it neither refuses nor narrates (asserted in every such cell).
- `jigc task discard` / `task finalize`: they stand at `.jigc/tasks/<id>/`, remove no worktree and drop no registration.
- `milestone join`, `task validate` previews: they remove nothing.
- zero-contribution refusal text: unchanged; its route chain was driven and works (finalize exit 3 → provision exit 1 with the recipe → recipe → provision exit 0 → finalize lands the file).

Cells owed and pinned in `crates/cli/tests/worktree_registration_anchor.rs` (8 tests, registered in `groups/g_milestone.rs`): the three refusing doors (read from `WORKTREE_DOORS`) × {commit, staged, both, empty} × {stale, live} × {un-forced, --force} = 48 cells; landed finalize × 4 × 2 × both commit arms × both formats = 32 cells; restore recipe 3 doors × 3 holdings; keep command at the 5 (door, standing) pairs that reach a commit, plus landed finalize × 2; 5 over-refusal controls.

### pinned_surfaces_moved

None of the pinned set moved: no frozen schema hash, no `--format json` key, no `contract-version`, no compose golden, no finding code added, removed or re-keyed, no new flag, store or knob. The landed test asserts the `committed.sub_tasks[]` key set is still exactly {code_files, discarded, docs, hash, id, provisioned, worktree_unreadable}.

What did move, all unpinned text or code-side registries:
- Exit codes 0 → the door's existing blocking code, only in the ruled cells: stale × {commit, staged, both} at provision / discard / uninstall, and live × commit at discard / uninstall.
- `uninstall.dirty-worktree`'s heading reworded (the old "`.jigc/` holds N … path(s)" was false of a path with no directory).
- New per-path refusal wording, a new route clause (`ANCHOR_CLAUSE`), a new stderr warning ("dropping git's registration of the fan-out worktree … discards what only it held"), and a new contribution-label clause.
- `--force` at provision / discard / uninstall now also names wholly staged paths of a live worktree.
- `render::SubTaskContribution` gained a `#[serde(skip)]` text-only field `stale`; `LeftoverShape` gained `Registration`; `LeftoverHold` gained `anchored` (the `cli` lib is doc(hidden), no API).
- Test registries extended: `GIT_SPAN_SITES` +2 rows, `NOT_A_POPULATION` +1 row, `DEBUG_REMAINDER` row renamed `discarded_work` → `worktree_work`, one struct literal in `text_json_parity_axis.rs`.

### notes

Fixed in one commit on `fix/rc24-tier1`: `ebfc79fb fix(milestone): a worktree door answers for what git's registration of the path holds`. Tree clean, nothing pushed, no branch switched.

WHAT WAS FIXED. The leftover probe (`probe_leftover` in crates/cli/src/milestone.rs) gained a third leg, `LeftoverHold::anchored`: what git's registration of a sub-task worktree path holds that no ref carries — a commit only its HEAD reaches, and, where no live checkout stands there, the paths staged in its index. The admin directory is located under git's common dir by its `gitdir` file. `provision`, un-forced `discard` and `uninstall` refuse over it under their existing codes with `--force` the existing consent; an empty registration still clears at exit 0; the landed `finalize` names the path, commit and staged paths as the registration goes, and its contribution line is true.

GATE EVIDENCE. `tests   passed=4497 failed=0  (over 17 test binaries)` and `GATE: PASS`, full `dev/gate` on the committed tree.

EMITTED COMMANDS RUN VERBATIM (release binary from this tree, cwd `/`):
- restore recipe `mkdir -p <path> && printf 'gitdir: %s\n' <admin> > <path>/.git && git -C <path> restore .` → exit 0, then `provision` exit 0 and `finalize` exit 0 with the staged file in HEAD;
- keep command `git -C <checkout> branch kept/<sub-task-id> <sha>` → exit 0 at provision, discard, uninstall and after a landed finalize; the door then proceeds un-forced;
- the three `--force` routes → exit 0, each naming what it took.

TWO DECISIONS I MADE INSIDE THE RULING, both stated in the design doc:
1. The recipe writes the `.git` link by hand instead of printing `git worktree repair`. That verb opens with a pass over every registration in the repository, and in a copied repository it would re-point the source's live worktrees at the copy — the reach the previous commit closed. (This is from reading git's behaviour and one spike where a bare `repair` rewrote a link; the cross-repository harm itself was not driven.)
2. The milestone's base pin counts as reached, per the ruling's "HEAD at the base pin". Driven with history rewritten so no ref reaches the pin: discard exits 0, live and stale, as on rc.24.

BEYOND THE HANDED LIST. Two members nobody had counted, both fixed: uninstall's subject enumeration could not see a registration with no directory at all, and `--force` at the three non-landing doors named nothing for a wholly staged file in a live worktree. Details and the full disposition are in `class`.

NOT VERIFIED. Nothing ran on a git older than 2.54.0 (Docker daemon down). The cross-repository effect of `git worktree repair` is reasoned, not driven.

FILES. New suite: crates/cli/tests/worktree_registration_anchor.rs (registered in crates/cli/tests/groups/g_milestone.rs). Code: crates/cli/src/milestone.rs, setup.rs, render.rs, repo.rs, cli.rs. Driver scripts and their outputs are in <scratch>/wt1/ (base.out is the pre-fix matrix; after.out, routes.final.out, routes2.final.out are the fixed binary).

### routes_driven

- All on target/release/jigc built from the committed tree, in `dev/jigc-rig fresh` rigs, each printed span extracted from the refusal and run verbatim through `sh -c` with cwd `/`.
- provision × stale × both: `jigc milestone provision anchor-probe` exit 1 `milestone.leftover-holds-work` → printed `mkdir -p <abs path> && printf 'gitdir: %s\n' <abs admin> > <abs path>/.git && git -C <abs path> restore .` exit 0 (`A  alpha-s.rs`, HEAD at the planted commit) → `jigc milestone provision anchor-probe` exit 0 (reuse) → `jigc milestone finalize anchor-probe` exit 0, `git show HEAD:alpha-s.rs` returns the staged bytes (recovered and landed).
- provision × stale × commit: refusal exit 1 → printed `git -C <repo> branch kept/add-alpha-file <full sha>` exit 0 → `jigc milestone provision anchor-probe` exit 0 with no warning; `kept/add-alpha-file` resolves to the planted sha.
- discard × live × commit: `jigc milestone discard anchor-probe` exit 1 `milestone.dirty-worktree` → printed `git -C <worktree> branch kept/add-alpha-file <sha>` exit 0 → `jigc milestone discard anchor-probe` exit 0; the branch holds the commit.
- uninstall × stale × staged: `jigc uninstall` exit 1 `uninstall.dirty-worktree` → printed restore recipe exit 0; `alpha-s.rs` back on disk, staged, bytes intact.
- uninstall × live × commit: `jigc uninstall` exit 1 → printed `git -C <worktree> branch kept/add-alpha-file <sha>` exit 0 → `jigc uninstall` exit 0; the branch holds the commit.
- landed finalize × live × commit: `jigc milestone finalize anchor-probe` exit 0, stderr names the path and commit; commit unreachable (fsck 1 hit) → printed `git -C <repo> branch kept/add-alpha-file <sha>` exit 0 → reachable (0 hits).
- The mechanical consent routes as printed: `jigc milestone provision anchor-probe --force` exit 0, `jigc milestone discard anchor-probe --force` exit 0, `jigc uninstall --force` exit 0 — each names the commit and `alpha-s.rs (staged, in no commit; blob …)` on stderr.
- zero-contribution chain over a stale holding registration: `jigc milestone finalize` exit 3 → its route `jigc milestone provision anchor-probe` exit 1 with the recipe → recipe exit 0 → provision exit 0 → finalize exit 0, file landed.
- Controls driven: empty stale registration at provision / discard / uninstall exit 0 with no registration warning; commit on a branch made inside the worktree → discard exit 0; orphaned base pin, live and stale → discard exit 0 (same as the installed rc.24); reflog-only commit → discard exit 0; crashed `.combine-*` holding a commit → uninstall exit 0; `.jigc/` hand-removed with a holding registration → uninstall exit 0, registration untouched; locked + gone + holding → discard exit 1, `--force` exit 0 with the existing leaked-worktree remedy and no loss narrated (the registration stayed); cp -R copy with clean worktrees → provision and discard exit 0 in the copy.

### docs_changed

- design/team-ready-state.md — the door rule's home: the "not answered here" bound is struck with the datum that falsifies its warrant; the third leg, the refusal's contents, the empty control, the landed boundary's naming and four stated bounds
- design/finalize.md — the landing manifest's fourth fact (registered path, directory gone), text-only
- design/storage.md — the `worktrees/<sub-task-id>/` row: a door answers for what the registration holds
- design/project-setup.md — uninstall's worktree subject widened to registrations, and scoped to runs that drop them
- crates/cli/guides/MIGRATING.md — step 7, what `milestone.dirty-worktree` refuses over
- --help text: `jigc milestone provision` and its `--force`, `jigc milestone discard` and its `--force` (crates/cli/src/milestone.rs), `jigc uninstall` (crates/cli/src/cli.rs)
- NOT touched, owed by the orchestrator: DECISIONS.md and implementation/project-history.md (see left_open)

### left_open

- OWED, orchestrator's shared logs (not touched): a DECISIONS.md entry for 2026-10-04 — fork worktree-1 ruled REFUSE; the M31 "idempotent re-provision" kept only for an empty registration; the struck bound and its falsifying datum; the four new bounds; the `--force` wholly-staged narration fix; the uninstall heading reword; the `kept/<sub-task-id>` branch name. Plus the project-history span for the pass.
- OWED, implementation/decisions-pending.md 1.x ledger rows for the bounds below (the plan assigns these to the pass's record commit).
- HUMAN'S CALL — reflog bound: a commit a sub-agent made and then `git reset` away from is reachable only from the registration's reflog and is NOT held; the reflog goes with the registration. Driven: discard exit 0. Stated in the design doc and pinned as a control.
- HUMAN'S CALL — JSON: the landed envelope has no key for a dropped registration. A sub-task whose directory was gone reads `provisioned: false, code_files: 0`, and what its registration held is on stderr only. Carrying it on the envelope is a pinned-key move, so I did not.
- HUMAN'S CALL — a pre-landing refusal at `milestone finalize`: the boundary still lands without a sub-task whose work sits in a commit or in a stale registration's index, and names it afterwards. Refusing before landing would be a new refusal beyond the ruling.
- HUMAN'S CALL — the keep branch name `kept/<sub-task-id>`: the printed command fails loudly if that branch (or a branch named `kept`) already exists. Not driven.
- LEAD — an unlinked checkout (directory present, `.git` link gone) whose registration holds staged paths: the refusal now names what the registration holds, but prints no re-link command (the restore recipe is printed only where nothing stands at the path, because `git restore .` would overwrite files). Driven: provision exit 1 naming both. The leaked-worktree remedy's "unlinked" arm has the same gap.
- LEAD — sibling-HEAD false refusal: reachability is asked under `--single-worktree`, so a commit another linked worktree's HEAD also reaches reads as unreached. Driven only for two sub-task worktrees at the same commit (both refuse, correctly, since discard drops both).
- NOT DRIVEN — older git. Docker's daemon was down, so nothing ran under `dev/runner-faithful`. The recipe needs `git restore` (git ≥ 2.23); the probe uses `rev-parse --absolute-git-dir` and `rev-list --single-worktree --ignore-missing`. All driven on git 2.54.0 only; CI's runner git will be the first older-git run of the suite.
- LEAD, pre-existing and unrelated: the binary panics with "failed printing to stdout: Broken pipe" when its stdout is closed early (seen when my driver piped `jigc milestone provision` into `head -1`).
- PRE-EXISTING, unchanged: a locked own registration whose directory is gone is still reused by `provision` ("provisioned N worktree(s)" over a missing directory) — the previous commit's Row P. Un-forced `discard` now refuses over it when it holds work; `--force` leaves it registered behind the existing remedy.
- PRE-EXISTING, now stated: `uninstall` with `.jigc/` already gone drops no registration, so it refuses over none and leaves them for git to expire.
- WORDING — provision's heading still says "would delete N path(s)" over a path that is only a registration; the per-path line carries the truth. I reworded only uninstall's heading, which was plainly false there.
- GATE TIMING — the gate's test step ran 497–504 s here. The new suite adds 8 tests (the longest about 40 s, run in parallel); I did not measure how much of the increase over the quoted ~5 min is the suite versus recompilation.

## a88f71cc fix(validate): a doc a task has both staged and bound is probed at its staged bytes only · e3a6ba58 fix(finalize): a linked worktree the user made commits code only

### status

fixed

### gate

Two full `dev/gate` runs, one per commit, each read from its own summary. Commit a88f71cc (gated alone, with the guard changes stashed): `tests   passed=4499 failed=0  (over 17 test binaries)` and `GATE: PASS`. Commit e3a6ba58: `tests   passed=4513 failed=0  (over 17 test binaries)` and `GATE: PASS`. A first gate run over the guard was red on five registry fences (`tests passed=4508 failed=5`, `GATE: FAIL (step: test)`): the dry-run refusal table, the two span-site tables, flow52's exempt-row count and count_fences. All five were registration homes I had missed; nothing was committed past it.

### class

An ordinary task whose commit boundary lands in a checkout other than the one the doc store binds to (owned by no milestone and `CommitSite::differing`), implemented as one type, `render::CodeOnlyCheckout`. Membership of the refused docs is `engine::finalize::promote_destination` answering `Some`, so the transient `commit` doc stays writable.

Real count against the count handed:
- **Converted as handed:** the 8 `doc` write leaves (9 arms, at 2 seams); the 4 backstop doors (`task finalize`, `--dry-run`, `task validate`, bare `jigc start`) under one `(code, target)`; `jigc migrate`; the doc-only mint (4 shipped workflows); the statement on mint, resume and orientation; the stderr note on `doc show`, `doc list` and `validate`.
- **Converted beyond the handed list (4):**
  - `changelog-recording.gate-granted-unused`'s route. Both its in-task write and its `record-change` route write a doc the checkout now refuses, so it gained a code-only arm, including for the gate promoted to blocking.
  - The amend arm's staged-doc gate. The guard now outranks it at the write seam, at finalize and in the preview, because the amend route ("an ordinary task commits the doc") is false from such a checkout.
  - Id-less composes (the router, `--preview`) carry the statement too.
  - A second defect, in a different mechanism, fixed as its own commit (a88f71cc). `enumerate_target_surface` enumerated a doc that was both staged and role-bound twice. Under `single-task` × `adr` the `doc-code` floor's own route ("update the citation") therefore could not clear the floor, from the main checkout as well. The dangling-anchor exit could not reach exit 0 without this fix.
- **Out, with reason:**
  - `rename`, `relocate`, `unmanage`, `ingest`, `migrate-corpus`, `config set`, the milestone doors, `setup` and `uninstall` are single-homed at jigc_home, so no bytes cross checkouts.
  - `task bind`, `task discard` and `task amend`'s mint carry no promoting bytes.
  - A milestone sub-task's writes from its fan-out worktree are exempt by ruling.
  - Composed step bodies still invite doc writes; the `checkout:` statement above them supersedes that, and no golden may move.

### pinned_surfaces_moved

None. No frozen schema hash, pinned `--format json` key, `contract-version`, `SCHEMA_VERSION`, compose golden, `GATE_COVERAGE` row or `.jigc/AGENT.md` byte moved. The linked-worktree test asserts the forecast and orientation key sets are equal from both checkouts and that read stdout is byte-identical. The statement and note are text only and print only when the predicate holds.

Registries that gained a row, each fenced: `FINALIZE_FAMILY`, `AMBUSH_CONTRACTS` (`Exempt`), `DRY_RUN_REFUSALS`, `GIT_SPAN_SITES`, `MIGRATE_SPAN_SITES`.

One existing test assertion changed: flow52's "exactly one exempt row" count now names the two exempt codes.

The embedded guides (QUICKSTART, MIGRATING) and two help texts changed, so an installed skill picks them up on the next `jigc setup`.

### notes

**Commits on `fix/rc24-tier1` (tree clean, nothing pushed, no branch touched):**
1. `a88f71cc fix(validate): a doc a task has both staged and bound is probed at its staged bytes only`
2. `e3a6ba58 fix(finalize): a linked worktree the user made commits code only`

**Gate evidence.** `dev/gate` was run in full once per commit. a88f71cc: `tests   passed=4499 failed=0  (over 17 test binaries)`, `GATE: PASS`. e3a6ba58: `tests   passed=4513 failed=0  (over 17 test binaries)`, `GATE: PASS`.

**What was fixed (e3a6ba58).** From a user-made linked worktree an ordinary task now commits code only.
- A doc that promotes is refused at every `jigc doc` write leaf, before any store read or staging.
- A doc staged some other way is refused at `task finalize`, `--dry-run`, `task validate` and bare `jigc start`, all with `finalize.linked-worktree-doc` keyed at the doc's home.
- A doc-only workflow and `jigc migrate` refuse before they mint. Driven first on rc.24: `migrate` minted under `<worktree>/.jigc/tasks/`, where `task list` could not see it.
- Mint, resume and orientation state the rule in a `checkout:` paragraph; `doc show`, `doc list` and `validate` print one stderr note.
- Code-only tasks and fan-out sub-tasks are unchanged. C1–C4 are regression arms in `crates/cli/tests/linked_worktree_doc_home.rs` (registered in `groups/g_finalize.rs`; 14 arms, 12 red with the predicate disabled).

**What was fixed (a88f71cc) — a second finding, not in the brief.** Copy-in binds the workflow's role, so an edited committed doc sat on both the staged and the bound surface of `engine::target_surface::enumerate_target_surface`. The old committed anchor stayed on the task surface, and the floor's own route could not clear it. This reproduces on rc.24 from the main checkout. Tests: an engine unit test and `doc_code_gate::an_in_task_citation_repair_clears_the_floor_under_a_role_binding_workflow`, both proven red without the fix.

**Dangling-anchor cell: reaches exit 0** with shipped commands, on both level and one-ahead branches: stash here, `cd` to main, merge, mint, pop, repair, finalize — one commit — then discard. The emitted commands and exits are in `routes_driven`.

**Decisions I made inside the ruling, for the orchestrator to record or overrule:**
- Mint refusals key at the linked worktree's path, because no doc is named yet. Precedent: `repo.operation-in-progress` at a fan-out worktree.
- Mint refusals use the findings envelope, since the code is listed under a target form.
- The backstop route is chosen by asking `decide_base_repin` over the main checkout's facts.
- The read note and the orientation statement are suppressed in jigc's own fan-out worktrees, so sub-agent surfaces do not move.
- The relocation route names `--workflow <the task's own>`, because bare `jigc start "<intent>"` composes the router and mints nothing.

**Process note.** I parked the guard's six source files with `git stash` while a88f71cc gated alone, then popped them. The stash list is empty.

**Files:**
- `crates/cli/src/render.rs`
- `crates/cli/src/task.rs`
- `crates/cli/src/doc.rs`
- `crates/cli/src/start.rs`
- `crates/cli/src/migrate.rs`
- `crates/cli/src/cli.rs`
- `crates/cli/src/pack.rs`
- `crates/engine/src/target_surface.rs`
- `crates/cli/tests/linked_worktree_doc_home.rs`
- `crates/cli/tests/doc_code_gate.rs`
- Drive scripts and outputs: `<scratch>/fix-lw/` (`d4.sh`, `d8.sh`, `d9.sh` and their `.out` files; `gate-A.log`, `gate-B2.log`).

### routes_driven

- All routes below were lifted from the emitted bytes and run verbatim on `target/release/jigc` built from the committed tree, in `dev/jigc-rig` corpora with a `git worktree add -b feature` beside the repo. The same routes run in `linked_worktree_doc_home.rs` on the debug binary.
- Write door, nothing staged: `jigc doc set-slot vision:vision#thesis … --task mixed-change` from the worktree -> exit 1, `finalize.linked-worktree-doc` at `VISION.md`. Route: `cd <main>` -> exit 0; `jigc start "sharpen the thesis"` -> exit 0. A doc task minted there wrote and finalized -> exit 0, main HEAD carries the doc. `jigc task finalize mixed-change` from the worktree -> exit 0, feature HEAD carries `code.txt` only. Two commits.
- Write door with code staged (the dangling-anchor cell), branches level and feature one commit ahead: finalize from the worktree -> exit 3 `doc-code.symbol-exists`; the repair `jigc doc set-field adr:single-node-cache#cites-code …` -> exit 1 with the guard. Route: `git -C <wt> stash` -> 0; `cd <main>` -> 0; `git -C <main> merge feature` -> 0; `jigc start --workflow single-task "…"` -> 0 (minted); `git -C <main> stash pop --index` -> 0; the same set-field from main -> 0; `jigc task finalize <new>` -> 0, ONE commit carrying `docs/decisions/single-node-cache.md` and `src/lib.rs`; `jigc task discard rename-the-getter --force` -> 0. Both checkouts clean, no tasks left, `jigc validate` exit 0.
- Backstop, doc staged from main: `jigc task validate` / `task finalize --dry-run` / `task finalize` from the worktree -> exit 3 each, one guard row at `VISION.md`; bare `jigc start` -> exit 0 with the same row. Route: `cd <main>` -> 0; `jigc task finalize sharpen-the-thesis` -> 0, the doc lands on main, worktree untouched.
- Backstop where the main checkout would refuse the pin (branch reworded the doc): `jigc task finalize` from the worktree -> exit 3; the route contains no `jigc task finalize` span. Control: finalize from main -> exit 3 `finalize.base-mismatch`. Route: `jigc doc show vision:vision --task …` -> 0; `cd <main>` -> 0; `jigc start "…"` -> 0; `jigc task discard … --force` -> 0. The branch's committed wording is still at its HEAD.
- Doc-only mint: `jigc start --workflow report-inconsistency "a finding"` from the worktree -> exit 1 with the guard, `jigc task list` shows no active tasks. Route: `cd <main>` -> 0; `jigc start --workflow report-inconsistency "a finding from main"` -> 0, `task minted: finding-from-main`.
- Migrate: `jigc migrate NOTES.md --as adr` from the worktree -> exit 1 with the guard, nothing under `<wt>/.jigc/tasks`. After the precondition the route states (branch merged into main): `cd <main>` -> 0; `jigc migrate <abs main>/NOTES.md --as adr` -> 0, `task minted: migrate-adr-notes-d656786e4a5f`.
- Changelog gate promoted to blocking: `jigc task finalize user-facing-change` from the worktree -> exit 3. Route: stash -> 0; `cd <main>` -> 0; merge -> 0; `jigc start --workflow single-task "…"` -> minted; stash pop -> 0; entry recorded and finalized there -> 0, one commit with `CHANGELOG.md` and `code.txt`; `jigc task discard user-facing-change --force` -> 0. The other exit: `jigc config set validation.changelog-recording.gate-granted-unused.severity advisory` -> 0, then finalize from the worktree -> 0.
- Unpromoted changelog advisory route (`cd <main>`, `jigc start --workflow record-change "…"`) -> exit 0. Run in the test suite on the debug binary only, not separately on release.

### docs_changed

- design/storage.md (CLI and git: the rule enforced, its predicate, what it refuses and leaves alone, why a later merge is inside it, the two prices)
- design/finalize.md (1. Preflight: the new bullet; The doc-only arm: the mint refusal)
- design/command-output-contract.md (the `finalize.*` table: the `linked-worktree-doc` row)
- design/surface-contract.md (the second exempt row)
- design/validation.md (Target surface: one enumeration per doc; the rc.24 fix pass registration section)
- design/doc-read-surface.md (the served-from note)
- design/workflow-dialect.md (the `checkout:` paragraph)
- design/write-commands.md (where a write may be typed)
- design/auto-migration.md (the `jigc migrate` door runs from the main checkout)
- crates/cli/guides/QUICKSTART.md and crates/cli/guides/MIGRATING.md (one paragraph each)
- --help: the `jigc doc` group long help and `jigc task finalize`'s long help (one paragraph each)
- NOT touched, owed by the orchestrator: DECISIONS.md and implementation/project-history.md. Owed there: (1) the guard as built, including the choices made inside the ruling — the mint refusals key at the checkout path, `AMBUSH_CONTRACTS` takes an `Exempt` row, the changelog route gained an arm, and the anchor exit travels through `git stash`. (2) A new entry for the second finding fixed in a88f71cc (staged-and-bound double enumeration); it is not in the rc.24 review record.

### left_open

- HUMAN'S CALL — the second commit. a88f71cc fixes a defect outside the handed finding: on rc.24 from the main checkout, under `single-task`, an in-task citation repair could not clear `doc-code.symbol-exists`. I fixed it because the dangling-anchor exit could not reach exit 0 without it. It is a separate, separately gated commit so it can be reverted alone, but reverting it makes the guard's anchor route a dead end under role-binding workflows. Driven: rc.24 red (exit 3 after the repair), `quick-fix` control green, both new tests red without the fix.
- `task-discard.staged-prose` and `uninstall.staged-prose` still route at "land them with `jigc task finalize <id>`". In the backstop state (promoting docs staged, reader in a linked worktree) that finalize now refuses with the guard, whose own route works — two hops, not a dead end. Not converted: `setup.rs` is another fixer's churn area and the state is the rare backstop one. Read, not driven.
- The `doc-code.symbol-exists` route still says "update the citation" from a code-only checkout. Followed there it meets the guard, which prints the working exit (driven). The floor's own wording is the probe's and was not changed.
- The relocation route when the standing worktree's HEAD is detached omits the merge step, because there is no branch to name. Read, not driven.
- `git stash` in the route takes all tracked uncommitted changes, and the route says so ("staged and unstaged"). A pop into a dirty main checkout can conflict; that is git's own refusal. Not driven with a dirty main.
- Bare-repository layout, pinned as driven: from a worktree of a bare repo every door answers "isn't set up", even though the worktree has `.jigc/config` checked out. jigc is unusable in that layout today. Pre-existing; whether to support it is the human's call.
- Law-1 lead from the design, not driven by me: do `jigc rename`, `migrate-corpus` and the milestone record commits say they acted in the main checkout when run from a linked worktree? `setup` and `uninstall` do.
- `(R1, F5)` — the pre-commit hook's nested `jigc validate` asks about the main checkout from a linked worktree. Unchanged; the hook discards stderr, so the new note never reaches a commit's output.
- A sub-task's doc write typed from a user-made (not fan-out) worktree is exempt by the predicate, since its docs land through `milestone finalize` at jigc_home. Read, not driven.
- Ordinary-model doc workflows (`record-decision`, `form-vision`, `record-change`, …) still mint from a linked worktree; they state the rule and refuse at the first write, as ruled. Only the doc-only commit model and `migrate` refuse at the mint.

