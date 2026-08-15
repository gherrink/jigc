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
| **A regression** — something that worked on rc.10 and does not on rc.11 | **BLOCKS**, unless it is the one **declared** behaviour change (`provision` refusing a non-empty leftover it cannot prove disposable) and the trial confirms the refusal reads as protection. |
| **A seventh discoverability landing** (§3's measurement returns *filesystem* or *neither*) | **BLOCKS THE CLAIM, and forces a decision the trial cannot make.** Not a bug — evidence that the mechanism fix did not take. See §3.4. |
| **A blocking dead end** — a refusal whose route cannot run, or a state with no recorded recovery | **BLOCKS.** This is the class M48's own audit found in its centrepiece; it is cheap to fix and expensive to ship. |
| **A surface/wording finding** — a lie, an ambush, a missing route on a non-blocking path | **SHIPS RECORDED**, routed to M46, listed in the 1.0.0 record as a known bound. |
| **A capability gap** — "I wanted a verb that does not exist" | **SHIPS RECORDED**, routed to M46. M48 refused seven of these deliberately; more are expected, and they are not defects. |

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

### 3.2 The designed need — and the rule that keeps it clean

**Each blind session carries one occasion where reading a doc back is necessary to succeed, and the
need arises from the task's own shape — never from an instruction.**

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
| **VERB** | `jigc doc show … --task …` appears in the invocation log at/after the correction |
| **FILESYSTEM** | the transcript shows a direct read of `.jigc/**` or a managed doc path (`cat`, `sed`, `head`, `grep`, an editor read) |
| **NEITHER** | the worker proceeds without reading, whether or not the outcome was correct |

**Both channels are recorded even when one fires** — a worker that uses the verb *and* also `cat`s
is a different result from one that only uses the verb, and the difference matters.

**Do not rely on the worker's own account.** The sharpest datum in RC-pre-1.0 (*"I never read back a
single managed doc I wrote"*) was a confession, and the next worker may not be that candid. The log
and the transcript are the instrument; the feedback report is context.

### 3.4 The pre-registered reading — and the control that makes a null readable

**The positive control, run first, on the operator walk. Added 2026-08-15 with the isolation
change.** Before any blind result is read, one arm must demonstrate that the VERB channel **can**
fire: an operator-driven task whose correct completion is impossible without reading a managed doc
back — the doc's on-disk state must differ from what the operator wrote, so recall cannot substitute
for reading. Its only job is to put one `jigc doc show … --task …` record in the invocation log.

**If the control does not fire, no blind result may be read at all.** A uniform null in both
directions is the signature of an apparatus that cannot discriminate, not of a finding — the failure
mode that voided a sibling project's entire results set three times, and the one this rig already
hit once (its first isolation probe returned UNKNOWN on the host *and* in the container). Fix the
instrument and re-run; do not report a landing.

Why it became necessary here: isolation removed the operator's global instructions from every
worker, including *"never ask what you can find out yourself."* Without a control, a moved number
cannot be attributed — it could be M48's fence failing, or it could be the removal of something that
was quietly propping the measurement up.

- **3/3 VERB** — the mechanism fix took. The claim holds; record it as the first trial in seven not
  to land the lens.
- **1–2 VERB** — partial. **Blocks the claim, not necessarily the release**: the fence works and
  discovery is still unreliable, which is a *surface* finding under §1 and routes to M46 — but the
  wave's claim is recorded as **unproven**, not proven.
- **0 VERB, control fired** — **the seventh landing.** Pre-declared by the wave itself as *"a signal
  about the product's shape, not about the trial."* This **blocks the claim** and forces a decision
  the trial cannot make: whether the adapter-enforced ergonomics bet
  ([VISION.md](../../../VISION.md) principle #3) needs revisiting before 1.0.0. **Escalate to the
  human with the evidence; do not adjudicate it inside the trial.**
- **0 VERB, control did not fire** — **apparatus failure, not a result.** Nothing about the product
  may be concluded, in either direction. This row exists so that the cheapest possible explanation
  for the wave's own predicted outcome cannot be skipped on the way to the expensive one.

A **NEITHER** outcome is scored as a non-VERB for this reading **and** examined separately: if the
worker proceeded blind and shipped something wrong, that is its own finding under §1.

---

## 4 · The three blind sessions

Clean-room rules throughout: **verbatim prompts, no operator hints, per-session feedback report,
invocation log ON.** Each carries its designed-need correction (§3.2) at a point where a doc has
been authored and not yet finalized.

**B1 · cold start.** A zero-doc repo: `jigc setup` → first task → first ADR → finalize. Carries the
correction at the ADR. Also the natural home for the two plants RC-pre-1.0 used and this trial keeps:
a **rejecting pre-commit hook under `core.hooksPath`**, and **pre-staged files before the mint** (the
carryover gate).

**B2 · the design altitude from zero.** `do-research` → `form-vision` → roadmap → first milestone.
Carries the correction at the vision or a research doc. This is the session that historically
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
  steps (17 methodology + 3 dev steps declare `read.staged-read-back`). A VERB result under §3.3 is
  therefore attributable to the M48 fence, not to the guide; state it that way in the record.

---

## 5 · The operator-scripted walk

Not a blind session, and the reason is evidence rather than preference: its value is entirely
operator-placed plants and destructive sequences **a blind agent cannot be relied on to reach**.
Every arm below is justified by a behaviour M48 changed that no blind probe reaches.

0. **The positive control — run FIRST, before any blind session is read** (§3.4). Numbered 0 so the
   arms below keep the numbers they are referenced by. Its only job is to prove the VERB channel can
   fire at all; it measures nothing about discoverability, because the operator already knows the
   verb.
   The shape that makes reading **unavoidable**: author a managed doc whose **derived** identity is
   not knowable from what was typed — a title long enough that the slug rule's word cap and edge
   stopword drop decide the slug — then perform an operation that must address the doc by that
   derived id. Recall cannot substitute, because nobody ever wrote the id down. Reading it back
   (`jigc doc show`/`jigc doc list`) is the only route, and it lands the record §3.3 counts.
   Pass condition: **at least one `jigc doc show … --task …` in that corpus's invocation log.**
   Fail: the instrument cannot discriminate — fix it and re-run before reading any blind session.
1. **The declared breaking change.** Plant a **non-registered leftover holding staged, unstaged and
   untracked work**, then drive `milestone provision`. Expect a refusal naming the path and the
   bytes, with `--force` as the hatch. **Then drive the same state at `discard` and `uninstall`.**
   The judgment to record is not whether it refuses — tests already fence that — but **whether the
   refusal reads as protection or as obstruction**, since this is the one deliberate regression an
   adopter meets. Also drive the **junk-directory** case, which now refuses where rc.10 succeeded.
2. **The rc.10 → rc.11 upgrade.** ⚠️ **Build rc.10 from `8979f16` — pin that sha, do not search for
   the version string.** **Two commits stamp `version = "1.0.0-rc.10"`**: `8979f16`, the genuine
   pre-M48 binary the last trial ran on, and `4fd7fbc`, which is still rc.10-stamped but **contains
   every M48 change including the audit fixes** (the bump landed late, at `9cb9b78`, deliberately
   *after* the audit). A search for the version finds the **wrong, newer** one, and building it makes
   this arm compare rc.11 against itself — **vacuous, with nothing in the output to reveal it**.
   Verify before authoring: `git -C <build-tree> rev-parse HEAD` is `8979f16`, and the built binary
   must **refuse nothing** at `milestone provision` over a planted leftover (rc.10's defect is the
   arm's whole baseline; if it refuses, you built the wrong tree).
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
is minted; cite the existing suite **by what it asserts, never by its name**. A REFUTED finding is
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
- **Long-horizon drift.** Every trial to date is a short arc. The salience-independent enforcement
  claim rests on the long-horizon study, not on this.
- **A direct comparison against RC-pre-1.0's reachability result.** Isolation (§2) removed a
  variable that was present in all seven prior trials, so a difference in the numbers cannot be
  attributed to M48's fence alone. The comparison is reported as **indicative**, and the absolute
  measurement — read against the §3.4 control — is what carries weight.
- **Whether the isolated environment is representative of an adopter's.** A real adopter has their
  own global instructions, plugins and skills; the container has none. This trial measures jigc
  without that layer, which is the right instrument for *attributing* a result and the wrong one for
  predicting an adopter's day.
