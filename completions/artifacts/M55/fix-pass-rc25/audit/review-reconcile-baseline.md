# Code review — area `reconcile-baseline` (rc.24 fix pass)

Commits reviewed: `eb18e5fe` (copy-in baseline + base-pin backstop) · `8d9c3afb` (record door compares against `HEAD`).
Tree: `fix/rc24-tier1` at `5f5b273a`. Read-only; nothing edited, committed or pushed.
Binary driven: a debug build of this tree in a private target dir (`jigc 1.0.0-rc.24` by version string — the pass has not bumped it). "rc.24" below is the published `1.0.0-rc.24` binary, driven beside it as the control.
Rigs: `dev/jigc-rig fresh | committed-singletons | bare`, plus real `git clone`s of them. Every loss claim was checked in four places: worktree, `HEAD`, `command grep -r` under `.jigc/`, and every blob in the object database.

## Verdict

**Red.** The two rules the pass built work as stated on the cells its tests iterate, but the class it claims to close — *a hand edit made after a task's first write to a doc is overwritten at exit 0 and is in no git object* — is still reachable on this build by three plain sequences (F1, F2, F4), and the new backstop introduces a refusal with no working printed exit in an eol-converting checkout, where rc.24 landed (F3).

## Findings

### F1 · HIGH · another writer moves the doc's baseline between the hand edit and the holding task's finalize — exit 0, the edit in no git object

- **Where:** `crates/engine/src/file_state.rs:929,968` (`task_touched` is the *sweeping* task's own area only) → `:629-631` absorb / `:586-589` adopt re-hash the key to the on-disk bytes; `crates/engine/src/state.rs:1112-1124` (`staged_in_another_task`) is consulted at the copy-in only (`file_state.rs:786`); `crates/cli/src/ingest.rs:251` (register-only absorb). Not introduced by the pass — rc.24 does the same in a baselined checkout — and not closed by it.
- **Mechanism:** the record is per path. The pass applies that fact to one writer of the key (a second task's copy-in → `Unrecorded::StagedElsewhere`) and to none of the others. Any other writer that re-hashes the path to what is on disk turns the holding task's `DRIFTED + TOUCHED` into `IN_SYNC`, and its finalize promotes over the edit.
- **Repro A (driven, this build; a real clone and a baselined checkout; rc.24 baselined identical):**
  - setup: `committed-singletons`; `jigc start --workflow single-task "beta pass"`; `jigc start --workflow single-task "alpha pass"`; `jigc doc set-slot vision:vision#open-questions --from-file <f> --task beta-pass` (copy-in; in the clone this records the baseline); author beta's commit doc; append a paragraph carrying a marker under `## Thesis` in `VISION.md`, uncommitted.
  - argv: (alpha) `git add alpha.rs` + commit doc, `jigc task finalize alpha-pass`; then `jigc task finalize beta-pass`.
  - observed: alpha → exit 0, `advisory · reconciliation.absorb — external edit absorbed: VISION.md … no action needed`, `1 file committed` (`alpha.rs` only; `git status` still ` M VISION.md`). beta → **exit 0**, `finalized … promoted VISION.md`. After: marker worktree 0 · `HEAD` 0 · under `.jigc` 0 · blobs holding it 0.
- **Repro B (driven, this build and rc.24; one task, no second task):** same setup with only `beta-pass`; after the hand edit run `jigc ingest` → exit 0, `advisory · file-state.absorbed — out-of-band edit to VISION.md absorbed — its file-state baseline now records the on-disk bytes … no action needed`; the key moved; `jigc task finalize beta-pass` → **exit 0**, marker worktree 0 · `HEAD` 0 · `.jigc` 0 · blobs 0.
- **Class and count:** every production writer of a path's `file-state` key other than the staging task. Derived by `command grep -n "\.record(" crates/engine/src/file_state.rs crates/cli/src/*.rs` minus the `#[cfg(test)]` modules: 10 lines (engine `588`, `650`, `789`; cli `migrate_corpus.rs:1012,1018`, `milestone.rs:1476`, `relocate.rs:297`, `rename.rs:1116`, `task.rs:7895,7909`) plus `ingest`'s writer behind `absorbed_drift` (`ingest.rs:251`). Only `:789` asks whether another open task holds the doc. **Driven at two movers** (another task's landed finalize; `jigc ingest`). `jigc rename`, `migrate-corpus`, `relocate`, the milestone join's sweep and a fan-out sibling's sub-task finalize are read-only — not driven.
- **Not in any record:** `tier1-verification/R3-F7.md` §3 drives "one unrelated landed finalize *first*" (as a way to get a baseline), never one *between* the hand edit and the holding task's finalize; `decisions-pending.md:79` names only non-jigc-writer windows.

### F2 · HIGH · the declared residual (no record, no blob at the pin) is reachable by a plain single-task sequence, and jigc's own route directs the hand edit

- **Where:** `crates/engine/src/file_state.rs:784-787` (the two `Unrecorded` arms) × `:580-585` (the backstop needs `pinned(path)` to be `Some`) → `:586-607` adopts or advises; `crates/cli/src/task.rs:2624-2626` (the lookup). Stated as a kept cell at `design/reconciliation.md:50` and `design/storage.md:341`, deferred at `implementation/decisions-pending.md:79` — which says the loss it leaves reachable is "two tasks and a hand edit between their copy-ins of an untracked doc … **not driven**".
- **Repro (driven, this build):**
  - setup: `fresh`; write an untracked draft `docs/decisions/draft-queue.md` — valid front-matter, `schema-version: 2`, Context/Options/Decision filled, **Consequences empty** (the shape of a hand-started draft; it parses, and fails `required-slot-present`). `jigc validate` prints `advisory · file-state.un-baselined` and `blocking · schema-conformance.required-slot-present … route: corrupt — … review it by hand`. `jigc start --workflow single-task "finish the queue decision"`.
  - argv: `jigc doc set-slot adr:draft-queue#consequences --from-file <f> --task finish-the-queue-decision` → exit 0, `copied in for update`, **no** `baseline-adopt` line, no `file-state.json`. Author the commit doc. Add a marker line under Context in the file on disk. `jigc task finalize finish-the-queue-decision`.
  - observed: **exit 0**. The finalize prints `advisory · reconciliation.conformance-block — unvetted file docs/decisions/draft-queue.md … route: fix the file to restore conformance, or revert the edit — this is the one case a managed file is yours to hand-edit`, then `finalized … promoted docs/decisions/draft-queue.md`. After: marker worktree 0 · `HEAD` 0 · `.jigc` 0 · no blob holds it.
  - control: the same draft with Consequences filled → the copy-in prints `baseline-adopt`, the finalize exits 3 on `reconciliation.conflict-block`, the marker survives.
- **Class and count:** {no record} × {no blob at the pin}. No-record causes, read off the backstop's own comment (`file_state.rs:569-579`) and the enum (`:697-708`): non-conformant at copy-in · staged elsewhere · key lost after the copy-in · copied in by an older binary = 4. No-blob causes reachable past the other finalize guards: untracked = 1 (`git add`ed-uncommitted is refused by `finalize.carried-staged`, committed-after-mint by `finalize.base-mismatch` — both driven). **3 of the 4 cells driven, all exit-0 losses:** untracked × non-conformant (above); untracked × `jigc unmanage` after the copy-in (exit 0, marker 0·0·0·0); untracked × staged-elsewhere (the second task's finalize exits 0 and a line added after its copy-in is gone 0·0). The older-binary cell is read-only. `jigc milestone finalize` shares the arm — not driven.
- **A BOM also lands here:** a committed doc carrying a UTF-8 BOM fails the gate (driven: the copy-in adopts nothing), so it is `Unrecorded(NonConformant)` too; tracked, it is protected by the pin.

### F3 · HIGH · in an eol-converting checkout the backstop conflict-blocks a doc nobody edited, at both doors, and no printed exit works — rc.24 landed

- **Where:** `crates/cli/src/task.rs:8302-8322` (`git_blob_at` answers one of **two** forms: checked-out, or the raw blob when the file is exactly that); `crates/engine/src/state.rs:1034-1047` (copy-in is *EOL-preserving*, so a slot written into a CRLF doc leaves a **mixed** file); the record doors splice in place. `design/reconciliation.md:50` states "a promote or a record write lands the bytes it commits (`\n` endings whatever the checkout converts to)"; `:54` states the record route's re-run "cannot block again". Both are false for an in-place edit of a checked-out file.
- **Repro, task door (driven):**
  - setup: `committed-singletons`; `git clone --config core.autocrlf=true <origin> clone` (same with `* text eol=crlf` committed). In the clone: task 1 `doc set-slot vision:vision#open-questions`, finalize → exit 0. `VISION.md` now holds CRLF on 15 of 17 lines; `git status` is clean.
  - argv: task 2 `doc set-slot` on the same doc → `jigc task finalize second-pass` → exit 3 `reconciliation.conflict-block` (pre-existing: `advance_file_state`, `task.rs:7906-7909`, records the hash of the *committed blob*, so the doc reads `DRIFTED` at once; rc.24 identical, and `jigc validate` shows `blocking (gates at finalize) · file-state.hash-matches` for a file nobody touched). Then `jigc unmanage VISION.md`; `jigc task finalize second-pass`.
  - observed: **this build exit 3**, the same block. **rc.24 exit 0**, `promoted VISION.md` — the exit rc.24 had from its own eol block is gone.
  - the same state reached by removing `.jigc/state/file-state.json` after a third task's copy-in, nobody editing (this build): exit 3. The route's second exit as printed — `git checkout -- VISION.md`, then `git checkout HEAD -- VISION.md` — each exits 0 and rewrites nothing (git holds the file unmodified; still CRLF on 15 of 17 lines), and each re-run exits 3. An unprinted `rm VISION.md && git checkout -- VISION.md` (17 of 17) → exit 0. rc.24 was not driven on this variant.
- **Repro, record door (driven):**
  - setup: `fresh` + `* text eol=crlf` committed; `milestone create "Probe ms"`; `milestone add-task probe-ms "Add alpha file"`; `git clone`. In the clone the record is CRLF on 16 of 16 lines. `jigc milestone add-task probe-ms "Add beta file"` → exit 0; the record is now CRLF on 11 of 23 lines, `git status` clean. Remove `.jigc/state/file-state.json`.
  - argv: `jigc milestone add-task probe-ms "Add delta file"`.
  - observed: exit 1, `reconciliation.conflict-block … differs from what HEAD holds`, route `git -C <clone> checkout HEAD -- docs/milestone-records/probe-ms.md`. Run as printed from outside the repository: exit 0, the file unchanged (11 of 30). Re-run: exit 1, the same block. rc.24 on that state: exit 0.
- **Class and count:** working-file forms of a doc git calls unmodified = 3 (checked-out · raw blob · mixed, the one an in-place jigc edit leaves); `git_blob_at` answers 2. Consumers of the seam = 4, by `command grep -n "git_blob_at" crates/cli/src` (`task.rs:2626`, `milestone.rs:1708`, `milestone.rs:8104`, `cli.rs:1511`). Driven at two (task finalize; record door). The join gate and the store sweep's lag arm are read-only. `core.autocrlf=true` and `eol=crlf` driven; smudge filters not.
- **Masked by fixture topology:** `copy_in_baseline.rs:1522-1554` builds "jigc's own write" as a doc landed *before* the attribute existed (all `\n`); `record_door_baseline.rs:534-583` iterates `Form::{Written, CheckedOut}` and runs one door once. Neither holds the form a clone's first in-place write produces.

### F4 · HIGH · a migration task's own source is exempt from the backstop unconditionally — a hand edit to it after the mint is replaced at exit 0 (unchanged from rc.24)

- **Where:** `crates/engine/src/file_state.rs:581` (`!conflict.keys(path)`), `:368-370`; stated as a kept cell at `design/reconciliation.md:50`, justified by the `jigc unmanage <source>` exit. The exemption does not ask whether an unmanage happened — and a foreign non-conformant source is never baselined, so in its dominant shape this path is `UNKNOWN` at every finalize.
- **Repro (driven, this build and rc.24, identical):**
  - setup: `fresh`; a foreign non-conformant `VISION.md` committed; `jigc migrate VISION.md --as vision`; `jigc doc author vision --from-file - --task <id>` (three sections); commit doc. Append `## Pricing` + a marker paragraph to `VISION.md` on disk.
  - argv: `jigc task finalize <id> --approve`.
  - observed: **exit 0**, `advisory · schema-conformance.unadopted-instance`, `finalized … promoted VISION.md`. Marker worktree 0 · `HEAD` 0 · `.jigc` 0 · blobs 0. No block, so the keyed route's sentence ("an edit made to the file since is replaced without appearing in the `--approve` fidelity diff") is never printed.
- **Class and count:** instance, unbounded — one doctype (`vision`), one source shape (in-location squatter), one door. The off-canonical source (retired, not replaced) and `milestone finalize` were not driven.

### F5 · MEDIUM · an unreadable `file-state.json` is a new refusal on every first-touch `doc` write, with no route and a message that names the wrong file

- **Where:** `crates/engine/src/file_state.rs:773` (`load(..)?`) → `crates/cli/src/doc.rs:6884-6892` (`"could not read committed `{addr}`"`); `design/reconciliation.md:48` says the failure is re-run by "the lock's own route".
- **Repro (driven):** `committed-singletons`; a task; `printf '{ not json' > .jigc/state/file-state.json`; `jigc doc set-slot vision:vision#open-questions --from-file <f> --task <id>` → **exit 1**, the whole output: ``could not read committed `vision:vision#open-questions`: key must be a string at line 1 column 3``. Nothing staged (held). rc.24: exit 0. `jigc task validate` and `jigc validate` also exit 1 on that record but name it.
- **Class and count:** the door's fail-closed arms = 3 (lock not taken · unreadable record · unreadable doc; `file_state.rs:762-765`). The lock arm carries a route that works (driven: 10 s, exit 1, nothing staged, the retry lands). The record arm carries none. The doc arm was not driven.

### F6 · MEDIUM · three sentences the pass wrote are false on this build

- `crates/cli/guides/MIGRATING.md:60` — "**A hand edit to a doc an open task has already written is never overwritten by that task** — in a fresh clone as much as anywhere". Falsified by F1 (both repros), F2 (three cells), F4. This guide is embedded into the installed skill.
- `design/reconciliation.md:222` — "a hand edit to a doc a task has staged blocks `jigc task finalize` and `jigc milestone finalize` whether or not the cache was ever populated. What is left outside it is stated there". F1 is stated nowhere.
- `design/validation.md:366` — the `file-state.un-baselined` route (*baselined on its next author or finalize*) "is true since the rc.24 fix pass". Driven in F2: the advisory printed for the draft, the next author write baselined nothing.
- `implementation/decisions-pending.md:79` — the residual's reachable loss described as a two-task sequence, not driven. Driven here from one task.
- **Class and count:** instance list — four sentences found by reading the two commits' doc hunks against the repros; not a sweep of every doc the pass touched.

### F7 · MEDIUM · the acceptance suites iterate the axis but not its crossing cells

- `crates/cli/tests/copy_in_baseline.rs:175-200` — `Absent::Untracked` is crossed only with a conformant doc; `:1144-1217` — the non-conformant cell uses a tracked doc in a clone shape and a break (`date: 2026/13/01`) that makes the write itself refuse. The crossing cell (untracked × non-conformant, the write succeeding) is F2. The cell's own doc-comment (`:1137-1142`) says the door blocks "instead of advising *fix the file* and then overwriting it" — which is what F2's finalize printed and did.
- No cell lands a second jigc writer of the key between the hand edit and the holding task's finalize (F1); `:1358` covers the copy-in writer only.
- eol: see F3.
- **Class and count:** three masked cells, each found by a driven loss or false block; the suites were not otherwise audited cell by cell.

### F8 · LOW · a hand edit reverted after the copy-in is committed anyway, and announced as "external edit absorbed"

- **Where:** `crates/engine/src/file_state.rs:617-625` (L1: on-disk bytes equal the pin ⇒ "the drift predates the task"). With the copy-in now recording the hand-edited bytes, a revert to the pin is read as a pull.
- **Repro (driven):** clone; task; add a paragraph under `## Thesis` in `VISION.md`; `doc set-slot` (copy-in, `baseline-adopt`); `git checkout -- VISION.md`; `jigc task finalize` → exit 0, `advisory · reconciliation.absorb — external edit absorbed: VISION.md`, and the paragraph is at `HEAD` and back in the worktree. No byte lost; the outcome is the same on rc.24 (adoption instead of absorb). The surface says the opposite of what happened.
- **Class:** instance, unbounded.

## Held — tried and could not break

- **The reported instance.** Clone, first write, hand edit after → exit 3 `conflict-block`, the edit on disk, at `task finalize`; also with the copy-in made by the rc.24 binary and the finalize by this build (no key → backstop blocks; with no hand edit it lands).
- **Declared order.** A hand edit before the first write lands merged, and the write's ack says `file-state.baseline-adopt` on the text surface and in `findings[]` on `--format json`; the ack's key set is identical to rc.24's (`chars, copied_in, findings, op, target`).
- **Cell 8.** A copies in, `jigc unmanage`, hand edit, B copies in: B records nothing, both finalizes exit 3 (tracked doc).
- **Save-lock timeout.** Lock held by a live process for 14 s: the write exits 1 after 10 s, nothing staged, no record written, the message names the lock and the retry; the retry lands and adopts.
- **Fresh clone, plain flows.** First task lands with no false block (LF checkout); a committed doc with a BOM and no final newline lands and its second task lands with the key removed; untouched docs still adopt at the sweep.
- **Store scope and the hook.** After a copy-in, a hand edit reads `file-state.hash-matches` at `jigc validate` (exit 0) and a plain `git commit` of that edit through the installed pre-commit hook exits 0.
- **Other guards around the residual.** A `git add`ed-uncommitted doc is refused by `finalize.carried-staged`; a doc committed after the mint by `finalize.base-mismatch`.
- **Record door.** Real clone, hand note → exit 1, nothing written; the route run as printed from outside the repository, then the re-run → exit 0 (LF checkout). A teammate's record op pulled into a checkout with **no** key is adopted; into one that **holds** a key it conflict-blocks and routes at reverting the commit — as declared (M55 P3; the review's `(R3, F4)`, not this pass's).
- **Scope rule.** Neither commit touches a golden, a snapshot, a schema manifest or a pack file (`git show --stat`); no new finding code.
- **Suites, run on this tree in a private target:** `g_finalize` `copy_in_baseline:: reconciliation_baseline_contrast:: file_state_concurrency::` 23 passed · `pre_guard_repair_route:: l1_pull_absorption:: migration` 20 passed · `g_milestone` `record_door_baseline:: milestone_record_` 39 passed · `g_methodology` `flow59_branch_and_pull::` 3 passed · `g_flow` `flow49_acceptance::` 10 passed · `jigc-engine` `file_state::` 45 passed. No failures.
- **jigc in a subdirectory of the git repo.** `jigc setup` from a subdirectory installs at the toplevel, so `<pin>:<path>` is always toplevel-relative.

## Not examined

- `jigc milestone finalize` (the join door) for F1, F2, F4; a fan-out sibling's sub-task finalize as F1's mover; `jigc rename`, `migrate-corpus`, `relocate` as F1's movers.
- The record doors other than `add-task` and a sub-task `task discard` (`add-from-spec`, `milestone discard`, the finalize flip).
- Smudge/clean filters, `working-tree-encoding`, Windows.
- The off-canonical migration source (retired rather than replaced).
- The external-writer race windows the fixer declared (tier 2 by ruling).
- The full gate — only the suites listed above were run.
- `design/command-output-contract.md`, `write-commands.md`, `worked-examples.md`, `team-ready-state.md`, `findings-channel.md` hunks were not read line by line; the engine unit tests were run, not reviewed.
