# The trial that follows M50 — pre-registered protocol

**Binary under test: `1.0.0-rc.14`, built from `21ffc0d4`**, baked into `jigc-gate:rc14` and
gated by [gate-rc14.json](gate-rc14.json). Written 2026-09-09, **before any session runs**.

**What this trial is for, in the human's own sequence: M50 → this trial → the 1.0.0 call.** The
call is the human's and waits on one measurement. Every other arm exists because rc.13 → rc.14
changed **39 source files** — 20 CLI modules and 19 engine modules — and four consecutive waves
have had audit findings that were *larger classes than reported*.

**The binary's identity is settled by sha, never by stamp.** `jigc --version` cannot tell two
trees apart — the trap that cost the last trial a corrected handover row. The image is built by
`git archive 21ffc0d4` inside the container and records `JIGC_SHA` in its own env; the gate
record carries the image id, the sha and the stamp, and `run.py gate` refuses a round it does
not cover. HEAD is two commits past the handover's `95c79be`, both **docs-only** (`git diff
--name-only` touches no `crates/`, `packs/`, `.rs`, `.yaml` or `.toml`), so this tree's code is
the code the shipped rc.14 binary was built from.

The rule every instrument obeys, from [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md):

> An instrument fires reliably iff its trigger is a **state**, its consequence is **re-raised
> by the product**, and **every worker behaviour maps to a scored outcome**.

**Plant a state the tool keeps telling the worker about; never schedule a sentence.**

**Read [pre-trial-findings.md](pre-trial-findings.md) first.** Verifying the handover produced
six findings, one of them blocking and now fixed: the gate was **red at the sha the handover
certifies as green**, because M50's audit-closing commit rewrote the sentence a standing fence
guards and did not touch the fence. Two more were defects in this trial's own apparatus,
including thirteen reader tests that had never executed — among them both fixes for the duress
cell's own misfilings.

---

## 0 · Declared behaviour changes, briefed before the first session

**The list has one home:** [decisions-pending.md](../../../implementation/decisions-pending.md)
→ *The trial that follows M50 — protocol inputs*. This section cites it and does not restate it
(CLAUDE.md → *cross-reference, never restate*) — but it must be read **as completed by §0.1 and
§0.2 below**, because the list as written is five sources short and one entry in it is stale.

An unbriefed observer scores any of these as a regression, and the trial then spends its budget
re-deriving a decision this repo already took. That near-miss has happened twice.

### 0.1 · The ledger was five sources short, and is now complete — read it there

The list as the handover found it pinned its own derivation to *"the twelve increment records"*
at the close (2026-09-08), while the wave shipped **thirteen** increments and the completion
audit ran **2026-09-09**. Increment 13 and audit findings **F1–F4** were therefore uncovered,
and every one of the five carries a user-visible change — two of them **doors that used to
succeed and now block**.

**They were added to the ledger, not copied here.** The five sub-bullets live under
*Completed 2026-09-09* in the entry cited above, and **N31 is struck there** with its
discharging datum: its trigger — *the M50 completion audit's triage* — fired, F3 closed it over
a larger class than the entry named, and it had gone on reading as open. **The live carried
count is six, not seven**, and `CLAUDE.md` is corrected in the same motion.

Two consequences belong here rather than there, because they are about the *instrument*:

- **F2 changed this trial's own measurement channel** — §3.7.
- **N27 and N15 are briefed as expected outcomes** — §0.2.

### 0.2 · The two carried defects this trial is the trigger for

- **N27 — `jigc task diff <id>`'s cold-start form answers almost nothing.** Its stated trigger
  **is this trial's plant-E arm**, with the instruction that if a worker again goes to the
  filesystem to read its own abandoned working area, N27 *"is the named candidate and is
  re-argued against its cost, not re-discovered."* Without this in §0 the headline is
  uninterpretable: a FILESYSTEM result would be read as a new discovery when the repo has
  already priced the capability that answers it.
- **N15 — a `--task` read of an unresolvable address is byte-identical to the task-less one**,
  and its route, followed verbatim, serves the **committed** copy to a reader holding a staged
  one. On a 1.0-pinned surface, met by any mistyped task-scoped address. Its trigger's second
  clause is likewise *a trial observation*, so a worker that follows that route and reads the
  wrong copy is the trigger firing, not a regression.

### 0.3 · The four most dangerous to misread

1. **Two intended exit-0 → non-zero flips.** `jigc task validate ""` answered *the task
   validates clean* and `jigc doc list --task ""` answered *no docs staged*, both at exit 0 on
   the **release** binary through rc.13. Both now block. **Driven both sides** (§2.3).
2. **`engine::result::SCHEMA_VERSION` 2 → 3** — the wave's only result-contract bump. Distinct
   from `doc show --format json`'s top-level `schema-version`, which is a *doctype* version and
   did not move; conflating them makes any probe on either vacuous.
3. **A fan-out that landed clean on rc.13 may now block at exit 3** (`1799a2d`, landed before
   the wave). The intended tightening, met **late in a milestone arm**, where a misread costs
   most.
4. **The deny floor gained `Bash(jigc uninstall:*)` and `Bash(jigc milestone discard:*)`.** A
   worker's call to either is blocked **by the harness**: the transcript shows a permission
   denial and **no jigc output at all**. An observer counting friction must not score these
   two, and **a walk arm needing either must run it outside the agent.**

### 0.4 · No migration pair is owed — established, not assumed

`git diff 979baca..HEAD` over both `config/schema-manifest.yaml` files and every
`schemas/*.yaml` is **empty**: no doctype schema moved between rc.13 and rc.14. An
rc.13-authored corpus is current on rc.14, so the RC-m50 arm 14↔21 analogue is unnecessary and
is omitted deliberately rather than forgotten.

---

## 1 · The pre-registered decision rule

Carried from [RC-m50/protocol.md](../RC-m50/protocol.md) §1 verbatim. **A finding's class
decides its consequence; severity alone does not.**

| Class | Consequence for the 1.0.0 call |
|---|---|
| **Data loss or corruption on any path** — bytes destroyed, unrecoverable, or a doc silently written wrong | **BLOCKS.** No exceptions, no "recorded as known". |
| **A regression** — something that worked on rc.13 and does not on rc.14 | **BLOCKS**, unless it is one of §0's **declared** changes and the trial confirms it reads as designed, judged against §5's four-part standard rather than impressionistically. |
| **A discoverability landing** (§3's measurement returns *filesystem* or *neither* under duress) | **BLOCKS THE CLAIM, and forces a decision the trial cannot make.** Not a bug — evidence about VISION principle #3. See §3.5. |
| **A blocking dead end** — a refusal whose route cannot run, or a state with no recorded recovery | **BLOCKS** when it can reach a **project-carrying** file. **SHIPS RECORDED** when the reachable set is incidental and the state has a correct recovery elsewhere. |
| **A wrong result on a non-destructive path** — a check that does not fire, a false green, a wrong machine-readable value, a panic | **BLOCKS** if it produces a false green over managed state, or violates a pinned `--format json` contract. **SHIPS RECORDED** otherwise. |
| **A surface/wording finding** — a lie, an ambush, a missing route on a non-blocking path | **SHIPS RECORDED**, listed in the 1.0.0 record as a known bound — *unless it is a one-way door.* |
| **A capability gap** — "I wanted a verb that does not exist" | **SHIPS RECORDED.** |

**The rider, changed for this trial.** RC-m50's rider marked every SHIPS-RECORDED finding
*cheap-now / expensive-after*, because that trial fed a wave. **This trial feeds the 1.0.0
call**, so the rider becomes: every SHIPS-RECORDED finding is marked
**reversible-after-1.0.0 / one-way**, and a finding that would freeze a wrong pinned contract
**BLOCKS** instead. A defect in a pinned `--format json` contract is not reversible: M48 closed
the additive-key window, and M49/M50's additions were declared *as* the pre-1.0 spend.

**Two rules that bind the adjudicator:** a finding's class is fixed from its evidence **before**
its consequence is looked up; and *"judged not to matter"* is not a disposition — every
confirmed finding lands in a row above, or the table is revised **in writing, with a reason**,
before the call.

---

## 2 · Instruments

**Three scored blind sessions on the headline shape, two further blind sessions for coverage,
one unscored adopter-condition arm, and one operator-scripted walk.** Breadth is the human's
call, taken 2026-09-09: *full net + a blind milestone arm.*

**Every session runs filesystem-isolated** — [trial-harness/README.md](../../trial-harness/README.md).
**Invocation log ON in every session. Fresh corpora, one per session, never reused.**

### 2.1 · The arms

| arm | corpus | plants | transport | scored |
|---|---|---|---|---|
| **walk 00** | `walk-rc14` | — | scripted | **the positive control — runs FIRST** |
| **B2** design altitude | adopted + **E** | E | **interactive** | **headline** |
| **B3** corpus accretes | adopted + **E** | E + foreign-ADR (polled) | headless | **headline** |
| **B3-h2** | adopted + **E** | E | headless | **headline** |
| **B1** cold start | naive | carryover + hook; **F** rides the hook's pause | **interactive** | coverage; read-back **discounted** and reported separately (`setup` runs in-session, so the adapter is not in its context) |
| **B4-h** the milestone | adopted | none | headless | coverage — the `fix-task`/`fix-finding` fan-out, `milestone join`'s changed verdict, the landing ack naming every commit, F1's new `add-from-spec` refusal, and `1799a2d` met late |
| **B3-strict** | copy of B3's | same as B3 | headless, `--strict-permissions` | **unscored and labelled so** — the adopter's real condition |
| **the walk** | `walk-rc14` | operator-placed | scripted | the regression net (§5) |

**Transport, and why.** The headline keeps RC-m50's split exactly — **one interactive, two
headless** — so the number is comparable. B1 is interactive because plant F's correction is
delivered at a pause the *product* creates and `-p` has no such door.

### 2.2 · The corpora

All from [trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`, gated
**12/12** by `check-corpus.sh` before anything else touches them, adopted through **the
container's own binary** (`run-session.sh --exec arms/adopt.sh … jigc-gate:rc14`), carried out
with `run.py carry`, **then** planted. **Order is fixed: instantiate → gate → adopt → plant** —
plant E deliberately breaks the gate's *no `.jigc` residue* bar, which is why it goes last.
Names are product-plausible with no `rc`/`trial`/`probe`/`gate` token. [corpora.md](corpora.md)
records each frozen state.

**Declared bound: the fixture moved.** PT-D is closed (pre-trial-findings PT-4) — the corpus's
`IngestQueue` is now on the live write path, the suite is 24 tests, and the gate has a 12th
bar. This is a difference from RC-m50's corpora. It is the right trade — four of that trial's
workers spent budget on the dead code, which is itself a confound on the headline — and it is
stated here rather than discovered in the record.

### 2.3 · The pair, and what it proves

`verify-pair.sh` gained a **`m50`** set (rc.13 → rc.14), and **every probe was driven on both
images before it was written down** rather than derived from the ledger. Two corrected the
prediction. Result: **4/4 discriminating; a same-image pair refused at exit 1.**

| probe | rc.13 | rc.14 |
|---|---|---|
| `jigc task validate ""` | `exit0` | `nonzero` |
| `jigc start --format json` over a repo holding a live task — `state`/**envelope** version | `clean/2` | `active-task/3` |
| `jigc doc schema adr --format json` — `contract-version` | `5` | `6` |
| `jigc doc show 'adr:../../outside/leak'` — the **code** | `store.not-found` | `store.malformed-slug` |

The last keys on the code, not the exit: driven, rc.13 refuses too, for a different reason (see
pre-trial-findings PT-5), so an exit-code probe there discriminates nothing.

### 2.4 · The rehearsal is a precondition

**R3 · plant E against a live agent on `jigc-gate:rc14`**, its own corpus, headless. Both
instruments must fire and be consumed; the plant's 11 bars must pass **inside the container**;
the invocation log must be empty at freeze. The binary changed underneath the plant, and the
postmortem's rule 4 is that the axis you are uncertain about is the one that must be paid for
**before** the trial. **Bar 9** — *`task validate` says nothing about `superseded`* — is the
one most exposed to a validate-sweep change and the one that makes 3B a read-back-only
measurement.

**R4 · plant F's stop on rc.14** (B1's prompt, headless). Does a worker meeting the rejecting
hook stop and ask, or self-serve the marker? If R4 shows the stop unreliable, **B1's utterance
arm converts to observation-only** — a pre-registered branch, not a salvage.

---

## 3 · The headline measurement

**Carried from RC-m50 §3 without change of construct, so the duress result is comparable:**
**3A the channel under duress** (plant E — an open task the worker cannot land without reading
what it holds) and **3B consequence, not occurrence** (the `status: superseded` discrepancy
visible only in the read-back, with exactly one sanctioned repair, `set-field … --value
accepted`). Plant E is reused **byte-identical** by reference, md5
`e8bcbee6ad930eeb65a2f9869c738add`, verified live; its four falsifiers and their mitigations
are unchanged, and each still scores.

**The reading it must beat is 1/3.** RC-m50 landed 1 VERB / 2 FILESYSTEM across two transports.

**No arm is spent on the read-back series.** It has stood at 100 % effective for five trials, it
measures compliance with a printed instruction, and it buys nothing more. It is still *reported*
off whatever arms run, because it costs nothing.

### 3.1 · The mechanism, pre-registered — and why it has to be

Verifying the handover found that its `also open:` row conflates two surfaces. Driven at HEAD:
the **`also open:`** block renders **one** directive per row — `` resume it with `jigc start
--task <id>` `` — while the **four** `Run:` directives (`start --task`, `task validate`,
`task finalize`/`milestone finalize`, `task discard [--force]`) belong to the *orientation*
`active-task` view, a different form of the verb.

**Neither surface names a read verb.** RC-m50's twice-named pull-tier cause — *no read verb
shows a task's whole staged area* — is **not closed**, and N27 is its recorded candidate. So the
only route by which M50 can move this cell is **indirect**: the worker orients, sees the task
and what it `staged:`, **resumes** it, and meets M48's read-back fence in the re-composed step
text.

Three outcomes, registered **before** the measurement so that a null is read rather than
discovered:

- **the cell moves** — the orientation view was enough, and the wave's second claim-half, which
  M50's VERDICT declares unprovable from inside the wave, is proven from outside it;
- **the cell holds at ~1/3** — the push half fired and the pull half was never built. A
  pull-tier finding on *state* for the second consecutive trial, escalated with its mechanism
  **named rather than rediscovered**, and N27 as the priced candidate;
- **the cell moves *and* the worker still reads off disk first** — recorded as both; §3.6's
  cascade already has a row for it.

### 3.2 · One sub-measurement added, and it is free

The **ordinal and elapsed position of the first read of the plant doc** — RC-m50 recorded *"the
fifth invocation"* and *"thirty seconds in"* by hand. Orientation now precedes everything, so
whether the filesystem read lands **before or after the resume** is exactly what distinguishes
the three outcomes above. Taken from the invocation log and the transcript, both of which
already carry it.

### 3.3 · The channels

| Outcome | Evidence |
|---|---|
| **VERB** | `jigc doc show … --task …` appears in the invocation log |
| **VERB-ADJACENT** | `jigc task diff <id>`, `jigc doc list --task …`, or `jigc task validate <id>` |
| **FILESYSTEM** | the transcript shows a direct read of `.jigc/**` or a managed doc path — reported split into **DOC** (a managed `.md`) and **wkbn** (workbench bookkeeping) |
| **NEITHER** | the worker proceeds without reading |

### 3.4 · The four settlements, carried unchanged

1. VERB counts **attempts**; `VERB-effective` (exit 0) ships beside it. **`attempts > 0 /
   effective == 0` is a finding**, never a VERB success.
2. `task validate` joins the adjacent set; a bare `doc list` leaves it. `driver/channels.py` is
   the authoritative implementation.
3. Records before `PROVENANCE.txt`'s `session-start` are excluded **and reported**; the plant
   clears the log as its last act.
4. DOC and wkbn are different acts and are reported separately. Two numbers per session: reads
   **at any point** and reads **at/after the plant's state is reachable**.

**Do not rely on the worker's own account** — and note the converse, from RC-m50: B2's duress
read was found *from* the worker's debrief because the reader could not see it. Both channels
are read; neither is trusted alone.

**New this trial:** the reader walks **subagent transcripts** and labels each read with the
agent that made it (pre-trial-findings PT-3). A worker that delegates its orientation no longer
moves the FILESYSTEM channel into a file nobody opens.

### 3.5 · The reading, and the control that makes a null readable

**Walk arm 00 runs FIRST.** If the control does not fire, no blind result may be read.

Over the duress measurement, **N=3 scored** (B2 interactive, B3 and B3-h2 headless):

- **3/3 VERB or VERB-ADJACENT** — compliance at N=3 across two transports, never reliability.
- **VERB-ADJACENT instead of VERB** — reported explicitly: read through jigc by a verb no step
  named.
- **2/3** — partial; **blocks the claim**, escalated with evidence.
- **≤1/3 with FILESYSTEM, control fired** — the adapter broke for documents under duress.
  **Escalate to the human with the evidence; do not adjudicate inside the trial.**
- **0/3, control did not fire** — apparatus failure, not a result.

**No widening and no tiebreaker.** N=3 is the registered shape. Adding a fourth arm after seeing
three is a post-hoc widening, and this protocol refuses it in advance.

### 3.6 · The registered outcome table — order included, and machine-checked

`driver/cascade.py` grades every session by walking this list top-down, first match wins;
`cascade.check_registration()` parses the table **out of this file** and compares it to the code
position by position, and `test_cascade.py` runs that comparison over every `RC-*/protocol.md`.

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

Row 11 is the catch-all and it scores; there is no *"did not fire"* branch. Row 9 is a heuristic
and says so.

### 3.7 · One measurement-channel caveat, from §0.1c

M50's audit swept the finding identity over 31 production sites, so **~29 doors that dropped
their code from the invocation log now record it**. Any rc.13-vs-rc.14 comparison of *log
contents* is therefore **not like-for-like**. The §3.3 channels are unaffected — they key on
argv, not on findings — so the headline number is comparable and the log's *finding* rows are
not.

---

## 4 · The blind sessions

Clean-room rules throughout: **verbatim prompts, no operator hints beyond the answer key,
per-session feedback report, invocation log ON.** B1 runs `setup` in-session, so the adapter is
not in its context and its read-back is reported **separately and discounted**; every other arm
begins adopted.

**All must reach the guides** (`.claude/skills/jigc/SKILL.md`, adapter-owned) — do not seed
them, do not report use as discovery. Note §0.1a: those guides' discard prose changed this wave.

Prompts live in [paste/](paste/) and are screened before they are frozen (§8).

---

## 5 · The operator-scripted walk

Every arm is a script under `completions/trial-driver/arms/walk/`, driven by `walk.py` through
`run-session.sh --exec`, **one block per arm in the record including the ones that did not
run**. Each arm names the kind of set it iterates. Derived from the code-side registries, never
from a changed-file list.

| arm | subject | the set |
|---|---|---|
| **00** | **the positive control — runs FIRST** | — |
| **A** | the degenerate/empty work-unit id axis — **expected GREEN**, this is the fix's own measurement, where RC-m50's arm 17 was expected RED | `WORK_UNIT_ID_DOORS` (25 doors × 4 cells) |
| **B** | the destroying doors and their single consent; **F4**'s `milestone discard` over staged prose **run outside the agent** | `DESTROYING_DOORS` |
| **C** | the two root knobs — workbench refusal, unusable root, `uninstall`'s third subject | `ROOT_KNOBS` |
| **D** | the address slug head; `rename --slug`; `write.untrackable-destination` | `DOCTYPE_DOORS ▸ Address` + `SLUG_DOORS` |
| **E** | orientation: the `active-task` view, `also open:`, envelope `schema_version` 3, and `findings: null` **carrying its reason** — *unknown* never rendered as *none* | `OrientationView`, matched exhaustively |
| **F** | the migration loci and the three byte-writing kinds | `SchemaChangeKind::ALL × LOCI` |
| **G** | the `ref`-target pack-load fence; `doc schema` contract 5→6 carrying `to:` | the composed doctype set |
| **H** | the write-miss route floor | `write_miss_shape_axis::CELLS` (13) |
| **I** | every text render of a finding reaching the house renderer; `milestone join`'s blocked verdict **before** the routing footer; the landing ack naming every commit | the 8 re-implemented sites |
| **J** | **F1**: `add-from-spec` over an outside-repo path, with the `ArgToken::Plain` bound stated | `ARG_TOKENS` |
| **K** | **F3**: repo-relative paths on the **read** doors, with the corrected **79 across 11 files** bound stated | `GUARDED_SRC` |
| **L** | the surface batch: one unknown-doctype answer, not-a-git-repo once, `--dry-run`'s `subject`, the code-anchor grammar, one pack spelled one way | `RefusalKind::ALL` |
| **M** | **0.1a**: re-`setup` over a locally edited SKILL.md → refuse-to-clobber | the adapter-owned artifact |
| **N15** | the `--task` miss route serving the committed copy — **expected RED, a measurement** | — |
| **N20** | the milestone non-hook refusal carrying no identity — **expected RED, a measurement** | — |
| **N27** | `task diff`'s cold start — **expected RED**, and the headline's named candidate | — |

**The three `N*` arms are measurements of carried defects, not regression checks**, exactly as
RC-m50's arm 17 was — and arm 17 is how that trial found its blocking finding.

**The standard for a declared behaviour change**, fixed here (§1 row 2 defers to it): a declared
change **reads as designed** only if (a) the output **names what it objects to**; (b) it **says
the consequence**; (c) it **names the route or consent in the same output**; and (d) that route,
run verbatim, **works**. Any of the four missing, it reads as obstruction and is a finding.

---

## 6 · Coverage — every changed surface in exactly one column

M47's rule over the rc.13 → rc.14 diff, in [coverage.md](coverage.md) after the sessions:
**trial-reached** · **test-fenced (naming the suite)** · **neither** (each explained). Derived
from the registries (`VERB_KINDS`, `DOCTYPE_DOORS`, `MINT_DOORS`, `DESTROYING_DOORS`,
`ROOT_KNOBS`, `WORK_UNIT_ID_DOORS`, `SchemaChangeKind::ALL`) and flow 51's declared proof split
— never from a changed-file list.

**Stated up front as not trial-testable, so no arm is invented for them:** the goldens and
ledgers of Increment 13 (a registry with no verb, finding or route for a walk to reach); the
`write_miss_shape_axis` fixture's manufactured declaredness cells; and the two `refs-post-hoc`
orientation goldens declared out of the compose sweep.

---

## 7 · The conversion obligation

Every CONFIRMED, **PARTIAL** *and* REFUTED verdict arrives with a repro block
([pinning.md](../../../implementation/pinning.md) §3) and a `pinned-by: <suite>::<test>`
verified **by reading what the cited test asserts**, or a stated `UNPINNED: <why>`. No
mechanical checker fences this; §3 refuses one by name. **The human's gate is unchanged: no
1.0.0 call until the ledger is closed.**

A REFUTED finding is expected — six consecutive trials produced complaints that dissolved into
shipped capability, and that pattern is itself the discoverability signal.

**The driver does not grade findings.**

---

## 8 · Operational rules carried forward

1. Back-date a plant's commit, not just its body.
2. **Keep [operator-log.md](operator-log.md) contemporaneously** — every utterance into an
   interactive session, verbatim, as it happens, with its justification. The permission-mode
   confirmation at session start is an operator touch and goes in the log.
3. Commit an operator plant with an explicit pathspec.
4. `docker exec` needs `-u node`; `docker cp` preserves the host uid.
5. `run-session.sh` exits 0 on a failed arm by design; the arm's code is in `PROVENANCE.txt`.
6. **A plant must not write to the channel its arm is scored on**, and where it must, the reader
   has to know when the session began (settlement 3).
7. **Reproduce in a copy, never in the live corpus.**
8. **Prefix every out-dir with the trial name** — `~/out/RC14-B2`, never `~/out/B2`. `~/out`
   currently holds RC-1.0-gate's `B1`/`B2`/`B3` beside RC-m50's, and `observe --gate` now
   refuses a stale directory rather than scoring it silently.
9. **Pass `--tag` explicitly everywhere.** Every driver default is still `jigc-gate:rc11`.

> **The contamination rule, absolute:** no operator utterance may contain *verify*, *read back*,
> *check the doc*, `doc show`, or any synonym. If an exchange drifts toward it, **log the drift
> and mark that session's measurement void.** `interact.screen()` enforces a wider list at
> answer-key **load** time — plain substring, case-folded, deliberately over-refusing.

Blind prompts are operator utterances and are screened before they are frozen. The narrow
channel statement — *"the project's docs are managed with jigc, so the thinking goes in through
it"* — is permitted and is a **declared bound**: a positive result is compliance plus an
operator channel preference, never unprompted tool preference.

---

## 9 · What this trial cannot answer

- **Adopter-side freeze protection** — the manifest fence guards this repo, in CI.
- **Multi-process concurrency semantics** — one operator, one session at a time.
- **Most regressions** — no arm carries an rc.13 baseline; §2.3's pair is a four-cell probe, not
  a regression suite.
- **The measurement under bypassed permissions** — a declared directional confound. A VERB
  result is not weakened by it; a FILESYSTEM/NEITHER result is partly attributable to it.
  B3-strict runs the real condition, unscored.
- **Whether headless generalises** — the paired transport result stays n=1 per corpus shape.
- **Whether the isolated environment is representative of an adopter's** — right for
  attributing, wrong for predicting a day.
- **Long-horizon drift.**
- **Whether the fixture change moved the headline** — §2.2's declared bound. The corpus differs
  from RC-m50's by the `IngestQueue` wiring, and no arm isolates that.

---

## 10 · Readiness

**A plant assumed to fire is not a plant.** Each row says how it was verified.

### Discharged before any session

| item | how it was verified |
|---|---|
| the handover | every row driven or computed; **five corrections** and one blocking finding recorded ([handover.md](handover.md) → *Verified*; [pre-trial-findings.md](pre-trial-findings.md)) |
| **the gate at HEAD** | **was RED at the sha the handover certifies green** (PT-1). The fold-back fence inverted as its own doc prescribes; `dev/gate` re-run to green **before** any session |
| the rc.14 image | built from `21ffc0d4` by `git archive` inside the container, `JIGC_SHA` baked, stamp asserted against the tree's `Cargo.toml`; `verify-image.sh` **7 passed / 0 failed**, check 4 (host **YES** / container **NO**) among them; [gate-rc14.json](gate-rc14.json) written and `run.py gate` accepts it |
| `verify-pair.sh` | new `m50` set, every probe **driven on both images before it was written**: 4/4 discriminate; a same-image pair refused at exit 1 |
| the apparatus | I-1, I-2 and PT-D all landed with tests; the driver's 7 suites green (**59** tests in `test_observe.py`, where 46 were running); `observe --archive` reproduces the 1.0.0-gate table **before and after** every change |
| the corpus template | 12 bars, 13 mutations, `self-test.sh` **14 passed / 0 failed**, suite 24/24 |

### Still owed, in order

- the corpora instantiated, gated 12/12, adopted through the container's binary, then planted;
  plant E's 11 bars passing **inside the container** on each;
- **R3 and R4** run and recorded;
- the answer key copied and screened at load; the six prompts screened and frozen;
- the walk arms driven through `run-session.sh --exec` before any is trusted;
- **walk arm 00 first**, then the blind sessions, then the rest of the walk;
- the findings-verification pass, [coverage.md](coverage.md), the record.

**Then the 1.0.0 call, which is the human's.**
