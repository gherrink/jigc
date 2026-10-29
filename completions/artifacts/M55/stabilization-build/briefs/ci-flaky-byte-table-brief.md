# Brief — two byte-table cells flip on Linux (2026-10-27)

The human ruled on 2026-10-27 that CI checks every commit on Linux again. CI run `40591137834` on `0bd8a5b5` (a commit
touching only agent definitions, docs and `tooling-tests/tooling_index.rs`) fails `test (g_migration)` on
`byte_table::the_cells_are_run_and_the_expected_red_list_is_exact::fresh`, two cells:

- `relocate-a-link-to-a-directory-outside-at-the-notes-new-directory` — *"not the listed outcome"*;
- `config-set-a-link-to-a-directory-outside-at-the-decisions-new-directory` — *"refuses, nothing changed"*.

Both were added by `09270673` (the byte table's fix 5, `Kind::LinkOnTheWay`), an ancestor of `c9a22385`, whose CI run
`40585095772` was green. Identical cells, green then red: nondeterministic on Linux. The byte table is the instrument for
the exit rule's first clause, so an answer that changes from run to run *records falsely* — repaired before the opening.

## Where to work

The main tree is shared; do not touch it. `W=$(mktemp -d "<scratch>/flaky.XXXXXX")`,
`git -C ~/projects/gherrink-jigc worktree add -b fix/rc24-byte-table-flake "$W/wt" 0bd8a5b5`; your own
`CARGO_TARGET_DIR`; remove the worktree at the end only with `git worktree remove`, after the push.

## What to do

1. Read the failure: `gh run view 40591137834 --log-failed` (never print an environment value). `implementation/tooling.md`
   first — `dev/runner-faithful` is the tool for the runner's view, **but it is known not to be faithful**: the runner has
   `[safe] directory = *` in `/etc/gitconfig`; a variant that copies it in is at
   `<scratch>/runner-faithful-etc`
   (its `repo_root` is hard-coded — adapt a copy in your scratch, never commit it).
2. Reproduce: the two cells, repeated (dozens of runs if cheap), in the runner-config container, and on macOS. Find the
   source of the nondeterminism — order of a directory read, a timestamp, a race in the plant, a link resolved
   differently, the cell's classification itself. Evidence, not a guess. `implementation/working-principles.md`: if the
   product's behaviour is what varies, that is a product finding (a cell may not paper over it) — stop and return it.
3. Fix red first (a test that shows the variance, or the deterministic construction), the smallest correct change.
   Check every other cell that uses the same plant kind or the same construction for the same flaw, and say how you
   counted. If the fix is in the frame (`crates/cli/tests/support/byte_cell.rs` and the family files), re-run the
   affected cells on both binaries and say whether any classification moved.
4. **Stop before the full gate** and return: cause, fix, the repeated runs before and after (counts), the class count.
   The main session tells you when to run the gate (one at a time on the machine). Then: the full `dev/gate` in your
   worktree in the background, unpiped; `git commit -F <file>`; push your branch by name; the main session merges.

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Message ends "(before the stabilization run's opening: CI's Linux flake)". `DECISIONS.md` newest first, one entry. Your
report ends with a **Tools** section (`implementation/tooling.md` → the harvest rule). No stash, no `rm -rf` on a
variable path, never print an environment variable's value, no host path / name / id in a file, the human is "they",
no edits to `.claude/` or `CLAUDE.md`.
