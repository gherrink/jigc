# The per-axis review instrument — the M52 re-run's copy

This is [M51's instrument](../../../M51/per-axis-review/instrument/README.md) re-pointed for the M52
re-run (2026-09-21, installed `jigc 1.0.0-rc.16` from `a3eb026b`): the eight Codex source-pass prompts
and the Workflow script (eight Opus drivers → eight reconcilers → one assembler). What changed against
M51's copy, so the next wave re-points the same things: the scratchpad root and the expected
`--version`; the drivers read M52's `acceptance-design.md` and `VERDICT.md` beside M51's Part 2 axis
rows and must **re-drive every M51 §A row for their axis first** (the row-by-row comparison is the
re-run's first deliverable); the assembler compares the coverage table against M51's *driven* table
rather than the planned one; each reconciler **waits for its Codex file** (`until [ -s … ]`) instead of
assuming it exists; and each Codex prompt carries a lead-in naming the registries the wave minted.

**Two lessons this run paid for, recorded so they are not re-learned.** (1) The Codex passes run
`codex exec -s read-only`, so **a prompt must never tell Codex to write a file** — the first run's
prompts did (a prepended *"write your pass to the path the harness names"*), the sandbox refused, and
the captured final messages were one-paragraph summaries instead of the reports; the passes were
re-run with the instruction *your entire report is your final message*, and the summary-only outputs
are not part of the record. (2) The reconcile phase starts the moment its driver returns, which can be
before Codex finishes — hence the wait guard.

Run order and the reconciliation rule are as M51's README states; they are not restated here.
