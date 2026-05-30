# Codex cross-model review — 2026-05-28

Cross-model second-opinion review of the full doc set (VISION + CLAUDE + DECISIONS + design/ + implementation/, 14 files, ~1,634 lines) by OpenAI Codex CLI. Five passes, each with its own lens; findings preserved verbatim per the `review-codex` skill ("the value of cross-model review is in the disagreement, so blending the two perspectives destroys what makes it useful").

## The five passes

| # | Pass | File | Lens |
|---|---|---|---|
| 1 | Consistency & terminology drift | [consistency.md](consistency.md) | Does the doc set agree with itself? |
| 2 | Completeness & coverage gaps | [completeness.md](completeness.md) | What's load-bearing but undefended, or unowned? |
| 3 | Invariant & determinism stress test | [invariants.md](invariants.md) | Adversarial — try to break the 8 invariants + boundary using only the docs. |
| 4 | MVP red-team | [mvp-redteam.md](mvp-redteam.md) | Does the thickened MVP actually prove its five claimed differentiators? |
| 5 | Implementation ↔ design fit | [implementation-fit.md](implementation-fit.md) | Do `implementation/` choices faithfully serve `design/` commitments? |

Each finding has: severity, location citations (file + section/line), one-sentence "why it matters", and a one-line proposed fix. Passes are independent — tackle in any order.

## Cross-pass themes worth prioritizing

Issues that surfaced from multiple passes' independent angles (= load-bearing):

1. **Emitted workflow micro-syntax is open and load-bearing.** Passes 1, 2, 3, 4. The MVP compose-loop ships through this surface; can't freeze the MVP without freezing this.
2. **`tool start` semantics drift across CLAUDE / write-commands / assistant-adapter.** Passes 1, 2, 3, 4. Bare-`start` orients vs. mints; `--orient` is referenced but undefined.
3. **Slot-prose ↔ CommonMark / final-bullet-list field-group ambiguity.** Passes 3, 5. Determinism-boundary leak with no clean escape hatch — slot prose can perturb structure, prose lists can be silently promoted to fields.
4. **No owned `finalize` transaction spec.** Pass 2 calls for `design/finalize.md`; Pass 4 says git-staging precision is "outside MVP but secretly depended on."
5. **Cross-task forward-ref deadlock for `supersedes`.** Pass 3 walked the T1→T2 scenario; Pass 4 says the differentiator is only PARTIALLY PROVEN with a self-relation that's also optional.
6. **Workflow front-matter is YAML while doc-instance front-matter is flat — same term, two parsers.** Passes 1, 5 (Pass 5 red).

Single-pass items that are still high-severity (one source isn't lower confidence — Codex didn't get to look at the same area from multiple angles):

- CLAUDE.md's "Project state" paragraph is stale (says repo has only VISION.md). [consistency.md]
- ID minting sites: structural grammar says exactly two, write-commands names a third (task IDs). [consistency.md, invariants.md]
- Relations as field-backed edges aren't end-to-end bound in the schema format. [completeness.md]
- Override application order during workflow composition has no single algorithm. [completeness.md]
- OOB import / conformance / writeback boundary is inconsistent across docs. [completeness.md]
- Fan-out collision suffixing only rewrites local self-refs; cross-area guessed sibling refs can dangle. [invariants.md]
- Pack-default home for non-CLI frontends (MCP) is foreclosed by embedding only in `cli`. [implementation-fit.md]

## Notes

- The `tool` placeholder is the unnamed CLI throughout these reviews — same as in the docs.
- Codex was given absolute paths to all 14 files and read them itself; no doc content was pasted into prompts.
- Each pass requested findings + a one-line proposed fix (per Maurice's instruction).
- Raw Codex stdout is preserved in `/tmp/codex-pass{1..5}.out`; these review files are formatted reproductions.
