# Delta report — the recovered original 2026-07-25 analysis vs. the committed RC-alpha4 reconstruction

**Sources compared**

| role | path |
|---|---|
| original (lost `ad54751`, recovered verbatim) | `recovery/original-trial-record.md` (66 ln), `recovery/original-findings-verification.md` (375 ln) — copied unmodified from `analysis/files/`; the writes-journal shows each was written exactly once (07:12:06 / 07:17:20) with no later Edit, so these are the final committed bytes |
| committed reconstruction | `completions/artifacts/RC-alpha4/{trial-record,findings-verification,recovered-analysis}.md` (repo `e2f28f7`) |
| supporting | `analysis/bash-ledger.md` (the live 504-record log analysis), `analysis/narrative.md` (5356 ln), `analysis/patches/`, `analysis/asks.md` |

**Headline.** The two findings analyses converge (as `recovered-analysis.md` already claims) — **one true verdict conflict, four same-verdict factual contradictions**. The *large* delta is not in the findings at all: the committed `recovered-analysis.md` captured only the **first-pass** rc.10 recommendation (the 18:05 turn) and missed **two superseding human turns, the persisted charter commit `a822f87`, and an entire M47 planning session that ran to a completed Settle.** Section (d) is the substantive part of this report.

---

## (a) Claims the committed reconstruction lacks, or states differently

### A.1 — VERDICT CONFLICT (one, genuine)

**The "author may be run only once per task" rule.**

- Original, `P3-2`: *"The 'author only once per task' reading is **PARTIAL**: a second author against the same staged doc **rejects**, but iteration continues freely through set-slot/set-field/add-item."*
- Committed, `B2`: *"**REFUTED**: no one-shot rule exists on rc.9; a second `doc author` succeeded on every path tried"* — including *"path 3 — the identical author payload run twice on that same fresh doc … `EXIT=0`"*.

Reconcilable in principle (the original was probably reasoning about an **items**-bearing doc, where a re-authored payload collides at `write.already-present`; the committed path 3 used an `adr`, which is slots/fields only) — but **as written the two documents give opposite verdicts on the same sentence of `doc.rs:196-199`**, and the committed one is the live-driven one. No action beyond recording the original's PARTIAL as superseded.

### A.2 — Same verdict, contradictory supporting fact (four)

1. **`P1-1` / `D2` — does the sibling ladder state the root?**
   - Original: *"The completion-workflow sibling **does** say 'repo-relative' (`author-completion-record.yaml:6-11`)"*.
   - Committed: *"**no surface says repo-root-relative**: not the ladder, not `jigc doc schema completion-record`, not either block message. Grep of the whole methodology pack: the only 'repo-root' statements attach to *other* doctypes' comments."*
   These cannot both be true of `packs/methodology/steps/author-completion-record.yaml`. The committed claim is the live-grepped one; the original's is the fix-shaping one ("the statement exists in one step and not its sibling" — the same pack-axis shape as A.2.3). **Worth one grep to settle before M47 builds the fix.**

2. **`P2-6` / `C1` — why the address-scoped read "didn't exist".**
   - Original (log-grounded): *"The P2 log segment (records 278–296) contains **no `doc show` with a fragment at all**: the worker guessed the grammar would be the bare item id, never ran it, declared the capability absent, and fell back to `sed` ranges."*
   - Committed (inferred from feedback): *"What P2 tried fails because it **elides the section segment** … `deferral-ledger:deferral-ledger#creator-wizard-state-lives` puts the item id in the section position"*, dead-ending at `store.no-such-section`.
   **The surviving bash-ledger settles this in the original's favour**: the verbatim P2 dump (records 278–296) shows exactly three `doc show` calls — `deferral-ledger:deferral-ledger`, `arch-doc:application-architecture`, `adr:keep-creator-wizard-state` — **all bare whole-doc, zero `#` fragments**. So the committed C1's central reconstruction ("the trial's address elided the section segment") describes a command the worker never ran. The confirmed residue (route quality) survives in both; only the mechanism differs. **Fold back.**

3. **`P2-1` / `D9` — root cause of the invisible commit doc.**
   - Original: a **pack-axis** miss — *"the methodology pack's own finalize step **does** carry the commit-doc line (`packs/methodology/steps/finalize.yaml:12`) — but body references resolve per-owning-pack (`pack.rs:1106-1113`), so dev-pack workflows get the dev finalize step, which lacks it."* Fix: the dev finalize step gains the line.
   - Committed: a **workflow-shaped** gap — *"`single-task` composes explicit `set-commit-type`/`set-commit-summary` steps — so the gap is `record-decision`-shaped."*
   Different diagnosis ⇒ different fix. The later M47 baseline audit proved the original right *and wider*: **five** code-less dev-pack workflows (`plan`, `architecture-documentation`, `record-change`, `record-decision`, `project-setup`) *"compose a finalize their own gate refuses, because the dev pack puts the commit-doc writes in `step:implement`"*. **Fold back — the committed diagnosis under-scopes a Tier-2 fix by 5x.**

4. **`P5-4` — the "good route" sharpener is wrong in *both*.** The original wrote *"the field-group-absent miss on the same verb DOES carry the good route."* The M47 baseline audit refutes it: *"P5-4's premise is wrong in the findings-verification. The 'field-group-absent miss gets the good route' case isn't a miss — it **succeeds**."* The real split is `set-field --value` → `wrong-shape` dead end vs `set-field --unset` → `write.not-present` **with `<address>`/`<task-id>` placeholders unsubstituted**. Neither committed doc carries either version. **Fold back the correction, not the original.**

### A.3 — Original rows with **no counterpart at all** in the committed findings-verification

| original row | verdict | substance |
|---|---|---|
| **H4** | CONFIRMED (field + repro) | The ahead-stamp surface **completed by repro on the two doors no worker walked**: `migrate-corpus` blocks by name with the restore-from-history route (dry-run + real, exit 1, never "current"); `set-field` refuses the forge (`write.machine-maintained-field`). Plus a **REFUTED sub-fear** with cites: *"`reconciliation.absorb` of the planted doc **cannot mask** the ahead detection — the store-scope check re-parses committed bytes, never the file-state record (`validate.rs:891-895`; absorb re-baselines the *drift* hash only, `file_state.rs:213-219`)."* |
| **P1-8** | CONFIRMED | **The drift hook cries wolf.** *"The installed hook greps the validate JSON for `\"probe\": \"doc-code\"` only (`setup.rs:194-196`) — severity is never inspected … it fired on four consecutive P1 commits."* Later escalated by the M47 baseline: any non-Rust project following `implement-from-spec` gets *"a permanent false 'drift detected' on every commit"* — self-inflicted by the tool's own routed flow. **A charter Tier-2 item with zero trace in the committed archive.** |
| **P2-8** | CONFIRMED (capability) | No deferral↔ADR settlement path: `supersedes` is adr→adr, no `resolved-by` relation, and `record-decision` — *"the workflow most likely to invalidate a deferral"* — never sweeps contradicted ledger entries. M46 Settle input. |
| **P4-6** | CONFIRMED | **Editing a committed doc from a task stages it silently.** `read_or_copy_in` (`doc.rs:3542-3572`, six call sites); the ack says only `set <addr> = <value>`; only `doc create` states it (`existed: true`). *"The M43 statement was an incomplete fix over the verb axis: the create door tells the truth; the five edit doors stay silent."* Charter Tier-2 item ("copy-in acks on the five edit verbs"). Mentioned in passing in the committed **trial-record** but never verified or rowed. |
| **P5-4** | PARTIAL + CONFIRMED | Wrong-item-id `set-field` → `write.wrong-shape` with a **type-level** `doc schema` route that cannot reveal instance ids; **and `write_not_present_route.rs:20-22` pins the exclusion as intended** — *"the trial is the field evidence the pinned intent is wrong for this case."* **A charter Tier-1 (now-or-never, stable-key) item with no row in the committed archive** — it appears only as the incidental scenario inside C7. |
| **P3-7** | CONFIRMED, known-tracked | Planning's human-gated Settle walked with **no record it was walked** — 4th independent demand. (Captured in `recovered-analysis.md`, not in findings-verification.) |
| **P4-2** | CONFIRMED | *"touches no documented code"* undefined — the worker's inference (grep the docs) is precisely what the missing search verb would answer. |
| **P3-8** | CONFIRMED (trivial) | `set-slot` outside the repo errors correctly but could resolve the repo from `--task`. |
| **P4-7/8 residue** | CONFIRMED | `doc list`'s three columns are **headerless** (`item-count` silently JSON-only); `create-gates:` appears in the footer **defined nowhere** (`render.rs:284-289`). Committed `C3` covers the anchors/`--addresses` half only. |

### A.4 — Where the committed reconstruction is **stronger** (nothing to fold back)

- **`A5` sharpens `H2`**: the original stopped at "re-run says already current, nothing committed"; the committed adds the live-proven **false green** — *"`jigc validate` … `no findings — the committed store validates clean` EXIT=0 … the sweep adjudicated the on-disk bytes; the message claims the COMMITTED store, whose copy is still stamped 1"* — plus the wrong-for-this-state carryover route. Genuinely new.
- **`A4`, `D7`, `D12` have no original counterpart.** D12 resolves the charter's dangling `P1-4` *and* finds a sharpener the original never had: `gates_at_task` never consults cascade severity, so the mixed store trailer claims a gate for `title-names-symbol`, which sits in an exit-0 `task validate`.
- **`B7`** names the slug mechanism (renormalize → 5 words → 50 chars → edge-stopword drop) where the original only reported the symptom.
- Original `P2-5` adds one thing worth carrying: the claim class *"was already pinned"* by `pinned_facts/finalize_json.rs`, and the **non-empty-`left_out` arm is the unpinned half**. The M47 baseline later corrected even that: the stdout arm **is** pinned (`finalize_manifest.rs:313`); the **stderr** half is what's unpinned.
- Original `P1-14`/`P5-8` carry a fact the committed `B6` drops: **`task diff` ignores `--format json`** — *"the one task verb outside the envelope."* This became a **charter Tier-1 item**.

---

## (b) Log-derived facts in the original / bash-ledger not in the committed `recovered-analysis.md`

The committed `recovered-analysis.md` preserves segmentation, exit inventory, finding-code inventory, output outliers, the 503/504 purity verdict, the expected-signals audit, and the truncation datum. It does **not** carry:

1. **The whole-log verb-family histogram** (original trial-record): `doc` 295 (`set-field` 96 · `set-slot` 87 · `show` 34 · `author` 26 · `schema` 17 · `add-item` 14 · `create` 8 · `list` 7 · `retitle-item` 3 · `remove-item` 1) · `task` 76 (`finalize` 41 · `list` 20 · `validate` 14) · `validate` 48 · `start` 42 · `migrate` 21 · `workflow` 10 · `ingest` 4 · `migrate-corpus` 3 · `describe` 3 · `upgrade` 1.
2. **Per-probe verb histograms and exact spans** (bash-ledger 06:54:50) — e.g. P1's `set-slot` 49 / `set-field` 46 / `validate` 35 / `finalize` 29 / `start` 23 / `migrate` 21 / `author` 20 / `task list` 20; full log span `2026-07-25T04:16:25Z → 06:49:02Z`.
3. **The segmentation *method* and its evidence**: four inter-record gaps >8 min at indices 278 (8.1 min), 374 (16.2), 430 (8.4), 448 (9.1), cross-confirmed by bare-`start` orientation calls and the commit timeline.
4. **The P2 zero-fragment proof** (see A.2.2) — the verbatim 278–296 dump. This is the evidentiary basis of a REFUTED headline and it contradicts the committed C1's reconstruction.
5. **`Store validate: 48 runs, 8 exit-1`** — every one carrying `schema-version-ahead` post-plant; zero false greens across three sessions.
6. **The nonzero-exit record index** with argv, incl.: record **9** `migrate-corpus` exit 1 `error_code: null` (H2's log-anonymity, proven); records **62/64** the owner-artifact double-block at 04:24; record **434** `finalize.commit-rejected` at 06:18:57 → clean re-finalize 06:20:16 (**80 s** recovery); records **138/149/165/186** the exit-2 usage errors from scraping `task list`'s one-line format; records **201–206** the mangled two-line id (an embedded newline visible in argv) — five consecutive exit-1s at 04:48:02; record **151** the wrong-hash paste rejected loudly (M44 path-hash working).
7. **P4's ADR read timing**: record 347 `doc show adr:audit-runs-on-puppeteer-and` at 05:33:17 vs. the mint at 05:33:51 — **34 seconds before the task existed**; and records 349/350, the two `--preview` calls (`single-task`, `decided-task`) that burned a round — incidentally the only field evidence of the M43-unhidden `decided-task` being consulted.
8. **The commit inventory**: **35** commits in the window — **33 through `jigc task finalize`**, 1 worker hand-commit (`a9731ef`), 1 operator plant (`d738126`, 07:32:04+02:00 = 05:32:04Z — the same second as record 343). P1 = **20 adoptions** (`810644f`…`8ba1c6e`); the full migration-commit list is in the bash-ledger. P5's milestone code sub-tasks: Node pin · ESLint config · docker image bake · MariaDB test switch.
9. **Honesty-statement item 5**, present in the original trial-record and in neither committed doc: *"**P3's prompt** carried an operator-authored instruction beyond the protocol's illustrative example (the worker refers to 'a document you had explicitly told me to leave untouched') — operator-authored, not archive-derived; noted for completeness."*
10. **A count discrepancy the committed set inherited.** The bash-ledger's first pass segmented `P3 = 297–343 (47 records)`; the original trial-record **corrected** it to `P3 = 297–342 (46 records)` once record 343 was identified as the operator's. `recovered-analysis.md` and the committed trial-record carry the **uncorrected 47**, which is internally inconsistent with their own 503/504-purity claim.

---

## (c) Numbering that later documents reference but the committed set cannot resolve

The original used **`H1–H4` + `P<probe>-<n>`**; the committed reconstruction uses **`A1–A5 / B1–B7 / C1–C7 / D1–D12`**. `recovered-analysis.md` publishes a three-row crosswalk only (`H1=A1`, `H2=A5`, `H3=A3`). Everything else is unresolvable — and the documents that reference it are load-bearing:

- **The M47 charter** (recovered from `patches/0031_…decisions-pending.md.patch`, committed at the time as `a822f87`) keys **every tier-2 item to an original row id**: `H3 · P1-7 · P1-6 · P1-8 · P3-2 · P2-1 · P1-1 · P4-6 · P3-3 · P2-6 · P4-5/7 · P1-5 · P1-4 · P5-3 · P4-2 · P3-9`, and tier 1 to `H1 · P5-4 · H2`. Against the committed archive, **`P1-8`, `P4-6`, `P5-4`, `P4-2`, `P2-8`, `P3-7` resolve to nothing** (and `P3-9` only partly), while `P1-4` resolves only by luck (it is the committed `D12`, which the original never rowed).
- The original's own cross-references (`P5-1 → H3`, `P5-2 → P1-5`, `P5-9 → P1-1`, "see P5-4", "the same route-quality gap as P5-4") are unresolvable in the committed set.
- `recovered-analysis.md` cites `H1/H2/H3` sharpenings and triage datums with no row ids at all.

**Minimum fix: publish the full H/P ↔ A/B/C/D crosswalk** (this report's §(a) tables are it), or the M47 charter cannot be read against the surviving verification.

**Full crosswalk:** H1=A1 · H2=A5 · H3=A3 · H4=(none) · P1-1=D2 · P1-2=D10 · P1-4=D12 · P1-5=D1 · P1-6=D3 · P1-7=D4 · P1-8=(none) · P1-10=B3 · P1-11/P3-4=B4 · P1-12=B6 · P1-14=B6 · P2-1=D9 · P2-3=D11 · P2-5=A2 · P2-6=C1 · P2-7=C2 · P2-8=(none) · P3-1=D8 · P3-2=B1(+B2) · P3-3=D5 · P3-5=B5 · P3-7=(none) · P3-8=(none) · P3-9=C6 · P4-2=(none) · P4-3=D6 · P4-5/7/8=C3(partial) · P4-6=(none) · P5-3=B7 · P5-4=(none; scenario inside C7) · P5-5=C7 · P5-6=C4 · P5-7=C5 · P5-8=B6 · P5-11=(none; in recovered-analysis). Committed rows with no original counterpart: **A4 · D7 · D12**.

---

## (d) The narrative tail — everything after the rc.10 recommendation

This is the largest delta, and it is **not** a findings delta. `recovered-analysis.md` § "The recovered decisions" reproduces the **18:05:23** turn and stops. The session continued for another ~2h45m that day, and then ran a full M47 planning session the next evening. All of it is in the export.

### d.1 — Two superseding human turns the committed record misses

**18:14:23 (human):** *"agree we should only fix not add new functionality. We should change now what would break the system later or will be hard to migrate … When you say the most problems came from init you may should drive one small test for an new greenfield project and **we may do not need lacon**. What is your opinion"*

**18:15:34 (assistant, accepted):** applies the razor — *"Almost nothing in the findings is a true one-way door … the razor filters the wave down to the items that touch **pinned machine contracts**"*: (1) H1's fix (exit-code semantics on a contract-pinned verb), (2) **P5-4's route/code flip** (*"changes a stable `(code, target)` finding key … flip it now or accept the wrong code forever"*), (3) **`task diff` ignoring `--format json`**, (4) H2's error code. And: *"**drop lacon, run one small blind greenfield session on rc.10**"* — the H2-needs-brownfield objection answered with **deliberate plants** instead, because *"after five trials with zero correctness regressions … [lacon's] marginal value over the fixes' own pinned tests is low."*

**18:25:02 (human):** *"one thing we should also include wording changes, clarifications … stuff that makes jigc more understandable and better usable by the agent. So we put jigc out there with a strong fundament. — as you said we did only one real greenfield run and we may should include two ore three tests more so we can be shoure when people start with jigc on a greenfield project they do not hit a wall."*

⇒ **the wording tier becomes declared scope, and the greenfield trial becomes three probes (G1/G2/G3).**

> **This directly supersedes the committed record.** `recovered-analysis.md` item 3 and `DECISIONS.md` (`e2f28f7`, 2026-08-02) both re-instate *"the **lacon** re-drive as a fix-verification trial … fresh pre-jigc lacon copy … one migration session + one ordinary task session."* **Lacon was dropped by the human that same evening.** The repo's re-instated charter is the superseded first draft.

### d.2 — The persisted charter (`a822f87`) — recoverable verbatim from `patches/0031`

Committed 2026-07-25 18:26:50 into `implementation/decisions-pending.md` as *"The rc.10 wave (M47 — the surface-fundament wave; **executes before M46 and before the 1.0.0 call**) — CHARTERED 2026-07-25"*, with:

- **Tier 1 — contract-touching fixes:** H1 (+ its Settle fork) · **P5-4 finding-key flip** ("a stable-key change, now or never; moves the `write_not_present_route.rs` pin") · **the `task diff` envelope hole** · H2 swept over **every committing door**.
- **Tier 2 — the understandability batch as declared scope**, ~17 items each keyed to a findings-verification row (list in §(c)).
- **Tier 3 — the test conversion:** confirmed repro blocks → red tests; **refuted blocks → `pinned_facts/`** (address forms resolve · bare-path code-anchor accepted · the finalize-JSON non-empty-`left_out` arm into `machine_output.rs`); the two lie-pinning goldens regenerate **with** their fixes, never before.
- **The boundary, stated checkably:** *"the wave changes what existing surfaces **say**, never what surfaces **exist**"* — anything needing a new verb/flag stays at M46.
- **Acceptance — the three-probe blind greenfield trial on the installed rc.10:** **G1** cold start (zero-doc repo; plants = a rejecting pre-commit hook under `core.hooksPath` + pre-staged files before the mint) · **G2** the design altitude from zero (`do-research → form-vision → roadmap → first milestone`; the M37 trio's first greenfield-blind run) · **G3** corpus accretion from nothing (first changelog entry · spec → `implement-from-spec` · first arch-doc · one hand-authored foreign doc mid-stream for detect-and-route).

A matching `DECISIONS.md` entry (`patches/0032`) and a `CLAUDE.md` fold-back (`440ba45`, `patches/0033`) are also recoverable verbatim. The WHY-JIGC.md evidence fold the assistant proposed deferring was then done anyway at 18:34–18:35 (`patches/0001–0006`).

### d.3 — The M47 planning session ran, and reached a completed Settle

`recovered-analysis.md` states: *"The M47 planning/build sessions that followed … died with the machine unpushed and are recoverable only … from their surviving claude.ai session logs, if any. Until recovered, **M47's Settle outcomes and build state are unknown**."*

**They are in this export.** `narrative.md` lines 105–3760 and 3843–4700 contain the M47 session (18:36 → 20:57): handover, scope brief (`files/…M47-scope-brief.md`, 26 KB), a six-auditor baseline capability ledger driving the real binary, docs + capabilities gap lists (17 and 16 ranked items), three robust-advocate reports, the settle agenda (`files/…M47-settle-agenda.md`, 23 KB), and a **Settle that closed every fork**. No decomposition and no build: the session's last M47 turn is *"Next: the independent design review over everything Settle produced, before anything is decomposed."*

**Human decisions on record** (`asks.md`, two rounds):

| fork | human's answer |
|---|---|
| H1 scope | **Carryover + the 6 staging-independent owner-artifact causes** (not prose-only, not carryover-only) |
| N5 — slug drops the leading component of a hyphenated compound | **Fix now, at generation 3** (forces `SLUG_RULE_VERSION` 2→3, un-migratable by design) |
| schema hash freezing authored prose | **Narrow, and also build the same-version-re-pin fence now** |
| N1 — the four milestone record-only doors brick on one rejected commit | **Captured-pre-image rollback on rejection** |
| N6 — where the commit-doc writes live | **Move the writes from `step:implement` to `step:finalize`** (overruled the recommendation) |
| H2 error codes | **Mint one code per door** (overruled the recommendation; forces revising `ERROR_CODE_REGISTRY`'s "deliberately not a per-verb code mint") |
| N8 — pre-commit drift hook | **Filter to blocking severity** |

Plus the assistant's stated-and-accepted calls at 20:57:35: P5-4 swept over the whole axis (7 cells + `retitle-item` + the unsubstituted-placeholder family, and a new **P6 route-followability** fence); N3/N4 both guarded; N7 resume aligned to the overlap-aware rule (+ `storage.md:238` revised, + the `form-vision` step and the false parallelism footer); `task diff --format json` shape pinned as `{op, task, base:{sha,short}, code_diff, staged_docs:[{id,body}], findings:[]}` with a non-empty empty state; fork 12 → build a named-fact guard (both stated-at fences are presence-only and were **broken live**); fork 13 → M47 formally re-Settles M46 capability entries 2/3/4/8 **after** M47's scope is fixed, so M46 does not inherit a paid-down demand count; N9 → fix the step so `describe`'s suppression reason becomes true.

**The baseline audit also corrected the findings-verification itself and roughly doubled the charter:**

- `jigc task finalize --dry-run` **already runs every pre-stage finalize gate read-only** and is named by no promise-making surface — so H1's fork was mis-posed.
- H1's promise sweep is **nine surfaces, not four** — incl. `render.rs:266`'s `what's-left:` line (the highest-traffic one), and two design docs state it as an **invariant** ("they cannot diverge").
- **No construction of "validate runs the gates" can make the sentence true** — two gates are unpreviewable in principle, `empty-commit` is *wrong* at validate, and owner-artifact **false-positives** (proven live: an artifact at the correct home but not yet `git add`ed makes validate blocking on a state that finalizes at exit 0).
- Five **integrity-tier** new defects: the four milestone record-only doors bricking permanently · H2's dead end **splitting per clone** (operator sees exit 0, a fresh clone blocks) · `set-field`/`doc author` at an undeclared item field returning **exit 0 with a positive ack and then a dead doc** (`store.unparseable`, only recovery is discarding the task) · `set-slot` at an undeclared leaf silently writing to the real slot while the ack names the phantom leaf · the **slug hyphen bug** minting wrong frozen identities.
- Five **greenfield-path** defects sitting on G1–G3's own route (incl. P1-8 escalated, and `single-task`'s changelog step being unable to record a change while `describe` justifies hiding `record-change` with a factually false reason).
- Structural: the **two `finalize.yaml` files are independently maintained and unfenced against each other** — *"P2-1 **is** this bug"*; `cargo test` fails fast so a prose batch needs `--no-fail-fast`; baseline **2385/0**.

---

## Recommendation

**Fold back (ranked).**

1. **Correct the M47 charter in the repo.** `DECISIONS.md` (`e2f28f7`) and `recovered-analysis.md` currently re-instate a **superseded** plan. Replace the lacon re-drive with the three-probe greenfield acceptance (G1/G2/G3 + the two plants), and restore Tier 1/2/3 + the checkable boundary from `patches/0031` — verbatim, it is the committed text of `a822f87`. **Two Tier-1 items are currently absent from the repo entirely: the P5-4 finding-key flip and the `task diff --format json` hole**, both argued as now-or-never under 1.0 key/contract stability.
2. **Record that the M47 planning session is recovered, not lost**, and archive its Settle. Seven human decisions (two overruling the recommendation) and ~15 non-fork resolutions exist; re-planning from the charter alone would silently re-decide them — including three one-way doors (the `SLUG_RULE_VERSION` 2→3 bump, the schema-hash narrowing, the per-door error codes) whose window closes at the 1.0 tag. `files/M47-scope-brief.md` and `files/M47-settle-agenda.md` are complete and should be archived alongside.
3. **Add the eight missing findings rows + the H/P↔A/B/C/D crosswalk** to the committed archive — chiefly **P1-8** (drift hook), **P4-6** (silent copy-in on five edit verbs), **P5-4** (the pinned-wrong route), **H4** (the ahead-stamp doors + the absorb-cannot-mask refutation), P2-8, P4-2, P3-7, P3-8. Without the crosswalk the charter is unreadable against the surviving verification.
4. **Fold the log-derived facts of §(b)** into `recovered-analysis.md` — especially the P2 zero-fragment proof (it corrects `C1`'s mechanism), the verb histograms, the H2 log-anonymity record (index 9, `error_code: null`), the 80 s hook-rejection recovery, the P4 34-second pre-mint read, and honesty item 5. Fix the P3 46-vs-47 record count.
5. **Note, do not fold, the one verdict conflict** (A.1) and the four factual contradictions (A.2): the committed live-driven verdicts should stand as the verification of record, with the original's version recorded as superseded — except A.2.2 (P2 zero-fragment) and A.2.3 (the pack-axis root cause), where the surviving evidence favours the original and the fix shape changes.

**Already covered — do not re-fold.** The convergence check, the 503/504 purity verdict, the segmentation ranges, the exit/finding-code inventories, the expected-signals audit, the truncation datum, the P4 read-before-mint fact, the three M46 triage datums, and the H1/H2/H3 sharpenings are all faithfully in `recovered-analysis.md`. The committed `A5` (false-green sharpener), `A4`, `D7`, `D12`, and `B7` are genuinely *better* than the original and need nothing from it.
