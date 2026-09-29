# The pre-trial surface-comprehension review — rc.7, 2026-07-17

**Method (human-directed, 2026-07-17):** before spending the rc.7 verification trial, the whole emitted CLI surface was captured verbatim (49 captures through the installed `1.0.0-rc.7` binary in throwaway repos — [surface-corpus-rc7.md](surface-corpus-rc7.md)) and put through **two independent comprehension reads**: (a) a **blind fresh-eyes agent** (Claude-family, zero project context, only the corpus, working the trial's real decision tasks — routing, recovery, the write path, the commit boundary), and (b) a **cross-model review** (OpenAI codex, gpt-5.5) grading the corpus against [surface-contract.md](../../../design/surface-contract.md)'s style guide. Rationale: the rc.5 precedent — running a trial on known defects pollutes it with findings already in hand; and a cross-model read catches phrasing one model family happens to tolerate.

## Convergent findings (both reviews independently)

1. **The decided/single/dev-task catalog tie (would-cause-wrong-action).** The blind agent genuinely guessed on "add caching — decide the eviction policy" and flip-flopped after `describe`. Root cause (both reviews): the discriminator that matters — the decision's **destination** (decisions-log entry vs ADR vs none) — is invisible at the router tier; the words that differ ("test-first"/"end-to-end", "decision"/"decisions") discriminate least. The documented-symbol caveat likewise lives only in `describe`.
2. **The `docs: docs:` doubled subject (ambush, reproduced).** No surface states the subject renders `<type>: <summary>` or warns against a self-prefixed summary; the doubled subject committed clean with no finding. The blind agent: *"I would have written that commit."*
3. **The heading-depth ceiling reaches only the migrate path.** The seam-generated statement rides the `{{schema:}}` generator (migration templates); ordinary `set-slot` solicits and help are silent — a law-3 hole in the M43 batch's own coverage.
4. **The required-slot route is not copy-runnable and claims falsely.** "this finding's target is the address" while the printed target is a file path + section name; both reviewers reconstructed `adr:<slug>#<section>` by convention.
5. Smaller: the staged-read route omits `jigc task list` · the post-finalize changelog advisory leads with dead in-task commands · slug caps absent from `doc create --help`.

## Blind-only findings (credible, accepted)

- AGENT.md's "managed docs are exactly the `jigc doc list` set" does not compose with staged docs (not yet in the set → literally read-free) or `unregistered` rows (membership undecidable).
- `doc author`-after-incremental-verbs rejection stated only in one compose, not `author --help`.
- "Retire the foreign original" mechanics unstated (deleted? same commit?).
- `task discard` silently drops staged docs — the style guide's own ack rule ("what a discard threw away") was never implemented.
- No exit-code taxonomy on any surface (1 error · 2 usage · 3 blocking findings · 4 review hold — reverse-engineered by both readers).

## Certified clean (both reviews)

The task loop (resume / what's-left footers on every compose — "best-in-corpus"; the blind agent answered the whole task-model section with total confidence) · the errors-that-teach routes (`store.not-found`, enum rejection, wrong-id, carried-staged — recovery was "almost entirely route-following") · the write/read split · the migrate compose's self-sufficiency (the `--approve` gate explained before it can fail) · staging honesty.

## Disposition (human call, 2026-07-17)

**Fix all confirmed + cheap before the trial** — the full batch (convergent 1–5 + the five blind-only items), all prose/route/help tier plus one small route-address render. Landed as the pre-trial surface-polish commits on main; the trial then runs on the polished binary.

---

# Round 2 — the milestone/operator/route tail (2026-07-17, same day)

Round 1 covered the first-contact core loop; the human asked the honest coverage question ("did we verify ALL our prose?") — answer: no. **Round 2** captured the tail: the milestone lifecycle end-to-end incl. a Spawn: line executed verbatim ([surface-corpus-rc7-r2a → cli-surface-corpus-r2a.md](cli-surface-corpus-r2a.md), 41 captures), the 11 remaining composed workflows incl. a real plan→spec→bind→implement drive, two more generated migrate templates, 12 `doc schema` listings, and the **complete static route inventory** ([cli-surface-corpus-r2b.md](cli-surface-corpus-r2b.md), 1847 lines). Same two readers: the blind fresh-eyes agent + the codex cross-review.

**The convergent top findings — heavier than round 1's, all on the probe's terrain:** the milestone execute compose asserted "All sub-tasks are complete and have been merged" as static step text before any work ran (both readers' #1) · `milestone finalize` succeeded with **empty stdout** at the highest-stakes commit boundary (the blind reader fell back to raw `git show` — the exact drive-around the tool exists to prevent) · `join` taught nowhere · **two surfaces taught opposite sub-task models**, and verification proved the doc.rs route not just false but **destructive** — `jigc task finalize <sub-id>` landed a commit on the detached worktree HEAD and the milestone finalize silently lost the work while the record claimed joined · the bind `<SPEC_ID>` placeholder invited a rejected shape (+ the project-setup `brief#` sibling) · `config set docs-root` relocated committed files (including a just-unmanaged one) with no solicit warning · `upgrade` printed "the task validates clean" on a config sweep, byte-identical with zero and with N deltas.

**Disposition (human): full batch + verify suspects.** Batches C+D landed as 7 commits (`a092908`…`ff7b5e2`): the milestone truth pass (state-honest execute + join taught + the landing manifest + contribution-naming join ack) · the **sub-task finalize guard** (`finalize.milestone-sub-task`, the promote-clobber refusal class — the verified work-loss path now refuses with the milestone route) · the operator honesty pass (unmanage/docs-root/upgrade surfaces state the committed-truth relocation basis, staged-`git mv` state, and the real sweep noun) · the route repairs (the no-criteria key + executable arm, the honest create-gate route, the sanctioned hand-edit clause) · the pack fixes (bind address form, prd write address, the payload whole-pair + repeat-per-item rules in the generation seam). **Five route-tail suspects refuted as static-extraction artifacts** — the rendered bytes were correct; the parse fence held. Re-verified **2149/0**.

**Carried forward (recorded, not fixed):** the project-workflow-shadow compose/gate divergence (the compose's `create-gates:` line reflects the shadow while `doc create` enforces the pack list) — decisions-pending; and `record-change`'s router invisibility (suppressed by design with a declared reason; the finalize advisory routes to it — watch whether the trial demands more).

---

# Round 3 — the full per-surface conformance census (2026-07-18)

Rounds 1–2 were sampled comprehension reads; the human asked the strict question — *does **each** command output follow **each** rule?* Answer then: not verified per-surface. **Round 3** built the complete answer. An enumerator derived the whole output-surface inventory from code at HEAD ([surface-inventory.md](surface-inventory.md), ~460 rows across four parts: verb outcomes, finding surfaces, error identities, composed/generated), marking each row's corpus evidence. Three graders then swept the matrix row-by-row against the three laws + the style guide, verifying against HEAD code (the corpora trail by ~46 commits, treated as weak evidence), flagging `UNVERIFIED` rather than assuming.

**Result: ~350 PASS · ~30 FINDING (≈7 root causes) · ~57 UNVERIFIED** (dynamic-route/new surfaces needing a rendered capture — not defects). No would-cause-wrong-action defect survived rounds 1–2; the census caught latent + consistency-class defects a trial would not surface.

**The load-bearing find — an open instance of the class M43 claimed closed.** Seven blocking findings shipped `route: None` while not route-exempt; a grader **refuted the inventory's own "never reaches the seam" framing** — five DO reach the serialization seam (`finalize.render-io/promote-io/provenance-io/source-path-io`, `doc-code.multi-valued-anchor`), so a debug binary panics the route-floor assert and a release emits a route-less blocked finalize — the exact no-recovery state the M43 floor was widened to kill. The M43 retrospective minted the seam-sweep rule (*every producer swept through the seam, or argued-enumerated*) but discharged it for only 4 producers; these 7 were never swept. **Fixed and the class closed by construction** (`6716465`): all 7 routed, plus a source-scanning test that fails the build for any new route-less blocking producer whether or not a test serializes it — the genuine discharge of the rule, now complete.

**The consistency batch** (`9c7a6d8`/`be5710b`/`4ce33a2`/`fea876b`): the 6 `config-*` verbs now ack their effect (the widest silent-success instance, `config.rs:151`) · `migrate-corpus --dry-run` says "would migrate / nothing written" instead of a past-tense lie byte-identical to a real run · `config.undeclared-key` names the declared knobs instead of routing to a command that reveals nothing · the cheap half of the route-fence-bypass population runs through the checked `Route::mechanical` constructor (the rest noted-not-forced, per the contract's own bound). Re-verified **2158/0**.

**Cleanly refuted by the census:** no route teaches a non-parsing command (rounds 1–2 held) · no A14 path-lie sibling · the finalize/validate stream asymmetry (json→stdout) is intentional and correct · silent-success was confined to config · the created-vs-existed ack, the "no action needed" advisory tag, and the milestone-finalize manifest all pass.

**Carried (UNVERIFIED, not defects):** ~57 rows — dynamic-route composites (`milestone.record-exists`, `finalize.base-mismatch`, the `ingest.rs` engine-code re-wrap), the doc-code probe binary's own findings (separate crate, outside the engine seam), and the newest acks with no rendered capture. Settling them needs rendered instances, not code fixes; the trial itself will render most. This is the honest coverage bound: the census verified every surface against the rules from code + the fences that enforce them, and rendered-byte confirmation exists for the ~90 surfaces the corpora captured plus every surface a fix touched.
