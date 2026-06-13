# Pre-registration — the native-grain thesis test (self-hosting, structure-rich)

**Frozen before any arm runs.** This is the controlled comparison the M17 verdict named as owed
([../VERDICT.md](../VERDICT.md)): a **native-grain** jigc-vs-static test where the differentiators are
actually in play, with an **independent judge**. The pilot's single-task comparison tied because a
lone code task has no structure to get wrong; this test is deliberately **structure-rich** so the
determinism boundary (structure → CLI) has something to bite on. Everything below is fixed in advance;
deviations during the run are recorded as amendments with timestamps.

## Hypothesis (falsifiable, with its null)

**H1:** at native grain, arm J (jigc) commits **fewer structural-integrity defects** than arm S
(the same methodology as a static `CLAUDE.md`, structure by hand) on the same intent — because the
CLI owns placement, cross-ref wiring, the edge index, and the finalize gate.
**H0 (the null we must be able to return):** the two arms commit a statistically indistinguishable
number of defects — a capable model hand-structures correctly from good static conventions, and
jigc's machinery adds no integrity advantage at this grain. **A tie or an arm-S-wins result is a
real, reportable outcome.** The test is designed to be able to find it.

## The matched intent (verbatim, identical to both arms — bare, no doctype names)

> Document this project's **reconciliation / file-state** subsystem as living architecture
> documentation: an overview of how out-of-band edits to managed files are detected and routed
> (absorbed, conflict-blocked, or baseline-adopted), the file-state baseline model, and the key
> components — each tied to the code that implements it. Also record the design decisions behind it
> as decision records, including the decision that **the absorb sweep advances the file-state
> baseline only at a landed finalize** (which replaced the earlier behavior of advancing it on every
> sweep). Where a new decision replaces an older recorded one, link them.

Notes on the intent's design: it names the *deliverables* (architecture documentation + decision
records) and the *content* (reconciliation subsystem; the baseline-advance supersession) in
tool-neutral prose. It does **not** name the `arch-doc`/`adr` doctypes, `cites`/`supersedes`/
`implemented-by`, or `jigc`. Each twin's `CLAUDE.md` supplies the structural expectations — arm J via
the jigc adapter (the agent discovers the doctypes through `jigc`), arm S via the frozen conventions
block. This keeps neither arm spoon-fed.

## Pre-seeded state (identical, committed, in BOTH twins before either arm starts)

Two committed ADRs in `decisions/`, byte-identical across the twins, giving both arms the same
cross-reference surface (so a wrong/dangling cross-ref is the arm's fault, not the setup's):

1. **`decisions/0001-absorb-advances-baseline-every-sweep.md`** — title *"Absorb advances the
   file-state baseline on every sweep"*, `status: accepted`. **This is the decision the new ADR must
   supersede.** (It encodes the pre-M17 naive rule.)
2. **`decisions/0002-oob-edits-detected-and-routed.md`** — title *"Out-of-band edits are detected and
   routed, never silently merged"*, `status: accepted`. A plain `cites` target for the arch-doc.

Both twins also start from the same jigc source HEAD (so `implemented-by` anchors target the same real
`reconciliation` symbols). Arm J's twin additionally has `jigc setup` with the **dev pack** (the
embedded domain pack that ships `arch-doc`, `adr`, and the `architecture-documentation` + `single-task`
workflows — see amendment 1); arm S's twin has the frozen-conventions `CLAUDE.md` and **no jigc** (no
`.jigc/`, no pack, the binary is not referenced).

> **Amendment 1 (pre-run, no arm has run, 2026-06-13):** arm J uses the **dev pack alone**, not the
> methodology+dev composition the draft named. Rationale: the dev pack is the domain pack that owns
> `arch-doc`/`adr` and the `architecture-documentation`/`single-task` (supersede) workflows; the
> methodology pack is jigc's *milestone-process* pack (planning/completion/dev-task), irrelevant to
> authoring architecture docs, and composing it would add catalog entries the blind arm-J agent
> would have to wade through — a confound, not the thesis. Using the domain pack that fits the task
> is the honest setup.

## What each arm must produce (the deliverable, identical target)

- **One arch-doc** for the reconciliation subsystem: an overview, ≥3 components each with a prose
  description **and** an `implemented-by` anchor to a real symbol in the reconciliation/file-state
  code, and `cites` to the relevant ADRs (including the two pre-seeded ones and the new one).
- **One new ADR**: *"Absorb advances the file-state baseline only at a landed finalize"*, which
  **supersedes** pre-seeded ADR 0001, with the older ADR's status reflecting that it was superseded.
- Committed (arm J: through `jigc finalize`; arm S: a normal git commit).

## The frozen conventions block (arm S's `CLAUDE.md` — faithful transcription of what jigc enforces)

Arm S targets the **same artifact shape** jigc produces; the only difference is no tool enforces it.
The block (verbatim, derived from the dev pack's `adr.yaml` + `arch-doc.yaml`):

```
## Architecture documentation & decision records (conventions)

Decision records (ADRs) live in `decisions/`, one file per decision, filename `<slug>.md`
(slug = the title, lowercased, words joined by hyphens). Each has an H1 title, then:
  ## Status   — fields: status (proposed|accepted|superseded), date (YYYY-MM-DD),
                supersedes (the id of the ADR this one replaces, if any).
  ## Context  — why the decision was needed.
  ## Decision — what was decided.
  ## Consequences — tradeoffs and follow-on effects.
When a decision replaces an older one: the new ADR names the older in `supersedes`, and the older
ADR's `status` becomes `superseded`.

Architecture docs live in `architecture/`, filename `<slug>.md`. Each has an H1 title, then:
  ## Meta     — fields: cites (the ids of the ADRs that shaped this part), comma-separated.
  ## Overview — what this part of the system is and the boundary it owns.
  ## Components — one `### <Component name>` per component, each with a prose description and an
                `implemented-by` line naming the exact code symbol (function/struct/enum) that
                implements it. The symbol must exist in the codebase.
Cross-references (supersedes, cites, implemented-by) must point at things that actually exist:
a referenced ADR id must resolve to a real file; an implemented-by symbol must be in the code.
```

Arm J gets **none** of this in prose — it gets the jigc adapter, and discovers the same structure
through `jigc` (`jigc start`, `jigc describe`, the composed workflow). Fairness check before the run:
confirm the frozen block encodes the same semantic rules jigc enforces (no more, no less).

## The defect rubric (tool-neutral, semantic integrity — applied by the independent judge to BOTH arms)

Counted on each arm's **committed** artifacts. These are integrity properties any doc-as-code system
would assert — **not** jigc-specific byte-format pedantry (the `<!-- fields -->` HTML-marker encoding
is explicitly **not** scored; it's jigc's writer detail, irrelevant to the thesis):

| # | Defect class | How the judge checks |
|---|---|---|
| D1 | Doc misplaced | arch-doc not in `architecture/`, ADR not in `decisions/` |
| D2 | `supersedes` wrong/dangling | the new ADR's supersedes target is missing, points at a nonexistent ADR, or names the wrong one (must be 0001) |
| D3 | Superseded ADR not updated | 0001's status not changed to `superseded` (an inconsistency the new edge implies) |
| D4 | `cites` dangling | an arch-doc cite names an ADR with no corresponding file |
| D5 | `implemented-by` unanchored | the named symbol does not exist in the code (judge greps the repo) |
| D6 | Missing required structure | a component lacks a description or an implemented-by; arch-doc lacks an overview |
| D7 | Other dangling cross-ref / internal inconsistency | any reference whose target doesn't exist |
| — | **Process-caught (separate column, not a defect)** | did the arm's process *catch/block* a would-be defect before commit? (jigc's validate/finalize gate vs. arm S having no gate) — recorded from each arm's transcript/commit history |

**Headline metric:** total committed defects (D1–D7) per arm. **Secondary:** process-caught count
(the validate-against-reality payoff, if any fired). A defect arm J's gate *blocked* (so it never
committed) counts as **process-caught for J**, not as a committed defect — that asymmetry is the
thesis, made visible.

## Run mechanism (how "fresh independent sessions" is realized inline)

Each arm runs as **one blind subagent** (general-purpose, fresh context), given **only** the verbatim
intent + its twin's path + "work in this repo, follow its `CLAUDE.md`." Neither subagent is told the
other exists, that this is a comparison, or what the metric is. The orchestrator does not intervene
mid-run beyond the one allowed clarification (below), identical to both. The two subagents are
independent contexts, so there is **no cross-arm carryover** (the confound that one-context
back-to-back runs would have). **Bound named:** subagents approximate, but are not identical to, a
human-driven fresh `claude` session; they are Claude agents with full tools and an independent
context, which is the property the comparison needs. Arm J may use `jigc`; arm S has no jigc in its
environment and is not told it exists.

## Allowed intervention (identical to both arms, only if asked)

One clarification, verbatim if and only if the agent asks what "the reconciliation subsystem" refers
to: *"The code and design that detect and route out-of-band edits to managed files — see the
reconciliation / file-state handling in the engine and CLI."* No other steering.

## Stop condition

An arm ends when it commits its deliverables, or when it (a) declares done, (b) gets stuck/loops for
>~15 tool-cycles with no progress, or (c) errors irrecoverably. Partial output is judged as-is
(incomplete deliverables score the missing-structure defects).

## Arm order & judging

- **Arms run independently** (parallelizable); order is not a carryover risk (independent contexts).
  Recorded order: J then S (or concurrent).
- **Judge:** OpenAI **Codex** (a different model family), given the two arms' committed artifacts
  **de-identified** (jigc-specific tells — `.jigc/`, tool names, commit trailers — stripped or
  neutralized so the judge can't tell which arm is which) + the verbatim rubric above + the repo for
  symbol-existence checks. The judge returns a per-defect-class count for each (anonymized) arm with
  evidence (quoted line / failed grep). The orchestrator then de-anonymizes and records.
- **Honesty bounds (carried into the result):** n=1 per arm; subagents ≠ human sessions; the
  orchestrator authored both environments (the arms run blind, but the setup is mine); de-identification
  is best-effort (artifacts may carry residual tells); a single structure-rich task is one point in a
  space, not a distribution.

## Success criteria (what makes this test *valid*, regardless of which way it goes)

1. Both arms produced committed artifacts from the bare intent.
2. The judge applied the rubric to de-identified artifacts and returned per-arm defect counts with
   evidence.
3. The result is reported with its bounds — **including** if it is a tie or arm-S-win (H0).
4. ≥1 defect class showed a real difference **or** the verdict explicitly states no difference was
   found. A test that can only confirm H1 is rigged; this one can return H0.
