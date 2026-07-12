# RC adoption trial — implementation half on rc.5 — project-alpha-2.0 (2026-07-11/12)

**Status: RC input, implementation half — the owed second half of the adoption trial** ([DECISIONS.md](../../../DECISIONS.md) → 2026-07-11 rerun + rc.5 charter re-sequenced it onto rc.5). Four instrumented probe sessions drove the installed **1.0.0-rc.5** through the migrated corpus's real work: a corpus schema upgrade, a doc-anchored refactor, an implement-from-spec run, and a full brownfield milestone. This record archives the probe designs, the executing agents' outputs, and their verbatim feedback + bug reports; the adversarial verification of every claim against the rc.5 code lives in [findings-verification.md](findings-verification.md); the invocation-log analysis is below.

## Provenance

- **Repo:** `~/ideas/project-alpha-2.0` — the corpus the migration-half rerun produced (41 managed docs, 11 specs / 153 criteria, store clean on rc.4 at start).
- **Binary:** jigc **1.0.0-rc.5** (installed release build) for all four probes; probe 1 *is* the rc.4→rc.5 store upgrade.
- **Method:** an **observer agent designed each probe** (a "Part A" prompt engineered to be naturalistic — no jigc hints, no mention of the surface under test — plus "Part B" observer notes stating the hypothesis and ranked outcomes), the human pasted Part A into a **fresh executing session**, and the executing agent worked blind. After each run the executing agent was asked for feedback and a bug report. This is the trial staying naturalistic per the 2026-07-11 decision (no instructed use — the log is the instrument).
- **Invocation log:** ON throughout, appending to the live `.jigc/logs/invocations.jsonl` (the slice after record 1154 is this trial; the earlier records are the migration half, analyzed in [../rerun-rc4/trial-record.md](../rerun-rc4/trial-record.md)).
- **The four probes:** (1) **migrate the corpus rc.4→rc.5** — the first-ever real run of `jigc migrate-corpus` on a committed corpus, with a designed trap: does the store-version advisory's own route lead to silent corpus corruption? (2) **a warranted refactor that breaks five code anchors** across four docs the task never touches — the first unprompted brownfield test of the headline anchor-gate claim, including two ADR `cites-code` fields; (3) **implement a committed spec** — the spec→code loop: routing, criteria injection, `maps-to-test`, spec closure; (4) **plan and run a whole brownfield milestone (v1.2 "make it safe")** — the entire milestone surface (planning/Settle/detect-gaps, execution, record, completion), with a designed one-way-door trap (an unresolved VISION.md open question the milestone depends on).

**Headline outcomes, one line each** (detail + evidence in the per-probe sections):

- **P1:** the transform itself was byte-stable, idempotent, auditable, and self-evidencing — but the store-version advisory's own route (`re-run jigc setup`) produces a **false green over an unmigrated corpus**, and only the agent's distrust of the route avoided it.
- **P2:** the anchor gate **blocked the commit and named all five orphaned anchors** — "That single behaviour changed the outcome of this session." The inverse gap surfaced with equal clarity: seven docs whose *prose* the commits falsified, invisible to a clean validate.
- **P3:** criteria injection after `task bind spec` was "the single best thing"; `maps-to-test` went unwritten again (now 0 writes across all trials), and a second implemented spec now asserts fixed bugs in present tense while validate stays clean.
- **P4:** planning's **Settle checkpoint caught the designed one-way door *and* a VISION.md self-contradiction the human had never noticed**, and detect-gaps caught the blocked dependency and built it first — jigc's strongest moments in the whole exercise. The milestone *unit* mis-fit the sequential brownfield spine (worktree fan-out assumes independent sub-tasks), discarding left a stale committed milestone-record, and **completion was skipped entirely** — the milestone ended with no verdict and a red suite, unnoticed.

**The human observer's own note (adherence data):** the executing agents sometimes read managed files directly rather than through `jigc` — especially in probe 4 (the milestone), where the natural first move on brownfield orientation is reading the roadmap/specs with plain file reads. This corroborates the probe-4 bug report's #14 (the read-rule fights orientation) and is standing input for the [managed-doc-enforcement-hook](../../../ideas/managed-doc-enforcement-hook.md) trigger ("the RC-trial invocation-log adherence data decides it") and the read-surface ergonomics cluster (`doc list`, singleton short form, [doc-search](../../../ideas/doc-search.md)) — the read path must be the path of least resistance before the rule can bind on reads.

## Probe 1 — migrate the corpus rc.4→rc.5 (`jigc migrate-corpus`)

**Design:** the prompt said only "jigc validate is flagging the store was last written by rc.4; get the managed docs onto rc.5 cleanly; no document changes meaning; stop rather than work around." The trap: rc.5 renamed the `deferral-ledger.kind` enum `D`/`I` → `Decision`/`Idea` (41 committed entries carry the old members), and the `store-version.binary-mismatch` advisory routes to "align the running jigc to 1.0.0-rc.4, or re-run `jigc setup` to re-stamp" — **without mentioning `migrate-corpus`**. Following the route literally re-stamps the store over an unmigrated corpus and validate goes green.

**Outcome:** the agent distrusted the advisory, found `migrate-corpus` via `--help`, and the migration was exemplary — 42 lines changed, 42 accounted for (1 schema-version stamp + 35 `D`→`Decision` + 6 `I`→`Idea`), zero prose touched, re-run a clean no-op, nothing blocked. It verified the letter→word remap against the schema and remap rationale embedded in the binary rather than taking it on faith. But it confirmed the trap is live: at its own intermediate commit, the store claimed rc.5, the ledger was still schema-v1, and `jigc validate` printed clean.

### Feedback (verbatim)

> The headline bug: the advisory's own route produces a false green
>
> The rc.4→rc.5 advisory offered exactly two routes: downgrade the binary, or "re-run jigc setup to re-stamp the store at 1.0.0-rc.5." Neither mentions migrate-corpus. If I had done the literal thing the tool told me to do, here's where I'd have landed — and this is a real commit from this session, not a hypothetical:
>
> At 6e387b2, .jigc/version says 1.0.0-rc.5, while the committed docs/deferral-ledger.md still says schema-version: 1 with 35 old-style kind: D entries. And at that exact moment, jigc validate printed "no findings — the committed store validates clean."
>
> So: the stamp claims rc.5, the corpus is still on the rc.4 schema, and validate is green. setup re-stamping the store silenced the only signal that anything was wrong, without migrating anything. The version stamp became a lie, and the one check that could have caught it had been switched off by the fix. There's a bitter irony in this landing in a repo whose CLAUDE.md opens by warning "do not trust a green badge."
>
> Two things need to change independently, because either alone still leaves a hole:
>
> 1. validate must probe corpus schema, not just the binary stamp. migrate-corpus --help explicitly promises "Detect with jigc validate; this migrates" — but validate does not detect it. That promise is currently false. A doc on a stale doctype schema should be its own finding ("1 doc on schema-version 1, current is 2 → run jigc migrate-corpus"), and it must be blocking, and it must not be silenceable by re-stamping.
> 2. setup should refuse to re-stamp over an unmigrated corpus. The ordering — migrate-corpus then setup — is load-bearing and written down nowhere. Enforce it rather than documenting it: if committed docs are on an old doctype schema, setup should stop and route to migrate-corpus.
>
> The only reason I didn't fall in the hole is that I distrusted the advisory and went reading jigc --help for other commands before touching anything. That shouldn't be what saves you.
>
> Second: migrate-corpus is a write with no way to look first
>
> There's no --dry-run or --check. The only way to answer "does my corpus need migrating?" is to migrate it. I ran it as reconnaissance and it mutated the working tree — which is how I found out the answer. That's backwards for what is framed as the detect-and-fix pair to validate. It worked out because the transform is genuinely clean and git makes it reversible, but on a less trustworthy transform I'd have been committed to an outcome I hadn't seen yet. --check exiting non-zero when work is pending would fix this and would also give CI something to call.
>
> Third: a small contract gap at the commit boundary
>
> AGENT.md is emphatic: "never read or edit managed docs directly; write every change back through jigc." But migrate-corpus wrote the doc and then just... left it dirty. It minted no task and made no commit, so the only way to land it was raw git add / git commit — the exact thing the contract forbids. Meanwhile setup, two commands later, cheerfully makes its own commit. So the write path is jigc's, the commit path is mine, and nothing says so. Worth deciding which side of the line this lives on.
>
> What was genuinely good
>
> Not a consolation list — these are the things that made the migration auditable, and they're the reason I could meet the user's "no document changes meaning" bar with actual evidence instead of a shrug:
>
> - The migrated / already_current / blocked triage, in --format json. Exactly the right shape. I could see at a glance that 40 docs were untouched and one moved, and that nothing was blocked. blocked being a first-class output — rather than a crash or a silent skip — is the right design.
> - The binary embeds its own schema and the remap rationale. I did not have to guess whether D meant Decision, Debt, or Deferral in a deferral ledger — a genuinely ambiguous letter. Grepping the binary turned up the kind enum and an explicit "Decision/Idea enum-member remap" string, plus the old schema's own comment glossing the field. That let me verify the mapping instead of trusting it. More tools should ship the evidence for their own transforms like this.
> - The transform was byte-stable and it reconciled exactly. 42 lines changed, 42 accounted for, zero prose touched. I could prove that mechanically.
> - setup didn't sweep my dirty tree into its commit. It committed .jigc/version alone, leaving my uncommitted doc change intact for review. It would have been very easy to write that as git add -A and it wasn't. Someone was careful there.
> - Idempotent. Re-running migrate-corpus was a clean no-op.
>
> Was it easy?
>
> Mechanically, yes — three commands, and the hard part (the transform) was correct and provable. But it was only safe because I ignored the advice the tool gave me. The failure mode here isn't "the migration is hard," it's "the migration is easy and quietly incomplete, and the tool tells you it's fine." For a stress test, that's the finding I'd act on first.

## Probe 2 — the doc-anchored refactor (the anchor-gate test)

**Design:** a genuinely warranted refactor (collapse two HTTP-client Service classes into one `app/Clients/Audit` client, following an existing repo convention) that deletes two classes cited by **five `cites-code`/`implemented-by` anchors across four managed docs** — two of them ADR `cites-code` fields — with zero mention of docs, drift, or jigc in the prompt. The crux: does finalize block a commit that orphans anchors in documents the task never touched?

**Outcome:** it blocked. The agent updated seven managed docs across the two commits — the five anchors *plus* prose the commits falsified — and deliberately left the append-only decisions-log untouched ("rewriting an append-only dated log would falsify history"). It found the two governing committed specs late, by accident, while grepping for orphaned references.

### Feedback (verbatim)

> What earned its keep
>
> The code-anchor gate is the whole product, on a brownfield repo. Deleting two Service classes silently orphaned five cites-code/implemented-by anchors across two ADRs and two architecture docs. I would not have found those on my own — I'd have shipped a clean-looking commit that left four managed docs pointing at files that no longer exist. jigc validate blocked the commit and named them. That single behaviour changed the outcome of this session, and it's the thing I'd protect above everything else.
>
> The write verbs are well-designed. retitle-item freezing the {#id} anchor so cross-references survive a heading rename is exactly the right call, and it's the kind of thing a human doing this by hand gets wrong. The commit doctype forcing type/scope/summary/body produced better commit messages than I'd have free-handed.
>
> The gap that worries me most
>
> A green validate does not mean the docs are true. jigc checks that anchors resolve. It does not — cannot, as built — notice that a paragraph now lies. When I deleted AuditService, the anchors were the small problem. The real problem was four documents whose prose said things like "services.audit does not exist in config/services.php, so the key is phantom" — a sentence my commit had just falsified. And when I revived the gate in commit 2, I falsified another three documents that said "the gate is dead."
>
> I found all seven by hand-grepping for phrases like "gate is dead" and "AuditService". Nothing in jigc pointed at them. That's a false-safety failure mode: the tool says clean, the corpus is lying.
>
> The cheap fix is a symbol-mention sweep. When a task removes or renames a file or symbol, grep the managed corpus for the old name and emit advisory findings: "6 managed docs mention AuditService in prose — review." That's a few lines of work and it would have surfaced the entire blast radius in one command instead of three rounds of me guessing grep patterns. It doesn't need to understand the prose; it just needs to point.
>
> The highest-value missing feature: spec awareness
>
> docs/specs/config-cleanup.md already specified this exact task. It even documented the trap — that naively pointing the old /audit-carrying const at a base-URL config key yields http://audit-svc:3000/audit/accessibility — and its acceptance criterion was nearly verbatim the test I ended up writing. And docs/specs/repair-the-audit-quality-gates.md owned commit 2 in full.
>
> I found both by accident, late, while grepping for orphaned references. If I hadn't, I'd have re-derived a committed spec from scratch and had a decent chance of walking into the double-path trap it warned about.
>
> jigc has implement-from-spec and a bind-spec command. But jigc start routed my intent across ten workflows without once checking whether any of eleven committed specs already covered it. On a brownfield repo, that's the question that matters most. The router should match intent against the committed spec corpus and say "this looks like spec X — bind it?" before offering dev-task.
>
> [...papercuts consolidated into the bug report below...]
>
> The thing worth deciding on purpose
>
> This repo's docs describe bugs in the present tense, which is why they're trustworthy — and it means every bug fix is also a documentation project. Fixing one nested-array bug obliged me to rewrite seven managed documents. That's not jigc's fault; it's the cost of the honesty. But it's a steep cost, it creates real pressure to skip the doc pass, and jigc's anchor gate only catches the mechanical half of it.
>
> I'd name that explicitly rather than let it be discovered per-task. If the answer is "yes, that's the deal," then the symbol-mention sweep above stops being a nice-to-have and becomes the thing that makes the deal payable.
>
> If you change three things
>
> 1. Spec matching in the router. Highest value on brownfield, by a distance. I nearly rebuilt a committed spec from memory.
> 2. Symbol-mention sweep on removal/rename. Turns "green validate, lying docs" into a listed set of advisory findings.
> 3. Fix the read surface — item IDs in JSON, consistent section shape, validate emitting addressable addresses. Right now, discovering where to write is harder than knowing what to write, which is backwards.
>
> For what it's worth: I'd want the anchor gate on any repo where docs are load-bearing. The friction is in the ergonomics, not the idea.

### Bug report (verbatim)

> Four confirmed bugs, and one correction — something I called a bug in my last message turns out to have been my own misuse.
>
> 1. validate emits an address that no verb accepts (and read/write disagree)
>
> For a top-level ADR field, there are three address forms in play and they don't line up:
>
> ```
> validate says:   adr:freeze-two-monorepo-contracts-in#status/cites-code
> doc show   →     blocking · store.no-such-item — names no item `cites-code` in section `status`
> doc show   →     adr:freeze-two-monorepo-contracts-in#cites-code
>                  blocking · store.no-such-section — names no section `cites-code`
> set-field  →     adr:freeze-two-monorepo-contracts-in#cites-code     ✅ works
> ```
>
> So: validate's own address works nowhere, and doc show cannot read a top-level field via either form — even though whole-doc --format json happily exposes a fields object. Only set-field accepts the #<field> form. [...] Worth noting the defect is specific to top-level fields. For arch-doc items, validate's address (arch-doc:<slug>#components/<item>/implemented-by) worked verbatim with both doc show and set-field. So the item path is right and the field path is wrong.
>
> 2. Item {#id} is missing from the JSON read surface
>
> Every repeatable item omits its {#id} — the one thing you need to address it. doc show --format json is documented as "the pinned stable shape … a repeatable section to its item array," but the item objects don't carry their identity. Consequence: you cannot discover an item's address programmatically. [...] Of everything here, this is the one I'd fix first. It's a small omission that makes the whole read surface unusable for automation.
>
> 3. task validate calls an existing file "absent from the working tree" when it's untracked
>
> After creating app/Clients/Audit/AuditClient.php — on disk, not yet git added — the anchor probe said: blocking · doc-code.symbol-exists — anchor […] resolves to no file (…is absent from the working tree) route: update the citation to match the renamed/moved code, or restore the cited symbol (e.g. revert the change). git add fixed it. The probe consults the git index, but the message says "working tree" — which in git terminology includes untracked files, so the message asserts something false. Worse, the suggested route is actively wrong advice here: the citation was correct and the symbol existed; nothing needed reverting.
>
> 4. jigc describe renders as one unbroken run-on paragraph
>
> Every workflow description concatenated with no separators, then every doctype, then every command — hundreds of lines of prose collapsed into about three paragraphs. It's unreadable and expensive in context. This looks like a missing join/newline in the --format agent renderer rather than a deliberate format.
>
> Correction: one thing I reported last message was not a bug — [the "inconsistent JSON shape across doctypes" claim was the agent piping stderr into its JSON parser; bare `roadmap` is a malformed address and jigc correctly errors on stderr].
>
> Not bugs, but warts (for completeness)
>
> - Singleton addressing requires roadmap:roadmap / decisions-log:decisions-log; the bare name is rejected. The error message is clear, but it's undiscoverable up front.
> - doc show is committed-only — no --task, so you can't read back prose you just staged. It routes you to task diff, which works.
> - jigc validate prints blocking findings and exits 0 ("report-only at store scope"). Documented and correct, but "blocking + exit 0" reads oddly at a glance, and I ran store-scope validate before task validate and briefly thought my anchor repointing hadn't taken.
>
> One thing that was not a bug and I want to credit: jigc doc schema decision-log (my typo for decisions-log) failed with unknown doctype and routed me to jigc describe. That's exactly right.

## Probe 3 — implement-from-spec (the spec→code loop)

**Design:** "implement docs/specs/correctness-monitoring-and-health-score.md … work from it directly … each acceptance criterion backed by something that actually runs." Engineered so the spec-driven path is the obvious one. Watch: does anything prompt `maps-to-test`; is there any way to close a spec; does the agent report honestly against the criteria.

**Outcome:** `implement-from-spec` was used (naming the spec routed it — the workflow works but is invisible unless pointed at a spec; probe 2's session had implemented a spec through `dev-task`/`quick-fix` without ever finding it). All 13 criteria implemented and tested (370→421 passing, pre-existing failures byte-identical), an honest criteria-by-criteria standing report delivered. But `maps-to-test`: zero writes — again. And the spec now asserts its bugs in present tense, fixed; validate clean.

### Feedback (verbatim)

> What worked
>
> The single best thing is binding a spec and having its criteria injected into the composed output. After jigc task bind spec, I was working from the committed criteria as text in front of me rather than from my memory of a file I'd read 40 tool calls earlier. On a 13-criterion spec that's the difference between drifting and not. I never once had to re-open the spec file.
>
> The second is owning the commit structure. Five slots (type/scope/summary/body/trailers) and I only authored prose. The resulting commit message is better than what I'd have hand-rolled into git commit -m, and I didn't have to think about staging conventions or trailer formatting. "You author prose, jigc owns structure" is the right split and it held.
>
> The gap that actually cost me: brownfield ground truth
>
> jigc told me what the spec wanted. It told me nothing about what the repo currently is. Three facts changed my plan materially, and I had to discover all three myself, by hand:
>
> - 33 tests already fail at HEAD. I snapshotted them to a scratch file and comm'd against them at the end, because otherwise "did I break something?" is unanswerable. That's me hand-rolling a baseline that the tool should own.
> - The suite runs on SQLite :memory:, so the MySQL-only WP view […] is unqueryable in tests. That's why the criterion I was asked to satisfy had no coverage, and it forced a real design decision.
> - ESLint is broken at HEAD — which then blocked jigc task finalize on a hook failure that had nothing to do with my work.
>
> Every one of those is a "the ground is not what the docs say" fact, and this project's own CLAUDE.md literally opens with "Do not trust a green badge... Check the code." jigc could institutionalize that: a baseline capture step at jigc start on a brownfield task — snapshot test/lint/build state before any edit, so at finalize it can tell me "you fixed 51, broke 0, and these 33 were already red." Right now that's artisanal work I did with comm and a scratchpad. It's the highest-value thing you could add.
>
> The gap that surprised me: open decisions vanished
>
> This spec had an explicit "Open decisions for the builder" section — two calls it said were "a product call, not a code call" […] Both were load-bearing — they changed the diff materially. I resolved them by asking the user. Those resolutions now exist only in my commit body. They never reached docs/decisions-log.md. jigc offered me jigc doc create adr as an option I could take if I felt like it, but nothing connected "this spec ships with unresolved decisions" to "record how you resolved them." That seems like exactly what the decision log is for, and the workflow walked right past it. If specs can carry a machine-readable open-decisions block, jigc could gate finalize on them.
>
> Same soft-gate concern for the changelog: this was an unambiguously user-facing fix bundle, the workflow said to record it "when the workflow opens a changelog create-gate," and finalize let me commit without one. If it's optional, I'll skip it; if it matters, gate it.
>
> Concrete papercuts
>
> 1. The task slug. My intent string became implement-docsspecscorrectness-monitoring-and — a slugified file path, unreadable, and I pasted it into eight subsequent commands. Once a spec is bound, derive the slug from it (correctness-monitoring-and-health-score). And make --task optional when exactly one task is active.
> 2. Satisfied gates keep talking. After I bound the spec, re-running jigc start --task ... still printed the whole "Pick the spec this work implements…" block, with the criteria buried below it. It reads like the bind didn't take. Composed output should drop gates that are already satisfied.
> 3. Everything-at-once. The first start dumped implement + commit fields + optional scope/body + trailers + changelog + ADR + verify + finalize, all before I'd written a line of code. Stage it: tell me what to do now; surface finalize/ADR guidance when I'm near finalize.
> 4. The trailer dance. add-item → read the address it prints → set-field <addr>/value is a round-trip that exists purely to learn an ID. Commit authoring was five commands; it could be one.
> 5. Finalize inherits hooks with no escape hatch. When husky rejected the commit, I got a wall of raw ESLint/husky stderr with jigc's framing lost, and my recourse was HUSKY=0 jigc task finalize — env-var trickery around a tool that's supposed to own the commit. Give it jigc task finalize --no-verify, and on hook failure say so as jigc: "the pre-commit hook rejected this; your staged work is intact."
>
> Net
>
> The context-compiler thesis is right, and the criteria injection proves it. Where it's incomplete is that it compiles context about the intent and not about the terrain — which is fine greenfield and is exactly the wrong half on brownfield, where the terrain is the thing lying to you. Add a baseline, and close the loop on open decisions, and I'd want this on by default.

### Bug report (verbatim)

> Confirmed bugs
>
> 1. The bind gate isn't re-evaluated after you bind — and the output contradicts itself. After jigc task bind spec succeeded, re-running jigc start --task … still printed the full pick-a-spec block […] and the caption "The spec's criteria — empty until you bind a spec and re-compose:" and then immediately prints all thirteen fully-populated criteria directly beneath. The caption and the instruction block are static text, not conditioned on bind state. It reads like the bind silently failed, which is exactly the wrong signal.
>
> 2. jigc task bind spec prints nothing on success. Literally zero output, exit 0. This violates jigc's own stated contract — "Read every command's output; a non-zero exit means stop" — and it's inconsistent with its siblings, which all confirm: set-field prints set commit:…#type = fix, set-slot prints set slot … (43 chars), add-item prints the new address. Bind is the one write that leaves you guessing. I only learned it worked by re-running start — and that output (bug 1) then told me it hadn't.
>
> 3. Task-slug generation mangles the intent string. implement docs/specs/correctness-monitoring-and-health-score.md became implement-docsspecscorrectness-monitoring-and. Two distinct defects: / and . are stripped without inserting a separator, fusing docs + specs + correctness into docsspecscorrectness; and it truncates mid-phrase (…-and, chopping and-health-score).
>
> 4. The "gates" instruction points at output that doesn't exist. The workflow output says: "record it on the changelog, when the workflow you are running opens a changelog create-gate (bare jigc start lists the gates it grants)." I just ran bare jigc start. It lists the pack, the workflows, and three Run: hints. It lists no gates at all. So the one instruction for discovering whether a gate is open is a dead end.
>
> 5. finalize swallows its own framing on hook failure, and has no escape hatch. When the pre-commit hook rejected the commit, jigc task finalize exited with a wall of raw husky/ESLint/Pint stderr and not one word from jigc about what happened to the task: did it commit partially, was my staged work intact, was the task marked done? […] It needs a first-class jigc task finalize --no-verify.
>
> Related but not jigc's bugs: [the repo's pre-commit hook is broken at HEAD (missing typescript-eslint dep); lint-staged's Pint call reports a false "not readable" error — both verified as the trial repo's own defects, hit *through* jigc.]
>
> Ranking: if you fix one: #1, because it actively tells you your bind failed when it succeeded, and the criteria — the single most valuable thing jigc gave me this whole session — appear under a caption asserting they're empty. #2 compounds it, since bind's silence is what sends you back to start to check in the first place.

## Probe 4 — drive a brownfield milestone (v1.2 "make it safe")

**Design:** "Plan v1.2 and run it. The specs in docs/specs/ already exist for most of it — use them." Tests the entire milestone surface: planning (scope → detect-gaps → Settle → review → decompose), execution, milestone-record, completion. Two designed traps: (a) one increment forks on an unresolved question **already recorded in VISION.md as an open question and flagged as a one-way door** (internal-team-only forever vs per-customer scoping) — does Settle surface it, or does the agent silently walk through the door? (b) one increment is blocked on a write path that doesn't exist (every remote write 404s) — does detect-gaps catch it?

**Outcome:** both traps were caught — see the observer's final report below. Six increments of real code landed (backup/rollback proven against a live CMS container, security fixes landed with tests), with an honest three-caveat self-assessment. But the milestone *unit* had to be abandoned mid-flight (worktree fan-out vs a sequential increment spine), the discarded sub-tasks left a stale committed milestone-record claiming 6 active sub-tasks, and **the milestone close never ran** — no completion-record, no verdict, suite still red (25 pre-existing failures), nothing gated or noticed.

### Feedback (verbatim)

> Overall jigc stayed out of the way where it should — the actual coding, subagent orchestration, and verification were unaffected by it, which is correct for a context/workflow compiler. The friction was concentrated at the workflow-orchestration boundaries (milestone setup, finalize/commit), and a couple of those were sharp enough that I worked around the tool rather than with it.
>
> The sharpest edge: finalize commits the staged index, silently
>
> jigc task finalize commits whatever is in git's index — but my edits (via Edit/Write) were never git added. My first commit captured only a file rename, not the actual code changes. I caught it because I habitually inspect the commit, but the natural workflow — edit files, then finalize — silently produces an incomplete commit. After that I prefixed every finalize with git add -A, but that's a footgun a less careful agent would ship straight past. Fix I'd want: finalize should either auto-stage the working tree (or the task's declared scope) or hard-refuse with a warning when there are unstaged changes in scope. Silent partial-commit is the wrong default.
>
> The milestone work-unit was the wrong tool for a brownfield milestone, and I discovered that by hitting a wall
>
> I set up jigc milestone with six sub-tasks, then realized the increments were a linear spine — increment 3 can't even compile without increment 2's plugin endpoints. The milestone surface fans sub-tasks into isolated worktrees pinned to a shared base, so each worktree would be missing the others' code. I had to discard all six sub-tasks, unmanage and delete the committed milestone-record, and fall back to sequential dev-tasks. That was a real detour, and nothing warned me — I found out via pinned to base X but you're on Y.
>
> The milestone unit assumes independent, parallelizable sub-tasks. A large fraction of brownfield work is sequential-with-dependencies. Either detect that shape, offer a "sequential milestone" mode, or at minimum document the assumption prominently. Right now the surface's name ("milestone") reads as general-purpose when it's actually for fan-out.
>
> Smaller friction
>
> - Task-id drift. Re-running jigc start mints the id from the intent text […] Echo the minted id prominently at start, or let doc/finalize fuzzy-match by prefix.
> - Hook-failure UX. The broken lint-staged/eslint pre-commit hook made finalize print "Reverting to original state because of errors" — which briefly read as "your work is gone." […] the revert-and-report should be unmistakably "commit aborted, tree intact," and ideally surface the escape hatch.
> - The author batch verb I opted out of. Its payload (YAML with <<…>> block-scalar markers) was fiddly enough that I used incremental set-slot --from-file - heredocs instead for every multi-paragraph doc.
>
> What genuinely earned its keep
>
> The doctype discipline is the best part, and I'd keep it. Being made to record decisions and deferrals with rationale — the three ADR-equivalents in the decisions log, and especially the deferral ledger ("per-user git-host OAuth is deferred, here's why, here's the trigger that resurfaces it") — is exactly the context that evaporates otherwise. On a brownfield project where the why matters more than the what, that forcing function is valuable. The planning-workflow's create-gate routing an adr attempt to decisions-log was helpful once I hit it. And the typed commit doctype folding message authoring into the workflow is nice.
>
> One calibration note: the planning workflow's output is a long, prescriptive wall (Restate → Verify → Detect → Settle → Review → Decompose → Record). For a brownfield milestone where I'd already done deep verification against the running binary, most of it was orientation I'd effectively completed. I used it as a checklist rather than literal steps, which felt right — but a lighter "you've clearly already scouted; here's the gate that still matters" mode would fit brownfield better than the full greenfield ceremony.
>
> If you change two things: make finalize safe against unstaged changes, and make the milestone unit either handle (or clearly disclaim) sequential dependencies. Those two caused the only moments where I was fighting the tool instead of the problem.

### Bug report (verbatim, condensed headings)

> Correctness footguns: (1) finalize commits the staged git index, not the working tree […] the natural flow (edit → finalize) produces incomplete commits by default. (2) A pre-commit hook failure makes finalize revert with alarming messaging […] read as "your work was destroyed."
>
> Doc/reality mismatches: (3) jigc start --help claims an intent auto-mints; it doesn't — it returned the workflow picker and required a re-run with --workflow. (4) The commit doctype's type/scope location is represented three different ways: doc schema lists them top-level; the workflow hint addresses #type/#scope; validate says "field type in section header"; what works is #header/type.
>
> CLI inconsistencies: (5) milestone create takes positional TITLE, doc create takes --title. (6) Singletons require the redundant type:slug form. (7) There's no jigc doc list.
>
> Design mismatch: (8) jigc milestone assumes independent parallelizable sub-tasks; brownfield milestones are often a sequential spine […] I never exercised the milestone execute/join/finalize happy path at all — which is itself a signal. (9) Discarding sub-tasks leaves a stale, committed milestone-record claiming 6 "active" sub-tasks that no longer existed — a lying record. I had to jigc unmanage it and git rm it manually.
>
> Ergonomics: (10) task ids minted with no prominent echo. (11) The ADR create-gate is discovered by failure. (12) The author batch payload is fiddly enough to avoid. (13) jigc's internal git operations surface as opaque "reset: moving to HEAD" reflog entries […] there was a transient window where I briefly thought all of increment 4's production work was lost. (14) "Never read or edit managed docs directly" fights session-start orientation — the natural first move on brownfield is reading the roadmap/specs; consider narrowing the rule to edits, or making jigc doc show the obvious orientation entry point.
>
> What did not break: no data corruption, no malformed doc output, and validation's blocking messages were clear and correct […] The doctype discipline itself worked as intended once past the address/verb friction.

## The observer's final report (verbatim)

> The strongest thing jigc has done in this entire exercise
>
> planning's Settle checkpoint caught a one-way door — and caught a contradiction I wrote and never noticed.
>
> The decisions-log entry it produced says: "VISION.md contradicts itself here, which is why this needed settling rather than assuming."
>
> It's right. VISION.md states the no-multi-tenancy invariant as settled ("per-customer data isolation is deliberately not a goal… this invariant is load-bearing") and lists multi-tenancy as open question 2 ("decide it, don't drift into it"). I authored both, in the same document, and missed it. jigc's planning loop surfaced it at exactly the moment a milestone depended on it, forced a human decision, and recorded the resolution with its rationale — including why the four ResourcePolicy TODOs become an explicit won't-do rather than an unfinished thought.
>
> That is the feature working better than designed: an open question in the vision resurfacing automatically when work touches it, plus an internal-consistency catch across documents. Do not touch this. It's the best argument for jigc I've seen.
>
> detect-gaps also earned its keep. It noticed pre-update-backup was blocked on a AppConnect write path that didn't exist, and built it first. The plugin now registers real routes. That's the dependency catch the phase exists for.
>
> The gap — and it's the thesis of this whole project
>
> Three specs now assert bugs that are fixed. jigc validate reports zero findings.
>
> | Spec | Still says | Reality |
> |---|---|---|
> | make-the-cms-write-path | every remote write 404s | plugin now has RemoteControl routes |
> | repair-the-audit-quality-gates | isset() always false, gate never fires | fixed |
> | close-the-authorization-holes | routes have no authorization at all | enforced |
>
> The doc set contains multiple documents making false claims about the code, and the tool built to keep documents honest about code says everything is fine.
>
> The reason is structural, and worth stating plainly: the anchor gate checks symbols, not claims. A spec is nothing but claims. So spec is precisely the doctype the anchor gate cannot protect — and it's the one that rots fastest, because its entire purpose is to describe a state of the world you intend to destroy.
>
> And maps-to-test — the one mechanism that could close that loop — has now been written exactly zero times. Across 11 specs, 153 criteria, ~1400 logged invocations, implement-from-spec, and a full five-increment milestone in which real tests were written. Never once, by any agent, under any workflow. That's no longer "underused." It is a field that does not exist in practice.
>
> The milestone close is optional, and it was skipped
>
> completion never ran. The milestone ended with chore(milestone): drop the v1.2 milestone work-unit — no completions/ directory, no completion-record, no verdict. Six commits of real code landed and there is no record that v1.2 happened or whether it holds. Meanwhile the suite is still red: 25 failed / 520 passed. (Genuine progress — it was 33/362 — but red.) The milestone ended anyway. Nothing gated on it, nobody was told.
>
> To jigc's credit: it did not produce a false pass. It said nothing rather than lying — strictly better than GSD. But the shape of the gap is identical: nothing at the milestone boundary is bound to anything executable.
>
> And note what this means for your crown-jewel question: completion is still untested, and we now know it can be bypassed entirely by dropping the work-unit. Whether its audit is genuine or self-scored is still open — but "it can be skipped" is arguably the more urgent finding, because an audit you can walk around is not an audit.
>
> What I'd fix, in order
>
> 1. implement-from-spec must demand maps-to-test for each criterion it satisfies. Not optional, not a suggestion. Zero-for-153 across four independent sessions is conclusive: if the workflow doesn't insist, nobody fills it. This is a prompt change, not an architecture change, and it's the highest-value fix in the tool — it's what turns "a criterion with no passing test is unfinished" from an aspiration into a gate.
> 2. Give spec a lifecycle. A status, a satisfied-by edge, closure at finalize — something that makes "this spec's work is done" a fact in the store, not something a human has to remember. Until then, every completed spec silently becomes a lie, and validate will keep certifying it.
> 3. Make the milestone close non-skippable, and bind its verdict to something executable. If completion can be dropped, it will be. And when it does finally run, if it can return passed over a red suite with a prose owner-artifact, you will have rebuilt the GSD audit inside jigc.
> 4. Keep Settle exactly as it is.
>
> The one-line version for the fold-back: jigc now guarantees your docs name real code. It does not yet guarantee they say true things about it — and the spec doctype is where that bites.

## Log analysis

**Slice:** records 1155–1439 of the live log — **285 invocations** (284 on rc.5, 1 rc.4 straggler from the prior trial's tail), the trial proper all on 2026-07-12 12:45–17:18 UTC. Exit codes: 0 ×246 · 1 ×32 · 2 (usage) ×4 · 3 (blocking) ×3 — **13.7% non-zero overall**, but the split matters: ~9 were *productive* failures (task-validate gate blocks, create-gate blocks, finalize preflight blocks, one correct transient-type refusal) and 11 of the ~30 friction failures collapse into just two identical-timestamp shell-batch cascades against a not-yet-started task and a guessed slug (both self-recovered within seconds via `task list`). The rest is address-grammar guessing and 4 clap usage errors.

**Per-verb:** `doc set-slot` 49 · `validate` 41 (0 failures, report-only) · **`doc show` 39 with the worst failure rate, 35.9%** — all address friction: bare singleton slugs ×7 (`roadmap` → needs `roadmap:roadmap`; the rc.5 V12 bare-slug fix covered `rename`, not `doc show`), guessed field paths ×4 (`#status/cites-code` / `#cites-code`), one correct refusal · `doc set-field` 35 · `start` 25 · `task finalize` 14 (2 gate blocks, both retried→passed) · `doc schema` 11 · `task validate` 8 (3× exit 3 = **the gate working**) · `doc create` 5 (2 productive `create.gate-blocked`) · `milestone create` 3 (2 usage errors — `--title` where title is positional) · `migrate-corpus` 3 · `doc retitle-item` 3 (used to clear the anchor findings) · **`doc list` 2 — the verb doesn't exist**, invoked anyway.

**Per-session:** migration (P1) **27/27 clean, 0% failure** · refactor/anchor-gate (P2) 85 records, 11% · implement-from-spec (P3) 18 records, 6% · milestone (P4) 136 records, 15% · plus an 18-record trailing repro block (deliberate failure replays, excluded from the friction read).

**Findings signal-to-noise:** 121 finding instances across 43 invocations. `file-state.baseline-adopt` 54 · **`doc-code.symbol-exists` 25 = 5 unique anchors × 5 sweeps — the anchor gate firing on the Service collapse, cleared via `retitle-item` within 40 minutes** · `file-state.hash-matches` 14 · `doc-code.unsupported-language` **9 in the rc.4 straggler, 0 under rc.5** — the M41 V4 Vue-SFC arm is live in the field · `store-version.binary-mismatch` 4 (transient, pre-migration only). Against the migration half's 8-actionable/607-advisory (1.3%): the advisory flood is gone (607 → 88), and the dominant rc.4 noise code was replaced by a genuinely actionable catch.

**Specific counters:** `maps-to-test` writes **0** (grep over all argv — confirmed) · `task bind spec` 1 (exit 0, silent) · git corroboration exact — every logged finalize lines up with a commit, incl. the `ValueRemapped` D/I→Decision/Idea migration commit (`64f899e`) and the milestone-record add-then-delete (`abc6e31` → `670a584`).

**The milestone arc collapsed ~80 seconds after creation:** 15:11:35 `milestone create` OK → 15:11:45–47 `add-task` ×6 → **15:11:56 `start --task increment-0-…` exit 1** (a 184-byte error, *no finding code logged*) → 15:12:35 discard ×6 → 15:12:51 `unmanage` the record → the six increments re-run as flat `dev-task`s. The milestone surface's first real use died at the very first `start --task`.

**Two instrumentation gaps the analysis itself surfaced:** (a) **blocked finalizes log `finding_codes: []`** despite ~3.8 KB of output — the block *reasons* are opaque to log analysis (the succeeding retry does log codes); (b) one `task diff` emitted **187,851 bytes** in a single shot (4× the next-largest output) — an output-size ceiling candidate.

## Findings verification

Every claim above was adversarially verified against the rc.5 code (file:line evidence, live repros where cheap): [findings-verification.md](findings-verification.md).
