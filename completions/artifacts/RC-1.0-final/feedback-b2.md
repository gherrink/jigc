Going back through the transcript rather than general impressions — here's what actually happened, command by command.

  1. What confused me

  - The split between jigc task list (enumerates tasks) and jigc milestone list-tasks (no plain jigc milestone list) is asymmetric enough that I guessed wrong on both sides — see (3).
  - There are two similarly-named renaming surfaces: jigc doc rename (task-scoped, for an uncommitted/staged doc identity) vs. a separate task-less jigc rename (for a committed identity). I never mixed them up, but
  only because I read jigc doc rename --help first — the names alone don't make the distinction obvious.
  - Bare jigc start returning "state": "clean" while an open task (record-the-ingest-queue-overflow) sat in .jigc/tasks/ was confusing in the moment — "clean" reads as "nothing pending," and the skill doc had told
  me bare start "reports project state and the next action."

  2. What jigc told me that turned out wrong or misleading

  - That "state": "clean" orientation is the concrete case: the skill text promises bare jigc start reports "the next action," but the JSON gave a plain workflow menu plus next_steps: [planning, ingest-existing] —
  nothing pointing at the half-done task. I only learned the task existed by listing .jigc/tasks/ directly, and separately from a note buried in jigc doc list's output. Nothing jigc told me was factually false, but
  the top-level orientation omitted the one thing most relevant to "pick up unfinished work," which is worse than a wrong flag name since it's not obviously an error to correct.

  3. What I looked for and didn't find

  - A jigc task show <id> for full task detail, mirroring jigc doc show — doesn't exist. It errored (unrecognized subcommand 'show') and routed me to piece task state together from jigc task list (id/workflow/intent
  only) + jigc task validate (findings only) + jigc task diff (changeset only). No single call gives "everything about this task."
  - A way to enumerate milestones project-wide. jigc milestone list doesn't exist (unrecognized subcommand 'list', tip suggested list-tasks, which by its name sounds scoped to one milestone's sub-tasks, not a
  milestone listing) — I never actually found or tried whatever the right verb is, because I didn't need milestones for this work.
  - Confirmation that freeform markdown links between doc slots (I wrote [Research: ...](../research/how-comparable-services-bound-distinct.md) inside the ADR's context slot, referencing another doc by relative
  path) are actually checked for existence the way MIGRATING.md's "dangling forward reference" language implies. finalize went through clean, but I never tried the negative case (a broken link) to see if it's real
  validation on freeform prose citations, or only on structured ref fields like supersedes/cites-code. I don't actually know which is true.

  4. What I did around jigc, and why

  - ls -la /work/.jigc/tasks/ — a direct filesystem listing of jigc's own task workbench, not a jigc verb. I did this because bare jigc start hadn't surfaced the open task and I needed some way to confirm one
  existed before trusting jigc doc list's incidental note. It's not a managed-doc file (AGENT.md's "never read directly" rule is scoped to jigc doc list's committed set), so it's a gray area rather than a clear
  violation, but it was still me going outside the CLI to get orientation the CLI was supposed to give me.
  - git log --oneline -- src/ingest.ts and, at the end, git log/git status — read-only inspection of commit history and working-tree state, never used to read or write a managed doc's content. Everything that
  actually touched a managed doc (reading, editing, committing) went through jigc doc show/set-field/set-slot/author/rename and jigc task finalize. No raw edits, no raw commits.