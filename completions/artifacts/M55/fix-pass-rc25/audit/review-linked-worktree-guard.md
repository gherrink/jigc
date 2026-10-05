# Code review — linked-worktree-guard (a88f71cc · e3a6ba58), tree at 5f5b273a

Verdict: RED. The guard holds in the layout it was built for (a `git worktree add` beside a normal
main checkout), but its predicate also fires in every layout where `.git` is a file and the resolved
jigc_home is not a checkout — there it removes doc creation that landed on rc.24 and prints a route
that cannot work. Four further route/class defects are MEDIUM/LOW.

Binaries: NEW = release build of this tree (private target dir); OLD = the published 1.0.0-rc.24.
All drives in `dev/jigc-rig` corpora. `<main>`, `<wt>` stand for the rig's absolute paths.

## F1 — HIGH — the predicate fires where jigc_home is not a checkout: doc creation that landed on rc.24 is refused, and the route dead-ends

Where: crates/cli/src/render.rs:2138 (`CommitSite::differing`, the one comparison) consumed by
`CodeOnlyCheckout::{of_task,of_mint,of_reader}` (render.rs:2221/2233/2238); jigc_home from
crates/cli/src/repo.rs:91 (`.git` is a file -> `dirname(git-common-dir)`).
False record: implementation/decisions-pending.md:91 ("answers *isn't set up* at every door … the guard
never speaks there … Pre-existing … jigc is unusable in that layout") and the pinning test
crates/cli/tests/linked_worktree_doc_home.rs:1596-1646, which never runs the `jigc setup` that
"isn't set up" routes at.

Driven (same script on both binaries; `single-task`, `jigc doc create adr --title "First decision"`,
three slots, commit doc, `jigc task finalize`):

| layout | OLD rc.24 | NEW |
|---|---|---|
| B: `proj/.bare` + `proj/.git` = `gitdir: ./.bare` + `proj/main` worktree | setup 0 · create 0 · finalize 0, HEAD carries `docs/decisions/first-decision.md` | setup 0 · mint 0 with `checkout:` block · create **1** `finalize.linked-worktree-doc` |
| C: sibling `bare.git` + worktree | setup 1 (`setup.install-hook`, project layer left behind) · mint 0 · create 0 · finalize 0, doc in HEAD | same setup · create **1**, same code |
| A: git submodule (`vendor/sub`) | setup 0 · create 0 · finalize 0, doc in HEAD | create **1**; message calls the submodule "the linked worktree" and `<super>/.git/modules/vendor` "the main checkout" |
| D: `git init --separate-git-dir` | setup 1 · create 0 · finalize 0 (2 files committed) | create **1**, same code |

Route as printed, NEW:
- B: `cd <proj>` 0 (`git rev-parse --is-inside-work-tree` -> false) · `jigc start "<intent>"` 0 (router, mints
  nothing) · `jigc start --workflow record-decision "record it"` 0 · `jigc doc create adr …` 0 ·
  `jigc task finalize record-it` **exit 1**: "`git status --porcelain` failed: fatal: this operation must
  be run in a work tree".
- C: `cd <parent of bare.git>` 0 · `jigc start …` **exit 1** "not inside a git repository".
- A: `cd <super>/.git/modules/vendor` 0 · `jigc start …` **exit 1** "this project isn't set up".
- B also: `jigc start --workflow report-inconsistency "a finding"` from the only checkout -> exit 1, same
  code, same dead `cd`.
So in these layouts no door can commit a managed doc any more, and every printed exit fails.
(rc.24 was already partial there: `jigc doc list` answers "no committed docs" and an edit of the
committed doc answers "no staged instance" — but create + finalize landed, which is the whole of the
M55 report workflows.)

Class: one predicate, three constructors; members = every repository whose `.git` is a file and whose
`dirname(git-common-dir)` is not a checkout of it. Four layouts driven (B, C, A, D); not enumerated
beyond those — unbounded.

## F2 — MEDIUM — a88f71cc leaves the second consumer of a staged doc's committed bytes open: the blast-radius walk

Where: crates/engine/src/validate.rs:2126-2135 (dedup by anchor *address*, not by doc);
design/validation.md:322 ("Effective state is the working overlay over the base, never both");
test crates/cli/tests/doc_code_gate.rs (`an_in_task_citation_repair_clears_the_floor_under_a_role_binding_workflow`).

Driven, NEW, main checkout, `fresh` rig + committed `adr:single-node-cache` citing `src/lib.rs#cache_get`:
- `jigc start --workflow single-task "rename the getter"`; rename to `cache_fetch`, `git add`;
  `jigc doc set-field adr:single-node-cache#status/cites-code --unset --task rename-the-getter` -> exit 0
  ("copied in for update"); staged copy has no `cites-code` line; `jigc task finalize` -> **exit 3**
  `doc-code.symbol-exists` on `src/lib.rs#cache_get` at `adr:single-node-cache#status/cites-code` — the
  committed bytes of a doc this task has staged.
- identical under `quick-fix` (no role binding) -> exit 3; identical on OLD.
- delete the function + unset the citation -> exit 3; workaround is two commits (doc-only finalize exit 0, then the code).
The printed route ("update the citation … or restore the cited symbol") names no repair for a removed citation.

Class: consumers of committed doc bytes inside `schedule_doc_code` = 2 (the bound surface in
`enumerate_target_surface`, fixed; the blast walk over `enumerate_committed_surface`, open). Derived
by `grep enumerate_committed_surface(` (3 call sites, one at task scope) plus reading the enumerator.
Test axis: the binary test drives 1 cell (single-task x adr x value update); 9 shipped workflow files
grant a code-anchored doctype with `as:` (grep over `packs/*/workflows`), and the repair-shape axis
(update / unset / remove-item) is not iterated — which is how this member stayed open.

## F3 — MEDIUM — the relocation route's mint collides with the task it relocates

Where: crates/cli/src/render.rs:2284-2311 (`relocate_change_steps`), printed at
crates/cli/src/task.rs:421 (write door, dangling-anchor arm) and task.rs:4915 (changelog gate promoted
to blocking) — 2 call sites by grep. Masked at crates/cli/tests/linked_worktree_doc_home.rs:1141,
which substitutes `<intent>` with "rename the getter from main".

Driven, NEW: task `rename-the-getter` minted in `<wt>`, rename staged, repair refused with the long
route. As printed: `git -C <wt> stash` 0 · `cd <main>` 0 · `git -C <main> merge feature` 0 ·
`jigc start --workflow single-task "rename the getter"` -> **exit 1** `task.serial-collision — task
rename-the-getter is already active` (the route discards the old task only as its last step).
Continuing with the printed pop puts the change in before any new task exists.
Also driven: the same task, typed from `<main>` after stash/merge/pop, takes the repair and
`jigc task finalize rename-the-getter` exits 0 with ONE commit (ADR + src/lib.rs) — an exit the route does not name.

Class: 2 print sites of one producer; the collision needs only that `<intent>` slugs to the old task's id.

## F4 — MEDIUM — the backstop's "finalize it from the main checkout" route commits the main checkout's pre-task staged work at exit 0

Where: crates/cli/src/task.rs:2957-2973 (`lands_from_home` asks `decide_base_repin` only);
route text task.rs:~428-436.

Driven, NEW: `<main>` holds the user's own `wip.txt`, `git add`-ed before any task. Task `sharpen` minted
in `<wt>`; `vision:vision#thesis` written from `<main>` with `--task sharpen`; `jigc task finalize sharpen`
from `<wt>` -> exit 3, guard, route "`cd <main>`, then `jigc task finalize sharpen` commits its docs on
that checkout's branch". Run as printed -> **exit 0**, `promoted VISION.md`, `added wip.txt`, 2 files committed.
Control (same, task minted in `<main>`): exit 3 `finalize.carried-staged — wip.txt was already staged
before this task existed`. The task's pre-task snapshot was taken from the worktree's index, so the
carryover gate is blind in the checkout the route sends it to. No bytes lost; the mechanism is on OLD
too (driven) — what is new is a printed route into it that says only the docs are committed.

Class: instance, unbounded (every route that moves a worktree-minted task's finalize to another
checkout; the relocation route avoids it by minting there first).

## F5 — LOW — route sentences that are false in a reachable state

- crates/cli/src/task.rs:409, :442 and crates/cli/src/render.rs:2321-2333 (3 print sites, grep):
  "`jigc start "<intent>"` starts a task there". Driven: from `<main>` it composes the router, exit 0,
  mints nothing (the CLI's own help says so). The suite runs the span and then mints separately with
  `--workflow single-task` (linked_worktree_doc_home.rs:1026-1032).
- crates/cli/src/task.rs:411: "Task `<id>` stays usable here for code — `jigc task finalize <id>` commits
  what you `git add` on this branch". Driven: task minted and doc staged from `<main>`, second write from
  `<wt>` refused with that sentence; `jigc task finalize mixed-change` from `<wt>` -> exit 3 (the backstop).
  Also printed from a detached linked worktree, where the same command answers `repo.head-detached`.

## Held (tried, could not break)

- a88f71cc does not let a dangling committed anchor land: staged+bound doc with an out-of-band edit to
  the committed bytes (uncommitted -> `reconciliation.conflict-block`; committed -> `finalize.base-mismatch`), exit 3 on both binaries.
- Fan-out untouched: sub-task in `.jigc/worktrees/area-one` — `set-slot`, `create adr`, slot fills,
  `set-field` repair of a renamed symbol all exit 0, no `checkout:` block on `workflow sub-task --task` /
  `start --task`, no served-from note; `milestone finalize` landed 3 promoted docs + the rename in one commit.
- Milestone doors typed in a user-made linked worktree commit in the main checkout; the worktree's index stays untouched.
- Code-only task from the linked worktree: validate / --dry-run / finalize exit 0, commit carries the one code file on `feature`.
- Ten write-leaf argvs from the worktree: exit 1, nothing but the commit doc in the task area.
- Backstop at validate / --dry-run / finalize (exit 3) and bare start (exit 0), one row each; the pin-refused arm names no finalize.
- `--format json`: mint and resume stdout keys exactly {task, text}, no `checkout:`; orientation keys unchanged;
  read notes on stderr only; mint/migrate refusal is the findings envelope on stderr, stdout empty.
- `jigc migrate` from the worktree: exit 1, nothing minted.
- The code cannot be demoted (`config.undeclared-key`).
- Hostile path (space and apostrophe) in the detached-worktree route: `-C` operand correctly quoted.
- Doc-only mint by a sub-agent inside a fan-out worktree: refused at the mint, route works from main (rc.24 dead-ended at `repo.head-detached`).
- Suites at HEAD: `linked_worktree_doc_home::` 14/14, engine arm 1/1.
- Diff stat of both commits: no pack schema, golden, snapshot or AGENT.md file.

## Not examined

- The rest of `dev/gate`; only the area's two suites were run.
- Older git than this machine's; Linux.
- `jigc ingest` / `unmanage` / `relocate` / `migrate-corpus` / `config set docs-root` typed in a linked worktree (out of this area's diff; `jigc rename` driven once: commits in the main checkout).
- Stash-route conflict cases (dirty main checkout, diverged branches) and the changelog-blocking arm end to end.
- The join when two sub-tasks edit one committed doc (read: `join.same-doc-clash` blocks).
- An unparseable staged copy of a bound doc (read only: enumerates nothing; conformance is another gate's).
