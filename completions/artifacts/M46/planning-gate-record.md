# M46 — the planning gate-record

Filled by hand before decomposition ([methodology-docs.md](../../../design/methodology-docs.md) →
The planning gate-record). **An empty or hand-wavy cell is a halt, not a pass.** Every cell carries
evidence or `N/A` with a one-line why.

Sources: the exercised [scope audit](scope-audit.md) (four capability-auditors on the real binary,
12 ledger dispositions re-derived), the four gap-detector dimensions
([gap-findings.md](gap-findings.md)), the **guarded** razor application — three appliers plus an
adversary — and the independent robust-advocate on N-3
([razor-ledger.md](razor-ledger.md)), the Settle ([DECISIONS.md](../../../DECISIONS.md) →
2026-08-18 M46 planned), and the independent pre-decompose review.

> **Status: NOT READY TO DECOMPOSE — the pre-decompose review returned *ready with corrections*, and
> five must bake back first.** It refuted a settled decision outright (D3, the stamp affirmation), and
> the two it stress-tested hardest (D2's merge premise, D4's take-neither) both survived independent
> verification.

---

## Milestone-level gates

| Gate | Verdict | Evidence |
|---|---|---|
| **strategic-claim-fresh** | ✅ | The claim — *every fix reconciles a contradiction the codebase already carries* — is grounded in a shape **measured at this sha**, not read off the charter: three of six new defects, the migration dead-end class, `is_route_exempt` and the `doctype-authoring` matrix all have the stated-here/violated-there structure. Falsifiable and named: **if the razor cannot refuse, the claim is wrong** — and it refused nine items with citations. The charter's own framing (*"the disposition wave"*) was **kept and sharpened**, not inherited: its claim (*a count is not a disposition*) is discharged on its hardest entry, entry 10, on measured evidence rather than a re-count. |
| **cheap-vs-robust** | ✅ | Run on every fork, and it **inverted three times**. N-3's *cheap* arm proved **not buildable** (`state::persist` receives opaque bytes — no delta, no base — so it cannot re-apply) and its *robust* arm proved **over-built** (3–4 increments bought by a coarse lock whose deadlock constraint dissolves once the delta is derived); the taken shape is neither, at ~1 increment. Fork 7's *robust-looking* refusal was **refused** and re-cut to a cheaper narration fix that dodges a measured `--force`-habituation trap. Fork 4's *cheap* answer (take neither) survived only because the residue was **driven empty on both arms**, not because it was cheap. N-3 went to an **independent robust-advocate before the human chose** (M34 discipline), and the advocate **argued against its own brief** — the strongest evidence the gate was not ceremonial. |
| **foreclosed-by-doc** | ✅ | **Six foreclosures found, each engaged on its rationale rather than obeyed or overridden.** `command-output-contract.md:363` + `finalize.md:41` foreclose Fork 3's wide arm — **rationale holds** (a preview must not promise *"this will commit"*), narrow arm taken · `finding.rs:262`'s route exemption forecloses N-4's route-floor half — **rationale holds as a declaration**, but is **factually false** on its own predicate, so the item is admitted at the predicate · `validation.md:435` + flow-48 arm 4 foreclose Fork 6(a) — **rationale holds and the razor inverts**: suppression would *create* the contradiction M48 removed · `team-ready-state.md:92` forecloses Fork 7's refusal — **rationale holds, predicate falsified**, re-cut at the predicate · `decisions-pending.md:207` (M42) forecloses Fork 4's window premise — **rationale holds**, fork closed · `decisions-pending.md:209` forecloses re-opening the publishing floor — **already decided post-v1**, discharged by citation. |
| **prior-art-reconciled** | ✅ | The sweep is what produced the wave's spine, and it **corrected the orchestrator five times** — `tasks.json` exists (`milestone.rs:41`), `finalize.base-mismatch` is a recorded exclusion (M47 Settle Decision 1, mirrored in four docs), `conformance.*` is route-exempt by declaration, `validation.md:39` *cites* rather than contradicts `:66`, and the "refuses three of six" premise was an artifact of an unqualified sweep. **Two locked docs disagreeing was treated as a blocking fork, not a free choice**, in every instance: C1 (`validation.md:418` vs `corpus-migration.md`, which has no foreign arm at all), C2 (`finalize.md:41` vs `validation.md:506`), C3 (`storage.md:124` vs `:143`), C4 (`validation.md:39` vs `:66`), and `storage.md:121`'s false universal over the destroying doors. |
| **census** | ✅ | Every enumeration is **derived and its hit-count shown**, never hand-listed. `is_unadopted_foreign` call sites = **5, and 0 in `migrate_corpus.rs`** · the route-owing ceiling if `is_route_exempt` narrows = **13**, all produced in `parse.rs`, one route per code · the `title-names-symbol` floor = **9 firings / 16 components on one surface** · the search-verb gap = **0 occurrences** across **29,886 bytes** of the whole help tree · foreign-advisory growth = **1:1, uncapped** (25 files → 25 rows × 2 doors, 23.6 KB) · the `set:`-kind migratability matrix = **7 rows, 3 of them permanent dead ends** · code citations across `design/` + `implementation/` = **46, zero pointing at a deleted file or past EOF** — which is what showed the drift is *concentrated in the artifacts this wave plans against* rather than diffuse. |
| **acceptance-spiked** | ✅ | Every in-scope finding was **reproduced against the real binary**, not taken from a record — including four that proved **different from their report**: N-4 has **two arms** (a fresh clone repairs; the repo where the corruption happened dies at `reconciliation.conflict-block` routed to the forbidden hand edit), PT-1's axis is **larger** (a second foreign file yields `migrate-corpus.deferred`, also unrunnable), entry 10's residue is **empty** (both arms driven), and N-5 reproduced on **three frameworks by three parties**. The two the appliers disagreed on (N-2) and the one the advocate flagged for attack (N-3's add direction) were **driven by the orchestrator**, and the second **falsified an advocate claim**. |
| **value-flow-exercised** | ✅ | The wave's value flow is *a surface that states a rule states it truthfully, and no door silently disables the guard that protects a human's work.* Both halves driven end-to-end on rc.11: a lost `file-state` baseline **silently disables `reconciliation.conflict-block`** and an uncommitted human edit dies at exit 0 on zero refs; `milestone finalize` enumerates one discarded path while destroying two others unnamed; `migrate-corpus` claims a file `validate` says is not its subject. |
| **overload valve** | ✅ | **Nine refusals recorded with citations, before the build rather than during it** — `doc search` · `milestone finalize --dry-run` · the acknowledged-findings ledger · the `off` severity member · the symbol-mention sweep · N-6 · the `planning-record` doctype · Fork 6(a) as written · the `bind` ack. The charter's own test (*a wave that cannot refuse cannot halt*) is met with instances. **Three of the nine carry measured evidence and were refused anyway** — the strongest form the valve takes. |
| **deliverable-reachable** | ✅ | For N-3, with two build-facts the review extracted and the increment brief must carry: **the base must travel with the moved value, not a fresh load** — `advance_file_state` takes `post_sweep: Some(swept)`, a record loaded at `task.rs:835` and carried **by value across the git commit and the hook's nested `jigc validate` process**, so stashing the base on `load` is correct *only* because the record moves; and `to_bytes` serializes `self`, so a stashed base needs `#[serde(skip)]` or the golden-locked on-disk shape changes. |
| **design-complete** | ❌ | Two gaps the review names. **F-I's fix direction is unsettled and the two artifacts point opposite ways** — the ledger frames the code as violating the docs, gap-findings calls the refusal a *false refusal* and the docs falsified; the arms (no-op + doc correction vs derive-a-value) have different tests, different doc edits, and different consequences for a future frozen bump, and the `Err` arm carries a deliberate *"needs the caller-supplied value (T4)"* comment, so no-op'ing **retires a designed extension point** rather than fixing a bug. And **four of the ten doc riders (`D1`,`D2`,`D3`,`D5`) are defined nowhere in the repo**, while `D1`–`D3` collide with the Settle-decision labels. |
| **check-scope-pinned** | ❌ | Self-declared, and the review confirms it. **PT-1's exit-status rule is undecided:** `unadopted-instance` is not in `STORE_EXIT_FLIPS` (`render.rs:676`), so after PT-1 lands **nothing exits non-zero on an unadopted foreign file** — today `migrate-corpus`'s exit 1 is wrong about its subject but is the only non-zero signal there is. Two further forks the evidence ranked blocking carry **no disposition at all**: Fork 3's preview-membership arm (F-E) and `is_route_exempt`'s narrowing (F-F). Undecided at Settle means decided at build. |
| **integration-seam** | ❌ | **The blast radius was not traced, and tracing it falsified the premise.** `jigc ingest` is register-only and adopts without a stamp, so the unstamped-managed population is refilled continuously by the product's own front door; `migrate-corpus`'s v0 arm is **live** (driven: `1 migrated`) and is the whole subject of `methodology_corpus_stamp.rs`; and deleting `classify_provenance`'s parse-against-prior arm would kill two engine tests that name it load-bearing verbatim. D3 is withdrawn. |

---

## Declared bounds carried into the build

Stated here rather than discovered later:

1. **`tasks.json` is a named bound on N-3, not a scoped decision.** It is written with plain
   `std::fs::write` at four sites in `milestone.rs`, is documented as *"shared by all N sub-agents"*,
   and is a **registry, not a flat map** — the base-relative merge does not transfer to it
   unexamined. It must be scoped in or excluded with a reason before N-3's increment is cut.
2. **N-3's admitted obligation is narrower than "fix concurrency".** What is admitted is
   *the reconciliation state machine must not be silently disabled by a concurrent write* — the
   citation is `CLAUDE.md:50`, not the deferral ledger. Scope creep toward general store locking is
   out by the razor's own reasoning.
3. **The stamp affirmation is a one-way door.** A legitimately-unstamped managed doc appearing after
   the pin would be misclassified as foreign. Accepted on the reasoning that the population is empty
   at the pin and 1.0 ships internal.
4. **N-2 and N-4 were relayed before they were driven.** Both are now orchestrator-driven, but the
   session's own rule — *an agent's report is a lead, not a measurement* — applies to everything in
   the evidence base that is still marked `read` rather than `driven` in the source probes.
5. **The `--ignored` refusal is refused, not deferred silently.** F-D measured why: a refusal on that
   axis fires on the ordinary fan-out success path, because a worktree that did its job holds build
   output. Only the **narration** half ships.
