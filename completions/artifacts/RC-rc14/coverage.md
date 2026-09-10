# Coverage — every changed surface in exactly one column

M47's rule over the rc.13 → rc.14 diff ([protocol.md](protocol.md) §6): **trial-reached** ·
**test-fenced** (naming the suite) · **neither** (each explained). Derived from the wave's own
increment set and the four audit findings — **never from a changed-file list**.

**The subject is 18 sources**: 13 increments + `1799a2d` + 4 audit findings.

| # | surface | column | how |
|---|---|---|---|
| **1** | the five resolve seams · `work-unit.malformed-id` at 25 doors | **trial-reached** | walk 17 — the `[empty]` column **53/53 green** |
| **2** | `--slug` mint identity · the address `<slug>` head | **trial-reached** | verify-pair `m50` probe 4 · walk 18 · arm 23 |
| **3** | the destroying-door consent pair · the deny floor | **trial-reached** | every arm's cleanup met `task-discard.staged-prose`; **B4-h met the deny floor live** |
| **4** | `ROOT_KNOBS` · `uninstall`'s third subject | **neither** | no arm set `docs-root`/`placement-root` to a refused home. Walk 15 drives M49's `placement-root`, not M50's knob refactor. **Test-fenced in part** (`flow51_acceptance` arm 4 reads `ROOT_KNOBS`), so this is a *trial* gap, not an unfenced one |
| **5** | the `active-task` orientation view · `SCHEMA_VERSION` 2→3 | **trial-reached** | walk 22 · verify-pair probe 2 · the first call of all five blind arms |
| **6** | `SchemaChangeKind` × `LOCI` | **test-fenced** | `flow51_acceptance.rs` arm 3. No corpus needed migrating — §0.4 established no doctype schema moved |
| **7** | the locus-3 byte-writing arms (`AddedItemField`, `AddedItemSlot`, `ValueRemapped`) | **test-fenced** | same suite, same reason |
| **8** | the `ref`→`to:` pack-load fence · `doc schema` contract 5→6 | **trial-reached** | verify-pair `m50` probe 3 |
| **9** | the write-miss route floor | **trial-reached** | walk 18 **66/66** — *and* **F-11** found its un-swept sibling one layer upstream |
| **10a** | every text render of a finding reaching the house renderer | **trial-reached** | walk 18 |
| **10b** | `milestone join`'s blocked verdict before the routing footer | **neither** | no join ran — see the note below |
| **11a** | the fan-out Fix phase (`fix-task` / `fix-finding`) | **neither** | no fan-out ran — see the note below |
| **11b** | the landing ack naming every commit | **neither** | same |
| **12** | the bootstrap warning · the surface batch | **trial-reached** | walk 18 |
| **13** | the guide edit → `SKILL.md`, and its re-clobber path | **trial-reached** | arm 23 — unedited guide re-installs idempotently; an edited one is **not** clobbered and setup says so |
| **`1799a2d`** | the milestone boundary gating a sub-task's transient commit doc | **neither** | no fan-out reached a boundary — see below |
| **F1** | `add-from-spec` over an address reaching outside the repository | **trial-reached** | arm 23, **three traversal depths**, all `store.malformed-slug` |
| **F2** | the finding identity swept over 31 production sites | **trial-reached** | R4's `finalize.commit-rejected` in the invocation log; the codes ride every arm's log |
| **F3** | law 1 on the read paths | **trial-reached** | arm 23 — `doc show vision` names no host path and carries `store.*` |
| **F4** | `milestone discard` refusing over a sub-task's staged prose | **trial-reached** | arm 23 — **run outside the agent**, as §0.3 requires: refusal + code + `--force` named + prose intact + the consent route running verbatim |

## The four `neither` rows are one event, not four

**10b, 11a, 11b and `1799a2d` all require a milestone fan-out to run, and none did.** The arm
chartered to reach them was **B4-h**, and **F-13 is why it could not**: it planned a
three-increment *dependent* spine, read four `--help` outputs, correctly judged that the fan-out's
isolation *"would silently drop each increment's dependency on the last"*, and moved to abandon
the milestone — where the deny floor stopped it and it asked.

So the coverage hole has a cause, and the cause is itself a recorded finding. That is a better
outcome than an unexplained gap, and a worse one than coverage: **a `squash: false` fan-out
reaching a boundary is unmeasured on this binary**, and `1799a2d`'s tightening — which
deliberately makes a fan-out that landed clean on rc.13 block at exit 3 — has been driven by no
trial, only by its own suite.

**Row 4** is the one genuine trial gap with no such excuse: nothing drove a refused root. It is
fenced by `flow51_acceptance` arm 4, so it is not unfenced — it is untried.

## Stated up front as not trial-testable, so no arm was invented for them

Carried from [protocol.md](protocol.md) §6 and discharged as written: Increment 13's goldens and
ledgers (a registry with no verb, finding or route for a walk to reach); the
`write_miss_shape_axis` fixture's manufactured declaredness cells; and the two `refs-post-hoc`
orientation goldens declared out of the compose sweep.

## What the walk actually ran

**24 arms.** 00 (control, first) · 01–21 (the regression net, derived for M49's surface) · **22**
(the M50 orientation surface + N15/N27, minted for this trial) · **23** (the four audit findings
+ Increment 13, minted after this document's first draft found them chartered and unrun).

Three arms needed repair before their result meant anything, and **all three repairs were to the
instrument, not the product** — the seven cleanup sites using a discard form rc.14 refuses (07,
08, 18), and arm 23's own two defects, one of which made **F1's bars pass for the wrong reason**
until the shape was driven afterwards. That last one is recorded in the arm's own comments,
because a bar that passes vacuously is worse than a bar that fails.
