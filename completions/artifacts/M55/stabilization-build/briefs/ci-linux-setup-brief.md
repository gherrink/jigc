# Brief — CI's red Linux test, fixed before the opening (2026-10-27)

The human ruled on 2026-10-27 (A): CI's one red test is fixed now, before the remaining pre-opening build tasks, so that
CI checks every commit on Linux again. It was recorded as owed to the stabilization run (`DECISIONS.md`, ~line 4963,
*`ci-ok` stays red*); this fix discharges that — say so in your `DECISIONS.md` entry, and the run's opening ledger then
carries it as fixed, not open.

## The failure

Every CI run on `fix/rc24-tier1` since 2026-10-05 fails `test (g_migrate)` on one test only:
`setup_install_pathspec_guard::where_git_cannot_answer_setup_refuses_before_its_first_write`
(`crates/cli/tests/setup_install_pathspec_guard.rs` ~3249), the test of round 4's fix `f0b1120a`. On Linux `setup`
refuses with `setup.dirty-install-path` where the test requires `setup.unverified-install-path`; macOS passes. Latest
red run: `gh run view 40544364786 --log-failed` (read it, never print an environment value).

## Where to work

Another builder is working in the main tree (`~/projects/gherrink-jigc`). **Do not touch it.** Work in a
worktree of your own: `W=$(mktemp -d "<the session scratchpad>/ci-linux.XXXXXX")` (use `$TMPDIR` if no scratchpad is
named), `git -C ~/projects/gherrink-jigc worktree add -b fix/rc24-linux-setup-code "$W/wt" f99658b8`. Remove
it at the end only with `git worktree remove`. `CARGO_TARGET_DIR` inside your worktree.

## What to do

1. **`implementation/working-principles.md` first: neither the doc nor the code is the oracle.** Establish why Linux
   differs — what `setup` asks git where it "cannot answer" (`InstallSubject::probe`, `ask_dirty_against_head`,
   `unseen_by_status`, `task::git_index_flags` are the sites the route-catalog census names), and what state the test
   builds. Reproduce on Linux with `dev/runner-faithful --commit <rev> cargo nextest run -p jigc …` (it runs a
   **committed** revision — commit locally on your branch first; `CLAUDE.md` → Build / lint / test).
2. Decide which is wrong — the product's code on Linux, or the test's construction of "git cannot answer" — from what
   the docs promise (`design/project-setup.md`, the round-4 fix's `DECISIONS.md` entry and the finding code registry)
   and from what makes sense for an adopter. Show it with evidence. If both readings are sound and they lead to
   different behaviour for an adopter, **stop and return both sides** — that is the human's.
3. Fix it, red first (on Linux, in the container), the smallest correct change that closes the class — check whether
   any other `setup`/`uninstall` test or route-catalog/byte-table cell takes the same path on Linux, and say how you
   counted. Docs in the same commit if they change.
4. **Stop before the full gate** and return: the cause, the fix, the Linux run green in the container, the class count.
   Another gate may be running on this machine; the main session tells you when to run yours. Then: the full `dev/gate`
   in your worktree in the background, read unpiped; `git commit -F <file>` with the trailers below; push your branch by
   name; the main session merges it into the loop branch.

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Message ends "(before the stabilization run's opening: CI's Linux red)". No stash, no `rm -rf` on a variable path,
never print an environment variable's value, no host path / name / id in a file, the human is "they", no edits to
`.claude/` or `CLAUDE.md`.
