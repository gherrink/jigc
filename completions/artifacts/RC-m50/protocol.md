# The pre-v1 trial — pre-registered protocol

**Binary under test: `1.0.0-rc.13`, built from `f266770`** (HEAD, four commits past the gate fix
`d854e25` — *not* the host-installed rc.13, which predates it; [pre-trial-findings.md](pre-trial-findings.md)
PT-2). Written 2026-09-04, **before any session runs**. Everything needed to execute it is here or
is named as owed in §10.

**What this trial is for, in the human's own sequence: gate fix → this trial → M50 → v1.** Its
findings are M50's input, and M50 is the last milestone before 1.0.0, admitting *everything that
improves usability, routing, bug-fixes … and above all everything that becomes impossible or
expensive to change once people start using the product* ([handover.md](handover.md)). So a
finding here is not only adjudicated against the §1 table; it is also **priced against the pin** —
whether it is cheap now and expensive after 1.0.0.

**What makes it different from the trial before it.** [RC-1.0-final](../RC-1.0-final/protocol.md)
measured `1.0.0-rc.12` and found no blocking finding. Since then M49 shipped **twelve increments**
(three schema bumps, a new doctype, a new knob, a closed `set:` vocabulary, a project-pack
composition, and the item-region integrity fix) and `d854e25` tightened the milestone boundary.
The human's two calls shape the instrument: **fresh corpora only**, so the blind sessions measure
the fixes and not the migration; and **the migration gets its own arm**, so the three bumps are
measured against an rc.12-authored corpus rather than met blind.

The rule every instrument obeys, from [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md):

> An instrument fires reliably iff its trigger is a **state**, its consequence is **re-raised
> by the product**, and **every worker behaviour maps to a scored outcome**.

**Plant a state the tool keeps telling the worker about; never schedule a sentence.**

---

## 0 · Declared behaviour changes, briefed before the first session

An unbriefed observer scores each of these as a regression, and the trial then spends its budget
re-deriving a decision this repo already took. Each row names its source. **The ledger was one
short last time** ([RC-1.0-final/protocol.md](../RC-1.0-final/protocol.md) §0.2); this list was
derived from the M49 roadmap, the VERDICT, and every DECISIONS entry 2026-08-28..09-04, then
checked against the binary where a claim could be checked in one command.

| # | on rc.12 | on rc.13 | source | who meets it |
|---|---|---|---|---|
| **0.1** | `doc show --format json` had no top-level `schema-version` | a top-level **integer** `schema-version` beside `type`/`slug`/`item-count`; `null` for the transient `commit`; `fields` stays stringly; slices and `doc list` gain nothing | M49 Inc 8 N2, DECISIONS 2026-08-31 | every arc that reads JSON |
| **0.2** | a nested-section-hop miss answered **four** codes across six item doors (`write.not-present` / `write.wrong-shape` / `write.unknown-field` / a bare `{"error"}`) | one code, **`write.unknown-section`**, with a `jigc doc schema` route, at every item-addressing door | M49 Inc 8 N1 | a mistyped nested address |
| **0.3** | no `planning-record` doctype | `jigc start --workflow planning` composes the 14-gate skeleton and `task finalize` **blocks** on any unfilled gate (`required-slot-present`, naming it) | M49 Inc 9 D9 | B4-h; walk 19 |
| **0.4** | `milestone add-task … --workflow <unknown>` minted at exit 0 | blocks **before** the mint — `workflow-refs.unknown-workflow`, enumerating the loaded workflows | M49 Inc 2 | any milestone arc; walk 12 |
| **0.5** | an rc.12 corpus is current | an rc.12 corpus holding a `completion-record` (1→2) or `milestone-record` (2→3) makes **`jigc validate` exit non-zero** (`schema-conformance.schema-version-current`, routed at `jigc migrate-corpus`) until the migration lands; **`migrate-corpus` first, `setup` second** ([MIGRATING.md](../../../MIGRATING.md)) | M49 Inc 9 | walk 14/21 only — a fresh corpus never meets it |
| **0.6** | under `finalize.fan-out.squash: false` the milestone boundary landed a sub-task whose commit doc had `type`/`summary` unset — subject `: …`, or `feat:` with no subject | the boundary **blocks** on the sub-task's transient commit doc exactly as `task finalize` does; the route carries `--task <sub-id>` | `d854e25`, DECISIONS 2026-09-04 | walk 14/21 only — the knob defaults to `true`, which is carved out |
| **0.7** | a project schema shadow could drop a section and validate clean | every door **blocks** by name with a route — **except `jigc setup`, which completes at exit 0** over it (declared bound: the bootstrap door cannot refuse circularly) | M49 Inc 3; VERDICT → declared bounds | walk 13 |
| **0.8** | `describe --commands` listed 16 ids | the **union** per (id, pack), **31** entries, each with a `pack` key; three more workflows carry `suppressed:` | M49 Inc 11 T6 | walk 18 |
| **0.9** | `jigc task validate ""` — | on the **release** binary: *"validates clean"*, exit 0 (a false green over a task that does not exist); on a **debug** build: panic, exit 101. `task discard ""` acks at exit 0 on both. **Known before the trial**, routed to M50, measured by walk 17 | PT-1 | any worker that passes an empty id |
| carried | — | `jigc validate` exits non-zero over never-adopted squatters (M46); `jigc task validate` draws `changelog-recording.gate-granted-unused` at exit 0 on a gate-granting task that touched no changelog (M46) | RC-1.0-final §0 | every adoption arm |

**A clean corpus does not meet 0.5 or 0.6.** That is the point of the human's *fresh corpora*
call, and it is why the migration is a walk pair and not a blind session.

---

## 1 · The pre-registered decision rule

Carried from [RC-1.0-final/protocol.md](../RC-1.0-final/protocol.md) §1 verbatim, with §0
above as the declared set. **A finding's class decides its consequence; severity alone does not.**

| Class | Consequence for the 1.0.0 call |
|---|---|
| **Data loss or corruption on any path** — bytes destroyed, unrecoverable, or a doc silently written wrong | **BLOCKS.** No exceptions, no "recorded as known". A wave absorbs it and 1.0.0 waits. |
| **A regression** — something that worked on rc.12 and does not on rc.13 | **BLOCKS**, unless it is one of §0's **declared** behaviour changes and the trial confirms it reads as designed, judged against the four-part standard in §5 rather than impressionistically. |
| **A discoverability landing** (§3's measurement returns *filesystem* or *neither* under duress) | **BLOCKS THE CLAIM, and forces a decision the trial cannot make.** Not a bug — evidence about VISION principle #3. See §3.5. |
| **A blocking dead end** — a refusal whose route cannot run, or a state with no recorded recovery | **BLOCKS** when it can reach a **project-carrying** file. **SHIPS RECORDED** when the reachable set is incidental and the state has a correct recovery elsewhere. |
| **A wrong result on a non-destructive path** — a check that does not fire, a false green, a wrong machine-readable value, a panic | **BLOCKS** if it produces a false green over managed state, or violates a pinned `--format json` contract. **SHIPS RECORDED** otherwise. |
| **A surface/wording finding** — a lie, an ambush, a missing route on a non-blocking path | **SHIPS RECORDED**, listed in the 1.0.0 record as a known bound — *unless it is a one-way door.* |
| **A capability gap** — "I wanted a verb that does not exist" | **SHIPS RECORDED.** M46's razor refused nine of these with citations; more are expected. |

**The M50 rider, new in this trial.** Every SHIPS-RECORDED finding is *also* marked
**cheap-now / expensive-after** — whether fixing it after 1.0.0 would mean a migration on a
released binary, a contract v2, or a changed stable key — because that is M50's admission
criterion and the reason this trial runs before it. The mark is made from the evidence, with the
one-way-door qualifier below as its test.

**The blast-radius qualifier on the dead-end row.** Keyed on the **reachable set**, demonstrated,
not on which file the reporter hit; the class is fixed from evidence **before** its consequence
is looked up.

**The one-way-door qualifier.** A finding that ships recorded must be *reversible after 1.0.0*. A
defect in a pinned `--format json` contract is not — M48 closed the additive-key window, M46
added `unadopted[]`/`unfilled[]` under it, and M49's `already_absent` and `schema-version` were
declared *as* the pre-1.0 additive spend. If a SHIPS-RECORDED finding would freeze a wrong
contract, it **BLOCKS** instead.

**Two rules that bind the adjudicator:** a finding's class is decided from its evidence before its
consequence is looked up; and *"judged not to matter"* is not a disposition — every confirmed
finding lands in a row above, or the table is revised **in writing, with a reason**, before the call.

---

## 2 · Instruments

**Three scored blind sessions on the headline shape, two further headless blind sessions, one
operator-scripted walk, one unscored adopter-condition arm.**

**Every session runs filesystem-isolated** — [trial-harness/README.md](../../trial-harness/README.md).
The image is `jigc-gate:rc13`, built from `f266770`, gated by [gate-rc13.json](gate-rc13.json):
`verify-image.sh` **7 passed / 0 failed**, `run.py gate` refuses a round the record does not cover.
The rc.12 side of the migration pair is the existing `jigc-gate:rc12` (`5ff85ea`), and the pair
was proven **two distinct trees** on the new `m49` behavioural probe set, **3/3 discriminating**,
and refuses a same-image pair (§10).

**Invocation log ON in every session** — pre-enabled on adopted corpora by `arms/adopt.sh`; named
in B1's prompt only, because B1 installs jigc itself.

**Fresh corpora, one per session, never reused.**

### 2.1 · Transport per arm — the human's call, and its basis

The human decided 2026-09-04: **B1 and B2 interactive, B3 headless, and more headless runs where
they buy something.** That discharges the two arms RC-1.0-final carried as owed (its §2.1 revision
ran both headless for lack of an operator): an interactive B2 keeps the transport from moving
under the headline, and an interactive B1 is the **only** way plant F's correction is delivered —
`doc rename` on a staged doc has been reached by nothing in two trials.

| arm | corpus | plants | transport | why this transport |
|---|---|---|---|---|
| **B1** cold start | `larkspur` (naive) | hook (**F**) + carryover | **interactive** | plant F's design is *let a plant open the door for an utterance*; `-p` has no door |
| **B2** design altitude | `quillon` (adopted + E) | **E** | **interactive** | the headline; the transport must not move under it |
| **B3** corpus accretes | `ashgrove` (adopted + E) | **E** + foreign-ADR | **headless**, default permissions | plant-driven, needs no utterance; two archived interactive runs of this prompt exist for comparison |
| **B3-h2** | `brackenmoor` (adopted + E) | same as B3 | **headless** | a second headless reading of the duress cell: the headline at **N=3 across two transports** |
| **B4-h** plan the first milestone | `saltmarsh` (adopted) | none — an observation arm | **headless** | reaches the M49 cluster no blind session has met: `planning` → `planning-record`, `milestone create/add-task/provision/execute/finalize`, `milestone-record` v3, the `Spawn:` line; a human buys nothing here |
| **B3-strict** | a copy of `ashgrove` | same as B3 | **headless**, `--strict-permissions` | the adopter's real condition; **unscored and labelled so** (RC-1.0-final §2.1's reasoning stands: a denied payload write halts headless with nobody to ask) |

**B4-h is scored on the §3 channels but is not part of the duress headline** — it carries no plant
E. Its scored cell is *the read-back series* (§3.4), and its value is coverage: 0.3, 0.4 and the
milestone doors under a blind worker.

### 2.2 · The corpora

All from [trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`, gated
**11/11** by `check-corpus.sh` before adoption, adopted through **the container's own binary**
(`run-session.sh --exec arms/adopt.sh … jigc-gate:rc13`), carried out with `run.py carry`, then
planted. **Order is fixed: instantiate → gate → adopt → plant.** Names are product-plausible with
no `rc`/`trial`/`probe`/`gate` token; the walk corpora (`walk-m50`, `walk-m50-mig`) may say what
they are. [corpora.md](corpora.md) records each frozen state.

### 2.3 · The rehearsals are a precondition

- **R3 · plant E against a live agent on rc.13** (`r3-corpus`, headless, the rehearsal prompt).
  Both instruments must fire and be consumed; the plant's 11 bars must pass **inside the
  container**; the invocation log must be empty at freeze.
- **R4 · plant F's stop on rc.13** (`r4-corpus`, headless, B1's prompt). Does a worker meeting
  the rejecting hook **stop and ask**, or self-serve the marker? R2 answered *stops* on rc.12,
  n=1. If R4 shows the stop unreliable, **B1's utterance arm converts to observation-only** —
  a pre-registered branch, not a salvage.

---

## 3 · The headline measurement

Carried from RC-1.0-final §3 without change of construct, so the duress result is comparable:
**3A the channel under duress** (plant E — an open task the worker cannot land without reading
what it holds) and **3B consequence, not occurrence** (the `status: superseded` discrepancy
visible only in the read-back, with exactly one sanctioned repair, `set-field … --value accepted`).
Three pre-registered 3B outcomes, all scoring: **acted** · **noticed and declined** · **did not
notice**. Plant E's four falsifiers and their mitigations are unchanged
([e-abandoned-task.sh](../RC-1.0-final/plants/e-abandoned-task.sh) is reused byte-identical).

### 3.4 · What is recorded — the channels and the four settlements, carried

Per session, from the invocation log and the transcript, by `run.py observe`:

| Outcome | Evidence |
|---|---|
| **VERB** | `jigc doc show … --task …` appears in the invocation log |
| **VERB-ADJACENT** | `jigc task diff <id>`, `jigc doc list --task …`, or `jigc task validate <id>` |
| **FILESYSTEM** | the transcript shows a direct read of `.jigc/**` or a managed doc path — reported split into **DOC** (a managed `.md`) and **wkbn** (workbench bookkeeping) |
| **NEITHER** | the worker proceeds without reading |

Settlement 1: VERB counts **attempts**; `VERB-effective` (exit 0) beside it; **`attempts > 0 /
effective == 0` is a finding**, never a VERB success. Settlement 2: `task validate` joins the
adjacent set, a bare `doc list` leaves it; `driver/channels.py` is authoritative. Settlement 3:
records before `PROVENANCE.txt`'s `session-start` are excluded and reported; the plant clears the
log as its last act. Settlement 4: DOC and wkbn are different acts and are reported separately.
Two numbers per session: reads **at any point** and reads **at/after the plant's state is
reachable**. **Do not rely on the worker's own account.**

### 3.5 · The reading, and the control that makes a null readable

**Walk arm 00 runs FIRST.** If the control does not fire, no blind result may be read.

Over the duress measurement, **N=3 scored** (B2 interactive, B3 and B3-h2 headless):

- **3/3 VERB or VERB-ADJACENT** — compliance at N=3 across two transports, never reliability.
- **VERB-ADJACENT instead of VERB** — reported explicitly: read through jigc by a verb no step named.
- **2/3** — partial; blocks the claim, escalated with evidence.
- **≤1/3 with FILESYSTEM, control fired** — the adapter broke for documents under duress. **Escalate
  to the human with the evidence; do not adjudicate inside the trial.**
- **0/3, control did not fire** — apparatus failure, not a result.

**The non-outcome rows** (voided session · log never enabled · no authoring occasion · halted
awaiting a human — *a halt looks exactly like success* · VERB *and* FILESYSTEM) are unchanged and
implemented as voiding rows in `driver/cascade.py`.

### 3.6 · The registered outcome table — order included, and machine-checked

`driver/cascade.py` grades every session by walking this list top-down, first match wins, and
`cascade.check_registration()` parses the table **out of this file** and compares it to the code
position by position; `test_cascade.py` runs that comparison over every `RC-*/protocol.md`.

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
and says so; registration state is not in the transcript, so a read of a foreign never-adopted
file at a managed home needs a human call.

---

## 4 · The blind sessions

Clean-room rules throughout: **verbatim prompts, no operator hints beyond the answer key,
per-session feedback report, invocation log ON.** The preload split of RC-1.0-final §4 stands:
B1 runs `setup` in-session, so the adapter is not in its context and its read-back is reported
**separately and discounted**; every other arm begins adopted.

**B1 · cold start (interactive, `larkspur`).** `jigc setup` → first task → first ADR → finalize.
The **carryover plant** (two paths staged before the first mint: an added script, a modified
router) and the **hook plant** (a rejecting `pre-commit` under `core.hooksPath`, refusing any
commit that touches `docs/`). **Plant F rides the hook's pause**: the correction is appended to
the docs-gate answer-key reply, using [plant-f-correction.md](../RC-1.0-final/plant-f-correction.md)'s
template and its same-slug pre-send check. Prompt: [paste/b1-prompt.txt](paste/b1-prompt.txt).

**B2 · the design altitude (interactive, `quillon`).** Entered through plant E's abandoned task
(an adr under a wrong title, `status: superseded`), then `do-research` → `form-vision`. Prompt:
[paste/b2-prompt.txt](paste/b2-prompt.txt). Rider C stays dropped ([RC-1.0-final/corpora.md](../RC-1.0-final/corpora.md)).

**B3 · the corpus accretes (headless, `ashgrove`) and B3-h2 (`brackenmoor`).** Plant E, then
changelog → spec → implement → arch-doc, with the **foreign-ADR plant** landed by the poller on
`git rev-list --count $INSTALL..HEAD >= 2`. Prompt: [paste/b3-prompt.txt](paste/b3-prompt.txt).

**B4-h · plan the first milestone (headless, `saltmarsh`).** No plant. The prompt asks for a
roadmap, a planned first milestone with its planning record, and that milestone executed to a
boundary. Prompt: [paste/b4-prompt.txt](paste/b4-prompt.txt) — screened like the others. It is
the one arm whose *scope* is M49's new surface, and its whole value is what a worker does with
fourteen required gates and a fan-out it did not design.

**All must reach the guides** (`.claude/skills/jigc/SKILL.md`, adapter-owned) — do not seed them,
do not report use as discovery.

---

## 5 · The operator-scripted walk

Every arm is a script under `completions/trial-driver/arms/walk/`, driven by `walk.py` through
`run-session.sh --exec`, one block per arm in the record **including the ones that did not run**.
Conventions per that directory's README. **Each arm names the kind of set it iterates.**

| arm | what it drives | the set | M49 |
|---|---|---|---|
| **00** | **the positive control — runs FIRST** | — | — |
| **01** | the identity surfaces at a bound role | — | regression net |
| **02** | the four destroying doors + the `--ignored` axis, **extended**: `uninstall` over a non-directory leftover (the audit's `is_dir()` hole) | `DESTROYING_DOORS` | audit |
| **03 / 10** | the rc.11 → rc.12 pair — **SKIP loudly** on this pass (wrong binary), kept so the record shows them skipped rather than absent | — | — |
| **04** | the foreign arm: `validate`'s exit flip, `unadopted[]`, the return to 0 | the declared M46 change | regression net |
| **05** | the milestone boundary over own / `cp -R` / `mv` linkage | the class's defining case-set | regression net |
| **06** | the pre-guard repair routes | a derivation with a stated hole | regression net |
| **07** | the changelog gate at `task validate`, both severities | the declared M46 change | regression net |
| **08** | T10 / T11 identity refusals, the idempotent no-op | the postmortem's T7–T11 | regression net |
| **09** | Increment 8 (M46) doors | hand-enumerated, said to be one | regression net |
| **11** | the item-region class on **shipped** doctypes: `--unset` of an absent field acks; a repeated field bullet reddens `task validate`; slot prose at the prescribed depth lands; `add-item --slug`, the enum-id refusal, the `--slug <id>-N` route run verbatim | a derivation; the 18-cell cube **declared test-fenced** | Inc 1 + audit 2 |
| **12** | the doors that lie: `add-task --workflow <unknown>` before the mint; a sub-task `discard` reaching the committed record; `list-tasks` writes nothing on a fresh clone; a before/after `.jigc/` census over every read verb | `VerbKind::Read` (12) | Inc 2 |
| **13** | the freeze at every layer: a section-dropping project shadow blocks every door by name; a presentation-only shadow loads; **`setup` exit 0 judged under the four-part standard**; a mis-keyed leaf inside `repeatable:` is refused, not erased | two layers, a stated derivation | Inc 3 |
| **14 → 21** | **the migration pair**: 14 authors on **rc.12** (adrs · research · changelog · a `squash: false` fan-out whose sub-task commit doc lacks `type` — landed at exit 0 · a `completion-record` refusing `HIGH` · the two stamps) into `.upgrade-baseline`; 21 continues on **rc.13** — `validate` non-zero + `schema-version-current` routed at `migrate-corpus`, `store-version.binary-mismatch` naming both; `migrate-corpus` lands **stamp-only** 1→2 and 2→3; `HIGH` exits 0; the boundary now **blocks** on `type`/`summary` with a `--task <sub-id>` route, filled → one conventional commit per sub-task; `setup` after, SKILL.md stamped rc.13 | the three bumps + `d854e25`'s author-required leaf set, read from `doc schema commit` | Inc 4, 9, `d854e25` — **§0.5 and §0.6's standard lives here** |
| **15** | `placement-root`: a dir re-roots the committed roadmap via `git mv`; root-declared `VISION.md` never re-roots; `""` → `.`; **`.git` refused** (the audit HIGH — `git mv` into `.git/` exits 0 and loses the doc from every clone); a hand-stranded placement doc reported by `validate` | the user-settable doors; the 8 mover sites **declared test-fenced** | Inc 7 + audit 1 |
| **16** | the pinned contracts: N1 at four item doors (`key.target` verbatim); N2 staged, committed, `null` for `commit`, slices and `doc list` untouched; D1 located text; the gate route carrying `--task` with two open tasks, run verbatim | `DOCTYPE_DOORS` address rows, fenced by flow50 | Inc 8 |
| **17** | **the empty-id axis** — every id-taking door with `""` and with `no-such-task`, on the **release** binary. **Expected RED on rc.13** — a measurement for M50, not a regression check | hand-enumerated from `--help`, said to be one | PT-1 |
| **18** | the surface batch: one unknown-doctype answer at create-gate vs store doors, no Debug leak; not-a-git-repo once; `write.unknown-section` at four write verbs; `rename`'s refusals with code + exit; non-UTF-8 argv no panic; `describe --commands` union; **S-1 / S-4 from the last trial confirmed discharged** | `DOCTYPE_DOORS` (15), `RefusalKind::ALL` | Inc 11 |
| **19** | `planning-record`: the gate set read **through the binary**; one gate missing blocks `task validate` and `task finalize` naming it, commits nothing; filled → lands at `planning-records/` | the schema's own gate set | Inc 9 D9 — **§0.3's standard** |
| **20** | PB-1: a listed project pack adds a doctype end-to-end; a frozen doctype in the same pack is **demoted**; `--explain` names the winner | a 3-pack composition | Inc 6 |

**The standard for a declared behaviour change**, fixed here (§1 row 2 defers to it): a declared
change **reads as designed** only if (a) the output **names what it objects to**; (b) it **says
the consequence**; (c) it **names the route or consent** in the same output; and (d) that route,
run verbatim, **works**. Any of the four missing, it reads as obstruction and is a finding. §0.7
(`setup` exit 0 over a shadow) is judged against this standard by arm 13 and its verdict is
recorded whichever way it falls.

### 5.1 · `verify-pair.sh`, the `m49` set — done

Extended 2026-09-04 with `PAIR_PROBES=m49` (now the default, `EXPECT_OLD_SHA=5ff85ea…`), three
behavioural probes each stating both sides — an unknown `--workflow` at `add-task` (exit 0 →
non-zero) · the top-level `schema-version` key on `doc show` (absent → present) · the `pack` key on
`describe --commands` (absent → present) — plus the two items decisions-pending had time-boxed
before the release: the `"jigc 1.0.0-rc."*` liveness match that would have sent a released `jigc
1.0.0` to the failure arm, and the `awk '/^  [a-z]/'` that dropped digit-leading slugs and fed `""`
into a probe (the reader that found T1-a). Driven: **3/3 discriminate on rc12 → rc13; a same-image
pair is refused (exit 1)**; the `m46` and `m48` sets are kept.

---

## 6 · Coverage — every changed surface in exactly one column

M47's rule over the M49 + `d854e25` diff, in [coverage.md](coverage.md) after the sessions:
**trial-reached** · **test-fenced (naming the suite)** · **neither** (each explained). Derived from
the registries (`VERB_KINDS`, `DOCTYPE_DOORS`, `MINT_DOORS`, `SetKind::ALL`, `DESTROYING_DOORS`)
and flow50's declared proof split — never from a changed-file list.

**Stated up front as not trial-testable, so no arm is invented for them:** Increment 12 (goldens,
ledgers, `MINT_DOORS` — a registry with no verb, finding or route); the 18-cell item-region shape
space and its slot-position dimension (`item_region_shape_space.rs` — a manufactured pack, not a
shipped doctype); the eight `location`-concatenation and eight mover sites of audit fixes 1 and 7
(`placement_override.rs`); the 47-door freeze table (`freeze_enforcement.rs`); the `doc author`
batch performance (no bytes move); and the un-markered M14 pack path, minted as a deferral with a
trigger at M49 rather than closed.

---

## 7 · The conversion obligation

Every CONFIRMED *and* REFUTED verdict arrives with a repro block ([pinning.md](../../../implementation/pinning.md)
§3) and a `pinned-by: <suite>::<test>` verified **by reading what the cited test asserts**, or a
stated `UNPINNED: <why>`. No mechanical checker fences this; §3 refuses one by name. **The human's
gate is unchanged: no 1.0.0 call until the ledger is closed.** A REFUTED finding is expected — six
consecutive trials produced complaints that dissolved into shipped capability, and that pattern
is the discoverability signal.

The driver does not grade findings.

---

## 8 · Operational rules carried forward

1. Back-date a plant's commit, not just its body.
2. **Keep [operator-log.md](operator-log.md) contemporaneously** — every utterance into an
   interactive session, verbatim, as it happens, with its justification. The `bypassPermissions`
   confirmation at session start is an operator touch and goes in the log.
3. Commit an operator plant with an explicit pathspec.
4. `docker exec` needs `-u node`; `docker cp` preserves the host uid.
5. `run-session.sh` exits 0 on a failed arm by design; the arm's code is in `PROVENANCE.txt`.
6. **A plant must not write to the channel its arm is scored on**, and if it must, the reader has
   to know when the session began (settlement 3).
7. **Reproduce in a copy, never in the live corpus** — an operator probe against a live container
   is invisible to the session-start split (RC-1.0-final's owed item 5, still open).

> **The contamination rule, absolute:** no operator utterance may contain *verify*, *read back*,
> *check the doc*, `doc show`, or any synonym. If an exchange drifts toward it, **log the drift and
> mark that session's measurement void**. `interact.screen()` enforces a wider list at answer-key
> **load** time.

Blind prompts are operator utterances and are screened before they are frozen. The narrow channel
statement — *"the project's docs are managed with jigc, so the thinking goes in through it"* — is
permitted and is a declared bound: a positive result is compliance plus an operator channel
preference, never unprompted tool preference.

---

## 9 · What this trial cannot answer

- **Adopter-side freeze protection** — the manifest fence guards this repo, in CI.
- **Multi-process concurrency semantics** — one operator, one session at a time.
- **Whether a narrated ignored-path teardown loses work** — the loss is visible, not prevented, by
  M46's measured refusal; an arm cannot supply an adopter's report.
- **Most regressions** — only the migration pair touches rc.12; blind sessions carry no baseline.
- **The measurement under bypassed permissions** — a declared directional confound; a VERB result
  is not weakened by it, a FILESYSTEM/NEITHER result is partly attributable to it. B3-strict runs
  the real condition, unscored.
- **Whether headless generalises** — the paired transport result is still n=1 per corpus shape;
  this trial adds one more paired cell (B2 interactive vs B3/B3-h2 headless, different prompts),
  which is indicative, not a control.
- **Whether the isolated environment is representative of an adopter's** — right for attributing,
  wrong for predicting a day.
- **Long-horizon drift.**
- **The item-region cube on shipped doctypes** — the shipped registry populates 2 of 18 cells; the
  corpus was safe by accident of shape and stays so.

---

## 10 · Readiness — what is discharged, and what is still owed

**A plant assumed to fire is not a plant.** Each row says how it was verified.

### Discharged before any session

| item | how it was verified |
|---|---|
| the handover | every row driven or read; two corrected on the record ([handover.md](handover.md) → Verified; [pre-trial-findings.md](pre-trial-findings.md)) |
| the rc.13 image | built from `f266770`; `verify-image.sh` **7 passed / 0 failed**; [gate-rc13.json](gate-rc13.json) written and `run.py gate` accepts it |
| `verify-pair.sh` | `m49` set: **3/3 discriminate** rc12 → rc13; a same-image pair refused at exit 1 |
| the corpora | nine instantiated with `--clean-prose`, each **gated 11/11** before anything else touched it |
| B1's plants | `b1-hook.sh` + `b1-staged.sh` on `larkspur`: 8 commits, `core.hooksPath=.githooks`, marker absent, `A scripts/retention-sweep.sh` · `M src/router.ts` staged |

### Still owed, in order

- adoption of the five adopted-shape corpora through the container's binary, then plant E on
  `quillon`, `ashgrove`, `brackenmoor`, `r3-corpus` (11 bars inside the container each);
- the walk arms 11–21 and the 02 extension, each driven through `run-session.sh --exec` before it
  is trusted; the arms README updated;
- the B4 prompt written and screened; the answer key copied, its `scope` pattern widened, a
  planning-gates entry added, screened at load;
- **R3 and R4** run and recorded;
- **walk arm 00 first**, then the sessions; then the rest of the walk; then the migration pair;
- the findings-verification pass, [coverage.md](coverage.md), the record, M50's scope brief.
