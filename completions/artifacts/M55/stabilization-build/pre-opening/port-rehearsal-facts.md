# Facts for the design of the port rehearsal

*Returned 2026-10-06 by a read-only exploration; transcribed by the orchestrator. Nothing was built or run. The instrument: a rehearsal of this repository's migration onto jigc, on a copy of it, with the candidate's binary — it tests the documented adoption path and is never a requirements source. Put to the human on 2026-10-06 as question 27, and ruled: shape **B** below — the scripted walk plus one real migration by an agent (`decisions.md` beside this file → *The port rehearsal's shape*).*

**The corpus:** 3,302 tracked files, 2,846 commits, 1,057 `.md` (29 MB); `DECISIONS.md` is 8.3 MB / 19,828 lines / 1,434 entries; `CLAUDE.md` is 50.6 KB; no `docs/`, no root `CHANGELOG.md`, no `.jigc/`, and `.gitignore` does not mention it.

## 1. The documented path (the two guides `setup` installs as the skill)
- `jigc setup` (QUICKSTART.md:47-104): writes `.jigc/AGENT.md`, the `CLAUDE.md` import, the `.claude/settings.json` allowlist and `SessionStart` hook, the skill and a warn-only `pre-commit`; it "commits its own install" and refuses dirty install paths (:106-123).
- Existing docs: "run `jigc ingest` first — it detects and routes existing content rather than overwriting it" (:357-359); then "`jigc ingest` to classify, `jigc migrate <path> --as <doctype>` per doc" (MIGRATING.md:5); a plain finalize holds, "`--approve` is the sole destructive gate" (:54).
- First task: `jigc start "<intent>"` → `--workflow <chosen>` → `jigc task finalize <id>` (QUICKSTART.md:149-224).
- **The guides have no pack-choice step and no on-ramp to a fan-out** (zero hits for `milestone create|add-task|execute`).

## 2. What the path would touch here
- With defaults (`docs-root: docs/`, `placement-root` unset) exactly one existing file sits at a managed home: root `VISION.md`.
- Foreign, with a migrate workflow: `implementation/roadmap.md`, `implementation/decisions-pending.md`, `DECISIONS.md`, 53 `ideas/*.md`, the VERDICTs.
- M56's order: deferral ledger → roadmap → project-history → `CLAUDE.md` (roadmap.md:3052), after adopting the seed (decisions-pending.md:92).
- No doctype: `design/` (29 files), `project-history.md`, `completions/artifacts/**`, `WHY-JIGC.md`.
- Layout: the path offers `jigc config set docs-root` / `placement-root` (QUICKSTART.md:353-354). *Inferred:* `docs-root .` turns the 53 ideas into squatters.

## 3. The five named steps, concretely
- **setup:** exit 0, one `chore(jigc): install jigc workspace config` commit, a re-run byte-identical. `jigc validate` then exits non-zero over `VISION.md` **by design** (validation.md:458).
- **ingest:** copy the 92 seed docs under `docs/` → `jigc ingest` → commit by hand (ingest leaves them untracked — F20). `seed_fence.rs:133` already proves this on a fresh rig. Noisy: about 1,057 rows.
- **"the project-listed pack":** the phrase is not in the repository. Setup auto-wires the methodology pack; a *listed* pack is an absolute-path house pack (multi-pack.md:117-135), absent from the guides.
- **doc task:** `jigc start --workflow park-idea|report-inconsistency` → one commit; `jigc doc list` shows it.
- **fan-out:** `milestone create / add-task / provision / execute / finalize` → exactly one commit.
- Slow: migrate authoring is quadratic in items (MIGRATING.md:65).

## 4. The copy
A plain `git clone` on a branch with the remote removed — **not** a `git worktree add` (doc writes there are refused, MIGRATING.md:51). *Inferred:* keep full history. `run-session.sh:196` copies the whole directory, so never hand it this working tree (`target/`). `check-corpus.sh` assumes 7 commits and README-only. `dev/jigc-rig` isolates `$HOME` on the host but builds only its named states.

## 5. Scripted versus agent
Setup, ingest, validate and doc list are deterministic. Doc task and fan-out: `seed-filing/file-seed` is the honest scripted form (fixed inputs, each `Spawn:` line run verbatim, a one-commit assertion); its genuine half is a main-session artifact (milestone-completion-workflow.md:26). A migrate rewrite needs an agent.

## 6. Where the rehearsal could tempt someone to bend the CLI
- `DECISIONS.md`: "structurally beyond `jigc migrate`"; freeze and start fresh (decisions-pending.md:904).
- `VISION.md`: needs a split; an in-place migrate drops or folds surplus.
- `design/` stays plain (roadmap.md:3052). `completions/` are plain owner-artifacts.
- Repository fences that read `CLAUDE.md`: the project's to change.
- Protected `main` (pull request + `ci-ok`): the guides are silent; finalize commits on the checked-out branch.

## 7. What already exists that is close
`self-migration-coverage.md` (2026-07-10, pre-dates the methodology migrate workflows) · the seed fence and filing, done in a `dev/jigc-rig fresh` rig, never in this repository · `M55/genuine-spawn` (simulation plus real spawn, tree-hash match) · `arms/adopt.sh` and walk arm 20 (project pack) · findings-channel.md:255 forbids running setup in this repository early.

## 8. Obvious designs that are wrong
"`validate` exits 0" as the bar · a worktree as the copy · rehearsing the pack and fan-out steps as "documented" when the guides cover neither · migrating `DECISIONS.md`.

## Shapes (the explorer's)
- **A — a scripted walk.** Copy: a host clone with an isolated `$HOME`. Steps: setup, seed ingest, one fixed-string doc task, a three-reporter N-process fan-out; "works" = exit codes, commit counts, a predicted findings set. Minutes; shows nothing about migration or a real agent.
- **B — A plus one real migration.** The same clone; then an agent runs `jigc migrate implementation/decisions-pending.md --as deferral-ledger` to the hold and `--approve`. One agent session on a 522 KB source; cannot show `DECISIONS.md` or `VISION.md`.
- **C — a container session.** The clone through `run-session.sh`, a source-mode image; a blind agent follows the guides only. Highest cost; tests the docs themselves; not deterministic.
