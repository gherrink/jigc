# Phase 0 — the quick-fix doc↔code hole, reproduced host-side (no Docker)

**Written 2026-06-22.** The empirical case for Phase 2 (the universal finalize
floor). Reproduced deterministically against the installed `jigc 0.0.0` + `doc-code`
probe in a throwaway repo (`/tmp/phase0-repro`) — **no agent, no container**. The
pilot's container arms were only ever for *agent behavior*; the *mechanism* hole is a
pure CLI sequence, so it reproduces on the host.

## What the code path predicted (verified before running)

`engine::validate::validate_task` always calls `schedule_doc_code` — it is **not**
workflow-gated. But `schedule_doc_code` enumerates **the task's own effective-state**
code-anchor leaves. A `quick-fix` task creates only a `commit` doc, which carries **no
code-anchor**, so it yields **zero anchor pairs → doc-code is skipped**. The dangling
anchor lives in the *committed store's* arch-doc, which the **task-scope** sweep never
looks at. Only the **store-scope** twin (`jigc validate`) enumerates the committed
surface. So `finalize` (task-scope) is structurally blind to drift a task *causes* in a
doc it does not *touch*.

## The reproduction

Baseline: a repo with `src/parser.ts` exporting `class CommentBlockParser`, and a
**managed** `arch-doc` (`docs/architecture/parser-api.md`) whose component anchors
`implemented-by: src/parser.ts#CommentBlockParser`. `jigc validate` → **clean**.

Then drive the exact Sonnet path:

```
jigc start --workflow quick-fix "rename CommentBlockParser to BlockParser"
sed -i 's/CommentBlockParser/BlockParser/g' src/parser.ts   # committed anchor now dangles
git add src/parser.ts
jigc doc set-field commit:<t>#type --value refactor ; ... set summary ...
jigc task finalize <t>
```

## Result — the side-by-side

| step | command | result |
|---|---|---|
| **the hole** | `jigc task finalize` (quick-fix) | **committed clean** — `finalized 459cd47 — refactor: rename CommentBlockParser to BlockParser`; task-scope validate had **no findings** (it only saw the commit doc) |
| backstop | warn-only `pre-commit` hook | **detected, did not block**: `jigc: doc<->code drift detected … (commit not blocked).` |
| **the floor** | `jigc validate` (store-wide) | **caught it**: `blocking · doc-code.symbol-exists — anchor src/parser.ts#CommentBlockParser resolves to no symbol (CommentBlockParser is absent from src/parser.ts)` |

So jigc's own detection is **sound** — it fires the instant it's pointed at the
committed store. The defect is purely **where finalize points it**: task-scope, not the
code the task changed.

## The extra detail that names the gap precisely

Store-wide `jigc validate` prints, under the finding:

> `1 finding(s) — report-only at store scope (exit 0); these gate at jigc task validate / jigc task finalize.`

That promise is **false for the task that caused the drift**. The rename task touches no
arch-doc, so its `task validate` / `finalize` are blind; the drift persists in the
committed store until some *unrelated future task* happens to touch that arch-doc and
re-validate it. Drift you caused can outlive your commit indefinitely.

## What this licenses for Phase 2

The fix is not "make quick-fix run the probe" (it already would, if it touched an
anchor). The fix is: **`finalize` must run a store-scope doc↔code sweep over the code
the task changed — blocking — regardless of workflow.** Scope it to *changed code* (not
the whole store) for correctness-without-cost on a big store: enumerate committed
anchors whose target path is in the task's staged diff, and re-check those. That catches
exactly the drift a task causes in docs it doesn't touch, which is the entire hole.

Repro is disposable (`/tmp/phase0-repro`); these commands reproduce it from a clean repo.
