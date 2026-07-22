# Verbatim feedback — the project-alpha-3.0 verification trial (rc.8, 2026-07-21/22)

> Archived verbatim from the untracked TODO.local.md scratch inbox at drain (2026-07-22). Session context lines (the handover briefs) are the probe designs; each "### Feedback" / "### Agent output" block is the executing agent's unedited report. Claim verification lives in [findings-verification.md](findings-verification.md).

# ~/ideas/project-alpha-3.0

## migration (session: migrate-gsd-to-jigc)

### Feedback

Happy to. This was a good stress test — ~17 migrations plus fixes, one recoverable corruption, and a fair amount of empirical discovery. Candid feedback below, grouped by your questions.

Overall: the core design is strong

The migrate loop is genuinely good to operate. jigc migrate <path> --as <doctype> staging the foreign bytes and emitting a schema-shaped batch template and inline mapping guidance ("foreign Overview → overview slot", "see-ADR → cites") meant I could author most docs without ever leaving the tool. Three things I'd call out as excellent:

- The finalize review-hold (fidelity diff → --approve) makes "the CLI guarantees structure, not content-faithfulness" concrete and safe. Atomic "retire foreign source + promote managed doc in one commit" is clean.
- Code-anchor validation (doc-code.symbol-exists blocking finalize) is the single feature that most enforces honest docs — a stale implemented-by can't land. That's exactly right for arch-docs.
- --format json on every verb made scripting 17 passes reliable, and the per-finding route: hints were mostly directly actionable. Deterministic task IDs (same id on re-mint after discard) were a nice touch.

Bugs / the one that actually hurt

1. A rejected write corrupts the task's staged doc (high severity). When a doc author failed with write.non-reparseable, the message said "nothing was persisted" — but the task's staged doc was left in a state where even a previously-valid payload could no longer be authored. I confirmed it: the minimal payload that succeeded moments earlier then failed identically. I had to jigc task discard + re-mint to recover. A rejected write must be atomic and leave the prior valid state intact. This cost me the most time.

2. non-reparseable diagnostics point at the wrong place (high). The real cause was #### headings inside my decomposition slot colliding with the slot's own #### Decomposition label — but the error located it at proves line 1 col 1 and said only "does not conform to the schema." The route claims "the message names the break," but it didn't. Naming the actual collision ("heading #### Built in decomposition collides with the slot heading level — use #####+") would have saved me a long bisect.

3. Slot heading-depth rule is documented as absolute but is actually doctype-relative (medium-high). Guidance everywhere says "headings must sit at #### or deeper." That's true for arch-doc slots (they nest under ### components) but wrong for roadmap, where the slot label itself renders at ####, so prose needs #####+. Either document the effective floor per doctype, or — better — auto-demote/renumber prose headings on write so authors don't have to model the render tree.

Where I got stuck / had to discover empirically

None of these are documented; I reverse-engineered them by testing:

- Anchor path base. The template says path#symbol but never that paths are repo-root-relative (not task- or app-relative). I authored ARCHITECTURE with anchors and ran a plain finalize just to confirm they resolved before trusting the format across 5 delegated arch-docs.
- Splitting one source into many doctypes. PROJECT.md → vision + prd + decisions-log. I didn't know whether the first --approve (which retires PROJECT.md) would break the other two tasks. I had to mint all three first and observe that mint-time byte-staging + idempotent retire makes it work. This is a natural migration need and is completely undocumented.
- Multiple open tasks. I initially assumed one-at-a-time, then found N tasks mint fine — which is what let me parallelize. Worth stating explicitly.
- Singleton title requirement. decisions-log/roadmap/deferral-ledger have fixed slugs, yet the author payload still requires title:, surfaced only as a runtime "malformed payload: missing field title." A stumble.
- quick-fix scope. For a one-line doc fix I wanted the lightest task. quick-fix worked (allowed a managed-doc set-slot + finalize), but its catalog description — "commit-only fix, changes no documented code, records no decision" — made me unsure it permitted doc edits at all. Clarify what it allows.

The over-eager advisory

doc-code.title-names-symbol will train users to ignore advisories. It flagged "Pest + PHPUnit setup," "WordPress API clients," and "Non-functional DeployJob zip store path" — because they contain the tokens "PHPUnit"/"WordPress"/"DeployJob," which it read as symbol names. Those are legitimate, readable component titles; renaming them to satisfy the heuristic makes the docs worse. It should only fire when the title token closely matches the component's anchored symbol, not any capitalized tech noun. As-is it's permanent noise on honest arch-docs.

Did I get everything from jigc, or research elsewhere?

For jigc mechanics: entirely from jigc — schemas, templates, mapping guidance, commit flow. I never needed external jigc documentation. Good.

For content: heavily outside jigc, by design and correctly so. The whole task hinged on reconciling drift against reality — reading the dependency manifests, walking app/, running Pest, inspecting the app Dockerfile. jigc is a doc compiler, not a code oracle, so "the code is the source of truth" is the right boundary. The one seam worth improving: implemented-by is the doc↔code bridge, but jigc gives no help finding or pre-validating an anchor — I relied on grep, and a wrong anchor only fails at finalize. A dry-run (jigc doc check-anchor "path#symbol") would de-risk that.

Prioritized change list

1. Make rejected writes atomic — don't poison the task; a failed author/set-slot should leave the prior staged state usable. (high)
2. Fix non-reparseable diagnostics — name the real break and point at the offending line, not the first slot. (high)
3. Handle slot heading depth doctype-aware — auto-normalize prose headings, or document the per-doctype floor. (medium-high)
4. Constrain title-names-symbol to the anchored symbol only. (medium)
5. Document the split pattern (N migrates on one source before approving; idempotent retire) and that multiple tasks can be open. (medium)
6. Hint the singleton title requirement and repo-root-relative anchor paths in the templates. (low)
7. Add an anchor dry-run verb. (low-medium)
8. Per-finding blocks: yes/no on jigc validate — the "3 gate at task-scope, the rest gate nowhere" summary line is easy to misread. (low)

Net: for a v1, the authoring ergonomics and the structure-vs-content safety model are already well thought out. The corruption-on-failed-write (#1) is the one I'd treat as a real bug; everything else is polish and docs. I'd happily run another migration through it.


## 1. Code-writing + the doc↔code gate (the core untested surface) - (session: test-task-1)

This is the biggest gap: I never ran a workflow that writes code. And it directly validates the anchors we just created.

- Task: "Implement the DeployJobStore 'Not implemented' path" or "Tighten ResourcePolicy authorization" via single-task. Both symbols are anchored in known-concerns-and-tech-debt.md.
- What it proves: does jigc's doc-gate actually force the arch-doc update when you reshape an anchored symbol? Rename DeployJobStore and see if finalize blocks on doc-code.symbol-exists and makes you fix the doc. If it doesn't, the coupling is cosmetic. This is the single most important thing to verify next.
- Bonus axis: run one as dev-task (test-first, no decision) and one as decided-task (a real choice → decisions-log entry, e.g. "require Redis in the test env vs. mock it") to compare the lighter flows and the decision-recording branch.

### Feedback

What I did: one single-task — removed a Not implemented kill-switch in an arch-doc-anchored symbol (DeployJobStore), fixed a null-deref it exposed, reconciled the managed docs. One clean commit shipped.

Verdict

I'd happily work with it again. The mental model — jigc owns structure, I author prose; every write goes through a verb; findings gate the commit — is coherent and calming: I always knew the next command and what would block me. start is a runnable checklist, finding routes are actionable, validate→finalize let me preview gates. The problem isn't ergonomics — it's that the flagship promise (docs stay true to code) is under-delivered, plus one real bug and a few sharp edges.

What worked well

- jigc start is self-documenting — handed me every command up front.
- Findings carry a route — the single best UX thing; every block told me how to fix it.
- The symbol-exists gate actually fires and blocks on rename. Real, not decorative.
- validate as a dry-run of finalize is the right design.
- Staged-index adjudication is principled and the route says so.
- retitle-item/remove-item exist and preserve {#id}; a rejected set-field on a title routed me straight to retitle-item.
- doc schema gave me all addressing — I got doc structure from jigc, never by reading files.

Bugs

1. 🐛 Commit trailers silently dropped at finalize (highest priority). add-item …#trailers + set-field …/value both returned success; the finalized commit had no trailer lines (%(trailers) empty). Had to git commit --amend --trailer to meet the repo's required Co-Authored-By/Claude-Session. Silent loss of an accepted write — worse than a rejection.
2. ⚠️ finalize --format json didn't parse as a single JSON document (Expecting ',' delimiter ~4KB in). Every other verb parsed fine. Observed, not root-caused — worth a check against the "every verb speaks JSON" contract.
3. (Prior session, shaped my behavior) a failed non-reparseable write corrupts the staged doc, needing discard+re-mint not retry. A rejected write should be a no-op.

Biggest gap — the coupling guards identity, not truth

The gate checks the anchor resolves to a symbol; it does not check the prose still describes what the code does. This session: renaming the class → blocking. Removing the throw (inverting the doc's "dead by design / uploads always fail" claim, symbol untouched) → zero findings. I could have shipped a managed doc that was now flatly false, silently. Risk: a green validate reads as "docs and code agree" when it only verifies names haven't moved. Anchoring a symbol implies a promise the tool doesn't keep.

Related — the workflow hides the docs a change threatens

single-task composed only the commit doc; it never surfaced known-concerns-and-tech-debt, the doc anchoring the very symbol I edited. The coupling data exists — the workflow could pull "docs anchoring symbols in your staged diff" into view proactively, not just react at the gate on rename.

Friction (non-blocking, but adds up)

- doc show --task on an unstaged doc returns a blocking-finding error envelope instead of the committed content — briefly looked like an error. Expected --task to fall back to committed.
- --format json shapes vary by verb (writes vs show vs error-envelope) — a stable top-level envelope would make scripting safer.
- Addresses are long — the task id is the verbatim intent slug; every line carries ~40 chars + doc:slug#section/item/leaf. A short task alias would help.
- start is a wall of text — great reference, heavy first read; a terse default + --verbose would suit a repeat agent.

Did I get everything from jigc?

- Doc structure/addressing: all from jigc (schema, show, start, finding routes) — a genuine strength.
- Project source: read freely outside jigc (job class, subclasses, controllers, Version, models/factories, fixtures) — exactly as designed, not a gap.
- One thing jigc couldn't tell me: that removing the throw would crash on first upload — inherent (code fact), but it underlines that the anchored doc described the symbol wrongly ("fully-written, just gated") and nothing surfaced that unreliability. Ties back to identity-vs-truth.

If I could change three things

1. Fix the trailer drop — silent loss of an accepted write is the worst failure class.
2. Make anchored prose reviewable when the implementation changes (or at least surface anchoring docs in the workflow) — so green validate means more than "names didn't move."
3. Guarantee rejected writes are no-ops, and stabilize the --format json envelope across verbs.


## 2. Close a roadmap ship-gate for real (dev-task / test-first gate)

- Task: fix a cluster of the 33 failing tests (start with the Redis-in-test MetricsStoreService group), or bake the missing tooling into the app Dockerfile.
- What it proves: the test-first "gate" step, the commit boundary, and — nicely — it actually moves the honest roadmap toward closeable. Watch whether jigc's gate meaningfully enforces the test-first discipline or is a rubber stamp.

### Feedback

Context: stress-test of jigc on project-alpha-3.0. This session ran one full decided-task end-to-end: reproduce a failing-test cluster → route it → implement test-first → record a decision → finalize one commit (c0ca587). Versions: Pack dev/1.0.0-rc.8 | methodology/1.0.0-rc.8.

TL;DR

Smooth and predictable — I never got stuck, and the composed workflow prose is genuinely good (it bakes in test-first, minimal-scope, "ask before solving the wrong problem"). Two headline items:

1. The "gate" isn't a gate. jigc never runs or verifies the project's test/lint/build. A red suite finalizes exactly as happily as a green one.
2. The dev-task vs decided-task fork is the sharpest, most consequential decision in the flow — and jigc offers no help making it, nor a cheap way to re-route if you picked wrong.

What worked well

- Self-documenting surface (doc author --help, doc schema, start orientation) — never had to guess a slot name or section id. AGENT.md's "ask the binary" advice paid off.
- JSON everywhere + stable .task path. jigc start --workflow decided-task "<intent>" in one shot worked.
- Incremental doc verbs landed first-try. create over the already-committed decisions-log correctly copied-in-and-appended rather than clobbering.
- finalize manifest (added/modified/promoted + explicit left_out: []) is clear and reassuring; the staged-copy → promote model is clean.
- One-commit-per-task with a structurally-authored commit doc is a nice discipline.

What was unclear

- "Touches no documented code" is unverifiable by the operator. The dev/decided boundary hinges on whether the code is arch-doc-anchored — I had to reason it out by hand. jigc knows the anchors; I can't ask it.
- What "the gate" gates on. validate/finalize check only doc/file-state, but the workflow prose ("a green gate is what separates a finished change from one that merely compiles in your head") strongly implies jigc enforces the test suite. It doesn't.

Where the friction / gaps are

1. The gate is discipline-only (biggest issue). The prose tells the agent to run the project gate and confirm green — but nothing observes it. finalize didn't object to landing while 21 other tests were still red. So "the gate" is neither my change nor the whole suite — it's "the docs are well-formed." Suggestion: let project config declare a gate command and have finalize run it and block on non-zero — or at minimum record the command + exit code in the commit doc as an evidence trail.

2. Routing is all on the operator, with no cheap correction. The most important finding this session: the handover said "start dev-task on Redis" — wrong, because making those tests pass forces a test-architecture choice (a repo-wide pattern), which is a decided-task. The fork bit before a line of code, and a careful prior operator mis-routed it. Asks: (a) when start looks like a code change, ask the one disambiguating question ("does this encode a decision worth recording?"); (b) make mid-task re-routing first-class and documented — mid-implementation discovery of the fork is the common case.

3. Surface doc-anchoring. A jigc doc anchors <path> query + a finalize warning when an anchored symbol changed without its doc — turns "touches no documented code" from honor-system into something checkable, and gives the natural signal for "this should've been decided/architecture."

4. The composed workflow is one large prose block — well-written, but prose I have to parse and self-track. Consider emitting an ordered steps[] array alongside the human text.

Bugs

- None reproduced this session. Every command behaved as documented.
- One reported-but-not-reproduced item to verify: the handover warned that a rejected doc author/set-slot can corrupt the task's staged doc so even a valid retry fails (recovery = discard + re-mint). I didn't hit it. If real, "a rejected write leaves the staged doc unrecoverable except by discarding the task" is a serious data-loss-shaped bug — worth an explicit test.

Did I get everything from jigc, or research elsewhere?

Both, and the split felt correct. jigc supplied the process (steps, schemas, grammar, commit structure) — complete and sufficient. I researched the codebase myself (failing tests, service source, phpunit.xml, Redis reachability) as AGENT.md intends. The notable bit: every decision-relevant fact came from my own investigation, not jigc — its context is doc-scoped, so it couldn't inform the very dev-vs-decided routing its taxonomy hinges on. That's an inherent boundary, not a bug — but it's why #2 and #3 matter: since jigc can't see the facts that pick the route, it should make the route cheap to correct and the constraints cheap to check.

Did I get stuck?

No. The flow was linear and each next step was clear. The one real decision point (the fork) was mine; jigc neither blocked nor helped — it got out of the way, which is fine (see #2).

---
One note on scope: this is grounded in the one decided-task I ran this session — I haven't yet exercised dev-task's linear no-decision path, so finding #1 (the gate) is confirmed for decided-task but only inferred for dev-task (same prose). If you want that verified before you act on the feedback, the SQLite view-migration cluster is a clean dev-task I can run next.


## 3. Milestone planning loop + human checkpoints

- Task: jigc start --workflow planning "Ship Project-alpha 2.0 Foundation" — scope the remaining ship-gates into an increment.
- What it proves: the human-gated Settle checkpoint, gap detection, decomposition, and the running-doc maintenance (it edits the roadmap/deferral-ledger/decisions-log we authored). Entirely different machinery — Checkpoints, loops — from the linear migrate flow. Watch how the checkpoint halt surfaces to me and whether it edits existing singletons cleanly.

### Feedback

Context: Ran planning end-to-end on a real open milestone ("Project-alpha 2.0 Foundation") in a PHP/JS app. Full walk: scope → detect-gaps → settle (checkpoint) → review → decompose → record → finalize. Landed one commit (aa254aa, 3 managed docs). Field report, ordered by what matters most.

Verdict

It worked. I planned a real milestone, halted at the human gate, recorded three decisions, re-authored an existing singleton without corrupting it, de-deferred two ledger items, and committed — all through jigc, without fighting it. The composed walk was self-contained enough that I never had to research how jigc works from outside; every command I needed was in the walk text or one --help away. That's the headline: the context-compiler model delivered. The issues below are refinements, not blockers.

What worked well (keep this)

- Baseline-verification framing caught real drift. The scope phase's "exercise the real binary, don't trust the roadmap's prose" was the single most valuable instruction. The roadmap said 33 failing / 362 passing; the live suite was 27 / 375 — a later fix had closed a cluster. Had I trusted the doc, every increment would've been cut against a stale baseline. This instruction earned its place.
- Re-authoring an existing singleton is clean and safe. jigc doc create roadmap on an already-committed singleton returned existed: true and copied it into the task rather than erroring. Then set-slot on the existing item id updated in place — no duplicate milestone, no clobber.
- Findings carry a route. The store.not-staged finding told me the exact fix (jigc doc show roadmap:roadmap). Every blocking finding did this. It turns errors into next-actions.
- The address grammar is learnable and consistent. roadmap:roadmap#milestones/<id>/proves composes predictably across show / set-slot / set-field / remove-item. Never guessed wrong after the first.
- validate before finalize is the right affordance. It showed me the two blocking commit-doc findings (empty type, empty summary) before I tried to commit. No surprises at the gate.
- finalize did exactly what it said. Promoted the 3 managed docs, left_out empty, subject rendered correctly from type/scope/summary.

Bugs / rough edges

1. The checkpoint is advisory text, not a mechanism. Checkpoint: settle was a line in the middle of the composed walk. Nothing in the engine gates it — no state tracked, no exit code, no pause. A less careful agent would sail straight through the human gate and self-approve, and jigc would neither know nor object. For a "human-in-the-loop gate," this is the biggest gap: the gate exists only in the prose's good intentions.
  - Related: no documented resume mechanic. The handover explicitly wanted to test "how the halt presents — exit code, prompt, resume." Answer: it doesn't. I only got through because I stayed in one session. If I'd stopped at Settle and later re-run jigc start --workflow planning, would it resume mid-walk or restart? Never surfaced. A checkpoint that can't resume across sessions isn't really a checkpoint.
2. The Record walk omits remove-item — but planning routinely needs it. Two ledger entries were pulled into scope by Settle, so they're no longer deferred. The Record step documents only appending to the ledger. remove-item exists and worked perfectly, but I discovered it via jigc doc --help — the walk never mentions the de-defer path, even though "planning pulls a deferred item into a milestone" is a first-class outcome the walk itself describes in Settle.
3. jigc task show doesn't exist. Reached for it reflexively to inspect task state; got a usage error. Verbs are list / diff / validate / discard / finalize / bind. Minor, but show is the obvious name for "what's in this task."

Unclear / had to interpret

1. "Resolve it... decisions via the design workflow." Settle says record decisions "via the design workflow," but the Record phase hands me concrete decisions-log doc verbs. Invoke a nested sub-workflow, or author directly? I authored directly and it worked, but "via the design workflow" implies a sub-workflow invocation that never materialized as a command. Pick one story.
2. The planning vocabulary is coupled to CLI/engine projects. The gap-detect/settle prose assumes a tooling project: "spike the new shape against the real binary," "doctype schemas no workflow has yet driven," "engine/CLI surface the milestone assumes," "acceptance flows — its invocation commands included." Planning a web app, I had to translate binary → app/test-suite, engine/CLI surface → routes/controllers, invocation commands → HTTP flows. Intent carried fine, but a first-timer on a non-CLI project will feel the prose was written for someone else. Neutralize the vocabulary, or let the project declare its domain so the walk speaks the right nouns.
3. Batch-vs-incremental guidance is repeated three times, verbatim-ish. For roadmap, ledger, and decisions-log, the Record walk each repeats the full "author is the batch alternative — run INSTEAD of create + per-entry verbs, never after them" paragraph. By the third repeat it's noise. Say it once, then reference it.
4. add-item id truncation is lossy and silent. "Green-suite gate stays on hermetic SQLite by guarding the WP-updates view" slugged to green-suite-gate-stays (first 5 words), dropping the meaningful half. --slug is the escape hatch, but the walk doesn't hint that long titles truncate — you notice after the fact.

Did I get everything from jigc, or research elsewhere?

- jigc behavior: entirely from jigc. The walk plus --help on doc, doc author, doc create, doc remove-item, and task covered every question. Never read jigc source or guessed. The "ask the installed binary" principle held.
- Project ground truth: researched directly, as required. Ran Pest, grepped routes, read the Dockerfile, read a form request and its frontend form, checked git history — all to verify the baseline the scope phase demanded. Correct division of labor (jigc owns docs; source is read freely), and the walk was explicit I should. So: yes, I researched the project, never jigc.

Concrete suggestions (prioritized)

1. Make the checkpoint real. Track checkpoint state on the task; halt with a distinct exit code or a jigc task resume affordance so the human gate survives a session boundary and can't be silently skipped.
2. Teach the Record walk to de-defer. When Settle pulls a deferred item into scope, name remove-item (or a resolve/promote verb) as the way to reflect it.
3. Reconcile "design workflow" vs. direct decisions-log authoring. One story for how a Settle decision gets recorded.
4. De-domain the planning prose (or make it domain-aware) so non-CLI projects aren't reading engine/binary vocabulary.
5. Trim the 3× repeated batch/incremental caveat in the Record phase.
6. Add jigc task show (state summary) and a truncation hint on add-item.

Net: a strong v1. The compiler model works, the doc surface is coherent, and the instruction that mattered most (verify the baseline for real) is exactly the one that paid off. The gaps are concentrated at the checkpoint machinery — fittingly, the one part of this workflow that was never exercised before.



## 4. The completion loop — build the record we deliberately skipped (Session: test-task-4)

- Task: once gates are (or are declared) closed, jigc start --workflow completion "Project-alpha 2.0 Foundation": audit → triage → fix → re-verify → author the completion-record.
- What it proves: the audit machinery, the human-gated triage/fix halts, and specifically the owner-artifact artifact-ladder resolution I dodged during migration. It closes the honesty loop the right way (an earned record vs the fabricated GSD one).

### Feedback

Context: ran --workflow completion end-to-end on the open milestone "Project-alpha 2.0 Foundation" — fresh audit → two Checkpoints → partial fix-round → completion-record + decisions-log → finalize. Landed one commit (e6a64eb), verdict RED. Below is my honest experience as the agent driving it.

---
Overall

Genuinely good to work with. The single biggest win: jigc start --workflow completion "<milestone>" composed the entire loop as one self-contained brief — both the incremental and the batch authoring paths, with exact command syntax and the slug already filled in. I never had to guess an address. That plus --format json everywhere and errors-that-carry-routes meant I was rarely guessing. I'd describe the ergonomics as "the tool tells you the next command" — which is exactly what you want an agent to lean on.

I did not get stuck anywhere that blocked me. I hit friction and one real contradiction, but validate caught the one thing that mattered before it could ship. Details ranked below.

---
Bugs / rough edges (ranked by impact)

1. owner-artifact presence gate contradicts its own composed prose. (Real bug — file this.)
The composed step says, verbatim: "finalize promotes that file in the same transaction and the presence gate asserts it is durably staged." Reading that, I wrote the artifact to completions/artifacts/<milestone>/… and left it untracked, expecting finalize to promote it. jigc task validate then returned it as blocking: "present but untracked — stage it so it is durably committed." So finalize does not git add the owner-artifact for you — you must stage it manually. "Promotes" is true for the managed docs (they show kind: promoted in the manifest) but the artifact lands as a plain kind: added. Either make finalize actually stage the recorded owner-artifact path (matching "promotes"), or change the step to say "write it and git add it." As written, anyone following the prose literally trips the gate.

2. jigc describe --workflow <id> is rejected. error: unexpected argument '--workflow'. To introspect a single workflow's composition I fell back to jigc describe --format json | grep. If per-workflow introspection isn't supported, the orientation output could say so; if it is, the flag name isn't discoverable from --help in the obvious place.

3. Checkpoints are prose, not enforced. triage-gate and fix-rounds-exhausted arrive as instruction text inside the composed step — the binary doesn't halt. The human-gate discipline is entirely on the agent to notice the words "The human owns this gate" and choose to stop. I halted at both, but a less careful agent would blow straight through two human gates and never know it did. Consider emitting checkpoints as a machine-readable field in the composed output (e.g. checkpoints: [{id, kind: human-gate}]) so a harness can enforce the pause instead of trusting prose comprehension.

---
Unclear (didn't block, but I had to infer)

- Meta-field addressing. The schema shows set-field: completion-record:<slug>#meta/verdict, but the composed step and my calls used the short #verdict — both resolved to section meta. Fine, but pick one canonical form in the docs so it's not ambiguous which is "right."
- What "the project's own gate" is. The re-verify step correctly says "run the project's format/lint/test/build gate, whatever toolchain it uses" — but jigc gives no pointer to which commands. I knew the test/lint tooling only from CLAUDE.md. Optional: let a project declare its gate command so the loop can name (or even run) it rather than the agent inferring it.
- --task read semantics. jigc doc show roadmap --task <id> failed with store.not-staged because the roadmap wasn't staged in this task. The error was excellent — it handed me the exact fix (jigc doc show roadmap:roadmap reads the committed copy). But the mental model ("--task reads the staged copy; drop it to read committed") isn't stated up front; I learned it from the error.

---
What worked well (keep this)

- validate before finalize is the hero. It previewed the blocking untracked-artifact as a non-committing dry run. That single feature turned bug #1 from "shipped broken" into "fixed in ten seconds." Don't ever lose this.
- Errors carry the next command. "resume with jigc start --task …", "the task-less read serves the committed copy", "stage it so it is durably committed." Most errors were self-healing.
- The finalize manifest (kind per file, promoted: [...], left_out: []) is a great final confirmation surface — I could see at a glance that exactly the right 10 files landed and nothing was silently dropped.
- create is idempotent for singletons. decisions-log returned existed: true, copied the committed doc in, appended my 2 entries, left the prior 14 untouched. Exactly right.
- The composed brief is complete enough to work from alone. For the process, I never needed to research jigc's behavior elsewhere.

---
Did I get everything from jigc, or research elsewhere?

- Workflow mechanics / doc structure / addressing: 100% from jigc. The composed step, jigc doc schema, and error routes were sufficient. No external research needed for how to drive the loop.
- Audit content came from the repo, as it should. Ground truth (test results, Dockerfile, source), and the gate commands, necessarily came from the codebase — jigc composes the process, not the findings, which is correct. Worth stating plainly in the workflow so agents don't expect jigc to supply the verdict inputs.
- One caveat you should know: I deliberately routed around a suspected doc author / set-slot corruption sharp-edge (used incremental set-field instead of batch author, and kept #### headings out of slot prose) based on prior-session knowledge, not any warning from jigc itself. So jigc didn't flag its own sharp edges — I avoided them because I already knew. Two implications: (a) I can't confirm whether that corruption bug still exists in this binary, because I never exercised the risky path; and (b) if those sharp edges are real, jigc should warn at the call site (e.g. reject #### in a slot with a helpful message rather than a write.non-reparseable error). An agent without that out-of-band memory would walk straight into them.

---
Suggested changes, condensed

1. Fix the owner-artifact contradiction — auto-stage on finalize, or tell the step to git add it. (Highest priority; it's a correctness trap.)
2. Make Checkpoints machine-readable, not prose-only, so the human gate can be enforced rather than hoped for.
3. Support jigc describe --workflow <id> (or document how to introspect one workflow).
4. Canonicalize meta-field addressing (#verdict vs #meta/verdict) in the docs.
5. Let projects declare their gate command so the re-verify step can reference it instead of the agent inferring the toolchain.
6. Warn at the call site for the known authoring sharp edges (slot headings, batch-author corruption) instead of relying on the author to already know.

Net: v1 is sound and pleasant to drive. The one thing that would have bitten a careless run is the owner-artifact prose/behavior mismatch — everything else is polish. The loop did the important thing correctly: it let me record an honest RED verdict with dispositioned findings and never once pressured me toward a false green.


## 5. The reference graph (grounded-in / cites / derived-from) - (Session: test-task-5)

I left every ref as plain prose this session — and validate even flagged the prd's missing derived-from. The whole edge/link dimension is untested.

- Task: author a spec for one deferred feature (e.g. "multiple domains per website") with derived-from → the prd; record an adr for a design decision and add cites → the relevant arch-doc; set the vision's grounded-in → a research doc.
- What it proves: cross-doc edges, inverse-cardinality gating, and whether refs survive/rewire when targets change. This is a completely separate correctness surface.

### Feedback

Task: wired three cross-doc edge types into 16 already-migrated docs — vision → grounded-in → research (multi-value), arch-doc → cites → adr, spec → derived-from → prd — plus created one ADR and one spec. 4 commits, tree clean, target advisory cleared.

Overall

Good to work with. The model is coherent once you have it: permissive at write, gated at the task boundary, structural writes owned by the tool. The single best feature is that every finding carries a route — the exact command to fix it. I never had to guess how to resolve a validation failure; I just ran what the finding told me. jigc doc schema <doctype> and the author batch grammar (--help) were enough to author correctly on the first try. I did not need to read managed doc files directly or research jigc mechanics elsewhere — the binary told me everything about itself. (I read content from jigc doc show and repo knowledge to decide what to record/spec, which is expected.)

Bugs (ranked)

1. plan workflow never surfaces derived-from. The composed steps cover create/goal/context/criteria but never mention setting the derived-from edge — the one edge that gives a PRD its inbound referrer. This is very likely why the whole graph stayed empty after migration: the workflow that authors specs doesn't guide you to wire the edge that the schema (and the standing inverse-cardinality advisory) is asking for. Same shape likely applies to form-vision's grounded-in in reverse: it does prompt it, which is why that one was easy. Every workflow that mints a doc with an edge field should include an explicit step to set it.
2. Wrong commit-field addresses in workflow text. Multiple workflows printed jigc doc set-field commit:<task>#type ... and #scope ..., but the real addresses are #header/type and #header/scope (the #type form errors). I only got the right ones from a validate finding's route. The generated instructions and the authoritative routes disagree — the instructions should print the section-qualified address.
3. form-vision re-compose doesn't render grounding findings. The workflow promises: "the findings of ALL grounding research appear here … each under its own > **type:slug** blockquote" after you set grounded-in and re-run jigc start --task. I set all four research targets (each with substantial findings content) and re-composed — the section rendered empty, byte-identical to the pre-set compose. The resolve/render step appears not to fire.
4. finalize --format json emits non-JSON after the JSON. The hook output (doc<->code drift detected…, lint-staged…) is appended after the closing }, so --format json isn't cleanly machine-parseable on finalize. Either route hook chatter to stderr or fold it into the JSON payload.

Friction / unclear

- Workflows assume create-fresh, not revise-existing. form-vision, record-decision, architecture-documentation all lead with jigc doc create …. But the vision and the arch-doc already existed (singletons / managed docs). I had to deviate: skip create, use set-field directly, which correctly copy-on-writes and preserves the prose. That worked well — but nothing in the workflow text tells you it's the right move; a first-timer would run create and either error or fear clobbering. Workflows that can target an existing doc should say "create or set-field on the existing one," and ideally the re-entry (jigc start --workflow X on a doc that exists) should adapt its steps.
- Ref semantics aren't stated up front. I had to probe to learn that a ref to a missing target is accepted silently at set-field (zero findings) and only blocks at validate/finalize (schema-conformance.ref-resolves). This is good, strict behavior — but the write-time silence means you don't discover a typo'd target id until finalize. Consider an optional soft warning at set-field time (advisory, non-blocking) when a ref doesn't resolve yet, so fat-fingered ids surface immediately instead of at the gate.
- Prior handover's mental model was wrong here — worth correcting in docs. The migrate templates apparently led the last session to believe refs "silently drop to prose" if the target is missing. They don't: they persist as structured refs and hard-block. Whatever text created that impression should be fixed, because it changes how people sequence "create target, then wire edge."

Change suggestions (concrete)

1. Add an explicit edge-setting step to every doc-minting workflow whose doctype has a ref field (esp. plan → derived-from).
2. Fix the commit-field addresses in generated workflow text to be section-qualified (#header/type, #header/scope).
3. Make workflows revise-aware: detect an existing target and emit set-field/author-updates-committed steps instead of a bare create.
4. Keep --format json output pure JSON on finalize (hook chatter to stderr).
5. Fix form-vision re-compose so grounded-in findings actually render.
6. Optionally: advisory-level "ref doesn't resolve yet" hint at write time.

What worked well (keep)

- Finding routes — the single biggest usability win.
- author batch verb — writing a whole ADR/spec (fields + slots + repeatable criteria) in one payload was clean and validated on first try.
- set-field copy-on-write preserving unrelated prose — editing one edge on a rich existing doc was safe.
- Multi-value refs in one set-field call ([a, b, c]) — worked exactly as documented.
- Task-boundary integrity gating (dangling ref blocks finalize) — strict in the right place.


## 6. spec → implement-from-spec (spec-driven build) - (Session: test-task-6)

- Task: plan a spec for a ledger item ("fleet overview dashboard"), then implement-from-spec against its acceptance criteria.
- What it proves: the spec doctype, locating an implementation against a committed spec, and the built-in record-changelog step (also untested — the changelog doctype).

### Feedback

Context: authored a spec via plan, built the feature via implement-from-spec (with an ADR + maps-to-test anchors), and recorded a changelog via record-change. Three clean commits, tree clean. Below is the honest read.

Overall

Working through jigc rather than around it felt good. The workflow prompts are the standout — each jigc start --workflow … output is self-contained: it hands you the exact next commands, the address grammar, and the gotchas inline. I rarely had to guess the procedure. The context-compiler idea holds up: I never touched a managed doc directly, and structural placement/commits "just worked." The plan↔implement separation is real and useful — plan produced a pure-docs commit, implement-from-spec bound that spec and read its criteria back verbatim.

The gates earned trust: validate previewing exactly what finalize will block on (without committing) is excellent, and I leaned on it before every finalize.

Bugs

1. --format json output is polluted by trailing hook text (finalize). jigc task finalize … --format json appended non-JSON after the JSON body — --- hook output ---, then jigc: doc<->code drift detected… and a lint-staged line. This broke json.load() on the finalize result twice. AGENT.md explicitly says "Every verb speaks --format json … parse the structured result — do not scrape the human-readable lines." A parser can't honor that if finalize interleaves hook stdout with the JSON. Either capture hook output into a JSON field or emit it to stderr. Severity: medium (the commit itself succeeded both times; only the machine-readable contract is violated).
2. Commit trailer accepted then silently dropped. I added a Co-Authored-By trailer (add-item commit:…#trailers → set /value), both calls returned zero findings, and validate was clean — but the trailer does not appear in the final commit message. Either the trailer wasn't wired into the commit template, or it needs a shape I wasn't told about. Accepted-then-dropped is the worst failure mode because nothing signals it. Severity: medium.

Friction / where the design fights a test-first repo

3. maps-to-test symbol resolution doesn't understand Pest — the biggest real-world snag. The anchor resolves PHP source symbols (parsed from the file), exactly like implemented-by. That means path#SomeClass/path#someFunc resolves, but Pest's per-test it(...) closures have no source symbol, so you cannot anchor a criterion to the specific test that proves it — even though CLAUDE.md mandates Pest 3. The implement-from-spec prompt tells you to use <path>#<test-fn>, which is actively misleading for Pest (there is no test-fn symbol). I only got it working by discovering empirically that a file-only anchor (path, no #symbol) is accepted and resolves — so all my criteria point at the test file, losing per-criterion precision.
  - Fix options: (a) document that file-only anchors are the supported granularity for closure-style test frameworks; (b) teach the resolver to recognize Pest test descriptions / test()/it() calls as anchorable; (c) at minimum, change the prompt's <path>#<test-fn> hint to acknowledge closure test files.
4. maps-to-test lifecycle is undocumented and had to be reverse-engineered. The full behavior — permissive at set-field (accepts anything, zero findings) but gated at validate/finalize against the staged index (not the working tree), resolving like implemented-by — appears nowhere I could find. The practical consequence is important and unstated: a spec cannot be committed with anchors to not-yet-written tests, so maps-to-test is inherently populated during implementation, not at plan time. That's a coherent design (and the implement prompt's "leave it off … rather than pointing it at a test you have not written" hints at it), but a one-liner in the spec schema help — "checked at finalize against the staged index; author it once the test exists" — would save the discovery.
5. deferral-ledger is append-only — no retire path. park-idea adds an entry, but nothing retires one once its idea ships, and the schema has no resolved/status field. After shipping the feature, its ledger entry is now stale (still listed as deferred). None of the 12 offered workflows fit "prune a shipped entry" — quick-fix explicitly touches no documented code. I flagged it rather than shoehorn it through a mismatched gate. A ledger needs a first-class "resolve/retire entry (with the commit/spec that closed it)" move, or an entry status.

Minor / unclear

6. set-field value must be --value, not positional. My first call passed the value positionally → error: unexpected argument (exit 2). Minor and my mistake, but note the inconsistency: add-item --title <x> reads almost positional while set-field requires the --value flag. Usage errors are pre-parse and did not corrupt anything — good.
7. Batch author payload shape for section-level slots is ambiguous. For a section that is itself a slot (spec #goal, #context), the help example only shows a section with a named sub-leaf (summary). I guessed set: { goal: |<<…>> } (leaf-id = section-id) and it worked — but a payload example for the "section is a slot" case would remove the guess.
8. Carried-over concern, not re-observed this session: the prior-session bug where a rejected author/set-field/set-slot corrupts the staged doc. I deliberately designed around it (batch author, payloads saved to files so re-author is one command), so I can't confirm whether v1 still has it — but the fact that my whole authoring strategy was shaped by fear of it is itself signal. If it's fixed, say so; if not, it's the highest-value thing to fix, because it makes incremental authoring feel unsafe.

Did I get everything from jigc, or research elsewhere?

- Project source (models, migrations, factories, DomainRule, phpunit/SQLite FK config) — read directly, as intended. No complaint; that's the documented freedom.
- jigc behavior — mostly self-served from --help, doc schema, and workflow prompts, which were sufficient for the happy path. The one real gap was code-anchor resolution semantics (maps-to-test/cites-code): what a symbol resolves against, that it's the staged index, that file-only is legal, that Pest closures don't resolve. None of that is in the surface I could reach — I got it only by probing validate against deliberate wrong anchors. That's the doc gap to close.

What worked well (keep it)

- Workflow prompts as executable runbooks — genuinely excellent.
- validate as a dry-run of finalize gates.
- Contextual gate-granting (implement-from-spec opened the adr gate exactly when a decision arose).
- Batch author — atomic, one write, and it happily set a meta ref (derived-from), section slots, and nested repeatable items together.
- Reference-graph payoff: setting spec → derived-from → prd cleared the PRD's standing inverse-cardinality advisory. The graph advisories feel earned, not noisy.
- record-change/changelog singleton — create-or-append worked first try.

Top 3 asks

1. Make --format json emit pure JSON (route hook output to stderr or a field) — it's a stated contract that finalize breaks.
2. Fix or explain the dropped commit trailer.
3. Document code-anchor resolution (staged-index, symbol-vs-file, closure-test frameworks) and fix the misleading <path>#<test-fn> hint for Pest.


## 7. Parallel execution + worktree isolation (concurrency machinery) - (Session: test-task-7)

- Task: milestone-execution fanning out 3 independent ledger items (ESLint peer deps + CI lint/format scripts + SQLite view-migration guard) to parallel sub-tasks joined at one commit.
- What it proves: the fan-out/join, isolation: worktree, and the single join-commit boundary — the most "different" concurrency surface, and a good stress test after I found that serial task state was fragile (the corruption bug).

### Feedback

Context: Drove one full jigc milestone fan-out end to end — decomposed Increment 1 (green the hermetic Pest suite) into 5 spec-seeded sub-tasks, ran them in parallel across isolated worktrees, joined and finalized to one commit, then ran a deliberate same-file collision probe. Deliverable landed: 406 passed / 0 failed, single commit 632e076.

Overall

Working with jigc for real concurrent work was good and, more importantly, trustworthy. The verb surface is coherent (create → add-from-spec/add-task → provision → execute → join → finalize → discard), the composed step-text is genuinely instruction-grade, and the safety behavior under conflict is the highlight: it fails safe and loud, never silently. The --format json contract with .task/.text/structured findings made it scriptable and unambiguous. I'd trust it to parallelize finalize — which is exactly the thing I didn't trust myself to do by hand during migration.

What worked well

- Isolation is real. Five detached worktrees at the base pin, each running against its own checkout. Verified concretely (a symlink shortcut cross-wired one worktree to main and it visibly blew up — proof the isolation boundary is load-bearing, not cosmetic).
- Single-commit join. finalize folded 8 files into one commit with a clean manifest + per-sub-task breakdown (code_files/docs). Worktrees auto-cleaned on finalize.
- Conflict handling fails safe. The code-collision probe hard-blocked at finalize with a precise message and a remediation route, committing nothing. No last-wins, no index contamination. This directly retires the migration-era fear that drove me to serialize every finalize.
- Good guardrails. discard refuses to nuke worktrees with uncommitted work (milestone.dirty-worktree); the sub-task compose explicitly warns "never git commit / never jigc task finalize here." Both saved me from foot-guns.
- Zero-diff members just work. Two no-op sub-tasks authored commit prose, joined, and finalized with code_files: 0 — no special-casing needed.
- Exit-code discipline held. Non-zero + actionable stderr every time it blocked; I never had to guess.

Bugs / rough edges

1. join does not detect code collisions — only finalize does. [contract gap, medium] My same-path divergent-file probe made join return overlay: {}, findings: [] (clean), then finalize hard-blocked on the exact same collision. join's own help says it "reports the merged outcome" and surfaces "a blocking finding (a same-doc clash…)" — but that's docs-only; the code index fold is invisible to it. A clean join is not a safe-to-finalize signal. Either join should dry-run the code fold and surface code collisions too, or its output/help should state plainly that it validates docs only.
2. Worktrees are provisioned without a runnable environment. [biggest practical gap, high] provision gives isolated tracked code, but vendor/, .env, and public/build are gitignored, so a fresh worktree cannot run the project's own gate (./vendor/bin/pest) as-is. The obvious workaround (symlink vendor) silently breaks isolation because composer resolves __DIR__ through the symlink back to main — I hit 129 bogus failures before switching to a hardlinked real vendor/. Any PHP/Node/etc. project will hit this. Options: a provision hook to materialize/hardlink non-VCS runtime deps, or at minimum document the trap and the safe pattern.
3. The documented entry point doesn't exist as documented. [onboarding trap, medium] jigc start --workflow milestone-execution "<intent>" mints nothing — it composes guidance text with an unresolved <MILESTONE_ID> and no Spawn: lines (because nothing is planned yet). It looks like it should work and quietly doesn't. The real path is the jigc milestone verb family; milestone-execution is reachable only via jigc milestone execute on an already-seeded milestone. Either reject that start invocation with "use jigc milestone …", or have it bootstrap the milestone.
4. Milestone bookkeeping commits land on the working branch before any real work. [surprising, low] create + each add-* made 6 chore(milestone): commits on release, moving HEAD out from under the just-provisioned worktrees. Harmless here, but it means the milestone flow mutates your branch history as a side effect of planning. Worth documenting; ideally these live somewhere less intrusive than the mainline branch.

Where I got stuck / had to work around

- Item #2 above was the only genuine stall — I lost a cycle diagnosing the 129-failure explosion before realizing the symlink cross-wired autoload. A one-line note in provision's output ("worktrees contain tracked files only; runtime deps are not populated") would have saved it entirely.
- git worktree list initially showed the main checkout at a different HEAD than git log had — which turned out to be the bookkeeping commits (#4), not corruption. Momentary confusion, resolved by reflog.

What was unclear

- The doc-merge vs. code-fold split isn't stated up front. join/finalize help talks entirely about "staged docs" and "suffix-resolved doc bodies." That the actual code travels via each worktree's staged git index (folded separately) I only learned by reading the sub-task compose text ("git add your code edits so the milestone can fold this worktree's staged index"). The two-channel model (docs overlay + code index) is central and should be explained once, plainly.
- What counts as "managed" wasn't always obvious at the point of use. I correctly read the spec via jigc doc show spec:…, but the roadmap sat right there as docs/roadmap.md and I read it directly with sed to get the increment breakdown. If roadmap is a managed doc, that was a rule slip on my part — but the friction is real: the human-readable file is present and inviting, and there's no in-path nudge ("this is managed; use jigc doc show"). A jigc doc list that's easy to consult before touching a file, or a guard, would help.

Did I get everything from jigc, or research elsewhere?

- From jigc: the entire workflow (compose text, spawn contract, doc/field/slot authoring), spec criteria (jigc doc show), the collision semantics (learned from the finalize error text), and all structural writes. The composed guidance was high quality — I rarely needed to guess intent.
- Outside jigc (legitimately — project source): the failing tests and the shipped code under them, phpunit.xml, migrations, controllers — all read freely, as intended. The sub-agents did their root-causing entirely in project source.
- Outside jigc (friction, arguably shouldn't have): docs/roadmap.md read directly rather than via jigc doc show (see above), and I leaned on the plain-text handover for the increment's cluster breakdown — which was stale: it claimed 27 failures / 9 clusters, but the binary showed 19 / effectively 3 with-work clusters, and its criterion-#1 root cause (MySQL view syntax on SQLite) was a misdiagnosis (real cause: a form request field was silently dropped). jigc's own committed spec carried the same wrong root cause. This is less a jigc bug than a reminder that add-from-spec faithfully seeds whatever the spec says, including its errors — there's no "does this criterion still reproduce?" check. A plan/add-from-spec that re-verifies criteria against the live gate before seeding would have caught two dead sub-tasks and one bad diagnosis.

Concrete suggestions (ranked)

1. Populate or hint runtime deps in provision (#2) — biggest real-world unblock.
2. Make join validate the code fold too, or relabel it docs-only (#1).
3. Reject/redirect jigc start --workflow milestone-execution (#3).
4. Document the two-channel (docs overlay + code index) model in join/finalize help.
5. Consider re-verifying spec criteria against the gate at seed time, so stale criteria don't mint no-op or misdiagnosed sub-tasks.

Net: v1 is genuinely usable for parallel milestone work and, crucially, safe under conflict. Fix the worktree-environment gap and the join/finalize detection asymmetry and this goes from "trust it after I double-check" to "trust it."


## 8. An adversarial "try to break it" session

Given the failed-write corruption I hit, worth a session aimed at the edges rather than real work:
- Provoke finalize.promote-clobber (drop a file at a doc's destination after creating the doc); exercise --carry-staged, a concurrent-write 409 + --force; jigc task bind + resume after a pause (resume is totally untested); and re-try the roadmap ####-heading case to confirm whether the corruption bug is fixed.
- What it proves: recovery paths, conflict handling, and whether the atomicity issues from this session are systemic.

### Agent output

jigc v1 adversarial probe — findings report

Scope: Direction #8 of the jigc stress test. Isolated on a throwaway branch (jigc-adversarial-probe, now deleted); release's committed content was never touched; each probe reset between runs. Pack dev/1.0.0-rc.8.

Headline: The severe "failed-write corruption" bug from the migration session is only partially fixed. The fix covers flat ##/###-slot doctypes but not the #### item-delimiter depth — which was the exact original repro. Plus one new global-blocker robustness bug and several smaller inconsistencies.

🔴 BUG A (severe, NOT fixed at #### depth) — silent write-through corruption in repeatable-item doctypes

In any repeatable-item doctype (roadmap milestones, decisions-log/deferral-ledger entries, completion-record), the item-slot delimiter renders at #### . But the write-time guard write.slot-heading-depth reserves only ##/### (as the composed guidance itself states: "headings must sit at #### depth or deeper — ##/### are schema-reserved").

Reproduction (roadmap proves slot):
printf '...\n\n#### Injected Level Four\n\n...' | jigc doc set-slot roadmap:roadmap#milestones/<id>/proves --from-file - --task <t>
→ {"chars":121,"findings":[]}   exit 0     ← ACCEPTED, no finding
jigc doc show roadmap:roadmap --task <t>
→ blocking · store.unparseable — `#### Injected Level Four` ... shadows the item-slot delimiter   ← doc now unreadable
Then in-place recovery is impossible — the write path must reparse the (now-broken) doc to locate the slot, so even a valid plain-prose payload is rejected:
printf 'plain recovery attempt' | jigc doc set-slot ...#proves ...
→ blocking · write.non-reparseable   (location: line 1, col 1)   exit 1
Only jigc task discard + re-mint recovers. This is the original severe bug, intact, at the depth that was its original repro target. The root cause is a writer/parser disagreement: the write guard doesn't know that in a multi-slot-item context #### is also reserved (it's the item delimiter). The guard should extend its reserved depth to #### for repeatable-item doctypes.

🟢 BUG B (the fix, and its gap) — flat ##-slot doctypes ARE now atomic

For flat doctypes (ADR, ##-level slots) the same class is caught at write time and rejected atomically:
printf 'X\n\n## Injected Heading\n...' | jigc doc set-slot adr:...#decision ...
→ blocking · write.slot-heading-depth — "heading at schema-reserved depth `##` in slot prose at line 3"
The staged doc was preserved intact, the task stayed usable, and a subsequent valid write succeeded with no discard needed. So the fix exists and works well — it just stops at ### and misses the #### item-delimiter depth (Bug A). The two are one incomplete fix, not two separate bugs.

🔴 BUG C (new, global blocker) — dangling file-state.json entries block every task and survive git reset --hard

At session start, file-state.json tracked docs/milestone-records/collision-probe-experiment.md as managed, but the file didn't exist, had no git history (never committed), and wasn't in jigc doc list. A prior probe session created it, its hash landed in state, then it was discarded/never-committed — leaving a dangling entry. Consequence:
jigc task validate <any-fresh-task>
→ blocking · reconciliation.rename — tracked managed doc milestone-record:collision-probe-experiment ... is missing   exit 3
It fires on every task, blocking all finalize. And because file-state.json is gitignored, git reset --hard 632e076 (the documented restore in this test series) does not clean it — the blocker is git-invisible and persistent. Remediation is jigc unmanage <path>, which I applied (see "state note" below). Suggestion: task discard/failed-finalize should roll back any file-state.json entries it wrote, and/or jigc validate should offer to prune entries for docs with no git history.

🟡 Smaller observations

- Misleading write diagnostic persists (write-verb layer only). The write.non-reparseable recovery rejection reports line 1, col 1 and routes "the message names the break" — but the real break is pre-existing corrupted state, not the payload's line 1. Notably, jigc task validate gets it right (conformance.item-slot-delimiter-shadowed, naming the actual #### Injected Level Four). The precise diagnostic exists; the write-verb path just doesn't use it.
- Exit-code taxonomy is inconsistent across verbs. Same-severity blocking findings map to different codes: rejected write-verb → 1; task validate/finalize gating → 3 (correct); whole-repo jigc validate on a "blocking (gates at finalize)" finding → 1. Per the AGENT.md contract (3 = blocking findings) the write-verb and whole-repo-validate cases are arguably off. (My first-pass "exit 0 everywhere" worry was a | head pipe artifact — retracted.)
- No re-sync recovery from finalize.base-mismatch. When an intervening commit touches a task's doc, finalize correctly refuses (exit 3, no clobber) — but the only route is discard (all staged work lost). There's no --force/rebase path.

✅ Things that work well (verified)

- Concurrent-write conflict detection — finalize.base-mismatch refuses cleanly; the first writer's milestone was not clobbered.
- finalize.promote-clobber — refused to overwrite an untracked "precious" file planted at a doc's destination; exit 3, excellent routing (retitle / --slug / migrate).
- Reference integrity — deleting a cited ADR (arch-doc --cites--> adr:introduce-a-service-layer) is caught as reconciliation.rename blocking-at-finalize (indirectly, via missing-doc detection rather than an explicit dangling-edge finding).
- Anchor-integrity checker works (doc-code.title-names-symbol).

### Feedback

Context: I drove jigc as the sole interface across an 8-direction stress test (happy-path exercise of every major surface, then a dedicated adversarial/recovery pass). This is my experience working with it as an autonomous agent, plus a v1-readiness call at the end.

Overall: how it was to work with

The core model — jigc as a context compiler — is genuinely good and I'd want to keep working this way. jigc start composing exactly the workflow steps + the doc slices that workflow needs, and owning every structural write, meant I almost never had to think about where a doc lives or how to cross-reference it. I authored prose; jigc did placement, slugging, edges, and commits. That division of labor is the right one for an agent.

Specific things that worked well:
- The composed workflow prose is excellent for an agent. The jigc start output hands you the exact verbs, the batch alternative, and the finalize semantics inline. I rarely had to guess the next command. Verbose for a human, but for an agent explicit-beats-clever every time.
- --format json with a stable .task field made the whole thing scriptable and deterministic. I never had to scrape human lines.
- Error messages carry a route: field with concrete remediation commands. This is the single best design choice — most failures were self-recovering because the error told me exactly how to proceed (promote-clobber, base-mismatch, unmanage, etc.).
- Identity ergonomics — slug-from-title, add-item minting ids, the atomic rename refactor, the create-or-update singletons — are clean and predictable.
- Guardrails that held under attack: concurrent-write finalize.base-mismatch (no silent clobber), finalize.promote-clobber (protected an untracked file at a destination), reference-integrity on deleting a cited doc, and the anchor-integrity checker. These all did the right thing with the right exit code.

Did I get all information from jigc, or research elsewhere?

Entirely from jigc for behavior — jigc --help, jigc describe, jigc doc schema <type>, and the composed start prose were sufficient. I never needed external docs or a jigc source tree to understand how to use it. That's a strong signal for a self-describing CLI.

One gap: to diagnose problems I had to read jigc's internal state directly — .jigc/index/edges.json and .jigc/state/file-state.json. There's no CLI introspection for "what does jigc currently track and baseline?" When something drifts, the only window into the tracked-hash set is the raw gitignored JSON. A read-only jigc state list / jigc doc list --with-baseline would have saved me from poking at internals.

Bugs I hit

1. 🔴 Write-through corruption in repeatable-item doctypes (the big one). Authoring a slot whose prose contains a ####  heading, in any repeatable-item doctype (roadmap milestones, decisions-log/deferral-ledger entries, completion-record), is silently accepted by the write guard ({"findings":[]}, exit 0) but then makes the whole doc store.unparseable. In-place recovery is impossible — even a valid plain-prose payload is then rejected — so the only escape is task discard + re-mint, losing all staged work. Root cause: the write-time guard write.slot-heading-depth reserves only ##/###, but in a multi-slot-item context #### is also reserved (it's the item delimiter). Writer and parser disagree on the reserved depth. Note this was the original repro depth: the fix that landed for flat ##-slot doctypes (which now reject atomically — nicely done) never extended to ####.
2. 🔴 Dangling file-state.json entries block every task and survive git reset --hard. A doc created-then-never-committed by a prior session left a hash entry in file-state.json with no file and no git history. It surfaced as a reconciliation.rename blocking finding on every task (including fresh unrelated ones), blocking all finalize. Because file-state.json is gitignored, git reset --hard doesn't clean it — the blocker is git-invisible and persistent. jigc unmanage fixes it, but you have to know it's there first. Suggestion: a failed/discarded task should roll back any file-state.json writes it made, and/or jigc validate should offer to prune entries for docs with no git history.
3. 🟡 Misleading write.non-reparseable diagnostic. On the failed recovery write it reports line 1, col 1 and routes "the message names the break" — but the real break is pre-existing corrupted state, not the payload's line 1. Interestingly, jigc task validate gets this exactly right (conformance.item-slot-delimiter-shadowed, naming the actual offending line). The precise diagnostic already exists; the write-verb path just doesn't use it.

Where I got stuck

- The corruption in bug #1 was a genuine dead-end: once a ####-slot was corrupted there was no in-place path forward — I had to discard and re-mint. For an agent mid-task that's expensive and non-obvious (the write reported success, so nothing signaled that discard was the only option).
- Bug #2 blocked me before I'd done anything wrong — the first fresh task I validated was already gated by a prior session's residue. Without dropping into the internal JSON I couldn't have explained why an empty task was blocked.

Things that were unclear / worth changing

- Exit-code taxonomy is inconsistent across verbs. Same-severity blocking findings map to different codes: rejected write-verb → 1; task validate/finalize gating → 3; whole-repo jigc validate on a "blocking (gates at finalize)" finding → 1. The AGENT.md contract says 3 = blocking findings, so the write-verb and whole-repo-validate cases read as off. Pin one rule: does exit code reflect finding severity or operation success? Right now it's a mix, and an agent keying recovery off exit codes will mis-handle it. (Caveat: watch for | head/| tail swallowing the real exit code — that bit me once and is a documentation-worthy footgun, not a jigc bug.)
- finalize.base-mismatch has no re-sync path. When an intervening commit touches a task's doc, finalize correctly refuses — but the only route is discard, losing all staged work. A --rebase/re-sync that replays the staged slots onto the new baseline would turn a total loss into a merge.
- Checkpoints aren't enforced by finalize. The planning workflow's Settle checkpoint is described in the composed prose, but finalize proceeded straight past it programmatically. If checkpoints are human-only gates that's fine — but it wasn't clear whether an agent is allowed to finalize past an unmet checkpoint, or whether that should be refused.
- The reserved-heading-depth rule should be stated per-doctype. The guidance says "##/### are schema-reserved, use ####+" — but in repeatable-item doctypes #### is also reserved. The rule an author is given doesn't match the rule the parser enforces, which is exactly what produced bug #1.

v1 readiness call

The design is v1-quality — the context-compiler model, the write-verb surface, the route:-bearing errors, and the guardrails that held under adversarial load are all things I'd ship. What I would not call v1 yet is the failed-write recovery story: bug #1 (silent-accept → unparseable → unrecoverable-without-discard) and bug #2 (git-invisible global blocker) are both data-loss-adjacent and both hit me without any misuse on my part. Fixing #1 (extend the write guard's reserved depth to #### in repeatable-item contexts) is a small, high-leverage change that also collapses the whole misleading-diagnostic/forced-discard chain that follows from it. I'd gate the v1 call on those two.

Net: I liked working with it, I'd choose it again, and it's close. Close the write-recovery hole and the state-drift blocker, and it's there.


## Prompt to get feedback

this was a stress test for jigc and it's v1 call. give me feedback to jigc. how did you like to work with it? did you encounter any bugs? did get stuck somewhere? was something unclear? should we change something? did you get all information from jigc or did you research somewhere else?
this should be a copy past output i can copy over into the project (no file writing)
