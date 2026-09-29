# M46 — the reconciliation wave: completion verdict

**Complete and audited clean** as of 2026-08-20. Ten increments built, independently validated, then
audited by an independent code review and an independent e2e pass through the real binary. **Four
audit findings — 1 MEDIUM, 3 LOW, no HIGH — every one confirmed before any fix and fixed
axis-complete.** Re-verified **2847 passed / 0 failed**, clippy and fmt clean. **`1.0.0-rc.12` built
and installed after the fixes, not before.**

Planning record: [DECISIONS.md](../../../DECISIONS.md) → 2026-08-18 M46 planned · the
[scope audit](scope-audit.md) · the [gap findings](gap-findings.md) · the
[razor ledger](razor-ledger.md) · the [planning gate-record](planning-gate-record.md) (12/12, no
empty cells) · [roadmap](../../../implementation/roadmap.md) → Milestone 46.

---

## 1 · The claim, and whether it held

> **Every fix reconciles a contradiction the codebase already carries — a rule stated in one place
> and violated in another.**

**It held, and it was falsifiable in the way that matters: the razor refused things.** Nine items
were refused with citations *before* the build rather than during it — `doc search` ·
`milestone finalize --dry-run` · the acknowledged-findings ledger · the `off` severity member · the
symbol-mention sweep · N-6 · the `planning-record` doctype · Fork 6(a) as written · the `bind` ack.
**Three of those nine carried measured evidence and were refused anyway**, which is the strongest
form the overload valve takes.

The claim was also **turned on the wave itself**, twice, by the completion audit — see §3. That is
the intended behaviour of a claim of this shape, not a failure of it.

---

## 2 · What shipped

Ten increments, risk-first. Named by what each proves rather than what it patches.

| # | Increment | The contradiction it reconciled |
|---|---|---|
| 1 | The write seam stops reporting success for a discarded write | `CLAUDE.md:50` — *"out-of-band edits are detected and routed … **never silently merged**"* — vs a lost baseline silently disabling `reconciliation.conflict-block` |
| 2 | The milestone doors: subject becomes the path; narration becomes true | `team-ready-state.md:92`'s *"names every path it discards"* and *"for the remainder it narrates"* vs a teardown naming one path while destroying three |
| 3 | `migrate-corpus` stops claiming a file that is not its subject | `validation.md:418` — *"**never** to `migrate-corpus`"* — vs a blocking exit-1 claim with an unrunnable route |
| 4 | The migration transform stops refusing what already conforms | `corpus-migration.md:131`/`:264` + `doctype-authoring.md:35`/`:41`'s *"spliced into every item"* vs `transform.rs` reading `decl.default` only |
| 5 | The pre-guard repair route, both arms | `finding.rs:262`'s *"no CLI verb repairs a hand-broken byte"* vs a shipped verb chain that does |
| 6 | The changelog gate keys on the gate, and previews what it gates on | `validation.md:568` — *"keys on the **gate**, never on the diff"* — vs a repeatable-item count |
| 7 | The closure-framework diagnosis stops asserting a false absence | law 1 vs a message claiming a symbol *"is absent from"* a file whose indexed blob contains it |
| 8 | The surface batch — seven admitted findings | seven stated rules, each violated at one door |
| 9 | The record tells the truth about itself | `doctype-map.md` recording the shipped `schema-version` for one of four bumped doctypes |
| 10 | Flow 49, the goldens, the fold-back | — the close |

**Acceptance:** flow 49, six arms, **each enumerating its class's axis from a code-side registry**
rather than pinning the reported repro — the discipline M45 introduced and M47/M48 held. The full
golden tree regenerates from an emptied root and **the empty diff is the assertion**.

---

## 3 · The audit, and the two findings that are M46's claim turned on M46

**e2e: 15/15 scenarios pass**, overall pass, through the real binary in throwaway repos. The concurrent
file-state merge held under **9–12 simultaneous processes** and across the finalize/pre-commit-hook
hand-off; all four destroying doors narrated both the untracked *and* the gitignored plant; the
milestone boundary read the worktree on disk rather than the registered set; and **increment 5's
repair route ran to green from the repo where the corruption happened** — the harder of its two arms.

**Code review: 4 findings, all confirmed live or at the seam before any fix.**

| | Finding | Fix |
|---|---|---|
| **F1** MEDIUM | Increment 8's own new sentence overclaims: *"each with the reason it sits off the catalog"* is false for `router`, `ingest-existing` and `increment` — `creates-task: false` workflows sit off the catalog while owing no reason, because M43's `suppressed:` fence binds only `selectable: false` | `706aadf` — clause scoped to the hidden set, a **law-1 scope repair**, no behaviour change. Blast radius was **18 goldens, not the 12 the finding named** — the fixer found the `start-intent-json` sextet embedding the same sentence in its `"text"` key |
| **F2** LOW | `migrate-corpus.set-field-unfilled` composes an item-locus address that cannot resolve | `332b335` — locus threaded through; **no address fabricated**. The fixer could have parsed the migrated bytes and emitted per-item findings, and refused on a decisive ground: *a repeatable section carrying zero items still has an unfilled leaf*, so a per-item finding is silent for exactly the corpus a doctype author most needs to hear about |
| **F3** LOW | `unfilled` advisories reach neither log branch, and `unadopted` drops the moment anything blocks — violating **the code's own comment three lines above** | `c5070e1` — both branches carry what the run decided; the exit predicate is stated once, and both test cells assert the exit **from the log record's own field** |
| **F4** LOW | Increment 1's sweep converted `tasks.json` and left `base.json` on truncate-then-fill **one line above**, in the same two functions | `40b6f51` — fixed **at the axis, not the instance** |

**Two findings are the wave's own claim pointed back at it.** F1 is increment 8 introducing a law-1
overclaim inside the wave whose claim is that every fix reconciles one. F4 is increment 1's sweep
leaving an un-swept sibling inside its own stated axis. Neither is a design defect; both are
**incomplete sweeps** — which is precisely the M45 complete-fix lens, and precisely what a completion
audit exists to catch.

**F4's fixer found more than was reported.** Told to derive the axis rather than take the two reported
sites on faith, it derived the class — *gitignored shared `.jigc/` workbench files another door
parses* — disposed every member, and found a **fifth writer nobody had counted**: `unwind_mint`
(`crates/cli/src/milestone.rs:744`), restoring `tasks.json` on `add-task`'s rejection path, a
truncating write into the shared area on a live door's failure path. The doc-comment asserting *"all
four of its writers persist through `crate::state::persist`"* had enumerated only the **engine**
writers. **The same incomplete-sweep shape as F4 itself, one layer out.** It then rewrote that
comment to state the rule over the class and record that the first statement of the rule shipped a
false enumeration — so the next reader trusts a claim a test iterates rather than a list that already
rotted once.

It also reproduced the symptom rather than settling for a structural pin: **215 torn reads over 2000
rounds** through the real `mint_milestone` door.

---

## 4 · Planning, and what it cost to get right

The planning phase corrected itself **six times**, every one the same shape — *a static read reaching
a cleaner conclusion than the binary supports* — and two of them reached decisions rather than drafts:

- **`tasks.json` does exist** (`milestone.rs:41`). An auditor's claim relayed into a document
  unverified and reported onward; it survived a full reporting cycle.
- **`finalize.base-mismatch` is a recorded deliberate exclusion** (M47 Settle Decision 1, mirrored in
  four docs), not a hole the trial missed.
- **`conformance.*` is route-exempt by declaration** (`finding.rs:262`).
- **`validation.md:39` cites the floor rather than contradicting `:66`** — *"27 lines apart"* was
  counting lines instead of reading the parenthetical.
- **The "razor refuses three of six correctness defects" tension was an artifact** of an unqualified
  citation sweep; under qualified citation it admits five of six.
- **D3, the stamp affirmation, was affirmed and withdrawn the same day** — a decision the human took
  on the orchestrator's recommendation, resting on a premise **checkable in one command**:
  `jigc ingest` is register-only and adopts without a stamp, so the unstamped-managed population is
  refilled continuously by the product's own front door rather than draining to empty.

**The rule these yield is now in the record, and it applies to subagent output exactly as it applies
to docs: an agent's report is a lead, not a measurement.** Everything load-bearing in M46's evidence
base is marked driven-by-orchestrator or explicitly marked relayed.

---

## 5 · Honest bounds, carried forward

1. **The `--ignored` loss is visible, not prevented.** A gitignored secret inside a fan-out worktree
   is still destroyed at the four doors; it is named first. Refusal was refused on measured evidence
   — a worktree that did its job holds build output (1 → 4 → 43 entries as the probe widens), so a
   refusal on that axis fires on the ordinary fan-out success path and trains `--force` into reflex.
   *Re-opening condition:* an adopter reports work lost to a narrated ignored-path teardown.
2. **N-3's admitted obligation is narrower than "fix concurrency."** What shipped is *the
   reconciliation state machine must not be silently disabled by a concurrent write* — the citation is
   `CLAUDE.md:50`. General store locking remains out.
3. **F2's fix does not enumerate per-item leaves**, by decision — the zero-item section is why.
4. **The `is_route_exempt` narrowing was refused** and the measured ceiling of **13** blocking
   `conformance.*` producers is recorded rather than spent; narrowing would not resolve the
   contradiction it names, which is about the *document*, not the code.
5. **`storage.md:121` still reads as a universal** over a door list; whether it gains the exclusion or
   the pointer was scoped to increment 2's doc work and should be re-read at the next wave.

---

## 6 · What this milestone claims

The build, the audit, the four fixes, and the `1.0.0-rc.12` install — **and nothing beyond it.** The
next acts are a trial on this binary and then the 1.0.0 call, which is the human's.
