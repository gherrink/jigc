# Handover — next session: the cross-doc forward-ref integrity study (the "harder claim")

Written 2026-06-23. Branch **`study/long-horizon-many-edit`**, tree clean, **NOT pushed**.
This session ran the long-horizon many-edit study (the founding empirical claim), closed
the one anti-jigc result it found, and scoped the next task. The next session **builds an
approved engine increment, then pre-registers + runs the decisive value study**. Start cold;
everything you need is below or in the linked files.

## What this session settled (don't re-derive)

1. **The long-horizon study ran and is the project's first controlled jigc-beats-static
   win** — for Sonnet, on doc↔code, in the long-horizon many-edit regime. Read
   [VERDICT.md](VERDICT.md) → [results.md](results.md) → [pre-registration.md](pre-registration.md).
   Headline: jigc held drift flat at 0 over 8 edits while plain compounded to 5.0 and every
   static arm shipped *residual* drift it structurally couldn't see (the file-move case:
   6/9 Sonnet static reps shipped a dead move-path; jigc 0/3). The win is **hook-driven, not
   engagement-driven** (jigc forced repair even on edits where the agent ran zero jigc commands).
2. **The one anti-jigc result (Opus prose drift) is closed.** Opus cleared the anchor gate
   and left stale component *titles* → jigc ended dirtier than static. Fixed with the
   pack-declared **`title-names-symbol`** check (commit `9d99924`); re-test dropped Opus stale
   prose **3.0 → 0.5** (commit `1dd5c08`). Read [title-fix-retest.md](title-fix-retest.md).
3. **The general lesson** (now the project's working thesis, in VISION.md proven-section):
   *jigc's value is the salience-independent enforcement of a **checkable surface** — extend
   the checkable surface and the win extends with it.*
4. **Value gate: partially cleared** — "proven-in-regime," not unconditional. The win was on
   doc↔code, which a static rule *could* express. The next study tests the **harder claim**.

## This session's commits (on the branch, unpushed)

```
ea2ba68 docs(decisions): OAuth token-expiry recorded as a closed known issue
1dd5c08 study(long-horizon): title-fix re-test — Opus prose 3.0 → 0.5
9d99924 feat(validate): arch-doc title-names-symbol — close the prose blind spot
75261a7 study(long-horizon): empirical superiority tested — qualified yes / self-refuting
```
Decisions log: three dated `DECISIONS.md` entries (2026-06-23). VISION.md proven-section updated.

## THE NEXT TASK (chosen with the user): the "harder claim"

**Does jigc beat a static `CLAUDE.md` on a differentiator a static file *structurally cannot
replicate* — cross-document forward-reference integrity** (the `supersedes` / edge-index
advantage)? Unlike doc↔code (a static rule *can* say "keep citations honest"), **no instruction
makes a cold agent verify that an ADR's `supersedes` ref three docs away still resolves across
the whole store.** A win here is a *capability gap*, not regime-bound — decisive for the value gate.

It is **two steps**. Step 1 is APPROVED to build (no API cost). Step 2 needs pre-registration +
the user's sign-off before any paid run.

### THE KEY DISCOVERY that shapes Step 1 (verified this session, in code)

`schema-conformance.ref-resolves` (the forward-ref integrity check — "this `supersedes` target
exists") fires **only at `jigc task finalize`, scoped to task-touched edges**
(`crate::index::ref_resolves`, called from `crate::validate`). The **store-wide `jigc validate`
sweep (`validate_store_families`) runs three families — doc↔code, workflow↔refs, file-state —
but NOT ref-resolves.** So a dangling `supersedes` in the committed store is **invisible to
`jigc validate`, and therefore to the pre-commit hook** (which keys on `jigc validate` findings).

Since the study's whole enforcement model is the **salience-independent pre-commit hook** (agents
bypass `finalize` — the decisive finding of the last study), the cross-doc differentiator is
**not hook-enforceable today.** Step 1 closes that gap. This asymmetry (doc↔code swept store-wide
since M18–M20, ref-resolves never) is itself a real architectural finding.

### Step 1 — engine increment: store-wide `ref-resolves` (APPROVED; build first, test-first)

Add a 4th family to the store sweep. Full design, verified buildable:

- **`crates/engine/src/index.rs`:**
  - Extract `committed_reachable(to, repo_root, schemas) -> bool` from the surface-a half of
    the existing private `target_reachable` (lines ~392–413): split `to` on `:`, look up the
    schema, `crate::store::canonical_path(repo_root, schema, slug).exists()`. Refactor
    `target_reachable` to call it (keep its surface-b working-area check).
  - Add `pub fn ref_resolves_store(committed: &EdgeIndex, repo_root, schemas) -> Vec<Finding>`:
    for each `edge` in `committed.edges`, if `!committed_reachable(&edge.to, ...)` push
    `dangling(edge)` (the existing private finding builder, lines ~417, emits
    `schema-conformance.ref-resolves`). Doc it as the store-wide analog of `ref_resolves`.
  - `rebuild_committed(repo_root, schemas, head) -> EdgeIndex` (line 171) is a **pure builder,
    no side effects, no save** — pass any stamp (e.g. `"store-sweep"`); you're not persisting.
- **`crates/engine/src/validate.rs` → `validate_store_families` (line ~391):** add Family 4
  after file-state:
  ```rust
  // Family 4 — cross-doc forward-ref integrity (store-wide analog of the task-scope
  // ref-resolves finalize gate): every committed forward edge's target must resolve in
  // the committed store. Closes the doc↔code-vs-ref-resolves store-sweep asymmetry so the
  // salience-independent pre-commit backstop reaches a dangling cross-doc ref.
  let committed_index = crate::index::rebuild_committed(repo_root, schemas, "store-sweep");
  findings.extend(crate::index::ref_resolves_store(&committed_index, repo_root, schemas));
  ```
- **Test to update:** `validate_store_folds_three_content_families` (validate.rs ~line 3795)
  asserts three families — extend it (or add a sibling) to cover the 4th: a committed store
  where adr-b `supersedes: adr:adr-a` and adr-a is absent → expect one
  `schema-conformance.ref-resolves` blocking finding. RED first (no finding today), then GREEN.
- **Verify serialization for the hook:** the finding code is `schema-conformance.ref-resolves`
  → `split_code` gives `probe="schema-conformance"`. **The study's blocking hook currently greps
  `"probe":"doc-code"` only** — so for Step 2 the hook variant must also key on
  `"probe":"schema-conformance"` (or on `ref-resolves`). Update
  `~/lh-study/harness/blocking-pre-commit` accordingly (one extra grep alternation). Note this
  in the study pre-registration as a study-harness config, not a jigc change.
- **Gate (the dev-workflow):** `cargo test` · `cargo clippy --all-targets -- -D warnings` ·
  `cargo fmt --check`. Then `cargo build --release -p cli && cp target/release/jigc ~/.local/bin/`
  (doc-code probe unchanged — this lives in the engine). Commit on the branch.
- **Honest design notes for the commit + DECISIONS:** is the store-wide ref-resolves
  *blocking* (matches the finalize gate) or *advisory* at store scope (like minimum-cardinality)?
  The finalize gate is blocking; mirroring it store-wide is the consistent choice and what the
  hook needs. Confirm there's no decisions-pending entry forbidding a store-wide ref sweep before
  shipping it blocking; if found, engage its rationale (the record is rebuttable).

### Step 2 — the study (pre-register → sign-off → run)

Pre-register exactly as the last study did ([pre-registration.md](pre-registration.md) is the
template). The design, ready to flesh out:

- **Twin:** a multi-doc managed store of ADRs with `supersedes` chains + cross-refs (build with
  `jigc doc create adr` + `set-field …#supersedes`). Reuse the `gherrink-ui-doc @ 542b320` base
  or a simpler synthetic repo — the differentiator is the *doc graph*, not the code. Seed ~6–10
  ADRs where B supersedes A, C supersedes B, etc., plus the arch-doc `cites → adr` edges.
- **Edit sequence (the many-edit axis):** N sequential tickets, fresh cold agent each, that
  **dangle a forward ref across docs** — e.g. "delete/rename ADR-A" (now B's `supersedes`
  dangles), "split ADR-C", "renumber a decision." Each edit creates a cross-doc dangle 1–3 hops
  from where the agent is working — the thing instruction can't make the agent check.
- **Arms:** (A) jigc-hook (Step-1 store-wide ref-resolves, hook keys on `schema-conformance`)
  vs (C) static `CLAUDE.md` stating "keep all cross-references valid" + the dilution ladder
  (reuse `~/lh-study/arms/C40|C160|C550`) vs (P) plain. The decisive point: **C's rule cannot
  actually be executed by a cold agent** — verifying a store-wide ref graph is not an
  instruction-followable act. Expect static to fail where jigc holds.
- **Models:** Sonnet primary (R=3); Opus on extremes (R=2) — same shape as last study.
- **Oracle:** extend `~/lh-study/harness/measure.py` to **walk the edge graph** — parse each
  doc's `supersedes`/`cites` refs, check each target doc exists in the store, count dangling
  cross-doc refs per committed HEAD (the cumulative-drift curve, exactly like the anchor curve).
  Drive `jigc validate` as the arm-A oracle (now reports ref-resolves after Step 1).
- **Blind judge:** Codex (cross-model, worked this session). **Caveat learned:** the judge
  mis-flags stable lowercase `{#slug}` ids as symbols — for ref-integrity judging, give it the
  doc-id list explicitly and ask "does any `supersedes:`/`cites:` value name a doc not in this
  list." Keep objective oracle canonical ("objective first").

## Auth handling for Step 2 (the user's answer to the open question)

The OAuth token mounted into containers **expires mid-run** (short, unstable lifetime; recurred
3× this session — see DECISIONS 2026-06-23, closed-not-investigated). The user will **manually
re-login if needed**. Operational protocol for the paid runs:

1. **Verify auth live immediately before launch** — the cheap probe (worked this session):
   ```sh
   docker run --rm -v $(readlink -f ~/.local/bin/claude):/usr/local/bin/claude:ro \
     -v ~/.claude/.credentials.json:/home/node/.claude/.credentials.json:ro \
     lh-toolchain bash -c 'claude -p "say OK" --model claude-haiku-4-5-20251001 \
     --permission-mode bypassPermissions --output-format json 2>&1' | grep -oE '"is_error":[a-z]+'
   ```
2. **Always integrity-check after a run:** count `authentication_failed` in transcripts +
   `turns==1,cost==0` dead edits (the runner records these). Discard + re-run any 401-corrupted
   sequence on fresh auth — never let a 401 run reach the verdict (this session's runner +
   `eval-sequence.py` already surface it).
3. **Fallback to consider:** the Sonnet matrix (15 seq) finished inside one token window;
   Opus (slower) is what got caught. Prefer **launching Opus in small batches** (1–2 sequences)
   so an expiry truncates one sequence, not five — or checkpoint per-sequence. Ask the user to
   re-login right before the Opus batch.

## The reusable harness (built last session, all under `~/lh-study/`)

- `harness/run-sequence.sh` — one rep: fresh cold agent per edit over an **evolving** twin
  (bind-mounted rw repo; measures committed HEAD; resets tree between edits). Container gotchas
  baked in (USER node, `--permission-mode bypassPermissions`, prompt via `-e PILOT_PROMPT`).
- `harness/run-matrix.sh` — bounded-concurrency matrix driver.
- `harness/measure.py` — arm-agnostic drift oracle (doc-code probe + dead-name grep). **Extend
  for edge-walking in Step 2.** Has `--selftest`.
- `harness/eval-sequence.py` — aggregates → cumulative-drift curves + rates.
- `harness/judge.py` — blind Codex judge (`codex exec --skip-git-repo-check`, prompt via stdin).
- `harness/blocking-pre-commit` — the blocking hook (1-line delta from jigc's warn-only). **Add
  `schema-conformance` to its grep for Step 2.**
- `harness/build-arm-A.sh` / `build-templates.sh` — twin builders (arch-doc authoring via jigc).
- `harness/sequence.json`, `harness/prompts/` — the doc↔code study's 8 edits (template for the
  new cross-doc sequence).
- **Twin workspace `~/lh-study/` is throwaway** and may be partially stale; the toolchain image
  `lh-toolchain` (node:20-trixie-slim + git + ripgrep, USER node) may need a rebuild
  (`docker build -f ~/lh-study/docker/Dockerfile.toolchain -t lh-toolchain ~/lh-study`).
  The exported artifacts under `completions/artifacts/long-horizon-study/` are the durable copy.

## Reading order for the next session

This file → [VERDICT.md](VERDICT.md) → [title-fix-retest.md](title-fix-retest.md) →
[results.md](results.md) → `DECISIONS.md` (the four 2026-06-23 entries) → VISION.md
proven-section → then the Step-1 code sites (`crates/engine/src/index.rs` `ref_resolves` /
`target_reachable` / `dangling`; `crates/engine/src/validate.rs` `validate_store_families`).

## Out of scope / do not

- **Don't go productive** (commit a real corpus / self-host) — M33 (schema freeze) + M34
  (corpus migration) still gate it; this study is what *might* fully clear the value gate first.
  Twin-only, plain-file export, zero lock-in.
- **Don't re-run the doc↔code study or re-open the title fix** — both are done, gated, committed.
- **Don't investigate the OAuth expiry** — recorded as a closed environment issue; just verify
  + re-login around runs.

## Success criterion for the next session

Step 1 built, gated, committed (store-wide ref-resolves enforceable by the hook). Then a
**pre-registered, signed-off cross-doc study** run end-to-end with the edge-walking oracle + the
blind judge — yielding a defensible verdict on **the harder claim**: does jigc's enforcement of
a *non-instruction-replicable* surface (cross-doc ref integrity) beat a static `CLAUDE.md`,
decisively (not regime-bound)? Whichever way it falls, it's the answer that gates productive-go.
