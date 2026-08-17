Going back through the transcript honestly:

  1. What confused me

  The biggest one: the milestone sub-task resume text (jigc start --task add-a-configurable-rollup-max) printed explicit instructions to author commit prose — jigc doc set-field
  commit:add-a-configurable-rollup-max#type --value <COMMIT_TYPE> ... — as if a per-sub-task commit doc existed and I owned it. It doesn't apply cleanly in that context (see #2). I also got confused by jigc start
  --task <sub-task-id> refusing outright right after minting: task is pinned to base a99b6d9 but you're on 1d46b86 — the milestone's own bookkeeping commits (chore(milestone): record task:...) had already moved HEAD
  past the sub-task's pin, so the very act of seeding two sub-tasks made both un-resumable from the main tree. Nothing up to that point told me sub-tasks need jigc milestone provision + working from
  .jigc/worktrees/<id> — I only learned it from the error text. And I couldn't build a confident mental model of how jigc milestone finalize combines code (not just docs) across independent sub-task worktrees when
  two increments are code-dependent rather than parallel/disjoint — more on that in #3 and #4.

  2. What jigc told me that turned out wrong or misleading

  The sub-task instructions telling me to jigc doc set-field commit:<task-id>#type ... and "Author this sub-task's own commit prose" were simply inapplicable: trying it gave create.gate-blocked — the workflow does
  not allow jigc doc create commit in-task; allowed doctypes: [adr]. So the printed steps described an action the tool then refused. I never found this reconciled anywhere; I inferred (from the eventual milestone
  finalize output showing a CLI-synthesized subject: "Finalize milestone ... (2 sub-tasks)") that commit-message authorship is a milestone-level concern, not a sub-task-level one, but the composed task text never
  said that — it just handed me commands that didn't work.

  Also mildly misleading: jigc milestone join --help describes the operation purely as a "disjoint-union" of "staged docs." That description says nothing about code. It was only from the sub-task instructions ("git
  add your code edits so the milestone can fold this worktree's staged index") and later the actual finalize output ("code_files": 7) that I learned code folding is real and separate from the doc-join. The --help
  text for the verb that actually does the work undersold what it does.

  3. What I looked for and didn't find

  - A dry-run for jigc milestone finalize. jigc task finalize has --dry-run; the milestone equivalent doesn't, per its --help. Given finalize is the one-shot commit boundary and I was uncertain how it would treat
  two worktrees with overlapping code changes, I wanted to preview it and couldn't.
  - Any explanation of how sub-task worktrees' code diffs combine at finalize when they're not independent — nothing in --help, jigc describe, or (per AGENT.md's own instruction to ask the binary, not a source tree)
  anywhere I could reach explained the merge semantics for dependent/overlapping diffs across sub-task worktrees.
  - A focused jigc describe <item> — I tried jigc describe milestone hoping for a targeted explanation; it errored (unexpected argument 'milestone' found). describe only does the full tour or category flags, not a
  single-item lookup.

  4. What I did around jigc instead of through it

  - git reset --hard a99b6d9 directly on the add-a-configurable-rollup-max worktree, to discard its staged code. I made this call because increment 2 genuinely depends on increment 1's code (same functions in
  store.ts/router.ts), the two sub-task worktrees start as independent detached checkouts of the same base with no jigc-sanctioned way to layer one sub-task's in-progress code onto another's, and there was no
  dry-run to test the dependent case safely first. Rather than risk jigc milestone finalize on an untested overlapping-diff scenario, I used plain git to consolidate all code into one worktree and reset the other to
  clean, then let join/finalize run over a case I actually understood.
  - To confirm that was safe, I inspected jigc's internal storage directly — cat .gitignore, git check-ignore -v .jigc/tasks .jigc/worktrees .jigc/milestones, and checked for a per-worktree .jigc/tasks directory —
  to verify a sub-task's in-flight doc content (the ADR) lives centrally under the gitignored .jigc/tasks/, not in the worktree's git-tracked files, so resetting the worktree's code wouldn't blow away the ADR I'd
  already authored through jigc. That's investigation of jigc's own plumbing via the filesystem/git rather than asking the tool, done only because no jigc verb surfaced that guarantee.

  I didn't read or hand-edit any managed doc content directly — all doc writes went through jigc doc verbs — but the worktree reset and the storage-layout probing are real workarounds, not sanctioned jigc paths.