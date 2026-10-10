# proposal-driver — `r1-setup-installs-outside-work-tree` (run `canary-one`, round 1, stage `test`, attempt 1)

I drove a proposal I did not write: the advocate's *robust-now* case for the fork on
`r1-setup-installs-outside-work-tree`. My brief was to refute it. Nothing in the repository was
edited, committed, staged or built; the one file I wrote for it is this report.

## Verdict in one paragraph

**`holds: false` — on one step, and the rest reproduces.** The proposal's diff applies to a
fresh clone of eeffe347 byte for byte (the same sha256), builds, and passes the whole gate
(`GATE: PASS`, `passed=4968 failed=0`). Its own walk (Repro AD-3) exits 0 at every step in all
four layouts, the install lands in the checkout, nothing is written beside it, and every
*before* the advocate reports is what the candidate and the previous release do. **What does
not hold is git's own next step.** The proposal puts `pre-commit` *in the directory git names
from the checkout* and reports git's hook run as passing (*exit 0, nothing on stderr*). Driven
with a commit that hook exists to refuse — a managed doc moved with a bare `git mv` — the
commit lands at exit 0 with nothing on stderr in three of the four layouts (a worktree beside a
bare repository, behind a bare `.git`, a `--separate-git-dir` checkout), where the same commit
in a plain checkout and in the submodule is blocked at exit 1. The hook runs and finds no
project: git exports `GIT_DIR` to its hooks in a checkout whose `.git` is a file, the
proposal's new probe (`git -C <dirname(git-common-dir)> rev-parse --show-toplevel
--git-common-dir`) inherits it, and under it git answers that the folder holding the bare
repository **is** the top of a work tree of this repository — so inside a hook rule 2 fires and
the home is the old one. The advocate's row 14 used a commit a dead hook and a live hook answer
alike. The proposal's tests assert the hook is *a file* there, so the gate is green over it.

It is a narrow defect with a short repair (finding PD-1, below, carries a variant of mine in
which the hook blocks again — driven on that one step, not gated). It is still a refutation of
the claim as written: *all four layouts are the ordinary single-checkout case from then on* is
false for the one party in the walk that is not jigc.

## The binaries

| binary | where | how its identity was established |
|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` under the scratch root | asked of the tool, one call: `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the prompt names |
| previous release | `bin/previous-91834b5e011d/jigc` under the scratch root | not asked of the tool by me; used as it lies there, and `jigc --version` prints `jigc 1.0.0-rc.24` |
| **the proposal's build** | `<W>/bin/jigc` | my own: `cargo build --release --locked` in my clone with the diff applied, a target directory of its own, exit 0; sha256 of the file `0d80e22f2f8727131325d8b90bfdd45867334124700e9c5578ceb1fadd3bf84a`. It is nobody's candidate, and it is not the advocate's spike (a debug build, `2ff0072d…`) |
| the proposal less rule 3 | `<W>/bin-norule3/jigc` | the product half with the last line of `jigc_home` answering the standing checkout; sha256 `e0ff5185…`; used for one step |
| a variant of mine | `<W>/bin-variant/jigc` | the proposal plus five `env_remove` lines on the new probe; sha256 `122cae14…`; used for one step, and not the proposal |

Every *before* below is the candidate (`dded1fac…`), its directory first on `PATH`; the walker
prints `command -v jigc` when it sets it.

## How it was driven

- `<W>` is my directory, minted with `mktemp -d` under the scratch root:
  `<scratch>/proposal-setup-outside.aXHOpU`. Every layout root is a fresh `mktemp -d` under it;
  nothing was removed.
- **The clone.** `git clone` of the repository into `<W>/full`, `git checkout --detach
  eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, then the advocate's file as it stands:
  `git apply --check` exit 0, `git apply` exit 0, `git diff --stat` *3 files changed, 389
  insertions(+), 62 deletions(-)*, and `git diff` hashes to
  `6d536acd6efbe63e78cde5f00bc3d1b9f2a1015d547ee213d9054b79a84ea74d` — the sha256 of the file
  itself and the one the proposal names.
- **Environment of every root:** `HOME` a fresh directory inside the root,
  `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null`, a synthetic author and committer.
  git 2.54.0, macOS.
- **Every exit was read from the command itself** (`cmd > out 2> err; rc=$?`), never through a
  pipe.
- **The walking is mine, not the advocate's.** I read `tools/walk.sh` and `tools/loss.sh` in
  the advocate's directory and ran neither; `<W>/tools/drive.sh` builds the same layouts from
  the report's table and walks the same commands, and adds the checks I wanted (a path-and-hash
  snapshot around `setup`, the names beside the checkout, the hooks directory asked of git).
  `<W>/tools/extra.sh` holds the probes that are not one straight walk, and `<W>/tools/cmp.py`
  compares two walks with the root, the checkout path and hashes normalized.
- Layout names as in the advocate's report: `bare`, `behind`, `sep`, `sub`, `plain`, `linked`.
  I added one to the walk, **`dotbare`**: `git init --bare .bare`, a `.git` *file* reading
  `gitdir: ./.bare` beside it, `git worktree add wt -b main` — the widespread arrangement of a
  bare clone with its worktrees as siblings. It is no new layout to the tree: the standing
  suite `linked_worktree_doc_home` builds it (*a bare repository behind a `gitdir:` pointer*),
  so the proposal's extended test walks it. The advocate's walk did not.

## Driven — one row per next step

*agrees* is whether what I observed is what the advocate reported for that step.

| # | whose next step | command | what it printed and exited | agrees |
|---|---|---|---|---|
| 1 | the candidate's identity | `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | one line of JSON, `content_sha256` `dded1fac…beadfc`; exit 0 | yes |
| 2 | git, on a fresh clone | `git clone`; `git checkout --detach eeffe347…`; `git apply --check <W>/proposal.diff`; `git apply <W>/proposal.diff`; `git diff \| shasum -a 256` (the hash read from a file written by `git diff > file`, its exit 0) | 0, 0, 0, 0; 3 files, +389 −62; `6d536acd…` | yes |
| 3 | the build | `cargo build --release --locked`, `CARGO_TARGET_DIR=<W>/target-release` | `Finished release profile`; exit 0 | — (the advocate built debug) |
| 4 | the gate, on the whole diff | `dev/gate` in `<W>/full`; the sha256 of `git diff` printed before and after, `6d536acd…` both | `fmt ok · clippy ok · build ok · tier1 ok (1475s) · tier2 ok · doctest ok`; `tests passed=4968 failed=0 (over 18 test binaries)`; `GATE: PASS`; exit 0. (No timing record in a fresh clone, so the first tier was the whole suite. My walks ran beside it.) | yes |
| 5 | the standing suites, the product half alone | `dev/gate` in `<W>/half`: eeffe347 plus the first five hunks of the diff (`crates/cli/src/repo.rs` less its `mod tests` hunk; `1 file changed, 84 insertions(+), 34 deletions(-)`) | `fmt ok · clippy ok · build ok · tier1 FAILED 100 (1399s)`; `tests passed=4965 failed=1 (over 16 test binaries)`; `GATE: FAIL (step: tier1)`; exit 1. The one failing test: `jigc::g_migrate setup_install_pathspec_guard::git_that_answers_is_never_refused_as_git_that_does_not`, at `setup_install_pathspec_guard.rs:3508`, its parity assertion for `--separate-git-dir, the first run`. The first tier was the whole suite and was red, so the doctests were not run — as in the advocate's run | yes — on the exact form the proposal ships (the advocate's was an earlier form with two git calls) |
| 6 | the tests, red first | in `<W>/testsonly` (eeffe347 plus the two suite files' hunks, no product change): `cargo nextest run -p jigc --no-fail-fast -E 'test(git_that_answers_is_never_refused_as_git_that_does_not) \| test(a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard)'` | `2 tests run: 0 passed, 2 failed`; exit 100. The first panics at `linked_worktree_doc_home.rs:2181` (*a bare repository behind a `gitdir:` pointer (None): the install is in the checkout the command was typed in*), the second at `setup_install_pathspec_guard.rs:3458` (*inside the submodule: the install is in the submodule's own checkout*). The two unit tests name functions the candidate does not have, so alone they do not compile | yes |
| 7 | the before: the whole walk, candidate and previous release | `drive.sh <bare\|behind\|sep\|sub\|plain> <root> bin=<candidate or previous> setup setup doc:Cache reads doc:Queue reads milestone uninstall uninstall-force` | identical on both binaries. `setup` 1, 1 (`bare`, `sep`) or 0, 0 (`behind`, `sub`); first finalize 0; `jigc doc list — no committed docs` in all four; `validate` 1 with six `blocking` lines, five of them `home-vacated` (`bare`, `sep`, `behind`), 0 in `sub`; second finalize 3 in all four (`reconciliation.rename … adr:cache … is missing`); `milestone create` 1 in all four; `uninstall` 1 in all four; `uninstall --force` 1 in `bare` and `sep`, 0 in `behind` and `sub`. `plain`, candidate: every step 0. (With no task open, `uninstall` says `uninstall.untracked-workbench-file` in all four — driven on the candidate.) | yes |
| 8 | the printed route of the blocked finalize | `… setup doc:Cache doc:Queue unmanage:docs/decisions/cache.md finalize:record-the-queue reads` (candidate: `bare`, `sep`, `sub`, `behind`; previous: `bare`, `sub`, `behind`) | `bare`, `sep`: `unmanage` 1 with the bare `git rev-parse --verify -q HEAD` line, the finalize 3 again. `sub`, `behind` (both binaries): `unmanage` 0 saying *there was no file at docs/decisions/cache.md to leave on disk*, the finalize then 0, `doc list` still *no committed docs* | yes (and `behind` on the candidate, which the advocate did not type: as on the previous release) |
| 9 | the loss cell, before | `… setup plant:docs/decisions/cache.md doc:Cache lossafter:…` — an untracked file with a marker at the doc's home; candidate and previous release in `bare`, `behind`, `sep`, `sub`; candidate in `plain` | all four layouts, both binaries: finalize 0; files holding the marker 1 before, 0 after (`command grep -rl`, the same command, with the before-control finding the plant); git objects holding it 0. `plain`: finalize 3 (`conformance.section-missing`), the file byte for byte | yes |
| 10 | the installer and its route, after | `drive.sh <layout> <root> bin=<proposal> snap:0 setup setup snap:1 …`, four layouts | 0, 0 in all four. Ten paths created: nine inside the checkout and `pre-commit` (5153 bytes) in the directory git names from it (`proj/store.git/hooks`, `proj/.git/hooks`, `proj/store/sep.git/hooks`, `super/.git/modules/vendor/lib/hooks`); the one path whose content changed is the branch ref. `git status --porcelain` empty after each run; `.jigc/config` a directory in the checkout; the folder the checkout stands in lists the same names; `proj/CLAUDE.md` 80 bytes; the superproject's `pre-commit` its own 41 bytes; `super/.git/modules/vendor` holds `lib` only; `chore(jigc): install jigc workspace config` in the checkout's log | yes |
| 11 | the writer twice, and the reader | `… doc:Cache reads doc:Queue reads` in the four layouts; and `jigc doc show adr:cache`, `jigc task list`, `jigc upgrade`, `jigc describe` in `bare` | both finalizes 0; `doc list` names `adr:cache` after the first and both after the second; `validate` 0, *no findings*; every read 0 | yes |
| 12 | the join | `… milestone`: `create "Ship it"`, `add-task` ×2, `provision`, a file written and `git add`ed in each `.jigc/worktrees/<id>`, `jigc start --task <id>` and `jigc doc list` typed there, `milestone finalize ship-it` | all 0 in all four layouts; `HEAD` is `Finalize milestone ship-it (2 sub-tasks)` and names `alpha-part.txt`, `beta-part.txt`, `docs/milestone-records/ship-it.md` | yes |
| 13 | the join without the fan-out rule | the same in `bare` on `<W>/bin-norule3/jigc` | `jigc start --task alpha-part` in the worktree: exit 1, `finalize.no-task`; `milestone finalize`: exit 3, `repo.head-detached` for both worktrees | yes |
| 14 | the teardown | `… uninstall` | 0 in all four; `pre-commit` gone from git's hooks directory, `.jigc` gone from the checkout | yes |
| 15 | equivalence with the ordinary case | my 36-step walk's 72 output files, the proposal's build in each of the four layouts against the candidate in `plain`, normalized | 70 identical, 2 differ, in each layout: the two `setup` acks, on the line naming where the hook went. (Control: candidate `plain` against the proposal's build in `plain`, 72 of 72 identical; candidate `bare` against the proposal's build in `bare`, 36 differ.) The advocate's walk is 39 steps and 78 files; mine is shorter by three reads and the proportion is the same | yes |
| 16 | every other repository: a linked worktree beside a real main checkout | the same walk in `linked`, candidate against the proposal's build | 76 output files, 0 differ (the doc create is refused there by `finalize.linked-worktree-doc` on both, as it should be) | yes |
| 17 | the loss cell, after | the loss walk on the proposal's build in `bare`, `behind`, `sep`, `sub` | finalize 3 in all four; the planted file byte for byte; 21 of 22 step outputs identical to the candidate's in `plain`, the one that differs the `setup` ack's hook line. `sep` and `behind` are the two the advocate left undriven: they hold | yes |
| 18 | the adopter already there | `extra.sh <W> adopter-bare spike`, `adopter-sub spike`: `setup`, `doc:Cache`, `doc:Queue` on the candidate, then the proposal's build | `doc list`, `validate`: 1, *this project isn't set up — run `jigc setup`*; `jigc setup` 0; `doc list` lists `adr:cache` as `managed`; `validate` 0 with one advisory, *no action needed*; the old task: `finalize.no-task`, its route `jigc task list` 0 and empty; a third doc finalizes at 0; every one of the old home's 20 files has the hash it had | yes |
| 19 | the other worktrees of one bare repository | `extra.sh <W> siblings spike`: `git worktree add ../wt2 -b other main`, `../wt3 -b early <first commit>`; `jigc doc list`, `jigc setup` in each | `wt2`: lists the doc, `setup` 0. `wt3`: *isn't set up*, 1; `setup` 0 with its own install commit; `doc list` 0. `wt` undisturbed, its status empty; `proj/` holds `store.git wt wt2 wt3` | yes — and see *What else I found*, 1 |
| 20 | git's own hook run, as the advocate drove it | `git commit` of a staged file in `bare`, the hook installed by the proposal's build | exit 0, nothing on stderr | yes — literally. It is also what a hook that does nothing prints |
| **21** | **git's own hook run, on the commit the hook exists to refuse** | `… setup doc:Cache`, then `git mv docs/decisions/cache.md docs/decisions/cache-moved.md`; `git commit -m "move a managed doc by hand"` | `plain`, candidate and proposal: **exit 1**, *jigc: out-of-band managed-doc rename staged in this commit … (commit blocked).* `sub`, proposal: **exit 1**, the same line. **`bare`, `behind`, `sep`, proposal: exit 0, nothing on stderr, `HEAD` is the move**; `jigc validate` afterwards exits 1, `blocking (gates at finalize) · reconciliation.rename … likely renamed via git mv` | **no** — finding PD-1 |
| 22 | the user's own linked worktree of a `--separate-git-dir` or submodule repository | `git worktree add <root>/second-… -b second`; `jigc doc list`, `jigc start --workflow record-decision "record the cache"`, `jigc doc create adr --title Cache --task record-the-cache` there, candidate and proposal | 0 on both; on the proposal's build the task area is under the second worktree and `jigc task list` in the first checkout shows no task (on the candidate it shows it). `git worktree list --porcelain` names the git dir as its first entry in both layouts | yes on every exit; the sentence built on it is half right — *What else I found*, 2 |
| 23 | the standing directory that is itself no work tree | `jigc setup` typed in the folder holding a bare `.git`; and inside a bare repository; candidate and proposal | identical on both: exit 0, `.claude .gitignore .jigc CLAUDE.md` written into that folder; and exit 1 `setup.repo-root` | yes |
| 24 | the cost | `GIT_TRACE=<file> jigc start`, `jigc validate` in a linked worktree; `jigc start --task alpha-part` in a fan-out worktree; candidate against proposal; `trace: built-in: git` lines counted | 14 → 19, 15 → 20, 8 → 13 | yes |
| 25 | the excepted cell, on the previous release | `drive.sh behind <root> bin=<previous> setup setup doc:Cache reads …` | `setup` 0, 0; the install in `proj/` beside the bare `.git`; finalize 0; `jigc doc list — no committed docs`; `validate` 1 with six blocking findings | yes |

## Finding PD-1 — the hook the proposal installs is inert in three of its four layouts

**What.** In a worktree of a bare repository (beside it or behind a bare `.git`) and in a
`--separate-git-dir` checkout, `jigc setup` on the proposal's build writes jigc's `pre-commit`
into the hooks directory git names, reports it (`pre-commit hook → …/hooks/pre-commit`), and
the hook then never does anything: every `jigc` it starts resolves the **old** home, finds no
project there, prints *this project isn't set up* to a stderr the hook discards, and the hook
exits 0. Its warn-only arm (doc↔code drift) and its one blocking arm (an out-of-band managed-doc
rename staged in the commit) are both off. In the submodule it works.

**Why.** Read from git and from the trace, then driven:

- git exports `GIT_DIR` to a hook wherever the checkout's `.git` is a file. In `bare`, a hook of
  my own printing its environment: `GIT_DIR=bp/store.git/worktrees/wt`,
  `GIT_INDEX_FILE=bp/store.git/worktrees/wt/index`.
- The proposal's `is_a_checkout_of(dir, common)` runs
  `git -C <dir> rev-parse --path-format=absolute --show-toplevel --git-common-dir` and passes
  the ambient environment through. At the folder holding the bare repository that command
  **fails** with no `GIT_DIR` (`fatal: not a git repository`, exit 128) — the proposal's rule 4
  — and **succeeds** with the hook's `GIT_DIR`, printing that folder and `…/store.git`, exit 0:
  with `GIT_DIR` set and no work tree configured, git takes the directory it is run in as the
  work tree. So inside a hook rule 2 holds and `jigc_home` is `dirname(git-common-dir)` again.
- In `sep` the same (`core.bare` is false, no `core.worktree`). In the submodule git's
  `core.worktree` (`../../../../vendor/lib`) overrides the directory, the probe names the
  submodule's own checkout, the comparison fails and rule 4 stands — which is why `sub` works.
- In an ordinary linked worktree the probe under `GIT_DIR` answers *the main checkout* for the
  same accidental reason, which is the right answer there: `jigc doc list` with and without the
  hook's `GIT_DIR` printed the same bytes in `linked`, on the candidate and on the proposal.

**Not a regression against the candidate.** On the candidate no hook is installed in `bare` and
`sep` (the refusal the fork is about), and in `behind` its home is the same wrong directory in
a hook and out of one. The state is new with the proposal: an install that reports a hook, a
hook file where git reads it, and nothing behind it. **It is what makes the claim false, not a
loss of something that worked.**

**Why the gate is green over it.** The proposal's tests hold `pre-commit` to *is a file in the
directory `git rev-parse --git-path hooks` names*. None commits through it.

```yaml
claim: "with the proposal applied, in a worktree of a bare repository, a managed doc moved with a bare `git mv` is committed by plain git at exit 0 with nothing on stderr; the same commit in a plain checkout is refused by the hook jigc installed"
verdict: CONFIRMED
binary: "a release build of eeffe347 with the proposal's diff applied — mine, sha256 0d80e22f2f8727131325d8b90bfdd45867334124700e9c5578ceb1fadd3bf84a; not the candidate"
control-binary: "candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc — in a plain checkout"
regression: false
setup:
  - env: "HOME=<fresh dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, author and committer set"
  - ["mkdir", "proj"]
  - cwd: proj
  - ["git", "init", "-q", "--bare", "store.git"]
  - ["git", "-C", "store.git", "worktree", "add", "-q", "../wt", "-b", "main"]
  - cwd: proj/wt
  - write: "README.md = '# readme\n'"
  - ["git", "add", "README.md"]
  - ["git", "commit", "-q", "-m", "base"]
  - ["jigc", "setup"]                                                  # exit 0
  - ["jigc", "start", "--workflow", "record-decision", "record the cache"]
  - ["jigc", "doc", "create", "adr", "--title", "Cache", "--task", "record-the-cache"]
  - "for each of context, decision, consequences: `jigc doc set-slot adr:cache#<slot> --from-file - --task record-the-cache`, stdin 'Prose.\n'"
  - ["jigc", "doc", "set-field", "commit:record-the-cache#type", "--value", "feat", "--task", "record-the-cache"]
  - ["jigc", "doc", "set-field", "commit:record-the-cache#scope", "--value", "core", "--task", "record-the-cache"]
  - "`jigc doc set-slot commit:record-the-cache#summary --from-file - --task record-the-cache`, stdin 'record the cache'; the same for #body, stdin 'Body prose.'"
  - ["jigc", "task", "finalize", "record-the-cache"]                  # exit 0
repro:
  - ["git", "mv", "docs/decisions/cache.md", "docs/decisions/cache-moved.md"]
  - ["git", "commit", "-m", "move a managed doc by hand"]
  - ["jigc", "validate"]
expect:
  exit: [0, 0, 1]
  stderr_of_the_commit: ""
  head_subject_after: "move a managed doc by hand"
  validate_contains: "blocking (gates at finalize) · reconciliation.rename — tracked managed doc adr:cache (docs/decisions/cache.md) is missing; docs/decisions/cache-moved.md has the same content hash — likely renamed via `git mv`"
mechanism:
  - "`GIT_DIR=$(git rev-parse --absolute-git-dir) jigc doc list` in proj/wt: exit 1, `this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)`; without GIT_DIR: exit 0, the doc listed"
  - "`GIT_DIR=<that> git -C proj rev-parse --path-format=absolute --show-toplevel --git-common-dir`: exit 0, prints proj and proj/store.git; without GIT_DIR: exit 128"
control-in-a-plain-checkout: "candidate and the proposal's build, `git init` and the same steps: the `git commit` exits 1 with `jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).`; HEAD stays `feat(core): record the cache`"
variants:
  - "a worktree behind a bare `.git`: exits 0, 0 — the move lands"
  - "`--separate-git-dir`: exits 0, 0 — the move lands"
  - "a bare `.bare` with a `.git` gitfile beside it and a sibling worktree (`dotbare`): exits 0, 0 — the move lands"
  - "a submodule: the commit exits 1, blocked, as in a plain checkout"
with-a-variant-of-mine: "the proposal plus `.env_remove(\"GIT_DIR\")`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, `GIT_INDEX_FILE`, `GIT_PREFIX` on the probe's `Command` (sha256 122cae14…): the commit exits 1, blocked, in bare, behind and sep, and `jigc doc list` under the hook's GIT_DIR exits 0. Driven on this step only — no gate, no other layout, no other door. It is evidence about the cause, not a second proposal"
observed: "<W>/hook-bare-spike.*, <W>/hook-behind-spike.*, <W>/hook-sep-spike.*, <W>/hook-sub-spike.*, <W>/hook-plain-cand.*, <W>/hook-plain-spike.*, <W>/HK-dotbare-spike.*; git's own answers <W>/gitfacts.*; the variant <W>/variant-hook-{bare,behind,sep}.*"
pinned-by: "UNPINNED — the proposal's tests assert the hook is a file in git's hooks directory; none commits through it in these layouts"
```

**Its reach beyond the installed hook, read and not driven:** any `jigc` started from a git
hook of the user's in those three layouts (a `commit-msg`, a hook manager) inherits the same
`GIT_DIR` and resolves the old home. `crates/cli/src/repo.rs` already declares a
*user-exported* `GIT_DIR` out of bounds for its probes; this one is exported by git, on every
commit, in exactly the layouts the proposal is about.

## What else I found — about the proposal, none of it a step that failed

1. **The worktrees of one bare repository are separate homes that share one hooks directory.**
   Driven (`siblings`): `git rev-parse --git-path hooks` names `proj/store.git/hooks` from all
   three worktrees, and `jigc uninstall` typed in `wt2` (exit 0, *removed pre-commit hook*)
   removes the hook `wt` and `wt3` were installed with — their `.jigc/config` stays, nothing
   tells them, and `jigc setup` re-typed in `wt` puts it back (exit 0, status empty). The
   `setup` ack calls the hook *local to this checkout*, which is not true there. And a task
   open in `wt` is not in `jigc task list` typed in `wt2` (on the candidate all of them share
   the one, wrong, home). Each is a consequence of *a home is a checkout* the proposal states
   for the doc store and does not state for the hook or the task list.
2. **"git offers no main work tree there" is true of `--separate-git-dir` and not of a
   submodule.** `git worktree list --porcelain` prints the git dir as the first entry in both —
   as reported. But asked *at the submodule's git dir*, `git rev-parse --show-toplevel` exits 0
   and names `super/vendor/lib` (its config carries `core.worktree`); at the separate git dir it
   exits 128. So for a submodule git can name the first checkout, and the proposal's choice —
   the user's linked worktree of a submodule is a home of its own, with its own task list and
   no `finalize.linked-worktree-doc` guard — is a choice, not the absence of a source.
3. **Source comments the proposal leaves saying the old rule**, in the file it edits and one
   beside it: the doc comment of `home_is_a_checkout` (*`jigc_home` answers
   `dirname(git-common-dir)` wherever `.git` is a file*, and its three layouts), and
   `InstallSubject::NoWorkTree` in `crates/cli/src/setup.rs` (*the homes this is true of are the
   ones `jigc_home` resolves to `dirname(git-common-dir)` where no checkout stands*). The
   proposal says nothing else in `src/` moves, and (C) lists design docs and guides only.
4. **The layout the advocate's walk left out behaves as the four do.** `dotbare`: on the candidate the walk is the
   broken one (`validate` 1, second finalize 3, `milestone create` 1, `uninstall` 1); on the
   proposal's build every step exits 0 and 70 of 72 outputs match the candidate's in `plain` —
   and the hook is inert there too.
5. **A count I could not reproduce:** *36 call sites in 9 files reach the home through this one
   resolver*. A search for `jigc_home(` under `crates/cli/src` finds 17 lines in 5 files, 10 of
   them in `repo.rs`. It is not a step and nothing rests on it; the resolver is one function
   either way.

## Undriven

- **The record edits of (C).** The proposal says they follow the ruling and are not part of
  what it drove. I applied none, so no fence that reads those docs ran over them.
- **An older git.** `--path-format=absolute` needs git 2.31; there is none on this machine.
- **My variant beyond the one step.** It was not gated and not walked; nothing here says it is
  right in an ordinary linked worktree, only that the hook blocks again in three layouts.
- **The doc↔code drift arm of the hook** in these layouts. It reads the same `jigc validate`
  report through the same home, so it is off by the same cause — reasoned, not driven.
- **The previous release's content hash** was not asked of the tool by me.
- **`jigc uninstall --force` on the proposal's build**, and any adapter profile but the one
  `setup` installs.
- **The regression set's part 2 in these layouts** — its fixtures do not exist.

## Left open — not this proposal

1. Everything under the advocate's *Left open* 1–7 and 9–10 reproduced as a fact about the
   candidate and the previous release (rows 7–9 above). They stand whatever is decided.
2. `jigc setup` typed in the folder holding a bare `.git` installs there at exit 0, on the
   candidate and on the proposal alike (row 23). The proposal does not close it and says so.
3. On the candidate, `jigc unmanage docs/decisions/cache.md` in `behind` exits 0 claiming no
   file was at a path where one is — the advocate typed it on the previous release only; it is
   the same on the candidate.
4. An adopter's open task under the old home is unreachable after the change: `jigc task list`
   is empty and the staged ADR's authored slots lie under the old `.jigc/tasks/` where no door
   reads them. The proposal names this and proposes a note. Whether a note is enough for prose
   somebody wrote is the human's to weigh; driven, row 18.

## Tree state

The repository was read and not written: branch `fix/canary-one` at eeffe347, its status the
one untracked directory `completions/artifacts/canary-one/r1/`, as I found it. No commit, no
stage, no branch, no push, no install, no build there. Under `<W>`: five clones (`full`, `half`,
`testsonly`, `norule3`, `variant`), each detached at eeffe347 with a diff applied and nothing
committed; four target directories; the layout roots; three scripts of mine and `cmp.py`. Two
gates ran whole, one after the other, each in its own clone.

<!-- end of report -->
