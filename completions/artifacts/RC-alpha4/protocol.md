# The final unseeded trial — protocol (prepped 2026-07-25, pre-trial)

**Provenance:** DECISIONS → 2026-07-24 *The road to 1.0* (2): the last trial before the 1.0.0 call — deliberately what no trial has been: the **unseeded clean-room half** (the only never-run half of the M44 fork-5 protocol; the rename identity-break is proven), the **pre-trial environment matrix's first binding occasion**, and the two never-exercised probes (the carryover gate · heavy batch-author over the fixed corruption edge). Composes with M45 fork 11 (no standalone re-probe — the probes ride here). Binary under trial: **`jigc 1.0.0-rc.9`** (installed at the confidence-audit wave close, `ee5ca72`/`a10e555`). Corpus: `~/ideas/project-alpha-4.0` — a fresh copy of the trial corpus, version-bump rename per the proven project-alpha-3.0 identity-break pattern; verified pre-trial: `.planning/` present, no `.jigc/`, pristine after the prep scrub below.

## Roles and the clean-room rules

- **Operator (the human):** runs the prep commands, starts sessions, pastes the worker prompts verbatim, intervenes never.
- **Observer session (optional, separate):** may read this file and the jigc repo; designs any additional probes; **never** edits the corpus mid-trial and never feeds workers anything beyond the verbatim prompts below.
- **Worker sessions (blind):** started fresh with `cwd = ~/ideas/project-alpha-4.0`, given ONLY a verbatim prompt from §Probes. They must not be given: this file, the jigc repo path, any `completions/artifacts/` content, any prior trial record, or any handover summary. The corpus itself must contain no such residue (structural absence — the prep scrub removes the one found: `.claude/subagent.log`, which referenced a project-alpha-2.0 transcript path). **Project-alpha-3.0's contamination — sessions carrying handover knowledge — is the anti-pattern this trial exists to avoid; if a leak happens, record it, don't restart-and-hide it.**
- **No mid-trial fixes.** Findings are recorded, never repaired while the trial runs.

## The environment matrix (first binding occasion)

Per the process rule (RC-alpha3 findings-verification → process change 7): each corpus property → the designed probe, and which **declared bounds must be printed before an agent walks into them**.

| Corpus / run property | Probe or watch | Declared bound to have in hand |
|---|---|---|
| Real git hooks live in the corpus (`.husky/`) | Hook-rejected finalize will occur naturally; the fresh `hook_output` producer axis and survivable-rejection frame run against real hooks for the first time since the sweep | — |
| PHP/JS monorepo | `maps-to-test` authoring hits Pest closures — the M45 rider (reworded hint · file-only fallback · split message) is what agents should see | The is-a-test tier is deferred (decisions-pending → M46 entry 5); Vue `symbol-exists` arm exists (M41) |
| 4th run of the same GSD corpus | Contamination is itself under test — the unseeded discipline above; the record closes with an explicit honesty statement on what leaked | — |
| First field run of rc.9 | The confidence-audit surfaces are live: `schema-version-ahead`, the rollback capture discipline, staged reads, the golden-swept router | — |
| `node_modules/`, `.env`, `.phpunit.result.cache` present | Watch only: gitignored trees stay out of the ingest funnel (closed M40); `.env` must never be staged by the tool | — |
| Bounds an agent may walk into (print if adjacent) | — | The matching-label OOB read-side arm (declared open) · item-slot corruption closed over the **write path only** (no repair verb for pre-guard bytes) · lost-update semantics under fan-out (M46 entry 9) · sparse-checkout false-deletion block (n/a on this machine unless used) |

## Probes

- **P1 — the migration half (unseeded).** Worker prompt, verbatim: *"This project's planning docs live in `.planning/` (GSD style). Migrate the project to jigc management using the installed `jigc` CLI — start from `jigc setup`, then `jigc config set invocation-log true`, and follow what the tool tells you. Finish when the corpus is managed and `jigc validate` is clean."* (Setup itself pre-dates the log — same as every prior trial.)
- **P2 — the carryover gate (owed: planned in two trials, run in zero).** Operator, before starting the worker: stage state in the corpus by hand — one modified file `git add`ed, one `git rm`ed (a staged deletion), one untracked file added and staged. Worker prompt: any ordinary task prompt (e.g. *"Record the decision to keep the session store in-memory as an ADR using jigc."*). Expected: task mint snapshots the staged set; finalize refuses with one blocking `finalize.carried-staged` per carried path; the `--carry-staged` override lands them labeled `carried-over` at the render sites; the staged deletion is **not resurrected**.
- **P3 — heavy batch-author over the fixed corruption edge (owed).** Worker prompt: a batch-authoring task that touches item slots at depth (e.g. *"Author the project roadmap and changelog under jigc: create them, then populate — several milestones with multi-slot items, several releases with nested change entries — from the repo's history."*). Expected: reserved-depth headings in item prose are atomically rejected with the `jigc task discard` route named; zero silent corruption; byte-stable re-reads.
- **P4 — blind implementation probes.** At least one ordinary implement-from-intent session on the migrated corpus, prompt designed by the observer at trial time (sealed until run — do not pre-write it into the corpus).
- **P5 — fan-out: only if a milestone arises naturally.** The live confirmation exists (project-alpha-3.0); not owed here.

## Operator command list (prep — run before the trial session)

```sh
# 1 · scrub the cross-run residue from the copy (clean-room structural absence)
rm ~/ideas/project-alpha-4.0/.claude/subagent.log \
   ~/ideas/project-alpha-4.0/.claude/settings.local.json
git -C ~/ideas/project-alpha-4.0 restore .planning/v1.0-MILESTONE-AUDIT.md
git -C ~/ideas/project-alpha-4.0 status --short        # expect: empty

# 2 · confirm the binary the trial runs on
which jigc && jigc --version                        # expect: ~/.local/bin/jigc · jigc 1.0.0-rc.9

# 3 · confirm the corpus is jigc-naive
ls ~/ideas/project-alpha-4.0/.jigc 2>/dev/null || echo clean   # expect: clean

# 4 · start worker session 1 (fresh session, corpus cwd, paste the P1 prompt verbatim)
cd ~/ideas/project-alpha-4.0 && claude
```

Between probes: P2's staging commands are the operator's (`git add` / `git rm` in the corpus before the worker starts). Everything else happens inside worker sessions.

## Per-session feedback (after every worker run)

At the end of each worker session — inside that session, while fresh — collect verbatim feedback with the standard prompt: *"What confused you, what did the tool tell you that turned out wrong, what did you look for and not find, and what did you work around without being told to?"* Save it to this directory (`feedback-<session>.md`), **never into the corpus**, and never feed it into a later worker's prompt — feedback is retrospective record for the post-trial triage only (the discoverability-lens findings of the last three trials came from these reports, not the log).

## After the trial

Trial record + invocation-log analysis land here (`completions/artifacts/RC-alpha4/`); every claim adversarially verified with live repros — **CONFIRMED and REFUTED alike ship repro blocks** (milestone-completion-workflow → delivery format); the record carries the **unseeded-honesty statement** (what, if anything, leaked); triage through the known-hole vs discoverability lenses; counts as RC input. Then the 1.0.0 call — the human's.
