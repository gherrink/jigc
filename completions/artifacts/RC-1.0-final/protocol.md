# The 1.0.0 trial — pre-registered protocol

**Binary under test: `1.0.0-rc.12`**, built from `314f59e`. Written 2026-08-28, **before the
trial runs**. Everything needed to execute it is here, or is named as owed in §10.

**What makes this trial different from the eight before it: its designed instrument is new.**
The 1.0.0-gate trial's cue card returned no data in **four sessions of four** — its injection
window measured 11–19 seconds, and §3.4 of that trial had to report a whole increment as
reached by nothing. The design of record for the replacement is
[cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md), and its rule governs every
instrument below:

> An instrument fires reliably iff its trigger is a **state**, its consequence is **re-raised
> by the product**, and **every worker behaviour maps to a scored outcome**.

The consequence, stated once and applied throughout: **plant a state the tool will keep telling
the worker about; never schedule a sentence.**

This trial's findings either delay 1.0.0, ship recorded, or are judged not to matter — and
**which applies to which is pre-registered in §1, not decided after the result is known.**

---

## 0 · Two declared behaviour changes, briefed before the first session

**A blind session meets both of these in its first minutes, on every adoption arm.** An
unbriefed observer scores each as a regression and the trial then spends its budget
re-deriving decisions this repo already took. Both are recorded with triggers in
[decisions-pending.md](../../../implementation/decisions-pending.md) → *The trial that follows
M46 — protocol inputs*.

**0.1 · `jigc validate` exits non-zero once un-adopted files sit at managed homes.** Through
rc.11, a stock brownfield repo — a Keep-a-Changelog `CHANGELOG.md`, a hand-written
`VISION.md`, MADR `docs/decisions/*.md`, plus `jigc setup` — validated at **exit 0** with one
adoption advisory per file. It now exits non-zero: `schema-conformance.unadopted-instance` is
`render::STORE_EXIT_FLIPS`' fifth member. **The finding itself is untouched** — advisory,
`GATES_NOWHERE`, routed at `jigc ingest` — so no task gate, no `finalize` and no pre-commit
hook starts blocking. What changed is the *sweep's verdict on its own run*.

Two consequences this protocol designs for rather than discovers:

- **An adoption arm cannot use `jigc validate`'s exit code as its "clean start" signal.** Read
  the findings, not the code.
- **A session that adopts its foreign docs should see the exit go to 0 as the last of them
  lands** — a positive-control-shaped check the protocol gets for free, and walk arm 04 takes
  it deliberately.

**0.2 · `jigc task validate` no longer reports a conformant task clean.** Through rc.11 a
`single-task` that wrote no changelog entry validated with *"no findings — the task validates
clean"*. It now draws `changelog-recording.gate-granted-unused`: the advisory's predicate moved
from an item count to a **staged write-touch** against the un-authored baseline, and it joined
`preview_gates`, so it fires at the door where its in-task route is still live. **Exit stays 0
at the default severity**; a project that promotes the key to `blocking` gets `jigc task
validate` **exit 3** on the same state `finalize` refuses.

Verified two ways before this was written: `crates/cli/tests/task_lifecycle.rs`'s
`NO_DELTA_CLEAN_VALIDATE_GOLDEN` *is* the advisory block, and M46 Increment 6 declares it as
bound (4) verbatim — *"The advisory now rides every gate-granting task's `task validate` until
the changelog is touched … It is intended."*

**This bullet was missing from the ledger when this protocol was drafted, and was added then.**
The entry whose whole job is to be complete before a briefing was found one short at the moment
it was consulted — recorded here because the near-miss is the useful part: **one declared
change briefed and one met blind is not "mostly briefed", it is a scored regression on the arm
that meets the unbriefed one.**

---

## 1 · The pre-registered decision rule

**Fixed before the trial. A finding's class decides its consequence; severity alone does not.**
Carried from [the 1.0.0-gate protocol](../RC-1.0-gate/protocol.md) §1, which earned each row,
with the §0 changes named in row 2.

| Class | Consequence for the 1.0.0 call |
|---|---|
| **Data loss or corruption on any path** — bytes destroyed, unrecoverable, or a doc silently written wrong | **BLOCKS.** No exceptions, no "recorded as known". A wave absorbs it and 1.0.0 waits. |
| **A regression** — something that worked on rc.11 and does not on rc.12 | **BLOCKS**, unless it is one of §0's **two declared** behaviour changes and the trial confirms it reads as designed, judged against the standard fixed in §5 arm 04 rather than impressionistically. |
| **A discoverability landing** (§3's measurement returns *filesystem* or *neither* under duress) | **BLOCKS THE CLAIM, and forces a decision the trial cannot make.** Not a bug — evidence about VISION principle #3. See §3.5. |
| **A blocking dead end** — a refusal whose route cannot run, or a state with no recorded recovery | **BLOCKS** when it can reach a **project-carrying** file — a repo-root managed doc, a decision record, or anything whose loss or misdirection costs project knowledge. **SHIPS RECORDED** when the reachable set is incidental and the state has a correct recovery elsewhere. |
| **A wrong result on a non-destructive path** — a check that does not fire, a false green, a wrong machine-readable value, a panic | **BLOCKS** if it produces a false green over managed state, or violates a pinned `--format json` contract. **SHIPS RECORDED** otherwise. |
| **A surface/wording finding** — a lie, an ambush, a missing route on a non-blocking path | **SHIPS RECORDED**, listed in the 1.0.0 record as a known bound — *unless it is a one-way door.* |
| **A capability gap** — "I wanted a verb that does not exist" | **SHIPS RECORDED.** M46's razor refused nine of these with citations; more are expected, and they are not defects. |

**The blast-radius qualifier on the dead-end row.** What a bad route costs is what the file
carries, so the row is keyed on the **reachable set**. Two guards keep it from becoming
discretion: (a) the reachable set is determined by **what the defect can reach, demonstrated**,
not by which file the reporter happened to hit; (b) the class is fixed from evidence **before**
its consequence is looked up.

**The one-way-door qualifier.** A finding that ships recorded must also be *reversible after
1.0.0*. A defect in a pinned `--format json` contract is not — M48 closed the additive-key
window and M46 added `unadopted[]` and `unfilled[]` under it. If a SHIPS-RECORDED finding would
freeze a wrong contract, it **BLOCKS** instead.

**Two rules that bind the adjudicator:**
- **A finding's class is decided from its evidence, before its consequence is looked up.**
- **"Judged not to matter" is not a disposition.** Every confirmed finding lands in a row
  above, or the table was wrong and is revised **in writing, with a reason**, before the call.

---

## 2 · Instruments

**Three scored blind sessions + one operator-scripted walk + one unscored adopter-condition
arm.** The RC-pre-1.0 / 1.0.0-gate shape, kept so the reachability result stays as comparable
as it can be.

**Every session runs filesystem-isolated**, per [trial-harness/README.md](../../trial-harness/README.md).
The confound that forced this was *measured, not assumed*: a host session answers YES to the
discriminating probe (the operator's `PRINCIPLES.md` and `LACON.md` are in every worker's
context) and a container answers NO. `verify-image.sh` runs **seven assertions in six numbered
sections** and check 4 is that flip; the image is gated with `run.py record-gate` before a
round, and `run.py gate` refuses a round the record does not cover.

**Invocation log ON in every session.** §3 depends on it. It is pre-enabled on the adopted
corpora and named in B1's prompt only, because B1 is the session that installs jigc.

**Fresh corpora, one per session.** Never reuse a corpus across sessions — a second session on
a touched corpus is not blind.

### 2.1 · Transport is chosen per arm, by whether the arm needs a human channel

**This is a decision, and here is its basis.** `trial-driver` increment 0 measured a headless
`claude -p` arm at **6** VERB against the archive's interactive **6 and 7**, same image, same
prompt: **the channel does not invert.** That licenses *"the channel survives the transport"*
and nothing quantitative — it is n=1, one corpus shape, one day. So headless is used where an
arm needs no human, and is not treated as a substitute where an arm does.

| arm | corpus | plants | transport | why this transport |
|---|---|---|---|---|
| **B1** cold start | greenfield | hook (**F**) + carryover | **interactive** | Plant F's whole design is *let a plant open the door for an utterance*. `-p` has no queued-message channel, so headless has no door. |
| **B2** design altitude | adopted | **E** + rider C | **interactive** | E carries the headline; the transport does not move under the headline. |
| **B3** corpus accretes | adopted | **E** + foreign-ADR | **headless**, default permissions | Plant-driven; needs no utterance. The archive holds **two interactive rc.11 runs of this exact prompt**, so it is the most comparable headless arm available. |
| **B3-strict** | a second copy of B3's corpus | same | **headless**, `--strict-permissions` | The adopter's real condition, which no trial has ever run. **Unscored and labelled so** — see below. |

That gives the headline **N=2 across two transports**, which is a check on the new measurement
rather than a bet on increment 0's n=1 on the old one.

**Why B3-strict is unscored, stated in advance.** Increment 0 drove it and it **truncated the
arc**: 13 records, 0 authoring writes, `unmeasured`. All 13 jigc invocations executed and all 7
denials were file writes — `jigc setup`'s `Bash(jigc:*)` allowlist is honoured, so the arm
reproduces the asymmetry faithfully — but a worker denied a payload-file write stops and asks,
and headless has nobody to ask. **That is a result about the adopter's condition, scored
`unmeasured`, never `NEITHER`**, and it is kept out of the headline denominator.

### 2.2 · The corpora

**Source.** All corpora are built from the committed
[trial-corpus-template](../../trial-corpus-template/) — 9 TypeScript modules, 3 test files,
23 tests, 7 commits, zero managed docs — with `./instantiate.sh --clean-prose <dest> <product>`.

**`--clean-prose` is used**, for the reason the last trial recorded: the template's
prose↔code contradiction is an *accident*, two of three blind sessions found it independently,
and in a trial whose findings have somewhere consequential to go, an accident that produces a
finding you must then bound is noise that looks like signal. The contradiction-surfacing
capability is probed **deliberately** by the foreign-ADR plant.

**Naming.** Blind corpora take product-plausible names with **no `rc`/`trial`/`probe`/`gate`
token** — a worker reads its own `package.json` and must learn nothing from it. Walk corpora
may say what they are.

**Every corpus is gated by `check-corpus.sh` before it is frozen, and the gate is not
optional.** It asserts **11 bars**: 7 commits · clean tree · no jigc/adapter residue
**including a non-sample file in git's hooks dir** · no remote URL · `core.hooksPath` unset ·
a 7-entry reflog · on `main` · `README.md` the only tracked `.md` · the suite green 23/23 ·
the `doc-code` anchor symbols · and no surviving forwarding-shaped claim. `self-test.sh` runs
12 mutations proving each bar can fail and that the *named* bar is the one that reports it.

**Order is fixed: instantiate → gate → adopt → plant.** A planted corpus deliberately breaks
gate bars; gating after planting proves nothing and re-gating a planted corpus is a category
error. Adoption is driven by **the container's own binary** through
`run-session.sh --exec arms/adopt.sh`, so the state under test is produced by the exact build
the sessions run: 7 template commits + setup's install commit + the adopt commit = **9**, clean
tree.

### 2.3 · The rehearsals are a precondition, not a nicety

Postmortem rule 4, and the reason the cue card died: *the assumption a design flags as its
largest is the one that must be paid for before the trial, not carried into it.* Both are one
headless session each, and both must run **before the arm they serve is scored**.

- **R1 · Plant E against a live agent.** Does the worker repair the title, or discard the task
  and start clean? Either scores (see §3.3), but the rehearsal tells us which branch the
  answer key must be ready for, and whether the staged prose is expensive enough.
- **R2 · Plant F's stop.** Does an agent meeting a rejecting hook **stop and ask**, or
  self-serve the sign-off marker? B1's 1.0.0-gate worker stopped and reasoned well about why
  it must not create the marker itself; n=1. A worker that fabricates the marker leaves no
  pause, and is itself a finding.

**If R2 shows the stop is unreliable, B1's utterance arm converts to observation-only** and
the identity coverage rests on B2/B3's plant E — recorded here so the conversion is a
pre-registered branch rather than a salvage.

---

## 3 · The headline measurement

### 3.1 · What is no longer the headline, and why

Six trials asked *does an agent read its own staged work back?* The 1.0.0-gate trial answered
it **4/4 unprompted**, on the fence's own verb, with the cue card never firing —
3 / 4 / 6 / 7 reads per session. **Manufacturing a fifth occasion to observe the same behaviour
a fifth time buys nothing**, and the answer already carries its own caveat: M48's fence makes
the composed step print the command verbatim, so it measures **compliance with a named
instruction**, not discovery.

That series continues as a **secondary line** (§3.4). The headline moves to the two things a
designed occasion still buys.

### 3.2 · 3A — the channel under duress

**Every read observed so far was optional.** A worker that reads through the CLI when it does
not have to says little about what it does when it *must* read, and the cheap path
(`cat .jigc/tasks/<id>/docs/…`) is one call away.

**Plant E creates that state with no operator utterance at all.** The corpus ships with an
**open jigc task already staged** — one managed doc, created and slot-filled through the binary
by a setup script, authored by "someone who left". The blind prompt is one ordinary sentence:
*"someone started this and left it half-finished — pick it up and land it."* The worker
**cannot know what the doc holds without reading it.**

Score the four §3.3 channels unchanged, and read the result against the **invariant** rather
than against the fence: this is a direct test of VISION principle #3, the adapter-not-sandbox
bet, on the axis where the last trial already found the adapter *breaking for the workspace*
(a worktree reset with raw git, commit surgery by hand) while holding for *documents*.
**Document reads under duress are the untested cell.**

### 3.3 · 3B — consequence, not occurrence

**A `doc show` in the log scores VERB even if the worker ignored every byte of the output.**
That is a weak construct, and it is the one the read-back claim rests on.

So: inside plant E's staged doc, plant a **discrepancy visible only in the read-back** — a
field the worker did not set, or an item id that does not match its heading — chosen so it has
**exactly one sanctioned repair** (a `set-field`, a `retitle-item`), and score whether the
worker's **next logged command** is that argv.

**Three pre-registered outcomes, all of which score** — the criterion-5 discipline:

| outcome | evidence |
|---|---|
| **acted** | the next logged invocation is the sanctioned repair |
| **noticed and declined** | the feedback report names the discrepancy and gives a reason |
| **did not notice** | neither; the doc lands carrying it |

*Adversarially:* attribution is judgement-laden — a worker may notice and reasonably decline.
That is why "declined" is a row rather than a failure, and why the repair is a specific argv
rather than an inference.

**Plant E's four falsifiers, each with its mitigation, and each still scoring:**

| falsifier | mitigation | why it still scores |
|---|---|---|
| the worker **discards** the task and starts clean | make the staged prose visibly expensive — three full slots of good content, criteria with ids | a worker that destroys authored prose rather than repair a title is a finding about `discard`'s framing |
| the worker **keeps the wrong title** | put the naming authority in a **committed** doc the task must conform to, so the contradiction is on the page, not in the operator's head | *"shipped a doc whose title contradicts the spec it cites"* is pre-registered as a finding, not a missing measurement |
| the task **will not resume** | make the planted task the prompt's first ask; rehearse the resume on a corpus whose HEAD has moved | a refusal here is a finding about the fresh-clone-continuable record the team-ready design claims |
| the plant **does not survive the rig** | `.jigc/tasks/` is gitignored loose state: it travels with `run-session.sh`'s directory copy and **dies with a `git clone`** | asserted by a bar **inside the container** before the prompt is delivered; a failed assert voids the arm loudly |

### 3.4 · What is recorded — the channels, and the two settlements

Per session, from the **invocation log** and the **session transcript**, read by
`completions/trial-driver/run.py observe`:

| Outcome | Evidence |
|---|---|
| **VERB** | `jigc doc show … --task …` appears in the invocation log |
| **VERB-ADJACENT** | `jigc task diff <id>`, `jigc doc list --task …`, or `jigc task validate <id>` — a staged read-back **through jigc**, by a verb the fence does not name |
| **FILESYSTEM** | the transcript shows a direct read of `.jigc/**` or a managed doc path |
| **NEITHER** | the worker proceeds without reading, whether or not the outcome was correct |

**Both channels are recorded even when one fires.** A worker that uses the verb *and* `cat`s is
a different result from one that only uses the verb.

**Settlement 1 (owed by the ledger, decided here): VERB counts *attempts*.** The registered
wording is *appears in the invocation log*, and it stays. The question the series answers is
**channel choice** — did the worker go through the CLI or around it — and an attempt answers
that; the exit code does not. Redefining it would also break the reader's agreement with the
archive it is validated against, and silently reinterpret six trials.

**But a failed read is a product signal, so it gets its own row.** `VERB-effective` (calls at
`exit_code == 0`) is reported **beside** VERB, never folded into it, and:

> **Pre-registered: `attempts > 0` with `effective == 0` is a FINDING** — the worker chose the
> channel and the channel refused it — **not a VERB success and not a NEITHER.** It is classed
> under §1's wrong-result row.

**All prior trials' figures are attempts, and are relabelled, not reinterpreted.** rosewater's
registered 7 is 7 attempts / 5 effective; b3-bypass carries the same shape.

**Settlement 2 (owed by the ledger, decided here): the counter is aligned to §3.3, not §3.3
narrowed to the counter.** `jigc task validate` **joins** the adjacent set — §3.3 lists it, and
since M47 it previews what `finalize` gates on, so it is a genuine staged read-back. A bare
`jigc doc list` **leaves** it — §3.3 requires `--task`, and a bare `doc list` is a
committed-store *index* read, a different act. `driver/channels.py` is the **authoritative**
implementation; `run-session.sh`'s close-of-session counters were aligned to it and demoted to
an explicitly indicative quick look. Two implementations of one registered measurement is how
the undercount happened.

*Verified when the change landed:* the aligned shell counter reproduces the reader's §3.3
numbers exactly on all four archived sessions — **2 · 9 · 5 · 6**, against the old counter's
0 · 4 · 0 · 1 — and `run.py observe --archive` still reproduces the 1.0.0-gate table byte for
byte, including B1's corrected 3.

**Settlement 3 (not owed by anything — found by rehearsing, 2026-08-28): the plant's own
invocations are not the worker's, and until R1 ran they were counted as if they were.**

`.jigc/logs/invocations.jsonl` lives **inside the corpus**. Plant E builds its state by driving
the real binary, and its end-state bar reads the doc back five times — so every one of those
calls sat in the very channel the session is scored on, timestamped before the session began.
Measured on the uncorrected R1 evidence: **`observe` reported VERB 6 where the worker had done
2**, and 13 authoring writes where the worker had done 6. The error is **3×, and it points the
flattering way** — the direction nobody double-checks. Both plant-E arms carry this trial's
headline, so this would have inflated the headline measurement itself.

Two fixes, because one does not cover it:

- **`run-session.sh` stamps `session-start` into `PROVENANCE.txt`, and `observe` scores only
  records at or after it** — reporting the excluded ones and *what they would have added*,
  rather than silently correcting. Pinned by `test_observe.py::ThePlantIsNotTheWorker`, whose
  first case preserves the defect as it was.
- **The plant clears the log as its last act.** This is not redundancy: a worker that reads the
  log would **watch itself being planted** — `doc create adr --title "Reject the newest…"`
  followed by `set-field status superseded`, in order, with timestamps. That is operational
  rule 1's hazard (a worker that can tell it is being tested), and no session-start filter
  fixes it.

**The general rule this yields, for any future plant: a plant must not write to the channel its
arm is scored on, and if it must, the reader has to know when the session began.** Two other
things already write there — `arms/adopt.sh`, and any mid-session plant whose commit trips the
`pre-commit` hook's `jigc validate` — so the split earns its keep beyond plant E. (Bare
`validate` matches no §3.3 channel, so the mid-session case is harmless today; it is named here
so the next plant is not designed on the assumption that it always will be.)

**Settlement 4 (also found by rehearsing): a managed-document read and a workbench-bookkeeping
read are not the same act, and §3.3 was counting them as one.**

The FILESYSTEM row reads *"a direct read of `.jigc/**` or a managed doc path"*. Two runs of the
**same prompt against the same plant** split on exactly that seam:

| run | managed-document reads | workbench reads | what it means |
|---|---|---|---|
| R1  | **2** (the staged `.md` itself) | 5 | the adapter bypass the invariant is about |
| R1b | **0** — the document went through `jigc doc show --task` | 6 | the invariant held |

Both are `filesystem = 6` under one count, and they are **opposite results on the axis 3A
exists to measure.** So the two are now reported separately (`DOC` / `wkbn`, with a split line),
and read differently:

- a **managed-document** read off disk is the bypass `.jigc/AGENT.md`'s own rule names, and is
  what a FILESYSTEM outcome under duress means;
- a **workbench** read (`roles.json`, `base.json`, `intent`, `workflow`, `provenance.json`,
  `staged-snapshot.json`) is bookkeeping **no read verb exposes at all**. Reporting it as
  evidence against VISION principle #3 would be scoring a **capability gap** as a channel
  violation. Both R1 workers reached for it; that is a finding about the read surface, and it
  belongs in §1's capability-gap row, not in the headline.

Still a heuristic — it keys on the file being markdown, which every managed doc is and no
workbench file is — so the last call stays a human's, as the whole FILESYSTEM channel already
does.

**Window.** Record two numbers per session: reads **at any point**, and reads **at/after the
plant's state becomes reachable**. The any-point number carries the fence's claim; the second
is the duress observation.

**Do not rely on the worker's own account.** The log and the transcript are the instrument; the
feedback report is context.

### 3.5 · The pre-registered reading, and the control that makes a null readable

**The positive control (walk arm 00) runs FIRST, before any blind result is read.** Its only
job is to prove the VERB channel **can** fire and **can be counted**. It proves nothing about
discoverability — the operator already knows the verb.

> **If the control does not fire, no blind result may be read at all.** A uniform null in both
> directions is the signature of an apparatus that cannot discriminate, not of a finding. Fix
> the instrument and re-run; do not report a landing.

Readings, over the **duress** measurement (§3.2), N=2 scored:

- **2/2 VERB or VERB-ADJACENT** — the invariant holds where reading is not optional. Recorded
  as **compliance at N=2**, never as reliability.
- **VERB-ADJACENT instead of VERB** — the invariant held and the fence's specific verb did not.
  Reported explicitly and separately: the worker read through jigc by a verb no step named.
- **1/2** — partial. Blocks the claim, not necessarily the release; escalated with evidence.
- **0/2 with FILESYSTEM, control fired** — the adapter broke for *documents* under duress.
  **Escalate to the human with the evidence; do not adjudicate it inside the trial.** This is
  the row that touches VISION principle #3.
- **0/2, control did not fire** — **apparatus failure, not a result.** Nothing about the
  product may be concluded in either direction.

**Rows for cases that are not outcomes at all** — without these each falls through to NEITHER
and is read as a product result. All of these are already implemented as voiding rows in
`driver/cascade.py`, in the registered order:

- a **voided session** (contamination, §8) — excluded from the denominator, reading taken at
  N=1 with that stated; never backfilled;
- **the invocation log was never enabled** — **unmeasured**, not NEITHER;
- **the session never reached an authoring step** — no occasion existed; unmeasured;
- **the session halted awaiting a human** — unmeasured for the part it did not reach.
  ⚠️ **A halt looks exactly like success**: exit 0, `subtype: success`, `is_error: false`,
  empty stderr. Only `permission_denials` and `result` reveal it, which is why the process exit
  code is never the completion signal;
- **VERB *and* FILESYSTEM** — counted as VERB for the claim, and flagged separately: a direct
  read of a managed doc is against `.jigc/AGENT.md`'s own rule even when the verb was also
  used.

**The N=2 knife edge, stated in advance.** One worker's behaviour separates *ships* from
*escalate*. Any non-2/2 result is escalated **with its evidence** rather than adjudicated
inside the trial.

### 3.6 · The registered outcome table — order included, and machine-checked

Rule 3 of the postmortem: *enumerate the branches, and refuse a design with a silent one.*
This is that enumeration, and it is **not prose**. `driver/cascade.py` grades every session by
walking this list top-down, first match wins, and `cascade.check_registration()` parses the
table **out of this file** and compares it to the code **position by position**. A test in
`test_cascade.py` runs that comparison, so this table and the grader cannot drift apart
silently — the trigger fires itself, in the gate of whoever changes either one.

**Order is load-bearing, not presentational.** A set comparison stays green while two rows are
implemented the wrong way round, which is exactly how a voiding row came to be outranked in the
rig this is modelled on. Three orderings below are deliberate and each has a reason:

- **Row 1 outranks row 2** — a dead CLI usually also leaves no log, and the honest cause is the
  dead CLI.
- **Row 6 outranks every scoring row** — a session with no authoring occasion has nothing to
  read back, and calling that `NEITHER` manufactures a product result out of an absent one.
- **Row 10 sits *after* the three scoring rows** — a VERB result is fully measured on the
  channel carrying the claim, so a missing transcript must not throw it away.

| n | outcome | kind |
|---|---|---|
| 1 | `apparatus — the session did not run` | void |
| 2 | `apparatus — no invocation log` | void |
| 3 | `apparatus — invocation log unreadable` | void |
| 4 | `apparatus — the fork began cold` | void |
| 5 | `apparatus — the session did nothing` | void |
| 6 | `unmeasured — no authoring occasion existed` | void |
| 7 | `read back through the fence's verb` | score |
| 8 | `read back through jigc, by another verb` | score |
| 9 | `read around jigc, off the filesystem (heuristic — verify the reads)` | score |
| 10 | `unmeasured — no transcript, so the filesystem channel is blind` | void |
| 11 | `proceeded without reading` | score |

**Row 11 is the catch-all, and it scores.** There is no *"did not fire"* branch anywhere in
this table — that is the property the cue card lacked, written down as a shape a test can
check rather than as an intention.

**Row 9 is a heuristic and says so in its own name.** FILESYSTEM comes from the session
transcript, whose format changes between CLI versions, and registration state is not in the
transcript at all — so a read of a foreign, never-adopted file at a managed home matches and
**needs a human call**. VERB and VERB-ADJACENT come from the invocation log, a product surface
with a pinned record shape, and are exact.

---


---

## 4 · The three blind sessions

Clean-room rules throughout: **verbatim prompts, no operator hints beyond the answer key,
per-session feedback report, invocation log ON.**

> **The preload problem — read before designing any arm.** The one surface stating the read
> rule in words is `.jigc/AGENT.md`, reached through the bootstrap `CLAUDE.md`'s
> `@.jigc/AGENT.md`. **Both are written by `jigc setup`.** A session that runs `setup` itself
> assembled its system prompt before either file existed, so neither the adapter nor the
> installed skill can be in that session's context. In such an arm a 0-VERB result is
> substantially attributable to **the adapter never having loaded**, which is a fact about the
> session's structure, not the product's shape.
>
> **So the arms split deliberately.** B1 keeps setup in-session — a real adopter moment worth
> testing — and its read-back measurement is reported **separately and discounted**, with the
> preload state stated. B2 and B3 begin adopted, so the adapter is loaded at session start;
> that is also the more common adopter condition. The split is verified two-sided with the
> discriminating probe, per corpus, before the round.

**B1 · cold start (interactive).** A zero-doc repo: `jigc setup` → first task → first ADR →
finalize. Carries the **hook plant** (a rejecting `pre-commit` under `core.hooksPath`) and the
**carryover plant** (pre-staged files before the mint). **Plant F rides the hook's pause**: the
correction is appended to the answer-key reply the operator was already going to give. Plant E
cannot run here — an open task presupposes an adopted repo. **Read-back measured but
discounted.**

**B2 · the design altitude (interactive).** `do-research` → `form-vision` → roadmap → first
milestone, **entered through plant E's abandoned task**. Aim E at a `research` doc, **not** the
vision: `vision` is `singleton: true` and `jigc doc rename` refuses it outright, so the arm
would measure a refusal route and the NEITHER it produces would be misattributed. Rider C
(the slug-collision seed) is free here and scored as **opportunistic coverage only** — never
cited as the reason a surface was reachable.

**B3 · the corpus accretes (headless).** First changelog entry · spec → implement-from-spec ·
first arch-doc, **entered through plant E**, with the **foreign-ADR plant** landed mid-stream
by the poller. That plant is unchanged from the last trial and its meaning has changed: it now
exercises M46's Increment 3 directly — the tool used to tell two stories about one file
(`validate` said *adopt it*, `migrate-corpus` said *author the prose*, over a file jigc never
wrote, on a route that changed nothing). **The plant fires on a state polled from outside the
session** — `git rev-list --count $INSTALL..HEAD >= 2` — never on a string watched inside it.

**All three must reach the guides.** M48 ships them as an adapter-owned
`.claude/skills/jigc/SKILL.md` installed by `jigc setup`, and the harness auto-advertises
`.claude/skills/`. **Do not seed them**, and **do not report use as discovery** — the worker is
*told* the skill exists. The guide names **no read surface at all**, so a VERB result is
attributable to the pack steps and the M48 fence, not to the guide.

---

## 5 · The operator-scripted walk

Not a blind session, and the reason is evidence rather than preference: its value is entirely
operator-placed plants and destructive sequences **a blind agent cannot be relied on to reach**.

**Every arm is a script**, driven by `walk.py` through `run-session.sh --exec` — the same
copy-in / copy-out / provenance chain a blind session rides. The record carries a block per arm
**including the ones that did not run**, so an unrun arm is *visibly blank* rather than quietly
absent. That is a process fix for the failure the postmortem exists for: a narrative walk lost
a chartered probe.

**Arm conventions**, earned by this directory's own history: every step goes through the
`step()` helper that echoes the command, its **combined** output and its **exit code** (on the
first real run a step printed nothing and the record could not say whether it had succeeded or
failed quietly — it had emitted a full blocking finding on stderr); **the runtime image is
node-based and carries no `python3`**, so JSON is parsed with `node -e`; an arm states its pass
condition **before** the run and prefers capturing output to asserting a guessed string.

| arm | what it drives | M46 |
|---|---|---|
| **00** | **The positive control — runs FIRST** (§3.5). Mint → author → `jigc doc show … --task`. Pass: at least one such record in that corpus's invocation log. Fail: the instrument cannot discriminate — fix it and re-run before reading any blind session. | — |
| **01** | The identity surfaces: a second `doc create` at a bound role, a same-title `doc rename`, the committed-home `jigc rename`. | — |
| **02** | The **four** destroying doors: refusal codes at the three that carry one, **narration at the two that were silent** (`provision --force`, `uninstall --force` — narration is *not* gated on `--force`), and the `--ignored` axis. | Inc 2 |
| **03** | The **rc.11 → rc.12 upgrade** — the real 1.0.0 upgrade path, covered by nothing today because the fixture builder constructs every state with the *current* binary. Requires the new pair probes (§5.1). | — |
| **04** | The foreign arm: `validate`'s exit flip, `migrate-corpus`'s `unadopted[]` key and exit 0, and the **positive-control-shaped** check that the exit returns to 0 as the last foreign doc is adopted. **This arm carries §1 row 2's standard for §0.1.** | Inc 3 |
| **05** | `milestone finalize` over `cp -R`- and `mv`-shaped repos — the on-disk-path subject; `worktree_unreadable` in text **and** in the `--format json` envelope. | Inc 2/7 |
| **06** | The pre-guard repair routes, both arms: a planted hand-broken doc → `conformance.item-heading-unanchored` → `reconciliation.conflict-block` → the `jigc unmanage <source>` route **run verbatim**. Also carries Increment 1's observable arm-A/arm-B contrast. | Inc 5, Inc 1 |
| **07** | The changelog gate at `jigc task validate`, write-touch suppression, and the promoted-to-`blocking` **exit 3**. **This arm carries §1 row 2's standard for §0.2.** | Inc 6 |
| **08** | **T10/T11 — the postmortem's §6 step-4 debt**: `doc rename` on a **committed** identity (expect the refusal, not exit 0) and the singleton refusal; plus the destination-occupancy guard in both homes. | — |
| **09** | Increment 8's doors: `jigc describe <positional>`'s tip, the sub-task base-pin route, the three `read_staged_routed` shapes, and a read-shaped near-miss confirming the tip names a **read** verb. | Inc 8 |

**The standard for a declared behaviour change, fixed here rather than judged in the moment**
(§1 row 2 defers to it, and the person running these arms has read the code that produces the
output — so it needs a written bar). A declared change **reads as designed** only if all four
hold: (a) the output **names what it is objecting to**; (b) it **says what the consequence is**
rather than just that something is wrong; (c) it **names the route or consent** in the same
output; and (d) that route, run verbatim, **works**. Any of the four missing, it reads as
obstruction and is a finding.

### 5.1 · `verify-pair.sh` cannot be used as shipped, and here is its replacement

⚠️ **The shipped `verify-pair.sh` is vacuous on an rc.11/rc.12 pair.** It probes three verbs
M48 shipped — `doc rename`, `config get`, `describe --workflows` — asserting `old=absent` /
`new=PRESENT`, and **rc.11 already has all three**. It is also hard-pinned to
`EXPECT_OLD_SHA=8979f163d628…`, the pre-M48 rc.10 build.

**M46 shipped no new verb**, so the replacement probes are **behavioural**, each with its
direction stated:

| probe | rc.11 | rc.12 |
|---|---|---|
| `jigc validate` on a brownfield corpus | exit 0 | non-zero + `schema-conformance.unadopted-instance` |
| `jigc migrate-corpus --format json`, same corpus | `blocked` non-empty, exit 1 | `unadopted[]` key present, exit 0 |
| `jigc task validate <id>` on a clean `single-task` | *"no findings"* | the changelog advisory |

Re-pin `EXPECT_OLD_SHA` to `9a37f0152744f0cba5f9140483e1ca1b1c453c46` — the rc.11 image's own,
read back from `docker image inspect`, not searched for by version string — and parameterize
the probe list rather than hard-coding a second verb triple that will rot the same way.
**The rewritten script must fail when given the same image twice**; that is its own test.

---

## 6 · Coverage — every changed surface in exactly one column

M47's rule, applied to M46's diff. Each changed surface lands in **exactly one** of
**trial-reached** · **test-fenced (naming the suite)** · **neither** — and the *neither* column
must be empty or each member explained. A two-way split with no way to say *"fenced by a test,
not by the trial"* inflates the probe count and leaves changed lines unexamined.

**A coverage classification is a claim about the code**
([pinning.md](../../../implementation/pinning.md) §5): derive it from the enumeration, the
registry and the suites — never from a changed-file list, and never from a previous analysis.
`reachable` is not `exercised`.

**Stated up front, so no arm is invented for them — three parts of M46 are NOT trial-testable,
and that is their honest disposition, not a gap:**

- **Increments 9 and 10 carry no verb, no finding and no route.** They are doc + test only
  (`doctype_map_versions.rs`, `ledger_record_truth.rs`, `flow49_acceptance.rs`, and the
  612-golden regeneration whose **empty diff is the assertion**). There is nothing for a
  done-picture walk to reach. The wave's own decomposition says so in as many words.
- **Increment 1's lock and merge are reachable only by concurrent processes**, not by anything
  a worker types. The audit reproduced the symptom with a 2000-round harness (215 torn reads).
  **Test-fenced**; only its observable arm-A/arm-B consequence is drivable, and walk arm 06
  takes that.
- **Increment 4 is schema-author-reachable, not adopter-reachable.** `migrate-corpus.set-field-unfilled`
  and `migrate-corpus.fold-refused` need a pack schema-version bump that adds a leaf; every
  driven cell in M46's own record used a `JIGC_PACK_DIR` dev-pack copy. It gets a walk arm on
  that footing **or** is declared test-fenced — decided when arm 03 is written, and recorded
  either way.

---

## 7 · The conversion obligation

**Every CONFIRMED *and* REFUTED verdict arrives with a repro block**
([pinning.md](../../../implementation/pinning.md) §3). A refuted block's obligation is that the
refuting fact **has** a standing test — not that a duplicate is minted. Cite it as
`pinned-by: <suite>::<test>`, and **verify it by reading what that test asserts, never by the
name looking apt**. No mechanical checker fences this; §3 refuses a `pinned-by:` symbol parser
by name, on the grounds that *a grep is not a fence*.

**A REFUTED finding is expected and valuable**: five of the last seven trials produced headline
complaints that dissolved into shipped capability, and that pattern is itself the
discoverability signal.

**The driver does not grade findings.** It produces counts, timings and scored session
outcomes. Confirming or refuting a claim and writing its repro block stay hand work.

---

## 8 · Operational rules carried forward

Each earned by a prior trial:

1. **Back-date a plant's commit, not just its body** (`GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE`) —
   a worker that notices a back-dated file can infer it is being tested.
2. **Keep a contemporaneous `operator-log.md`** — every operator utterance into a blind
   session, logged verbatim *as it happens*, with its justification. **Required**, because the
   contamination rule can only be audited against it. `driver/interact.py` provides the
   answer-key parser, the contamination screen and the log writer; **nothing in the package
   drives it**, because a headless turn has no channel to inject a reply into, so it is used by
   the operator in the interactive arms.
3. **Commit an operator plant with an explicit pathspec** (`git commit -m … -- <path>`) so a
   worker's in-flight index can never be swept into the operator's commit.
4. **`docker exec` needs `-u node`.** It bypasses the entrypoint's gosu and lands as root, and
   every `git` call in `/work` then dies on *dubious ownership*. And `docker cp` preserves the
   **host** uid, so anything copied in may be unreadable by the container user.
5. **`run-session.sh` exits 0 on a failed arm by design.** The arm's real code is in
   `PROVENANCE.txt`; `session.arm_exit_code()` reads it.

> **The contamination rule, absolute:** no operator utterance may contain *verify*, *read
> back*, *check the doc*, `doc show`, or any synonym. The moment the worker is told to verify,
> the thing being measured has been handed over. If an operator exchange drifts toward it,
> **log the drift and mark that session's measurement void** rather than salvaging it.
> `interact.screen()` enforces a deliberately wider list than this one, at answer-key **load**
> time — a screen that only fires on use fires mid-session.

**Blind prompts are themselves operator utterances** and are read against the contamination
rule before they are frozen. The narrow channel statement — *"the project's docs are managed
with jigc, so the thinking goes in through it"* — is permitted; the unscoped *"do the whole
thing through the tool"* is **barred**, on the ground that it pushes against FILESYSTEM across
all the worker's activity including its own inspection behaviour, which is precisely what §3.2
measures. **The narrow form is still a declared bound**: a positive result is reported as
compliance plus an operator channel preference, never as unprompted tool preference.

---

## 9 · What this trial cannot answer

Stated so the 1.0.0 record does not imply coverage that was never bought.

- **Adopter-side freeze protection.** The manifest fence guards *this repo*, in CI. An adopter
  editing a manifest has no equivalent. Declared bound, not a trial gap.
- **Multi-process concurrency semantics.** M46's obligation was explicitly narrower than *fix
  concurrency*: what shipped is *the reconciliation state machine must not be silently disabled
  by a concurrent write*. **General store locking remains out**, and the save lock's own degrade
  arm is a declared weakening — after ~1 s of bounded spin the critical section runs
  **unlocked** rather than blocking. A single operator running one session at a time will not
  exercise any of it.
- **Whether a narrated ignored-path teardown loses work.** M46 refused a refusal here **on
  measured evidence** — a worktree that did its job holds build output, so refusing on that
  axis fires on the ordinary fan-out success path and trains `--force` into reflex. The loss is
  **visible, not prevented**. Its re-opening condition is an adopter reporting work lost that
  way; a trial arm cannot supply that.
- **Most regressions.** §1 makes a regression BLOCK, but only walk arm 03 touches rc.11 at all.
  Blind sessions carry no baseline. The row is retained because its consequence matters if one
  is found, not because this trial searches for them systematically.
- **The measurement under bypassed permissions — a declared, directional confound.** Scored
  blind sessions run with `bypassPermissions`. `jigc setup` allowlists `Bash(jigc:*)` while
  `cat`/`sed`/`head`/`grep` — every FILESYSTEM-channel action — are not, and would prompt in an
  adopter's real session. Bypassing removes friction from **one side only**, in a known
  direction: it makes FILESYSTEM *cheaper* than an adopter would find it. So a VERB result is
  **not weakened** by it; a FILESYSTEM or NEITHER result is **partly attributable** to it and
  must be reported with this bound attached. B3-strict runs the real condition, unscored.
- **Whether the headless transport generalises.** Increment 0's paired result is n=1, one
  corpus shape, one model alias, one CLI build; the interactive arms it is compared against
  were driven eight days earlier, so transport effect and date effect are not separated. It
  licenses *"the channel survives the transport"* and nothing quantitative.
- **Whether the isolated environment is representative of an adopter's.** A real adopter has
  their own global instructions, plugins and skills; the container has none. This is the right
  instrument for *attributing* a result and the wrong one for predicting an adopter's day.
- **Long-horizon drift.** Every trial to date is a short arc.

---

## 10 · Readiness — what is discharged, and what is still owed

**A plant assumed to fire is not a plant.** Every row below says how it was verified, not that it
exists.

### Discharged

| item | how it was verified |
|---|---|
| the rc.12 image | built from `314f59e`; `verify-image.sh` **7 passed / 0 failed**; gate record written, and it refuses rc.11 by name |
| `verify-pair.sh` | rewritten on behavioural probes (M46 shipped no new verb); **3/3 discriminate**; refuses a same-image pair; the `m48` set still verifies rc.10 → rc.11 |
| **plant E** | driven through the container's binary, **11/11 bars**, survives the transport (asserted *inside* a fresh container), and **rehearsed against a live agent twice** — both instruments fired and were consumed ([rehearsal-R1.md](rehearsal-R1.md)) |
| **plant F** | the pause is real on rc.12: a live agent stopped at the hook and declined to self-approve ([rehearsal-R2.md](rehearsal-R2.md)) |
| the walk | **11 arms, every one written and driven**; arms 04 and 07 carry §5's standard for the two declared changes; arms 03/10 assert each flip against a *measured* rc.11 baseline |
| the three blind prompts | written and **mechanically screened** (`interact.contaminates` — clean) |
| the answer key | 6 entries, **screened at load**, each matched against a probe question; an unmatched question halts rather than being improvised |
| plant F's correction | kept out of the answer key (it is a template, not send-as-is), with the same-slug no-op pre-send check inherited from `cue-cards.md` §0.7 |
| the corpora | instantiated → **gated 11/11 → adopted → planted**, in that order; frozen state asserted; the preload split **verified two-sided** ([corpora.md](corpora.md)) |
| the operator log | created before the first session, so *"logged as it happens"* is possible rather than aspirational |

### Still owed

- **§6's coverage table** — every changed surface in exactly one column. The arms now say what
  they reach; three cells are already known to be **test-fenced, not trial-reached** and are
  declared in the arms themselves rather than left to the table (Increment 5's conflict-block
  migration arm, Increment 1's lock and merge, and Increments 9/10, which carry no verb, finding
  or route at all). Increment 4 is schema-author-reachable and takes the same disposition unless
  a `JIGC_PACK_DIR` arm is written for it.
- **Running the sessions**, in the order §3.5 fixes: **walk arm 00 first**, and if the control
  does not fire, no blind result may be read at all.
- **The findings-verification pass** — every CONFIRMED *and* REFUTED verdict with a repro block
  and a `pinned-by:` citation verified by reading what the cited test asserts (§7).

### Carried into the trial as known, not discovered

Two candidate findings are already recorded in [pre-trial-findings.md](pre-trial-findings.md),
found while building the instrument and **deliberately not fixed** — fixing product surface here
would be scope creep, and would remove findings the trial could legitimately produce. They are
kept out of the trial's yield: an operator's discovery is not a blind session's.
