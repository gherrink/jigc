# Fixers' reports of the rc.24 fix pass, round 3 — verbatim structured returns (leads, not verified facts)

## 104a7d4b fix(setup): a file jigc replaces is written as a regular file, never through a link

### status

fixed

### gate

tests   passed=4527 failed=0  (over 17 test binaries) — GATE: PASS. Full `dev/gate` (mode full, shared target), third run, on the exact tree committed as 104a7d4b; fmt/clippy/build/test/doctest all exit 0. Run 1 was GATE: FAIL (step: test), passed=4525 failed=1: `jigc::g_milestone task_area_writer_registry::every_area_join_name_is_a_registry_member_or_a_counted_remainder` — a counted registry of `.join(…)` sites, two of which the fix removed; rows corrected. Run 2 was GATE: PASS at 4526; I then made one more functional change (the pre-image skip, see notes) and re-gated. Nothing was committed past a red gate.

### class

Mechanism: a writer that REPLACES a file opened its destination with `fs::write`, which follows whatever is at the path.

HANDED: 4 paths at `jigc setup` (leaf symlink / non-regular), `--force`, the unborn refusal's commit route, and the stamp writer at `task finalize` (2 cells: hand-edited, symlink).

DERIVED, by axis:

A. Replacing install members (production table, `InstallWriter::Replaces`): 5, not 4. The fifth is the adapter guide artifact. All converted.
- `.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`: refuse before the first write under `setup.write-bootstrap` / `setup.version-stamp` / `setup.init-project-layer` / `setup.compose-marker`.
- Guide: a link at its own path is answered by its existing ownership oracle (left alone, `adapter-guide.user-modified`, exit 0); a link on the way to it refuses under `setup.write-guide`.

B. Path positions: leaf AND every directory on the way below the checkout root. Ancestors were not in the brief. Driven on the parent tree: `.jigc -> ../store` had `../store/AGENT.md` regenerated, then exit 1 at `git add` ("beyond a symbolic link") with a route nothing could satisfy; `.claude/skills -> shared` had the guide written into the shared dir, same exit 1. Both converted; neither was an exit-0 success before, so no success became a refusal.

C. Occupant shapes (all converted, all driven): live link with target untracked / gitignored / outside the repository; dangling link; directory; FIFO.

D. The link's own git state (all converted): committed (the loss cell); uncommitted on a born HEAD (was `setup.dirty-install-path` with two dead route arms); untracked on an unborn HEAD (R1F1's cell 26, code moved); the link itself gitignored (driven on the release binary only, not a test cell).

E. Doors that reach a replacing writer: `jigc setup` (5 writers) and `jigc task finalize`'s two stage functions through one `refresh_version_stamp`. Both converted. `setup::write_version_stamp` is now private, so no other door can reach the unconditional write.

F. Finalize occupant shapes at `.jigc/version`: hand-edited prose, link (committed, uncommitted, target holding a stamp-shaped line), directory. All leave-and-say-so. The directory cell was NOT in the brief and was found by driving: `--dry-run` forecast a landed commit, then the committing run refused on a pre-image read.

G. Consumers of the stamp oracle: setup's dirty guard (now asks without following a link), finalize's stage, its forecast, its manifest prediction, its pre-image capture.

OUT, with reasons:
- Preserving members (`CLAUDE.md`, settings file, root `.gitignore`, `pre-commit` hook): a merge through a link changes the target by what it would change in a regular file; `.jigc/.gitignore` already refuses a link.
- Fan-out, amend and doc-only finalize arms: they neither write nor stage the stamp (read from the code).
- `record_install_footprint` under `.jigc/state/`: workbench-internal, not an install member.
- Readers (`read_version_stamp`): read-only.

DIFFERENCE FROM HANDED: +1 replacing member (guide), +1 path axis (directories on the way), +2 occupant shapes at setup (directory, FIFO), +1 finalize shape (directory), +1 git state (ignored link).

### pinned_surfaces_moved

No frozen schema hash, manifest, pinned `--format json` key, `contract-version`, compose golden or NEW finding code moved. What did move:
1. `setup.version-stamp` gains a second producer: an ADVISORY in `jigc task finalize`'s landed envelope and `--dry-run` forecast (`key.target: null`, existing `findings` array). It was setup-only and blocking-only.
2. `setup_install_pathspec_guard` cell 26: an untracked link on an unborn HEAD now refuses under `setup.write-bootstrap`, not `setup.dirty-install-path`.
3. Same suite, cell 22: each replacing row of the pinned install table now carries its refusal code.
4. `task_area_writer_registry` counted remainder: adapter.rs 1 -> 0, setup.rs 5 -> 4.
5. Five suites' stale-stamp fixtures (`config_layer_preimage`, `flow52_acceptance`, `flow53_acceptance`, `rejection_frame_outcome`, `version_stamp_rollback`) now use `jigc-version: 0.0.0-…`; the bare bytes they used are no longer refreshed by finalize.
6. `jigc setup --help` text.
7. Internal API of the `cli` lib: `InstallWriter::Replaces` is now `Replaces { refusal: &'static str }`; `setup::write_version_stamp` is private; new module `cli::regular_file`.
8. Behaviour outside the handed doors: `jigc uninstall` and `jigc upgrade` read the same guide oracle, so a symlinked guide is now treated as the user's (uninstall leaves it with the existing advisory instead of removing the link; `--force` still removes it). Read from the code, not driven.

### notes

COMMIT: 104a7d4b `fix(setup): a file jigc replaces is written as a regular file, never through a link`, on `fix/rc24-tier1` (confirmed before starting and before committing). 22 files, staged by name. Tree clean. Not pushed, no branch switched.

GATE: `tests   passed=4527 failed=0  (over 17 test binaries)` · `GATE: PASS`.

WHAT WAS FIXED: `jigc setup` no longer writes through a symlink (or over any non-regular entry) at a path it replaces, and `jigc task finalize` no longer regenerates a `.jigc/version` that is not jigc's own stamp.

RED FIRST: the new suite ran against the unfixed tree with 7 of 9 cells failing for the defect itself — `jigc setup` exit 0 "adapter installed" over a committed link, `--dry-run` forecasting `modified .jigc/version` over hand-edited prose — and the two controls green.

HOW IT IS BUILT
- `crates/cli/src/regular_file.rs` (new): `blocker(root, relative)` asks, without following a link, what stands at the path or on the way to it; `replace(root, relative, bytes)` asks again, then opens with `O_NOFOLLOW` and checks the open handle. The promote sink's `copy_regular` now shares that open.
- `crates/cli/src/setup.rs`: `replaced_path_refusal` runs before the first write and before the dirty gate, over every `InstallWriter::Replaces { refusal }` member of the production table. `--force` is not read there. One finding names every blocked path; its code is the first blocked member's.
- Same file: `stamp_standing` / `refresh_version_stamp` / `stamp_left_finding` are the finalize door's oracle, write and advisory. `write_version_stamp` is private, which is the fence against a future door calling the unconditional write.
- `crates/cli/src/task.rs`: the stage writes and stages the stamp only where it is jigc's; the door pushes the advisory into the report that both the forecast and the landed envelope render; no pre-image is captured for a stamp that will not be written.

THE CHOICE I WAS ASKED TO MAKE — leave-the-file-and-say-so at `task finalize`, not refuse. Why: the stamp is provenance and the only check that reads it is advisory, so a refusal would block a task's whole commit over a file the task did not write, and would put the finalizing agent in front of a human's file with a route to move it. Leaving it loses nothing: no write, no stage, the commit lands, one advisory with a route. Driven both ways it matters: prose, link and directory all land at exit 0 with the bytes intact.

THREE THINGS DRIVING THE ROUTES CHANGED
1. The first route I wrote ("remove the link, re-run") failed as printed for a COMMITTED link: the re-run exited 1 under `setup.dirty-install-path`. The route now says to commit the removal; driven to exit 0 in one re-run.
2. A directory at `.jigc/version` made the forecast and the door disagree (forecast exit 0, commit exit 1). Fixed by not capturing a pre-image for a stamp that will not be rewritten. This also changes the fan-out arms in that one shape (they no longer fail on the capture); not driven there.
3. I did not print a "replace the link with your own regular file" arm, although the brief names it as an example: at three of the four paths that act draws the dirty-install refusal on the re-run. Removing the entry is the one act that always works.

TESTS: new suite `crates/cli/tests/replacing_writers_never_follow.rs` (10 cells, registered in `groups/g_migrate.rs`). Cell 1 iterates the production table and panics on a replacing member with no plant or no pinned code. Four unit tests in `regular_file.rs`.

HONEST BOUNDS: the uninstall and upgrade consequences of the guide oracle change, the `MigrationFixed` arm and the fan-out arms are read from the code, not driven. An intermediate drive ran on a release binary one comment-only edit behind; the final drive was repeated on the rebuilt binary.

### routes_driven

- Binary: `target/release/jigc` (1.0.0-rc.24 + this tree), rebuilt after the last functional change and re-driven; every cell in its own `dev/jigc-rig <state> --binary target/release/jigc` root.
- SETUP REFUSAL, committed link at `.jigc/AGENT.md`, target untracked: `jigc setup` -> exit 1 `setup.write-bootstrap`; `jigc setup --force` -> exit 1, same finding; target mark intact, link still a link, `CLAUDE.md` absent. Route printed: "remove the link at `.jigc/AGENT.md` — committing the removal where git tracks the entry — then re-run `jigc setup`: it writes its own regular file there and never touches what a link pointed at. `--force` does not change this: it consents to replacing a file, not to following one".
- Route followed (committed link): `git rm -q -- .jigc/AGENT.md` -> exit 0; `git commit -q -m ...` -> exit 0; `jigc setup` -> exit 0, install commit made, regular file at the path, target mark intact, status `?? kept/` only.
- Route followed (untracked link at `.jigc/version`): `jigc setup --format json` -> exit 1, envelope `schema_version: 3`, `code: setup.version-stamp`, `key.target: null`; `rm .jigc/version` -> exit 0; `jigc setup` -> exit 0, stamp `jigc-version: 1.0.0-rc.24`, target mark intact.
- Committed link at `packs.yaml`, target gitignored (`git status` empty): `jigc setup` -> exit 1 `setup.compose-marker`; target comment intact.
- Linked `.jigc` (out of the repository): `jigc setup` -> exit 1 `setup.write-bootstrap`, message "`.jigc` is a symbolic link on the way to `.jigc/AGENT.md`", target intact, `CLAUDE.md` absent. Route "replace the link at `.jigc` with a real directory": `rm .jigc && mkdir .jigc` -> exit 0; `jigc setup` -> exit 0.
- Directory at `.jigc/AGENT.md`: `jigc setup` -> exit 1; route "move the directory at `.jigc/AGENT.md` out of the way": `mv` -> exit 0; `jigc setup` -> exit 0.
- Laundering route (unborn HEAD, untracked link): `jigc setup` -> exit 1 `setup.write-bootstrap`; commit the link; `jigc setup` -> exit 1 again; target mark intact throughout.
- Link itself gitignored: `jigc setup` -> exit 1; `jigc setup --force` -> exit 1; target intact.
- Guide leaf link: `jigc setup` -> exit 0, advisory `adapter-guide.user-modified`, target intact, link still a link.
- FINALIZE, hand-edited uncommitted `.jigc/version`: `jigc task finalize <id> --dry-run` -> exit 0, `.jigc/version` listed left-out, advisory `setup.version-stamp`; `jigc task finalize <id>` -> exit 0, commit holds `code.txt` only, prose byte-identical, status ` M .jigc/version`. Route printed: "move the file at `.jigc/version` out of the way, then run `jigc setup` — it writes jigc's stamp there again; or leave it, and jigc keeps leaving it alone and saying so". Followed: `mv .jigc/version version-notes.txt` -> exit 0; `jigc setup` -> exit 0, stamp restored, notes kept.
- FINALIZE, committed link at `.jigc/version`: `jigc task finalize <id> --format json` -> exit 0, one JSON document, `findings` carries `setup.version-stamp` at severity `advisory`, target intact, link not swept. Route followed: `rm .jigc/version` -> exit 0; `jigc setup` -> exit 0, regular stamp, status `?? kept/`.
- FINALIZE, directory at `.jigc/version`: `--dry-run` -> exit 0 with the advisory; `jigc task finalize <id>` -> exit 0, landed, directory contents intact. On the build before the pre-image skip this same run exited 1 ("could not read the pre-finalize `.jigc/version` ... Is a directory").
- FINALIZE control, stale jigc-shaped stamp: `jigc task finalize <id>` -> exit 0, stamp refreshed and in the commit, no stamp finding.
- Not driven: a FIFO at `.jigc/version` at the finalize door (see left_open); the `MigrationFixed` finalize arm (shares the one stamp function, covered by reading, not by a run).

### docs_changed

- design/assistant-adapter.md — The install commit's pathspec: a new paragraph stating the regular-file rule, its codes, the `--force` and route rules, the guide's oracle answer and why preserving members are not asked
- design/storage.md — Store provenance: a refresh rewrites only a stamp that is jigc's to write; the finalize leave-and-say-so rule and why it is not a refusal
- design/finalize.md — 5. Stage: the stamp is staged only where it is jigc's to write
- design/validation.md — the `setup.dirty-install-path` row: a link is not that finding's subject and refuses ahead of it
- design/project-setup.md — Idempotency & irreversibility: one sentence on the regular-file rule
- design/command-output-contract.md — the declared singleton exception: `setup.version-stamp`'s second producer
- crates/cli/src/cli.rs — `jigc setup --help` long description and the `--force` flag help
- crates/engine/src/finding.rs — `is_declared_singleton` doc comment (comment only)
- Doc comments in crates/cli/src/setup.rs, adapter.rs, task.rs and the new crates/cli/src/regular_file.rs
- NOT touched: DECISIONS.md, implementation/project-history.md, implementation/decisions-pending.md, the two guides

### left_open

- OWED IN DECISIONS.md (I wrote none of it): (a) finalize is leave-and-say-so, not refuse, with the reason; (b) directories on the way are in the rule; (c) the guide's leaf link is answered by its oracle at exit 0 while a link on the way to it refuses; (d) the setup route says to commit the removal and offers no 'put your own file there' arm; (e) the pinned cells in pinned_surfaces_moved.
- PRE-EXISTING WEDGE in the dirty-install guard, found by driving, not fixed: a tracked install path deleted before the run passes the pre-write gate (absent, so not a candidate), the install is written, and the commit-time backstop then refuses over the path jigc just wrote, with a route pointing at jigc's own file; a second run completes. My route avoids it by committing the removal. Closing it means changing the settled M51 predicate (a path absent with index == HEAD carries nothing to lose) — the human's call.
- A linked `.claude/skills` (a plausible shared-skills setup) cannot take the install at all: exit 1 before this fix (at `git add`), exit 1 after (pre-write, `setup.write-guide`). Making that an advisory skip of the guide would turn a refusal into a success and needs new advisory wording; not built.
- The guide advisory's wording ("no longer carries the bytes jigc wrote ... rather than clobber your edits") is loose for a link, as it already was for a file jigc never wrote. Not reworded.
- FIFO at `.jigc/version`: the finalize door no longer opens it for a pre-image or a write, but `read_version_stamp` (the `store-version.binary-mismatch` reader) still does a plain read, which would block on a FIFO at any door that runs the store sweep. Reasoned from the code, not driven; a reader, outside this class.
- A write-time link (one that appears between the door's question and the write) fails the write under the member's code with the existing "ensure … is writable" route, which is imprecise for a link. Reachable only by a race; the error text names the link.
- The two codes are spelled twice: once in the install table, once in `write_install_span`'s error mapping. The new suite pins the table against the expected codes; nothing fences the two spellings against each other.
- `jigc milestone finalize` stages neither the stamp nor writes it (read from the code); a hand-edited `.jigc/version` in the main checkout is untouched there. Not driven.
- STILL UNBUILT, not this finding: the 2026-10-04 ruling that a gitignored regular file at a replaced install path refuses unless it is jigc's own content. `install_candidates` still drops an ignored path. My check covers an ignored LINK only.

## ef9456b5 fix(setup): an install path is asked what it holds where git status cannot say

### status

fixed

### gate

tests   passed=4535 failed=0  (over 17 test binaries) — GATE: PASS. Full `dev/gate` on the exact tree committed as ef9456b5; fmt, clippy, build, test and doctest all exit 0. I stopped three earlier gate runs myself to make wording corrections, so none of them reported; nothing was committed past a red or partial gate. The test step took 520 s this run (nextest summary 513 s, 4 slow tests, all pre-existing), which is close to the 10-minute foreground ceiling.

### class

Mechanism: the dirty-install question was asked through `git status`, and status says nothing about some bytes that are in no commit. Handed: 1 instance (an ignored `.jigc/AGENT.md`) and 1 mechanism (`git_path_ignored` dropping candidates). Real, by axis:

A. Ways the question could not see the bytes — 3, not 1:
- Ignored file: CONVERTED.
- Tracked file whose index entry is flagged assume-unchanged or skip-worktree: CONVERTED. Not in the handed finding. Driven on the pre-fix release binary at 104a7d4b: under assume-unchanged, `.jigc/AGENT.md` prose was destroyed at exit 0 with `findings: []`; under skip-worktree it was destroyed and the run then failed at `setup.install-commit`; a hidden edit in an assume-unchanged `CLAUDE.md` rode a first install's commit at exit 0.
- git cannot answer at all (`InstallSubject::Unknown`): OUT, left open. It needs a choice that is the human's.

B. Install members — 10:
- Ignored, 4 replacing members (`.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`): CONVERTED at both HEADs.
- Ignored guide: answered earlier by its own digest oracle (left byte-identical, advisory), unchanged.
- Ignored, 5 merged-into members (`CLAUDE.md`, settings, both `.gitignore`s, hook): OUT as decided; each driven at both HEADs, exit 0 with the adopter's bytes still in the file.
- Flagged: all 7 members a first install leaves tracked refuse under each flag (3 merged-into, 4 replaced); the guide is left alone with its advisory.
- Flagged hook: OUT. Its home is resolved by the install, so the pre-write ask cannot name it.

C. HEAD shapes — 2, both converted. On an unborn HEAD only the ignored class can occur.

D. Doors with a replacing writer — 2:
- `jigc setup`: converted.
- `jigc task finalize`'s stamp refresh: already asks the file with no git query since 104a7d4b, so ignored and flagged are covered there. Verified by reading `stamp_standing`, not driven.

E. Other `check-ignore` callers — 2 (`migrate_corpus::stageable`, `task::path_is_ignored`): OUT. Both are staging filters that decide a pathspec, not whether a file is replaced.

F. Route arms — 5, all driven as printed: born ignored, born flagged, born mixed, unborn all-ignored, unborn mixed.

Difference from handed: +1 status-blind class (index flags, covering merged-into members too), +1 class left open (git unavailable), +1 mechanism added (the footprint record now carries ignored replaced paths across upgrades), +5 route arms.

### pinned_surfaces_moved

No frozen schema hash, manifest, pinned `--format json` key, `contract-version`, compose golden or finding code moved. `setup.dirty-install-path` is reused, and with no status-blind path listed its message and route bytes are unchanged (an engine test pins that).

What did move:
1. The installed guide's body digest, because QUICKSTART.md bytes changed. It is computed at build and no golden pins it.
2. Cell 22 of the dirty-install suite: each replacing row gains its `own:` oracle.
3. `replacing_writers_never_follow.rs`: its table check matches on the refusal code instead of whole-variant equality.
4. The `git_span_aim` registry row for `setup_dirty_install_finding`: reason text only, same row count.
5. `jigc setup --help` text.
6. Published-crate API: `jigc_engine::finalize::setup_dirty_install_finding` takes a fourth argument, `Option<SetupUnseen>`, and `SetupUnseen` is a new pub struct. In `cli` (doc-hidden, no API): `InstallWriter::Replaces` gains `own`, and `OwnContent` is a new pub enum.
7. `.jigc/state/setup-install-footprint` now also holds lines for ignored replaced paths. Same file, same line format, no new store.

### notes

COMMIT: ef9456b5 `fix(setup): an install path is asked what it holds where git status cannot say`, on `fix/rc24-tier1` (confirmed before starting and before committing). 12 files staged by name, tree clean, not pushed, no branch switched.

GATE: `tests   passed=4535 failed=0  (over 17 test binaries)` · `GATE: PASS`, on the committed tree.

WHAT WAS FIXED: `jigc setup` no longer destroys a file git ignores at a path its writer replaces. It refuses before the first write with the existing `setup.dirty-install-path`, names the path, and routes at an act that works on an ignored file. A repository that ignores jigc's install paths and never edited them still re-runs `setup` at exit 0, across an upgrade too. `--force` names the path it replaced. Ignored merged-into members are unchanged.

RED FIRST: driven on the pre-fix release binary at 104a7d4b — ignored `.jigc/AGENT.md` on a born and an unborn HEAD, exit 0, `findings: []`, mark gone and in no object. As tests: with the new question neutralised by one line, the 6 new cells failed (exit 0, "adapter installed") while the suite's 30 existing cells passed; the fix was restored byte-identical (cmp) and all 36 pass.

HOW IT IS BUILT (crates/cli/src/setup.rs, crates/engine/src/finalize.rs):
- `unseen_by_status` asks every present install member that status did not name, in two git calls (`ls-files -s`, `hash-object`, which applies the path's clean filters; checked under autocrlf). Tracked and different is dirty at every member. Untracked, so ignored, is a subject only at a replacing member.
- `InstallWriter::Replaces` gains `own: OwnContent`, with no default, so a replacing member cannot be declared without its oracle. Stamp shape, empty `.gitkeep` and guide digest are build-independent. `.jigc/AGENT.md` and `packs.yaml` are recognised when this run's write would leave them byte-identical; the `packs.yaml` writer and its oracle share one rendering function.
- Upgrade: `record_ignored_footprint` upserts the hash of every ignored replaced file into the existing install footprint record after the write span, under any subject. The existing subtraction then exempts a path whose bytes still hash to it.
- The engine words the refusal per class. Flag clearing prints two commands because one `update-index` call clears only the first flag it names (probed on git 2.54).

TESTS: cells 30–36 in crates/cli/tests/setup_install_pathspec_guard.rs (already registered in groups/g_migrate.rs; no new suite file), plus one engine unit test.
- 30 iterates the production table × both HEADs with an ignored plant.
- 31 is the never-edited control with a per-member re-arm.
- 32 is the upgrade, with the older build planted as it leaves the repository: clean, edited, no record, the bound, and the build-independent oracles.
- 33 iterates every tracked member × both flags and runs the printed commands from outside the repository.
- 34 is the flag-over-unchanged-bytes control.
- 35 is the reported `.gitignore` spelling and the mixed routes.
- 36 is the failed-run record.

FOR THE ORCHESTRATOR:
- The class was larger than handed. I converted the index-flag sibling (it reaches merged-into members too) and left the git-unavailable sibling open; both are in `left_open`, the first as a confirm, the second as a decision.
- The published `jigc-engine` API changed (`setup_dirty_install_finding`'s fourth argument, new `SetupUnseen`); release-plz's semver check may surface it.
- The gate's test step ran 520 s here.
- Files: crates/cli/src/setup.rs, crates/engine/src/finalize.rs, crates/cli/tests/setup_install_pathspec_guard.rs, design/validation.md.

### routes_driven

- All on target/release/jigc built from ef9456b5 (cmp-identical to the driven copy), in `dev/jigc-rig --binary` roots, one mktemp repo per cell.
- Born, ignored `.jigc/AGENT.md` with prose: `jigc setup` -> exit 1 `setup.dirty-install-path`, path named and marked `ignored`, mark on disk, HEAD unmoved. Route as printed, 'move the file out of `.jigc/AGENT.md`': mv -> exit 0; `jigc setup` -> exit 0; notes kept; status empty. `jigc setup` again over jigc's own ignored file -> exit 0; record holds the path.
- Same cell with `jigc setup --force` -> exit 0, advisory `setup.forced-install-path` 'over 1 install path(s)' naming `.jigc/AGENT.md`; plain re-run -> exit 0.
- Born, tracked `.jigc/AGENT.md`, skip-worktree, prose, `git status` empty: `jigc setup` -> exit 1, mark on disk. Printed commands run from `/`: `git -C <abs repo> update-index --no-assume-unchanged -- .jigc/AGENT.md` -> exit 0; `git -C <abs repo> update-index --no-skip-worktree -- .jigc/AGENT.md` -> exit 0; status ` M .jigc/AGENT.md`; `git commit -am` -> exit 0; `jigc setup` -> exit 0; mark in history; status empty.
- Born, tracked `.jigc/AGENT.md`, assume-unchanged, prose: `jigc setup` -> exit 1. Same two printed commands -> exit 0 each; `git stash` -> exit 0; `jigc setup` -> exit 0; mark in the stash.
- Born, tracked `CLAUDE.md` (merged into), assume-unchanged, hidden edit: `jigc setup` -> exit 1 naming `CLAUDE.md` as tracked with the change hidden; mark on disk, in no object.
- Unborn, ignored `.jigc/AGENT.md`: `jigc setup` -> exit 1, 0 commits, mark on disk. Route 'move the file(s) out of those path(s), then re-run': mv -> exit 0; `jigc setup` -> exit 0; 1 commit.
- Unborn mixed (ignored `.jigc/AGENT.md`, untracked commented `packs.yaml`, untracked `CLAUDE.md`): `jigc setup` -> exit 1 naming both replaced paths. Commit arm as printed, 'commit the one(s) git does not ignore in one commit with `.gitignore`, `CLAUDE.md` ... and move `.jigc/AGENT.md` out': `git add` -> exit 0; `git commit` -> exit 0; mv; `jigc setup` -> exit 0.
- Born mixed (ignored AGENT.md plus untracked CLAUDE.md), in the suite: `git stash -u` takes CLAUDE.md and not the ignored file; AGENT.md moved out; one re-run -> exit 0.
- Whole `.jigc/` ignored, never edited: three `jigc setup` runs -> exit 0 each; record holds 4 lines.
- Upgrade, real: installed rc.24 binary installs into an ignored `.jigc/`, then this binary -> exit 0 (rc.24's bootstrap body is byte-identical to this tree's).
- Upgrade, two real builds whose bootstrap bodies differ: this build installs with `.jigc/AGENT.md` ignored -> exit 0; the varied build -> exit 0, body regenerated to its wording; again -> exit 0. The varied build was compiled before the last one-sentence message change, so it differs from ef9456b5 by that sentence and the body.
- Same, with a line appended after the first install: varied build -> exit 1 naming `.jigc/AGENT.md`, mark on disk.
- The bound: rc.24 (no record) installs, then the varied build -> exit 1 once; `setup --force` -> exit 0; `setup` -> exit 0.

### docs_changed

- design/validation.md — the `setup.dirty-install-path` row: the predicate asked of the files, both status-blind classes with their driven data, the per-member ownership oracle, the footprint record across upgrades, the stated bound, the new route arms
- design/assistant-adapter.md — The install commit's pathspec: a new paragraph separating the pathspec's filter from the question's
- design/project-setup.md — Idempotency & irreversibility: the re-run stays a no-op for ignored install paths, across an upgrade too
- design/surface-contract.md — The carryover gate, second member: one sentence
- crates/cli/guides/QUICKSTART.md — 1. `jigc setup`: a new paragraph
- crates/cli/src/cli.rs — `jigc setup --help` long description
- Doc comments in crates/cli/src/setup.rs and crates/engine/src/finalize.rs; the header and cell 9 of crates/cli/tests/setup_install_pathspec_guard.rs; one cell-count cross-reference in crates/cli/tests/flow52_acceptance.rs
- NOT touched: DECISIONS.md, implementation/project-history.md, implementation/decisions-pending.md, crates/cli/guides/MIGRATING.md

### left_open

- DECISION FOR THE HUMAN — git cannot answer (`InstallSubject::Unknown`). Driven on the pre-fix binary and unchanged on ef9456b5: `fresh` rig, tracked-modified `.jigc/AGENT.md` with prose, `jigc setup` with no git on PATH -> the prose is destroyed and the run exits 1 at `setup.install-hook`. The recorded premise ('no commit means no sweep') is silent on replacing writers. Options: (a) refuse before the first write when git cannot answer; (b) ask each present replacing member its ownership oracle and refuse the rest, which would falsely refuse a committed older-build file whose content has since moved; (c) leave it. Not built.
- CONFIRM — I converted the index-flag class (assume-unchanged, skip-worktree), including at merged-into members. The handed decision named ignored bytes only and said merged-into members are unchanged. I read that sentence as being about ignored files, which cannot be swept, and treated a flagged file whose bytes differ from HEAD as dirty under the predicate the product already states. If the human wants merged-into members out of the flag class, it is one filter in `unseen_by_status` plus cell 33's expected list.
- The stated bound: a build older than this rule installed into an ignored path, and the first fixed build that runs has a different generated body -> one refusal, cleared by `--force` or deleting the file. rc.24's body is byte-identical to this tree's, so rc.24 to the next release is clean, provided the bootstrap body does not change before that release.
- The flagged hook is not asked: its home is resolved by the install, so only the commit-time backstop sees it, after the write. A hidden edit in an assume-unchanged in-worktree `pre-commit` can still ride the install commit. Not driven.
- A status-visible replaced path holding jigc's own bytes (for example an untracked `.jigc/AGENT.md` byte-identical to this build's body) still refuses, as before. The new oracle could exempt it later, turning refusals into successes, but for `packs.yaml` that would let an uncommitted canonical-form edit ride the install commit, so it is not a mechanical widening.
- Pre-existing, seen, not touched: a skip-worktree install path makes `git add` refuse on any run that reaches the commit (`setup.install-commit`, routed at git's own words).
- Edge not covered: a run that fails mid-span after rewriting a flagged tracked path records nothing for it, because the failed-run record is built from `git status`; a re-run would refuse over jigc's own bytes there.
- OWED BY THE ORCHESTRATOR: DECISIONS.md — one line that the ignored sibling landed as ef9456b5, that the index-flag class was converted with it, that the ownership oracle is per replacing member with the footprint record carrying provenance across upgrades, and the stated bound. implementation/project-history.md — the fix-pass span. implementation/decisions-pending.md — a trigger row for the `Unknown` arm if the human defers it.

## 4fa1c0f5 fix(uninstall): the teardown never starts the invocation log it blocks over

### status

fixed

### gate

dev/gate (full, foreground-equivalent: backgrounded, its own exit code read from the file, never through a pipe) — fmt exit=0 · clippy exit=0 · build exit=0 · test exit=0 · doctest exit=0 · `tests   passed=4543 failed=0  (over 17 test binaries)` · `GATE: PASS` · process exit 0. Hygiene: denylist clean, gitleaks clean.

### class

Handed: one site (a refused `jigc uninstall`, one reason, re-creating the log). Derived axis: every way an `uninstall` invocation ends, times every shape of "the log is not there".

Mechanism census:
- Writers of the log: 1 (`invocation_log::append_record`), reached from 1 call site (`main`). Converted — the write is now a per-verb policy, `invocation_log::LogWrite { Mint, AppendOnly }`.
- Doors that hold the log as a BLOCKING subject: 1 (`jigc uninstall`). `setup::own_transient_paths` has six own-file rows; five are `Disposable` (go with the tree, named) and only the log is `SoleCopy`, which an existing unit test already fences as "the one own file the teardown does not treat as rebuildable". So no second file shares the "teardown writes what the teardown refuses over" mechanism.
- The other five `DESTROYING_DOORS` members (milestone provision/discard/finalize, task discard/finalize): out — none removes `.jigc/logs/`, none holds the log as a subject.

Invocation endings, all converted (19 driven cells against the 1 handed):
- Refusing: 4 codes read off `UNINSTALL_DOOR.codes` (dirty-worktree, staged-prose, foreign-bytes, untracked-workbench-file) x 2 absent shapes (file moved out with `logs/` standing; whole `logs/` deleted) = 8 cells, each driven twice so the second run proves it is refused for the same reason and not for the log.
- Answered by clap before the door runs: 8 cells — `uninstall --help`, `uninstall -h`, `help uninstall`, `--format json uninstall --help`, `uninstall extra`, `uninstall --bogus`, `uninstall --format nope`, and a non-UTF-8 argument. This is the part the finding did not name: reading the teardown's help between two attempts also put the log back.
- Landing, and landing after a refusal (the drive the finding names): converted; subsumes the earlier `(R9, F3)` "never mints `.jigc/`" case for this verb.
- Control cells kept true: a log that is already there still takes a refused teardown's record (exactly one line, argv `["uninstall"]`, exit 1, the finding code); every other verb, a clap-answered one included, still mints the log.
- Writer-level shapes (unit): `logs/` gone, `logs/` empty, dangling symlink at the log's path — nothing created in any; the open carries no `create`, so there is no check-then-write.

Class fences: `the_teardown_is_the_one_leaf_that_never_starts_the_log` iterates `BEHALF_DOORS` (total over the clap tree) and holds the append-only set to exactly the leaf clap parses `jigc uninstall` into; `a_rejected_argv_reaches_the_leaf_whose_help_it_asks_for` iterates every leaf's `--help`; the acceptance's `plant_the_refusal` panics on a fifth `UNINSTALL_DOOR` code until it is given a cell.

How the verb is identified: off the parsed command (`Command::leaf`) when clap parsed, and off the node the argv reached when clap answered instead (`cli::rejected_argv_node`, the walk `parent_path` already made, now shared). The append-only leaf is read off `milestone::UNINSTALL_DOOR.verb`, not a retyped name.

Out, with reason: every other verb (decided: "logs as before"); the pack-default `knobs.yaml` comments (pack files, see left_open).

### pinned_surfaces_moved

No finding code, `--format json` key, `contract-version`, frozen schema hash, schema manifest or compose golden moved; no new store, knob or doctype. The invocation log's record shape (eight keys) is unchanged. Two things did move, neither pinned by a golden: (1) the installed guide's `jigc-body-blake3` digest, because one QUICKSTART.md sentence changed (it is `include_str!`'d into the installed skill; six earlier commits of this pass already moved it); (2) `jigc uninstall --help` text. One internal signature changed: `invocation_log::log_invocation` gained a `LogWrite` argument (the `cli` lib is `#![doc(hidden)]`, no API).

### notes

Fixed in one commit on `fix/rc24-tier1`: `4fa1c0f5 fix(uninstall): the teardown never starts the invocation log it blocks over`. Tree clean; nothing pushed, no branch switched, DECISIONS.md and project-history.md untouched.

Gate: `tests   passed=4543 failed=0  (over 17 test binaries)` · `GATE: PASS` · exit 0.

What was fixed: with the `invocation-log` knob on, a `jigc uninstall` invocation — landing, refusing under any of its four codes, or answered by clap (`--help`, `help uninstall`, a usage error) — now appends only to a log that is already there and creates neither the file nor `.jigc/logs/`. Every other verb mints the log as before.

Red first: three new arms in crates/cli/tests/uninstall_workbench_subject.rs (already registered in groups/g_milestone.rs) failed on the unfixed tree for the stated reason — the log was back holding one record, `{"argv":["uninstall"],"exit_code":1,"finding_codes":["uninstall.dirty-worktree"]}`, likewise for the foreign-file drive and for `uninstall --help`. A fourth arm (a log that is there still takes the refused run's record) passed before and after, as the control.

Where the change lives:
- crates/cli/src/invocation_log.rs — `LogWrite { Mint, AppendOnly }`, `LogWrite::for_leaf` (reads the append-only leaf off `milestone::UNINSTALL_DOOR.verb`), and `append_record` opening without `create` under `AppendOnly`.
- crates/cli/src/cli.rs — `rejected_argv_node` (shares `walk_node` with `parent_path`) and the help text.
- crates/cli/src/main.rs — picks the policy from the parsed command, or from the argv node when clap answered.

Tests added: 4 acceptance arms (s–v), 2 unit tests in invocation_log.rs, 2 in cli.rs, 1 help-fence arm in help_truth.rs.

Decision for you: the first `left_open` item. The ruling is applied exactly as decided, and driving it showed where it stops — a refusal whose printed route is another jigc verb re-mints the log, so the operator clears it twice in that order. It is stated in measurement.md item 7 and the help rather than hidden; the remedy is yours to choose.

Drive transcript and script are in the session scratchpad (`drive.out`, `drive.sh`).

### routes_driven

- Built target/release/jigc (jigc 1.0.0-rc.24) from the fixed tree; every cell ran in a fresh `dev/jigc-rig fresh --binary target/release/jigc` rig with `jigc config set invocation-log true` and `git add -- .jigc/config`. No command newly refuses in this fix; the routes below are the existing ones, run as printed.
- A (the decided drive). `jigc doc list` -> exit 0, log present (2 lines). `mv .jigc/logs/invocations.jsonl <outside>` -> exit 0, log absent. Plant `.jigc/state/notes.txt`. `jigc uninstall` -> exit 1 `blocking · uninstall.foreign-bytes`, route verbatim: "move what you need out of the paths above, or delete the ones you do not (`rm -r` takes them — jigc has no verb that clears `.jigc/displaced/`), then re-run `jigc uninstall`; or, once you have confirmed they hold nothing you need, `jigc uninstall --force` deletes them with the install" -> LOG ABSENT afterwards.
- A, between attempts: `jigc uninstall --help` -> exit 0, LOG ABSENT · `jigc help uninstall` -> exit 0, LOG ABSENT · `jigc uninstall extra` -> exit 2, LOG ABSENT · `jigc --format json uninstall` -> exit 1, one finding `uninstall.foreign-bytes`, schema_version 3, key.target null (envelope unchanged), LOG ABSENT.
- A, the route followed as printed: `mv .jigc/state/notes.txt <outside>` -> exit 0; `jigc uninstall` -> exit 0 "repo-local install removed", `.jigc` ABSENT; second `jigc uninstall` -> exit 0 "(nothing to remove — no repo-local jigc install was present)", `.jigc` ABSENT; the moved-out log still holds its 2 records.
- B (control, log present): `jigc doc list` -> exit 0 (1 line); plant `.jigc/state/notes.txt`; `jigc uninstall` -> exit 1; log now 2 lines, last record `{"argv":["uninstall"],"exit_code":1,"finding_codes":["uninstall.foreign-bytes"],...}` — a refused teardown is still recorded where a log exists.
- C (control, other verbs): log absent -> `jigc validate --help` -> exit 0 -> log present, 1 line, argv `["validate","--help"]` — every other verb mints as before.
- D (the bound, driven — see left_open): `jigc start --workflow single-task "probe the guard"` -> exit 0; move the log out; `jigc uninstall` -> exit 1 `uninstall.staged-prose`, LOG ABSENT; its route's `jigc task discard probe-the-guard --force` -> exit 0, and that verb mints the log (1 line); `jigc uninstall` -> exit 1 `uninstall.untracked-workbench-file` over `.jigc/logs/invocations.jsonl`; move it out again; `jigc uninstall` -> exit 0, `.jigc` ABSENT.
- E (the bound, driven): log absent -> `jigc config set invocation-log false` -> exit 0 -> log present, 1 line. Turning the knob off is itself a logged run.

### docs_changed

- design/measurement.md — The in-repo invocation log, item 7: the sentence "A refused teardown is a run like any other and is recorded" is replaced by the rule (an `uninstall` invocation appends only to a log that is already there and creates neither the file nor `logs/`; per verb, not per outcome; how the verb is read) and its stated bound (two refusals route through another jigc verb, which still mints the log; turning the knob off is a logged run).
- design/project-setup.md — Teardown / cleanup (G5), (c): one cross-referencing sentence ("A log moved out stays out across the teardown's own runs") pointing at measurement.md item 7; nothing restated.
- crates/cli/src/cli.rs — `jigc uninstall --help` about: four new sentences stating the rule and that every other jigc verb still starts the log while the knob is on. Fenced by a new arm in crates/cli/tests/help_truth.rs (`uninstall_help_names_every_guard_the_door_runs`).
- crates/cli/guides/QUICKSTART.md — the invocation-log sentence: "One run appends nothing: a `jigc uninstall` that succeeds…" became false (a refused one with no log appends nothing either) and now reads "One verb never starts that log: `jigc uninstall` adds its record only to a log that is already there…".
- Doc-comments: crates/cli/src/invocation_log.rs (module header, `LogWrite`, `log_invocation`, `append_record`), crates/cli/src/cli.rs (`rejected_argv_node`, `walk_node`), crates/cli/src/main.rs (the wrapper).

### left_open

- HUMAN CALL — the bound of the ruling, driven on the release binary (cells D and E). Two of the teardown's four refusals route through ANOTHER jigc verb: `uninstall.staged-prose` -> `jigc task discard <id> --force` / `jigc task finalize <id>`, and `uninstall.dirty-worktree` -> `jigc milestone discard <id> --force`. With the knob still on, that verb mints the log ("every other verb logs as before" is the decided boundary), so an operator who clears the log FIRST and follows one of those routes SECOND is refused once more over the log and has to clear it again. It converges in one extra move. `jigc config set invocation-log false` does not avoid it: the knob is resolved before the verb runs, so that run is logged too. I did not pick a remedy. Options: (a) accept, as now stated in measurement.md item 7 and the help; (b) have the teardown report the log's refusal LAST / together with the others so the operator clears it once, at the end; (c) say in the log refusal's route to clear the log last; (d) make the knob-off `config set` run not log itself.
- DECISIONS.md owes an entry (not touched — the orchestrator's log): the (R9, F5) ruling is applied to refused and clap-answered `uninstall` invocations; the rule is per verb, not per outcome; the bound above; and that the QUICKSTART byte moved again.
- The verb of a clap-REJECTED argv is read by walking the real clap tree, with the same bound `parent_path` has always had: a token is a step only where the tree has a subcommand of that name at the node reached, so `jigc --format uninstall …` (itself a usage error) is read as the teardown. The error is one-sided — a rejected run's record is not minted into an absent log; it can never make a teardown mint. Making the walk skip flag values exactly would mean re-implementing clap's option parsing; not done.
- The pack-default comments in crates/cli/packs/dev/config/knobs.yaml and crates/cli/packs/methodology/config/knobs.yaml (and the fixture copy in pack.rs) still say one record is appended "per `jigc` invocation". Left alone as the (R9, F5) fixer left them: they are pack files. MIGRATING.md:41 says "every jigc run appends one record" while deferring to QUICKSTART for the exception; also left.
- Not driven: an `uninstall` run from inside a fan-out worktree or a subdirectory with the knob on. The rule is derived from the verb and not from the cwd, and `enabled_logs_dir` already resolves jigc_home, so no separate path exists — but no cell proves it.

## 8d9c3afb fix(milestone): a record door with no baseline compares the record against HEAD

### status

fixed

### gate

dev/gate (full, shared target, background, exit code read directly from its own rc file, run on the exact tree committed — the diff checksum taken at gate start equals the one taken before `git add`): fmt ok · clippy ok · build ok · test ok (530s) · doctest ok — `tests   passed=4551 failed=0  (over 17 test binaries)` — `GATE: PASS`. An earlier full run of a near-final tree was red on one test (`tests   passed=4550 failed=1`: `milestone_record_stale_base::stale_base_routes_finalize_without_committing`, a fixture that relied on the adoption this fix closes); the fixture was restaged and the whole gate re-run before the commit.

### class

A door that rewrites a managed doc in place, in a checkout with no file-state key for it, adopting the on-disk bytes through the reconciler's UNKNOWN arm instead of comparing them with what git holds.

Axis as derived:

(a) Consumers of the classifier (`reconcile_committed`): 2 production callers.
- The shared sweep (`validate_task` → `reconcile_committed_store`): converted by (R3, F7).
- The record door (`reconcile_record_preflight`): CONVERTED here.

(b) Call sites of the record preflight: 7, all converted by the one function. The (R3, F7) fixer reported 8; the eighth grep hit is the definition.
- `milestone add-task`
- `milestone add-from-spec`
- `milestone discard`
- sub-task `task discard` (`settle_discarded_sub_task`)
- its `unreadable_record_refusal`
- `milestone finalize`'s hoisted preflight
- the finalize flip (`flip_record_for_finalize`)

(c) Record-writing doors named in the brief:
- `add-task`, `add-from-spec`, `discard`, sub-task `task discard`, finalize flip: converted.
- `milestone create`: OUT for a present file. It mints at a home it found free (`milestone.record-exists`, O_EXCL, and the D-7 non-regular-entry guard), so it has no bytes to compare.
- `milestone provision`: OUT. The brief listed a status flip at provision; driven, provision writes nothing to the record. `execute`, `join` and `list-tasks` likewise read only.

(d) Ways the key is absent: fresh clone, emptied cache, `jigc unmanage <record>` (driven: unmanage accepts a record path). All covered.

(e) Hand-edit shapes: unstaged note, staged note, valid status flip. All covered.

(f) Consumers of the pin-read seam `git_blob_at`: 4 (task sweep, milestone merged gate, store sweep's HEAD lag arm, record door). All four are corrected for a defect found while driving — see notes.

(g) The absent-file cell (a home git holds and the disk does not, no key): 3 doors — `task finalize`, `milestone finalize` (the (R6, D-1) lead) and `milestone create` over a deleted record. All OUT: a different mechanism, and an undecided fork — see left_open.

Difference from the brief: it named one door family and one lead. The real count is 7 call sites over 5 record-rewriting doors, one listed door that does not write (provision), a seam defect reaching 4 consumers including the (R3, F7) backstop, and a lead that is 3 doors wide and not closable without a human choice.

### pinned_surfaces_moved

None. No frozen schema hash, schema-manifest, pinned `--format json` key, `contract-version`, compose golden or registry row moved. No new finding code, store, knob or doctype.

- `git_span_aim`'s row for `record_conflict_block` still covers the new restore span; both are rendered inside that one function, so no row was added.
- The held-baseline conflict presentation is byte-identical.

Text pins that moved:
- `milestone_record_stale_base.rs` is restaged. Its fixture rewrote the record's `base:` on disk, uncommitted, and its header said the preflight 'adopts (no drift block)' with the cache gone — the hole itself. The stale pin is now committed before the clone simulation, as a history rewrite leaves it. Both tests keep their assertions.
- `copy_in_baseline.rs` gains one test.

Not pinned but changed: `record_conflict_block` takes a `RecordWitness`; `git_blob_at`'s answer can now be the raw blob where the file on disk holds exactly that.

### notes

COMMIT: 8d9c3afb `fix(milestone): a record door with no baseline compares the record against HEAD`, on fix/rc24-tier1, one commit, 12 paths staged by name, tree clean, nothing pushed. Branch and clean tree were confirmed before starting.

GATE: `tests   passed=4551 failed=0  (over 17 test binaries)` · `GATE: PASS` (full `dev/gate`, on the exact tree committed).

WHAT WAS FIXED: the record door's reconcile preflight passed the classifier no pin. In a checkout with no file-state key for the record it adopted whatever was on disk, and the op then wrote over it. Now, when there is no key, `reconcile_record_preflight` hands the existing UNKNOWN + TOUCHED backstop the record's blob at HEAD. On-disk bytes that differ raise the door's existing `reconciliation.conflict-block` at exit 1, with nothing adopted and nothing written. Bytes equal to HEAD's (an untouched clone, a pulled edit) are adopted as before. The lookup answers only where there is no key, so the pull-absorb arm stays off at this door and (R3, F4)'s case is not turned into a block; a control test pins that.

EMITTED ROUTE, run verbatim from `/` on the release binary: `git -C <clone> checkout HEAD -- docs/milestone-records/file-findings.md` → exit 0, then the same door → exit 0, at all five doors and for a staged edit. It restores from HEAD rather than the index, because the bare `git checkout -- <record>` puts a `git add`ed edit straight back.

RED FIRST: the new suite ran against the unfixed tree with the class test and the real-clone test failing on the defect (`AddTask/CacheGone/Note: exit Some(0)`, record commit landed). The three controls passed there, as they should — they pin behaviour that must not change.

SECOND DEFECT, found by driving and fixed in the same commit because the new route would otherwise loop:
- The pin was read in its checked-out form only (`git cat-file --filters`). jigc writes files with `\n` endings whatever the checkout converts to, and git calls that unmodified.
- Under `eol=crlf`, a record `add-task` had itself written conflict-blocked the next `add-task` once its key was gone. The route's `git checkout HEAD --` rewrote nothing (checked with plain git), so the block had no exit.
- The (R3, F7) backstop had the same false block on a doc jigc's own finalize had landed. That was driven red in `copy_in_baseline.rs` before the change.
- `git_blob_at` now answers in whichever faithful form the file is in. This reaches all four consumers of the seam, including two the brief did not name. It slightly widens the L1 absorb and the store sweep's lag advisory: a file equal to the raw blob now counts as at the pin.

WHAT THE ORCHESTRATOR SHOULD KNOW:
- The (R6, D-1) lead was driven first, as asked. (R3, F7) did not close it, and I did not close it either. It is three doors wide, loses no bytes, and closing it collides with the printed `jigc unmanage` exit. I judged that a human fork rather than something the decided rule settles; the options are in the first left_open item. This is the item most likely to come back at the re-review.
- `milestone provision` does not write the record; the brief's 'status flips at provision' has no member.
- One standing test was leaning on the hole and is restaged (`milestone_record_stale_base.rs`).
- Adopters will see a new refusal: in a clone, a hand-edited record now blocks the next record-rewriting milestone command, where it was silently re-rendered or committed before.

FILES: crates/cli/src/milestone.rs · crates/cli/src/task.rs · crates/cli/tests/record_door_baseline.rs (new, registered in crates/cli/tests/groups/g_milestone.rs) · crates/cli/tests/copy_in_baseline.rs · crates/cli/tests/milestone_record_stale_base.rs · crates/engine/src/file_state.rs and validate.rs (doc-comments only) · design/reconciliation.md · design/team-ready-state.md · design/findings-channel.md · crates/cli/guides/MIGRATING.md.

Work files (not committed): <scratch>/rec/ — drive.sh, recdoors.sh, d1lead.sh, taskdoor.sh, create-base.sh, crlf.sh, gate2.out.

### routes_driven

- Built `target/release/jigc` from the committed tree. Rig: `dev/jigc-rig committed-singletons --binary target/release/jigc`, then a real `git clone` of $REPO (no `.jigc/state/` at all). A hand line was appended to `docs/milestone-records/file-findings.md` and each door run as the next jigc call. Every route below was run verbatim with cwd `/`.
- `jigc milestone add-task file-findings "bravo files a finding"` → exit 1 `reconciliation.conflict-block`; HEAD unchanged, hand line still on disk, no key recorded. Emitted route `git -C <clone> checkout HEAD -- docs/milestone-records/file-findings.md` → exit 0. Re-run → exit 0, record commit landed, hand line 0 in HEAD.
- `jigc milestone add-from-spec file-findings spec:rate-limit` → exit 1, same finding. Emitted route → exit 0. Re-run → exit 0, two record commits.
- `jigc milestone discard file-findings --force` → exit 1, same finding (before the fix: exit 0 with the hand line committed into the record). Emitted route → exit 0. Re-run → exit 0.
- `jigc task discard alpha-files-a-finding --force` (after `milestone provision` in the clone) → exit 1, same finding, sub-task area intact. Emitted route → exit 0. Re-run → exit 0, record commit landed.
- `jigc milestone finalize file-findings` (provisioned, code staged in the sub-task worktree) → exit 1, same finding, record not flipped. Emitted route → exit 0. Re-run → exit 0, `finalized d6594ae`.
- Staged hand edit (`git add` of the record), `add-task` → exit 1. Emitted route → exit 0, index and worktree both back at HEAD. Re-run → exit 0.
- `eol=crlf` attribute, record written by jigc (LF on disk, `git status` clean), `jigc unmanage <record>`, `add-task` → exit 0. Before the seam fix this was exit 1 with a route that changed nothing.
- Untouched clone, `add-task` → exit 0 (the adoption is unchanged).
- In the suite (`record_door_baseline.rs`): 30 arms (5 doors x 2 ways the key is absent x 3 edit shapes). Each extracts the emitted `git -C …` span, runs it through a real `sh` word split from outside the repository, asserts exit 0 and a clean `git status` for the record, then re-runs the door to exit 0 with its own write in HEAD and the hand line absent.
- Open leads re-driven on the fixed binary (unchanged, see left_open): deleted committed occupant with no baseline → `milestone finalize` exit 0 and `task finalize` exit 0, each replacing `stage-2.md` under its id; `milestone create` over a deleted record → exit 0 in a clone and in a baselined checkout. Baselined control: `task finalize` exit 3 `reconciliation.rename` → its printed `jigc unmanage docs/inconsistencies/stage-2.md` exit 0 → finalize lands over the id.

### docs_changed

- design/reconciliation.md — Baseline adoption: a new paragraph and three bullets on the record door's witness (HEAD), its route, the lookup answering only where there is no key, and `milestone create` being off the arm; the backstop paragraph now says the pin is read in the form the working file is in; the L1 'the pin is the caller's' bullet; Open questions: 'the milestone-record door with no baseline' removed, 'a home git holds and the disk does not, with no baseline' added as unsettled
- design/team-ready-state.md — No-silent-overwrite discipline: the discipline does not depend on the gitignored cache (the no-key rule, the doors, the route, what is still adopted)
- design/findings-channel.md — the L1 absorb table row: one clause ('passes this arm no pin')
- crates/cli/guides/MIGRATING.md — item 8: the milestone record is jigc's alone to write, the block and its route
- Doc-comments kept true: crates/engine/src/file_state.rs (`reconcile_committed`), crates/engine/src/validate.rs (`PinnedBlob`), crates/cli/src/task.rs (`git_blob_at`), crates/cli/src/milestone.rs (`reconcile_record_preflight`, `record_conflict_block`)
- No `--help` text states the old behaviour; none changed.
- OWED BY THE ORCHESTRATOR (not touched) — DECISIONS.md: one line that the record door landed as 8d9c3afb with HEAD as its witness; that `git_blob_at` now reads the pin in either faithful form, which also corrects the (R3, F7) backstop under eol conversion; and that the absent-home cell is an open fork. implementation/project-history.md: the fix-pass span. implementation/decisions-pending.md: a trigger row for the absent-home fork if the human defers it.

### left_open

- HUMAN'S CALL — the absent-home cell (the (R6, D-1) lead), driven on the fixed binary and NOT closed. With no file-state key and a committed doc deleted from the worktree (uncommitted), a created doc lands under that doc's id at exit 0 at three doors: `jigc task finalize`, `jigc milestone finalize` (a join suffix landing on it included), and `jigc milestone create` over a deleted record — the last in a baselined checkout too, since create runs no sweep. No byte is lost (the replaced doc is in history at the parent commit), so it is outside the letter of the decided rule, which is about on-disk bytes. It is a different mechanism from the UNKNOWN arm: the classifier is never asked about an absent file, and the occupancy probes read the worktree only. It needs a choice because `jigc unmanage` is the printed way to confirm a deletion (`reconciliation.rename`'s weak route) and leaves exactly this state — driven: baselined `task finalize` exit 3 → `jigc unmanage <path>` exit 0 → finalize lands over the id. Any refusal keyed on the pin, HEAD or the index would also refuse the doc of a task that followed that route. Options: (1) refuse, and re-gate the rename route so `unmanage` is not offered as the way to land a doc the task stages (the exits become restore, discard, or commit the deletion first); (2) keep it as the stated bound; (3) give the confirmation a home of its own, which is a new store. Stated in design/reconciliation.md → Open questions. The (R6, D-1) fixer declined the same fork for the same reason.
- TIER 2, unchanged — (R3, F4). With a held baseline a pulled edit to the record still conflict-blocks at the record door, and the next unrelated `task finalize` still absorbs it (the store sweep's untouched arm), after which the edited bytes are what jigc last wrote. An uncommitted hand edit swept into an unrelated finalize's commit reaches HEAD the same way. This fix deliberately does not turn either into a block.
- RESIDUAL, same shape as (R3, F7)'s — a record with neither a key nor a blob at HEAD (a hand-written record never committed, an unborn HEAD) or any git failure still adopts: there is nothing to compare against. Reasoned, not driven.
- NOT CONVERTED, no write happens — a record file absent on disk at the non-create doors. The preflight returns early, and the door then errors on the read or answers no-such-milestone. One exotic cell reasoned, not driven: a sub-task `task discard` whose record file is deleted finds no record claiming the task and discards the area without settling the record.
- BOUND of the either-form pin read — a file matching neither the checked-out form nor the raw blob reads as an edit, as before. Under a custom clean/smudge filter whose smudge does not reproduce what jigc wrote, a jigc-written file with a lost key could still false-block. Only `eol=crlf` was driven.
- TIER 3 wording lead from the (R3, F7) report, untouched — the milestone-boundary conflict route ends 'then re-run the join' at the `milestone finalize` door.

## e842342e fix(milestone): the boundary refuses before it lands without work a sub-task worktree holds

### status

fixed

### gate

dev/gate (full, never --quick, unscoped, run in the background, read from its own summary and exit code) on the exact tree committed: fmt ok · clippy ok · build ok · test ok (554s) · doctest ok · `tests   passed=4557 failed=0  (over 17 test binaries)` · `GATE: PASS` · gate exit=0 (log <tmp>). Red first: with crates/cli/src stashed back to the parent commit, the rewritten suite ran 13 tests, 10 failed, each on the defect (e.g. `jigc milestone finalize × a settled sub-task × Commit × Stale … the boundary must refuse; got exit 0`; `jigc milestone finalize × Commit × Live × a cp -R copy … got exit 0`; `kept/area-low cannot be created beside kept/area-low, so the command must name another branch`; `the refusal must print the command that re-links the checkout; spans: []`; `jigc milestone finalize removes a worktree-shaped path, so it owes a refusal code over that subject; got []`).

### class

Class: `jigc milestone finalize` lands a boundary WITHOUT work one of the milestone's own sub-task worktree registrations holds (and its teardown then drops the registration), plus the printed routes of that class that dead-end. Handed: finalize over (a) a commit on a worktree's detached HEAD, (b) a stale registration's staged index; plus two route items (keep-branch name taken; unlinked checkout with no re-link). Real count: 7 refusing cells at finalize (3 nobody had counted), 5 print sites of the keep command, 4 doors for the re-link.

CONVERTED
1. finalize × landed sub-task × commit no ref reaches, live or stale — refuses before landing (both commit arms: squash true and the per-sub-task chain; both formats).
2. finalize × landed sub-task × stale registration's staged paths — refuses.
3. NOT COUNTED BEFORE — finalize × a sub-task settled by `jigc task discard`: the teardown takes the FULL task list, the boundary lands only the live one, so a settled sub-task's commit, its stale staged index AND the staged paths of its LIVE worktree were all dropped at exit 0 (driven on the parent: `alpha-s.rs (staged, in no commit)` … `they are not recoverable`). Refuses. The live-staged cell is a decision I made inside the ruling (see notes).
4. NOT COUNTED BEFORE — finalize × a checkout that stands with its `.git` link gone while the registration's index holds staged paths (the boundary read it as an unreadable path and landed without it). Refuses, with the re-link.
5. NOT COUNTED BEFORE — finalize in a `cp -R` copy × commit in a live worktree the copy never registered: the copy's boundary still feeds from that worktree, so the milestone would be settled without the commit. Refuses; the line says the teardown would leave the worktree standing.
6. finalize × a registration that cannot be read — fail-closed hold, no command falsely pointed at.
7. finalize × a file standing at the path of a stale registration holding staged paths — refuses, says to move it aside, then prints the recipe.
8. The keep command's branch name, at all five places it is printed (provision, discard, uninstall, finalize refusals; the `--force` narration after a drop): chosen against the refs that exist (`kept/<id>-2`, or `kept-<id>` when a branch named `kept` exists).
9. The re-link line for an unlinked checkout, at all four worktree doors (one shared line composer).
10. The landed contribution line: the `— git's registration of it held …, which did not land` clause and its payload are removed, because a registration holding work no longer lands.

OUT, with reason
- `leaked_worktree_remedy`'s unlinked arm: it speaks after a consented teardown, or behind a boundary that now refuses first when the registration holds work — what it still meets un-consented is an empty registration, where re-linking recovers nothing.
- `jigc task validate <sub-task-id>` preview: not added. A new previewed member would have to join `gate_coverage`'s table, whose fragments are pack text — that moves compose goldens, which is on the STOP list. Stated as a bound in the design doc.
- The bytes leg at finalize (a live worktree's unstaged/untracked/ignored bytes): M46's measured ruling, not reopened.
- `jigc milestone join`, the two aborted finalize arms, `.combine-*` dedicated worktrees, `provision` over a live registered worktree, `jigc task discard`/`task finalize`: none drops a sub-task registration or settles a milestone.

Acceptance iterates the class axis in crates/cli/tests/worktree_registration_anchor.rs (15 tests, already registered in groups/g_milestone.rs): finalize × {commit, staged, both, empty} × {stale, live} × 2 commit arms × 2 formats = 32 cells; the same 8-cell grid over a settled sub-task; keep-name × 2 blocking branches × every (door, standing) that prints it + the narration; re-link × 4 doors + 2 controls; recipe × 4 doors × 3 holdings through to a landed boundary; fold × 2 arms × 2 holdings; copy, unreadable and file-in-the-way cells.

### pinned_surfaces_moved

None of the STOP-list set moved: no frozen schema hash, no `--format json` key, no `contract-version`, no compose golden, no new flag, store, knob or doctype. The landed envelope's `committed.sub_tasks[]` key set is still asserted as exactly {code_files, discarded, docs, hash, id, provisioned, worktree_unreadable}.

What did move:
- ONE FINDING CODE ADDED: `milestone.unlanded-work`. The brief said to use the door's existing blocking code for held work; the door had none (`FINALIZE_DOOR.codes` was empty). I did not borrow a sibling's because `flow48_acceptance` and `flow49_acceptance` fence that no blocking identity is shared between two doors, and both sibling codes route to `--force`, which this door does not have. `milestone.zero-contribution` is false whenever a sibling contributes.
- Exit code: `jigc milestone finalize` 0 → 3 in the cells listed under `class`, and only there.
- `FINALIZE_DOOR.codes`: [] → [`milestone.unlanded-work`]; its disposition (`Displace`) and `consent()` (`None`) are unchanged. `flow53_acceptance`'s assertion that a displacing member's code set is empty is re-derived per door.
- `render::SubTaskContribution.stale` (text-only, `#[serde(skip)]`): `Option<StaleRegistration>` → `bool`; `StaleRegistration` removed; the contribution clause `— git's registration of it held …, which did not land` is gone.
- New or changed unpinned text: the finalize refusal; the re-link clause on the provision / discard / uninstall refusal lines; the keep command's branch name now varies with the refs present.
- Test registries: `GIT_SPAN_SITES` +1 row (`unlanded_work_finding`); `worktree_registration_reach.rs`'s leaked-remedy cell `finalize × unlinked` now unstages first, since the boundary refuses over a staged path there.

### notes

Fixed in one commit on `fix/rc24-tier1`: `e842342e fix(milestone): the boundary refuses before it lands without work a sub-task worktree holds`. Tree clean, nothing pushed, no branch switched, nine paths staged by name.

GATE EVIDENCE. `tests   passed=4557 failed=0  (over 17 test binaries)` and `GATE: PASS`, full `dev/gate` on the committed tree, exit 0.

WHAT WAS FIXED. `jigc milestone finalize` now refuses BEFORE it lands — exit 3, nothing committed, the record never flipped from `active`, the registration byte-identical — while git's registration of one of the milestone's own sub-task worktrees holds work the boundary would land without: a commit no ref reaches (stale or live), or staged paths the boundary does not carry (directory gone, `.git` link gone, or a sub-task settled by `jigc task discard`). A live sub-task's staged paths still land as before. One blocking finding per path on the pinned findings envelope, keyed at `.jigc/worktrees/<sub-task-id>`, placed beside the fan-out posture guard (`unlanded_work` / `unlanded_work_finding` in crates/cli/src/milestone.rs).

NO `--force` EXISTS AT THIS DOOR, AND NONE WAS ADDED. The exits are the commands printed on the path's own line followed by the same `finalize`, or `jigc milestone discard`. Beyond the sibling doors' keep command and restore recipe, the finalize refusal prints two of its own: `git -C <worktree> reset --soft <base pin>` (turns a commit into staged paths so it LANDS — with only the keep command the reader could keep the work but not land it) and `git -C <worktree> stash` for a settled sub-task's staged paths.

THE TWO ROUTE ITEMS. (1) The keep command no longer dead-ends: the branch name is chosen against existing refs. (2) An unlinked checkout gets a printed re-link at all four worktree doors. The brief suggested `git worktree repair` and asked me to drive it: driven, it re-points every other worktree the repository has a registration for (in a `cp -R` copy, the source's live worktrees). So the re-link is the hand-written `printf 'gitdir: …' > <path>/.git`, printed only where no `.git` entry exists at all.

THREE THINGS TO RULE ON (details in left_open and pinned_surfaces_moved):
1. A new finding code, `milestone.unlanded-work` — the brief asked for the door's existing code and there was none.
2. The settled-sub-task × live × staged cell refuses — inside the ruling's words, but never put to the human by name.
3. `FINALIZE_DOOR.codes` is no longer empty, so `flow53_acceptance`'s 'a displacing member's code set is empty' assertion was re-derived per door.

EMITTED COMMANDS RUN VERBATIM: every route in `routes_driven`, on the release binary from the committed tree, cwd `/`, all exit 0.

FILES. Code: crates/cli/src/milestone.rs, crates/cli/src/render.rs. Suite: crates/cli/tests/worktree_registration_anchor.rs (existing file, already registered in crates/cli/tests/groups/g_milestone.rs — no new suite file). Adjusted: crates/cli/tests/flow53_acceptance.rs, git_span_aim.rs, text_json_parity_axis.rs, worktree_registration_reach.rs. Docs: design/team-ready-state.md, design/finalize.md. Driver scripts and outputs: <scratch>/fin1/.

### routes_driven

- All on target/release/jigc built from the committed tree, in `dev/jigc-rig fresh --binary target/release/jigc` rigs; each printed span extracted from the refusal and run verbatim through `sh -c` with cwd `/`. 42 exit-0 results, no printed span exited non-zero. Scripts and outputs: <scratch>/fin1/ (routes.sh, routes.final.out, edges.sh, base.out = parent-commit matrix).
- live × commit, keep: `jigc milestone finalize anchor-probe` exit 3 `milestone.unlanded-work` → printed `git -C <worktree> branch kept/add-alpha-file <sha>` exit 0 → finalize exit 0, record `joined`, landed `beta.rs` only, commit reachable from refs/heads/kept/add-alpha-file.
- live × commit, fold: finalize exit 3 → printed `git -C <worktree> reset --soft <base pin>` exit 0 → finalize exit 0, landed `alpha-c.rs beta.rs`.
- live × commit+staged, fold: finalize exit 3 → printed `reset --soft` exit 0 → finalize exit 0, landed `alpha-c.rs alpha-s.rs beta.rs`.
- stale × staged, restore: finalize exit 3 → printed `mkdir -p <path> && printf 'gitdir: %s\n' <admin> > <path>/.git && git -C <path> restore .` exit 0 → finalize exit 0, landed `alpha-s.rs beta.rs`.
- stale × commit+staged: finalize exit 3 → printed recipe exit 0 → finalize exit 3 (now a live commit) → printed `reset --soft` exit 0 → finalize exit 0, landed all three files.
- stale × commit, keep aimed at the main checkout: finalize exit 3 → printed `git -C <repo> branch kept/add-alpha-file <sha>` exit 0 → finalize exit 0.
- The other exit as printed: `jigc milestone discard anchor-probe` exit 1 `milestone.dirty-worktree` (its own route) → `jigc milestone discard anchor-probe --force` exit 0, record `discarded`.
- settled sub-task × live × staged: `jigc task discard add-alpha-file` exit 0 → finalize exit 3 → printed `git -C <worktree> stash` exit 0 → finalize exit 0 (1 sub-task), `git show stash@{0}:alpha-s.rs` returns the staged bytes after the worktree is gone.
- settled sub-task × live × commit: finalize exit 3, no `reset --soft` offered → printed keep exit 0 → finalize exit 0.
- unlinked checkout × staged at finalize: exit 3 → printed `printf 'gitdir: %s\n' <admin> > <path>/.git` exit 0 → 0 prunable records, `A  alpha-s.rs` still staged → finalize exit 0, landed `alpha-s.rs beta.rs`.
- unlinked checkout at `jigc milestone provision` (exit 1), `jigc milestone discard` (exit 1), `jigc uninstall` (exit 1): the same printed re-link exit 0 at each; the path is a worktree of its own again with the path still staged.
- keep name taken, branch `kept/add-alpha-file` exists: provision, discard, uninstall, finalize each refuse and print `… branch kept/add-alpha-file-2 <sha>` → exit 0 → the door's re-run exit 0, commit reachable from that branch.
- keep name taken, a branch named `kept` exists: the same four doors print `… branch kept-add-alpha-file <sha>` → exit 0 → re-run exit 0.
- keep name taken after a consented drop: `jigc milestone discard anchor-probe --force` exit 0 → narrated `git -C <repo> branch kept/add-alpha-file-2 <sha>` exit 0.
- By hand only, not pinned in the suite: a branch `kept/add-alpha-file/x` exists → finalize prints `kept/add-alpha-file-2` → exit 0 → finalize exit 0.
- Edge cells driven: unreadable registration (HEAD garbage) → finalize exit 3 quoting git's message, no command pointed at; file at the path → exit 3 with `move it aside`; all work held and no sibling code → the held-work refusal answers before zero-contribution, with the recipe directly; a rebase in one worktree plus held commits → one run prints `repo.operation-in-progress` and `milestone.unlanded-work` together; ordinary fan-out with both sub-tasks staged → exit 0, stderr empty.
- Pre-fix dead ends driven on the parent binary: the printed `git … branch kept/add-alpha-file <sha>` exited 128 with the branch present (`a branch named … already exists`) and 128 with a branch `kept` present (`cannot lock ref`).
- `git worktree repair` driven (git 2.54.0): run in a `cp -R` copy it printed `repair: .git file incorrect` for both of the SOURCE's live sub-task worktrees and re-pointed their `.git` links at the copy — so the re-link is written by hand, as the restore recipe already was.

### docs_changed

- design/team-ready-state.md — the door rule's home: the 'landed boundary cannot refuse after landing, so it narrates' sentence is struck with the datum; the pre-landing refusal, its subject, its own code and why, its exits; the keep-name and re-link rules; `git worktree repair` recorded as driven; bound (4) corrected and bounds (5) and (6) added
- design/finalize.md — the fourth contribution fact now says only that the directory is gone; a new paragraph, 'The unlanded-work refusal', beside the zero-contribution one
- --help text for `jigc milestone finalize` (crates/cli/src/milestone.rs): what it refuses over with `milestone.unlanded-work`, the commands it prints, and that there is no `--force`
- NOT touched, owed by the orchestrator: DECISIONS.md and implementation/project-history.md (see left_open)

### left_open

- OWED, orchestrator's shared logs (not touched): a DECISIONS.md entry for 2026-10-04 — the pre-landing refusal at `milestone finalize`; the new code `milestone.unlanded-work` and why no existing code fit; the struck 'cannot refuse after landing' sentence; the settled-sub-task cells; the keep-name and re-link rules; `git worktree repair` now driven. Plus the project-history span for the pass.
- OWED, implementation/decisions-pending.md 1.x rows for the bounds below.
- HUMAN'S CALL — a decision I made inside the ruling: a sub-task settled by `jigc task discard` whose LIVE worktree still has staged paths now refuses at finalize (exit was 0 with `not recoverable` printed afterwards). The ruling's words cover it ('staged paths … stale or live') and nothing lands from a settled sub-task, but this cell was not put to the human by name. To reverse it: `staged_is_held` in `unlanded_work` becomes `!checkout`.
- HUMAN'S CALL — where the consent for a settled sub-task's worktree belongs. `jigc task discard <sub>` removes no worktree and asks nothing about it; finalize is now the door that refuses. The alternative is for `task discard` to answer for the worktree itself.
- HUMAN'S CALL — the bytes leg for a SETTLED sub-task's worktree at finalize: its unstaged and untracked files are still destroyed by the landed teardown and named afterwards. M46's measured warrant (build output on the success path) was taken over landed sub-tasks; no commit will ever carry a settled one's bytes. Refusing there would fire on most 'discard one sub-task, land the rest' runs, so I did not widen it.
- LEAD — an operation git left un-concluded (paused rebase, bisect) in a SETTLED sub-task's worktree is torn down by a landed finalize and narrated, not refused: the posture guard asks only the worktrees the boundary commits from. Read from the code, not driven.
- LEAD — a sub-agent that committed on a BRANCH inside its worktree: a ref reaches the commit, so nothing is held, and the boundary lands without that work saying `nothing staged`. Nothing is lost, but the deliverable is left out of the milestone silently. Outside the ruling's 'unreachable commit'.
- BOUND — `jigc task validate <sub-task-id>` does not preview `milestone.unlanded-work`. Adding it means a new `gate_coverage` member and moved compose goldens — an undecided fork if wanted.
- BOUND, changed in consequence — the sibling-HEAD false refusal (a commit another linked worktree's HEAD also reaches reads as unreached) now has no `--force` at finalize; the keep command (a ref) clears it. Not driven at finalize.
- NOT DRIVEN — a settled sub-task's worktree that this repository has not registered (settled × copy): the code skips it by rule and the design doc says so; only the landed × copy cell is tested. Also not driven: git older than 2.54.0 (`for-each-ref --format=%(refname:lstrip=2)` needs git ≥ 2.13; `git stash` and `git branch` need a configured identity only for the stash).
- NOT PINNED — the keep name when a branch sits BENEATH it (`kept/<id>/x`): driven by hand on the release binary (picks `kept/<id>-2`, exit 0), no suite cell.
- WORDING — the discard staged-prose route still describes finalize as refusing 'while the milestone has nothing to land or a required slot is empty'; a third condition now exists. Left as written.
- GATE TIMING — the test step ran 554 s, close to the 10-minute foreground ceiling; the suite grew from 8 to 15 tests in this file.

## 5f5b273a fix(rename): a door that writes a committed doc in place never writes through or moves a link

### status

fixed

### gate

dev/gate (full, shared target, foreground, exit 0) on the exact tree committed: fmt exit=0 · clippy exit=0 · build exit=0 · test exit=0 · doctest exit=0 · `tests   passed=4570 failed=0  (over 17 test binaries)` · `GATE: PASS`. The diff was byte-compared before and after the gate (unchanged) and committed straight after. A first gate run was red on one test (`git_span_aim::every_production_caller_of_the_one_home_is_a_row`, passed=4569 failed=1): my new record route calls `git_at` and had no row in that fence's table. I added the row (test file only) and re-ran the whole gate; the second run is the evidence.

### class

Mechanism: a door that writes or moves a committed managed doc WHERE IT STANDS (no working area, no promote sink), guarded by a probe that follows links (`is_file()`, `exists()`, or a byte read) and acting with something that follows or carries them (`fs::write`, `git mv`).

HANDED: 2 cells at one door (`jigc rename`: source is a live link; destination is a dangling link) plus 4 door families to disposition.
FOUND: 3 seams, 8 gate/probe sites and 6 write sinks converted; 6 members already safe; 5 out with a reason. The difference from the brief: rename had a THIRD home nobody named (referrers), the retitle-only arm lied about a no-op, the pure movers had a driven exit-0 store break, and the record doors had an exit-0 commit of the link.

CONVERTED (all on `engine::store::home_entry`, no-follow):
1. `jigc rename`, own home (re-slug and retitle-only): refused before `git mv` and any write. Driven pre-fix: retitle-only wrote through the link and then acked `no-op … nothing committed`.
2. `jigc rename`, destination: dangling link (in/out of repo), live link, directory all refuse at the gate under `write.already-present`. Pre-fix: git's `fatal`, a bare `Is a directory (os error 21)`, and "a different doc" for a live link.
3. `jigc rename`, every REFERRER's home (not in the brief). Driven pre-fix: exit 0, `repointed 1 referrer(s)`, repoint written into the link's target uncommitted, so the commit left a dangling committed reference.
4. `cli::relocate::move_doc` source guard (not in the brief as a harm), covering `config set docs-root`, `config set placement-root`, `jigc relocate` and `jigc rename`. Driven pre-fix: `config set docs-root handbook/sub` exit 0, the relative link dangled, `jigc doc list` then exit 1 code-less for the whole store and `validate` routed at `jigc unmanage`.
5. `relocate::displace_foreign_squatter` probe: a dangling link squatting the destination is now parked like any foreign entry (was a code-less `fatal: destination exists` blocked row).
6. Milestone record preflight `reconcile_record_preflight` (one seam, 6 write call sites: `milestone add-task`, `add-from-spec`, `milestone discard`, `milestone finalize` twice, sub-task `task discard`). Driven pre-fix: `add-task` over a link to a byte-identical copy exited 0 with a record commit holding the LINK and the new body in the untracked target; over a committed link every door wrote through, failed its commit and blamed a hook.
7. Six write sinks now `engine::store::rewrite_home` (in place, asks the open handle before writing): rename's retitle and referrer writes, the record's append, join, discard and sub-task settle.

ALREADY SAFE:
- `migrate-corpus` (in place and relocation arm): temp file + rename, never opens the home. Driven and pinned by a test. It replaces the link rather than naming it (see left_open).
- `jigc migrate` landing: goes through the finalize planner and promote sink (9465f9b6); its source retire is `ValidatedRetirement`.
- in-task `jigc doc rename` and every in-task doc write: act on the task working area via temp+rename; destination guard converted by 9465f9b6.
- `milestone create`: 9465f9b6 (exclusive create).
- `unmanage`, `ingest`, reconcile absorb: write no doc bytes.

OUT, with reason:
- Rollback restores (`cli::rollback` `fs::write`, `PreImage::capture` reads through a link): the external-writer window, ruled tier 2 for M57. With the gates in front it is reachable only by an entry swapped inside the transaction.
- `.jigc/config/**` writers, hooks, gitignore: not managed docs; sibling of 104a7d4b's install members. Not driven.
- `config set docs-root`'s loop silently skips a DANGLING link source (`read … else continue`): nothing is moved or written.
- A symlinked parent directory, and reads through a link: 9465f9b6's declared bounds, unchanged.

### pinned_surfaces_moved

None of the pinned ones: no frozen schema hash, no `schema-manifest.yaml`, no pinned `--format json` key, no `contract-version`, no compose golden, no pack file, no new store/knob/doctype, no new finding code.

What did move, for the record:
- One existing refusal changed identity: `jigc rename` of a doc whose own home is a DANGLING link answered `store.not-found` (route `jigc describe`) and now answers `finalize.promote-clobber` keyed at the path, naming the link.
- `finalize.promote-clobber` is now raised outside a finalize (rename, the move primitive). It is still minted only in `engine::finalize`, so `FINALIZE_FAMILY`'s producer column and its scan fence are unchanged.
- `jigc relocate`: a dangling link squatting the destination is now displaced into `.jigc/displaced/` and reported (was a blocked row).
- Registries grew by one member each: `cli::rename::RefusalKind::ForeignHome` (12 members) and `cli::relocate::RelocateRefusal::ForeignSource` (11 states, 10 codes).
- Engine pub API added (pre-1.0): `store::rewrite_home`, `finalize::store_home_refusal`, `file_state::ConflictBlock::finding_at`. Nothing removed or re-signed.

### notes

(1) COMMIT: 5f5b273a `fix(rename): a door that writes a committed doc in place never writes through or moves a link`, on fix/rc24-tier1 (confirmed by `git branch --show-current` before and after). Started from a clean tree; 20 paths staged by name; tree clean after; nothing pushed, no branch switched.

(2) GATE: dev/gate full — `tests   passed=4570 failed=0  (over 17 test binaries)`, `GATE: PASS`, exit 0, on the committed tree.

(3) WHAT WAS FIXED: every door that writes or moves a committed managed doc where it stands now asks the home's own directory entry without following a link, before `git mv` and before any write, and refuses anything that is not a regular file. Three seams: `jigc rename` (own home, referrers, destination), the move primitive (source), and the milestone record's shared preflight. The six writes behind those gates go through one new in-place, no-follow writer, `engine::store::rewrite_home`.

Codes, as applied:
- rename destination -> `write.already-present` (as decided).
- rename own home and referrer, move-primitive source -> `finalize.promote-clobber`, keyed at the path (my reading of the decided reference; see left_open).
- record doors -> `reconciliation.conflict-block` (my choice; see left_open).

RED FIRST: the new suite `crates/cli/tests/store_door_home_shape.rs` (registered in g_finalize) ran 5 of 6 red on the unfixed tree, each for the defect (doors exiting 0 or answering git's fatal). The sixth is the migrate-corpus pin, green before and after by design. The two new relocate unit tests were also proven red with the guards removed. Every cell was first reproduced by hand on the unfixed release binary.

THINGS TO KNOW BEFORE RESUMING:
- I went past the two reported cells, on the brief's instruction to close the class. Three additions were not named anywhere: rename's referrer homes, the move primitive's source, and the record doors' exit-0 commit of the link.
- The record-door finding is the most serious thing driven here: `milestone add-task` exited 0 and committed the link in the record's place.
- The (R6, D-7) fixer's bound (3) in design/finalize.md said these writers were unchanged; I rewrote that sentence because it is no longer true.
- `git_span_aim.rs` needed a row for the new aimed `git checkout HEAD --` span; that was the first gate's only red.

TESTS ADDED:
- store_door_home_shape.rs, 6 tests: rename own home × {re-slug, retitle-only} × {live, dangling}; rename destination × 4 entry shapes with both route exits; rename linked referrer; docs-root re-point; record doors × {add-task, discard, task discard, finalize} × {uncommitted, committed, dangling}; the migrate-corpus pin.
- flow37_rename.rs: two `ForeignHome` scenes on the refusal axis and a code-equality test.
- tests/relocate.rs: the `ForeignSource` axis cell.
- cli::relocate unit tests: link source refused, dangling squatter displaced, code equality.
- engine unit tests: `rewrite_home` (in place; every non-regular shape refused, nothing written) and `store_home_refusal` identity over three shapes.

Scratch drive scripts and logs are under <scratch>/rn/ (drive.sh for the pre-fix reproductions, routes.sh for the release route drives, gate2.out); not committed.

### routes_driven

- All on target/release/jigc built from the fixed tree, in `dev/jigc-rig fresh --binary target/release/jigc` rigs; each emitted span run as printed.
- rename, own home a live link (re-slug): `jigc rename adr:cache-strategy --to "Cache Strategy Two"` -> exit 1, finalize.promote-clobber. Exit taken by hand (copy of the target in the link's place, committed). Emitted `jigc rename adr:cache-strategy --to 'Cache Strategy Two'` -> exit 0; HEAD holds 100644 docs/decisions/cache-strategy-two.md, tree clean, old target untouched.
- rename, own home a live link (retitle-only): `jigc rename adr:cache-strategy --to "Cache strategy!"` -> exit 1. Same exit by hand. Emitted `jigc rename adr:cache-strategy --to 'Cache strategy!'` -> exit 0; HEAD 100644, H1 `# Cache strategy!`.
- rename, a referrer's home a live link: `jigc rename adr:cache-strategy --to "Cache Plan"` -> exit 1, finalize.promote-clobber at the referrer's path. Referrer regularized and committed. Emitted `jigc rename adr:cache-strategy --to 'Cache Plan'` -> exit 0; HEAD referrer 100644 with `supersedes: adr:cache-plan`.
- rename, destination a dangling link: `jigc rename adr:cache-strategy --to Taken` -> exit 1, write.already-present. Emitted `jigc rename adr:cache-strategy --to Taken --slug <other-slug>` (slug filled) -> exit 0; link untouched.
- rename, destination a directory: `jigc rename adr:taken-instead --to Again` -> exit 1, write.already-present. Directory moved out by hand (untracked, no commit). Emitted `jigc rename adr:taken-instead --to Again` -> exit 0; HEAD 100644.
- config set docs-root, linked doc: `jigc config set docs-root handbook/sub` -> exit 1, config.repoint-failed carrying finalize.promote-clobber; nothing moved, knob unchanged. Doc regularized and committed. Emitted `jigc config set docs-root handbook/sub` -> exit 0; both docs moved; `jigc doc list` exit 0.
- relocate (freeze-exempt `note` pack), stranded link: `jigc relocate note --from legacy` -> exit 0 (triage door) with a blocked row `finalize.promote-clobber` and a route; the regular sibling moved. Link regularized and committed; the route's 're-run this command' -> exit 0, `1 moved, 0 blocked`.
- milestone add-task, record link uncommitted: `jigc milestone add-task ship-it "second task"` -> exit 1, reconciliation.conflict-block. Emitted `git -C <repo> checkout HEAD -- docs/milestone-records/ship-it.md` -> exit 0. Re-run -> exit 0; HEAD record 100644 with `{#second-task}`; old target has no written-through marker.
- milestone add-task, record link committed: -> exit 1. Record regularized and committed by hand (the route's second arm). Re-run -> exit 0; HEAD record 100644.
- milestone discard, record link uncommitted: `jigc milestone discard ship-it` -> exit 1. Emitted git checkout -> exit 0. Re-run -> exit 0; record `status: discarded`.
- task discard (sub-task), record link uncommitted: `jigc task discard first-task --force` -> exit 1. Emitted git checkout -> exit 0. Re-run -> exit 0; item `status: discarded`, record commit named.
- milestone finalize, record link uncommitted: `jigc milestone finalize ship-it` -> exit 1. Emitted git checkout -> exit 0. Re-run -> exit 0; `finalized … 2 files committed`, record `status: joined`.
- Not driven separately on release: `milestone add-from-spec` (shares `append_and_commit_record` and the preflight) and `config set placement-root` (shares the primitive through `relocate_stranded`). The dangling-record and committed-link cells for discard/task-discard were driven by the suite on the debug binary only.

### docs_changed

- design/write-commands.md — `jigc rename` step 2: the home-shape gate over all three homes, the driven cells, the codes, the in-place write; the refusal-axis paragraph now says twelve states and lists `finalize.promote-clobber` among the reused codes
- design/finalize.md — §4 Promote: Declared bound (3) no longer says the in-place writers are unchanged; new paragraph 'The doors that write a committed home in place' (rename, the relocation primitive, the record's later writers, the route, `rewrite_home`) and 'One in-place writer is left as it was' (migrate-corpus, with its declared difference)
- design/team-ready-state.md — The lifecycle, 'the id belongs to the record': the later record writers, the entry-before-bytes preflight, both restores
- design/reconciliation.md — Relocation collisions: the occupant is the destination's own entry read without following a link; a link source is not carried
- design/validation.md — the relocate refusal axis: eleven states / ten codes, and the new member's code and owner
- design/command-output-contract.md — the `promote-clobber` sub-table row: the shape arm is also raised outside a finalize, by the store doors
- --help: `jigc rename` gained one sentence (crates/cli/src/cli.rs); `FINALIZE_FAMILY`'s promote-clobber subject_note and the `FinalizeCode` doc (crates/cli/src/render.rs)
- NOT touched, owed by the orchestrator: DECISIONS.md and implementation/project-history.md

### left_open

- CONFIRM — the source code is my resolution of the brief's reference. 'The code the (R6, D-7) fix uses for a non-regular home' I read as `finalize.promote-clobber` (what the committing doors raise for a doc's own home that is a link). It is now raised at `jigc rename` (own home, referrer) and at the move primitive. If another code was meant, it is one constructor: `engine::finalize::store_home_refusal`, plus the two registry rows that name it.
- CONFIRM — the record doors' code was my choice, not decided: `reconciliation.conflict-block`. Reason: those doors already raise it for a link whose target bytes differ, so a same-bytes link now gets the same identity. The shape check runs before the byte compare, so a differing-bytes link keeps its code but gets the link-specific message and route.
- HUMAN'S CALL — `jigc migrate-corpus` over a doc whose home is a link lands a regular file and writes through nothing, but it REPLACES the user's link without naming it. Left as is and pinned by a test (`the_corpus_migration_lands_a_regular_file_and_writes_through_nothing`). Refusing instead would be a new blocked finding at that door and has no honest existing code.
- The move-primitive refusal turns two previously-succeeding commands into refusals over a link source: `config set docs-root` / `placement-root` (whole re-point undone) and `jigc relocate` (blocked row). I converted them because the driven outcome was an exit-0 store that `doc list` could not enumerate. If you read that as beyond the decision, it is isolated to `refuse_foreign_source` in crates/cli/src/relocate.rs.
- Rendering, pre-existing shape that my refusal inherits: under `config.repoint-failed` a per-doc cause that is itself a finding prints nested, with two `at:` and two `route:` lines (and the inner route says 're-run this command'). The same nesting exists for any primitive-raised cause on `placement-root`. Both routes work as printed; cleaning the nesting is a separate change.
- `config set docs-root`'s loop still silently skips a DANGLING link under a doctype directory (`read … else continue`): the re-point lands and the dangling link stays at the old home. Nothing is moved or written; not changed.
- Config-family writers under `.jigc/config/**` (`config set`'s manifest, `insert-step`/`replace-step`, workflow files) are still plain `fs::write`. Not managed docs, so out of this class; not driven. Sibling of 104a7d4b's install members.
- Still open from 9465f9b6 and not touched here: plain create over a body-less entry; copy-on-first-touch of a doc whose home is a live link; the FIFO read seam; `jigc doc list` code-less exit 1 over one unreadable entry; the rollback's follow-links `PreImage::capture` (tier 2, M57).
- Not driven on Linux: every drive and the gate ran on macOS. `rewrite_home` uses only `std` (dev/ino comparison under `cfg(unix)`); CI is the first Linux run.
- OWED IN THE SHARED LOGS (orchestrator): a DECISIONS.md line for the contract extending to the store doors, the two code choices above, the move-primitive conversion and the migrate-corpus disposition; the project-history fix-pass span; decisions-pending rows for whichever of the items above the human defers.

