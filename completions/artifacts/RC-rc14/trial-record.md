# The trial that follows M50, on `1.0.0-rc.14` — record

**Binary:** `1.0.0-rc.14` from `21ffc0d4`, in `jigc-gate:rc14` ([gate-rc14.json](gate-rc14.json)).
**Run:** 2026-09-09/10. **Protocol:** [protocol.md](protocol.md), pre-registered before any
session ran. **Status:** the arms are complete and adjudicated; **the conversion ledger is open**
([findings-verification.md](findings-verification.md)), and that ledger is the human's own gate
on the 1.0.0 call.

## What ran

| arm | corpus | transport | recs | wrote | VERB | eff. | adj | fs (DOC/wkbn) | outcome |
|---|---|---|---|---|---|---|---|---|---|
| **B2** | thornbury | **interactive** | 53 | 24 | 4 | 4 | 4 | 0 / 0 | `read back through the fence's verb` |
| **B3** | marlowe | headless | 120 | 62 | 6 | 6 | 6 | 1\* / 0 | `read back through the fence's verb` |
| **B3-h2** | oakhurst | headless | 70 | 27 | 5 | 5 | 5 | 0 / 0 | `read back through the fence's verb` |
| B1 | wickfield | **interactive** | 27 | 5 | 3 | 3 | 2 | 0 / 0 | `read back…` — **discounted** (setup in-session), reported separately |
| B4-h | redbourne | headless | 41 | 12 | 2 | 2 | 2 | 0 / 0 | `read back…` — **HALTED at the deny floor** |
| B3-strict | marlowe copy | headless, strict | 45 | 10 | 6 | — | 3 | 1† / 2 | **unscored** — halted on 4 denials |
| R3 / R4 | elmsworth / clayforth | headless | 19 / 14 | 5 / 3 | 3 / 1 | | 2 / 2 | (0/0) · (0/0) | rehearsals, both fired ([R3](rehearsal-R3.md), [R4](rehearsal-R4.md)) |

\* B3's single filesystem read is the **planted foreign ADR** — a never-adopted file at a managed
home, which no read verb can serve. Not an adapter bypass; the reader flags exactly this case as
needing a human call, and this is the call.

† B3-strict's one **DOC** read is the `find … | xargs … cat` pipeline the harness **denied**; its
two **wkbn** reads are `intent` and `workflow` — workbench bookkeeping, not managed documents.
Bound 4 below carries the ordinals; PT-7 ([pre-trial-findings.md](pre-trial-findings.md)) carries
why a FILESYSTEM figure under a denying posture is an upper bound.

**[Corrected 2026-09-15 (M51 Increment 10, T2):** this table printed a bare `fs` column where
RC-m50's printed `fs (DOC/wkbn)` ([RC-m50/trial-record.md](../RC-m50/trial-record.md) → *What
ran*), collapsing two different acts into one integer. **The reader never stopped computing the
split** — `python3 completions/trial-driver/run.py observe ~/out/RC14-B3-strict ~/out/RC14-B2
~/out/RC14-B3 --gate completions/artifacts/RC-rc14/gate-rc14.json` prints `fs split: 1
managed-document read(s), 2 workbench-bookkeeping read(s)` for B3-strict — so the column was
dropped from the record, not from the instrument, and the drop is what hid the correction in
bound 4: collapsed to `3`, B3-strict reads as three filesystem reads under the strict posture;
split, it is **one denied pipeline plus two bookkeeping reads**. Re-scored from the archived
channels at the gated provenance, every published total reproduces unchanged — `45 10 6 3 3` ·
`53 24 4 4 0` · `120 62 6 6 1` · `70 27 5 5 0` · `27 5 3 2 0` · `41 12 2 2 0` — so only the fs
cells move, and they move by gaining their split.**]**

The walk: **24 arms** — `00`–`23`, the control first and PASS ([coverage.md](coverage.md) →
*What the walk actually ran* enumerates them). ~~*23 arms, arm 00 PASS first.*~~
— **struck, with the datum**: `ls completions/trial-driver/arms/walk/*.sh | wc -l` → **24**. The
sentence admits two readings and the datum refuses both — 23 either excludes the control it names
in its own next clause, or drops arm 23, which existed when this was written (`8228a42a`, the
parent of the commit carrying this record). The count is stated as the enumerable one.
`run.py observe --archive` reproduced the 1.0.0-gate table exactly, before and after every
apparatus change.

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

**The pre-registered mechanism is confirmed in 2 of the 3 scored arms, and it was never the only
route on offer.** §3.1 was written before any session on the strength of handover correction C3 —
that **neither** M50 surface names a read verb, so the route *the wave's own surfaces* offered was
indirect: orient → see the task and what it stages → **resume** → meet M48's read-back fence in
the re-composed step. Walk arm 22 confirmed that premise against the binary (`the active-task view
names NO read verb`; `the also-open block carries RESUME ONLY`). Two of the three traces are that
sequence. **B3's is not** — it read the plant at record 3 and did not resume until record 6:

```
B3-h2   1 start  →  2 start --task <id>  →  3 doc show adr:<plant> --task <id>              ← pre-registered
B2      1 start  →  2 start --task <id>  →  3 doc show adr:<plant> --task <id>              ← pre-registered
B3      1 start  →  2 doc show commit:… →  3 doc show adr:<plant> --task <id>  … 6 start --task <id>
```

**Record 1 is the harness's `SessionStart` hook in every arm, not a worker's move** — so these are
record ordinals, and the orient hop in the two pre-registered traces was *delivered*, not elected.
And the direct route was on the screen the whole time: the first thing every worker did was launch
the `jigc` skill, whose shipped body names `jigc doc show adr:<slug> --task <id>` verbatim. That
body shipped at M48 and was equally present in RC-m50, so it **cannot** be what moved the cell from
1/3 to 3/3 — but it does mean B3 needed no indirect route to find the verb, and **what carried B3
to it is not established by this trial.**

**[Corrected 2026-09-15 (M51 Increment 10, T3):** this passage read *"The mechanism is the one
pre-registered, and the ordinals are how we know … the wave's only available route was indirect …
The traces are that sequence."* Three data falsify it, each driven at HEAD or read from the
trial's own archived channels:

1. **`evidence/B3/invocations.jsonl`** — record 3 is `doc show
   adr:reject-the-newest-sample-when --task record-the-ingest-queue-overflow --format json`
   (`21:21:15Z`) and the **resume** is record **6**, `start --task record-the-ingest-queue-overflow
   --format json` (`21:21:25Z`). B3 read the plant *before* it resumed, so its trace is not the
   pre-registered sequence. B2 (record 2 `start --task`, record 3 `doc show adr:<plant>`) and B3-h2
   (the same shape) are. **2 of 3, not 3 of 3.**
2. **`grep -n SessionStart crates/cli/adapters/claude-code.yaml`** → `7:  - hook: { event:
   SessionStart, run: "jigc start" }`. Record 1 is that hook: in all four archived arms the `start`
   record precedes the worker's **own** first tool call (B3 `21:21:10Z` vs `21:21:12.784Z`;
   B3-strict `05:26:33Z` vs `05:26:37.649Z`; B2 `12:51:19Z` vs `12:52:15.132Z`; B3-h2 `21:19:13Z`
   vs `21:19:17.992Z`), and that first tool call is `Skill{jigc}` in every one.
3. **`git show 21ffc0d4:QUICKSTART.md`** — line **128** is `jigc doc show adr:<slug> --task <id>
   # the staged doc itself` and line **183** carries the same verb in prose. At that same sha
   `crates/cli/src/setup.rs:69` `include_str!`s `QUICKSTART.md` and `guide_body()` splices it into
   the installed `.claude/skills/jigc/SKILL.md`; neither line contains a `](`, so
   `unlink_in_repo_links` passes both through byte-identically. The read verb was therefore named
   verbatim in the artifact that was tool call 1 in every arm.

*Bound on leg 3:* the transcripts record the skill **launch** (`Launching skill: jigc`), not the
body served into the context, so what is established is that the worker invoked the artifact and
what that artifact said at the trial's own sha — not a byte-level read receipt. **The headline
number is untouched: 3/3 stands, and the honest bounds below stand with it.** What this correction
removes is the claim that one mechanism explains all three, and the claim that the indirect route
was the only one available.**]**

RC-m50's interactive arm reached for `find … | xargs cat` **thirty seconds in, before any jigc
read**. This trial's interactive arm read through the verb at record 3, having resumed at record 2.
**The push half moved the interactive cell** — B2's trace is the pre-registered sequence, its
orient hop delivered by the hook — and it moved it in the transport that had been the worst.

### The honest bounds on that number

1. **N=3 measures compliance, not reliability.** One worker moves the reading.
2. **The scored arms ran `bypassPermissions`** (§9, a declared directional confound). A VERB
   result is *not* weakened by it — permissive settings make the filesystem *easier*, so a worker
   choosing the verb chose it against the cheaper path.
3. **The fixture moved — a disclosure this record carries, not a defect it fixes.** PT-D was
   closed before the trial ([corpora.md](corpora.md); [protocol.md](protocol.md) §2.2), so the
   corpus differs from RC-m50's by the `IngestQueue` wiring, and **no arm isolates that.** It is
   the one legitimate soft spot in the 1/3 → 3/3 comparison: the comparison is across two
   corpora that are not byte-identical, and nothing a later reading of this file can retire —
   only a re-run with the fixture held fixed could, and none was spent.
4. **B3-strict, unscored, is a *fourth* VERB-first arm — its filesystem reach came later, and was
   denied.** Under the adopter's *real* permission condition the worker's first read of the plant
   was `jigc doc show adr:reject-the-newest-sample-when --task record-the-ingest-queue-overflow`
   at **invocation 3, tool call 3**, `05:26:41Z` against a `session-start` of `05:26:32Z` — **9 s
   in, before any write.** The `find .jigc/tasks/<id> -type f | xargs … cat` — the reach RC-m50
   recorded twice — is **tool call 7**, `05:26:53.490Z`, 12 s later, and the harness **denied**
   it; three `Edit` denials on `src/store.ts` followed and the session halted. So the strict
   posture produced a reach worth recording (PT-7 prices it), but not a first move, and not a
   counter-signal to 3/3.
   **[Corrected 2026-09-15 (M51 Increment 10, T2):** this bullet read *"B3-strict, unscored,
   points the other way … the worker's first move on the unfamiliar staged task was `find
   .jigc/tasks/<id> -type f | xargs … cat` … it is the strongest available caution against
   reading 3/3 as settled."* Falsified from that arm's own archived channels:
   `evidence/B3-strict/invocations.jsonl` records 1–3 are `start` `05:26:33Z`, `doc show
   commit:… --task … --format json` `05:26:40Z`, and **`doc show
   adr:reject-the-newest-sample-when --task record-the-ingest-queue-overflow` `05:26:41Z`**;
   `evidence/B3-strict/transcript.jsonl` puts the `find … | xargs -I{} sh -c 'echo ==={}===; cat
   {}'` at **tool call 7**, `05:26:53.490Z`, with a `tool_result` of `is_error: true` — *"This
   Bash command contains multiple operations. The following part requires approval: xargs …"* —
   i.e. **denied**. The correction **strengthens** the headline it was written against: under the
   strictest posture run, the worker still went to the verb first. It does not make the headline
   4/4 — B3-strict is unscored by design, and stays unscored.**]**
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

- **N=3, two transports, 3/3 — compliance measured, reliability not.** B3-strict, unscored, read
  the plant through the verb first as well; what the strict posture carries is a **later, denied**
  filesystem reach, which is a caution about the posture rather than a counter-reading of the
  cell. **[Corrected 2026-09-15 (M51 Increment 10, T2):** this bullet read *"B3-strict points the
  other way under the adopter's real condition"*, the same claim §3's bound 4 carried and the same
  datum falsifies it — the plant read at **invocation 3, tool call 3**, `05:26:41Z`, against the
  `find | xargs` **denied** at **tool call 7**.**]**
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
   outside the wave, with B3-strict's later, **denied** filesystem reach beside it (§3, bound 4)
   and the fixture disclosure (§3, bound 3) beside that.
   **[Corrected 2026-09-15 (M51 Increment 10, T2):** this item read *"with B3-strict's
   counter-signal beside it"*; bound 4's datum falsifies *counter-signal* — that arm read the
   plant through `doc show … --task` at tool call 3 and its `find | xargs` was tool call 7 and
   denied.**]**
2. **The conversion ledger** ([findings-verification.md](findings-verification.md)).
3. **Two declared changes reached by nothing** — the fan-out boundary and the SKILL.md
   re-clobber — either walked or stated as uncovered in the 1.0.0 record.
4. **N27's re-argument as a cost question**, per its own trigger's wording.
5. **The F-2 → F-9 → F-10 chain**, which is the only adapter bypass the trial produced and the
   only finding about behaviour rather than wording. Cheapest link to cut is F-9: the rename
   already knows both titles and knows the task stages a commit doc.
