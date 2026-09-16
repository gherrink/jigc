# The per-axis review instrument — reusable, not per-trial

The eight Codex source-pass prompts (`axis1-prompt.md` … `axis8-prompt.md`) and the Workflow script
that drove the M51 review (`per-axis-review.workflow.js`: eight Opus drivers → eight reconcilers →
one assembler) live here so the next wave re-runs the review instead of rebuilding it. Everything
in them that is M51-specific is a path or a version string: the scratchpad root, the installed
binary's expected `--version`, the commit the record names, and the VERDICT the drivers read for
the fixes that landed after the build. Re-point those and run:

1. the eight Codex passes, in parallel, each `codex exec -s read-only -o <out> - < axisN-prompt.md`
   (they read production source only, so they need no installed binary and can start early);
2. the Workflow (`Workflow({ script: <this file's body>, … })` from the session, with the scratchpad
   and binary paths edited) once the wave's rc is built and **installed**;
3. commit the assembled directory the assembler writes.

The reconciliation rule is in [acceptance-design.md](../../acceptance-design.md) and is not
restated here: *a claim by one that the other cannot reproduce is a lead, not a finding.*
