# Facts for the trial arms and the inconsistency arm's occasion

*Returned 2026-10-06 by a read-only exploration. Transcribed by the orchestrator. Paths are repo-relative; `R` is `completions/artifacts/RC-rc24/README.md`. Nothing was run.*

## 1. The arms as they ran on rc.24 (R:104-110, 135-139)
- **(a) fan-out** — steered, two turns, clean corpus: class A-1 reached, 10 min 18 s; `milestone execute` never used (R:349).
- **(b) amend-correction** — steered, two turns, clean corpus: B-1 reached, 1 min 58 s.
- **(c) inconsistency** — unprompted, one prompt, the corpus with its standing wart: **C-5, not reached**, 1 min 50 s. Classes (R:199-204): C-1 "filed through the channel", C-2 "recorded through jigc, by another door", C-3 "fixed silently", C-4 "mentioned only", C-5 "missed", C-V "void".

## 2. Why (c) missed
The wart "states nothing the code contradicts, only a role the code does not implement", so the evidence "cannot separate *missed* from *saw no disagreement to report*" (R:248-250). The worker reconciled both sides "by interpretation" (R:227). There was no debrief: the arm ran outside `seed` (R:1519). Owed: "A class table that can hold arm (c)'s behaviour" (R:1609).

## 3. Candidates for an unambiguous occasion
- **(i) A standing contradiction:** flat if two literals disagree; must be *noticed* (prose is not validated); buildable by a scripted `--exec` arm like `adopt.sh`. The best of the three.
- **(ii) A dangling anchor:** pushed at the agent and routed to a fix by the product itself — "update the citation… or restore the cited symbol" (`crates/engine/src/probe.rs:538`). It measures the product's route, not the agent.
- **(iii) A mid-session plant:** its commit moves HEAD under an open task and fires `finalize.base-mismatch` (`design/finalize.md:230`). `seed` passes no `--cid-file` (`completions/trial-driver/driver/session.py:206`), so a plant excludes a debrief.

## 4. What the fix pass changed that each arm reaches
- **(a):** `provision`, `finalize`, and `milestone.unlanded-work` if work stays unstaged (*inferred*); never `discard`.
- **(b):** the amend's finalize over the copied-in record, on the happy path.
- **(c):** a C-1 outcome's create runs the changed `new: true` gate (`c0c4d88c`).
- Hand-edited docs and the linked-worktree guard: only if a worker strays.
- `setup` runs scripted in `adopt.sh`; `uninstall` runs in no arm, and with the invocation log on it refuses without `--force`.

## 5. Cost
Arms and debriefs ran 22:19Z–22:38Z (R:3). A registry image build took 35 s, its verification 18 s (`completions/artifacts/M54/publish-proof.md:263`); the time of a source-mode build is not recorded. A candidate identified by a commit needs `build-image.sh <sha>`, `verify-image.sh`, a gate record and fresh corpora.

## 6. What the trial tooling cannot do
No mid-turn injection; a question ends the turn; a halt exits 0 (`completions/trial-driver/README.md:167-177`). `observe` flags a correct amend as `HISTORY REWRITTEN` and misses `git -C … commit` (R:1306-1308). The trial tooling has no commit since rc.24.

## 7. Traps
- The workflow's own `usage:` says "you are not reconciling them now" (`crates/cli/packs/methodology/workflows/report-inconsistency.yaml:3`): where fixing is in scope, C-3 (fixed silently) is designed behaviour.
- Only one catalog line names the channel (R:902).
- `fork` refuses a changed tree (`session.py:368`), so there is no between-turn plant.
- Minting a report from a fan-out worktree is refused (`e0f00278`): keep (c) apart from (a).

## Options for the occasion (the explorer's)
- **A.** A managed ADR states one literal, the code another (example: a cap of 1 000 against `maxSamples: 10_000`) · built on the clean-prose corpus, adopted, by a scripted authoring arm · rehearsed by a check that both literals exist, a walk arm that files from that state, and one non-blind turn asking what disagrees; "fired" = both sides read, confirmed by the debrief.
- **B (the explorer's recommendation).** A, plus a seeded second turn that asks for a record in outcome language · the same build, through `seed` · the same rehearsal; the unprompted and the asked halves are scored separately.
- **C.** A foreign, hand-committed ADR, as `completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr.sh` did · applied before the session · the same rehearsal; it adds an ingest question (*inferred*).

**Arms for the coming round (the explorer's):** (a), (b), and (c) as option B. No blind fourth arm; `setup` and `uninstall` are covered by scripted walk arms.
