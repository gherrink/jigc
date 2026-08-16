# The 1.0.0-gate trial — pre-registered protocol

**Binary under test: `1.0.0-rc.11`.** Written 2026-08-15, **before the trial runs**, by the session
that planned and built M48. **Run by a different session.** Everything needed to execute it is here;
nothing depends on the authoring session's context.

**What makes this trial different from the six before it: it has nowhere to send its findings.**
RC-greenfield through RC-pre-1.0 were acceptance instruments — they ran, they found things, and a
following wave absorbed them. M46 is explicitly *not* scheduled before the 1.0.0 call. So this
trial's findings either delay 1.0.0, ship recorded, or are judged not to matter — and **which
applies to which is pre-registered below, not decided after the result is known.**

---

## 1 · The pre-registered decision rule

**Fixed before the trial. A finding's class decides its consequence; severity alone does not.**
Recording this in advance is the whole point: without it the result gets adjudicated by how the
week went.

| Class | Consequence for the 1.0.0 call |
|---|---|
| **Data loss or corruption on any path** — bytes destroyed, unrecoverable, or a doc silently written wrong | **BLOCKS.** No exceptions, no "recorded as known". A wave absorbs it and 1.0.0 waits. |
| **A regression** — something that worked on rc.10 and does not on rc.11 | **BLOCKS**, unless it is the one **declared** behaviour change (`provision` refusing a non-empty leftover it cannot prove disposable) and the trial confirms the refusal reads as protection, judged against the standard fixed in §5 arm 1 rather than impressionistically. |
| **A seventh discoverability landing** (§3's measurement returns *filesystem* or *neither*) | **BLOCKS THE CLAIM, and forces a decision the trial cannot make.** Not a bug — evidence that the mechanism fix did not take. See §3.4. |
| **A blocking dead end** — a refusal whose route cannot run, or a state with no recorded recovery | **BLOCKS.** This is the class M48's own audit found in its centrepiece; it is cheap to fix and expensive to ship. |
| **A wrong result on a non-destructive path** — a check that does not fire, a false green, a wrong machine-readable value, a panic | **BLOCKS** if it produces a false green over managed state, or violates a pinned `--format json` contract. **SHIPS RECORDED** otherwise. |
| **A surface/wording finding** — a lie, an ambush, a missing route on a non-blocking path | **SHIPS RECORDED**, routed to M46, listed in the 1.0.0 record as a known bound — *unless it is a one-way door* (see below). |
| **A capability gap** — "I wanted a verb that does not exist" | **SHIPS RECORDED**, routed to M46. M48 refused seven of these deliberately; more are expected, and they are not defects. |

**The wrong-result row was added 2026-08-16**, on review, because this project's most common finding
shape had no home: M42's placement false green (*"the tool's own route produces a false green over an
unmigrated corpus"*) and the confidence audit's above-current stamp are neither data loss, nor
regressions, nor blocking dead ends — and calling them "surface findings" would have shipped them.
Adding it now, before the run, is the point; revising the table after a result is known is exactly
what pre-registration exists to prevent.

**The one-way-door qualifier.** A finding that ships recorded must also be *reversible after 1.0.0*.
A defect in a pinned `--format json` contract is not: M48 just closed the additive-key window and
took `doc schema` to contract-version 5. If a SHIPS-RECORDED finding would freeze a wrong contract,
it **BLOCKS** instead. Reversible prose and a frozen wrong shape are not the same disposition.

**Two rules that bind the adjudicator:**
- **A finding's class is decided from its evidence, before its consequence is looked up.** Deciding
  the class *after* seeing which column it lands in is the failure this table exists to prevent.
- **"Judged not to matter" is not a disposition.** Every confirmed finding lands in a row above, or
  the table was wrong and is revised **in writing, with a reason**, before the call.

---

## 2 · Instruments

**Three blind sessions + one operator-scripted walk** — the RC-pre-1.0 shape, kept so the
reachability result stays as comparable as it can be to the trial that produced M48's findings.

### Revised 2026-08-15, before the trial ran: every session runs filesystem-isolated

**This paragraph records a change to a pre-registered instrument, made before any session ran and
with its cost stated.** The original text said the comparison *is* the headline measurement and that
changing the instrument would forfeit it. That was written without knowing the environment was
already contaminated.

**The confound, verified rather than assumed.** Every jigc trial through RC-pre-1.0 ran as an
ordinary session on the operator's machine, which loads `~/.claude/CLAUDE.md`. Measured 2026-08-15:
from `/tmp`, with no project in sight, a host session answered **YES** to *"do your loaded
instructions mention a bash output filter, or a rule about asking the user only one question at a
time?"* — the operator's `PRINCIPLES.md` and `LACON.md`, in every worker's context. So every prior
result measured jigc **plus** those instructions, with nothing separating the two.

**Why it cannot simply be declared and left in place.** Its direction is unknown. *"Never ask what
you can find out yourself"* plausibly pushes a worker toward reading — but toward reading **files**,
which §3.3 scores as FILESYSTEM, the non-VERB outcome. A confound that could inflate *either* column
cannot be corrected for after the fact.

**The cost, stated plainly: the RC-pre-1.0 comparison becomes indicative, not direct.** That trade
is taken deliberately. The comparison was already weakened by six fresh corpora, a different binary,
and a designed-need correction no prior trial carried — and a clean absolute measurement is worth
more than a comparison with an uncontrolled variable inside it.

The mechanism, its five verification checks and its declared bounds live in
[harness/README.md](harness/README.md). A redirected config dir does **not** defeat this; this
project's own M17 pilot recorded that as insufficient.

**Invocation log ON in every session** (`jigc config set invocation-log true` in the worker prompt,
as every prior trial did). §3's measurement depends on it.

**Fresh corpora, one per session.** Never reuse a corpus across sessions — a second session on a
touched corpus is not blind.

### 2.1 The corpora — added 2026-08-16, because the protocol claimed self-sufficiency without them

RC-pre-1.0's protocol carried a corpora table, a naming rule, a pre-trial verification line and the
verbatim prompts. None of that was here, so *"everything needed to execute it is here"* was not true.

**Source.** All corpora are built from the committed
[trial-corpus-template](../trial-corpus-template/) — 9 TypeScript modules, 3 test files, 23 tests,
7 commits, zero managed docs — with `./instantiate.sh --clean-prose <dest> <product-name>`.

**`--clean-prose` is used, and here is the reason.** The template carries a known prose↔code
contradiction (README/`package.json`/`src/store.ts` call the service a cache *"in front of whatever
long-term store you already have"* while the router has no egress). It is an **accident**, not a
plant: two of three blind sessions found it independently last trial and the record had to spend
paragraphs disclaiming it (*"must not be reported as a designed trap"*), while it consumed two of the
six operator interventions. In a trial with nowhere to send findings, an accident that produces a
finding you must then bound is noise that looks like signal — and the contradiction-surfacing
capability is already probed **deliberately** by the foreign-ADR plant. Keep the designed probe, drop
the accidental one.

**Naming.** Blind corpora take product-plausible names with **no `rc`/`trial`/`probe`/`gate` token** —
a worker reads its own `cwd` and must learn nothing from it. Walk corpora may say what they are.

**Every corpus is gated by `check-corpus.sh` before it is frozen**, and the gate is not optional: it
asserts 7 commits, a clean tree, no jigc/adapter residue **including `.git/hooks/pre-commit`** (which
survives `git reset --hard` and `git clean -fdx`), no remote URL, an unset `core.hooksPath`, a
7-entry reflog, `README.md` as the only tracked `.md`, the suite green 23/23, and the `doc-code`
anchor symbols. For B2/B3 the naive bars are asserted **before** the operator runs `jigc setup` (see
§4's preload note).

**Owed before the run, and not yet written:** the three verbatim blind prompts, the three cue cards
(trigger string + verbatim correction), the plants with their rehearsals, and the answer key for the
forks a worker predictably raises. The prompts are themselves operator utterances and must be read
against §3.2's contamination rule before they are frozen — RC-pre-1.0's said *"do the whole thing
through the tool"*, and whether that phrasing is permitted here is a decision this protocol must
record rather than leave to the moment.

---

## 3 · The headline measurement: did F1 change what an agent does?

**The claim under test.** M48's centrepiece is a pack-load fence making an authoring step unable to
ship without naming `jigc doc show <addr> --task <id>`. **The wave proved the surface now names the
verb. It did not prove an agent then finds it** — that is what this trial exists to answer, and the
wave pre-registered the possibility that it does not.

### 3.1 Why the prior method is not enough here

Six trials measured this lens by observing agents `cat` their own staged work. That is a fine
**finder** and a weak **verifier**: at N=3, **absence of `cat` is not evidence of success** — a
session that never read its work back may have found the verb, or may simply never have needed to.
An open trial can easily produce no read-back occasion at all, and then the wave's central claim
goes untested.

### 3.2 The designed occasion — and the rule that keeps it clean

**Revised 2026-08-16 after rehearsal and an independent review. This section used to claim: *"Each
blind session carries one occasion where reading a doc back is **necessary to succeed**."* That
claim is false, and it is retracted here as well as in §5 arm 0, where it was first caught.**

The retitle path acks the new address in full:

```
$ jigc doc create adr --title "Drop the oldest sample when the ingest queue overflows" --task <id>
adr:drop-the-oldest-sample-when
$ jigc doc rename adr:drop-the-oldest-sample-when --to "Evict the oldest sample…" --task <id>
adr:evict-the-oldest-sample-when (renamed to "Evict…" from adr:drop-the-oldest-sample-when)
```

A competent worker takes the correction, runs one `rename`, and finalizes — **never reading, never
wrong**. Nothing in the arm's shape compels a read, and §3.1 already named that exact hazard.

**Why this matters more than a wording fix.** If the arm cannot manufacture necessity, then the
*expected* outcome for three competent workers is 0 VERB with the control firing — which §3.4 routes
to the seventh landing and §1 routes to BLOCKS THE CLAIM. The trial's designed failure mode would be
indistinguishable from its success condition being unreachable, and the escalation would land on the
human as a statement about the product's shape when it is a statement about the arm's.

**So the honest construction is: the correction creates an *occasion*, not a necessity.** It is a
natural human utterance that mentions no read surface, and it puts the worker at a moment where
reading back is the obviously prudent thing to do and the composed step has already named the verb.
What is measured is whether the worker takes that occasion. A NEITHER outcome is a worker declining
a prudent, named read — informative, but **not** evidence that a needed capability could not be
found. §3.4's rows are written to that weaker, true claim.

> **The contamination rule, absolute:** no operator utterance may contain *verify*, *read back*,
> *check the doc*, `doc show`, or any synonym. The moment the worker is told to verify, the thing
> being measured has been handed over. If an operator exchange drifts toward it, **log the drift and
> mark that session's measurement void** rather than salvaging it.

**The mechanism: a mid-task correction.** Partway through an authoring task, the operator changes a
name: *"actually, that decision should be called X, not Y."* Nothing more. This is a natural thing
for a human to say, mentions no read surface, and does three jobs at once — it forces the **F2**
path (the reject-and-retitle M48 built), it creates a genuine reason to look at the doc, and it
leaves the channel entirely to the worker.

### 3.3 What is recorded — two countable channels, no self-report

Per session, from the **invocation log** and the **session transcript**:

| Outcome | Evidence |
|---|---|
| **VERB** | `jigc doc show … --task …` appears in the invocation log |
| **VERB-ADJACENT** | `jigc task diff <id>`, `jigc doc list --task …`, or `jigc task validate <id>` — a staged read-back **through jigc**, by a verb the fence does not name |
| **FILESYSTEM** | the transcript shows a direct read of `.jigc/**` or a managed doc path (`cat`, `sed`, `head`, `grep`, an editor read) |
| **NEITHER** | the worker proceeds without reading, whether or not the outcome was correct |

**VERB-ADJACENT was added 2026-08-16, and its absence was a defect that could have inverted the
trial's conclusion.** `jigc task diff <id>` renders the whole staged doc — verified on rc.11, it
serves precisely the state arm 0 claims for `doc show`: the CLI-owned front matter, the rendered
empty optional `## Options`, the full H1 against the capped slug — and it is recorded in the same
log with the same argv shape. Under the old two-value scheme, **a worker reading its staged write
back through jigc, exactly as the architectural invariant demands, scored NEITHER** → seventh
landing → escalate VISION principle #3. An instrument that converts correct product usage into
evidence against the product's central bet is measuring the wrong thing.

**How the adjacent column reads against the claim:** it **satisfies the invariant** (the read went
through the CLI, not the filesystem) and **partially satisfies the fence's claim** (M48's fence
names `doc show` specifically, so another verb means the worker found its own way there). Score it
separately; do not fold it into either VERB or NEITHER.

**Both channels are recorded even when one fires** — a worker that uses the verb *and* also `cat`s
is a different result from one that only uses the verb, and the difference matters.

**Window.** Record two numbers per session: reads **at any point**, and reads **at/after the
correction**. The any-point number carries the fence's claim, because M48's fence makes the
*authoring* step name the verb — a worker that reads back at authoring time is doing exactly what
the fence asks. The post-correction number is a secondary observation about the occasion in §3.2.
An earlier version of this section scored only the post-correction window while the harness counted
the whole log and called it the same measurement; they are two measurements and both are reported.

**Do not rely on the worker's own account.** The sharpest datum in RC-pre-1.0 (*"I never read back a
single managed doc I wrote"*) was a confession, and the next worker may not be that candid. The log
and the transcript are the instrument; the feedback report is context.

### 3.4 The pre-registered reading — and the control that makes a null readable

**The positive control, run first, on the operator walk. Added 2026-08-15 with the isolation
change; its justification corrected 2026-08-16 after rehearsal.** Before any blind result is read,
one arm must demonstrate that the VERB channel **can** fire and **can be counted**: an
operator-driven task that authors a managed doc and reads it back with
`jigc doc show <addr> --task <id>`, leaving exactly that record in the invocation log.

**What the control does and does not prove.** It proves the channel records — nothing about
discoverability, because the operator already knows the verb. It deliberately no longer claims the
read is *unavoidable*: see arm 0 for the premise that rehearsal falsified.

**If the control does not fire, no blind result may be read at all.** A uniform null in both
directions is the signature of an apparatus that cannot discriminate, not of a finding — the failure
mode that voided a sibling project's entire results set three times, and the one this rig already
hit once (its first isolation probe returned UNKNOWN on the host *and* in the container). Fix the
instrument and re-run; do not report a landing.

Why it became necessary here: isolation removed the operator's global instructions from every
worker, including *"never ask what you can find out yourself."* Without a control, a moved number
cannot be attributed — it could be M48's fence failing, or it could be the removal of something that
was quietly propping the measurement up.

**What is being measured, named honestly (revised 2026-08-16).** The composed `record-decision` step
prints, verbatim and copy-pasteable, *"Read your write back before you move on — with `--task` the
read serves THIS task's staged copy"* followed by the exact command. So this trial measures
**compliance with a named instruction**, not discovery of an unnamed capability. That is a different
construct from the six prior landings, and a **worse** result if it fails: ignoring a printed,
copy-pasteable instruction is a stronger negative than failing to find something nobody mentioned.
Report it as *instructed read-back compliance*; keep the historical discoverability series as a
separate, explicitly non-comparable line.

Readings, over a session's **any-point** number (§3.3):

- **3/3 VERB** — the fence's mechanism took. The claim holds at N=3, which proves compliance, not
  reliability.
- **1–2 VERB** — partial. **Blocks the claim, not necessarily the release**: a *surface* finding
  under §1, routed to M46, with the wave's claim recorded as **unproven** rather than disproven.
- **VERB-ADJACENT instead of VERB** — the invariant held and the fence's specific verb did not.
  Scored with the partial row above, and reported explicitly: the worker read through jigc, by a
  verb no step named.
- **0 VERB and 0 VERB-ADJACENT, control fired** — the read-back landing. **Blocks the claim** and
  forces a decision the trial cannot make: whether the adapter-enforced ergonomics bet
  ([VISION.md](../../../VISION.md) principle #3) needs revisiting before 1.0.0. **Escalate to the
  human with the evidence; do not adjudicate it inside the trial.** Note the weakened basis from
  §3.2 — this is workers declining a prudent, named read, not failing to find a needed capability.
- **0 VERB, control did not fire** — **apparatus failure, not a result.** Nothing about the product
  may be concluded, in either direction. This row exists so that the cheapest possible explanation
  for the wave's own predicted outcome cannot be skipped on the way to the expensive one.

**Rows for the cases that are not outcomes at all** — added because without them each falls through
to NEITHER and is read as a product result:

- **A voided session** (§3.2's contamination rule fired) — excluded from the denominator, and the
  reading is taken at N=2 with that stated. Do not backfill it.
- **The invocation log was never enabled** — the primary channel is absent; that session is
  **unmeasured**, not NEITHER. `run-session.sh` prints this at close.
- **The session never reached an authoring step** — no occasion existed; unmeasured, not NEITHER.
- **VERB *and* FILESYSTEM** — counted as VERB for the claim, and flagged separately: a direct read
  of a managed doc is against `.jigc/AGENT.md`'s own rule even when the verb was also used.

**The N=3 knife edge, stated in advance.** One worker's behaviour separates *"routes to M46 and
ships"* from *"escalate and possibly revisit VISION principle #3"*. Any non-3/3 result is therefore
escalated **with its evidence** rather than adjudicated inside the trial, and 3/3 is recorded as
compliance at N=3 — never as reliability.

A **NEITHER** outcome is scored as a non-VERB for this reading **and** examined separately: if the
worker proceeded blind and shipped something wrong, that is its own finding under §1.

---

## 4 · The three blind sessions

Clean-room rules throughout: **verbatim prompts, no operator hints, per-session feedback report,
invocation log ON.** Each carries its designed-need correction (§3.2) at a point where a doc has
been authored and not yet finalized.

> **The preload problem, found on review 2026-08-16 — read before designing any arm.** The one
> surface that states the read rule in words is `.jigc/AGENT.md` (*"A doc staged in your open task is
> read with `jigc doc show <doc> --task <id>`, not from the file"*), reached through the bootstrap
> `CLAUDE.md`'s `@.jigc/AGENT.md`. **Both are written by `jigc setup`.** A session that runs `setup`
> itself assembled its system prompt before either file existed, so neither the adapter, nor the
> `SessionStart` hook, nor the installed skill can be in that session's context — and `jigc start`'s
> orientation does not carry the AGENT.md text as a fallback.
>
> Consequence: in an arm where setup runs in-session, a 0-VERB result is substantially attributable
> to **the adapter never having loaded**, which is a fact about the session's structure, not the
> product's shape. §3.4 escalates that outcome as if it were the latter.
>
> **So the arms split deliberately.** B1 keeps setup in-session — that is a real adopter moment and
> the bootstrap deserves testing — but its read-back measurement is reported **separately and
> discounted**, with the preload state stated. B2 and B3 begin on a corpus where the operator has
> already run `jigc setup` and committed it, so the adapter is loaded at session start. That is also
> the more common adopter condition: nobody authors their first ADR in the same session they
> installed the tool. Those corpora are checked with `check-corpus.sh` **before** setup, then
> prepared; the naive-state bars are asserted against the pre-setup snapshot.

**B1 · cold start.** A zero-doc repo: `jigc setup` → first task → first ADR → finalize. Carries the
correction at the ADR. Also the natural home for the two plants RC-pre-1.0 used and this trial keeps:
a **rejecting pre-commit hook under `core.hooksPath`**, and **pre-staged files before the mint** (the
carryover gate). **Read-back is measured here but discounted** per the preload note above.

**B2 · the design altitude from zero.** `do-research` → `form-vision` → roadmap → first milestone.
Carries the correction at **a research doc** — *not* the vision, which is a `singleton: true` doctype that `jigc doc rename` refuses outright (a dead end, `crates/cli/src/doc.rs`). Aimed there, the correction measures a refusal route and the NEITHER it produces would be misattributed to read-back. This is the session that historically
authored the most prose without reading any of it back.

**B3 · the corpus accretes.** First changelog entry · spec → implement-from-spec · first arch-doc ·
one hand-authored foreign doc dropped in mid-stream (detect-and-route). Carries the correction at the
spec or arch-doc.

**All three must reach the guides.** M48 ships them as an **adapter-owned artifact installed by
`jigc setup`** — which **drains RC-pre-1.0's process note 4** (which required seeding
`QUICKSTART.md`/`MIGRATING.md` into a sibling directory because setup shipped neither). **Do not
seed them.**

**Two corrections to how that is measured, both verified against the installed artifact on
2026-08-15:**

- **It is not a discovery measurement.** `setup` commits the guide to
  `.claude/skills/jigc/SKILL.md`, and the harness **auto-advertises** `.claude/skills/` in the
  worker's system prompt. The worker is *told* the skill exists. Record whether it is **used**, and
  do not report use as a worker having *found* anything.
- **The guide names no read surface at all.** Across its 250 lines there is not one occurrence of
  `doc show`, `doc list`, `doc schema`, `--task`, or any read-back instruction. So the guide
  **cannot** be the channel by which a worker learns the read-back verb — that channel is the pack
  steps (**17 methodology + 13 dev** steps declare `read.staged-read-back`; 31 step files name
  `doc show` — an earlier version of this line said "3 dev", understating the very channel the
  result is attributed to). A VERB result under §3.3 is
  therefore attributable to the M48 fence, not to the guide; state it that way in the record.

---

## 5 · The operator-scripted walk

**How the arms are actually driven under isolation (added 2026-08-16 — the walk previously had no
harness path at all).** `run-session.sh` grew the modes these arms need:

- **`--shell`** gives a plain `bash -l` in the image instead of a Claude session — the operator walk
  is a sequence of CLI commands, not an agent session, and there was no way to get one.
- **The container id is printed at start**, so a **mid-stream plant** (§4's foreign ADR) can be
  landed while a blind session is live: `docker exec -it <cid> bash -l`, then commit with an explicit
  pathspec per §8 rule 3 and back-dated env vars per rule 1.
- **`--bypass-permissions` is opt-in.** It is appropriate here and **not** in a blind session: it
  removes prompt friction from file reads while `jigc setup` allowlists `Bash(jigc:*)`, i.e. from
  exactly one side of the asymmetry §3 measures.
- **Arm 2's two-binary sequence** is: `run-session.sh --shell <corpus> <out-a> jigc-gate:rc10`, then
  feed `<out-a>` as the corpus of a second run against `jigc-gate:rc11`. One image carries one
  binary; the corpus moves between them.
- **The control (arm 0) runs in the rig, not on the host.** Its 2026-08-16 rehearsal was a host run,
  which cannot validate the containerised chain the control exists to prove — copy-out, the uid
  chown, whether the worker's `invocation-log` setting survived, whether
  `.jigc/logs/invocations.jsonl` reaches `$OUT`. A host-only control would pass while an apparatus
  failure in the container was read as a landing.

Not a blind session, and the reason is evidence rather than preference: its value is entirely
operator-placed plants and destructive sequences **a blind agent cannot be relied on to reach**.
Every arm below is justified by a behaviour M48 changed that no blind probe reaches.

0. **The positive control — run FIRST, before any blind session is read** (§3.4). Numbered 0 so the
   arms below keep the numbers they are referenced by. Its only job is to prove the VERB channel can
   fire at all; it measures nothing about discoverability, because the operator already knows the
   verb.
   **Rehearsed on the host 2026-08-16, and the rehearsal corrected this arm.** The original shape
   claimed reading was *unavoidable* because the doc's **derived** slug — decided by the word cap
   and edge-stopword drop — was never written down by the author. **That is false, and the binary
   says so:** `jigc doc create adr --title "Keep the rollup buffer in memory and drop the oldest
   sample when it overflows"` acks `adr:keep-the-rollup-buffer`. The ack *is* the id, written down.
   Recorded rather than quietly rewritten, because a premise falsified before the run is cheap and
   the same premise discovered inside a result is not.

   **The shape that survives** — and it is enough, because the control's job is the channel, not
   discoverability: mint a task, author a managed doc through the write verbs, then read it back
   with `jigc doc show <addr> --task <id>`. What the read returns genuinely is *not* knowable from
   what was typed — the CLI owns the front matter (`status`, `date`, `schema-version`), renders the
   empty optional `## Options` section, and keeps the full title as the H1 while the slug is capped
   — so the arm still demonstrates a read serving state the author never wrote. It simply no longer
   pretends the author *had* to run it.

   **Verified in that rehearsal, on `1.0.0-rc.11`:** the read served the staged copy (slots written,
   no commit carrying them), and the invocation log recorded
   `["doc","show","adr:…","--task","…"]` at `exit_code: 0` — so §3.3's primary channel both fires
   and is greppable. Also observed, and it sharpens attribution: the composed `record-decision` step
   **instructs the read in so many words** — *"Read your write back before you move on — with
   `--task` the read serves THIS task's staged copy"* — which is M48's fence live in this binary.
   A VERB outcome therefore measures **acting on a named instruction**, not finding an unnamed
   capability; a NEITHER outcome means the instruction was there and went unfollowed.

   Pass condition: **at least one `jigc doc show … --task …` in that corpus's invocation log.**
   Fail: the instrument cannot discriminate — fix it and re-run before reading any blind session.
1. **The declared breaking change.** Plant a **non-registered leftover holding staged, unstaged and
   untracked work**, then drive `milestone provision`. Expect a refusal naming the path and the
   bytes, with `--force` as the hatch. **Then drive the same state at `discard` and `uninstall`.**
   The judgment to record is not whether it refuses — tests already fence that — but **whether the
   refusal reads as protection or as obstruction**, since this is the one deliberate regression an
   adopter meets.

   **The standard, fixed here rather than judged in the moment** (§1 row 2 defers to it, and the
   person running this arm is the person who read the code that produces the refusal — so it needs a
   written bar). The refusal *reads as protection* only if all four hold: (a) it **names the path**
   it is protecting; (b) it **says what would be lost** — staged, unstaged, untracked — rather than
   just "not empty"; (c) it **names the consent** (`--force`) in the same output; and (d) the consent,
   run verbatim, **works**. Any of the four missing, it reads as obstruction and is a finding. Also drive the **junk-directory** case, which now refuses where rc.10 succeeded.
2. **The rc.10 → rc.11 upgrade.** ⚠️ **Build rc.10 from `8979f16` — pin that sha, do not search for
   the version string.** **Two commits stamp `version = "1.0.0-rc.10"`**: `8979f16`, the genuine
   pre-M48 binary the last trial ran on, and `4fd7fbc`, which is still rc.10-stamped but **contains
   every M48 change including the audit fixes** (the bump landed late, at `9cb9b78`, deliberately
   *after* the audit). A search for the version finds the **wrong, newer** one, and building it makes
   this arm compare rc.11 against itself — **vacuous, with nothing in the output to reveal it**.
   **Since the isolation change (§2), build it with `harness/build-image.sh 8979f16`** rather than
   to a temp prefix: the sha is an argument, so the wrong-tree hazard above is unreachable rather
   than merely warned about, and this arm then runs in the same isolation as every other. Confirm
   the pair with `harness/verify-pair.sh` — it fails unless rc.10 **lacks** three verbs M48 shipped.
   Then: author ~5 managed docs of mixed doctypes plus a milestone record, commit; **then switch to
   rc.11** and continue (`validate` · `doc show` · a task → finalize · a `rename` ·
   `migrate-corpus`). **This is the real 1.0.0 upgrade path** and it is covered by nothing today —
   the fixture builder constructs every state by driving the *current* binary. Expect **`setup`** to
   install the guide artifact onto a corpus that predates it — **not `upgrade`**, see arm 3.
3. **The guide artifact's clobber refusal.** On that same upgraded corpus, **edit the installed
   guide**, then re-run **`jigc setup`**. Expect a refusal-to-clobber with a route, not a silent
   overwrite and not a blocked setup. Then `uninstall` and confirm the artifact is taken back out.
   ⚠️ **Not `jigc upgrade` — it does not touch the guide.** Verified 2026-08-15: with `SKILL.md`
   deleted, `upgrade` exits 0 reporting only that the recorded config deltas re-apply clean, and
   restores nothing; `setup` restores it. `upgrade` re-checks **config deltas** and is report-only
   by its own help (*"it changes nothing"*). An executor who drives `upgrade` here sees no refusal
   and would record a false finding — which is why the earlier `setup`/`upgrade` wording is gone
   from both this arm and arm 2.
4. **The verbs a blind session may never reach:** `config get` / `config list`, `describe
   --workflows`, `doc rename` on a **committed** identity (expect the refusal, not exit 0), and a
   read-shaped near-miss (`jigc config get` typo'd, `jigc doc read`) to confirm the tip names a read
   verb and clap's contradictory suggestion is gone.

---

## 6 · Coverage — every changed surface in exactly one column

M47's rule, applied to M48's diff. Each changed surface lands in **exactly one** of
**trial-reached** · **test-fenced (naming the suite)** · **neither** — and the *neither* column must
be empty or each member explained. The first version of this rule used a two-way split with no way
to say *"fenced by a test, not by the trial"*, which inflated the probe count and left ~27% of
changed lines unexamined.

**Stated up front, so no arm is invented for it: the manifest-hash fence is NOT trial-testable.** It
fires in CI over a pushed range and carries no verb, finding or route. It is **test-fenced**
(`manifest_freeze_fence`) plus a live CI arm, and that is its honest disposition.

---

## 7 · The conversion obligation

**Every CONFIRMED *and* REFUTED verdict arrives with a repro block** ([pinning.md](../../../implementation/pinning.md) §3).
A refuted block's obligation is that the refuting fact **has** a standing test — not that a duplicate
is minted. Cite the existing suite as `pinned-by: <suite>::<test>` per
[pinning.md](../../../implementation/pinning.md) §3, and **verify it by reading what that test
asserts, never by the name looking apt**. (An earlier wording — *"never by its name"* — read as a ban
on the citation format the ledger requires, which would have produced rows that cannot close it.) A REFUTED finding is
expected and valuable: five of the last six trials produced headline complaints that dissolved into
shipped capability, and that pattern is itself the discoverability signal.

---

## 8 · Operational rules carried forward

From RC-pre-1.0 §"Process changes for the next trial", each earned:

1. **Back-date a plant's commit, not just its body** (`GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE`) — a
   worker that notices a back-dated file can infer it is being tested.
2. **Keep a contemporaneous `operator-log.md`** — every operator utterance into a blind session,
   logged verbatim *as it happens*, with its justification. It is what lets the record's purity
   statement be written from notes rather than recall, and it is **required** here because §3.2's
   contamination rule can only be audited against it.
3. **Commit an operator plant with an explicit pathspec** (`git commit -m … -- <path>`) so a
   worker's in-flight index can never be swept into the operator's commit.
4. ~~Stage the shipped guides outside the corpus~~ — **DRAINED by M48**: setup now installs them.
   Do not seed; measure instead (§4).

**New for this trial:** log the **designed-need correction** verbatim in the operator log with its
timestamp, and record the two channels (§3.3) immediately at that point — not reconstructed later.

---

## 9 · What this trial cannot answer

Stated so the 1.0.0 record does not imply coverage that was never bought:

- **Adopter-side freeze protection.** The manifest fence guards *this repo*, in CI. An adopter
  editing a manifest has no equivalent. Declared bound, not a trial gap.
- **Whether the read-back fence generalises beyond `doc show`.** The fence proves one verb is named
  by the steps that solicit writes. It says nothing about the next capability nobody finds.
- **Multi-process concurrency semantics.** Lost updates remain deferred (M46 entry 9); a single
  operator running one session at a time will not exercise them.
- **Most regressions.** §1 makes a regression BLOCK, but only walk arm 2 touches rc.10 at all;
  blind sessions carry no baseline, and §2 already reduced the RC-pre-1.0 comparison to indicative.
  So *"worked on rc.10, does not on rc.11"* has a real detection path **only** on that one arm. The
  regression row is retained because its consequence matters if one is found, not because this trial
  searches for them systematically.
- **Long-horizon drift.** Every trial to date is a short arc. The salience-independent enforcement
  claim rests on the long-horizon study, not on this.
- **A direct comparison against RC-pre-1.0's reachability result.** Isolation (§2) removed a
  variable that was present in all six prior trials, so a difference in the numbers cannot be
  attributed to M48's fence alone. The comparison is reported as **indicative**, and the absolute
  measurement — read against the §3.4 control — is what carries weight.
- **Whether the isolated environment is representative of an adopter's.** A real adopter has their
  own global instructions, plugins and skills; the container has none. This trial measures jigc
  without that layer, which is the right instrument for *attributing* a result and the wrong one for
  predicting an adopter's day.
