Going through this specifically:

  1. What confused me

  The docs-review gate failure was ambiguous at first glance. jigc task finalize failed with output prefixed docs-gate:, sitting right below jigc's own finalize — output, so it initially read as if jigc itself were
  the thing refusing the commit. It isn't — I had to cat .githooks/pre-commit to see that jigc's own managed block ends at # jigc-managed pre-commit hook — end, and the docs-gate is a separate section appended after
  it. git log -1 def6192 confirmed it was a distinct commit from a different author, unrelated to jigc setup. jigc's finalize just relays whatever the hook prints to stderr — it doesn't distinguish "a check I own"
  from "a check bolted onto the same hook file."

  Separately, jigc rename vs jigc doc rename cost me a wrong guess (below).

  2. What jigc told me that turned out wrong or misleading

  Nothing I found was actually incorrect — every error I hit was accurate once I understood it. The closest thing to misleading: nothing in jigc start's workflow text or the record-decision --preview output
  mentioned that a second, non-jigc gate (docs-gate) could also block finalize. That's not jigc lying, just an unadvertised interaction with a hook it doesn't control.

  3. What I looked for and didn't find

  - A rename command that works uniformly regardless of commit state. I first ran jigc rename adr:drop-the-oldest-sample --to "..." --task "$TASK", assuming task-scoped rename took --task like every other doc write
  verb. It doesn't — jigc rename is explicitly task-less and only operates on committed identities; it errored on both the unexpected flag and then (after I dropped --task) on the doc not existing in the committed
  store. I had to discover jigc doc rename (a different command, same word, different subsystem) for the staged case. The distinction is documented in --help, but nothing surfaced it to me proactively before I
  picked the wrong one.
  - I didn't look for a way to preview the rendered Markdown before staging (e.g. a dry-run diff) — I just wrote, then doc show'd to check, which was fine here but would be a nuisance for a doc where I only wanted
  to check formatting.

  4. What I did around jigc rather than through it

  - Read src/ingest.ts directly with cat/grep — appropriate, since jigc owns docs, not source, and I needed to ground the ADR's claims in the real overflow logic (shift(), the dropped counter) rather than trust the
  task description alone.
  - Read .githooks/pre-commit directly and ran git log -1 def6192 -p — to diagnose the docs-gate failure, since jigc's finalize output only relayed the hook's stderr, not its provenance.
  - Ran raw git status / git restore --staged on src/router.ts and scripts/retention-sweep.sh — but those are non-doc files unrelated to this task, not something jigc surfaces or manages; I did that purely to keep
  the pre-existing unrelated staged changes out of this commit.
  - Checked ls -la .git/docs-approved directly rather than through any jigc verb — that marker is explicitly outside jigc's domain (the hook comment says it's kept out of git status on purpose), so there's no jigc
  surface for it at all.

  I never hand-edited docs/decisions/*.md or committed it myself — every write to the ADR and the commit doc went through jigc doc create/author/set-slot/set-field/rename, and the final commit landed via jigc task
  finalize.