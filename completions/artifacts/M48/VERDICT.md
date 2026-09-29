# M48 — the rc.11 wave (the pre-1.0 reliability + discoverability wave): VERDICT

**Complete and audited, 2026-08-15.** Twelve increments built, independently validated, then a
milestone-completion audit whose **four findings were all confirmed live and fixed axis-complete**.
Re-verified **2735 passed / 0 failed**, clippy clean, fmt clean — cargo's own exit codes, measured
unpiped. **`1.0.0-rc.11` built and installed after the audit fixes, not before.**

Planning record: [settle-record.md](settle-record.md) · [baseline-ledger.md](baseline-ledger.md) ·
[planning-gate-record.md](planning-gate-record.md). Decomposition + scope:
[roadmap.md](../../../implementation/roadmap.md) → Milestone 48. Acceptance:
`crates/cli/tests/flow48_acceptance.rs`.

---

## The claim, and whether it was proven

**The claim:** the discoverability lens that landed six consecutive trials is a **mechanism** defect,
not a series of instances — and the mechanism is countable and fenceable.

**Built:** a pack-load fence whose owe-set is **derived from the pack tree itself** — the union of a
`{{cli.<id>}}` ref resolving to a `doc <write-verb>` and a `{{schema:<T>}}` ref — so an authoring step
that solicits a write **cannot ship** without naming the read-back. Withdrawing any one member's
declaration blocks pack load non-zero.

**Honest bound on the claim:** the fence proves *the surface now names the verb*. It does **not**
prove an agent then finds it. That is what the next trial tests, and the wave's own recorded
prediction stands: **if a seventh probe finds the lens again, that is a signal about the product's
shape, not about the trial.** F1 is the first fix aimed at the generator rather than another
instance; whether that generalises is the open question this wave hands forward.

---

## What shipped

| Inc | Deliverable |
|---|---|
| 1 | The destroying-door guard — the subject becomes **the path about to be removed**, not the registered set. One fail-closed classifier at `provision` / `discard` / `uninstall`, plus the `.jigc/tasks/<id>/docs/` authored-prose arm. `--force` at each door. |
| 2 | The write path stops lying about identity — a divergent-or-dropped `title:` is rejected, routed to an in-task title change split on **committed-store identity**. |
| 3 | The read-back fence (the claim), `doc show` into both command catalogs, `doc list`'s route + `--task`. |
| 4 | One foreign file, one code, one route at every door — with the managed cell routing **on the stamp**. |
| 5 | The install commit's pathspec derived from where the hook landed, not hardcoded. |
| 6 | The cascade's read rung (`config get` / `config list`), and jigc rendering the unknown-subcommand block itself so a read intent never lands on a write verb. |
| 7 | The text/JSON parity fence, and the **pre-1.0 additive-key window discharged**. |
| 8 | The idempotent-rename class sweep over all nine committing doors; `describe`'s filter. |
| 9 | The tier-2 truth batch — six verified wording items. |
| 10 | The adapter's first owned artifact: the guides, version-stamped, replaced on update, refusing to clobber a user-modified copy. |
| 11 | The manifest-hash fence — a hash may move only with its co-located version. |
| 12 | Flow 48 (every arm iterating a registry), the golden regeneration from an emptied root, and **the conversion ledger closed**. |

---

## The completion audit — four findings, all confirmed, all fixed

Each was **reproduced live before any fix**, and each fixer's red test constructed the cell through
its real door rather than planting it.

| Sev | Finding | Fix | What the fix also found |
|---|---|---|---|
| **HIGH** | M48's own centrepiece created a dead end: a `doc author` failing mid-payload rolled back the staged file but **not** the role binding, and the new pre-check made that orphan **blocking** — refusing every corrected re-author with a message naming a doc the task did not hold and a route that could not run | `d9af1b8` — a role binding whose doc is gone is not an incumbent, via a **state-derived** probe (holds however the orphan arose, including out-of-band working-area edits) | The auditor's prescription was **too narrow**: a staged-only predicate would have let a divergent title mint beside a **committed** doc the task had bound. The shipped predicate spans **both homes**. |
| **MEDIUM** | `milestone provision`'s refusal was **not transactional** — the guard sat inside the per-sub-task loop while `DECISIONS.md` and the function's own doc comment both claimed *"not a half-provisioned set"*. **Both fixtures planted at the first path**, with a comment naming that topology — masked, not merely unpinned | `fc06a93` — a two-phase provision: probe every path, then mutate. The claim was made **true** rather than weakened | Both sibling doors (`discard`, `uninstall`) checked and **already correct**; `provision` was the only one mutating as it walked |
| **LOW** | `doc rename` re-slugged a staged doc onto a **committed** identity at exit 0; only finalize caught it | `d6b6e45` — the destination is free in **both homes** or it is refused, adjudicated **before a byte is written** | An **unreported second defect**: the staged arm was a *partial write* — `rewrite_h1` + `persist` ran before the collision check, so a refused rename left the source's H1 already rewritten |
| **LOW** | On the **default** `.git/hooks` shape — the dominant one — `setup` still listed the hook as installed with nothing saying it is not in the install commit | `78b288f` — the summary says when the hook is local only, keyed on the **settled pathspec** the commit was made from | That key is strictly stronger than re-testing the shape: it also covers the **sparse-checkout** cell, where every committability test says yes and git refuses anyway |

**Three of the four fixers pushed back on or extended the finding they were handed, and each was
right to.** The pattern is the wave's own lens turned on its own audit: a prose claim about code is a
hypothesis until something drives it.

---

## The two design halts — both caught a fix that would have made things worse

**Increment 4.** The Settle carried a **misattributed route string**, taken from the pre-decompose
review and propagated without checking which function owned it. `"ingest, migrate, or move it out of
the managed location"` is `conformance_advisory_finding`'s **own** route, not `unadopted_instance`'s —
so the lie did not *arrive* with convergence; it was already on the managed cell, and the settled
split **kept** it there. Worse after the fix than before: the arm would then serve managed docs only,
making the route wrong for **100%** of its population. Re-settled: **route on the stamp**, reusing two
already-shipped strings.

**Increment 11.** The fence's **comparison window** was falsified by measurement. GitHub Actions fires
one run per push, at the tip, so `HEAD~1` only ever sees a push's last commit — and this repo pushes
in batches. The instance that decided it: `2c5eee5`, which re-pinned **all 16 doctype hashes at
unchanged `schema-version`s in both manifests** — the exact shape the fence exists to catch, and the
one both manifest headers name as *"the declared genesis exemption and the ONLY one"* — landed **34
commits from its push tip**, over which the settled check is provably clean. Re-settled: **widen the
base ref to the pushed range**, with `HEAD~1` as the floor.

Neither was caught by review or by planning. Both were caught by an agent that **measured** —
`git reflog`, `rev-list --count`, an empty diff over the one commit that mattered.

---

## Honest bounds carried forward

**From planning** (seven, in [planning-gate-record.md](planning-gate-record.md)), of which the live ones:
1. **F8's error-surface interception** shipped on its spike, not the declared fallback — recorded so the fallback's absence is a fact, not an assumption.
2. **F7's fence is not axis-complete** — the derived-`optional:` half is; the step-body half is a bounded omission-vocabulary probe. Arm 1 (the canonical-form change) was **refused**, and the parked deferral it would have fired now carries a counted-and-still-deferred line.
3. **The parity fence does not cover the prose tier** — value-rendered surfaces are mechanically fenced; prose surfaces got a one-time enumerated census.
4. **The adapter artifact is a declared deviation from principle #5** — refuse-to-clobber is an untracked-fork *detector* with no delta. The delta discipline is owed **when a second artifact appears**.
5. **The manifest-hash fence protects this repo only**, not adopters, and is a build fence rather than a surface.
6. **`provision` refuses where it used to succeed** — a shipped-behaviour change, taken now because it is breaking after 1.0, and because **the binary cannot distinguish `junk.txt` from `precious.txt`**.

**Added by the audit fixes:**
7. **The provision refusal's transactional claim covers the refusal.** A `git worktree add` that fails mid-phase-2 still leaves earlier paths provisioned, and nothing rolls that back — an idempotent re-run reuses them. Stated in both homes.
8. **Post-mint drift can still produce a `finalize.promote-clobber`** the write path cannot foresee (a file appearing at the destination after the create). Pre-existing, not caused by `rename`, deliberately not widened into.

---

## Refused, and recorded — because a wave that cannot refuse cannot halt

The **search verb** and the **file-state read verb** (M46 entries 4, 8) — *missing capabilities, not
unnamed ones*, and this wave's lens has nothing to name · the **checkpoint record** (entry 2),
re-counted at **seven** demands, with the charter's zero price corrected to an eleventh manifest entry
or a version bump plus migration · **F15's structural half** — a one-way door on two frozen doctypes,
resting on an item-block `ref` shape **zero shipped doctypes exercise** · **F7's canonical-form
change** · **F14's unbundling** · **`describe <name>`**.

**M46's ledger is adjudicated entry by entry** — the table lives in
[decisions-pending.md](../../../implementation/decisions-pending.md) → the capability wave. Entry 12
absorbed, entry 11 partially absorbed, entry 4 split, entries 2/3/10 re-counted and still deferred.

---

## The gate on the 1.0.0 call

**The conversion ledger is CLOSED** ([findings-verification.md](../RC-pre-1.0/findings-verification.md)
→ The conversion ledger). Every row carries `pinned-by:` or a stated `UNPINNED: <why>`; every citation
was verified **by what the test asserts, never by its name**; the lumped `F10–F16` row was **split**,
because one row cannot carry seven citations and a lumped row is exactly where an uncited member
hides. Where a finding had two halves and one shipped, the row says so and the unshipped half carries
its own `UNPINNED:` — **F10**'s advisory-habituation half (routed to M46 entry 3; a standing test over
it would pin the floor as expected output) and **F16** (no fix exists, so there is no behaviour to
cite).

**Next: the 1.0.0 call — the human's.**
