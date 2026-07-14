# M42 — the rc.6 wave: completion verdict

**Status: COMPLETE.** Built (12 increments), audited, **5 audit findings fixed**, re-verified at HEAD `a309a53`: **1977 passed / 0 failed**, `clippy -D warnings` clean, `fmt --check` clean, tree clean.

Planning: [DECISIONS.md](../../../DECISIONS.md) → 2026-07-13 (the Settle) · [planning-gate-record.md](planning-gate-record.md) · [roadmap.md](../../../implementation/roadmap.md) → Milestone 42.
Acceptance: [worked-examples.md](../../../design/worked-examples.md) → flow 43 · `crates/cli/tests/flow43_acceptance.rs`.

---

## The claim, and whether it holds

**M42's claim: *the tool's own routes tell the truth.*** It was falsified at four independent sites when the wave opened. All four now hold, proven through the real binary:

| the route that lied | now |
|---|---|
| `validate` greened an unmigrated corpus | **exits non-zero** on a stale *managed* corpus; a stock brownfield repo stays **exit-0** |
| `validate` could not see an OOB edit to `CHANGELOG.md`/`VISION.md` | **detected and routed** (the placement census finished) |
| `migrate-corpus` answered `0 migrated` on the doctype it was asked to fix | **finds, migrates, and lands** it — through the tool, not raw git |
| a discarded milestone left a permanently lying committed record | **settles the record**; the terminal is a guard, not a snapshot |

**The audit found two more instances of the same disease, and both are fixed:** a v0-era doc of a bumped doctype was a **permanent dead end** (validate exit 1 forever; migrate-corpus refusing with a route that could not be followed) and `migrate-corpus` **exited 0 on a refused migration** while `validate` exited 1 telling you to run it — an infinite CI loop with nothing machine-readable naming why.

## Audit findings — all 5 fixed, none deferred

| # | severity | finding | fix |
|---|---|---|---|
| 1 | **HIGH** | A v0-era doc of a doctype at schema-version ≥ 2 is a permanent dead end — `strip_stamp(current)` silently asserted *no doctype ever bumped past v1* | `3da864b` — the v0 arm sources the prior shape from the doctype's **earliest shipped snapshot** |
| 2 | **HIGH** | `discarded` was **not terminal** — `provision`/`add-task` resurrected an abandoned milestone from its own settled record | `a309a53` — **one predicate at one site**: *a record in a terminal state does not re-seed a workbench* |
| 3 | MEDIUM | `migrate-corpus` exited 0 on a refused migration; `blocked` was untyped `[path, route]` tuples outside the finding-key contract | `f3cd145` — exits non-zero; `blocked` is a `Findings` collection with stable `(code, target)` keys |
| 4 | LOW | The pack-load slug-rule fence silently passed any manifest **omitting** the key | `6c431d0` — an omission is not an opt-out |
| 5 | LOW | The relocation walk consulted only `version - 1`'s home | `9111ef7` — unions **every** prior home |

**Two masking tests were caught and fixed, and both are worth recording.** Flow 43's discard arm asserted the record settled and then **never ran a milestone verb afterwards** — a snapshot assertion where the claim is a *lifecycle invariant*. And Increment 7's own e2e proof fixture (`setup_partially_joined_milestone`) minted its "partially-joined" record by doing `finalize` → `add-task`: **the fixture that proved the claim was itself an instance of the defect.** Both now exercise the invariant they assert.

## Two shipped-behaviour changes

1. **`jigc migrate-corpus` now exits non-zero whenever anything is blocked** — including `prose-needed`, the Framing-A waypoint where the CLI blocks, the LLM authors the required prose, and the operator re-runs. The migration genuinely is not done, and a driver must see that; but a pre-existing driver keying on exit 0 at that waypoint will now see a failure. **Intended, and stated.**
2. **A milestone in a terminal state (`discarded` *or* `joined`) refuses all eight workbench verbs**, `list-tasks` included — it reads the *workbench cache*, and serving it means rebuilding a workbench for a milestone that has none. The route names `jigc doc show milestone-record:<id>`, which returns strictly more than `list-tasks` ever did. **A milestone that is over is read, not operated.**

## An honest retraction

Increment 7 claimed *"a genuinely joined sub-task stays joined"* as a **verb-reachable e2e path**. It is not: a *mixed* record — a `joined` item under a live header — **cannot be reached through the verbs**, because `finalize` joins the milestone whole. The per-item rule stands as **engine-level honesty**, proven over a directly-constructed record, and the e2e claim is **withdrawn** rather than kept alive by a test that could only demonstrate it by resurrecting a finalized milestone. Recorded in [team-ready-state.md](../../../design/team-ready-state.md) → *The terminal is terminal*.

## The bound the wave did not close

**`spec` was not bumped.** Fork 5's "last cheap window" premise self-refuted — *if the window is real, the migration machine has failed its declared purpose* — so M42 fixed the machine instead. With the classifier closed, the `spec` lifecycle bump is now a routine `migrate-corpus` run **whenever it lands**, and the lifecycle gets *designed* rather than shaped by a classifier gap. Deferred with its rationale to the post-1.0 doctype-completeness milestone ([decisions-pending.md](../../../implementation/decisions-pending.md)).

---

## What this milestone actually taught

M42 caught **the same failure nine times**, in nine costumes:

- the `schema.location` census — keyed on the *name* `location`, missed `prior.location`
- the pack-prose census — keyed on the *dev pack*, missed the methodology pack
- the empty-diff backstop's false-refusal class — swept **one member of nine**
- the `finalize.*` census — keyed on the *file*, missed two CLI members
- the finding-key sweep — keyed on *five clusters someone listed*, missed two more (47 codes)
- a DECISIONS entry that recorded an unchecked premise **while warning against unchecked premises**
- the seam's own enforcement — installed at *three hand-listed funnels*, missed the fourth
- `strip_stamp(current)` — true when every doctype was at v1, false the moment one moved
- the relocation walk — consulting `version - 1`, the exact defect Increment 1 fixed, one version over

**Every one is a rule that was true of the members that existed when it was written, and silently false the moment the set grew.** And every fix but the last was *one level too shallow*: we added the missed members, and the **list stayed a list**.

**The rule, paid for nine times:**

> **A census cannot enforce a predicate.** If a property must hold of every member of a growing set, the check must live where membership is **decided** — on the constructor, the trait impl, the serialization — never on a list of the places you currently remember. **A list you must remember to update is not a fence.**

The tell was sitting in a doc comment the whole time: *"call it at each of the three funnels."* **A guard that has to ask to be called is not a guard.**

That rule is now **enforced structurally**, not documented: `Finding`'s `Serialize` asserts its own address, so a finding cannot reach a driver without a discriminating key, and there is no funnel list to forget ([DECISIONS.md](../../../DECISIONS.md) → 2026-07-14, *a census cannot enforce a predicate*).

## Process note — three halts, and they were the point

The build **halted three times at Increment 9**, each time refusing to proceed against a design-of-record claim that turned out to be false:

1. the `task-discard` ack pinned an effect key on an **idempotent no-op that does not exist** (the code's own doc comment was lying, and the design pass trusted it);
2. the finding-key sweep **had not closed the class** it existed to close (two more clusters, 47 codes);
3. the seam's enforcement was **itself a census**, and a fourth serializing surface was emitting a different key for the same defect.

Every earlier catch in this wave came from a *reviewer* — the gap-detectors, the robust-advocates, the design-reviewer, codex. **These three came from the builder refusing to build**, with a filled gate-record and an approved scope in hand. The gate-record was **necessary and not sufficient**: a cell can be filled confidently and wrongly, and what caught it was **independent readers with different priors** and **a builder that ran the binary instead of trusting the doc**.
