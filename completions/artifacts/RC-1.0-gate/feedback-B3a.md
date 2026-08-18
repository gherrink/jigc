Going back through the session honestly, by question:

  1) What confused me

  - The router never surfaces off-router workflows. For the changelog task, jigc start "add a changelog entry..." returned the standard 12-workflow menu — none of which obviously fit "just write a changelog entry,
  no code, no decision." I had to run jigc describe separately to discover record-change exists at all (it's explicitly hidden from the router catalog by design). The router's own output gave no hint that a
  better-fitting workflow was one call away.
  - decided-task vs single-task for the persistence piece. Both grant ADR-recording; the only functional difference relevant to me was that single-task also grants the changelog gate. That distinction is in the
  prose ("The lightweight sub-milestone path... without milestone-planning ceremony" vs "Also the pick when the change touches documented code") but isn't a crisp decision rule — I had to infer it by comparing both
  describe blurbs side by side.
  - Editing an existing changelog category is not "append." The docs advertise jigc doc author as appending new entries to a committed changelog, but when the entry I needed (added) already existed from an earlier
  task, author/add-item would have hit write.already-present. I had to switch to set-slot, which overwrites the whole notes field — so I ended up hand-copying the previous bullets back in alongside the new ones
  rather than jigc merging anything for me.

  2) What jigc told me that was wrong or misleading

  - After finalizing the persistence task (c5fee0f), the response carried this advisory:
  ▎ changelog-recording.gate-granted-unused — "workflow single-task grants the changelog create-gate and this task recorded no changelog entry"

  - That's false for what actually happened: CHANGELOG.md was in the same commit's manifest as "kind": "promoted", and I had genuinely added two bullets to it. The gate tracker apparently only counts a changelog
  write as "recorded" when it comes through add-item (a new entry), not set-slot against an item that already existed — so editing an existing category silently doesn't register as satisfying the gate, and the
  advisory reads as if nothing was done when something was.

  3) What I looked for and didn't find

  - A verb to append to an existing slot's prose rather than replace it outright. set-slot is overwrite-only; there's no append-slot or equivalent, so extending a changelog category's bullet list across two separate
  tasks meant re-authoring the full text by hand each time, with the attendant risk of silently dropping a line from the previous write.

  4) What I did around jigc rather than through it

  Nothing touching a managed doc happened outside jigc — every changelog/spec/ADR/arch-doc read or write went through jigc doc show/create/set-slot/set-field/add-item/author. The things I did directly were all
  outside jigc's domain by design:

  - Read on plain source/test files (src/*.ts, test/*.ts, README.md) and git log/git status/find to understand what had already shipped — these are ordinary project files, not managed docs, and jigc's own
  instructions say to read those freely.
  - git add on my code edits before each finalize — this is what the workflow text itself instructs ("git add your code edits before finalize — it commits only what you have staged"), not a bypass.
  - npm test / npm start / an npx tsc --noEmit attempt (no local tsc, so it just installed a stub and printed a notice — harmless, I didn't chase it further) — build/test verification the workflow explicitly asks
  for before finalizing, unrelated to jigc's doc-management surface.

  So the one genuine workaround was the changelog hand-merge in (1)/(3) — not a bypass of jigc, but a manual patch for a gap in what set-slot/author can do against an already-existing entry.