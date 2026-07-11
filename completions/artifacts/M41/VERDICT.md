# M41 — the rc.5 wave: completion verdict

**✅ SHIPPED 2026-07-11.** Built (9 risk-first increments via the milestone-build harness) + audited clean + 3 discovered defects fixed test-first + re-verified green. Base `368b5af` → HEAD `77543e5` (39 commits). **Full unscoped gate: 1740 passed / 0 failed**; `cargo fmt --check` · `cargo clippy --all-targets -- -D warnings` · `cargo build` all clean.

## What shipped (the five forks + the 13 findings)

- **Fork 1 — command-output contract v1** ([design/command-output-contract.md](../../../design/command-output-contract.md)): composed `jigc start --format json` carries `{task, text}` (minted id / `null` on the router arm); every write verb — including `create`/`add-item`/`author`, previously bare-string — emits a `DocAck` with the **decomposed** `target{doctype,slug,section?,item?,leaf?}` (add-item's target is the minted item); **findings-as-data on the successful write path** (scoped to the intrinsic `surplus-sections-absent`); a **stable `(code, target)` finding key** in URI normal-form with a collision-free per-family fragment (proven: a `0..*` two-dangling-ref sweep emits two distinctly-keyed findings, byte-identical across divergent source orders). Evolution posture declared.
- **Fork 2 — the advisory-route floor**: every finding carries a route, two kinds (repair / informational); the per-instance acknowledge-ledger deferred to 1.1 (trigger-keyed); the stable finding-key minted this wave (findings-as-data's contract citizen).
- **Fork 3 — symbol-granular Vue**: `<script>`/`<script setup>` extracted under the vendored `tree-sitter-typescript` + a filename-component unit; a fabricated `.vue` script symbol now **blocks** at the gate (proven through the real binary — the first Vue-symbol proof), a real one resolves. No new dependency.
- **Fork 4 — the `ValueRemapped` transform kind** + `D`/`I` → `Decision`/`Idea`: the first parameterized (authored-map) transform kind; the **first methodology v1→v2 corpus migration ever driven** — `migrate-corpus` remaps a committed `kind: D` byte-faithfully, idempotently, deterministically; the freeze gate (schema-version 2 + regenerated hash + a real v1 snapshot) respected.
- **Fork 5 — slug stopwords** + the V7 char-backstop word-boundary retreat.
- **The remaining findings**: V1 block-scalar templates (fold-safety proven end-to-end on **all 11** slot-bearing templates), V3/V6/V11 the `doc author` discoverability cluster (`doc schema` enum members + field→section, contract-version 1→2), V5 `set-field --unset` + N2 the `[]`-scalar reject, V8/V13 route repairs, V9 ingest grouping, V10 arch-doc ordering warning, V12 rename address ergonomics.

## Process integrity — three defects caught and fixed, not shipped

- **Cross-model (codex) pre-build review** of the command-output contract (`368b5af`) caught real stable-key collisions the same-model design-review missed (inverse-cardinality per-relation, schema-conformance/section-missing fragments, mention dedup, file-state path exception) — verified against the code, baked before build.
- **Build-phase halt (Increment 8, correctly escalated):** F5's edge-stopword drop had silently broken `slugify` idempotency, with a *wrong rationale* in its own DECISIONS entry, un-reconciled with the standing `idempotent` proptest. The build agent refused to mask it; resolved the robust way — restored the invariant (edge-drop after the char cap), corrected the record, added a deterministic regression test (`fe53318`).
- **Completion-audit LOW findings (2), fixed test-first:** LOW-2 — the `ref-resolves` key carried a slug-only target resting on an unenforced single-target-type assumption; now the full `<type>:<slug>` identity (`0f6c1de`). LOW-1 — the flagship V1 fix had round-trip proof for only 2 of 11 templates; extended to uniform parametrized coverage (`77543e5`), and the fixer caught a `contains`-vs-per-slot-count masking hole in its own first cut.

## Audit summary

- **Code-review:** deliverable holds; no correctness bugs, no invariant violations, no scope dishonesty, no masking goldens. Engine makes no LLM calls (the `ValueRemapped` map is a CLI-authored deterministic input); composition deterministic; finalize transactional; the freeze gate respected; every OOB-reachable parser (the Vue extractor, the tree-sitter resolvers) verified panic-free.
- **E2e (real binary, throwaway repos):** every done-picture arm passed — composed task-id, decomposed write-acks across all verbs, findings-as-data + the stable key, the two-divergent-order determinism check, `doc schema` contract-v2, the byte-faithful `D`→`Decision` remap, and the fabricated-`.vue`-blocks proof.

**Next:** version bump + install **1.0.0-rc.5** → the adoption trial's **implementation half** on rc.5 → its log analysis → the **1.0.0 call**.
