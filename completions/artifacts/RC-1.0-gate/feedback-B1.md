Going through the transcript honestly:

  1) What confused me

  - Two different senses of "staged" collide in jigc's vocabulary. jigc talks about a task's docs being "staged" (living in .jigc/tasks/<id>/docs) using the same word git uses for the index. When the docs-gate hook
  rejected my first finalize, jigc's output was your staged changes are still staged — I read that as "check git status," and had to go verify with git diff --cached and find .jigc/tasks to convince myself nothing
  was actually in git's index. The two staging areas are unrelated, but the message doesn't say so.
  - I wasn't sure whether status was a slot (author with <<…>>) or a plain field inside the adr doctype. The record-decision --preview text lists it separately from context/decision/consequences and shows set-field
  for it, but I still drafted a jigc doc author YAML payload that nested status under its own fake "section" before checking. I caught this myself by running jigc doc create + jigc doc show --format json and reading
  the real shape back, and threw the bad payload away without ever submitting it — but the confusion was real.

  2) What jigc told me that was wrong or misleading

  - The claim above — task ... is intact — nothing was committed and your staged changes are still staged — is misleading taken at face value. From git's perspective nothing was staged; the ADR only existed in
  jigc's own task directory. It's not factually wrong (jigc's internal staging genuinely was intact), but it's phrased in a way that borrows git's vocabulary for a different mechanism, and cost me a verification
  detour.

  3) What I looked for and didn't find

  - A jigc-native way to satisfy or even inspect the docs-gate. The finalize error just said ask the docs reviewer to sign off — no command, no pointer to the mechanism. I had to git show the .githooks/pre-commit
  source directly to discover the actual contract (a file at $(git rev-parse --git-dir)/docs-approved). There's no jigc doc approve / jigc docs-gate status surface at all — it's a bare git hook jigc doesn't know
  about or expose.
  - A schema/introspection command for a doctype's exact address tree (which leaves are fields vs. slots, required vs. optional) before writing to it. describe and the workflow preview give prose, not a structured
  schema. I ended up reverse-engineering it empirically (create then show --format json) rather than finding a documented way to ask "what does an adr look like."
  - A way to redo or narrow a finalize after the fact — see below.

  4) What I did around jigc instead of through it, and why

  - git restore --staged on your two pre-existing files before minting the task — there's no jigc verb for "exclude these unrelated paths from what finalize will see," so I unstaged them myself. Reasonable, since
  they're not jigc-managed docs.
  - git show f97cb5f to read the pre-commit hook's source — necessary because jigc's own error text didn't explain the gate's mechanism.
  - The significant one: after finalize succeeded but bundled your unrelated staged files into the ADR commit (my mistake — I git added them back right before calling finalize, not realizing finalize commits the
  literal git index), I fixed it with raw git reset --soft HEAD~1 followed by a hand-written git commit -m ... reproducing the same message, entirely bypassing jigc task finalize. jigc has no "redo finalize with a
  narrower path set" or "split this commit" verb, and by that point the task's finalize had already run once, so I judged a manual git fix was the only path — then confirmed jigc reconciled correctly afterward (jigc
  task validate reported no task, jigc start showed no dangling task). It worked, but it's a direct hand-edit of a jigc-managed doc's commit history that happened outside any jigc command.