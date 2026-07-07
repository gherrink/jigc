# M39 completion verdict — the pre-1.0 RC-findings wave

**Verdict: ✅ SHIPPED** (2026-07-07). Built (8 increments), independently audited (code-review + e2e), all confirmed findings fixed, re-verified green. The last build milestone before the rc.3 → adoption-trial → 1.0.0 sequence.

## What shipped (8 increments, base `09e8a82` → HEAD `ac37fb6`; 43 commits, 86 files, +9062/−521)

1. **`jigc doc show`** — the committed-doc read surface (`read_slice` extended to whole-doc + item/leaf/deep slices) + a **pinned 1.0 `--format json` contract**. Closes the declared "reads *and* writes" hole (an agent can now read every committed instance the adapter rule forbids reading directly).
2. **Compose renders ALL grounded sources** — multi-target `walk_edge` (order-invariant, byte-identical), revising the M37 first-only design + flipping its two pinning tests.
3. **The `milestone-record` doctype (engine core)** — `set: on-transition` machine-maintained leaves (append + in-place-mutate arms) + **read-back authority** (the committed `.md` is the source of truth; `.jigc` JSON demoted to a rebuildable cache).
4. **Team-ready layout** — `milestone_dir` record/WIP split, path-scoped commit-at-each-op, gitignore 3→1, fresh-clone re-derive + resume, OOB conflict-block, stale-base route. The `.jigc`-is-the-workbench / repo-is-the-record principle made real.
5. **The freeze-exempt relocation floor** — re-keyed orphan detector, extracted move primitive, parallel freeze-exempt path, `config set docs-root` detect+route+move, foreign-squatter-into-workbench. Closes the owed `storage.md:148` strand.
6. **Invocation-log fd-tee output-size** — true stdout+stderr byte total (incl. clap `--help`). The adoption-trial prerequisite.
7. **Minting ergonomics** — ~5-word slug cap, `--slug` on `start`/`doc create`, form-vision empty-research advisory.
8. **Acceptance + reading-order doc fold-back** (`flow40_acceptance.rs` + the design-of-record spread).

## The mid-build fork (surfaced by the harness, human-decided)

Per-op record commits advance HEAD, colliding with `plan_milestone_finalize`'s `base == HEAD` guard (the M31 worktree-combine drift protection); `record.base` is structurally always behind HEAD, so the guard could never pass in this model. **Settled (human): relax the guard to advance `base → HEAD` over a linear record-only range; any foreign commit still blocks** — external drift stays caught, the milestone's own bookkeeping is tolerated. Fixed test-first (`0790bcb`), recorded in DECISIONS + [team-ready-state.md](../../../design/team-ready-state.md). ([DECISIONS.md](../../../DECISIONS.md) → 2026-07-07 M39 build: the milestone-finalize base-guard refinement.)

## Audit (independent, adversarial)

- **Code review — deliverable holds.** One LOW finding.
- **E2E — overall pass.** Drove all 8 increments through the real binary in throwaway repos. One minor contract over-claim finding.

## Findings → fixes (5 post-audit commits, all test-first, sequential, gate-green)

| # | Finding | Fix | Commit |
|---|---------|-----|--------|
| 1 | Slug word-cap dropped the char-length ceiling for single-word id-sources → unbounded filename (a regression this milestone introduced) | Restored a 50-char filename backstop after the word cap; hardened the proptest that missed it | `02213f3` |
| 2 | `doc show` `#section/<item>/<leaf>` over-claimed field-leaf slicing on the 1.0 witness (returned `store.no-such-leaf`) | Made field-leaf + id-from slices resolve the value (contract made true, not narrowed); absent-leaf still blocks | `e2a4c28` |
| 3 | `author-vision` pack step still told the agent "only the first source appears" — contradicted the shipped all-source render | Aligned the guidance prose | `f9db38b` |
| 4 | `doc show vision:<any-slug>` returned VISION.md exit 0 (placement singleton ignored the slug) | Read-path rejects a non-canonical singleton slug, routes not-found | `0d8fcb1` |
| 5 | (Contract decision, human) the pinned json rendered `base` as a fragile space-joined `"<sha> <short>"` string | Renders structured `{sha, short}` (`.md` scalar + round-trip unchanged) | `ac37fb6` |

The base-json refinement resolved the Inc-3 audit advisory that flagged the compound base; the human chose the structured shape from three previewed options ([DECISIONS.md](../../../DECISIONS.md) → 2026-07-07).

## Re-verify (the acceptance gate)

Full unscoped gate at HEAD `ac37fb6`: `cargo fmt --check` clean · `cargo clippy --all-targets -- -D warnings` clean · `cargo build` clean · `cargo test` → **1583 passed, 0 failed across 160 test binaries** (incl. the `--bin jigc` pack/describe goldens). Tree clean.

## Honest bounds (carried into the adoption trial)

- **Headless fan-out.** The e2e auditor is headless, so the milestone-record's team-ready arc under a *real concurrent* fan-out (the assistant's Task-tool spawn) was proven via an N-process binary simulation + verbatim spawn-template execution — not a live spawn. Accepted (human); confirm under the real adoption trial. This is the milestone-completion-workflow's flagged spawn-class caveat, honestly not-run.
- **Team-ready is methodology-composed.** The milestone-record is a freeze-exempt methodology-pack doctype, so the team-ready Proves are reachable for `[dev ▸ methodology]` projects (the jigc self-host + the RC-trial composition), not a bare dev-pack adopter — by design.

## Owed / flagged (not blocking)

- **Planning-gate-record doctype** stays condition-keyed (M39 did not re-encode the planning workflow, so it did not reopen).
- **Per-developer/team-layer config distribution** stays deferred (state legibility ≠ config distribution).
- The M39-adjacent idea pool (`state-aware-compose` stretch → 1.1; `task-record-graduation` validate-first) is unchanged.
