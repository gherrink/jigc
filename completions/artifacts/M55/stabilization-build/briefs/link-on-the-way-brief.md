# Brief — no door writes through a link on the way (2026-10-28)

## The human's ruling, 2026-10-28 → B

CI run `40591137834` on `0bd8a5b5` is red in `g_migration` on two byte-table cells. It is **not a flake**: the hosted
runner's image `20261025` carries git 2.56 (from the git-core PPA), which refuses `git mv` into a destination beyond a
symbolic link; git ≤ 2.55 follows the link. jigc's one `git mv` is `relocate::move_doc` (`crates/cli/src/relocate.rs`),
reached from `config.rs` (~1419, ~1543 via `relocate_stranded`), `relocate.rs` (~509, `relocate_freeze_exempt`) and
`rename.rs` (~1080). Measured by the previous fixer, 30 runs per environment: macOS git 2.54 and Ubuntu's stock 2.43
write outside every time, the PPA 2.56 refuses every time; of the 602 gate cells exactly these two differ.

The two cells are under the byte table's key `r1-a-write-follows-a-link-at-a-directory-on-the-way-out-of-the-repository`
— `completions/artifacts/M55/stabilization-build/reshape/byte-table-findings.md` §28 (read it whole: the nine cells, the
five doors, *which is wrong and why*, *the reach, counted*, the repro block, and the two §17 cells and the six
"refuse at `git add` having changed nothing" cells after it). The rows' record's §6 entry 25, ruled 2026-10-26: **the
check follows no link**.

**Ruled (B):** one shared guard, the robust end state §28 names — *every door asks the directories on the way to each
path it writes, moves, or reads in order to write whether one is a link, as `setup` does on this tree, and refuses before
its first write, naming the link and routing at replacing it with a real directory.* Applied to every door of the
class, not only `move_doc`'s callers: `workflow` (sub-task re-entry), `relocate`, `config set` (`docs-root` **and**
`placement-root`, which has no cell), `task finalize`, `migrate-corpus`, `rename`, the §17 milestone doors, and the six
doors that today write through the link and roll back. CI must go green because jigc refuses on every git version before
git is asked — never because a cell's expectation depends on git's version.

**Why B over the move-only fix:** the human wants robust, future-proofed fixes, nothing we have to come back to; the
move-only fix leaves three doors writing outside the repository on every git and owes round 1 the same mechanism again.

## Where to work

The main tree is not yours. `W=$(mktemp -d "<scratch>/link.XXXXXX")`,
`git -C ~/projects/gherrink-jigc worktree add -b fix/rc24-link-on-the-way "$W/wt" 0bd8a5b5`; your own
`CARGO_TARGET_DIR` inside `$W`. Remove the worktree at the end only with `git worktree remove`, after the push.

The previous fixer's tools, in the session scratchpad (`<scratch>/`):
`link-cells-repro.sh` (the two cells, repeated), `runner-ppa/` and `runner-stock/` (runner-shaped images with the PPA's
git and with stock git). Read them before writing your own.

## What to do — two commits, in this order

**`implementation/working-principles.md` and `implementation/tooling.md` first.**

### Commit 1 — `dev/runner-faithful` made faithful (tooling)

It is unfaithful on two counts, both found this week: it installs Ubuntu's stock git (2.43) where the runner installs
the newest from the git-core PPA, and it asserts "no git config at any level" where the runner's `/etc/gitconfig` holds
`[safe] directory = *`. Establish from the runner image's own published sources and from a CI job's log (`gh run view
… --log`, never print an environment value) exactly how the runner gets git and what its system config holds; make the
image do the same, print `git --version` on every run, and keep the build-time assertion true to the new shape. Its own
test (it has one — find it via `tooling.md`), its `tooling.md` row, `implementation/dev-workflow.md` → Gate where it
describes the image. **`CLAUDE.md` names it as "stock git … no git config at any level"** — you do not edit `CLAUDE.md`;
return the corrected sentence byte for byte and the main session applies it. Then show the two cells red at `0bd8a5b5`
in the faithful image (that is the red of commit 2's CI half).

### Commit 2 — the guard

1. **Find the chokepoints before the doors.** Read how `setup` refuses (`regular_file::blocker` and its callers, the
   `setup.write-bootstrap` / `setup.write-guide` codes, the route it prints). Establish where the class's writes,
   moves and rebuilds actually go through — shared write/move primitives, the edge-index rebuild, the re-entry
   provisioning — and put the guard where **every present door and the next one** inherit it, rather than at each door
   by hand. Where one chokepoint cannot serve, say why. Count the class yourself (the 26 needs `aimed` derives is the
   record's count — re-derive it) and say how you counted.
2. **Red first**: the nine §28 cells, the two §17 cells, a cell for `config set placement-root` (and for any door of
   the class you find with none), each listed as *refuses, nothing changed*, naming the link — red on this tree on
   macOS before the fix. A cell's expectation never depends on git's version.
3. **Codes and routes.** A refusal names the link's path and routes at replacing it with a real directory, the route
   run as printed. Prefer the codes the doors already own; a **new finding code is a halt** under the fix lane's rules
   — so before the gate, **return** every code you would add or change, with its message and route, and the main
   session clears them. Every new refusal site gets its route-catalog case (`sites.tsv` and its fence).
4. **Docs in the same commit.** One home for the rule *jigc writes through no link on the way* (find where the write
   path's guarantees live in `design/` — `write-commands.md`, `storage.md`; one home, pointers elsewhere), the finding
   code registry, §28 and §17 (the key fixed, the cells' new classification), the byte-table record's counts and the
   expected-red list, both binaries re-run where the frame asks it. `DECISIONS.md` newest first, one entry: the ruling,
   its why in a line, and that CI's red was git 2.56 exposing a code finding.
5. Run the affected cells on both binaries; run the two CI cells in the faithful image on your committed revision
   (commit locally first; it runs **committed** code — nothing is pushed until the gate is green).

**Stop before the full gate** and return: the chokepoint design and the class count with its derivation, the codes and
routes (for clearing), the `CLAUDE.md` sentence, the cells before → after, the container run. The main session tells
you when to run the gate (one at a time on this machine). Then: the full `dev/gate` in your worktree in the
background, read unpiped; `git commit -F <file>` per commit; push your branch by name; the main session merges.

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Messages end "(before the stabilization run's opening: no write through a link on the way)". Your report ends with a
**Tools** section. No stash, no `rm -rf` on a variable path, never print an environment variable's value, no host
path / name / id in a committed file, the human is "they", no edits to `.claude/` or `CLAUDE.md`.
