<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: Claude Opus 5 (1M context); see VERDICT.md -->
# Adversarial review — is the rc.14 trial valid input for the 1.0.0 call?

Read-only review at HEAD `74627547`. Every number below was re-derived from the raw
evidence or driven on the installed `jigc 1.0.0-rc.14`, never taken from the record's prose.

---

## VERDICT

**Valid input for the 1.0 call, with these stated corrections.**

The headline measurement is sound and I reproduced it independently: the scored table
(`recs/wrote/VERB/adj/fs` for all six arms) comes back byte-identical when I re-run the
reader over the surviving corpora, and the three scored ordinals are right. The instrument
defects do **not** contaminate the scored numbers, for a structural reason the record never
states plainly: the reader is a *post-hoc scorer over persisted channels*, so a fix applied
after a session still applies to that session — and I verified the fixed reader still
produces the published figures today.

Four corrections are owed before the human reads it, one of which the record gets **backwards**:

1. **B3-strict is not a counter-signal. It is a fourth VERB-first arm, under the stricter
   posture.** The record says twice that under the adopter's real permission condition "the
   worker's **first move**… was `find .jigc/tasks/<id> …| xargs cat`". Driven from that arm's
   own archived evidence: its first read of the plant doc was `jigc doc show adr:<plant>
   --task` at **invocation 3, tool call 3, 9 s in**. The `find | xargs cat` was **tool call 7,
   12 s later**, after the verb had already served the doc — and it was denied. The record's
   single strongest stated caution against 3/3 is false as written.
2. **The pre-registered mechanism is confirmed in 2 of 3, not 3 of 3**, and a third surface
   the record never counts — the adapter's own `SKILL.md`, loaded as tool 1 in every arm —
   names `jigc doc show … --task <id>` verbatim.
3. **Coverage row 12 is derived from the artifact, not the code** — the sixth instance of the
   failure `pinning.md` §5 catalogues five of — and one M50 surface (`pack.resource-missing`)
   sits in no column of a table titled *every changed surface in exactly one column*.
4. **The ledger is closed under this repo's own stated criterion**, not open. Calling it OPEN
   contradicts both prior closures. What is open is the human's *reading*, which is a
   different obligation and should be named as one.

Reversibility holds: **no finding requires a breaking change to a pinned contract.** But the
blanket *"none moves a pinned contract"* is imprecise for two rows (F-3, F-8) that cost a
frozen-schema/format bump plus a shipped corpus migration.

---

## 1 · THE HEADLINE — ordinals right, plant fired on a state, no missed filesystem read

**Ordinals: correct in all three arms**, from `evidence/<arm>/invocations.jsonl`:

| arm | inv 1 | inv 2 | inv 3 | Δ from session-start |
|---|---|---|---|---|
| B2 | `start` 12:51:19Z | `start --task record-the-ingest-queue-overflow --format json` 12:52:17Z | `doc show adr:reject-the-newest-sample-when --task … --format json` **12:52:19Z** | +64 s |
| B3 | `start` 21:21:10Z | `doc show commit:record-the-ingest-queue-overflow --task …` 21:21:15Z | `doc show adr:reject-the-newest-sample-when --task …` **21:21:15Z** | **+5 s** |
| B3-h2 | `start` 21:19:13Z | `start --task … --format json` 21:19:22Z | `doc show adr:reject-the-newest-sample-when --task …` **21:19:25Z** | **+12 s** |

The record's "+5 s" / "+12 s" check out against `PROVENANCE.txt`'s `session-start`.

**One qualification the record should carry: invocation 1 is not the worker's.** In all six
arms the first transcript tool call postdates invocation 1 by 2–56 s, and
`crates/cli/adapters/claude-code.yaml:7` binds `hook: { event: SessionStart, run: "jigc start" }`.
So record 1 is the harness's hook. The worker's own first jigc call is #2. The record's trace
table (`B2  1 start → 2 start --task → 3 doc show`) reads as three worker acts. It does not
change the direction — the worker's *second* call read the plant through the verb — but "the
ordinals are how we know" deserves the correct attribution.

**Plant timing: fires on a state, and the state predates the session.** `corpora.md:3`
records the fixed order *instantiate → gate → adopt → plant*, and `corpora.md:79-80` records
each planted corpus frozen with the task open, `adr:reject-the-newest-sample-when.md` staged,
and **the invocation log 0 bytes**. So every logged record is post-session, the ordinal
counting has no pre-session contamination, and the reader confirms it: it prints the
*"N record(s) predate this session … NOT scored"* note only for B4-h. Plant E is a state, not
a scheduled utterance — the postmortem's own rule, satisfied.

**Filesystem: I grepped the transcripts myself** (all tool_use of Bash/Read/Grep/Glob across
`transcript.jsonl` **and** `subagents/*.jsonl`, matching `.jigc|reject-the-newest|docs/decisions|*.md`):

- **B2 — zero.** The only `.jigc`-adjacent act is `ls docs/decisions/` (a directory listing of
  the *committed* store, not a read of the plant). Its one delegated subagent
  (`agent-a7c77d79a173b88ae.jsonl`) reads nothing managed.
- **B3 — one, and it is the one the record declares.** `Read /work/docs/decisions/0002-keep-the-sample-store-in-memory.md`
  — the planted **foreign, never-adopted** ADR, which no read verb can serve; the worker then
  ran `jigc migrate docs/decisions/0002-… --as adr`, which is the correct handling. Not the
  plant-E doc.
- **B3-h2 — zero** on the plant doc. Its `find . -path ./.jigc -prune …` at tool 17
  **explicitly prunes** `.jigc`.

**Independent re-derivation.** `python3 completions/trial-driver/run.py observe ~/out/RC14-{B2,B3,B3-h2,B1,B4-h,B3-strict}`
reproduces the published table exactly: `53 24 4 4 0` · `120 62 6 6 1` · `70 27 5 5 0` ·
`27 5 3 2 0` · `41 12 2 2 0` · `45 10 6 3 3`.

**3B acted on both instruments — confirmed per arm** from the logs: B2 rename @12 + `set-field
…#status --value accepted` @13; B3 rename @9 + status @13; B3-h2 rename @8 + status @12.

**The comparison to 1/3 is like-for-like.** RC-m50 ran the same model (`claude-sonnet-5`), the
same posture (`bypassPermissions`), the same harness, and scored DOC reads of the plant doc
(`RC-m50/trial-record.md:13,35,37`). I checked its traces: all three RC-m50 arms discovered the
task by `ls -la /work/.jigc/tasks/`; **none of the rc.14 arms did.** That is a concrete,
mechanism-level difference consistent with M50's orientation change, not a re-labelling.

### The one place the mechanism claim overreaches

The record: *"§3.1 was written before any session… the wave's only available route was
indirect: orient → … → **resume** → meet M48's read-back fence… **The traces are that
sequence.**"*

- B2 ✔ (`start` → `start --task` → `doc show`), B3-h2 ✔.
- **B3 ✘.** Its resume is invocation **6** (21:21:25Z), *ten seconds after* it read the plant
  at invocation 3. B3 never met the re-composed step before reading. The record's own trace
  table shows this (`B3 1 start → 2 doc show commit: → 3 doc show adr:<plant>`) while the
  prose asserts the opposite.
- And the premise *"neither M50 surface names a read verb"* is true of the two M50 surfaces
  and **false of the composition the worker was actually in**: tool call 1 in every arm is
  `Skill{jigc}`, and the shipped guide body (`crates/cli/src/setup.rs:91-97` → `QUICKSTART_GUIDE`)
  contains, at `QUICKSTART.md:120-128`, *"Managed docs are read through `jigc`, never off disk"*
  followed by `jigc doc show adr:<slug> --task <id>  # the staged doc itself`.

That guide shipped at M48 and was present in RC-m50 too, so it **cannot** explain the 1/3→3/3
move — but it does mean the route was *available directly*, and B3 took it directly. The honest
statement is: the cell moved, the orientation change is the best-supported cause, and the
sequence-of-record holds in two arms of three.

---

## 2 · THE INSTRUMENT — the scored numbers are **not** contaminated

Seven corrections; here is each with its timing and its reach into the scored cells.

| # | defect | fixed before/after scored arms | could it move a scored cell? |
|---|---|---|---|
| PT-1 | the gate was red at the sha the handover certified green | **before** (`pre-trial-findings.md`, written 2026-09-09 pre-session) | No. It is a prose fence (`foldback_truth.rs`); the rc.14 binary is untouched. |
| PT-2 | 13 of `test_observe.py`'s tests had never executed (`unittest.main()` mid-file) | **before** | **No — and this is the one that looks worst and is not.** The thirteen were *fences* for channel code that already shipped in RC-m50's reader. The classifier was live; only its guard was inert. Re-run today: **63 tests, OK.** |
| PT-3 / I-1 | reader opened one transcript per session, ignoring subagents | **before** the headline arms were *scored* | No. The subagent channel was **captured** (`evidence/{B2,B4-h,R3}/subagents/*.jsonl`, 4 files). I walked all four myself: zero managed reads. Had they not been archived this would be unrecoverable; they were. |
| PT-4 | corpus `IngestQueue` dead-code (PT-D) | **before** | It is a **declared fixture change** vs RC-m50 (`protocol.md` §2.2), correctly flagged as a confound on comparability. No arm isolates it. This is the one legitimate soft spot in the 1/3→3/3 comparison, and the record names it. |
| walk 07/08/18/22 repairs | instrument arms, not product | **before** each arm's result was written | No — walk arms are the regression net, not the headline. |
| PT-7 | FILESYSTEM counts **attempts**, not reads | **recorded, never fixed** | **No, for the stated reason and I checked it:** the three scored arms ran `bypassPermissions` — `result.txt` carries **zero** `permission_denials` for B2/B3/B3-h2 — so attempt ≡ read there. The asymmetry bites only B3-strict, which is unscored. |
| #7 | **there was no write channel at all** | **after B1 ran** | No. Applied retroactively over archived channels and, crucially, over the **corpus reflog**, which is independent of the worker's account. I verified both halves myself (§5). |

**Is any scored figure resting on a channel not open when the arm ran?** No. Every scored
figure is a function of two persisted artefacts (`invocations.jsonl`, `transcript.jsonl`) plus,
now, a third that also persists (the corpus reflog). I re-ran the *current* reader over the
*original* evidence and got the published table back. That is the test the worry demands, and
it passes.

**Plainly, for the user's worry:** the trial did test something in its own apparatus — seven
times — but **none of the seven changes a scored cell's value**, and the reason is checkable
rather than argued: the channels were captured before the fixes and the fixed reader still
reproduces the numbers.

**One thing that is contaminated and is disclosed:** the corpus fixture moved (PT-4).
**One thing the reader lost between trials:** RC-m50's table split `fs (DOC/wkbn)`; rc.14's
table prints a bare `fs`. The reader still computes the split (it printed `1 managed-document
read(s), 2 workbench-bookkeeping read(s)` for B3-strict when I ran it) — the *record* dropped
it, and that is exactly what hides §6's correction.

---

## 3 · THE LEDGER — closed under the stated rule; the label is wrong

**The stated criterion** (CLAUDE.md, and `pinning.md` §3): *"every trial repro block carries
`pinned-by:` or a stated `UNPINNED: <why>`"*, with §3 adding *"an audit finds any block still
`UNPINNED` **without a stated reason**"*. All thirteen rows carry one or the other. Both prior
closures used exactly this test:

- RC-pre-1.0: *"every row … now carries a `pinned-by:` citation … or a stated `UNPINNED: <why>`"*
  → **"the conversion ledger closed"**.
- RC-1.0-gate: *"every row now carrying `pinned-by:` or a stated `UNPINNED: <why>`"*
  → **"the conversion ledger … is closed"**.

So **the ledger is substantively closed and only unread by the human.** Declaring it OPEN is a
third, unstated criterion (presumably *all rows pinned*), and under that criterion no trial's
ledger has ever closed. This matters because the human's gate is written as a *mechanical*
condition; if the real gate is "I have read it", the record should say so rather than leaving a
satisfied condition marked unsatisfied.

**Are the UNPINNED reasons sound under §3?** §3's conversion discipline is asymmetric:
*refuted* claims must become standing tests; *confirmed* claims' blocks become **the fix's red
test**. Every confirmed row here is `SHIPS RECORDED` — no fix shipped — so an unpinned confirmed
row is procedurally correct, not evasion. And §5's addendum makes it more than correct: *"would
pinning it now pin the bug?"* — over these rows, yes.

Checked individually, including the three you flagged:

- **F-3** (`superseded` with no `supersedes` validates clean) — sound. Its reason as written
  (*"pinning the presence of a constraint would be pinning the fix"*) is the weaker half; the
  stronger half is that pinning the *current* behaviour pins the gap. Either way UNPINNED is
  right. **One honesty wobble:** the row writes `pinned-by:` `crates/cli/tests/…` — an ellipsis
  where a citation goes — immediately before the UNPINNED. That is a placeholder that reads as
  a citation. It should be deleted.
- **F-9** (rename leaves the stale commit summary) — sound. Any standing test over today's
  behaviour asserts the stale subject as expected output, which is precisely the harm §5's
  addendum names.
- **F-11** (payload parse carries no code/route) — sound, and its reason is *verified true*: I
  drove both halves of the contrast on the installed rc.14 (§4) and the divergence is real.
- **F-1, F-12** (the two refuted rows, where §3's obligation is strongest) — F-12 carries a real
  citation; F-1 carries UNPINNED with the pinnable half explicitly named as already fenced
  (`machine_output.rs` pins the `--dry-run` envelope incl. M50's `subject`). Acceptable: the
  refuted *fact* is fenced; what is unpinnable is "a worker did not look".
- F-5, F-6, F-7, F-8, F-10, F-13 — all either carried defects (pinning = pinning the gap) or
  absences of a verb/field (pinning = asserting a negative). Sound.

**F-12's `pinned-by:` — read and confirmed.** `crates/cli/tests/commit_rejected_axis.rs`
(1580 lines): line 736 asserts the frame's ``"`git commit` was rejected (no commit was made):"``
clause; line 740 + the per-door `survived:` strings (`:502`, `:538`, `:678`) assert the
state-truth sentence; `lift_rerun` (`:406-407`) lifts the re-run argv **verbatim** and re-runs
it to exit 0 after the hook is removed; the door set derives from `ERROR_CODE_REGISTRY`
(module doc `:19`). And I grepped: it asserts **no hook path** anywhere — exactly consistent
with the surviving clause of the finding. The citation is real and the one-clause test §3's
addendum demands is writable from it.

---

## 4 · REVERSIBILITY — nothing breaks a pinned contract; two rows are not "additive"

The governing map is `design/doc-read-surface.md:175-186`. After the 1.0 pin:
`doc show` / `doc list` / the command-output contract carry **no in-band version** and evolve
*"additive keys pre-1.0 only, then a versioned extension"*; `doc schema` carries
`contract-version` (now **6**) and *"bumps on any structural change — no additive carve-out"*,
i.e. it is the one surface designed for cheap post-1.0 evolution; the result contract's
`schema_version` (now **3**) bumps only when an external consumer must notice.

Row by row:

- **F-5** (22 doors, unknown id, no code) — **additive, verified by driving.** I checked the
  envelope shape: `jigc task validate no-such-task --format json` → `{"error": "no task …"}`
  and `jigc task validate "" --format json` → `{"error": "blocking · work-unit.malformed-id — …"}`.
  Both ride the **single-key `{"error": …}` reject envelope**, which `command-output-contract.md:361`
  pins as a contract shape and `:202-211` declares as the *flattened* complement for whole
  families (`work-unit.*` among them, `:211`). So the fix is putting a code inside a string that
  already exists: text-level, goldens move, no contract does. **But the finding's stated
  significance overstates what the fix buys** — *"a driver keying on the stable `(code, target)`
  pair gets nothing"* is equally true of the *fixed* malformed sibling, which also flattens.
  The repair gives a text-scraping driver a code, not a keyed pair.
- **F-2, F-9** — a new advisory in an existing `findings` array. A value, not a key. Free.
- **F-10, F-13** — a new verb / a new execution shape: a new surface declaring its posture at
  ship, which `command-output-contract.md:417` establishes as precedent. Free.
- **F-11** — **two repairs, and the record does not distinguish them.** The additive one (code +
  route inside the flattened string) is free. Enveloping it — turning `{"error": "malformed
  `doc author` payload: …"}` into the findings envelope its sibling emits — is a *shape* change
  at that door, which after the 1.0 pin is a versioned extension, not an additive key. I drove
  the contrast on rc.14: `doc author` (flat payload) → `{"error": "malformed \`doc author\`
  payload: unknown field \`status\` …"}`; `doc set-field 'adr:nope#nosuch'` → the full
  `{"schema_version":3,"findings":[{"code":"write.unknown-field","key":{…},"route":…}]}`.
  Same door, two shapes. Reversible **if** the fix takes the flattened path.
- **F-8** (`adr → research` ref) — **reversible, but not additive.** `adr` is a frozen dev-pack
  doctype currently at `schema-version 2` (the ledger's own F-3 repro shows `"schema-version":"2"`).
  Adding an optional ref field is a schema-*shape* change: manifest `schema-version` 2→3 **plus a
  shipped corpus migration for every adopter**. The transform kind exists
  (`SchemaChangeKind::AddedOptionalField`, `crates/engine/src/schema_diff.rs:718`) and the
  precedent exists (M36's adr `options` slot via `AddedOptionalSection`), so the frozen-migration
  path fits. But this is the freeze doing its job, not a free change.
- **F-3** (`status == superseded` ⇒ `supersedes` present) — **the sharpest one.** It needs a
  *conditional field requirement*, which no schema key expresses — i.e. a change to the
  **schema-definition format**, itself declared frozen v1 (CLAUDE.md → *Frozen-v1 doctype
  schemas*). Plus a `doc schema` projection member → `contract-version` 6→7, which is the
  sanctioned mechanism. So: reversible, at format-evolution cost, not at sentence cost.

**Answer to the question as asked:** no finding requires a *breaking* change to a pinned
contract. The record's blanket rider — *"none moves a pinned contract"* — is true; the implied
gloss *"and is therefore cheap"* is not, for F-3 and F-8.

---

## 5 · THE BYPASS CHAIN F-2 → F-9 → F-10 — demonstrated, not asserted; and I looked for a lying variant

**Demonstrated.** The carried corpus survives at `~/out/RC14-B1` and I re-ran the ledger's repro:

```
reflog:  e288714 HEAD@{0} commit: docs(ingest): record the decision to shed the oldest sample …
         a5d6081 HEAD@{1} reset: moving to HEAD~1
         445729a HEAD@{2} commit: docs(ingest): record the decision to drop the oldest sample …
git rev-parse 445729a:docs/decisions/shed-the-oldest-sample.md → 6628b9439a695090b243ca6c9e53bfcaf8094802
git rev-parse e288714:docs/decisions/shed-the-oldest-sample.md → 6628b9439a695090b243ca6c9e53bfcaf8094802   IDENTICAL
.jigc/state/file-state.json  docs/decisions/shed-the-oldest-sample.md = 6eba78aa…   (matches the ledger)
```

**"Content-keyed, not sha-keyed" is a structural fact, not an accident of this doctype.**
`crates/engine/src/file_state.rs:63-64` — the record is a single `BTreeMap<String,String>` of
`path → hex-hash`, and `:42` hashes **raw bytes** with blake3. There is no per-doctype variant
and no sha-keyed branch anywhere in it. So the "some doctype might be sha-keyed" variant does
not exist.

**Scope claim independently verified.** I grepped every arm's transcript for
`git (reset|commit|rebase|cherry-pick|filter-branch|amend)`: **B1 = 2 hits, every other arm = 0**,
and the reflogs of `~/out/RC14-{B2,B3,B3-h2,B3-strict,B4-h}` contain **zero `reset:` entries**.
The claim holds.

**The variant I went looking for, and what I found.** The obvious candidate for "raw-git
recovery leaves the store lying" is not the file-state map but the **base pin**:
`crates/engine/src/state.rs:557-559` — `BasePin { sha: String }`, *"the full commit SHA HEAD
pointed at when the task was minted"*, persisted to `.jigc/tasks/<id>/base.json`. A
`reset --soft HEAD~1` + re-commit makes that sha unreachable. I drove it on rc.14:

```
mint a task with base = bfcd3e8…; git reset --soft HEAD~1; git commit -m c1-rewritten
git merge-base --is-ancestor bfcd3e8 HEAD → NO        (base is unreachable)
jigc task validate <t>  → only the ordinary content findings, exit 3 (nothing about the base)
jigc start --task <t>   → composes normally, exit 0
jigc validate           → "no findings — the committed store validates clean", exit 0
jigc task finalize <t>  → finalized d7002d3, 1 file committed, exit 0
```

**On the single-task path the store does not lie — it does not care.** So my hypothesis is
refuted and I report it as such. The residual, which I could not drive cheaply: the **milestone**
path reads `base.sha` rather than HEAD for the merged code-anchor blast radius at the join
(`crates/cli/src/task.rs:4843`) and the boundary carries a `base == HEAD` guard (`:4371`). A
raw-git recovery performed while a milestone is open is the one shape where an unreachable base
pin is load-bearing. **F-10 makes raw git the prescribed recovery**, so that combination is
reachable in principle. Unverified; stated as residual risk, not as a finding.

**One residue the ledger does not mention** (harmless, but the ledger claims to have sized the
bypass): the manual commit dropped `scripts/retention-sweep.sh` and `src/router.ts`. They are
not lost — `git status` in that corpus still shows `A scripts/retention-sweep.sh` / `M src/router.ts`
in the index — but the repository was left with staged, uncommitted work that no jigc surface
reports. Consistent with "no data loss"; worth a clause.

---

## 6 · THE COUNTER-SIGNAL — the record has B3-strict backwards, and correcting it strengthens 3/3

The record, honest bound 4: *"under the adopter's **real** permission condition the worker's
**first move** on the unfamiliar staged task was `find .jigc/tasks/<id> -type f | xargs … cat`
— **the same reach RC-m50 recorded twice.** … the strongest available caution against reading
3/3 as settled."* `pre-trial-findings.md` PT-7 repeats it: *"the worker's first move on an
unfamiliar staged task was the filesystem, not a read verb."*

**Driven from `evidence/B3-strict/` — both statements are false.**

```
invocations.jsonl:
 1 05:26:33Z  start                                               ← the SessionStart hook
 2 05:26:40Z  doc show commit:record-the-ingest-queue-overflow --task … --format json
 3 05:26:41Z  doc show adr:reject-the-newest-sample-when --task …   ← THE PLANT DOC, via the verb, +9 s
 4 05:26:45Z  doc list --format json
 5 05:26:48Z  doc schema adr --format json
 6 05:26:48Z  doc schema commit --format json
 7 05:27:04Z  start --task …                                        ← resume, 23 s AFTER the read

transcript.jsonl tool calls:
 1 Skill{jigc}   2 doc show commit:   3 doc show adr:<plant>   4 ls docs/decisions/ + doc list
 5 doc schema adr   6 doc schema commit
 7 find .jigc/tasks/<id> -type f | xargs … cat        ← DENIED; 12 s AFTER the verb read
 8 find .jigc/tasks/<id> -type f   9 Read …/intent   10 Read …/workflow
```

`result.txt`'s `permission_denials[0]` is that exact `find | xargs` pipeline; the other three
are `Edit` on `src/store.ts`.

So under `--strict-permissions` the worker **also** read the plant doc through
`jigc doc show … --task` as its first read, at invocation 3 — the same shape and a *faster*
elapsed time than the interactive scored arm. Its filesystem reach came later and was for
**the whole staged area**, which is F-6/N27's named gap — the thing no read verb serves. That
is a different behaviour from RC-m50's, where the `find | xargs cat` at tool 6 **was** the first
plant read and no `doc show` of the plant preceded it.

**Weighed honestly:**

- It does **not** undercut 3/3 more than the record allows. It undercuts it **less** —
  materially so. Four arms across two transports and two permission postures read the plant
  through the verb, before any write, at invocation 3.
- What survives as a real caution is narrower and still worth carrying: under the strict
  posture the worker *did* reach for `.jigc/tasks/<id>` and *did* read two workbench files
  (intent, workflow) off disk when the verb could not show it the task's whole staged area.
  That is a live pull-tier gap on *state* — but it is F-6, not a refutation of the read-back
  behaviour.
- The reader's own DOC/wkbn split says the same thing and the record's table hides it: the
  reader prints *"1 managed-document read(s), 2 workbench-bookkeeping read(s)"* for B3-strict,
  and the one DOC read is the **denied** pipeline. RC-m50's table carried the split column;
  rc.14's does not.

This is the correction I would put in front of the human first, because it is the one place the
record argues *against* its own result on a fact its evidence contradicts.

---

## 7 · COVERAGE — the four `neither` rows are honest; the table is not complete

**The `neither` rows are right and well-explained.** 10b / 11a / 11b / `d854e25` all require a
milestone fan-out; none ran; B4-h was the chartered arm and F-13 is a documented, corroborated
reason it could not (its own words are in the transcript and quoted in the ledger). Row 4
(`ROOT_KNOBS` untried, fenced by `flow51_acceptance` arm 4) is correctly separated into
*untried* vs *unfenced*. That is the three-column rule applied properly.

**Three defects in the derivation:**

1. **`d854e25` is not in the rc.13→rc.14 diff.** `git merge-base --is-ancestor d854e25 f266770`
   → **yes**. It landed 2026-09-04 and is an ancestor of the sha the *previous* trial ran. The
   table's stated subject is *"the rc.13 → rc.14 diff"* and *"18 sources: 13 increments +
   `d854e25` + 4 audit findings"*. Including it is conservative (it was reached by neither
   trial) but the subject statement is wrong about that member.
2. **Row 12 is classified from the artifact, not the code.** It reads
   *"the bootstrap warning · the surface batch | **trial-reached** | walk 18"*. Increment 12's
   own roadmap scope (`implementation/roadmap.md`, *Increment 12*) is ~7 distinct surfaces:
   `setup`'s findings warning, the `code-anchor` grammar hint, the `LeftoverShape` sweep across
   **three** destroying doors, `milestone provision --force`'s narration, `--explain`'s `v`
   prefix, `--dry-run`'s composed subject, and the host-path leak. `18-surface-batch.sh`'s own
   header declares it an **M49 Increment 11** arm and states its pass conditions (a)–(h): the
   unknown-doctype Debug leak, the non-git-dir text, `write.unknown-section` at four verbs,
   the rename refusals, the non-UTF-8 argv byte, `describe --commands`, S-1/S-4, the changelog
   gate. **None of Increment 12's seven.** Its only `setup` line is `jigc setup >/dev/null 2>&1`.
   The host-path half is genuinely covered — but by **row F3 / arm 23**, not row 12. This is the
   sixth instance of the exact failure `implementation/pinning.md` §5 catalogues five of, and
   §5 binds *"at the moment of classification"*.
3. **One M50 surface is in no column at all.** Increment 10 shipped `pack.resource-missing`
   (commits `d974134e`, `9b9c0e1a` — a new code, a searched-pack-set message, a `Human` route,
   and the front door joining the family). No walk arm mentions it (`grep -l 'pack.resource-missing'
   arms/walk/*.sh` → empty) and no coverage row names it. Its correct column is **test-fenced**
   (`crates/cli/tests/pack_resource_miss_axis.rs` exists), so nothing is unfenced — but a table
   titled *"every changed surface in exactly one column"* has a surface in none.

**Nothing else is missing.** I walked `git log f266770..82075cc3` (13 `design(m50): increment N
task decomposition` markers, confirming 13 increments) against roadmap Milestone 50's increment
headings and the coverage table's rows 1–13 + F1–F4: the mapping is 1:1 apart from the three
items above.

---

## What I could not verify

- **The milestone/fan-out lying variant of the bypass chain (§5).** Driving it needs provisioned
  worktrees and a join; I did not build one (no cargo, and the rig drives the debug binary). The
  code citations are real (`state.rs:557`, `task.rs:4371`, `task.rs:4843`); the behaviour is
  untested by me and by the trial.
- **Whether the reader's channel *definitions* were identical at scoring time and now.** I
  verified the *outputs* are identical (the table reproduces) and that `test_observe.py` passes
  63/63 today. I did not diff the reader across the trial's working-tree states — the whole
  apparatus landed in one commit (`ba826655`), so per-fix timestamps are unrecoverable from git.
  This is why the "post-hoc scorer over persisted channels" argument does the load-bearing work
  rather than a timeline.
- **B3-strict's counterfactual.** Whether the strict-posture worker would have read the plant
  doc off disk *had the verb not already served it* is unknowable. I can only report that the
  verb came first.
- **Whether F-13's judgement about the fan-out is correct as a design matter.** The worker's
  reasoning is quoted and internally consistent with CLAUDE.md's isolation invariant; whether a
  dependent spine *should* have a milestone shape is the design question the ledger routes, and
  I did not adjudicate it.
- **`--explain`'s `v` prefix and `--dry-run`'s composed subject (Increment 12).** I confirmed no
  walk arm asserts them and that B2 *ran* `task finalize --dry-run --format json` at invocation
  19; I did not check whether that run's output carried the `subject` key, because the arm's
  captured stdout is not in the archived evidence.
