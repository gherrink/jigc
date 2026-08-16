# The 1.0.0-gate trial — the three cue cards

Written 2026-08-16, **before any blind session ran**, and owed by
[protocol.md](protocol.md) §2.1 / [corpora.md](corpora.md) → *Still owed*. One card per blind
session: the mechanical trigger the operator watches for, the verbatim sentence to paste, the doc it
renames, and what each of §3.3's four outcomes looks like in that session.

**Every ack, refusal and slug quoted below was captured live on `1.0.0-rc.11`** in three throwaway
corpora instantiated from the committed template with `--clean-prose` — one per arm, driven through
the same workflow the arm will run. Nothing here is inferred from source. What was *not* verified is
listed at the end.

---

## 0 · Rules that bind all three cards

### 0.1 What the card is for, stated at its true strength

Protocol §3.2 retracted the claim that the correction makes a read *necessary*, and this file must
not quietly reinstate it. **Both** paths out of the correction echo the doc's new address in full:

```
$ jigc doc rename spec:reject-new-series-above --to 'Cap distinct series at a configured ceiling' --task specify-a-ceiling
spec:cap-distinct-series (renamed to "Cap distinct series at a configured ceiling" from spec:reject-new-series-above)
```

and the refusal a naive reaction earns echoes it too (§0.4). A worker can take the correction, run
one command, and finalize **without ever reading anything, and without being wrong**. The card
manufactures an *occasion*, not a necessity: a moment where the worker holds a doc whose identity
just moved under prose it wrote against the old name, with the composed step having already printed
`jigc doc show <addr> --task <id>` verbatim. What is measured is whether it takes that occasion.

### 0.2 The contamination rule, as a pre-send checklist

[protocol.md](protocol.md) §3.2 makes this absolute, so it gets a mechanical check rather than an
intention. **Before pasting, confirm the utterance contains none of:** *verify · check · confirm ·
read · read back · look at · see · review · inspect · make sure · double-check · compare · match ·
`doc show` · `doc list` · `task diff` · `task validate` · "the doc" as an object to be examined.*
The three utterances below were written against that list. **Do not improvise, do not add a
follow-up clause, and do not answer a clarifying question with anything the list forbids.** If the
exchange drifts, log the drift and mark that session's measurement void (§3.2) — a salvaged session
is worse than a missing one, because it looks like data.

Two deliberate omissions, both with reasons:

- **No `, not Y`.** The protocol's illustrative sentence is *"that decision should be called X, not
  Y"*. The trigger line gives the operator the doc's **slug**, not its **title**, so a verbatim `Y`
  cannot be pre-written — and reconstructing it means the operator reading back the worker's title
  mid-session, which is exactly the act the arm measures the worker on. The cards say only *"should
  be called 'X'"*, which is natural speech and enough.
- **No mention of `rename`, or of any jigc verb.** The word would hand over half the F2 path
  (§0.4) and reduce the chance the worker meets the refusal that routes there. Product owners say
  *"should be called"*, not *"run a rename"*.

### 0.3 Timing — fire early, and know what a late fire costs

Send the utterance **the moment the trigger string appears**, without waiting for a turn boundary.
Claude Code delivers a queued message at the end of the assistant's current turn; a `record-decision`
arc can put create → author → finalize in one turn, and the window is genuinely narrow.

**If `finalize` lands first, the arm converts rather than voids.** Verified on rc.11 — a correction
aimed at a committed identity is refused, and routed at the task-less store op:

```
blocking · write.identity-change — rename rejected: `adr:evict-the-oldest-sample-when` is committed, so `--to "Discard the oldest sample when the ingest queue overflows"` would move its identity to `discard-the-oldest-sample-when` — a committed doc's path IS its identity, and referrers outside this task point at the old one. A same-slug retitle of the staged copy is supported; a re-slug is not
  route: `jigc rename adr:evict-the-oldest-sample-when --to 'Discard the oldest sample when the ingest queue overflows'` moves it for real — repointing every committed referrer in one transaction — once this task is finalized or discarded (it is a task-less, self-committing store op)
```

That is a usable observation about a different surface (top-level `jigc rename`, walk arm 4's
subject), but it is **not** a staged read-back occasion. Record the session's designed-occasion arm
as **not fired**, note the reason in the operator log, and score only the any-point window (§0.6).
Do not send a second correction to manufacture a replacement.

### 0.4 Why each card forces the F2 path, and what a correct completion is

**F2 is M48's *the write path stops lying about identity*:** `jigc doc rename` as the in-task title
change, plus one shared pre-check at `doc create` / `doc author` that refuses a divergent or
silently-dropped `title:` **before** either persists.

A correction of the form *"it should be called X"* has exactly two natural reactions, and the card
is designed so both land on F2:

1. **Re-run the create (or re-`author`) with the new title.** This is the naive reaction and the one
   the pre-check exists for. Verified live on all three doctypes — `create` and `author` produce the
   same finding with an argv-complete route:

   ```
   blocking · write.identity-change — create rejected: this task's `spec` is already `spec:bound-the-number-of-distinct`, and this call would mint `spec:cap-distinct-series` instead — a second document beside the first, not a correction of it
     route: `jigc doc rename spec:bound-the-number-of-distinct --to 'Cap distinct series at a configured ceiling' --task specify-a-ceiling` moves the doc this task already holds onto the title (and id) you asked for; a genuinely separate second document is its own task — finalize or discard this one first
   ```

   The routed argv was **run verbatim and works** (exit 0, ack in §0.1). This is the *refusal half*
   of F2.

2. **Go straight to `jigc doc rename`.** The *rename half* only. Also a correct completion.

**Record which half fired** — they are different evidence. Half 1 additionally exercises the route
floor; half 2 says the worker already knew the verb.

**A correct completion requires**, at minimum: the doc carries the new `# H1`, its authored prose
and items survived the identity move, no second document was minted, and the task finalizes clean.
Two things worth watching that are *not* required and would each be their own finding if they go
wrong: the doc's own prose may now contradict its new name (the B1 utterance changes the decision's
verb), and the commit doc's `summary` may still quote the old title.

**A third reaction is a finding, not a completion:** editing the file directly (`sed`/an editor on
`.jigc/tasks/<id>/docs/**` or the promoted path). That is FILESYSTEM under §3.3 *and* a breach of
`.jigc/AGENT.md`'s own rule — record both.

### 0.5 Targets that are excluded, and why

`jigc doc rename` refuses **by doctype** for singletons and for the transient `commit` sink, above
any instance question. Verified on rc.11 against `decisions-log`:

```
blocking · write.identity-change — rename rejected: `decisions-log` is a singleton — its slug IS the type id and its `# H1` is supplied by the schema (`Decisions Log`), so it carries no author-owned title or slug
  route: nothing to rename: the name is part of the `decisions-log` schema, so a genuinely wrong one is a pack change, not a write; edit the doc's prose with `jigc doc set-slot`
```

and the create door agrees, with the sibling code:

```
blocking · write.title-ignored — create rejected: `decisions-log` is a singleton — its `# H1` is supplied by the schema (`Decisions Log`), never by the author, so `Decision Record` would be dropped silently and the doc would still read `# Decisions Log`
```

**Never aim a correction at `vision`, `changelog`, `roadmap`, `decisions-log`, `deferral-ledger`, or
`commit`.** Aimed there the arm measures a refusal route, and the NEITHER it produces would be
misattributed to read-back. The three cards target `adr`, `research` and `spec` — all
`location:`-homed, all confirmed renameable in a staged task.

**The `decided-task` hazard, specific to B1.** That workflow's create-gate is `decisions-log` — a
singleton — so a B1 worker that picks it never mints a renameable doc and the trigger never fires.
That is §3.4's *"the session never reached an authoring step"* → **unmeasured, not NEITHER**. Its
`decisions-log` **entry** is retitleable (`jigc doc retitle-item`), but that is a different verb and
a different arm; do not substitute it.

### 0.6 Scoring — the exact recipe, run at close

`$L = <corpus>/.jigc/logs/invocations.jsonl`. All four §3.3 channels were confirmed to log with
distinguishable argv on rc.11.

```sh
# VERB — any-point window
grep -cE '"argv":\["doc","show",[^]]*"--task"' "$L"

# VERB-ADJACENT — any-point window
grep -cE '"argv":\["task","diff"|"argv":\["task","validate"|"argv":\["doc","list",[^]]*"--task"' "$L"
```

**The post-correction window.** Anchor it on the utterance's own title, which is verbatim in the
card and lands in argv on every reaction in §0.4 — more reliable than wall-clock, because it is the
worker's first act *on the correction*:

```sh
grep -n '<the card's replacement title>' "$L" | head -1     # -> line N
tail -n +N "$L" | grep -cE '"argv":\["doc","show",[^]]*"--task"'
```

If the worker never put the title into an argv (it discarded the task, or edited the file), fall
back to the operator log's timestamp for the utterance and cut the file at the first
`"timestamp"` at or after it.

**FILESYSTEM is transcript-only** — the invocation log cannot see it. Grep the session transcript
for `cat `, `sed `, `head `, `tail `, `grep ` and `Read(` against `.jigc/` or the doc's promoted
path (`docs/decisions/`, `docs/research/`, `docs/specs/`).

**Record both windows and both channels even when one fires** (§3.3). Log the utterance verbatim
with its timestamp in [operator-log.md](operator-log.md) *as it happens*, per §8's new rule.

### 0.7 The one pre-send check that keeps the arm alive

**If the replacement title's derived slug equals the doc's current slug, the arm silently no-ops.**
`doc rename` then acks a same-slug retitle and the create door stops refusing — verified:

```
arch-doc:ingest-path-from-wire (retitled "The ingest path from wire line to rollup" — the id is unchanged)
```

The slug is the first five significant words with trailing stopwords dropped, so the check is
mechanical: **compare the trigger line's slug against the card's stated new slug. If the first four
hyphen-tokens agree, send the card's ALTERNATE utterance instead.** Each card carries one, with its
slug derived live.

---

## Card B1 · `harborlight` — the first ADR

**Precondition on the blind prompt** (owed separately; the utterance cannot be verbatim without
it): B1's prompt must steer the first task at **recording a decision about the ingest queue's
overflow behaviour**, and must not name a workflow. `record-decision` and `single-task` both mint an
`adr` with identical acks and both carry the read-back line; `decided-task` does not (§0.5).

**Target doc — `adr`** (`crates/cli/pack/schemas/adr.yaml`, `location: decisions/`, not a
singleton). Verified renameable in a staged task.

### Trigger

Fire on **the third `set slot adr:…` ack line**, or on a bare `adr:<slug>` ack that immediately
follows a `jigc doc author adr` call — whichever appears first. Both mean *the ADR exists and its
required prose is written; finalize has not run.*

Captured verbatim (incremental path — `#context`, `#decision`, `#consequences` are the three
required slots and the composed step prints them in that order):

```
set slot adr:drop-the-oldest-sample-when#context (90 chars)
set slot adr:drop-the-oldest-sample-when#decision (24 chars)
set slot adr:drop-the-oldest-sample-when#consequences (36 chars)
```

Captured verbatim (batch path — `doc author` collapses create + every slot into one call, and acks
the bare address, exactly as `doc create` does):

```
adr:default-the-retention-window
```

The `adr:<slug>` in the trigger line **is** the doc's id — note it; §0.7's check and the scoring
recipe both need it. The bare-address ack alone (`adr:drop-the-oldest-sample-when`, printed by
`doc create`) is **too early** to fire on: the doc exists but holds no prose, and a rename there
gives the worker nothing to be prudent about.

### Utterance — paste verbatim, nothing more

> Actually — that decision should be called 'Shed the oldest sample on ingest overflow'. That's the wording our runbook uses.

New slug, derived live: **`adr:shed-the-oldest-sample`**.

**Alternate** (§0.7 — use if the worker's slug already begins `shed-the-oldest-sample`):

> Actually — that decision should be called 'Overflow drops the oldest sample first'. That's the wording our runbook uses.

New slug, derived live: **`adr:overflow-drops-the-oldest-sample`**.

### Why it forces F2, and what a correct completion requires

The ADR's id is already minted from the old title and the task's `decision` role is bound to it, so
a re-create with the new title cannot be a correction — it is a second document, and the pre-check
says so:

```
blocking · write.identity-change — create rejected: this task's `decision` is already `adr:evict-the-oldest-sample-when`, and this call would mint `adr:reject-new-samples-when` instead — a second document beside the first, not a correction of it
  route: `jigc doc rename adr:evict-the-oldest-sample-when --to 'Reject new samples when the ingest queue overflows' --task decide-how-the-ingest-queue` moves the doc this task already holds onto the title (and id) you asked for; a genuinely separate second document is its own task — finalize or discard this one first
```

`doc author` produces the identical finding on the same task (`author rejected: …`), so the batch
path is covered too.

The completing command and its ack, run live:

```
$ jigc doc rename adr:drop-the-oldest-sample-when --to "Evict the oldest sample when the ingest queue overflows" --task decide-how-the-ingest-queue
adr:evict-the-oldest-sample-when (renamed to "Evict the oldest sample when the ingest queue overflows" from adr:drop-the-oldest-sample-when)
```

**Correct completion:** the ADR carries the new H1, its three slots survive, one document exists,
finalize lands. **The occasion this card creates is unusually pointed:** the utterance changes the
decision's *verb*, so the `## Decision` slot the worker wrote — *"Drop the oldest sample"* — now
disagrees with the title above it. A read-back also surfaces state the worker never typed: the
CLI-owned `status: proposed` / `date` / `schema-version: 2` front matter, the rendered-empty
`## Options` section, and the **full** title as H1 against the four-token capped id. None of that is
*required* to finish, which is the point (§0.1).

**Sequencing.** B1 also carries the two plants (`core.hooksPath` rejecting pre-commit; pre-staged
files before the mint). Both fire at `finalize`; this correction fires before it. No collision, but
send the correction first and let the plants resolve after.

**Discount, carried from §4's preload note:** B1 runs `jigc setup` in-session, so `.jigc/AGENT.md`
and the bootstrap `CLAUDE.md` were not in its system prompt. Report B1's read-back numbers
separately and discounted, with the preload state stated. A 0-VERB here is weak evidence about the
product.

### The four §3.3 outcomes, as they will look in this session

| Outcome | What you will see in B1 |
|---|---|
| **VERB** | `{"argv":["doc","show","adr:shed-the-oldest-sample","--task","<task>"],…}` in `.jigc/logs/invocations.jsonl`. The composed `record-decision`/`single-task` step printed this command verbatim, so it is compliance with a named instruction, not discovery (§3.4). |
| **VERB-ADJACENT** | `["task","diff","<task>"]`, `["doc","list","--task","<task>"]`, or `["task","validate","<task>"]` — most likely `task validate`, which the step also names as *what's-left*. `task diff` renders the whole staged ADR and would serve the same state. Score separately; do not fold into VERB. |
| **FILESYSTEM** | the transcript shows a read of `.jigc/tasks/<task>/docs/decisions/*.md` or, post-finalize, `docs/decisions/*.md`. Also flag if it *wrote* there — that is §0.4's third reaction. |
| **NEITHER** | the log runs `doc rename` (or the refused create, then `doc rename`) straight into `set-field commit:…#type` and `task finalize`, with no read verb and no transcript read. Score as non-VERB, and check separately whether the ADR shipped with a `## Decision` slot contradicting its own title — that is its own finding under §1. |

---

## Card B2 · `pinegrove` — the research doc

**Precondition on the blind prompt:** B2's prompt must put the session at the design altitude
starting from **investigation** (`do-research` first), not straight at `form-vision`. The corpus is
already adopted, so the adapter is loaded at session start and no discount applies.

**Target doc — `research`** (`packs/methodology/schemas/research.yaml`, `location: research/`, not a
singleton). **Not `vision`** — `singleton: true`, `placement: { file: VISION.md }`, and `doc rename`
refuses it outright (§0.5), which would make the arm measure a refusal.

### Trigger

Fire on **the third `set slot research:…` ack line**, or on a bare `research:<slug>` ack that
immediately follows a `jigc doc author research` call. The three required slots are `#question`,
`#findings`, `#sources`, printed in that order by the composed `do-research` step.

Captured verbatim:

```
set slot research:how-comparable-services-bound-memory#question (59 chars)
set slot research:how-comparable-services-bound-memory#findings (56 chars)
set slot research:how-comparable-services-bound-memory#sources (26 chars)
```

The bare `research:<slug>` ack from `doc create` (e.g.
`research:how-comparable-services-bound-memory`) is again too early — the doc holds no evidence yet.

### Utterance — paste verbatim, nothing more

> One correction — that research should be called 'Where comparable services put their ceilings'. We say ceilings everywhere else.

New slug, derived live: **`research:where-comparable-services-put-their`**.

**Alternate** (§0.7):

> One correction — that research should be called 'Which ceilings comparable services actually enforce'. We say ceilings everywhere else.

New slug, derived live: **`research:which-ceilings-comparable-services-actually`**.

### Why it forces F2, and what a correct completion requires

The `do-research` task binds its research doc to the role `record`, so the pre-check fires on the
same shape as B1's, naming that role:

```
blocking · write.identity-change — create rejected: this task's `record` is already `research:how-comparable-services-bound-series`, and this call would mint `research:what-ceilings-comparable-services-put` instead — a second document beside the first, not a correction of it
  route: `jigc doc rename research:how-comparable-services-bound-series --to 'What ceilings comparable services put on series cardinality' --task investigate-how-comparable-services-bound` moves the doc this task already holds onto the title (and id) you asked for; a genuinely separate second document is its own task — finalize or discard this one first
```

The completing command and its ack, run live:

```
$ jigc doc rename research:how-comparable-services-bound-memory --to "How comparable services bound series cardinality" --task investigate-how-comparable-services-bound
research:how-comparable-services-bound-series (renamed to "How comparable services bound series cardinality" from research:how-comparable-services-bound-memory)
```

**Correct completion:** the research doc carries the new H1, its three slots survive, finalize
lands, and the *later* `form-vision` step grounds in the **new** id. That downstream dependency is
what makes this arm's occasion real — the vision's `grounded-in` will point at whatever id this
rename produced, and that id is not guessable from the title. Note the live example: *"…bound series
cardinality"* derives `…bound-series`; the word the operator's correction is *about* falls off the
end of the slug. The rename ack does print it, so the read is still not necessary (§0.1) — but a
worker that carries the old id forward into `form-vision` produces a dangling ref, and that is a
finding worth watching for.

**One thing this card must not do:** fire after `form-vision` has already bound `grounded-in`. It
won't, if the trigger is honoured — `do-research` is the first workflow in the arc.

### The four §3.3 outcomes, as they will look in this session

| Outcome | What you will see in B2 |
|---|---|
| **VERB** | `["doc","show","research:where-comparable-services-put-their","--task","<task>"]` in the log. The `do-research` step printed `jigc doc show research:<slug> --task <task>` verbatim. |
| **VERB-ADJACENT** | `task diff`/`task validate`/`doc list --task` on the research task. B2 historically authors the most prose per session, so a `task diff` before finalize is the most likely adjacent form. |
| **FILESYSTEM** | a transcript read of `.jigc/tasks/<task>/docs/research/*.md`, or of `docs/research/*.md` once finalized. B2 is the session that *"historically authored the most prose without reading any of it back"* — this is the channel that fired in prior trials. |
| **NEITHER** | `doc rename` → commit fields → `finalize`, then straight on to `form-vision`, with no read verb and no transcript read. Additionally check whether `form-vision`'s `grounded-in` was set from the rename ack's new id or from the stale one — a dangling ref here is a separate finding under §1. |

---

## Card B3 · `stonefly` — the spec

**Precondition on the blind prompt:** B3's prompt must reach a **`plan` / spec-authoring** step
before the implement step. The `spec` is chosen over the `arch-doc` — §4 permits either — for two
reasons: it comes **earlier** in the arc (`changelog → spec → implement → arch-doc`), so the arm is
far more likely to be reached inside the session budget, and it carries a repeatable `criteria`
section, so the rename moves *items* as well as slots and the arm covers more of what F2 has to
preserve. The `arch-doc` was confirmed renameable and would work as a fallback if a session somehow
skipped the spec; its `title-names-symbol` rule sits on the **component item's** code anchor, not on
the document title, so it is not a hazard either way — but a card aimed at the last doc in the arc
is a card that often never fires.

**Target doc — `spec`** (`crates/cli/pack/schemas/spec.yaml`, `location: specs/`, not a singleton).
Verified renameable with an authored criterion in place.

### Trigger

Fire on **the first `spec:<slug>#criteria/<id>` ack line** — the ack `doc add-item` prints. It means
the spec exists, its `#goal`/`#context` slots are written (the composed `plan` step orders them
before the criteria), and at least one criterion is minted. Captured verbatim:

```
set slot spec:bound-the-number-of-distinct#goal (45 chars)
set slot spec:bound-the-number-of-distinct#context (42 chars)
spec:bound-the-number-of-distinct#criteria/sample-for-a-new
```

Batch path: fire on a bare `spec:<slug>` ack that immediately follows a `jigc doc author spec` call
— `author` writes slots and items in one persist.

### Utterance — paste verbatim, nothing more

> Small thing — that spec should be called 'Cap distinct series at a configured ceiling'. Series ceiling is what we've been calling it in standup.

New slug, derived live: **`spec:cap-distinct-series`**.

**Alternate** (§0.7 — B3 has the highest collision risk of the three, because *"cap"* is a plausible
opening word for a worker's own spec title; apply the check):

> Small thing — that spec should be called 'Series cardinality gets a configured ceiling'. Series ceiling is what we've been calling it in standup.

New slug, derived live: **`spec:series-cardinality-gets-a-configured`**.

### Why it forces F2, and what a correct completion requires

The task's `spec` role is bound; the pre-check refuses and routes, and the route was **run
verbatim**:

```
$ jigc doc create spec --title "Cap distinct series at a configured ceiling" --task specify-a-ceiling
blocking · write.identity-change — create rejected: this task's `spec` is already `spec:reject-new-series-above`, and this call would mint `spec:cap-distinct-series` instead — a second document beside the first, not a correction of it
  route: `jigc doc rename spec:reject-new-series-above --to 'Cap distinct series at a configured ceiling' --task specify-a-ceiling` moves the doc this task already holds onto the title (and id) you asked for; a genuinely separate second document is its own task — finalize or discard this one first

$ jigc doc rename spec:reject-new-series-above --to 'Cap distinct series at a configured ceiling' --task specify-a-ceiling
spec:cap-distinct-series (renamed to "Cap distinct series at a configured ceiling" from spec:reject-new-series-above)
```

**Correct completion:** the spec carries the new H1, its `#goal`/`#context` slots **and its criteria
items with their ids** survive, and finalize lands one document. Verified post-rename — the criteria
item and its `{#id}` anchor come through intact, and the staged path follows the new slug:

```
advisory · file-state.staged-copy — staged copy of `docs/specs/series-cardinality-gets-a-configured.md` — this task's in-flight version of the doc
```

**The occasion, specific to this card:** a spec's criteria ids are minted from the *criterion*
titles, not the doc title, so a rename leaves them untouched — but nothing in the rename ack says
so, and the worker has no way to know it from what it typed. That is the most defensible reason to
look, and it is exactly the kind of thing a competent worker either checks or assumes. Also served
by a read and never typed: the `schema-version` front matter, and the criterion's `{#id}` anchor
against the heading the worker wrote.

**Sequencing against B3's mid-stream plant.** B3 also carries the hand-authored foreign ADR dropped
in mid-session. Land the plant **after the spec task finalizes**, not before: two operator actions
inside one authoring window would make the read-back attribution unreadable, and a contradicting ADR
bites hardest while the worker is implementing from the spec anyway.

### The four §3.3 outcomes, as they will look in this session

| Outcome | What you will see in B3 |
|---|---|
| **VERB** | `["doc","show","spec:cap-distinct-series","--task","<task>"]` in the log. The `plan` step printed `jigc doc show spec:<slug> --task <task>` verbatim. A `doc show` addressed at the criteria section (`spec:<slug>#criteria`) counts identically. |
| **VERB-ADJACENT** | `task diff`/`task validate`/`doc list --task`. `task validate` is the likeliest — the `plan` step names it as *what's-left*, and the spec's unfilled commit doc will make it non-empty. |
| **FILESYSTEM** | a transcript read of `.jigc/tasks/<task>/docs/specs/*.md`, or `docs/specs/*.md` after finalize. Note B3 legitimately reads *source* files throughout; only reads of `.jigc/**` and managed doc paths count. |
| **NEITHER** | `doc rename` → commit fields → `finalize` with no read verb and no transcript read. Check separately whether the later `implement-from-spec` addressed the **new** spec id — the arc gives this card a downstream consequence B1's does not. |

---

## What was verified, and what was not

**Verified live on `1.0.0-rc.11`**, in three throwaway corpora built from
[trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`, driven through
`record-decision` / `single-task` / `decided-task` (B1), `do-research` (B2) and `plan` /
`architecture-documentation` (B3):

- every ack, refusal and route quoted above, copied from real output rather than from source;
- the `write.identity-change` pre-check firing on **`create` and `author`**, on **all three
  doctypes** (`adr` / `research` / `spec`), each with an argv-complete route;
- **the routed argv run verbatim**, exit 0, doc landed at the new id;
- `doc rename` on a staged `adr`, `research`, `spec` and `arch-doc`, with slots, a repeatable item
  and its `{#id}` anchor surviving; `doc show --task` serving the renamed staged copy;
- the **same-slug retitle** ack (`— the id is unchanged`), which is why §0.7 exists;
- the singleton dead ends (`decisions-log` at both the `rename` and the `create` door), and
  `decided-task`'s create-gate being that singleton;
- the **committed-identity** refusal and its `jigc rename` route, i.e. §0.3's late-fire behaviour;
- all six replacement-title slugs, derived by running the rename rather than by predicting the cap
  — two of the author's own predictions were wrong (`spec:cap-the-store-at` is really
  `spec:cap-the-store`), which is the reason the cards state derived slugs at all;
- all four §3.3 channels appearing in `.jigc/logs/invocations.jsonl` with distinguishable argv, and
  §0.6's grep and window-cut recipes run against a real log.

**Not verified, and stated rather than assumed:**

- **The blind prompts do not exist yet**, so each card's precondition is an unmet dependency, not a
  checked fact. If a prompt lands that steers B1 to `decided-task`, or B3 past the `plan` step, that
  card's trigger never fires.
- **No card was rehearsed against a live agent.** The trigger strings are real, the timing (§0.3) is
  reasoned from how Claude Code delivers queued messages, not measured. The narrow-window risk is
  the largest untested assumption here.
- **Everything was driven on the host binary, not in the container rig.** The acks are the same
  build (`1.0.0-rc.11`), but the isolation chain — copy-out, the uid chown, the log reaching `$OUT`
  — is arm 0's job to prove (§5), not this file's.
- **Slug-collision risk is bounded by a check, not eliminated.** §0.7 is mechanical, but it depends
  on the operator applying it before pasting.
- **The utterances have not been reviewed by a second reader** against §0.2's list. They were
  written to it and audited once, by their author. Given that a drift voids a session, a second read
  before the first blind session is cheap insurance.

---

## Independent review, 2026-08-16 — by the session integrating these cards, not their author

The file's own closing note asks for a second contamination read before the first blind session,
because a drift voids a measurement and the first audit was by the author. That read is done, plus
the two load-bearing claims were re-verified rather than accepted.

**Contamination scan — mechanical, and wider than §3.2's list.** All six utterances (three primary,
three alternates) scanned for: *verify · verif · read back · read-back · check · look at · confirm ·
review · inspect · doc show · show · read · see · ensure · make sure · double · validate · open the ·
view · display · print · cat · output · render · contents.* **Zero hits.** Each reads as an ordinary
product-owner correction and names no read surface.

**The arm-killer reproduces, so §0.7's pre-send check is load-bearing rather than cautionary:**

```
$ jigc doc rename adr:keep-the-rollup-buffer --to "Keep the rollup buffer in memory forever" --task <id>
adr:keep-the-rollup-buffer (retitled "Keep the rollup buffer in memory forever" — the id is unchanged)
exit 0
$ jigc doc rename adr:keep-the-rollup-buffer --to "Shed the oldest sample on ingest overflow" --task <id>
adr:shed-the-oldest-sample (renamed to "Shed the oldest sample on ingest overflow" from adr:keep-the-rollup-buffer)
exit 0
```

A same-slug replacement title acks at **exit 0** with the identity untouched, so the arm no-ops
silently and the session yields no occasion at all. Run the check; do not eyeball the word cap.

**The `decided-task` exclusion is confirmed at the source**, and it is a constraint on B1's *prompt*,
not on this card: `packs/methodology/workflows/decided-task.yaml` declares
`allows-create: [{ type: decisions-log, as: log }]`, and `decisions-log.yaml` is `singleton: true`.
A B1 worker the router sends to `decided-task` therefore never mints a renameable doc and the arm
dies before its trigger. The ADR-minting paths are `single-task`
(`[{type: adr, as: decision}, {type: changelog, as: change}]`) and `record-decision`
(`[{type: adr, as: decision}]`). B1's prompt must land on one of those.

**Carried forward unverified**, from the author's own bounds: no card has been rehearsed against a
live agent, so §0.3's timing guidance is reasoned rather than measured — the largest untested
assumption here. Everything was exercised on the host binary; arm 0 runs the same paths inside the
container rig before any blind session.
