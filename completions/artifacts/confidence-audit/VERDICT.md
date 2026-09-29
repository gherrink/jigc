# The confidence-audit wave — VERDICT (2026-07-24)

**Provenance:** written at wave close by the session that ran it, from the delegated agents' reports and the committed diffs — persisted per the persist-the-audit rule ([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md) → Audit: *an audit left only in a summary has no record*). Companion records in this directory: [back-sweep-triage.md](back-sweep-triage.md) (the M39–M44 triage table), [minor-batch-dispositions.md](minor-batch-dispositions.md) (the fix-or-declare batch), [mutation-results.md](mutation-results.md) (the full cargo-mutants report and per-mutant adjudication).

**Charter:** DECISIONS → 2026-07-24 *The confidence-audit session runs as a chartered wave* — the pre-1.0 confidence audit (DECISIONS → *The road to 1.0*) run as dev-workflow loops, one fix one commit, closing with its own completion audit. Scope: the sibling-hunt's ranked fix-list ([M45/sibling-hunt.md](../M45/sibling-hunt.md)) + minor fix-or-declare batch + record-truth corrections, targeted mutation testing on the fixed seams, and the M39–M44 back-sweep triage-first. Wave span: `f879401..dc284b3`, 18 commits.

## Verdict in one line

**All 8 ranked findings verified real (zero refuted) and fixed axis-complete; the batch surfaced two additional live data-loss defects (both fixed); the back-sweep found and pinned the two genuinely unpinned load-bearing facts; the completion audit returned no HIGH; mutation testing over 192 seam mutants confirmed three seams fully clean and converted its 7 surviving-mutant gaps into killing tests — the wave closes with every finding either fixed test-first or declared with a recorded reason.**

## 1 · The ranked fix-list (all verified live before fixing — none refuted)

| # | Finding (class) | Verified-real evidence | Fix | Commit |
|---|---|---|---|---|
| 1 | Above-current stamp undetected (freeze-stamp) — **integrity tier** | `schema-version: 99` ADR: zero findings, exit 0, migrate-corpus "already-current" | Distinct un-keyed `schema-conformance.schema-version-ahead` (+ verb-side code), exit-flipping, Human route; migrate-corpus `>` blocks, never silent; axis = all 15 registry-derived versioned doctypes | `0aa4afb` |
| 2 | Promotions rollback resets to HEAD (rollback axes) — **integrity tier** | Mid-task staged blob at a promotion destination destroyed on hook-rejected finalize (`git show :<dest>` → does not exist) | Reuses the axis-3 `capture_owner_artifact_index`/restore primitive — all four staged-path families now on captured pre-images; whole-index byte-identity asserted | `ec9fa23` |
| 3 | Rejected-write-exits-1 unpinned (exit codes) | Local mutation: flip to 3 accepted by every prior assert | Pin across the clap-derived doc write-verb family (set-equality fence, exact exit 1 + refusing code per arm); `milestone.rs` literal derives `EXIT_ERROR` | `0666d0c` |
| 4 | `hook_output` producer axis 2-of-5 (JSON purity) | Marker-hook text dropped entirely by rename / migrate-corpus / milestone record ops; fan-out envelope 1-of-3 | **Axis corrected to 7 members** (two sibling-hunt claims corrected against code); seam unified at `git_commit_capture` — a hook-capable commit cannot run without its stream in hand; response re-derived per producer | `bc9b396` |
| 5 | `creates-task:false` start surface un-goldened (goldens) | No golden existed for router/orientation/AGENT.md surfaces; code-structure skip bypassed EXCLUSIONS | Golden tree 558 → 612 cells (zero existing bytes changed); the filter removed, no-definition-shape-filter rule stated at the seam; EXCLUSIONS verified still empty | `d009b98` |
| 6 | Retitle-item misses the trailer-key rule (trailer) | `retitle-item … --title "BREAKING CHANGE"` exited 0 on a commit trailer item | Shared adjudicator wired at the retitle door (unified constructor, same code/route); door axis enumerated 7 members covered/safe/excluded; M45 Settle's "both doors" overstatement corrected in place | `c3a6e07` |
| 7 | History-gate coverage argued-not-tested (history-gate) | — (pin work) | 13 op-axis tests as real git ops incl. sparse-checkout (pinned-and-declared conservative bound, trigger-keyed) and the §1.3 milestone-create baseline writer; collapse argument recorded at the oracle; Decision-7 citation **corrected** (the split was decided in the Inc-7 planning entry, not Decision 7); mutation check caught the rejected `--all` oracle alternative | `0db6971` |
| 8 | Bind-map type-uniqueness unfenced (role-binding) | Mutated dev pack with duplicate-type `allows-create` drove `jigc start` to exit 0, silently first-match-binding | Blocking `workflow-refs.allows-create-duplicate-type` asserted in `load_workflow_def` — fence where the set is defined, every load door blocks | `9c55e93` |

## 2 · The minor batch and the two extra live defects

Dispositions in [minor-batch-dispositions.md](minor-batch-dispositions.md): **8 fixed, 2 declared with reasons** (the unshipped single-slot+nested cell — already trigger-keyed by the registry-derived ceiling suite; the set→proof linkage — honestly list-enforced, the checkable half is not the load-bearing half). Commits `05d3977` · `8a4e134` · `92999a1` · `12a618b` · `da9e5d4`.

Two findings in this wave were **live data-loss defects beyond the sibling-hunt's list**, both reached red-first past all gates:

- **`05d3977`** — a same-path untracked foreign's only copy deleted on hook-rejected `finalize --approve` (`plan_retirements` skips `source == destination`, bypassing the retire byte-capture). Carried into the wave by fix 2's agent as an unverified observation; verified real.
- **`4ea6a7a`** — the code-review MEDIUM: the displaced-byte capture was discarded on a mid-promote failure, so the rollback deleted the foreign at a *different* failure point of the same class. Fixed over the **failure-point axis** (mid-promote and mid-retire both proven reachable red-first; stage/commit points already covered; the three index captures excluded-with-reason); the reviewer's LOW-2 (tracked destination's uncommitted pre-promote bytes restored from capture, not HEAD) folded in with its own red→green pin.

## 3 · The back-sweep (M39–M44, triage-first)

Full table in [back-sweep-triage.md](back-sweep-triage.md): **58 already pinned · 21 superseded/declared · 2 load-bearing and unpinned · 1 unclassified.** Both (c) items pinned this wave:

- **c1 (HIGH)** `b92285f` — concurrent N-process fan-out → join determinism: three fresh-built fixtures, genuinely simultaneous compose phase, explicitly sequenced divergent completion orders + serial baseline, whole-tree byte-identity + id-ordered join asserted; mutation check (mtime-keyed join) reddens it; 5/5 stable.
- **c2 (MEDIUM)** `5b7ed10` — the M40/M41/M43 cross-order byte-determinism witnesses as three §3 repro blocks, each mutation-checked against an order leak.

The unclassified item ("jigc never destructively rewrites HEAD") is **disposed covered-by-composition**: its one known live inconsistency was sibling-hunt item 2, fixed here; the optional reflog repro block was not added (the composition — WIP-survival arms + all four rollback-axis suites now on captured pre-images — is the honest fence; a global negative has no non-grep pin).

## 4 · The completion audit

**Code review** (independent, read-only, whole wave diff): **no HIGH.** 1 MEDIUM (fixed, `4ea6a7a`, §2 above) · 3 LOW — LOW-2 folded into the MEDIUM fix; LOW-3 (the chain-commit fold's abort-path trade) and LOW-4 (fixes 3+8's unrecorded decisions) closed as record repairs at `91d9ca6`. Clean per category on: the priority seams' correctness (ahead/below mutual exclusion, `s == current` boundary, no double-restore, seam-unification claims verified by grep), invariants (no schema/pack file touched — the frozen-v1 gate untripped), scope honesty (sampled commit-message claims all verified), test quality (no masking shapes), and the record (no checkably false rationale).

**Audit-shape declaration:** this wave's audit = code review + mutation testing + the per-commit full gates (every fix's acceptance already drives the real binary in its own suite; the flow suites ran green in all 18 gates). No separate e2e-tester pass was run — a judgment call for a fix wave whose fixes are themselves e2e-shaped, declared here rather than left silent.

**Mutation testing** ([mutation-results.md](mutation-results.md)): cargo-mutants over the 9 seam files / 34 seam functions — **192 mutants: 180 killed or unviable, 12 missed**, adjudicated 7 (a) genuine test gaps · 3 (b) equivalents (reasoned concretely; one names a latent third-party-pack hazard — the retitle first-field guard holds for all 15 shipped schemas but is order-dependent) · 1 (c) survivable-by-design (its direct red-path test added anyway) · 1 bonus (a nondeterministically-red proptest with an unslugable-title generator). **Three seams fully clean: the history gate, the allows-create fence, the settability predicate.** All 7 gaps + the flake + the (c) test closed at `dc284b3`, each with applied-mutant → red → restored → green evidence. Headline gap: the M40-F7 rollback test's trimmed-porcelain oracle could not distinguish staged from unstaged — the exact "test passes over a broken implementation" failure mode the charter named.

## 5 · Residue carried forward (declared, not silent)

- Fix 5's two golden observations (judgment tier, not law violations): bare-`start` orientation is byte-invariant across states (does not name the live task — long-shipped; a discoverability candidate, not a lie); the degenerate `milestone-execution` walk is now pinned *visible* (zero `Spawn:` lines) — its substance belongs to the M46 capability wave's scope.
- The (b)-adjudicated retitle first-field order-dependence: safe for every shipped schema; becomes real only under a third-party pack whose id-from field is not first — the multi-pack-platform non-goal's boundary, noted in [mutation-results.md](mutation-results.md).
- The chain-commit fold's abort-path trade — recorded at the fix-4 DECISIONS addendum.
- The sparse-checkout false-deletion conservative bound — pinned by its op-row test, declared at the oracle and Decision 7, re-weigh trigger "a sparse-checkout user exists".

## 6 · Final state

Final independent full-gate re-run at wave close: fmt clean · clippy `-D warnings` clean · full `cargo test` **2385 passed / 0 failed** · build clean. The version-stamp confirmation step caught that **M45's owed rc.9 bump never shipped** — the workspace bumped `1.0.0-rc.8 → rc.9` at `6cf6cd7` (the 12 version-bearing orientation goldens regenerated version-line-only; gate re-run green at the new version), and the release binary is built and installed to `~/.local/bin/jigc` as **`jigc 1.0.0-rc.9`**, so the final unseeded trial runs post-fix on an attributable stamp (the road-to-1.0 sequencing).
