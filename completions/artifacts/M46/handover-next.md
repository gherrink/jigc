# Handover — the rc.12 trial, then the 1.0.0 call

Written 2026-08-20 by the session that planned, built, audited and closed **M46**, while the context
is live. Nothing below needs reconstructing from a transcript.

## State you're inheriting

| | |
|---|---|
| HEAD | `8545e56`+, pushed, tree clean |
| Binary | **`1.0.0-rc.12`** at `~/.local/bin/jigc`, built **after** the audit fixes |
| Gate | **2847 passed / 0 failed**, clippy + fmt clean, measured at HEAD (not inherited) |
| M46 | **complete and audited** — [VERDICT](VERDICT.md) · [DECISIONS.md](../../../DECISIONS.md) → 2026-08-20 M46 complete |
| Capability ledger | **every entry disposed** — none left deferred on a count |
| Open waves | none. The next act is a **trial**, not a milestone |

## What you're doing

**A trial on `1.0.0-rc.12`, then the 1.0.0 call — which is the human's.** There is no chartered wave
waiting; if the trial produces findings, that adjudication charters the next one.

**Do not open with `/milestone-plan`.** M46's handover said *"then run `/milestone-plan`"* because a
wave was chartered. This time the instrument is a trial, and the reusable tooling for it is already
built and indexed in [CLAUDE.md](../../../CLAUDE.md) → **Trial tooling**:
[trial-corpus-template/](../../trial-corpus-template/) (foreign pre-jigc corpus, `instantiate.sh
--clean-prose`, `check-corpus.sh`) and [trial-harness/](../../trial-harness/) (sha-pinned container,
`build-image.sh` / `verify-image.sh` / `verify-pair.sh` / `run-session.sh`). **That paragraph exists
because the corpus template was committed precisely so it would be reused, nothing referenced it, and
the next session assumed it was lost and planned to rebuild it.** Check it before you build anything.

## Two things must land in the protocol BEFORE the first blind session is briefed

Both are recorded, both have live triggers, and both are cheap to miss.

**1 · The cue card must be replaced.** The designed-need cue card **never fired in any of four
sessions** at the 1.0.0-gate trial — the window between its trigger and the `finalize` that closed the
doc's staged life was 3–5 seconds, so it was *impossible*, not mistimed. Consequences: the correction
path is **untested, not passed**, and `doc rename` was reached by nothing. The redesign and the rule
it yields live in [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md). **The rule, worth
applying before you build any instrument:** one fires reliably iff its **trigger is a state**, its
**consequence is re-raised by the product**, and **every worker behaviour maps to a scored outcome**.
Three plants satisfied it and fired; the cue card satisfied none and did not.

**2 · M46's declared exit-semantics change must be pre-registered.**
[decisions-pending.md](../../../implementation/decisions-pending.md) → *the next trial's protocol*
carries it in full. In short: `jigc validate` now **exits non-zero** once un-adopted files sit at
managed homes (`schema-conformance.unadopted-instance` is `STORE_EXIT_FLIPS`' fifth member), and
`migrate-corpus` excludes a never-adopted foreign file before its fold. **The finding itself is
untouched** — advisory, `GATES_NOWHERE`, routed at `jigc ingest`; no task gate, no `finalize`, no
pre-commit hook starts blocking. But a blind session's *first act* on a brownfield corpus is
`jigc validate`, so this is met within minutes on every adoption arm, and an unbriefed observer scores
it a regression on a repo that has done nothing wrong. Two consequences to design for rather than
discover: an adoption arm **cannot use `jigc validate`'s exit code as its clean-start signal** (read
the findings, not the code), and a session that adopts its foreign docs **should see the exit go to 0
as the last of them lands** — a positive-control-shaped check the protocol gets for free.

*One correction to a claim the closing session made aloud:* it said M46 shipped **two** declared
behaviour changes needing pre-registration. Only the exit-semantics one is regression-shaped. The
four-door `--ignored` narration adds output at exit 0 and refuses nothing new — worth mentioning in
the protocol as context, not as a regression exemption.

## What the trial should reach that no trial has

1. **`doc rename` / the identity surface.** Zero calls across all four blind logs *and* the control at
   the 1.0.0-gate trial; M48's second-largest increment is carried entirely by tests and one operator
   rehearsal. The postmortem's recommendation is the **abandoned-task corpus** — ship the corpus with
   an open task already staged whose doc's title is wrong in a way a committed doc settles. It reaches
   `doc rename` deterministically, needs no operator utterance, and makes the read genuinely necessary.
2. **A mixed corpus.** No trial has ever built one store holding foreign + stale-managed +
   ahead-stamped files at once — the real adopter shape, and the exact state M46's PT-1 fix
   reclassifies. Every prior `migrate-corpus` result was measured on a single-class corpus.
3. **A continuation arm authored under rc.11 or rc.12**, not only rc.10. M46 changed predicates on
   `migrate-corpus` and the changelog gate, both of which act on **existing corpora**.
4. **The four-door narration and the concurrent-write merge**, since the standing rule is *the
   acceptance must reach what the wave changed*. The merge held under 9–12 simultaneous processes in
   the audit; a trial arm should meet it through ordinary use, not a stress harness.

## Declared bounds M46 ships with — do not re-discover them as findings

1. **The `--ignored` loss is visible, not prevented.** A gitignored secret inside a fan-out worktree
   is still destroyed at the four doors; it is **named first**. Refusal was refused on measured
   evidence: a worktree that did its job holds build output (1 → 4 → 43 entries as the probe widens),
   so refusing on that axis fires on the ordinary fan-out **success** path and trains `--force` into
   reflex. *Re-opening condition:* an adopter reports real work lost to a **narrated** teardown.
2. **N-3's obligation is narrower than "fix concurrency."** What shipped is *the reconciliation state
   machine must not be silently disabled by a concurrent write* (`CLAUDE.md:50`). General store
   locking is out.
3. **`is_route_exempt`'s narrowing was refused**, with its measured ceiling of **13** blocking
   `conformance.*` producers recorded rather than spent.
4. **`storage.md:121` still reads as a universal** over a door list. Whether it gains the exclusion or
   a pointer was scoped to increment 2's doc work; re-read it at the next wave.

## Lessons this session paid for

- **An agent's report is a lead, not a measurement.** This session relayed subagent claims into a
  document **four** times and was wrong or incomplete every time — `tasks.json` "does not exist" (it
  does, `milestone.rs:41`), `finalize.base-mismatch` as an unnoticed hole (it is a recorded exclusion,
  M47 Settle Decision 1), N-4 in both directions, and an advocate's fan-out topology claim my own race
  test falsified. The rule now binds subagent output exactly as it binds docs.
- **A premise checkable in one command must be checked before it becomes a decision.** D3 was
  affirmed, taken by the human on that recommendation, and withdrawn the same day: `jigc ingest` is
  register-only, so it adopts *without* a stamp and the population the affirmation called "empty at
  the pin" is refilled continuously.
- **Don't hand a fixer its finding's own boundary.** F4's fixer, told to derive the axis instead, found
  a fifth writer nobody had counted. Every fixer that was given room to widen or narrow its finding
  used it correctly; the ones handed a boundary would have shipped the reported instance.
- **`git diff --cached` is not the index.** It shows changes against HEAD; a tracked unmodified file is
  in the index and absent from that output. Use `git ls-files --cached` for membership,
  `git show :<path>` for indexed content. This nearly put a false fact in the record.
- **Measure exit codes unpiped**, and remember zsh has no `PIPESTATUS` (it is `pipestatus`, 1-indexed).
  The shell guard caught this twice in one session.

## What I'd watch for

The completion audit found **two of its four findings inside M46's own increments** — an overclaim
shipped by the wave whose claim is that every fix reconciles one, and an un-swept sibling inside the
sweep increment's own stated axis. Neither was a design defect; both were incomplete sweeps. **Expect
the same shape from anything this session built**, and point the complete-fix lens at M46 the way M46
pointed it at M47 and M48.
