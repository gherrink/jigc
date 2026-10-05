# Fixer's report — the copied-in witness (option A), commit 0f34d8f0 — verbatim (a lead, not a verified fact)

**Status: fixed.** F1, F2, F7 and E2E F5 are closed in one commit on `fix/rc24-tier1`; the tree is clean, nothing was pushed and no branch was switched.

One thing for the human first, because it is where accepted consequence 1 touches the exit rule: an uncommitted hand edit made before the task's first write is carried into the staged copy, and if the task then writes that same slot, the task's prose replaces it and the edit ends in no git object at exit 0. This was already the behaviour with no baseline (rc.24 included); with option A it is now also the behaviour under a held baseline. I judged it inside the decision. The advisory, design doc and guide state it; I rewrote the advisory after catching my first wording over-claiming.

## Per finding
- **reconcile-baseline F1 — fixed `0f34d8f0`.** Reproduced before the change on the pre-fix release binary: unrelated `task finalize` as mover and `jigc ingest` as mover both gave exit 0 with the marker in worktree 0 · HEAD 0. `milestone finalize` after `jigc ingest` lost it the same way. All now exit 3 with the edit on disk.
- **reconcile-baseline F2 — fixed `0f34d8f0`.** Reproduced before: exit 0, the "yours to hand-edit" advisory printed, then promoted over the line. Now exit 3 and the sanction is not printed; the unedited draft lands with no such sentence.
- **E2E F5 — fixed `0f34d8f0`.** Reproduced before: exit 0, marker 0 · 0. Now exit 3.
- **reconcile-baseline F7 — fixed `0f34d8f0`.** The crossing cells are tests (below).
- Also closed: F8's silent sibling (edit, copy-in, revert now blocks, with the key held or absent).

## Commit and gate
- `0f34d8f0 fix(reconcile): a doc a task holds is decided by what that task itself copied in` — 19 files.
- `dev/gate`, full, backgrounded, own exit code read: `tests   passed=4635 failed=0  (over 17 test binaries)`, `GATE: PASS`, `GATE_EXIT=0`. Test step 590 s.
- The staged diff's checksum equals the gated tree's, so what was committed is what the gate ran on.
- Red first: with the witness consult disabled, all four crossing cells failed with "expected exit [3], ended with exit 0", as did the held-baseline, undone-edit, non-conformant and cell-9 cells. With a bytes-only comparison, the re-checkout cells failed for `core.autocrlf=true`, `eol=crlf`, `text=auto` and at the join.

## Stored shape
`<task>/docs/provenance.json` gains one member, written by `engine::state::copy_in` in the same atomic write as `edited-from-base`:

```json
"copied-in": { "adr:squat": { "bytes": "<blake3 of the raw bytes read>", "git": "<git hash-object --path=<home> --stdin>" } }
```

- **Back-compat:** the member is skipped while empty and defaulted on read, so an area with no copy-in is byte-identical to before (asserted) and a previous-format area loads as holding no witness.
- **Why it is eol-safe:** equal bytes means as copied in, and git is not asked. Different bytes are hashed by git at the same path, and an equal id means git rewrote the file in another form. Both sides are working files through one conversion, never one against a committed blob, so the CRLF-in-blob rule behind the `setup` regression cannot split them.
- **Fail direction:** different ids, a witness with no git id, or git unable to answer all count as edited.
- **`core.safecrlf=true`:** `git hash-object --path --stdin` does not die and gives one id for mixed, CRLF and LF input (checked directly).
- **One deviation from the brief's wording:** the value is two hashes, not one, because byte equality must never depend on git and the re-checkout case needs git's id.

## Axis: handed versus real
- **Copy-in sites: 2 handed, 2 real.** `read_or_copy_in` serves six edit leaves; `stage_minted` sits behind `doc create` / `doc author`. The ungated `state::create` has no production caller.
- **Doors: 2 handed, 2 real.** `TaskArea::validate` covers `task validate`, `finalize`, `--dry-run`, `--approve` and `start --task`; `milestone_boundary_gate` reads each sub-task area's own manifest.
- **Out, with reason:**
  - the milestone-record door has no task area and keeps its own `HEAD` witness;
  - `jigc validate` and the pre-commit hook are read-only on the per-path record;
  - in-task `doc rename` refuses a re-slug of an `edited-from-base` doc (the manifest re-key carries the witness anyway).
- **F1's eleven key writers:** closed by construction, since the record no longer decides a held doc. Driven: 2 at the task door, `ingest` at the milestone door. Not driven: `rename`, `migrate-corpus`, `relocate`, the join's sweep, a sibling sub-task finalize.

## Must-not-refuse cells
**Driven, in tests:**
- Seven git configurations: none, `core.autocrlf=true`, `=input`, `* text=auto`, `*.md text eol=crlf`, a clean/smudge filter, and a committed CRLF blob under `autocrlf=input`.
- Each runs the clone sequence: first task, key held, key lost, a doc git checks out again after the copy-in (with witness and in a previous-format area), then a hand edit blocking and landing once reverted.
- The milestone join under `eol=crlf` with a re-checkout.
- Untracked draft unedited, untracked conformant doc with key lost unedited, held baseline with edit before first write, previous-format area unedited.

**Driven by script on the release binary only:** a provisioned fan-out worktree (sub-task writes from its worktree; unedited lands, edited exits 3).

**Kept working:** `flow59_branch_and_pull`, `l1_pull_absorption`, `pre_guard_repair_route` (the migration `unmanage` exit), first-encounter adoption of untouched docs.

**Not covered:**
- A real `git clone` for the unedited side (clone shape is used; the real-clone test covers the refusing side).
- Two open tasks crossed with conversions.
- LFS-style process filters, `working-tree-encoding`, Windows.
- Submodule, bare repository plus worktree, `--separate-git-dir`.
- A user-made linked worktree cannot reach the door.

## Routes run as printed (`target/release/jigc`, built from the commit)
- **Task door**, route `` `jigc task discard beta-pass --force` … or revert the external edit on disk ``:
  - `jigc task discard beta-pass --force` → exit 0, edit still on disk; then `git add` + `git commit` + `jigc validate` → 0.
  - `git checkout -- VISION.md` → 0; `jigc task finalize beta-pass` → 0, promoted, tree clean, `jigc validate` → 0.
- **Untracked draft:** block exit 3; file put back (`cp`) → 0; `jigc task finalize finish-the-queue-decision` → 0, finished draft at HEAD, `jigc validate` → 0.
- **Milestone door**, route `` `jigc task list` names the live tasks — discard the sub-task … or revert … re-run the join ``:
  - `jigc task list` → 0.
  - `jigc task discard alpha-sharpens-a-doc --force` → 0; `jigc milestone finalize sharpen-the-docs` → 0, edit still on disk.
  - Other exit: `git checkout -- VISION.md` → 0; `jigc milestone finalize` → 0.
- **Non-conformant doc carried into the task** (test, debug binary): `jigc doc set-field adr:…#status/date --task <id> --value 2026-10-05` → 0; finalize → 0.
- **Migration `jigc unmanage <source>`:** driven by `pre_guard_repair_route` on the debug binary, not re-driven on release.

## The three accepted consequences (release binary)
1. Hand edit, then `jigc doc set-slot vision:vision#open-questions --task sharpen`, then `jigc task finalize sharpen` under a held baseline → **exit 0**, edit at HEAD, advisory `reconciliation.absorb — … it is what this work copied in`.
2. After a block: `jigc unmanage VISION.md` → exit 0; `jigc task finalize sharpen` → **exit 3** again, edit on disk.
3. `core.autocrlf=true` clone: copy-in of the mixed file (15 of 17 lines CRLF), `rm VISION.md`, `git checkout -- VISION.md` (17 of 17, bytes differ), `jigc task finalize second` → **exit 0**. A hand edit then → 3; `git checkout -- VISION.md` → finalize 0, status clean.

## Pinned or registered surfaces that moved
- No new finding code, file, registry or flag. No frozen schema hash, `--format json` key, contract-version or compose golden moved. No registry row was needed; the gate is green.
- **Snapshot:** `jigc_engine__state__tests__provenance_two_staged_docs.snap` gains the member for its copied-in doc.
- **Text:**
  - new wording under `reconciliation.absorb` for a doc the task holds;
  - the advisory `reconciliation.conformance-block` over a staged doc no longer carries the hand-repair sanction;
  - the embedded `MIGRATING.md` item 8 states the one rule.
- **Engine public API** (published rc crate):
  - `state::copy_in` takes `&CopyInRead`;
  - `read_for_copy_in`, `create`, `create_gated` gain an `AsGitStores` parameter;
  - new: `state::CopiedIn`, `state::held_docs`, `CopyInRead::of`, `ProvenanceRecord.copied_in`, `file_state::CopyInVerdict`, `ConflictBlock::holding`, `validate::AsGitStores`.
- **Behaviour:** consequences 1–3, plus the undone-edit cell (exit 0 on rc.24, now exit 3).
- **Tests restaged to the new rule:** `reconciliation_baseline_contrast` (three baseline states by two edit orders; the order decides), `flow49`, `flow19`, `singleton_running_doc` (drift moved to after the copy-in).

## Test cost
Measured one at a time:
- The new crossing test `a_doc_a_task_holds_is_decided_by_what_that_task_copied_in` (four cells, one fixture): 27 s.
- Estimated from scenario counts: `a_hand_edit_before…` +4.5 s, cell 9 +6 s, undone-edit +1.5 s, conversions +4 s wall, contrast +3.5 s. Engine tests are negligible.

The gate's test step was 590 s against 563–569 s before, with Spotlight loading the machine. Headroom to 600 s is gone; one discovery run took 1047 s under that load.

## Seen and left open
- **Previous-format areas** keep the recorded baseline and base-pin backstop, with both old bounds, until their task lands or is discarded. Stated in `design/reconciliation.md`.
- **The conflict's words are unchanged**, as instructed. In the undone-edit cell "revert the external edit" means putting the line back, and the only printed command is the discard, which drops the sole copy of that line.
- **Milestone door with an unrelated finalize as mover** is pre-empted by `finalize.base-mismatch`, so only `ingest` is a mover there.
- **Unreadable manifest:** yields no verdicts at the previews, so `task validate` does not newly fail. The finalize itself still refuses on `finalize.provenance-io` before any write.
- **Pre-existing, not this door:**
  - under a committed CRLF blob plus `autocrlf=input`, a no-change task ends in `finalize.commit-rejected` ("nothing to commit") routed at "fix the hook's complaint";
  - under `core.safecrlf=true`, `git add` of a mixed-eol file is fatal (git only, not driven through jigc);
  - the sweep skips a home it cannot read.

## Owed in the shared logs (not touched)
- **`DECISIONS.md`:** a 2026-10-05 entry for option A, its three consequences and the as-built. `design/reconciliation.md` cites "DECISIONS.md → 2026-10-05", which does not exist yet. The 2026-10-04 `(R3, F7)` entry's "both orders block when the key is lost" now holds only for a previous-format area.
- **`implementation/decisions-pending.md:79`** (residual of `(R3, F7)`, trigger M57): what is owed there is built; the row is obsolete except for the previous-format bound. Its two external-writer windows stay in the tier-2 row at line 78. The row at line 86 (block at the write) stands.
- The earlier owed items from Area 4 (as-built entries for `4ba04efc`, `d24a0c91`, `e39d9ac9`, `b54b58b2`) remain.

Main files: `crates/engine/src/state.rs`, `crates/engine/src/file_state.rs`, `crates/cli/src/task.rs`, `crates/cli/tests/copy_in_baseline.rs`, `design/reconciliation.md`. Probe scripts are in the session scratchpad under `w/`.