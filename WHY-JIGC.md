# Why jigc — the evidence dossier

**Purpose:** the collected, sourced case for using jigc — every claim below carries its evidence and its honest bound. This is the raw material for positioning/adoption material; it is *not* marketing prose. Updated as evidence lands (last: 2026-07-02, at the go-live adjudication). One rule governs this file: **no claim without a source, no source without its caveat.**

## The thesis, in one line

Take every *structural* operation away from the LLM and give it to a deterministic CLI; leave the LLM only the prose. ([VISION.md](VISION.md) — the determinism boundary.)

## The name

**jigc** — pronounced **"jig-see"** — is **a jig for coding agents**. In woodworking and machining, a jig is the device that holds the workpiece and guides the tool so the cut lands in exactly the right place, reproducibly, regardless of who wields the tool. That is the product: the CLI is the jig, the LLM is the tool — structure is held deterministically, the prose is the cut. A jig doesn't do the cutting, and it doesn't let the cut wander; it makes the *tool's* work land true. Second reading, equally sanctioned: **j**ust-**i**n-time **g**enerated **c**ontext — the CLI assembles exactly the context a task needs, when it needs it. (Settled 2026-07-03; availability verified clean on crates.io/npm/GitHub/web.)

## Measured results (jigc's own studies — controlled, but ours)

- **The cost win (M35, pre-registered, 2026-07-02).** On the rename surface, jigc is *strictly cheaper than a plain agent* on all three models tested (Sonnet 4.6 $0.080 vs $0.125 · Sonnet 5 $0.129 vs $0.262 · Opus 4.8 $0.193 vs $0.266 per rename) **and ≥ static on completeness** (1.0 vs 0.875/1.0), at 3.5–4.6 turns/rename vs 8.3–14.4 manual. Verb engagement was 100% (64/64) — availability *did* induce usage. *Bounds:* directional not powered (2–3 reps/cell; ordering identical in every rep), synthetic seed. ([completions/artifacts/M35/VERDICT.md](completions/artifacts/M35/VERDICT.md))
- **Enforcement holds where instruction decays (long-horizon replication, n=16/arm, 2026-06-22).** Over a many-edit sequence: drift rate plain **88%** · jigc **19%** · jigc-with-gate **0%** · static instruction **0%**. The honest reading both ways: jigc's gate arm matched the best static arm at zero drift, **and** a well-maintained static instruction also hit 0% on this task size — jigc's durable edge is that the guarantee is *salience-independent* (it doesn't decay as instructions age or context fills), which a single-task study cannot show decaying. ([completions/artifacts/differentiator-pilot-study1/REPLICATION.md](completions/artifacts/differentiator-pilot-study1/REPLICATION.md))
- **A newer model closes the correctness gap but widens the cost gap (M35 Sonnet-5 cell).** Capable models get renames *right* unaided — at 14.4 turns/rename. The cost win is the durable differentiator; detection is the capability-dependent backstop. Empirically-backed design rule: **own the verb, don't police the mistake.**

## Independent validation (evidence whose authors never saw jigc)

A 12-axis research corpus (external/web sources only, provably jigc-blind — zero jigc mentions) independently converged on jigc's four load-bearing bets:

- **The determinism boundary** — "an MCP server has no intelligence… never decides anything"; deterministic tool surfaces for agents (Anthropic tool-design guidance; research 06/08).
- **CLI-first as the agent channel** — "CLIs are the most context-efficient way to interact with external services" (Anthropic; research 08 §1).
- **Core/interface separation** — the engine/cli split matches the researched library-CLI discipline (research 06 §2).
- **Agent-native CLI surface** — structured output modes, distinct branchable exit codes, self-describing help (vendor checklists, research 08 §3). jigc ships `--format agent|json|human`, exit codes 0/1/2/3/4, and `jigc describe`.

Also independently corroborated: the one-bounded-primitive concurrency stance (Google Research: diminishing returns past ~5 agents; sequential degradation −39…−70%) and the small-batch/atomic-commit design (DORA 2025: under AI, throughput rises but stability needs the guardrail more). *Circularity caveat, stated once:* the same corpus's KB layer used jigc as its top exemplar — process/doc-hygiene agreements from that layer are jigc echoing jigc and are **not** cited here; only the jigc-blind research track is.

## Engineering facts (verified, current at 2026-07-02)

- **1427 tests, 0 failed, 0 ignored**; clippy `-D warnings` clean; 36/37 worked-example flows have real-binary acceptance coverage (the 37th is a human protocol).
- **Zero panics across ~35 hostile probes** (corrupt front-matter, invalid UTF-8, deleted section markers, git index.lock mid-commit, garbage payloads) — every failure path returns a handled, routed error.
- **Byte-stable writes** — `render(parse(x)) == x`, proptest-fuzzed on the highest-risk doctypes; human out-of-band edits are detected and routed (absorb / block / route-to-human), never silently merged or forbidden.
- **Transactional boundaries** — finalize and rename commit atomically and roll back cleanly on failure (rollback paths test-verified).
- **Frozen-v1 schemas with a real migration path** — schema changes are version-gated (pack-load hash assertion blocks un-migrated changes loudly) and `jigc migrate-corpus` carries a committed corpus across versions deterministically (measured v1→v2 facts, M34).
- **Scale sanity** — a 1000-doc corpus: `ingest` 30ms, `validate` 44ms.
- **Injection-safe git integration** — all production git calls are arg-vector `Command::new("git")`; no shell interpolation.

## The one-paragraph answer to "isn't this just another workflow framework?"

The critique ("orchestration frameworks create an illusion of work") is answered through its own rationale, with data: jigc's M35 study was *pre-registered to lose* if the framework only added ceremony — instead jigc was strictly cheaper than no-framework on every model, because it owns a structural operation the agent otherwise does by hand across 8–14 turns. Where the framework *would* add only ceremony (prose judgment, code correctness), jigc deliberately owns nothing: no DAGs, no runtime, no auto-authored prose, no LLM calls in the core. The boundary is the answer.

## Honest limits (say these before an adopter finds them)

- Prose quality and code correctness are the agent's, not jigc's — jigc validates structure and reality (refs, anchors, presence), never content.
- The adapter boundary is ergonomic, not enforced — a determined agent can edit files directly (the enforcement hook is a parked, evidence-gated decision).
- The value evidence is strongest on structural-op-heavy work (renames, supersessions, drift); on a single small task, a well-maintained static instruction ties or wins.
- Single-store concurrency is one agent per store (fan-out sub-agents excepted, by design).
