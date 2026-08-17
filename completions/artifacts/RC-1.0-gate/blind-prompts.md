# The 1.0.0-gate trial — the three verbatim blind prompts

Written 2026-08-16, **before any blind session ran**, and owed by
[protocol.md](protocol.md) §2.1 / [corpora.md](corpora.md) → *Still owed* /
[cue-cards.md](cue-cards.md) → each card's *Precondition on the blind prompt*.

Each prompt is pasted **verbatim** into a fresh session whose `cwd` is that corpus, under
[protocol.md](protocol.md) §2's isolation and §9's declared `bypassPermissions` bound. Nothing is
added, and no clarifying question is answered with anything [cue-cards.md](cue-cards.md) §0.2
forbids.

**What is deliberately absent from all three:** any mention of `QUICKSTART.md`, `MIGRATING.md` or a
guide directory. RC-pre-1.0 §8 rule 4 is **DRAINED** — M48 ships the guides as an adapter-owned
`.claude/skills/jigc/SKILL.md` installed by `jigc setup`, and the harness auto-advertises
`.claude/skills/`. Seeding them would destroy the measurement protocol §4 replaces it with.

---

## 0 · Two decisions this file records rather than leaves to the moment

### 0.1 *"do the whole thing through the tool"* — **the broad form is BARRED; a narrow form is permitted**

[protocol.md](protocol.md) §2.1 names this as undecided and requires the protocol to record it. It
is decided here, before the run.

**RC-pre-1.0's G2 prompt said** *"do the whole thing through the tool"*; G1 said *"do this piece of
work through it"*; G3 said *"build out the project's documentation through it"*. The phrasing was
never uniform, so there is no single precedent to preserve.

**The ruling.** A channel statement scoped to **the documentation work** is permitted — *"the
project's docs are managed with `jigc`, so the thinking goes in through it"*. The unscoped form
— *"do the whole thing through the tool"* — is **not used in any of the three prompts below**.

**Reasoning, in the order it was weighed:**

1. **Neither form violates §3.2 on its face.** §3.2 bars *verify · read back · check the doc ·
   `doc show` · any synonym*. "Through the tool" names no read surface and no act of reading; it is
   a statement about which channel *writes* go through. A mechanical scan (§4 below) confirms zero
   hits for either form.
2. **But the broad form leans on the axis being measured, and the narrow form leans less.** §3.3's
   four outcomes are VERB · VERB-ADJACENT · FILESYSTEM · NEITHER. Telling a worker to do *the whole
   thing* through the tool pushes against FILESYSTEM across **all** its activity — including reading
   source, reading its own notes, and reading its own staged work. Scoping the statement to the
   documentation deliverables leaves the worker's *inspection* behaviour entirely its own, which is
   what §3.2 asks for (*"leaves the channel entirely to the worker"*).
3. **Dropping the statement altogether is worse than the narrow form.** Without it a session may
   simply not route its documents through jigc at all, which lands in §3.4's *"the session never
   reached an authoring step"* row — **unmeasured, not NEITHER**. An instrument that can produce no
   reading at all is a worse instrument than one carrying a declared, bounded nudge. B1's arc
   additionally *requires* the tool to be used, since both its plants fire only at `jigc task
   finalize`.
4. **The nudge is asymmetric in a known direction, and that direction is the safe one.** A narrow
   channel statement makes FILESYSTEM marginally less likely and VERB/VERB-ADJACENT/NEITHER
   marginally more. Under §3.4's readings, that **weakens a positive** (3/3 VERB is partly
   compliance with an operator preference for the tool, on top of §3.4's already-stated compliance
   framing) and **strengthens a negative** (0 VERB despite the operator having said docs go through
   jigc *and* the composed step having printed the read command verbatim). The trial's consequential
   outcome under §1 is the negative one, and this nudge cannot manufacture it.

**Declared bound, registered here before the run:** even the narrow form is an operator channel
preference. A VERB result must be reported as *instructed read-back compliance* (§3.4) **plus** an
operator statement that documents go through jigc — not as discovery, and not as unprompted tool
preference.

### 0.2 *"Finish with a clean finalize"* — retained, with its bound stated

All three prompts end with RC-pre-1.0's closing instruction. It is operationally load-bearing: B1's
two plants and B3's discriminator signal fire **only** at a finalize, and a session that leaves its
tasks open produces no plant evidence at all.

**Bound:** the word *clean* is a mild pull toward `jigc task validate`, which §3.3 scores as
**VERB-ADJACENT**. It is retained because (a) it is the phrasing all six prior trials used, so
dropping it changes the instrument on the axis §2 is already trying to keep indicative-comparable,
and (b) `task validate` is named as *what's-left* by the composed step regardless. Report any
VERB-ADJACENT result with this bound attached.

---

## 1 · B1 · `~/ideas/harborlight` — cold start, both plants

### The prompt

> This project has no docs and no tooling around them yet. The `jigc` CLI is installed — set the project up with it (start with `jigc setup`, then `jigc config set invocation-log true`). Then the piece of work: `IngestQueue` drops the *oldest* sample when it overflows rather than refusing the newest, and we've settled that this is right — under sustained pressure a recent picture beats a stale one. There is nothing to build; the call has been made. What's missing is that the reasoning lives in someone's head instead of in the project, so lay it down as a decision record with its rationale, through jigc. Finish with a clean task finalize.

### Preconditions it satisfies

| Source | Requirement | How |
|---|---|---|
| [corpora.md](corpora.md) | B1 is naive — the worker runs setup and enables the log | Both commands named literally, as every prior trial seeded them |
| [cue-cards.md](cue-cards.md) Card B1 | *"must steer the first task at recording a decision about the ingest queue's overflow behaviour"* | The whole task is that decision |
| [cue-cards.md](cue-cards.md) Card B1 | *"must not name a workflow"* | No workflow name appears; the worker meets the router |
| Task constraint 2 | **must not route to `decided-task`** | See below — this is the prompt's single most load-bearing property |
| [protocol.md](protocol.md) §4 | Do not seed the guides | No guide path is named |

### Why it cannot land on `decided-task` — the shape, not a hope

`decided-task` (`packs/methodology/workflows/decided-task.yaml`) is
`allows-create: [{type: decisions-log, as: log}]`, and `decisions-log` is `singleton: true` — so
`jigc doc rename` refuses it by doctype and Card B1 dies before its trigger
([cue-cards.md](cue-cards.md) §0.5).

Its catalog line, which is what the router shows the worker, is:

```
- decided-task — implement one scoped change test-first, recording its design decision on the running decisions log
```

and its `usage:` requires *"the work is one coherent change you can carry to a single commit **AND**
it makes a design decision worth recording"*.

**The prompt therefore removes the code half outright** — *"There is nothing to build; the call has
been made"* — which is the exact wording of `record-decision`'s own `usage:` (*"a call has been made
and is worth preserving with its rationale, and there is nothing to build — just the record to lay
down"*). RC-pre-1.0's G1 asked for a source comment alongside the ADR; that clause is **dropped
here on purpose**, because a code touch is precisely what makes `decided-task` (and `single-task`'s
implement path) attractive, and no cue card or plant needs it.

The two acceptable landings are `record-decision` and `single-task` — both `allows-create` an `adr`,
both mint identical acks, both carry the read-back line ([cue-cards.md](cue-cards.md) Card B1). The
catalog was rendered live on `1.0.0-rc.11` against this intent and reads:

```
- record-decision — capture a choice you have settled, preserving its rationale with no code to write
```

### Which cue card it feeds, and how the trigger is reached

**Card B1**, target doctype `adr`. Its trigger is *the third `set slot adr:…` ack*, or *a bare
`adr:<slug>` ack immediately following `jigc doc author adr`*. `record-decision`'s
`step:author-adr` solicits `#context`, `#decision`, `#consequences` — the three required slots — so
the trigger string appears on the incremental path at the third ack and on the batch path at the
`doc author` ack. Reached within the first authoring act of the session.

**Slug-collision pre-check ([cue-cards.md](cue-cards.md) §0.7) — derived live on rc.11 against this
prompt's wording**, so the operator can apply §0.7 in seconds:

| plausible worker title | derived slug | vs primary `adr:shed-the-oldest-sample` |
|---|---|---|
| Drop the oldest sample when the ingest queue overflows | `adr:drop-the-oldest-sample-when` | safe (token 1 differs) |
| Drop the oldest sample on ingest queue overflow | `adr:drop-the-oldest-sample` | safe |
| Overflow drops the oldest sample | `adr:overflow-drops-the-oldest-sample` | safe |
| Prefer recent samples when the ingest queue overflows | `adr:prefer-recent-samples-when` | safe |

The prompt deliberately never uses the word **shed**, so the primary utterance is the expected one.
⚠️ **Note for the operator:** the third row above collides with the card's **ALTERNATE**
(`adr:overflow-drops-the-oldest-sample`), not with its primary. If the worker's slug is
`adr:overflow-drops-…`, the primary utterance is still correct and the alternate is unavailable.

### Plants it must not disturb

| Plant | Why the prompt is safe for it |
|---|---|
| **B1-1** rejecting `pre-commit` under `core.hooksPath` | The prompt's only deliverable promotes into `docs/decisions/`, which is exactly what the docs-gate refuses. Fires at the ADR-promoting finalize — the same arc the plants README rehearsed, twice, with `record-decision`. |
| **B1-2** two files staged before the mint (`scripts/retention-sweep.sh` **A**, `src/router.ts` **M**) | The prompt names **no source file and no code change at all**, so the worker has no task-driven reason to touch, stage, unstage or commit either path. The carryover gate fires per path at the first finalize. |

**Watch item, not preventable without contaminating** (carried from the plants README's own bounds):
a worker that tidies the index after `jigc setup` — `git stash`, `git restore --staged` before any
mint — disarms plant B1-2. The prompt says nothing about the working tree in either direction.
Likewise, a worker that reads `.githooks/pre-commit` and unsets `core.hooksPath` to get past it is a
legitimate observation; watch the transcript.

**Sequencing** ([cue-cards.md](cue-cards.md) Card B1): send the correction at its trigger, which is
before any finalize; let both plants resolve after it.

**Discount** ([protocol.md](protocol.md) §4 preload note): setup runs in-session, so `.jigc/AGENT.md`
and the bootstrap `CLAUDE.md` were not in this session's system prompt. B1's read-back numbers are
reported separately and discounted.

---

## 2 · B2 · `~/ideas/pinegrove` — the design altitude from zero

### The prompt

> This service does one thing today: it rolls up time-series samples in memory. Before we build anything else on it, I want the thinking captured properly — starting with the problem I keep circling. Nothing bounds how many distinct series the store will hold: one runaway label and the map grows until the process dies. So investigate that first — what comparable services do about it and what it costs them — then write down what we're betting on, then a roadmap, then get the first milestone under way. The project's docs are managed with `jigc`, so the thinking goes in through it, and each piece finishes with a clean finalize.

### Preconditions it satisfies

| Source | Requirement | How |
|---|---|---|
| [corpora.md](corpora.md) | B2 is **adopted**; setup and the invocation log are committed | Neither `jigc setup` nor `jigc config set invocation-log true` appears. The prompt states the corpus's actual condition (*"the project's docs are managed with `jigc`"*) rather than instructing an install. |
| [protocol.md](protocol.md) §4 | `do-research` → `form-vision` → roadmap → first milestone | Ordered explicitly: *investigate that first · then what we're betting on · then a roadmap · then the first milestone* |
| [cue-cards.md](cue-cards.md) Card B2 | *"must put the session at the design altitude starting from **investigation**, not straight at `form-vision`"* | *"So investigate that first"* — matching `do-research`'s `when:` (*"you need to investigate or gather evidence before forming a vision or making a decision"*) |
| Card B2 §0.5 | The correction must **not** land on `vision` (a singleton `doc rename` refuses) | The research doc comes first and is the only doc in view at the trigger |
| [protocol.md](protocol.md) §4 | Do not seed the guides | No guide path is named |

### Which cue card it feeds, and how the trigger is reached

**Card B2**, target doctype `research`. Trigger: *the third `set slot research:…` ack*, or a bare
`research:<slug>` after `jigc doc author research`. `do-research`'s `step:author-research` solicits
`#question`, `#findings`, `#sources` — three required slots — so the trigger appears in the
session's first authoring act, well before `form-vision` binds `grounded-in` (Card B2's *"one thing
this card must not do"*).

**The prompt gives the research a subject** — *what comparable services do about unbounded distinct
series, and what it costs them* — because Card B2's utterance is a **rename**, not a subject change:
*"that research should be called 'Where comparable services put their ceilings'"* is only a natural
operator correction if the doc is about comparable services and bounding.

**Two words are excluded from the prompt on purpose:**

- **"ceiling"** — the utterance's justification is *"We say ceilings everywhere else"*, which only
  reads naturally if the worker did **not** use the word. The prompt says *bounds* / *grows*.
- **"where … put"** — the primary replacement slug is `research:where-comparable-services-put-their`;
  a worker prompted with that phrasing could derive it and §0.7's arm-killer would fire silently.

**Slug-collision pre-check — derived live on rc.11:**

| plausible worker title | derived slug | vs `research:where-comparable-services-put-their` |
|---|---|---|
| How comparable services bound unbounded series growth | `research:how-comparable-services-bound-unbounded` | safe (token 1 differs) |
| What comparable services do about unbounded series growth | `research:what-comparable-services-do-about` | safe |
| How other rollup services bound distinct series | `research:how-other-rollup-services-bound` | safe |
| How comparable services handle runaway series labels | `research:how-comparable-services-handle-runaway` | safe |
| How comparable services limit distinct series | `research:how-comparable-services-limit-distinct` | safe |

### Plants it must not disturb

**None — B2 carries no plant.** The prompt is correspondingly free of any working-tree or index
instruction.

**Known friction, pre-registered so it is not mistaken for a defect:** `planning` — the milestone
workflow — is `selectable: false` with `suppressed.reason` *"invoked by name with the milestone in
hand"*, so the router will not offer it for *"get the first milestone under way"*. RC-pre-1.0's G2
carried the identical clause. Whatever the worker does there is a **discoverability observation**,
adjudicated under §1's capability-gap or surface rows — and it happens long after Card B2's trigger,
so it cannot cost the measurement.

---

## 3 · B3 · `~/ideas/stonefly` — the corpus accretes

### The prompt

> The project's docs are managed with `jigc`; build them out through it, in this order. (1) A changelog entry for what's shipped so far — the service has been in use for a couple of releases and none of it is written down. (2) A spec for the gap that worries me most: nothing bounds how many distinct series the store will hold, so one runaway label grows the map until the process dies. Write that spec, then implement it. (3) Then the other gap — samples do not survive a process restart, and a bounce loses everything collected since the last one; that has to stop being true. (4) Finally an architecture document covering how ingest, store and rollup fit together. Finish each piece with a clean finalize.

### Preconditions it satisfies

| Source | Requirement | How |
|---|---|---|
| [corpora.md](corpora.md) | B3 is **adopted** | No setup, no invocation-log line |
| [protocol.md](protocol.md) §4 | changelog · spec → implement-from-spec · arch-doc | Items (1), (2), (4) |
| [cue-cards.md](cue-cards.md) Card B3 | *"must reach a `plan` / spec-authoring step before the implement step"* | Item (2) is *"Write that spec, then implement it"*, and it is the **second** item — inside budget |
| [plants/README.md](plants/README.md) **Finding 3** | *"must keep a durability/persistence task or signal 2 is inert"* | Item (3) |
| [protocol.md](protocol.md) §4 | Do not seed the guides | No guide path is named |

### The one real conflict between the inputs, and how it is resolved

[cue-cards.md](cue-cards.md) Card B3 pins the spec's **subject** — its utterance renames the spec to
*'Cap distinct series at a configured ceiling'*, and its captured worker slugs
(`spec:bound-the-number-of-distinct`, `spec:reject-new-series-above`) are all series-ceiling shaped.
Aimed at a persistence spec, that utterance is not a rename at all — it is a different document, and
a worker could reasonably mint a second one.

[plants/README.md](plants/README.md) **Finding 3** requires a **persistence** task, or the foreign
ADR's contradiction trap (signal 2) is inert while signal 1 still fires — *"which makes the loss
silent"*.

**Resolution: both, in sequence, rather than one merged item.** The spec (2) is the series-ceiling
one the card renames; persistence is a **separate later item** (3). This satisfies the card exactly
and keeps the trap alive, at the cost of a four-item arc.

**Consequence for the plant's timing, stated so the operator is not surprised.** Card B3 says land
the foreign ADR *after the spec task finalizes*, and gives as its rationale *"a contradicting ADR
bites hardest while the worker is implementing from the spec anyway."* Under this prompt the **rule
still holds and the rationale shifts**: the plant lands after item (2)'s spec task finalizes
(satisfying both the card and `b3-foreign-adr.sh`'s own ≥2-commits-since-install bar), signal 1 (the
discriminator) is live from that moment, and **signal 2 fires at item (3)**, not during the
ceiling implement. Nothing about the plant changes; only when its second signal is expected.

**Budget risk, declared:** item (4), the arch-doc, is the most likely casualty of a short session.
That is acceptable — Card B3 already chose the spec over the arch-doc precisely because *"a card
aimed at the last doc in the arc is a card that often never fires"*, and §3.4 has no row that
depends on the arch-doc.

### Which cue card it feeds, and how the trigger is reached

**Card B3**, target doctype `spec`. Trigger: *the first `spec:<slug>#criteria/<id>` ack* (or a bare
`spec:<slug>` after `jigc doc author spec`). `plan`'s `step:author-spec` orders `#goal`/`#context`
before the repeatable `criteria`, so the trigger appears once the spec holds prose and at least one
criterion — inside item (2), the session's second piece.

**Two words are excluded from the prompt on purpose** — **"cap"** and **"ceiling"** — for the same
two reasons as B2: the utterance's justification (*"Series ceiling is what we've been calling it in
standup"*) needs the worker not to have used the word, and both replacement slugs
(`spec:cap-distinct-series`, `spec:series-cardinality-gets-a-configured`) start from those words.
Card B3 calls itself *"the highest collision risk of the three"*; the prompt is written to lower it.

**Slug-collision pre-check — derived live on rc.11:**

| plausible worker title | derived slug | vs `spec:cap-distinct-series` |
|---|---|---|
| Bound the number of distinct series the store holds | `spec:bound-the-number-of-distinct` | safe (token 1 differs) |
| Bound distinct series growth in the store | `spec:bound-distinct-series-growth` | safe |
| Limit how many distinct series the store holds | `spec:limit-how-many-distinct-series` | safe |
| Reject new series above a configured maximum | `spec:reject-new-series-above` | safe |
| Cap the number of distinct series | `spec:cap-the-number-of-distinct` | safe (token 2 differs) |

§0.7 is still applied before pasting — the check is mechanical and this table is a prior, not a
substitute.

### Plants it must not disturb

| Plant | Why the prompt is safe for it |
|---|---|
| **B3** foreign ADR at `docs/decisions/0002-keep-the-sample-store-in-memory.md`, landed mid-stream | The prompt names no path under `docs/`, so nothing in the worker's instructions collides with the plant's file. Item (3) is what the plant contradicts, and the prompt's *"that has to stop being true"* leaves both correct resolutions open — respect the decision, or supersede it — without naming either. |

**Reachable-by-design, pre-registered:** [plants/README.md](plants/README.md) **Finding 1** —
`migrate-corpus` claiming the foreign ADR and routing back to itself — is reachable by this prompt's
worker, since items (1)–(4) give it every reason to run a corpus-wide verb once an unadopted doc
appears. It is adjudicated under §1 from its evidence, not pre-classified here.

---

## 4 · Contamination audit of these three prompts

Run mechanically over the three prompt texts, against a list **wider** than
[protocol.md](protocol.md) §3.2's and wider than [cue-cards.md](cue-cards.md) §0.2's:

> verify · verif · read back · read-back · read · check · confirm · review · inspect · look at ·
> show · see · ensure · make sure · double-check · double · validate · open · view · display ·
> print · cat · output · render · contents · doc show · doc list · task diff · task validate ·
> compare · match

**Result: 30 of 31 terms — zero hits. One substring hit, disclosed:**

| term | hits | disposition |
|---|---|---|
| `cat` | 1 | **`invocation`**, inside the literal command `jigc config set invocation-log true` in B1. A substring of an operationally required argv, not the word `cat`, and unreadable as an instruction to read a file. **Accepted.** |

Words that were **avoided deliberately** because a substring scan would flag them, even though their
plain meaning is innocent — recorded so a re-audit does not read their absence as accidental:

- **"already"** (contains `read`) — B3 says *"what's shipped so far"*, not *"what's already shipped"*.
- **"open the first milestone"** (contains `open`) — B2 says *"get the first milestone under way"*.
  RC-pre-1.0's G2 used *"open"*; it is dropped here.
- **"specification"** (contains `cat`) — all three say *spec*.
- **"README"**, **"the summary output"**, **"ready"** — absent entirely.

**Beyond the word list — three phrasings weighed and rejected as contaminating in substance:**

1. *"…and make sure the doc came out right"* — the arm handed over. Never sent.
2. *"…do the whole thing through the tool"* — barred, §0.1.
3. *"…the doc"* as an object to be examined ([cue-cards.md](cue-cards.md) §0.2's last item) — no
   prompt refers to any authored document as something to be looked at; each names it as something
   to be **written**.

**Phrasings retained with a declared, directional bound:**

| phrasing | bound |
|---|---|
| *"the project's docs are managed with `jigc`, so the thinking goes in through it"* (B2/B3) and *"through jigc"* (B1) | §0.1 — an operator channel preference; weakens a VERB result, cannot manufacture a negative one |
| *"Finish with a clean finalize"* (all three) | §0.2 — mild pull toward `task validate`, a VERB-ADJACENT channel |

---

## 5 · What was verified, and what was not

**Verified live on `1.0.0-rc.11`**, in a throwaway corpus instantiated from
[trial-corpus-template](../../trial-corpus-template/) with `--clean-prose` in a scratch directory
(destroyed corpora untouched — none of `~/ideas/harborlight`, `~/ideas/pinegrove` or
`~/ideas/stonefly` was modified, and no jigc command was run inside any of them, since that would
append to their `invocation-log`):

- the **router catalog** rendered against B1's intent, confirming `record-decision`'s line reads as
  a near-verbatim match for the prompt's *"the call has been made / nothing to build"* framing and
  `decided-task`'s does not;
- **all fourteen slug derivations** in the three collision tables, produced by running
  `jigc doc create <type> --title …` and copying the ack — not by predicting the word cap
  ([cue-cards.md](cue-cards.md) records two wrong predictions as the reason this matters);
- the **contamination scan** in §4, run over the three verbatim texts.

**Not verified, stated rather than assumed:**

- **No prompt has been run against a live agent.** Which workflow each session's router-facing
  worker actually picks is a prediction from the catalog text, not a measurement. B1's
  `decided-task` exclusion is argued from the workflow's own `usage:` gate (*"AND it makes a design
  decision"* over *"one coherent change"*), which the prompt negates — but a worker is not a parser.
- **Session budget is unmeasured.** B3's four-item arc is the longest of the three and its item (4)
  may not be reached; §3 states why that is acceptable but not that it will not happen.
- **The slug tables are priors, not a substitute for [cue-cards.md](cue-cards.md) §0.7.** They cover
  plausible titles, not all titles. The operator still runs the check against the *actual* trigger
  line before pasting.
- **Everything was driven on the host binary, not in the container rig.** Same build
  (`1.0.0-rc.11`); the isolation chain is walk arm 0's job to prove ([protocol.md](protocol.md) §5).
- **These prompts have not been read by a second reader.** [cue-cards.md](cue-cards.md) got one, and
  its own closing note argues that a drift-voided session makes a second read cheap insurance. The
  same argument applies here.

---

## Independent review, 2026-08-16 — by the integrating session, not the author

The file asks for a second reader on the same reasoning cue-cards.md did. Done, plus the one claim
that was argued rather than measured.

**Contamination scan, re-run independently** over the three prompt texts against the 26-term list:
**one hit, and it is a substring artifact** — `cat` inside `invo`**`cat`**`ion-log`, in B1's required
`jigc config set invocation-log true`. Disclosed by the author, confirmed here, and not
contamination: the phrase names a config knob, no read surface and no reading act. B2 and B3 are
clean outright.

**The `decided-task` exclusion is now measured, not inferred.** The author argued it from the
workflow's `usage:` gate. What matters is narrower and better: **the router does not choose.** It
prints the selectable catalog and hands the pick to the worker, so B1's routing rests entirely on the
one line the worker reads. Rendered live on rc.11:

```
- decided-task    — implement one scoped change test-first, recording its design decision on the
                    running decisions log
- record-decision — capture a choice you have settled, preserving its rationale with no code to write
- single-task     — implement one scoped change end-to-end, recording its decisions as ADRs …
```

B1's *"There is nothing to build; the call has been made"* discriminates against both code-bearing
entries and onto `record-decision`'s *"no code to write"* near-verbatim. Dropping RC-pre-1.0's
source-comment clause is what buys this, and it costs nothing: no cue card and no plant needed the
code touch.

**Residual risk, stated rather than closed:** a worker may still pick `decided-task`, and no prompt
has been run against a live agent. If B1 lands there, its create-gate is the `decisions-log`
singleton, `doc rename` refuses a singleton outright, and **Card B1 dies before its trigger** — the
session is then *unmeasured* under §3.4's added rows, not a NEITHER. The operator should recognise
this from the first `jigc start --workflow` line and record it immediately rather than reconstructing
it afterwards.

**The `through the tool` decision is sound and its bound is the right one.** The narrow form leaves
inspection behaviour to the worker, which is what §3.2 asks; the broad form would push against the
FILESYSTEM outcome across all activity. The asymmetry is in the safe direction — it can weaken a
positive (3/3 VERB) and cannot manufacture the negative §1 makes consequential — which is the same
shape as the bypassed-permissions bound in §9, and for the same reason.

---

## 4 · The closing feedback prompt — added 2026-08-17, mid-trial, because it was missing

**This file is where every text the operator pastes into a session lives, and this one was not in
it.** The runbook said *"collect the standard feedback with the four-question prompt (RC-pre-1.0's,
unchanged)"* and protocol.md §4 required a *"per-session feedback report"* — but the **words were
nowhere in this trial's directory**, only in the previous trial's protocol. That is the third
instance of the same defect: the protocol opens *"everything needed to execute it is here; nothing
depends on the authoring session's context"* and then defers. §2.1 closed it for corpora and prompts;
this closes it for feedback.

Found by the operator at the keyboard, at the moment of use, which is the expensive way to find it.

**Paste verbatim into the session, at its end, while it is still fresh** — never after exiting, and
never into a later worker's session.

> About the `jigc` CLI specifically: (1) what confused you; (2) what did jigc tell you that turned
> out to be wrong or misleading; (3) what did you look for — a command, a flag, a way to read or
> write something — and not find; (4) what did you do around jigc rather than through it (any direct
> file read/edit, raw git command, or other workaround touching managed docs), and why?

**Why it is unchanged from RC-pre-1.0.** The discoverability findings of the last four trials came
out of these reports rather than out of the logs, and question (4) is the one that produced
RC-pre-1.0's sharpest datum — a worker volunteering *"I never read back a single managed doc I
wrote"*. Changing the wording would forfeit the only part of this trial that stays directly
comparable after the isolation change (§2).

**It is asked, and it is not evidence.** §3.3 is explicit that the log and the transcript are the
instrument and the feedback report is context — a worker's account of its own conduct is the thing
this trial's measurement was built to stop relying on.

**Save it as `feedback-B<n>.md` in this directory, never into the corpus** — a tracked `.md` inside
the corpus becomes an ingest candidate and changes what a later session's `jigc ingest` reports.
