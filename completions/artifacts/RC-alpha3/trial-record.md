# RC trial — the verification trial on rc.8 — project-alpha-3.0 (2026-07-21/22)

**Status: RC input — the fourth adoption corpus, and the run that (substantially) discharges the chartered verification trial**, including the designed **milestone fan-out live probe** (the M39 honest bound's last owed half). A full GSD→jigc corpus migration of `~/ideas/project-alpha-3.0` (a real brownfield PHP/JS monorepo) followed by **eight designed task sessions** — each task driven in a **separate session** with a written handover — on the installed **1.0.0-rc.8**: code+doc-gate (single-task) · a ship-gate dev/decided-task · the planning workflow · the completion workflow · the reference graph · spec→implement-from-spec · the **live milestone fan-out** (5 parallel worktrees, one join commit) · a dedicated adversarial/recovery probe. This record archives provenance, the invocation-log analysis, the session inventory, what held, and the honest bounds; the adversarial verification of every claim against the rc.8 code — with live repros — lives in [findings-verification.md](findings-verification.md); the executing agents' verbatim feedback in [feedback-verbatim.md](feedback-verbatim.md).

## Provenance

- **Repo:** `~/ideas/project-alpha-3.0` — a real GSD-managed PHP/JS monorepo (apps/ + packages/, docker, pnpm workspace). **First corpus with:** stdout-emitting git hooks (husky + lint-staged), a gitignored runtime dir (`vendor/`), closure-style tests (Pest), and a live multi-worktree fan-out.
- **Binary:** jigc **1.0.0-rc.8** (installed release build) throughout — the invocation log confirms **639/639 records on `1.0.0-rc.8`**.
- **Method:** the migration half ran as ~17 `jigc migrate` adoptions (19 migrate invocations, 18 exit-0) incl. the first **one-source→many-doctypes split** (PROJECT.md → vision + prd + decisions-log); the implementation half ran as **eight handover-driven sessions**, each executing agent working blind-ish (see Honest bounds) and asked for candid feedback via a fixed prompt (archived at the end of [feedback-verbatim.md](feedback-verbatim.md)).
- **Invocation log:** ON — `.jigc/logs/invocations.jsonl`, **639 records** spanning 2026-07-21 15:51 → 2026-07-22 15:53 UTC (~24 h wall).
- **Commits:** **40** in the trial window, including the milestone join commit `632e076` (5 sub-tasks folded, 406 tests passed post-fix) and the completion-record commit `e6a64eb` (an honest **RED** verdict — the loop never pressured a false green).

## Headline outcomes, one line each

- **Zero true regressions.** All three suspected regressions dissolved under adversarial verification (the `#type` address claim refuted; the form-vision empty render is a role-binding trap, not an M39 regression; the trailer drop is latent-since-birth, not a break).
- **The recurrence signature is the incomplete class fix**: ~19 of ~44 distinct findings are the same defect class returning through an axis the original fix never swept — doctype shape, write path, verb family, or surface. Detail + diagnosis in [findings-verification.md](findings-verification.md) → Recurrence.
- **Two v1-gating latent defects found**: item-slot writes bypass the heading guard entirely (the original corruption bug, unfixed on its original path — session 8's Bug A), and commit trailers are silently dropped on the entire real write path (the feature has never worked through the CLI; commit is exempt from the round-trip fixture suite).
- **The fan-out core held under its first live run** — isolation proven load-bearing (a deliberate symlink cross-wire visibly blew up), fail-safe same-path collision block at finalize, single-commit join, worktree guards — "directly retires the migration-era fear." Residue: `join` is docs-only while its surface implies merged-outcome, worktrees lack runtime deps, and `start --workflow milestone-execution` is a dead on-ramp (all predicted or new-environment, none core).
- **The route floor is unanimously the most-praised feature** across all nine feedback reports; zero dangling-route reports; the adversarial pass confirms promote-clobber, base-mismatch, dirty-worktree, unmanage all correct with right exit codes.
- **M44's fixes survived their exact provocations**: path-hash task ids (17 same-directory migrations, zero collisions — "deterministic task IDs were a nice touch"), the fidelity boundary-guard (on a repo literally named `project-alpha-3.0`, zero version-scan noise).
- **The flagship gate fired for real**: `doc-code.symbol-exists` blocked an unprompted rename in a brownfield PHP codebase (session 1) — its first live PHP confirmation. And the identity-not-truth bound was rediscovered the same session, for the third trial running.
- **Four condition-keyed deferrals accumulated their 3rd–4th independent field demands** (checkpoint/gate-record mechanism · gate-command/evidence rung · acknowledged-findings ledger · managed-doc read-side) — the trigger-counting gap is itself a finding.

## Invocation-log analysis

**639 invocations, 100% on rc.8**, ~24 h wall. Verb families: `doc` 393 (set-field 120 · set-slot 94 · author 42 · show 37 · schema 31 · list 24 · add-item 23 · create 12 · remove-item 4 · retitle-item et al.) · `task` 85 (finalize 40 · validate 25 · list 8 · discard 6) · `validate` 59 · `start` 41 · **`milestone` 21 (create 3 · add-task 3 · add-from-spec 2 · provision 2 · execute 1 · join 2 · finalize 2 · discard 2 · list/list-tasks 3)** · `migrate` 19 (18 ok) · `describe` 9 · `workflow` 6 · `unmanage` 2 (both the Bug C recovery) · `status` 2 (guessed, nonexistent) · `ingest` 1.

**Exit codes:** 568×0 · 38×1 · 9×2 · 22×3 · 2×4. Notable clusters (session attributions in [findings-verification.md](findings-verification.md)): the write-verb blocking rejections exiting 1 where AGENT.md's one-liner says 3 (the taxonomy finding); the two exit-4 fidelity review holds (`--approve` flow, worked as designed); the adversarial session's deliberate provocations. Every read surface was exercised heavily — `doc show` 37 / `doc schema` 31 / `doc list` 24 — a step-change from lacon's thin usage: **the M43/M44 pull-tier fixes demonstrably moved read behavior.** First-ever field writes of `maps-to-test` (session 6) — zero across ~1400 invocations in prior trials.

## Session inventory

| # | Session (handover design) | Landed | Feedback highlights |
|---|---|---|---|
| M | GSD→jigc migration (~17 adoptions, incl. the PROJECT.md 3-way split) | adoption commits through `3bb0224`… | the corruption-on-failed-write report (→ verified as Bug A's item-slot path); title-names-symbol noise; split-pattern + N-open-tasks undocumented |
| 1 | Code + doc↔code gate (single-task on an anchored symbol) | `beeefa8` | trailer drop (verified dead-on-real-path); anchor gate fired on rename; identity-not-truth rediscovered |
| 2 | Ship-gate closure (decided-task, test-first) | `c0ca587` | "the gate isn't a gate" (deliberate deferral, 3rd demand); dev/decided fork routing |
| 3 | Planning workflow end-to-end | `aa254aa` | checkpoint prose-only; baseline-verification instruction "earned its place"; de-defer path unnamed |
| 4 | Completion workflow (honest RED) | `e6a64eb` | owner-artifact prose lie (verified law-1); validate-before-finalize "the hero" |
| 5 | Reference graph (grounded-in / cites / derived-from) | 4 commits (`9f0dc21` `6c28a54` `5f06439` `f819a66`) | form-vision empty render (verified: role-binding trap); plan omits derived-from; `#type` claim (refuted) |
| 6 | spec → implement-from-spec → changelog | 3 commits (incl. `0893792` `158a4dd`) | Pest anchors unresolvable (parked-idea trigger fired); trailer drop re-hit; maps-to-test lifecycle undocumented |
| 7 | **Milestone fan-out, live** (5 parallel worktrees) | `632e076` + 6 bookkeeping | isolation real; join docs-only gap; vendor/ runtime gap; dead `start` on-ramp |
| 8 | Adversarial probe (throwaway branch, deleted) | none (by design) | Bug A confirmed at `####`; Bug C (dangling file-state); exit-code taxonomy; guardrails held under attack |

## What held (prior fixes this trial confirms)

M44 path-hash + fidelity guard (exact-provocation survivals, above) · M41 V1 fold-safety (heavy batch-author, zero folding corruption) · the `##`/`###` section-slot write guard's **atomicity** (session 8 Bug B: atomic reject, task usable, clean retry) · decided-task un-hidden (M43 — run first-try by name) · the staged read `doc show --task` (M43 — used productively) · the route floor (M41/M43 — universal praise, zero dangling routes) · idempotent create/copy-in over committed docs (M16/M43 — `existed: true` verified thrice) · finalize manifest + `left_out: []` (M42) · `.task` JSON / doc schema / addressing (M41–M43 — "I got doc structure from jigc, never by reading files") · the fan-out core (M31/M39 — first live confirmation) · PHP `symbol-exists` (M27/M40 — first brownfield fire). **Not exercised:** the carryover gate (planned in session 8, not covered in its report) — *unconfirmed, not failed*.

## Honest bounds

- **Not the clean-room protocol.** The M44 fork-5 direction (structural absence of this repo's checkout + unseeded probes) was **not** followed: sessions ran from written handovers that carried prior-session knowledge forward. Sessions 4 and 6 explicitly *routed around* the staged-write corruption from memory ("my whole authoring strategy was shaped by fear of it") — so on those surfaces, "no recurrence" means *avoided*, not *survived*. Session 8 then went back and confirmed the bug live at the `####` depth. The trial's within-corpus contamination is the inverse of rerun-rc7's archive contamination — probes were not seeded from this repo's artifacts, but were seeded from each other.
- **The fan-out probe counts, with residue.** Session 7 is effectively the chartered live-spawn probe: genuinely parallelizable work, real concurrent worktrees, a designed collision probe. Unit verdict: **sound at the core** — the residue (join contract gap, runtime deps, on-ramp) is fix/design-shaped, not architecture-shaped. Whether this fully discharges the chartered probe, or a clean-room re-probe is still owed for the avoided surfaces, is the human's call at the M45 charter.
- **Session 8 left state**: it applied `jigc unmanage` to clear the dangling file-state entry it diagnosed (its "state note"); its throwaway branch is deleted.
- The per-session commit attribution in the inventory above is reconstructed from the sessions' self-reports + git history; sessions 5/6 share the multiple-domains arc and their boundary is approximate.
