# The cue card — post-mortem, and the mechanism that replaces it

Written 2026-08-18, at the close of the 1.0.0-gate trial, from the four archived invocation logs
and the operator log rather than from recall. It is owed by nothing; it exists because the trial's
**designed-need instrument returned no data in four sessions out of four**, and the same event is
the reason [coverage.md](coverage.md) has to report the wave's second-largest increment as reached
by no session and no walk.

Two things are being separated throughout, and the separation is the whole point:

- the **measurement** the cue card was built to serve — *does an agent read its own staged work back
  through `jigc doc show --task`* — which **succeeded anyway, 4/4, without the card**;
- the **coverage** the cue card was the only route to — `jigc doc rename` and the identity guards —
  which **nothing reached**, and which no other part of the trial was designed to reach.

The instrument failed at both, but only the second failure cost anything.

---

## 1 · What the instrument was, and what it returned

Partway through an authoring task, the operator was to paste one natural sentence changing a name —
*"Actually — that decision should be called 'Shed the oldest sample on ingest overflow'."* — and
nothing more. The sentence names no read surface (an absolute contamination rule,
[protocol.md](protocol.md) §3.2), and was designed to do two jobs at once: force the **F2** identity
path M48 built (`write.identity-change` → `jigc doc rename`), and manufacture an occasion where
reading the doc back is the prudent next move.

[cue-cards.md](cue-cards.md) is a careful document. Every ack, refusal, route and derived slug in it
was captured live on `1.0.0-rc.11`; two of the author's own slug predictions were wrong and were
corrected by running the rename rather than reasoning about it; the singleton dead ends were checked;
a same-slug no-op hazard was found and given a mechanical pre-send check (§0.7); a second reader
scanned all six utterances against a widened contamination word list and found zero hits. The
document names its own largest untested assumption in plain words:

> **No card was rehearsed against a live agent.** The trigger strings are real, the timing (§0.3) is
> reasoned from how Claude Code delivers queued messages, not measured. The narrow-window risk is the
> largest untested assumption here.

**That assumption is the one that failed.** Everything verified against the binary held. The one
thing verified against nothing is the one thing that decided the outcome.

**Result: 0 fires in 4 sessions.** B1 `harborlight`, B2 `pinegrove`, B3a `stonefly`, B3b `rosewater`
— every worker ran its entire authoring arc to completion with no pause an operator could inject
into. `doc rename` appears **zero times** in all four blind invocation logs and in the control.

---

## 2 · The timing evidence

Reconstructed from `<corpus>/.jigc/logs/invocations.jsonl` in each archived corpus. The **window** is
the interval between the trigger the card told the operator to watch for and the `task finalize` that
closed the doc's staged life — the entire time in which a correction could have landed on a
renameable doc.

| session | target | trigger the card specified | trigger | finalize | **window** | longest silence inside it |
|---|---|---|---|---|---|---|
| B1 `harborlight` | `adr` | 3rd `set slot adr:…` ack (`#consequences`) | 05:32:33 | 05:32:47 | **14 s** | 4 s |
| B2 `pinegrove` | `research` | bare `research:…` ack after `doc author` (fallback) | 15:15:19 | 15:15:38 | **19 s** | 17 s |
| B3a `stonefly` | `spec` | first `spec:…#criteria/<id>` ack — **never occurred** | 04:47:48 (fallback) | 04:47:59 | **11 s** | 9 s |
| B3b `rosewater` | `spec` | first `spec:…#criteria/<id>` ack — **never occurred** | 05:24:12 (fallback) | 05:24:28 | **16 s** | 14 s |

Three facts fall out of that table, and each is a separate failure.

**2.1 · The window was 11–19 seconds.** §0.3 told the operator to *"send the utterance the moment the
trigger string appears, without waiting for a turn boundary"*, and warned the window was *"genuinely
narrow"*. It was narrower than the warning implies: the operator had, at best, nineteen seconds to
notice a string in a scrolling transcript, run the §0.7 slug check, `cat` the right paste file, copy
it and paste it. In B1 the same span is fourteen seconds and the longest silence inside it is four.
This is not a window a human operator hits; it is a window a human operator is told about
afterwards.

**2.2 · The trigger string itself did not exist in half the sessions.** Card B3's primary trigger was
the ack `doc add-item` prints for a spec criterion. Across the two spec-authoring sessions,
`doc add-item` was invoked on a spec **zero times** — both workers used the batch `doc author` path
and wrote slots and criteria in one persist. The only seven `add-item` calls in the whole trial are
B3a's, on an `arch-doc`, at the very end of its arc. The card carried a batch fallback, so the arm
was not dead — but the trigger it was designed around was derived from the incremental authoring path
while three of the four target docs were written the other way: **`research`, and both `spec`s, were
authored in a single `doc author` call** that collapses create and every slot into one invocation,
and therefore into one turn. Only B1's `adr` took the incremental `create` + `set-slot` path the
trigger assumed — and B1's window was the shortest of the four.

**2.3 · The card was trying to buy, with a fourteen-second window, something the worker had already
done two seconds earlier.** In **all four** sessions the worker's own `jigc doc show <addr> --task
<id>` sits *inside* the very window the card was aiming at, 2 seconds after the authoring write and
before `finalize`:

```
B1   05:32:37  set-slot …#options        B2   15:15:19  doc author research
     05:32:39  doc show … --task              15:15:21  doc show … --task
     05:32:47  task finalize                  15:15:38  task finalize
B3a  04:47:48  doc author spec           B3b  05:24:12  doc author spec
     04:47:50  doc show … --task              05:24:14  doc show … --task
     04:47:59  task finalize                  05:24:28  task finalize
```

The read-back the occasion was built to manufacture was **already the workers' habit**, 3 / 4 / 6 / 7
times per session, unprompted, in the same seconds the operator was supposed to be interrupting.

---

## 3 · Why it failed structurally, not by bad luck

Four causes, in increasing order of how much they generalise. Only the fourth is interesting.

**3.1 · The delivery channel is turn-bounded and the arc is one turn.** Claude Code delivers a queued
user message at the end of the assistant's current turn. An authoring arc that runs
`create → author → show → set-field → validate → finalize` without asking the operator anything is
**one turn**, and a message queued inside it arrives *after* `finalize` — i.e. after the doc's
identity has been committed, at which point the correction converts to the committed-refusal
observation §0.3 pre-registered. So even a perfectly attentive operator with a stopwatch does not
land the utterance on a staged doc unless the worker happens to yield. The card's design assumed a
gap between "the operator can paste" and "the worker receives"; the real constraint is that the
worker receives **only where it stops**, and it did not stop.

**3.2 · The instrument depended on a state the operator did not control and the tool could not
create.** A cue card needs a pause. Nothing in the corpus, the pack, the binary or the prompt
produces a pause. The one variable the whole arm rested on was the only variable outside everybody's
reach.

**3.3 · Its failure mode is silence.** When the card does not fire, nothing is written anywhere. There
is no refusal, no advisory, no log record, no artefact — the session simply ends, indistinguishable
from a session where the arm was never designed. Compare the arm's *other* silent failure, §0.7's
same-slug no-op, which the author found and fenced with a pre-send check: that one was caught because
it produced an ack (`— the id is unchanged`) somebody could read. The narrow-window failure produces
nothing to read.

**3.4 · The consequence lived only in the operator's sentence.** This is the load-bearing one. A
cue card is a **one-shot, unrepeatable, tool-invisible event**. If the utterance misses its moment,
nothing re-raises it: jigc has no knowledge that a correction was owed, so no `validate`, no
`finalize`, no advisory, no route ever mentions it again. The instrument had exactly one chance per
session and no recovery path — which, for an instrument whose firing condition it could not control,
is the same as having no firing condition at all.

---

## 4 · The comparison that names the mechanism: why the plants fired

Two plants ran in the same trial, under the same operator, in the same rig.

**The B3 foreign-ADR plant (B3b) fired, and was fully consumed.** It landed at 05:23:40, on the
worker's first successful `finalize` — the fire condition being a **count over the corpus's own
history** (`git rev-list --count $INSTALL..HEAD`), polled from outside the session, not a string
watched inside it. The worker met it through its own `jigc validate`
(`schema-conformance.unadopted-instance`), followed M42's discriminator to `ingest` / `migrate`, ran
**zero** `migrate-corpus` invocations, and then **superseded** the decision it contradicted, unhinted
— the trial's strongest single result. **The carryover plant fired too**, in the weaker sense that it
was live and reached: the worker met it in its first minute and defused it by unstaging both paths
before minting, which is itself a result and produced a real finding about what the gate can and
cannot see.

The structural difference the brief names — *a plant is an event landed into the corpus; a cue card
is an utterance typed at a pause that must exist* — is right, and it decomposes into four properties.
The fourth is the one nobody had written down.

| property | plant | cue card |
|---|---|---|
| **trigger is a state, not a moment** | the corpus either holds the foreign ADR or it does not; true continuously until acted on | a string in a scrolling transcript, true for seconds |
| **firing is a command, not a vigil** | `b3-foreign-adr.sh /work`, with its own precondition bar (`git rev-list --count $INSTALL..HEAD`) refusing a premature fire — the guard was **observed refusing** | watch, judge, run a slug check, `cat`, copy, paste, inside 14 s |
| **failure is loud** | a refused plant prints `refusing: …` and plants nothing; the B3a rig failure printed `chmod: … Operation not permitted`, `plant exit=1`, and voided two arms **visibly** | nothing is printed, nothing is written, the session looks normal |
| **the product re-raises it** | `unadopted-instance` fires on **every** `validate` and `finalize` until adopted — so reliably that B3b-3 is a *complaint about the repetition* | jigc never knows a correction was owed; missed once, gone forever |

**B3b-3 is the sharpest evidence in the trial for the mechanism.** A worker complained that the
planted file's advisory *"resurfaced on every validate/finalize call across three unrelated tasks"*.
That noise **is** the reliability. An instrument whose consequence the product keeps restating cannot
be missed by being late.

**And the trial contains a direct demonstration that a plant can manufacture the very pause a cue card
needs.** B1's docs-gate hook rejected the ADR-promoting `finalize` at 05:32:47. The next invocation is
at **06:43:10 — a 4 223-second gap** in which the worker had stopped, asked the operator a question,
and waited. The operator spoke into that pause and the session continued. A second stop, later,
produced the second intervention. **Both moments where an operator successfully spoke into a blind
session were stops the *tool* created, not stops the schedule predicted.** The instrument that could
not find a fourteen-second window was running beside an instrument that had opened a seventy-minute
one an hour earlier.

---

## 5 · Replacement designs

Judged against the brief's four criteria, plus one the trial's own history forces:

1. **Forces the surface** — does it drive `doc rename` / `write.identity-change` /
   `write.title-ignored` / `write.already-present`, or only near them?
2. **Survives a non-stopping worker** — does it need the worker to yield?
3. **Clean under the contamination rule** — does the operator have to say anything at all?
4. **Fireable mechanically** — is it a command with a precondition bar, or a vigil?
5. **Every branch scores** — is there any worker behaviour that returns *no data*? (§3.3's lesson:
   an instrument with a silent branch will take it.)

Ratings are ● yes / ◐ partly / ○ no.

### A · A mid-session commit landed by the operator that renames a concept

The operator commits, into the live corpus, a change to committed prose or code that renames a
concept the worker is working with.

| 1 forces | 2 survives | 3 clean | 4 mechanical | 5 all branches score |
|---|---|---|---|---|
| ○ | ● | ● | ● | ○ |

**Against, as the identity instrument.** It lands reliably and needs no utterance, but its consequence
is **invisible to jigc**. A prose rename in a foreign or unmanaged file trips no probe; a conformant
edit to a *managed* doc is absorbed by reconciliation by design. So the worker only meets it if it
happens to re-read a file it has already read — which is the same "the worker must do something
unprompted" dependency the cue card had, wearing a git commit as a costume. And it does not reach
`doc rename` at all: the concept moving in prose does not make any *document's identity* wrong.

**Keep it for one job it is genuinely good at:** aimed at a **managed** doc with a *non-conformant*
edit, it is the only cheap way to drive the reconciliation/`file-state` conflict path in a blind
session. That is a different probe with a different answer key, and it should be chartered as one.

**Falsified by:** a session where the operator lands the commit and the worker's subsequent
invocations show no finding, no re-read and no behaviour change — which is the expected outcome, and
is why it is not the instrument of record.

### B · A plant that makes an existing managed doc's title wrong

Land a commit that makes a **committed** managed doc's title false — the classic form being an
`arch-doc` component whose named symbol the commit renames.

| 1 forces | 2 survives | 3 clean | 4 mechanical | 5 all branches score |
|---|---|---|---|---|
| ◐ | ● | ● | ● | ◐ |

**Partly for.** The symbol-rename variant has the property A lacks: **jigc surfaces it itself**.
`doc-code.symbol-exists` / `title-names-symbol` fire on the doc's own validation, so the worker meets
the plant through the tool on a path it already runs. That is the reliability property, present.

**Against, on the surface it reaches.** A wrong *component item* title routes to
`jigc doc retitle-item` — a different verb, a different guard, and not the coverage hole. And a wrong
**document** title on a **committed** doc is not `jigc doc rename`'s subject at all: `doc rename`
refuses a committed identity outright and routes at the top-level, task-less `jigc rename`
(cue-cards.md:65). So this reaches T10's refusal and the top-level verb, never the staged in-task
path that is the bulk of Increment 2.

**Worth running anyway**, as its own arm, because top-level `rename` over a corpus with real referrers
is itself unreached by this trial: [coverage.md](coverage.md) records arm 2's three `jigc rename`
attempts as the top-level verb but §5 arm 4's chartered `doc rename`-on-committed probe as absent.

**Falsified by:** a rehearsal where the operator renames the symbol and the doc's validation stays
clean — i.e. if the anchor does not actually bind the way the plant assumes. Check that before
relying on it; this project has misread a green five times.

### C · Seeding the corpus so a rename is required by the work itself — the slug-collision seed

Commit, into the corpus before the session, a managed doc whose **slug** is what the worker's natural
title will derive, with a **different `# H1`**. The five-word slug cap makes this easy: *"Cap distinct
series at ten thousand"* and *"Cap distinct series at a configured ceiling"* both mint
`cap-distinct-series`. When the worker mints its own doc, the shipped title pre-check refuses
(`write.title-ignored` — *"a title that would be silently dropped is refused"*) and routes at
`jigc doc rename`.

| 1 forces | 2 survives | 3 clean | 4 mechanical | 5 all branches score |
|---|---|---|---|---|
| ◐ | ● | ● | ● | ○ |

**For.** No utterance, no pause, no timing; the plant is a committed file, verifiable by a corpus bar
before the session and after it. When it fires it fires on **the worker's own act**, with jigc's own
refusal carrying the route, and it reaches `write.title-ignored` on a **non-singleton** — strictly
better evidence than the rehearsal's `decisions-log`-only capture (T8), because the singleton arm
proves a doctype-level refusal while this one proves the instance-level one.

**Against, and it is disqualifying as the instrument of record.** The fire depends on the worker
choosing a title whose first five significant words collide. That is a bet on prose, and **a lost bet
is silent** — exactly criterion 5, exactly the cue card's fatal property in a new place. The trial's
own evidence says the bet is worse than it looks: across four sessions no two workers titled the same
concept the same way (`bound-distinct-series-count` vs `bound-the-number-of-distinct` for one identical
prompt intent).

**One reason to build it anyway, beyond coverage.** `title_ignored_refusal` (`crates/cli/src/doc.rs`)
handles the committed incumbent explicitly — *"is already committed and would be copied in for
update"* — and emits a **mechanical** route of the form
`jigc doc rename <address> --to '<title>' --task <id>`. On a **committed** doc that command is the one
`doc rename` refuses outright, routing on to the top-level `jigc rename`. If that reads at runtime the
way it reads in the source, it is a mechanical route that cannot run as given — the same class as
M48's own audit HIGH. **Stated as a hypothesis, not a finding:** it was read from the code, not driven,
and this project has been wrong three times this trial doing exactly that (session-findings.md →
Correction 3). Drive it before believing it.

**Use it as a free rider, never as the arm.** It costs one committed doc in any corpus that already
has managed docs, it cannot hurt anything, and it occasionally pays. Record it as opportunistic
coverage, and never let a coverage table cite it as the reason a surface was reachable.

**Falsified by:** measuring the collision rate. Take the four archived sessions' actual titles against
a candidate seed slug; if fewer than half collide, the rider is decoration.

### D · A second worker / live handover

Session 2 picks up session 1's work, with session 1's operator prompt as the only briefing.

| 1 forces | 2 survives | 3 clean | 4 mechanical | 5 all branches score |
|---|---|---|---|---|
| ◐ | ● | ◐ | ○ | ◐ |

**Against, as built.** It doubles session cost, and the second worker's prompt becomes the
contamination surface: any briefing precise enough to force an identity change is a briefing that
starts describing the state of a document, which is one clause away from naming a read. Worse, it is
not fireable mechanically — what session 2 inherits depends on where session 1 happened to stop,
which is the same uncontrolled variable that killed the cue card, moved up a level.

**The good half of it survives in E**, which takes the inherited state and makes it a *constructed*
artefact instead of a *sampled* one.

### E · The abandoned task — the recommended instrument

**The corpus ships with an open jigc task already staged**, authored by "someone who left": one
managed doc, created and slot-filled through the binary by the operator's setup script, whose
**title is wrong in a way the work itself settles** — it contradicts a committed spec, or the
vocabulary the rest of the corpus uses. The blind prompt is one ordinary sentence: *"someone started
this and left it half-finished — pick it up and land it."*

| 1 forces | 2 survives | 3 clean | 4 mechanical | 5 all branches score |
|---|---|---|---|---|
| ● | ● | ● | ● | ● |

**Why it forces the surface.** The doc is **staged, never committed, and bound to the task's
create-gate role** — the exact and only state in which `jigc doc rename` is the answer. Both natural
reactions land on M48's Increment 2, as cue-cards.md §0.4 already established and verified live: fix
the title by re-running `create`/`author` → `write.identity-change`, blocking, with the argv-complete
route to `doc rename`; or go straight to `doc rename` → the staged re-slug, slots and item anchors
surviving. The card's own analysis of *why the correction forces F2* transfers wholesale; only the
delivery changes.

**Why it survives a non-stopping worker.** It is true at t=0. There is no window. A worker that never
yields meets it in its first orientation call, because `jigc task list` / `jigc start` / `doc list
--task` all report it and the pack's own steps route through it.

**Why it is clean.** The operator says nothing beyond the opening prompt. The prompt names no verb,
no surface and no reading act — the contamination checklist (§0.2) passes it unchanged.

**Why it is mechanical.** It is a setup script that drives the real binary, exactly like
`instantiate.sh` and the existing plants, and it is checkable afterwards by a bar: task open, N docs
staged, title == the wrong one, tree clean. Follow the plants' own order of operations —
**gate the corpus first, plant second** — because this plant deliberately breaks `check-corpus.sh`'s
*"no `.jigc` residue"* bar, exactly as B1's two plants break five of its bars.

**And it restores the measurement the cue card was really after.** With the doc already staged and
authored by someone else, the worker **cannot know what it holds without reading it**. §3.2 retracted
the necessity claim for the cue card and was right to; here the necessity is real, and it converts
the headline question from *"does the worker read back?"* (answered, 4/4) into the sharper one:
**when a read is unavoidable, does it go through the CLI or around it?** That is a direct test of
VISION principle #3 — the adapter-not-sandbox bet — on the axis where this trial already found the
adapter *breaking*: B2 reset a worktree with raw git, B1 did commit surgery by hand, and both
sessions kept every **document** write inside jigc. Document reads under duress are the untested cell.

#### What would falsify it, and what each falsification costs

- **The worker discards the task and starts clean** (`jigc task discard`, then its own `create`).
  Plausible, and it skips the rename. *Mitigation:* make the staged prose obviously expensive — three
  full slots of good content, a criterion or two with ids — so discarding is the visibly worse move.
  *And note it still scores:* a worker that destroys authored prose rather than repair a title is a
  finding about `discard`'s framing, not a void. **This is the property C and the cue card lack.**
- **The worker keeps the wrong title.** Also plausible — jigc has **no probe for "this title is
  wrong"**, so the wrongness is human-judgeable only. *Mitigation:* put the naming authority in a
  **committed** document the task must conform to (a spec the ADR decides against, a `changelog`
  entry using the other word), so the contradiction is on the page rather than in the operator's
  head. *Residual risk, stated:* this is the one place the design still depends on the worker's
  judgement, and it must be pre-registered as an outcome — *"worker shipped a doc whose title
  contradicts the spec it cites"* is a finding, not a missing measurement.
- **The task will not resume.** B2-2 found `start --task <id>` refusing with *"task is pinned to base
  … but you're on …"* once the milestone's own bookkeeping commits moved HEAD. If the blind arc does
  other committing work before reaching the planted task, the resume may refuse. *Mitigation:* make
  the planted task the **first** thing the prompt asks for, and rehearse the resume on a corpus whose
  HEAD has moved — that rehearsal is itself worth doing, because if it refuses, that is a finding
  about a fresh-clone-continuable record that the team-ready design claims.
- **The plant does not survive the rig.** `.jigc/tasks/` is gitignored, so the planted task is loose
  state, not history. It travels with a directory copy and **dies with a `git clone`**. Verify the
  harness's provisioning path copies the tree, and add a bar that asserts the task is present
  *inside the container* before the prompt is pasted.
- **It cannot run on a greenfield corpus.** An open task presupposes an adopted repo, so B1's
  from-nothing shape cannot carry this plant. That is fine — F sits underneath it for exactly that
  session.

### F · The gated finalize as the injection point — the salvage, and the cheapest thing here

If a mid-session **utterance** is still wanted, stop scheduling it and **let a plant open the door**.
B1's docs-gate hook already did this: it rejected the ADR-promoting `finalize`, the worker stopped and
asked, and the operator answered into a window that stayed open for seventy minutes. At that moment
the ADR was **staged, uncommitted, and its task open** — precisely `doc rename`'s preconditions.

So the correction rides the answer:

> Signed off — though the wording should be 'Shed the oldest sample on ingest overflow'; that's what the runbook says. Go again.

| 1 forces | 2 survives | 3 clean | 4 mechanical | 5 all branches score |
|---|---|---|---|---|
| ● | ● | ● | ◐ | ● |

**For.** The pause is created by the product, not predicted by the schedule; the plant (`b1-hook.sh`)
and the release (`b1-release-hook.sh`) already exist and are rehearsed; the reply passes §0.2's
contamination checklist unchanged; and the state at the pause is exactly right. It costs one sentence
appended to an answer-key reply the operator was already going to give.

**Against, honestly.** It is still an utterance, so it inherits two weaknesses. The worker must
actually stop and ask rather than route around the hook — B1's did, and reasoned well about why it
must not create the sign-off marker itself, but a worker that self-serves the marker leaves no pause.
(That worker would be producing a finding of its own: an agent fabricating a human sign-off.) And the
operator must not improvise, which is a discipline, not a fence.

**Falsified by:** a rehearsal in which a live agent meets the rejecting hook and does **not** stop —
either creating the marker itself, retrying blindly, or abandoning the task. One rehearsal against a
live agent settles it. **That rehearsal is the thing this trial did not do.**

---

## 6 · Reaching `doc rename` and the identity surface specifically

The coverage hole is five surfaces: T7 `write.identity-change` at both minting verbs, T8
`write.title-ignored`, T9 the staged re-slug, T10 the committed-identity refusal, T11 the singleton
refusal — all currently **[R]**, carried by the operator's rehearsal on the host binary, plus F6–F9's
axis-iterating suites.

**The structural fact that determines everything here, stated once:**

> `jigc doc rename`'s subject is a **staged, never-committed** doc bound to an open task's role. A
> committed doc's rename is the top-level `jigc rename`. Therefore any instrument that hopes to reach
> `doc rename` must put the worker in possession of a staged doc whose title must change — and the
> only *deterministic* way to produce that state is to **hand the worker the doc already staged.**
> Everything else is a bet on the worker's title choice (C) or on the operator's timing (the cue
> card).

The cue card conflated the two homes and §0.3 recorded the consequence as a conversion rule
(*"if `finalize` lands first, the arm converts rather than voids"*) rather than as the design flaw it
was: the arm's subject changed depending on when it fired.

**The plan, in the order it should be built:**

1. **Plant E on the adopted corpora** (B2/B3 shapes). Reaches T9, and T7 when the worker's first
   reaction is to re-mint. Deterministic, no window.
2. **Plant F on the greenfield corpus** (B1's shape, where E cannot run). Reaches the same pair via
   the utterance, at a pause the hook creates.
3. **Rider C** on any corpus with a committed doc of the same doctype. Free; reaches T8 on a
   non-singleton when it hits; scored as opportunistic.
4. **The operator walk carries T10 and T11 explicitly, as a checklist with recorded verbatim output
   per arm.** §5 arm 4 already chartered a committed-identity `doc rename` and the walk record does
   not carry it — a narrative walk lost a chartered probe. Make each arm a line with a paste command
   and a captured output block, so an unrun arm is visibly blank rather than quietly absent.
   This is a **process fix, not a design fix**, and it is the cheapest item on this list.
5. **The destination-occupancy guard (F7) gets an arm on the walk too** — two `doc rename`s onto one
   identity, in both homes. It is three commands and it converts an [R]/test-fenced row into a
   walk-reached one.

Steps 4 and 5 alone would have prevented the coverage hole this post-mortem exists for. The blind
sessions were never the right instrument for a refusal that needs a specific wrong command typed on
purpose.

---

## 7 · What to measure now, given the read-back happens unprompted

**Stated plainly, because the brief asks for it: a designed read-back occasion is no longer needed for
the headline measurement.** Four sessions, four VERB results, 3 / 4 / 6 / 7 `doc show … --task` calls
each, on the fence's own verb, with the cue card never firing. §3.4's reading is 4/4 — with the
standing caveat it already carries, that this measures **compliance with a printed instruction**
rather than discovery, because M48's fence makes the composed step name the command verbatim.
Manufacturing a fifth occasion to observe the same behaviour a fifth time buys nothing.

**Three things a designed occasion still buys, and they are not the same thing:**

**7.1 · Channel under duress — the measurement that should become the headline.** Every read observed
so far was **optional**. A worker that reads through the CLI when it does not have to tells you
little about what it does when it *must* read, and the cheap path (`cat .jigc/tasks/<id>/docs/…`) is
one call away. Plant E creates that state without a word from the operator. Score the existing four
channels (VERB / VERB-ADJACENT / FILESYSTEM / NEITHER) unchanged — the instrument is already built
and already proven to discriminate by the control — but read the result against the **invariant**
rather than against the fence: this is a test of VISION principle #3, on the axis where this trial
already found the adapter breaking for the *workspace* (B1-3's commit surgery, B2-5's `reset --hard`)
while holding for *documents*.

**7.2 · Consequence, not occurrence — the measurement nobody has yet made.** A `doc show` in the log
scores VERB even if the worker ignored every byte of the output. That is a weak construct, and it is
the one the wave's claim rests on. It can be strengthened cheaply: plant, inside the staged doc, a
**discrepancy visible only in the read-back** — a `status` the worker did not set, a slot carrying
someone else's prose, an item id that does not match its heading — and score whether the worker's
**next logged command** addresses it. That converts a measurement of *invocation* into a measurement
of *use*, on the same two countable channels, with no new apparatus.
*Adversarially:* attribution is judgement-laden — a worker may notice and reasonably decline to act.
Mitigate by choosing a discrepancy with exactly one sanctioned repair (a `set-field`, a
`retitle-item`), so the follow-up is a specific argv rather than an inference, and pre-register both
"acted" and "noticed but declined, and said so in feedback" as distinct outcomes.

**7.3 · Coverage of surfaces nothing else reaches.** §6. This is the job the cue card was actually
indispensable for, and the one it failed at.

**What should be dropped:** the cue card as a *scored* instrument. Nothing should be scored on an
utterance whose delivery the operator cannot guarantee. Retain the answer key and the contamination
checklist for **replies** — a worker that asks a question must still get a clean answer, and the
operator log must still record it verbatim.

---

## 8 · The rule that generalises

For the next trial designer, applicable **before** the trial runs, on a whiteboard, without a rig:

> ### An instrument fires reliably iff its trigger is a **state**, its consequence is **re-raised by
> the product**, and **every worker behaviour maps to a scored outcome**.
>
> **1 · Trigger on state, never on a moment.** A state is true continuously until something acts on
> it; it can be established before the session, asserted by a bar, and re-checked afterwards. A
> moment must be caught. If firing requires an operator to notice something and act within a bounded
> window, **measure that window first** — and treat any window under a minute as zero, because the
> operator is also reading, judging and copy-pasting inside it.
>
> **2 · Make the product carry the consequence.** Land the instrument where a surface the worker
> already runs will keep restating it. `unadopted-instance` fired on every `validate` and `finalize`
> until it was dealt with; the worker complained about the repetition, and the complaint is the proof
> the instrument could not be missed. If the only thing that knows about your probe is your protocol
> document, you have one chance and no recovery.
>
> **3 · Enumerate the branches, and refuse a design with a silent one.** For every plausible worker
> behaviour — including *does nothing*, *discards*, *routes around* — write down what gets recorded.
> If any branch records **nothing**, the instrument can return no data, and a null is unreadable:
> §3.4 already has to distinguish *unmeasured* from *NEITHER*, and an instrument with a silent branch
> guarantees that ambiguity. **The instrument must have no "did not fire" branch, only outcomes.**
>
> **4 · Rehearse on the axis you are uncertain about.** cue-cards.md verified every ack, refusal, and
> derived slug against the live binary, and named its own untested assumption in writing — *"no card
> was rehearsed against a live agent"*. **The untested axis is the one that failed.** Verification
> effort spent where you already have confidence buys nothing; one rehearsal against a live agent
> would have cost one session and saved the arm. Corollary: **the assumption a design flags as its
> largest is the one that must be paid for before the trial, not carried into it.**
>
> **5 · Prefer instruments that need the operator to say nothing.** Every utterance is a
> contamination surface, a discipline requirement, and a scheduling dependency. If the state can be
> planted instead of spoken, plant it. Where an utterance is genuinely needed, **let a plant open the
> door for it** — the pause the tool creates is unbounded and free.

**The single-sentence version:** *plant a state the tool will keep telling the worker about; never
schedule a sentence.*

---

## 9 · What this post-mortem does not establish

- **The four sessions' turn boundaries were not reconstructed directly.** The window figures come
  from invocation timestamps, which bound the opportunity from above; the archived B1 transcript is a
  partial capture (it carries one operator-authored user message and neither logged intervention), so
  "the arc was one turn" is inferred from the absence of operator-visible stops, not read off a turn
  log. It does not change the conclusion — a 14-second window fails whether or not it is also
  turn-bounded — but it is not proof.
- **No replacement here has been rehearsed.** E, F and C are designs, each with its falsifier
  written down. Rule 4 applies to this document as much as to the one it replaces: **rehearse E
  against a live agent before a session is scored on it**, and rehearse F's stop specifically —
  whether an agent meeting a rejecting hook stops and asks. Both are one session each.
- **E's residual judgement dependency is real.** "The title is wrong" is not machine-checkable, and
  putting the naming authority in a committed document narrows the risk without closing it.
- **The 4/4 read-back result carries §3.4's own caveat unchanged** — instructed compliance at N=4,
  not discovery, and not reliability.

---

## The rule, promoted out of this trial (2026-08-18)

**An instrument fires reliably iff (a) its trigger is a *state*, not a moment; (b) its consequence is
**re-raised by the product**, not carried in an operator's sentence; and (c) every worker behaviour
maps to a scored outcome, so a miss is data rather than silence.**

Measured against it, this trial's instruments sort cleanly and in advance:

| instrument | trigger | consequence re-raised? | fired? |
|---|---|---|---|
| the foreign-ADR plant | a committed file, permanent | **yes** — every `validate`/`finalize` | **yes**, and caught on its harder branch |
| the carryover plant | a staged set at mint | yes — the gate re-checks | fired; defused by a tidy worker |
| the hook plant | a state in `.git/` | yes — every commit | **yes**, twice |
| the cue card | *a moment in a 3–5 s window* | **no** — only the operator knew | **no. 0 of 4** |

The corollary that costs the most to learn late: **rehearse on the axis you are uncertain about.**
`cue-cards.md` §0.3 named its own untested assumption in writing — *"reasoned from how Claude Code
delivers queued messages, not measured"* — and that is precisely the assumption that failed. Naming
an assumption is not testing it.
