# Brief — two repairs after the fifth canary root (2026-10-18)

Branch `fix/rc24-tier1`, the main tree at the repository root. Two commits, in this order, each behind the full
`dev/gate` (run in the background, read unpiped, never `| tail`), each by `git commit -F <file>`, with the
trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Then `git push origin fix/rc24-tier1` (by name; never main, never force, never a tag). Never `git stash`,
never `rm -rf` on a variable path. Public repo: no host path, no name, no session id in a file. You do not
edit `.claude/agents/`, `.claude/settings.json` or `CLAUDE.md` — the one edit there is already made, by the
main session (below), and you commit it as it stands. Read the governing docs yourself before you change
anything: `implementation/stabilization-workflow.md`, `implementation/dev-workflow.md` → Gate,
`.claude/workflow-sources/stabilize.js`'s header (the harness's source; regenerate
`.claude/workflows/stabilize.js` with `dev/stabilize-harness` after every edit and commit both), and the
DECISIONS.md entries of 2026-10-16/17 on the canary in parts.

## Commit 1 — a verifier hands on every printed route it did not run (the human's ruling, A)

**What happened.** The fifth canary root, part 3 act 2: the regression plant's verifier
(`setup-hook-link-route-deletes-the-script`) met a second refusal of `jigc setup` on its own drive and
quoted its route and code verbatim, then judged the route "plausibly right … nobody drove it, and I did
not either" and handed nothing on (`hands_on: []`). `.claude/agents/finding-verifier.md:34` said what
evidence an entry needs, but not whether an undriven route that looks right is one. **The human ruled
(2026-10-18): A — every route a verifier saw printed and did not run as printed is handed on, whether or
not it looks right**; the project's rule is that a route is confirmed only by being run as printed. No
further canary is run for it; the real run's first round shows it.

**Already done (uncommitted, in the tree):** the sentence at the end of `.claude/agents/finding-verifier.md`
line 34, naming the DECISIONS entry *2026-10-18 — A verifier hands on every printed route it did not run*.

**To do:**
1. The verifier's prompt in the harness source (`verifyPrompt`, around `stabilize.js` 2836–2846): both
   branches of the hand-on line (`handOn == null` and the number) carry the rule in one sentence — a route
   seen printed and not run as printed is an entry of `hands_on` whatever its look; a route run as printed
   belongs to the verdict. Keep the prompt's register. Regenerate.
2. A fence, in the suite that already fences the harness's prompts and the agent definitions
   (`tooling-tests/stabilize_harness_fence.rs` — read how its arms are written): the verifier's prompt, in
   both branches, carries the rule; `finding-verifier.md` carries it. Red first (before step 1), then green.
3. `DECISIONS.md`: the dated entry the definition links to, at the log's end, in the log's form — the
   ruling, its one line of why, what was observed (the fifth root, part 3 act 2, the verifier's words, no
   host path), and that no canary is re-run for it.
4. `completions/artifacts/M55/stabilization-build/reshape/staged-canary.md` → Part 3: a dated
   `[Noted 2026-10-18 …]` bracket on its last paragraph (the one dated 2026-10-17 on a verifier that hands
   nothing on): it happened on the fifth root, what became of it (the ruling), and that part 4 ran as
   written.

## Commit 2 — a stage's return is read from its notification only

**What happened.** Same root, part 4 act 1: the act's session invoked the Workflow tool, got
"launched in background", and **seven seconds later wrote an invented return** ("refused") to the act's
return file and told the human nothing else ran; the real result came 23 minutes later as a task
notification and the session then corrected the file (byte-equal to the real result, verified). The act
prompts never say that the result arrives later. In the real run the orchestrator reads `next` from that
return.

**To do:**
1. `dev/stabilize-canary`'s act prompts (every act, the guard's part 0 and the rulings' prompts too): one
   sentence — the Workflow tool answers at once that it launched in the background; the stage's return is
   the result of its task notification, which comes when the stage ends; the return file is written from
   that result and nothing else, and nothing is said of the stage before it arrives. Its suite
   (`tooling-tests/dev_stabilize_canary.rs`) holds every prompt to it — red first.
2. `implementation/stabilization-workflow.md`, where the orchestrator invokes the harness and reads `next`
   (→ Invoking the harness, → Reading the next step, or Orchestration — wherever the doc already says
   where `next` comes from; one sentence, pointed at, not restated twice): the same rule for the
   orchestrator.
3. The record: one line where this build records its repairs (the stabilization-build record or
   DECISIONS.md, by the form the 2026-10-16/17 repairs used).

## Return

Compact: the two commit shas, the gate's `passed=` line for each, the pushed remote head, and anything you
left open (with why). Write nothing else into the repository.
