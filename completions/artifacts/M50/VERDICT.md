# M50 — the last wave before 1.0.0 (the unvalidated-token wave) · VERDICT

**Status: complete.** Built + audited, **four audit findings, all confirmed live before any fix and
all fixed axis-complete**, re-verified **3341 passed / 0 failed**, clippy + fmt clean.
**`1.0.0-rc.14` built and installed after the fixes, not before** — the version-stamp confirmation
M47's audit added has now caught the owed bump unshipped in five consecutive waves, and did not need
to here.

Planning: [settle-record](settle-record.md) · [baseline-ledger](baseline-ledger.md) ·
[gap-findings](gap-findings.md) · [planning-gate-record](planning-gate-record.md).
Decomposition: [roadmap](../../../implementation/roadmap.md) → Milestone 50.
Acceptance: [worked-examples](../../../design/worked-examples.md) → flow 51.

---

## What the wave claimed, and whether it is true of what shipped

> **No caller-supplied token becomes a path component without the door validating it against the
> grammar, home or value rule that token's own family already declares — and `jigc start` tells the
> truth about task state on both its text and its versioned envelope.**

**The first half is true, over four families and after one audit correction.** The charter scoped
Tier 0 to *"the empty-id class over its one seam"*; the baseline falsified all three nouns and the
design review found a fourth family. Shipped:

| family | subject | the defect it closed |
|---|---|---|
| 1 · resolve | `--task` / `milestone_id`, 25 doors / 5 seams | `jigc task discard "../.."` destroyed the repository incl. `.git` at exit 0 |
| 2 · mint identity | `--slug` at `jigc rename` | a managed doc landed outside the docs root; `validate` said clean |
| 3 · root knobs | `docs-root`, `placement-root` | `uninstall` destroyed committed docs *and* an untracked file at exit 0; a file-shaped root landed the knob with every move failed |
| 4 · address slug head | the `<slug>` of `<type>:<slug>` | outside-repo bytes through the **1.0-pinned JSON**; an arbitrary in-repo file **committed** into the docs root |

**The second half is true of the product and unproven by the wave, by design.** `jigc start` no
longer emits `"state": "clean"` over a live task, and the golden that pinned that falsehood is gone.
Whether it moves the trial's duress cell from 1/3 is measurable **only** by a blind worker over an
abandoned task; the charter said so before the build, and this verdict does not claim it.

---

## The audit — four findings, four fixed, nothing deferred

**Every one was a larger class than the finding reported.** That is now four consecutive waves
(M46, M48, M49, M50) and it is recorded here as a property of the audit instrument, not a
coincidence: the auditors find the *instance* reliably and the *class* only when someone drives the
axis afterwards.

| # | severity | reported as | what it actually was | commit |
|---|---|---|---|---|
| F1 | HIGH | one unguarded door (`milestone add-from-spec`) | a registry whose root was a **hand-maintained name allowlist** | `cee0ae1` |
| F2 | MEDIUM | four new codes outside the envelope | **31 production sites** dropping any carried finding from the invocation log | `38f860a` |
| F3 | LOW | one leaking predicate + one read path | the shared predicate **and 4 read-path sites**, with 60 producers left countable | `b34a8c7` |
| F4 | LOW | a third destroying door disagreeing | confirmed, and the class derived to **6 doors, 3 in scope, 3 excluded with reasons** | `33c6478` |

### F1 — the guard reached every address door, and the registry stopped being a name allowlist
`jigc milestone add-from-spec` read a file **outside the repository** at exit 0, seeded a sub-task
from it, and landed a commit naming the foreign source — on the wave whose claim is that no token
becomes a path component unvalidated. Driven by the orchestrator before and after.

The root cause was the fix's own instrument: `DOCTYPE_ARG_IDS` was an allowlist of clap **argument
names**, and this door's argument is `spec_addr`. Adding the name would have closed the instance and
left the mechanism. Instead all three `*_ARG_IDS` allowlists were **deleted** for `ARG_TOKENS`, a
**total** classification of every argument of every leaf verb (35 rows), with the three vocabularies
as projections and a ⇔ fence in both directions, proven by two applied mutants.

**The fixer refused to claim a derivation that is not one, and said so in the code.** Clap's
introspection carries an id, a help string and a value name — never the field's Rust type or where
the value flows — and every one of these arguments is a `String`, so address-shapedness **cannot be
computed**. *Totality* is what makes it asked: a new argument reddens until classified, classifying
it `Doctype` reddens the door registry, joining that reddens the three axis suites.

### F2 — a refusal that names itself on the surface names itself in the log, at every door
`render::blocked_finding` had **two** production consumers, so `work-unit.malformed-id` was recorded
at `task discard` and dropped at nine other doors. The sweep's axis is **every production site that
renders an operational error** — 31 sites, enumerated by a fence rather than hand-listed, with the
funnel moved to where membership is decided. Red proof was **8 of 10 driven doors dropping the
code**, with the two working doors as controls, so the fix reads as the table going uniform.

**The fixer declined the fix this orchestrator recommended, on measured grounds now written into
the contract doc:** enveloping the new codes would have to key `(code, null)` — both guards build
`Finding` with `location: None`, and a null-target key is **strictly less informative** than the
flattened message, which names the offending address verbatim — and it would trade a within-door
divergence for a **cross-door** one in the same code.

**And it falsified the finding's own premise.** `store.malformed-slug` is *not* the first `store.*`
member outside the envelope: `jigc rename adr:nosuch --format json` flattens `store.not-found`, the
very code `doc show` envelopes. The divergence is a property of the **door**, predating this wave by
several milestones; recorded in the contract doc rather than closed, because uniformity there is a
byte move at ~10 verbs against a declared bound.

### F3 — law 1 binds the shared predicate and the pinned read contract, not just four doors
`untrackable_reason` composed 3 of its 5 reasons from absolute paths and had **four** production
callers; M50's own Increment 2 had fixed the hazard **at `rename` only**, leaving the predicate
unchanged and its comment naming the hazard by name. Fixed at the predicate. The read-path half was
**4 sites, not the 1 reported** (`store.not-found` *and* `store.unparseable`, in both `engine::store`
arms plus `engine::milestone`'s own copies), so one JSON object had been naming one file two ways.

`repo_relative` moved into `crates/engine/src/path.rs` as a pure render-against-a-root function —
the engine's empty-by-invariant property is untouched — and the standing fence's subject widened
from one file to a list.

### F4 — the abandon refuses over its sub-tasks' staged prose
Contested at triage, and **adjudicated by the human against an independent robust-advocate**, never
on a lone recommendation. Recorded as a **basis-has-changed reconciliation, not an override**: M46
decided this door's narrating arm deliberately, but on the warrant *"its worktree refusal already
carries the consent"* — and that clause is **driven false in the only cell where bytes die**, because
an agent authoring through jigc writes into `.jigc/tasks/<id>/docs/`, which is not inside the
worktree. In the repro the worktree's `git status --porcelain` was **empty** while the staged doc
held authored prose.

The guard-killer counter — *a milestone you meant to discard has staged prose by construction* — was
**driven false**: `add-task` and `provision` stage nothing, so the guard fires only after real work
exists, a strictly **better** false-positive profile than the `task discard` guard the human had
already accepted, which fires from the instant of mint.

The driven 2×2 after the fix, re-verified by the orchestrator: **exactly one cell changed.**

| staged prose | dirty worktree | exit | code |
|---|---|---|---|
| 0 | 0 | 0 | — (discards, no `--force`) |
| 0 | 1 | 1 | `milestone.dirty-worktree` |
| 1 | 1 | 1 | `milestone.dirty-worktree` |
| **1** | **0** | **1** | **`milestone.staged-prose`** |

`design/team-ready-state.md`'s falsified sentence was **struck with the datum** rather than
annotated, so the M50/D1 sentence eleven lines below — *"the guard is keyed on staged bytes rather
than on being a task"* — is now true as written. One doc had been proving both.

---

## The instrument, honestly

**Planning's own instrument failed twice, and both failures reached a decision rather than a
draft.** The `adr → research` edge was settled as a methodology-pack shadow on the orchestrator's
reasoning that *"the pack already shadows `commit`"* — true of the **files**, false of the **shipped
composition**, where dev wins and that shadow is itself already inert. A spike refuted it. Its
dev-pack replacement was then refuted by the design review for the **same shape one level out**:
nobody drove the marker-false composition the new ref depended on. The rule is on the record — **a
claim about how the composed product behaves is not established by reading the files it is composed
from** — and both were caught *before* decompose, by instruments the orchestrator did not get to
frame.

**Three of three robust-advocates corrected the brief they were given**, each against its own
interest: the T0 advocate conceded the "one predicate" framing was wrong and offered the narrower
split; the fork-2 advocate found `task diff` does not diff docs, refuting the orchestrator's own
reframing; the fork-4 advocate removed a leg from its own case.

**All four fixers corrected the brief they were given.** F1 refused a derivation clap cannot
support; F2 refused the recommended fix with measured grounds; F3 found four sites where one was
reported; F4 corrected a route caveat that named a cause which does not fire.

**The razor refused two product-shaped items with citations** — W-15 as chartered, and the
`adr → research` ref — where its chartered exclusion list had been four entries of build and trial
infrastructure. *If the razor cannot refuse, the claim is wrong*; it can.

---

## Declared bounds, carried in writing

1. **`ArgToken::Plain` is unchecked from the outside.** The classification catches *forgetting* — a
   new argument reddens until answered — but not *mis-answering*: an author who classifies a new
   address argument `Plain` still gets a green gate. Only the `Doctype` half is verified
   shape ⇔ argument.
2. **79 host-path producers remain across 11 files**, each with a measured count and a stated reason
   in `UNSWEPT_PRODUCERS`, so closing one reddens the row. *(Corrected 2026-09-09 at the RC-rc14
   handover verification: this bound and its two siblings said **60**, which the table it cites has
   never summed to — `UNSWEPT_PRODUCERS` is 11+22+20+9+8+3+1+1+2+1+1 = **79**, and the const did not
   exist before `b34a8c72`, the commit that also wrote the 60. The **per-file** counts hold and are
   mechanically fenced by `the_unswept_remainder_is_counted_not_described`; only the unchecked
   summary was wrong — a bound nothing measures is a sentence.)* Two are genuine law-1 siblings left
   deliberately: `engine::finalize` (11 sites / 5 finding helpers, none handed a repo root —
   threading one is its own increment) and `cli::start` (2 of 22 reaching a finding).
3. **`rename`'s flattening door predates this wave** and stays outside the findings envelope;
   making it uniform is a contract-wide byte move at ~10 verbs against a declared bound.
4. **The wave's second claim-half is unproven inside the wave**, by design. The orientation variant
   is built and its lie is gone; whether it moves the duress cell is the next trial's measurement.
5. **The fan-out Fix phase shipped and this audit round did not use it.** F1/F4 both touch
   `milestone.rs` and F2/F3 both touch the finding surface, so the round was not partitionable and
   D11's own stated fallback — serial — applied. That is the rule working, not a dodge; but it means
   the primitive's first real use is still owed.

---

## What is next

`1.0.0-rc.14` is built and installed. The **1.0.0 call is the human's** and was deliberately not
taken here: the product case is strong — the blocking finding closed, zero data-loss defects
surviving, the conversion ledger discharged — while the *instrument* case argues for one more
measurement, because four consecutive waves have had audit findings that were larger classes than
reported, and a trial on the fixed binary is how that has been caught every time. The next trial
inherits RC-m50's instrument as fixed and keeps the duress cell as its headline.
