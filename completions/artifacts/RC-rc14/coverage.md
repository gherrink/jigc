# Coverage — every changed surface in exactly one column

M47's rule over the rc.13 → rc.14 diff ([protocol.md](protocol.md) §6): **trial-reached** ·
**test-fenced** (naming the suite) · **neither** (each explained). Derived from the wave's own
increment set and the four audit findings — **never from a changed-file list**. That rule held
for the subject and slipped for one row; see *Row 12 was classified from the artifact* below.

**The subject is 17 sources**: 13 increments + 4 audit findings. ~~*The subject is 18 sources:
13 increments + `1799a2d` + 4 audit findings.*~~ — **struck, with the datum**:
`git merge-base --is-ancestor 1799a2d 979baca` → **exit 0**, and `979baca` is the sha the
**previous** trial ran (`1.0.0-rc.13`). `1799a2d` (2026-09-04, *fix(cli): the milestone boundary
gates the commit docs it commits*) is therefore an **ancestor of the rc.13 side** of this diff,
not a member of it. Its row below is **kept and re-labelled** — *reached by nothing* is a fact
this record still owes — but it is **carried from RC-m50's uncovered set**, not counted here.

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
| **10c** | `pack.resource-missing` — a resource miss names the pack it **searched**, with a code and a route (Increment 10's other half) | **trial-reached** | walk 20, verbatim: *blocking · pack.resource-missing — no composed pack ships `config/commands` — searched, highest-precedence first: house/fs-local (…)*, under the arm's own bar *"the fault names the pack that lacks the catalog (house), not 'the embedded pack' — law 1"*. Also fenced, by `crates/cli/tests/pack_resource_miss_axis.rs` (`CODE`, :63) |
| **11a** | the fan-out Fix phase (`fix-task` / `fix-finding`) | **neither** | no fan-out ran — see the note below |
| **11b** | the landing ack naming every commit | **neither** | same |
| **12a** | `setup` warns through its **existing** `findings` key about what the next door will refuse | **trial-reached** | walk 13 cell C: `advisory · setup.pack-load` over a shape-changing project schema shadow, install at exit 0, scored **3/4** on the four-part standard — the declared bound, measured |
| **12b** | the `code-anchor` grammar in `doc schema`'s projection | **trial-reached** | walk 20 renders `cites-code: code-anchor <repo-relative-path>[#<symbol>]` on `jigc doc schema adr`. **Rendered, not asserted** — that arm's bars are on the frozen shape |
| **12c** | the same grammar at `doc set-field --help` and in the installed guide · the anchor miss naming the grammar instead of asserting an absence | **test-fenced** | `code_anchor_grammar_sites.rs` · `doc_code_probe.rs`. **No arm ran `set-field --help`** — not in the walk scripts, not in a blind log — **and no `doc-code.*` finding fired anywhere**; the string's one occurrence in the walk record is composed step prose. The guide line (`QUICKSTART.md:153-160`, `include_str!`d into `SKILL.md`) was in the bytes every blind arm read first — exposure, not a drive |
| **12d** | the non-directory leftover answered honestly at the destroying doors — a route that fits a file, the masked sibling, `--force` named | **trial-reached** | walk 02 cell C, **both shapes planted at once**: 9/9 green, including the two bars written to **FAIL** on rc.13 (*names the directory leftover beside it*, *names the consent*) |
| **12e** | the narration keyed on the removal's **outcome** — the cell that fails for a cause the narrator cannot see (`Shape::Unremovable`) | **test-fenced** | `leftover_probe_fail_closed.rs` · `milestone_teardown_loss.rs`. Walk 02 drives `--force` only where the removal **succeeds**; no arm made one fail |
| **12f** | `--explain`'s pack label: one pack, one spelling (the unconditional `v` prefix dropped) | **trial-reached** | walk 20: `workflow:router    (pack-default · dev/1.0.0-rc.14)` — rc.13 rendered `dev/v1.0.0-rc.13`, as that arm's own comment records at `20-project-pack-composition.sh:159` — and the arm's `MEASURED` line reports **one** house label, `house/fs-local`, where the shape it was written against had two (`house/fs-local` + `house/vfs-local`) |
| **12g** | `--dry-run` forecasts the composed commit subject | **trial-reached** | **B2 and B3, unprompted** (no operator cue; the installed guide names the flag): `task finalize <id> --dry-run --format json`, exit 0, each envelope carrying `"subject"` (`docs: Record the ingest queue overflow policy…` · `feat(store): bound the number of distinct series the store holds`). Walk 17 reaches the flag only on its **refusal** cells, which compose no subject |
| **12h** | the absolute host paths out of finding loci and error text | **trial-reached** | walk 02 cell C — both loci repo-relative (`.jigc/worktrees/leftover-file`). The `/work/…` inside walk 13's `setup.pack-load` **relay** is the disposed `DeclaredAbsolute` half (`repo_relative_paths.rs` → `UNSWEPT_PRODUCERS`, `pack.rs`: *pack-load has no repo-root subject to be relative to*), not a miss |
| **13** | the guide edit → `SKILL.md`, and its re-clobber path | **trial-reached** | arm 23 — unedited guide re-installs idempotently; an edited one is **not** clobbered and setup says so |
| **`1799a2d`** | the milestone boundary gating a sub-task's transient commit doc — **carried, not a member of this diff** | **neither** | no fan-out reached a boundary — see below. Carried from **RC-m50's** uncovered set (it is an ancestor of `979baca`, the sha that trial ran): *reached by nothing* is still owed, so the row stays; the subject arithmetic above is what was wrong |
| **F1** | `add-from-spec` over an address reaching outside the repository | **trial-reached** | arm 23, **three traversal depths**, all `store.malformed-slug` |
| **F2** | the finding identity swept over 31 production sites | **trial-reached** | R4's `finalize.commit-rejected` in the invocation log; the codes ride every arm's log |
| **F3** | law 1 on the read paths | **trial-reached** | arm 23 — `doc show vision` names no host path and carries `store.*` |
| **F4** | `milestone discard` refusing over a sub-task's staged prose | **trial-reached** | arm 23 — **run outside the agent**, as §0.3 requires: refusal + code + `--force` named + prose intact + the consent route running verbatim |

## Row 12 was classified from the artifact, not the code — struck and re-derived

~~`| **12** | the bootstrap warning · the surface batch | **trial-reached** | walk 18 |`~~

**The datum:** `completions/trial-driver/arms/walk/18-surface-batch.sh:2` declares its own subject —
*"protocol.md §5 arm 18 (**M49 Increment 11**, T1–T8, + the two items owed after RC-1.0-final, S-1
and S-4, + the M46 changelog-gate carry)"* — and its cells (a)–(h) are that batch item for item.
Row 12 credited it with **M50 Increment 12**'s surfaces, which it does not drive. The row named a
column from an arm's *shape* (a surface batch, so the surface batch) instead of from the code the
increment shipped — *a coverage classification is a claim about the code; derive it from the code,
never from the artifact in front of you* ([pinning.md](../../../implementation/pinning.md) §5). That
section's addendum counts **five** instances; this is the **sixth**, and the count in §5 is not this
record's to move.

**The re-derivation** reads M50's Increment 12 decomposition ([roadmap](../../../implementation/roadmap.md)
→ Milestone 50) surface by surface against the arms and the blind logs, and it does **not** reproduce
the row it replaces: **eight** cells, **six** *trial-reached* and **two** *test-fenced*, and not one
of them via walk 18. Two of the six are reached by evidence no artifact read would have produced —
**12g** by two blind workers who ran `--dry-run --format json` with no operator cue, and **12d** by
walk 02's cell C, an arm written for **M49**'s audit whose two bars, left deliberately FAILing
against rc.13, both went green here.

**The table was also missing a row** (10c): M50 Increment 10's `pack.resource-missing` appeared in no
column of a table titled *every changed surface in exactly one column*. Driving the evidence rather
than reading the increment reversed its expected column too — walk 20 reaches it live, with a bar,
so it is **trial-reached** and fenced as well, not merely fenced.

**Named rather than fixed:** arm 20's own source comment still describes the rc.13 shape it was
written against — `20-project-pack-composition.sh:157-159`, *"renders as `house/fs-local` on its
Pack-input line and `house/vfs-local` on the workflow line"* — which the `echo` on the **next line**
falsified when the arm ran on rc.14 (one label, `house/fs-local`). The instrument's comment is stale;
saying so is this record's job, editing the instrument is not.

**What did not move:** the `neither` set is unchanged — rows 4, 10b, 11a, 11b and the carried
`1799a2d`. No re-derived cell landed there.

## The four `neither` rows are one event, not four

**10b, 11a, 11b and `1799a2d` all require a milestone fan-out to run, and none did** (`1799a2d`
being a carried row rather than a member of this diff — the subject strike above). The arm
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
