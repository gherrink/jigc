# The assumption register — an unfenced assumption is a gated defect

**Status: parked 2026-07-14.** From the M42 post-mortem ([DECISIONS.md](../DECISIONS.md) → 2026-07-14, *Where the bugs came from*). Generalizes the `trigger-test: code-anchor` shape specified for the `deferral-ledger` in the same post-mortem. Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

**The bug class no diff review can catch**, measured on jigc's own history:

| | |
|---|---|
| `strip_stamp(current)` — the corpus-migration v0 arm — written | **2026-06-25**, and it was **correct**: every doctype was at v1, so *"a v0 doc's shape is the current schema minus the stamp"* was **true** |
| `adr` bumped to schema-version 2 | **2026-07-03** — **57 commits later** — and also **correct** |
| did the bump touch `migrate_corpus.rs`? | **No. Not one line.** |

Two clean diffs. A permanent dead-end bug living in the space between them: `validate` exiting 1 forever while `migrate-corpus` refused the doc with a route that could not be followed. **A reviewer of either change sees nothing wrong, because nothing *in* either change is wrong.** The defect is in the *relationship* — a later, unrelated, **correct** decision silently falsified an earlier assumption.

It survived **six milestones, two RC trials, three completion audits and a cross-model review**, corrupting real adopter corpora the whole time.

**And it is not rare.** The same shape produced the freeze-assert hole (accepted on the trigger *"if packs are ever loaded from the filesystem"*; **multi-pack shipped at M14 and made it true, and nothing noticed**), and it is the mechanism behind most of M42's nine un-enumerated-sibling catches. **An assumption about a set, held in code, with nothing watching the set.**

## Why jigc is the right home for it — and where it stops

**jigc already proves the medicine works, in its own domain.** The schema-hash freeze assert *is* "fence the assumption where the set is defined": change a doctype's shape and the build goes red, **for everyone, immediately, in the gate of the person who changed it**. That is exactly what `strip_stamp` needed and never had.

**But jigc cannot write your fence.** It composes instructions and manages documents; it cannot assert *"no doctype is past v1"* inside arbitrary Rust. The fix for a defect-at-a-distance is always a `debug_assert` or a test — a **tier-1** fence ([dev-workflow.md](../implementation/dev-workflow.md) → *a defect at a distance*).

**So the honest scope of this idea is one rung lower, and it is still worth having: make an *unfenced* assumption a visible, gated defect.** jigc does not make the fence. It makes the *absence* of the fence impossible to ignore.

## The shape

A managed doctype — working name `assumption` — whose repeatable entries each carry:

- **`title`** — the assumption, stated as a claim (*"every persisted doctype is at schema-version 1"*).
- **`set`** — the population it quantifies over (*the doctype manifest*), prose; this is the thing someone else can grow.
- **`fenced-by`** — a **`code-anchor`**, `check: symbol-exists`: the test or assert that **holds today and reddens on the commit that makes the assumption false**.
- **`body`** — what breaks if it expires, and where (*"`migrate_corpus.rs`'s v0 arm derives the prior shape as `strip_stamp(current)`"*).

**The machinery already ships.** `code-anchor` + the `doc-code` `symbol-exists` probe is exactly the `spec` → `maps-to-test` shape (M42 made `maps-to-test`'s prompting real; the check has been live and blocking since M33). The anchor gate then **verifies the fence exists** and **blocks when a refactor deletes it** — which is precisely the guarantee the trial proved the anchor gate already delivers unprompted ([WHY-JIGC.md](../WHY-JIGC.md) → the brownfield refactor).

**A `fenced-by` left empty is the finding.** An assumption with no fence is an unexploded charge, and the register makes it a row you can see rather than a sentence in a file nobody re-reads.

## The determinism boundary holds

jigc asserts the **fence exists** (`symbol-exists` — a mechanical, structural check). It **never** asserts the fence is *correct*, or that the assumption is *true* — those are judgment, and grading them would be the A-3 `methodology_honesty_artifact` FAIL ([methodology-docs.md](../design/methodology-docs.md) → What M16 does not prove). **Presence, never content**, exactly as `maps-to-test` and the gate-record slots.

## Open

- **Doctype vs. field.** Is this its own doctype, or a repeatable section on `arch-doc` (which already owns "what this part of the system assumes")? The arch-doc route needs no new doctype and inherits its existing `cites-code` posture — but it scatters the register across parts, and the whole value is *one place you can see every unfenced assumption*.
- **Who authors an entry, and when?** The honest trigger is *the moment you write the assumption* — which means a step in `implement` (*"does this change assume something about a set someone else can grow?"*), already folded back as prose. Whether that step should also **compose the register's open rows** (so the author sees what is already unfenced) is the interesting half.
- **Does the register itself rot?** An assumption nobody registers is invisible, and jigc cannot know what you assumed. **This is the honest ceiling: the register catches the assumptions you *notice*, not the ones you don't.** It converts *"I forgot to fence it"* into *"I never wrote it down"* — strictly better, and not a solution. Any claim beyond that is the overclaim this idea exists to guard against.

## Trigger

**The post-1.0 self-migration**, when this repo's own working docs become managed jigc documents — the point at which jigc's own assumptions (*every doctype at v1*, *every pack embedded*, *every finding addressed*, *every serializing surface one of three*) become entries in a register jigc itself gates. That is also the cheapest possible proof: **the register's first four rows are the four assumptions that cost M42 nine catches.**

*(Cost note: an added optional field on a repeatable item block was **unclassifiable** by the corpus-migration classifier until 2026-07-14 — **M42 built `AddedItemField`**, so the `deferral-ledger`'s sibling `trigger-test` field, and any `arch-doc` route for this one, are now routine bumps rather than corpus strands. The wave paid for its own next improvement.)*
