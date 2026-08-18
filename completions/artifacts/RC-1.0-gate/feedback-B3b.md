Here's a straight accounting from this session, organized as asked.

  1. What confused me

  - The router menu itself. My very first intent ("add a changelog entry") got the generic 12-item catalog, which doesn't include record-change — that workflow is deliberately hidden from the router (it's reached
  "by name" since routine changelog entries normally ride single-task's gate instead). For a standalone changelog-only task with no code change, nothing in the presented menu actually fit well, and I only found
  record-change by reading jigc describe's full workflow list, not from anything the router surfaced.
  - Predicting minted task IDs. The slug jigc mints from an intent string isn't a simple truncation I could reliably guess — spec-bound-how-many-distinct, implement-the-distinct-series-cap,
  implement-restart-durability-from are all different lengths/word-cuts. I guessed wrong once (implement-restart-durability-from-the) and got a no task error until I went back and read the .task field from the
  actual JSON output instead of predicting it.
  - --task read semantics tripped me up once: right after jigc task bind spec ... , I tried jigc doc show spec:<slug> --task <id> to pull the criteria IDs, and it refused with store.not-staged because binding
  doesn't stage a copy — I had to drop --task and read the committed copy instead. Sensible in hindsight, not obvious in the moment.
  - The same advisory finding — schema-conformance.unadopted-instance on the pre-existing docs/decisions/0002-keep-the-sample-store-in-memory.md — resurfaced on every validate/finalize call across three unrelated
  tasks before I finally adopted it. It's non-blocking and harmless, but seeing an identical finding reappear on unrelated work made me re-check each time whether it was something new.

  2. What jigc told me that was wrong or misleading

  Nothing I can point to as factually incorrect. The closest thing is the router-menu gap in (1) — the catalog it showed me was accurate for what it contained, it just didn't contain the workflow that turned out to
  be the best fit, which reads as misleading-by-omission rather than jigc stating something false.

  3. What I looked for and didn't find

  A batch way to set maps-to-test (or any single field) across several already-existing items in one call. jigc doc author batches whole-document creation, but its own rule is that a payload item whose title matches
  an item the doc already holds gets refused (write.already-present) — so for a spec with 4–5 committed criteria, wiring each one's maps-to-test back to its test still meant five separate jigc doc set-field calls,
  one per criterion, with no bulk-edit verb for existing items. Might well be deliberate (author is create-shaped, not patch-shaped), but I went looking for a shortcut and didn't find one.

  4. What I did around jigc instead of through it

  Only one thing touched a managed-adjacent doc directly: I used Read on docs/decisions/0002-keep-the-sample-store-in-memory.md before it was adopted, to understand the prior in-memory-storage decision before
  scoping the durability spec. At that point jigc doc list showed no managed docs yet, and jigc ingest later confirmed it as unregistered/foreign — AGENT.md's own rule is that an unregistered file is "readable
  directly until adopted," so this was sanctioned, not a bypass. I never edited it directly; once I needed to change its status, I went through jigc migrate ... --as adr and jigc doc set-field inside the task.

  Everything else outside jigc was either plain source/test code (explicitly free territory) or read-only git research (git log, git tag, git status) to reconstruct version/commit history for the changelog and to
  sanity-check the working tree — never a git commit, git mv, or direct edit on anything under docs/ or CHANGELOG.md. The one git add I ran directly was staging code edits before jigc task finalize, which is the
  documented flow, not a workaround.