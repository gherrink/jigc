# The trial that follows M50, on `1.0.0-rc.14` — record

**Binary:** `1.0.0-rc.14` from `21ffc0d4`, in `jigc-gate:rc14` ([gate-rc14.json](gate-rc14.json)).
**Run:** 2026-09-09/10. **Protocol:** [protocol.md](protocol.md), pre-registered before any
session ran. **Status:** the arms are complete and adjudicated; **the conversion ledger is open**
([findings-verification.md](findings-verification.md)), and that ledger is the human's own gate
on the 1.0.0 call.

## What ran

| arm | corpus | transport | recs | wrote | VERB | eff. | adj | fs | outcome |
|---|---|---|---|---|---|---|---|---|---|
| **B2** | thornbury | **interactive** | 53 | 24 | 4 | 4 | 4 | 0 | `read back through the fence's verb` |
| **B3** | marlowe | headless | 120 | 62 | 6 | 6 | 6 | 1\* | `read back through the fence's verb` |
| **B3-h2** | oakhurst | headless | 70 | 27 | 5 | 5 | 5 | 0 | `read back through the fence's verb` |
| B1 | wickfield | **interactive** | 27 | 5 | 3 | 3 | 2 | 0 | `read back…` — **discounted** (setup in-session), reported separately |
| B4-h | redbourne | headless | 41 | 12 | 2 | 2 | 2 | 0 | `read back…` — **HALTED at the deny floor** |
| B3-strict | marlowe copy | headless, strict | 45 | 10 | 6 | — | 3 | 3 | **unscored** — halted on 4 denials |
| R3 / R4 | elmsworth / clayforth | headless | 19 / 14 | 5 / 3 | 3 / 1 | | 2 / 2 | 0 / 0 | rehearsals, both fired ([R3](rehearsal-R3.md), [R4](rehearsal-R4.md)) |

\* B3's single filesystem read is the **planted foreign ADR** — a never-adopted file at a managed
home, which no read verb can serve. Not an adapter bypass; the reader flags exactly this case as
needing a human call, and this is the call.

The walk: **23 arms**, arm 00 PASS first. `run.py observe --archive` reproduced the 1.0.0-gate
table exactly, before and after every apparatus change.

## §3 — the headline

**The duress cell (plant E) is scored at N=3: B2 interactive, B3 and B3-h2 headless — the same
shape RC-m50 ran, against the 1/3 it produced.**

| arm | channel | first read of the plant doc | 3B |
|---|---|---|---|
| **B2** (interactive) | **VERB** | `doc show … --task`, invocation **3**, before any write | **acted** on both |
| **B3** (headless) | **VERB** | `doc show … --task`, invocation **3**, **+5 s** | **acted** on both |
| **B3-h2** (headless) | **VERB** | `doc show … --task`, invocation **3**, **+12 s** | **acted** on both |

# **3/3.** Up from 1/3.

Under §3.5 that is *3/3 VERB or VERB-ADJACENT — compliance at N=3 across two transports, never
reliability.* All three are VERB proper, not adjacent. FILESYSTEM on the plant doc: **zero, in
all three.**

**The mechanism is the one pre-registered, and the ordinals are how we know.** §3.1 was written
before any session on the strength of handover correction C3 — that **neither** M50 surface names
a read verb, so the wave's only available route was indirect: orient → see the task and what it
stages → **resume** → meet M48's read-back fence in the re-composed step. Walk arm 22 confirmed
the premise against the binary (`the active-task view names NO read verb`; `the also-open block
carries RESUME ONLY`). The traces are that sequence:

```
B3-h2   1 start  →  2 start --task <id>  →  3 doc show adr:<plant> --task <id>
B2      1 start  →  2 start --task <id>  →  3 doc show adr:<plant> --task <id>
B3      1 start  →  2 doc show commit:… →  3 doc show adr:<plant> --task <id>
```

RC-m50's interactive arm reached for `find … | xargs cat` **thirty seconds in, before any jigc
read**. This trial's interactive arm read through the verb at invocation 3. **The push half
moved the cell**, and it moved it in the transport that had been the worst.

### The honest bounds on that number

1. **N=3 measures compliance, not reliability.** One worker moves the reading.
2. **The scored arms ran `bypassPermissions`** (§9, a declared directional confound). A VERB
   result is *not* weakened by it — permissive settings make the filesystem *easier*, so a worker
   choosing the verb chose it against the cheaper path.
3. **The fixture moved.** PT-D was closed before the trial ([corpora.md](corpora.md)), so the
   corpus differs from RC-m50's by the `IngestQueue` wiring. No arm isolates that.
4. **B3-strict, unscored, points the other way** and is reported rather than buried: under the
   adopter's *real* permission condition the worker's first move on the unfamiliar staged task
   was `find .jigc/tasks/<id> -type f | xargs … cat` — **the same reach RC-m50 recorded twice.**
   It was **denied by the harness**, and the session halted. That is not a scored cell, and it is
   the strongest available caution against reading 3/3 as settled.
5. **N27 was not needed to get here.** The carried defect whose trigger *is* this arm — no read
   verb shows a task's whole staged area — reproduces (F-6), and the cell moved anyway. Its
   re-argument is now a cost question, not a blocked-measurement question.

**The read-back series** (secondary): every scored session read staged work back through jigc,
`VERB-effective == attempts` throughout. **Six trials at 100 %.** No arm was spent on it.

## §0 — the declared changes, as met

| change | met by | reads as designed? |
|---|---|---|
| the two exit-0 → non-zero flips (`task validate ""`, `doc list --task ""`) | walk 17 · verify-pair | **yes** — 53/53 on the empty column |
| `SCHEMA_VERSION` 2 → 3 + the orientation lie's death | walk 22 · verify-pair | **yes** |
| `doc schema` contract 5 → 6 | verify-pair | yes |
| `store.malformed-slug` at the address doors | verify-pair · walk 18 | yes |
| **`task discard` refuses over staged prose** | **every arm that cleans up** | **yes, 4/4** — see below |
| the deny floor (`uninstall`, `milestone discard`) | **B4-h**, live | **yes** — and it halted a blind worker, correctly |
| `1799a2d`'s boundary tightening | not reached | **unmet** — no arm provisioned a fan-out |
| Increment 13's guide edit / SKILL.md re-clobber | not reached | **unmet** |

**The `task discard` change is the trial's clearest declared-change result, and it was nearly
scored as a regression.** Walk arms 07 and 08 were green in RC-m50 and red here. Driven on both
binaries: rc.13 exit 0, rc.14 `blocking · task-discard.staged-prose` exit 1. Judged against §5's
four-part standard it reads as designed on all four: it names the doc, says *discarding it would
destroy them*, carries three routes in the same output, and **the consent route runs verbatim at
exit 0** — as does the read route it offers **first**, `jigc doc show <address> --task <id>`.
Repaired at seven cleanup sites, the arms run 13/13 and 17/17.

**This is what §0 is for.** An unbriefed observer scores two green→red arms as regressions and
spends the trial re-deriving a decision this repo already took — the near-miss the ledger says
has now happened twice.

## §1 — the adjudication

| finding | class | consequence |
|---|---|---|
| **F-5** the unknown-id column: 22 doors, route but **no code** | surface | **SHIPS RECORDED** — reversible |
| **F-2** mid-task staged content committed; only the manifest names it | surface — framing | **SHIPS RECORDED** — reversible |
| **F-3** `superseded` with no `supersedes` validates clean everywhere | capability gap | **SHIPS RECORDED** |
| **F-4** `doc rename` vs `rename`, 2–3 help reads in 4 of 5 arms | surface / discoverability | **SHIPS RECORDED** |
| **F-8** no structural ref `adr → research` | capability gap | **SHIPS RECORDED** |
| **F-9** `doc rename` leaves the staged commit summary naming the old title | surface | **SHIPS RECORDED** — reversible |
| **F-10** no jigc verb amends a landed finalize | blocking dead end | **SHIPS RECORDED** — §1's qualifier: correct recovery elsewhere (git) |
| **F-11** the `doc author` payload parse error carries no code, no route | surface | **SHIPS RECORDED** — reversible |
| **F-13** a milestone offers one execution shape; a sequential spine has nowhere to go | capability gap | **SHIPS RECORDED** |
| **F-12** the hook frame does not name the hook's path | surface, minor | **SHIPS RECORDED** |
| **F-6** N27, **F-7** N15 | carried, declared | already routed, re-priced below |
| **F-1** the `--dry-run` manifest · **F-12** in substance | **REFUTED** | — |

**Thirteen findings. Nothing lands in the blocking row.** No data loss, no corruption, no regression, no false green
over managed state, no violated `--format json` contract. Every SHIPS-RECORDED row is marked
**reversible after 1.0.0** under §1's rider: none moves a pinned contract, and adding a code or a
sentence to a text refusal is additive.

**Two things this trial cannot say**, and it says so rather than implying otherwise: the fan-out
boundary (`1799a2d`) and the SKILL.md re-clobber path were **reached by nothing**. B4-h was the
arm chartered to reach the first, and **F-13 is why it could not**: it judged the fan-out unfit
for the sequential increment spine it had just planned, moved to abandon the milestone, and was
stopped by the deny floor. The uncovered boundary and the capability gap are the same event.

## What the trial reached that it did not set out to

- **Plant F was delivered, and landed on T9** — the staged re-slug, which
  [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md) §6 says *"any instrument that
  hopes to reach `doc rename` must put the worker in possession of a staged doc whose title must
  change."* B1 met the hook (`finalize` exit 1, `finalize.commit-rejected` in the log), stopped,
  **refused to fabricate the sign-off marker**, received the screened correction at a pause the
  *product* created, ran `jigc doc rename` on the staged doc at exit 0, read it back through the
  verb, kept authoring under the new identity, and landed
  `docs/decisions/shed-the-oldest-sample.md`. **Second delivery in the plant's history, first
  time reaching T9 by a correction rather than by a worker's own initiative.**
- **The pre-send check earned its place.** B1's own message named
  `ingestqueue-drops-the-oldest-sample` — the **task** id. The staged doc was
  `adr:drop-oldest-sample-on-ingest`. Had the correction been checked against the reported name,
  the arm would have measured nothing.
- **The trial's one adapter bypass, and it was not a preference.** Raised by the human after
  reading this record, and the earlier draft **understated it**: B1's `git reset --soft HEAD~1`
  plus manual `git commit` rewrote a commit that **contained a managed doc**, so that doc's
  committed state was produced by raw git. Driven and sized in
  [findings-verification.md](findings-verification.md): the doc's bytes are byte-identical across
  jigc's commit and the manual one, jigc's baseline is **content-keyed** rather than sha-keyed so
  nothing went stale, and `jigc validate` reading clean afterwards was correct rather than lucky.
  One arm, one commit — B2/B3/B3-h2/B4-h/B3-strict have **zero** resets and every `docs/` commit
  in them is a finalize. **What drove it is the finding**: F-2 swept two carryover paths in, F-9
  landed a commit message naming the doc's *old* title, and F-10 means no jigc verb can fix
  either once finalize has closed the task. The worker named the bypass in its debrief and ran
  `jigc validate` afterwards precisely because it knew it had gone around.
- **B4-h halted at the deny floor and did the right thing** — met `Bash(jigc milestone discard:*)`,
  saw a harness denial with no jigc output, and stopped and asked rather than routing around.
- **Three sessions delegated — B2, B4-h and R3 — and the I-1 fix was load-bearing on this very
  trial.** Four subagent transcripts, all walked, **zero** managed reads in any of them. Before
  the fix the reader opened one `.jigc`-adjacent `.jsonl` per session on the reasoning that
  *"the largest is the session itself; sidecars are small"*, so those four would have gone
  unread and three cells — including one scored arm of the headline — would have reported clean
  **without the delegated channel having been opened at all**. They report clean because it was.
  The owed apparatus item was owed for exactly this, and it came due on the trial that fixed it.

## The lens

**It is the discoverability lens again — seventh consecutive time — but on a narrower surface.**
The refuted row (F-1) is a capability one arm used and another reported missing. The
most-corroborated finding (F-4) is two verbs whose distinction costs every worker 2–3 help reads.
Neither is a defect in what jigc *does*.

**The one result that is about what jigc *does*, not what it says, is the F-2 → F-9 → F-10
chain** — and it is the only place in five blind sessions where an agent left the adapter. Each
link is small and reversible; composed, they took the one recovery jigc offers off the table and
left raw git as the only path. That is the adapter-not-sandbox bet being paid, visibly, on the
axis VISION principle #3 names.

**And the complete-fix lens landed once more on M50's own work**: F-5 is the *unknown-id* sibling
of the axis M50 swept to 53/53. The arm that found it (17) is the same arm that found RC-m50's
blocking finding, and it found this one the same way — by enumerating an axis rather than
checking a repro.

## Honest bounds

- **N=3, two transports, 3/3 — compliance measured, reliability not.** B3-strict points the other
  way under the adopter's real condition and is unscored.
- **The instrument was corrected six times before any figure was written**, every time by running
  it ([session-findings.md](session-findings.md); [pre-trial-findings.md](pre-trial-findings.md)).
  Two of the six were tests that had never executed, on the cell the headline rests on.
- **The gate was red at the sha the handover certified green** (PT-1), and the fold-back fence had
  been printing that since M50's audit-closing commit. Fixed here; the number the handover quoted
  (3341/0) is the number the gate now gives.
- **The conversion ledger is OPEN.** Every row carries a repro block; most carry `UNPINNED:` with
  a reason, several because pinning them would pin a gap or a carried defect as expected output.
  **The 1.0.0 call is not taken until it closes, and that is the human's gate, not this record's.**

## Owed after the trial

1. **The human's reading of 3/3 against 1/3** — the wave's second claim-half, now measured from
   outside the wave, with B3-strict's counter-signal beside it.
2. **The conversion ledger** ([findings-verification.md](findings-verification.md)).
3. **Two declared changes reached by nothing** — the fan-out boundary and the SKILL.md
   re-clobber — either walked or stated as uncovered in the 1.0.0 record.
4. **N27's re-argument as a cost question**, per its own trigger's wording.
5. **The F-2 → F-9 → F-10 chain**, which is the only adapter bypass the trial produced and the
   only finding about behaviour rather than wording. Cheapest link to cut is F-9: the rename
   already knows both titles and knows the task stages a commit doc.
