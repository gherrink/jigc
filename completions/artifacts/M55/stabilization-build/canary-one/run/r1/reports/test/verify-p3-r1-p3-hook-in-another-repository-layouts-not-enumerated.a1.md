# verify-real — `r1-p3-hook-in-another-repository-layouts-not-enumerated` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, driven from nothing. Door: `jigc setup`. Clause it is said to break:
`no-lost-files`. Triage's grade: *unclear*. The finding has no block of its own: it is the
section `The class` of an earlier verifier's report, which says the layouts in which the
`pre-commit` hook lands in a repository other than the one `setup` was typed in were not
enumerated, and that where the directory git runs the hook in *does* hold an install, nothing
there speaks. Triage asked for the enumeration and, per layout, for what the block's
`jigc validate` does on a commit of the other repository.

## Verdict in one paragraph

**`refuted` as a blocker — basis `breaks-no-clause`. `contested: false`.** The observation the
finding rests on is real and reproduces: in nine of the eleven layouts driven, `jigc setup`
exits 0 and writes jigc's block into a `pre-commit` file that commits of *another* checkout or
*another* repository run. In the other two it exits 1 at its hook step and the hook is left as
it was. In all eleven, a commit that adds a tracked file and a commit of a staged `git mv` of
that file both land at exit 0 in the other checkout or repository with exactly the staged
content; the only files that change anywhere under the root across each commit are the files
git itself rewrites on a commit; and across a by-hand run of the block's one jigc command and
of the installed hook under `sh -x`, the manifests are identical — **the block's `validate`
writes no file in any layout**, whether it answers *not set up* (exit 1, empty stdout: another
repository that is no jigc project) or runs a real sweep (exit 0: the same repository's other
checkout, or another repository that is a jigc project of its own). No file was removed in any
step of any layout. The one commit the block refused is the designed one — a bare `git mv` of
a managed doc, committed in the main checkout of the repository the install belongs to — and
it lost nothing: the rename is still staged afterwards. So the class the earlier report left
open holds no loss and no wrong write on the candidate, in the layouts listed below. What I
noticed on the way and did not pursue is in *Left open*, and the first item there is not
small.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

The candidate's directory was first on `PATH` for every driven step and `command -v jigc`
printed its path — checked at the top of every driver run, where a mismatch exits before the
first git command. Nothing was built and nothing under `target/` was driven. **The previous
release was hashed and not driven:** the regression fact is owed with `confirmed` only, and
this verdict is `refuted`.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-hooklayouts.Z8o4ju`. **One fresh root per layout**, each minted the same
  way inside it — `c1-L1.fMarMf`, `c1-L2.F7ZQ3A`, `c1-L3.PQaAM6`, `c1-L3b.fM67uU`,
  `c1-L3c.TVwn4c`, `c1-L4.PuBNJj`, `c1-L4j.yU9oiC`, `c1-L4g.pngYpd`, `c1-L5.ACCI57`,
  `c1-L6.jhKxeb`, `c1-L1r.mjOBbR`. `<R>` below is the root of the layout in question. No
  root is a reporter's or another verifier's.
- Environment of every root: `HOME` a fresh empty directory inside the root,
  `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null` (in `L4g` a config file of the root's
  own, for the length of that layout), the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables
  set to a synthetic identity. Git 2.54.0 (Apple Git-157), macOS.
- Every exit status was read bare — `( cd <dir> && cmd ) > out 2> err; rc=$?` — never through
  a pipe.
- **Ten layouts are hand-built** from git steps, because no state of `dev/jigc-rig` builds a
  second checkout or a second repository. **One (`L1r`) starts from the rig**:
  `dev/jigc-rig committed-singletons --binary <the candidate>`, its stdout captured alone and
  evaluated in two steps, with two `git worktree add` on top — so that one layout has a store
  with managed docs in it and the sweep has something to read.
- **A manifest** — sha256 of every file and every link under the root, git object files and my
  evidence directory left out — was taken before and after `setup`, before and after each
  commit, and before and after the by-hand runs; two manifests are compared as *added /
  removed / changed* per path.
- **The user's own hook.** In the ten hand-built layouts a 106-byte `pre-commit` stands in the
  hooks directory before `setup`: `#!/bin/sh`, two lines that write the `GIT_*` environment
  git hands the hook and the hook's working directory to a file, and
  `echo own-hook-ran >&2`. It is a pre-existing foreign hook, so `setup` wraps it — and it
  records, from inside the real commit, the environment the by-hand runs are then given.
- **How the block was traced, three ways, per layout.** (1) Commit A ran under `GIT_TRACE`
  pointed at a file. (2) Between staging the move and committing it, the block's one jigc
  command — `jigc validate --format json` — was typed by hand in the directory git ran the
  hook in, under the recorded hook environment, stdout and stderr kept apart (the hook sends
  stderr to the null device), itself under `GIT_TRACE`. (3) The installed hook, unedited, was
  run as `sh -x <hook>` in the same directory under the same environment.
- **What I reconstructed, since the finding hands no block.** The two commit shapes are the
  earlier report's (`Repro VR-2`): add `docs/a.md` and commit (commit A), `git mv docs/a.md
  docs/b.md` and commit (commit B). To them `L1r` adds a third: a bare
  `git mv VISION.md moved-VISION.md` of a managed doc, committed — the one shape the block's
  only `exit 1` exists for. In `L1r` the rig's hook is jigc's alone (no foreign hook to record
  the environment), so the by-hand environment there is **reconstructed** in the shape the
  other layouts recorded.
- An earlier complete run in one shared root (`<W>/cand.EHtM14`, nine layouts) and a first
  attempt whose by-hand replay was broken by my own script (`<W>/cand.iqphk7`: it handed
  `env` a variable with a space in it) are in `<W>` too. The run in the shared root agrees
  with the per-layout roots cell for cell; the tables below are the per-layout roots'.

## The enumeration — how it was made, and what it is not

I listed the ways git lets the directory `git rev-parse --git-path hooks` names be other than
`<the checkout>/.git/hooks`, and for each asked where jigc asks it. **jigc does not ask in the
directory `setup` is typed in**: `install_precommit_hook` asks at *jigc_home*
(`crates/cli/src/setup.rs`, `resolve_hooks_dir(jigc_home)`), and `jigc_home`
(`crates/cli/src/repo.rs`) is the checkout itself when its `.git` is a directory and
`dirname(git-common-dir)` when its `.git` is a file. That second rule is what splits the two
layouts triage named into several, because the answer then depends on what stands at the git
dir's parent.

| # | layout | the family |
|---|---|---|
| L1 | a linked worktree of an ordinary repository; `setup` typed in the worktree | common hooks dir of one repository |
| L1r | the same, the repository already set up and holding managed docs (rig) | common hooks dir of one repository |
| L3 | a worktree of a bare repository, the bare dir beside the worktrees, its parent in no repository | common hooks dir, home is no checkout |
| L3b | worktrees of a bare repository behind a `.git` pointer file (`proj/.bare` + `proj/.git`) | common hooks dir, home is no checkout |
| L3c | worktrees of a bare repository that is itself named `.git` (`proj/.git`, `core.bare=true`) | common hooks dir, home is no checkout |
| L2 | a `--separate-git-dir` checkout, the git dir's parent in no repository | relocated git dir |
| L5 | a `--separate-git-dir` checkout whose git dir sits inside **another repository's** work tree | relocated git dir |
| L4 | one `core.hooksPath` directory named in the local config of two repositories; the other is no jigc project | shared hooks path |
| L4j | the same; the other repository is a jigc project of its own | shared hooks path |
| L4g | `core.hooksPath` in the user's global git config; the other is no jigc project | shared hooks path |
| L6 | two repositories whose `.git/hooks` is a symlink to one shared directory | shared hooks dir without `core.hooksPath` |

**Eleven layouts driven, on the candidate, on one git on one platform.** The list was made by
hand from git's own rules; it is not derived from a registry, and it is not a bound — see
*The class*. The submodule layout is the earlier report's and was not driven again.

## What was observed, per layout

`A`/`B` are the two commits in the other checkout or repository. *Hook env* is what the user's
own hook recorded from inside commit A. In every row the commit's whole stdout is empty and
its whole stderr is the one line `own-hook-ran` (in `L1r`, where no foreign hook stands, both
are empty).

| # | `setup` typed in · exit | the hooks dir git names there | where jigc asks · what git answers | the hook file after `setup` | the other commit's site | hook env | by-hand `validate` there: exit · stdout · stderr | `sh -x`: `report` · `moves` · exit | A · B exit | files changed by A, by B | files changed by the by-hand runs |
|---|---|---|---|---|---|---|---|---|---|---|---|
| L1 | `l1/wt` · **0** | `l1/main/.git/hooks` | `l1/main` · the same dir | `l1/main/.git/hooks/pre-commit`, 106 → 5218 bytes, mode 755, two marker lines, the user's last 96 bytes intact | `l1/main` (the main checkout) | `GIT_INDEX_FILE=.git/index`, no `GIT_DIR`, cwd `l1/main` | **0** · clean envelope (`"blocking_probes": []`, `"findings": []`) · empty | envelope · empty · 0 | 0 · 0 | git's five | none |
| L1 | (the same install) | | | | `l1/wt2` (a second linked worktree) | `GIT_DIR=<R>/l1/main/.git/worktrees/wt2`, `GIT_INDEX_FILE` absolute under it, cwd `l1/wt2` | **0** · the same clean envelope · the *served from the main checkout* note | envelope · empty · 0 | 0 · 0 | git's five (the worktree's `COMMIT_EDITMSG`, `index`, `logs/HEAD`; the branch's ref and log) | none |
| L3 | `l3/wt1` · **1** | `l3/store.git/hooks` | `l3` · `fatal: not a git repository` | **unchanged**: 106 bytes, no marker line | `l3/wt2` | `GIT_DIR` and `GIT_INDEX_FILE` absolute under `store.git/worktrees/wt2` | (the hook holds no block) | no block: the trace is the user's own lines alone · 0 | 0 · 0 | git's five | none |
| L3b | `proj/main-wt` · **0** | `proj/.bare/hooks` | `proj` · the same dir | `proj/.bare/hooks/pre-commit`, 106 → 5218, two marker lines, last 96 bytes intact | `proj/feat-wt` | `GIT_DIR` and `GIT_INDEX_FILE` absolute under `.bare/worktrees/feat-wt` | **0** · clean envelope · the *served from the main checkout at `<R>/l3b/proj`* note | envelope · empty · 0 | 0 · 0 | git's five | none |
| L3c | `proj/main-wt` · **0** | `proj/.git/hooks` | `proj` · the same dir | `proj/.git/hooks/pre-commit`, 106 → 5218, two marker lines, last 96 bytes intact | `proj/feat-wt` | `GIT_DIR` and `GIT_INDEX_FILE` absolute under `.git/worktrees/feat-wt` | **0** · clean envelope · the same note | envelope · empty · 0 | 0 · 0 | git's five | none |
| L2 | `l2/repo` · **1** | `l2/store.git/hooks` | `l2` · `fatal: not a git repository` | **unchanged**: 106 bytes, no marker line | `l2/repo` itself (there is no other) | `GIT_DIR=<R>/l2/store.git`, `GIT_INDEX_FILE` absolute under it | (the hook holds no block) | no block: the user's own lines alone · 0 | 0 · 0 | git's five | none |
| L5 | `l5/repo` · **0** | `l5/enc/gitdirs/x.git/hooks` | `l5/enc/gitdirs` · `l5/enc/.git/hooks` — **the enclosing repository's** | `l5/enc/.git/hooks/pre-commit`, 106 → 5218, two marker lines, last 96 bytes intact | `l5/enc` (the enclosing repository) | `GIT_INDEX_FILE=.git/index`, cwd `l5/enc` | **1** · 0 bytes · the one-key *this project isn't set up* error | empty · empty · 0 | 0 · 0 | git's five | none |
| L4 | `l4/a` · **0** | `l4/shared-hooks` | `l4/a` · the same dir | `l4/shared-hooks/pre-commit`, 106 → 5218, two marker lines, last 96 bytes intact | `l4/b` | `GIT_INDEX_FILE=.git/index`, cwd `l4/b` | **1** · 0 bytes · *isn't set up* | empty · empty · 0 | 0 · 0 | git's five | none |
| L4j | `l4j/b`, then `l4j/a` · **0**, **0** | `l4j/shared-hooks` | each in its own checkout · the same dir | the same file; after the second `setup` it is byte-identical to what the first left (`cmp` exit 0) | `l4j/b` | `GIT_INDEX_FILE=.git/index`, cwd `l4j/b` | **0** · clean envelope, b's own store · empty | envelope · empty · 0 | 0 · 0 | git's five | none |
| L4g | `l4g/a` · **0** | `l4g/global-hooks` | `l4g/a` · the same dir | `l4g/global-hooks/pre-commit`, 106 → 5218, two marker lines, last 96 bytes intact | `l4g/b` | `GIT_INDEX_FILE=.git/index`, cwd `l4g/b` | **1** · 0 bytes · *isn't set up* | empty · empty · 0 | 0 · 0 | git's five | none |
| L6 | `l6/a` · **0** | `l6/shared-hooks` (through the link) | `l6/a` · the same dir | `l6/shared-hooks/pre-commit`, 106 → 5218, two marker lines, last 96 bytes intact; both `.git/hooks` are still links | `l6/b` | `GIT_INDEX_FILE=.git/index`, cwd `l6/b` | **1** · 0 bytes · *isn't set up* | empty · empty · 0 | 0 · 0 | git's five | none |
| L1r | `l1r/wt`, in a set-up repository · **0** | `<rig repo>/.git/hooks` | the rig's repo · the same dir | the rig's hook, 5129 bytes, byte-identical after the `setup` typed in the worktree (`cmp` exit 0) | the rig's repo (main checkout) | reconstructed: `GIT_INDEX_FILE=.git/index` | **0** · an envelope with one advisory finding (`schema-conformance.repeatable-populated`), `"blocking_probes": []` · empty | envelope · empty · 0 | 0 · 0 | git's five | none |
| L1r | (the same install) | | | | `l1r/wt2` (a second linked worktree) | reconstructed: `GIT_DIR` and `GIT_INDEX_FILE` absolute under `.git/worktrees/wt2` | **0** · the same envelope · the *served from the main checkout* note | envelope · empty · 0 | 0 · 0 | git's five | none |

"git's five" is `COMMIT_EDITMSG`, `index`, `logs/HEAD`, the branch's ref and the branch's log —
in a linked worktree the first three are the worktree's own under `worktrees/<name>/`. A first
commit on a branch *adds* its log file rather than changing it. No step of any layout shows a
*removed* path (`command grep -c '^REMOVED'` over the eleven logs prints 0 for each).

In `L2` and `L3` the hook holds no jigc block, so the by-hand `validate` there is not *the
block's*; typed anyway, it exits 0 with the clean envelope, because the `setup` that exited 1
had already written a project layer at the git dir's parent.

After commit B, in every layout: `git show --stat -M HEAD` is `docs/{a.md => b.md} | 0`,
`git ls-tree -r --name-only HEAD` holds `docs/b.md` and no `docs/a.md`, and
`git status --porcelain --untracked-files=all --ignored` in the committing checkout is empty —
but in `L5`, where it lists the 25 untracked files of the relocated git dir that stood there
before `setup`, and in `L1r`'s main checkout, where it lists the three ignored cache files
under `.jigc/` the rig's own construction left.

**What git's trace of commit A shows the hook starting.** Where `validate` answers *not set
up* (`L4`, `L4g`, `L5`, `L6`): no git process between the hook's start and git's own
`maintenance run --auto` — the trace holds `git commit` and `git maintenance` and nothing
else. Where it sweeps: `git ls-files` twice and `git log` (five times over a fresh install,
once over the rig's store), and in a linked worktree seven `git rev-parse` and one
`git symbolic-ref` more. All of them read.

### The one shape the block can refuse — `L1r`, a bare `git mv` of a managed doc

| committed in | staged | by-hand `validate`: exit · what the envelope holds | `sh -x`: `moves` · verdict · exit | the commit | afterwards |
|---|---|---|---|---|---|
| the main checkout | `R100 VISION.md moved-VISION.md` | **1** · `reconciliation.rename` and `schema-conformance.home-vacated`, two blocking; the route span `git -C <rig repo> mv moved-VISION.md VISION.md` | that span · `verdict=0` · **1** | exit **1**; stderr the one line `jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).` | 7 commits, as before; `HEAD` unmoved; porcelain `R  VISION.md -> moved-VISION.md` — **the rename is still staged**; the one file the refused commit changed is `.git/index` |
| the linked worktree `wt2` | the same line | **0** · the advisory finding only, `"blocking_probes": []`, no `git -C` span | empty · — · 0 | exit **0**, stdout and stderr empty; the commit landed (8 commits on `side2`) | porcelain empty; git's five changed |

The first row is `design/validation.md` → *The M19 pre-commit backstop stays doc↔code-keyed
for content; M35 adds a hard block on rename* doing what it says, in the repository the
install belongs to, from a hook a `setup` typed in a linked worktree regenerated. The second
row is in *Left open*.

## The finding against the design that owns the behaviour

- **`L1`, `L1r` — the hook in the main checkout's hooks dir, from a `setup` typed in a linked
  worktree, is the design.** `design/assistant-adapter.md` → *Where it installs: jigc_home,
  from every cwd (M53)*: "The install is one per **repository**, not one per checkout … and
  the `pre-commit` hook lives in the repository-wide common hooks dir whichever checkout
  installed it", and the ack says so (*installed at `<R>/l1/main` — the main checkout … not
  the worktree you are standing in*). The other checkout here is the same repository's.
- **`L4`, `L4g`, `L6` — a hooks directory the user's own git configuration shares.** The
  install discipline of the same doc: the hook "must resolve the real hooks dir (honoring
  `core.hooksPath` …)", "non-destructive (preserves/wraps any existing `pre-commit` hook)".
  The other repository's commit is the doc's case (iii) of the output contract —
  "not-a-jigc-project" → "prints nothing and exits 0", "an unconfigured … repo never breaks a
  commit" — and that is what each of the three showed. `L4j` is case (i), a clean store, the
  other repository's own.
- **`L2`, `L3`, `L3b`, `L3c`, `L5` — the three layouts whose `.git` is a file with no second
  checkout behind it.** `implementation/decisions-pending.md` → the M57 list, row (D) holds
  open whether a worktree of a bare repository and a `--separate-git-dir` checkout are
  supported at all, and records as pre-existing that `setup` "exits 1 at its hook step in the
  sibling-bare and `--separate-git-dir` layouts, after writing" and that its ack "calls that
  directory *the main checkout*". Both were seen again. That row is the human's and I grade
  nothing of it.

## Does it break the clause, inside its scope

The clause, as the run's opening names it and as `DECISIONS.md` → *2026-10-04 — The exit rule,
revised* sharpens it: "In a healthy repository used as documented — which includes ordinary
git configuration: line-ending conversion, `status.showUntrackedFiles`, a symlinked
`CLAUDE.md`, linked worktrees — no jigc command at exit 0 destroys bytes no git object holds or
commits content the user did not ask for."

The finding's subject is the hook's block on a commit of the other repository. For that
subject, in eleven layouts:

- **Bytes destroyed: none.** No path is removed in any step; across the by-hand runs of the
  block's `validate` and of the whole hook the manifests are identical in every layout,
  including the two where the sweep reads a real store from a linked worktree; across each
  commit only git's own files change. The user's own hook survives every rewrite — its last
  96 bytes are byte-identical before and after, and it still runs, after the block.
- **Content committed that the user did not ask for: none, by the block.** Every commit A
  holds the one added file and every commit B the one staged rename. The block stages
  nothing.
- **A commit refused: one, the designed one,** and it lost nothing.

Linked worktrees are inside the clause's scope by its own words, and `L1`/`L1r` are clean.
Whether the bare, relocated-git-dir and shared-hooks layouts are inside it is not settled by
anything I can cite — row (D) is open — and **the verdict does not turn on it**: read with all
eleven inside the scope, the block breaks nothing.

**Why `breaks-no-clause` and not `does-not-reproduce`.** The hook does land outside the
checkout `setup` was typed in, in nine layouts, and other checkouts' and other repositories'
commits do run `jigc validate` first from it. That is real and stays a row of the ledger. What
does not follow from it is a loss or a wrong write by that block.

**Why not `contested`.** The finding argues with no settled decision, and the verdict needs no
ruling.

**What this verdict does not cover.** It is about the block. In `L5`, `jigc setup` *itself*
makes a commit in a repository it was not typed in. That is the `setup` door's own write and
not the hook's effect; I did not look for it, I did not pursue it, and it is the first item of
*Left open* — a `refuted` here must not be read as a verdict on it.

## Coverage

The finding makes no coverage claim. For the block's `pinned-by` I looked this far:
`crates/cli/tests/precommit_hook_acceptance.rs` holds
`commit_is_silent_on_unconfigured_repo`, which I read: it runs `setup` in one repository,
removes that repository's own `.jigc/`, commits, and asserts the commit's success and the
absence of the drift warning. That pins the mechanism the four *not set up* layouts rest on,
in a single repository; it builds no second checkout and no shared hooks directory. The unit
tests `install_precommit_honors_core_hookspath` and
`install_precommit_resolves_worktree_common_hooks_dir` (`crates/cli/src/setup.rs`) pin where
the hook lands, by their names and the lines I read around them. `command grep -rl hooksPath`
over `crates/cli/tests` and `tooling-tests` names thirteen files. I read their matching lines
and none of them whole: they set a hooks path — in the tree, outside the repository, inside a
submodule or an embedded repository — for one repository's `setup`, its summary or its install
commit, or they mask hooks with the null device; no matching line commits in a second
repository. I did not read the suites that add a linked worktree; one that commits in a
second checkout after a `setup` in the first would have been missed if it does not name
`hooksPath`.

## Repro VR-3 — where the hook lands outside the checkout `setup` was typed in, its block's `jigc validate` writes nothing and changes no commit

```yaml
claim: "in a layout where `git rev-parse --git-path hooks` leaves the checkout `jigc setup` is typed in, the hook's block loses bytes, commits unasked content or wrongly refuses on a commit of the other checkout or repository"
verdict: REFUTED
basis: breaks-no-clause
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous-release: "hashed (accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d), not driven — the verdict is refuted"
env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, GIT_AUTHOR_NAME/EMAIL and GIT_COMMITTER_NAME/EMAIL set; one fresh root per layout"
own-hook: "'#!/bin/sh\\nenv | grep \"^GIT_\" | LC_ALL=C sort > \"$HOOKENV_OUT\"\\npwd >> \"$HOOKENV_OUT\"\\necho own-hook-ran >&2\\n', mode 755 — 106 bytes, written into the hooks dir BEFORE setup"
layouts:
  L1:
    setup:
      - ["git", "init", "-q", "l1/main"]
      - ["git", "-C", "l1/main", "commit", "-q", "--allow-empty", "-m", "base"]
      - ["git", "-C", "l1/main", "worktree", "add", "-q", "../wt", "-b", "side"]
      - ["git", "-C", "l1/main", "worktree", "add", "-q", "../wt2", "-b", "side2"]
      - own-hook: l1/main/.git/hooks/pre-commit
      - { cwd: l1/wt, argv: ["jigc", "setup"], exit: 0 }
    other: [l1/main, l1/wt2]
    hook: l1/main/.git/hooks/pre-commit
    validate_by_hand: { exit: 0, stdout_json: { blocking_probes: [], findings: [] } }
  L2:
    setup:
      - ["git", "init", "-q", "--separate-git-dir", "<R>/l2/store.git", "l2/repo"]
      - ["git", "-C", "l2/repo", "commit", "-q", "--allow-empty", "-m", "base"]
      - own-hook: l2/store.git/hooks/pre-commit
      - { cwd: l2/repo, argv: ["jigc", "setup"], exit: 1, stderr_contains: "setup.install-hook" }
    other: [l2/repo]
    hook: "l2/store.git/hooks/pre-commit — unchanged, 106 bytes, no marker line"
  L3:
    setup:
      - ["git", "init", "-q", "--bare", "l3/store.git"]
      - "one commit pushed into it as refs/heads/main from a throwaway repository; HEAD pointed at it"
      - ["git", "-C", "l3/store.git", "worktree", "add", "-q", "../wt1", "-b", "one", "main"]
      - ["git", "-C", "l3/store.git", "worktree", "add", "-q", "../wt2", "-b", "two", "main"]
      - own-hook: l3/store.git/hooks/pre-commit
      - { cwd: l3/wt1, argv: ["jigc", "setup"], exit: 1, stderr_contains: "setup.install-hook" }
    other: [l3/wt2]
    hook: "l3/store.git/hooks/pre-commit — unchanged, 106 bytes, no marker line"
  L3b:
    setup:
      - ["git", "init", "-q", "--bare", "l3b/proj/.bare"]
      - "one commit pushed into it as refs/heads/main; HEAD pointed at it"
      - write: "l3b/proj/.git = 'gitdir: ./.bare\\n'"
      - ["git", "-C", "l3b/proj", "worktree", "add", "-q", "main-wt", "main"]
      - ["git", "-C", "l3b/proj", "worktree", "add", "-q", "feat-wt", "-b", "feat", "main"]
      - own-hook: l3b/proj/.bare/hooks/pre-commit
      - { cwd: l3b/proj/main-wt, argv: ["jigc", "setup"], exit: 0 }
    other: [l3b/proj/feat-wt]
    hook: l3b/proj/.bare/hooks/pre-commit
    validate_by_hand: { exit: 0, stdout_json: { blocking_probes: [], findings: [] } }
  L3c:
    setup:
      - "a throwaway repository with one commit, then:"
      - ["git", "clone", "-q", "--bare", "<R>/l3c/seed", "l3c/proj/.git"]
      - ["git", "-C", "l3c/proj/.git", "worktree", "add", "-q", "../main-wt", "-b", "main-wt"]
      - ["git", "-C", "l3c/proj/.git", "worktree", "add", "-q", "../feat-wt", "-b", "feat"]
      - own-hook: l3c/proj/.git/hooks/pre-commit
      - { cwd: l3c/proj/main-wt, argv: ["jigc", "setup"], exit: 0 }
    other: [l3c/proj/feat-wt]
    hook: l3c/proj/.git/hooks/pre-commit
    validate_by_hand: { exit: 0, stdout_json: { blocking_probes: [], findings: [] } }
  L4:
    setup:
      - "two repositories l4/a and l4/b, one empty commit each; in each:"
      - ["git", "config", "core.hooksPath", "<R>/l4/shared-hooks"]
      - own-hook: l4/shared-hooks/pre-commit
      - { cwd: l4/a, argv: ["jigc", "setup"], exit: 0 }
    other: [l4/b]
    hook: l4/shared-hooks/pre-commit
    validate_by_hand: { exit: 1, stdout: "", stderr_contains: "this project isn't set up" }
  L4j:
    setup:
      - "as L4, under l4j/"
      - { cwd: l4j/b, argv: ["jigc", "setup"], exit: 0 }
      - { cwd: l4j/a, argv: ["jigc", "setup"], exit: 0 }     # the hook is byte-identical after it
    other: [l4j/b]
    hook: l4j/shared-hooks/pre-commit
    validate_by_hand: { exit: 0, stdout_json: { blocking_probes: [], findings: [] } }
  L4g:
    setup:
      - write: "l4g/gitconfig = '[core]\\n\\thooksPath = <R>/l4g/global-hooks\\n'; GIT_CONFIG_GLOBAL names it"
      - "two repositories l4g/a and l4g/b, one empty commit each"
      - own-hook: l4g/global-hooks/pre-commit
      - { cwd: l4g/a, argv: ["jigc", "setup"], exit: 0 }
    other: [l4g/b]
    hook: l4g/global-hooks/pre-commit
    validate_by_hand: { exit: 1, stdout: "", stderr_contains: "this project isn't set up" }
  L5:
    setup:
      - ["git", "init", "-q", "l5/enc"]
      - ["git", "-C", "l5/enc", "commit", "-q", "--allow-empty", "-m", "base"]
      - ["git", "init", "-q", "--separate-git-dir", "<R>/l5/enc/gitdirs/x.git", "l5/repo"]
      - ["git", "-C", "l5/repo", "commit", "-q", "--allow-empty", "-m", "base"]
      - own-hook: l5/enc/.git/hooks/pre-commit
      - { cwd: l5/repo, argv: ["jigc", "setup"], exit: 0 }
    other: [l5/enc]
    hook: l5/enc/.git/hooks/pre-commit
    validate_by_hand: { exit: 1, stdout: "", stderr_contains: "this project isn't set up" }
    also_observed: "setup's own install commit landed in l5/enc — see Left open, not asserted by this block"
  L6:
    setup:
      - "two repositories l6/a and l6/b, one empty commit each; in each, .git/hooks moved aside and replaced by a symlink to <R>/l6/shared-hooks"
      - own-hook: l6/shared-hooks/pre-commit
      - { cwd: l6/a, argv: ["jigc", "setup"], exit: 0 }
    other: [l6/b]
    hook: l6/shared-hooks/pre-commit
    validate_by_hand: { exit: 1, stdout: "", stderr_contains: "this project isn't set up" }
  L1r:
    setup:
      - fixture: committed-singletons          # dev/jigc-rig committed-singletons --binary <candidate>
      - ["git", "-C", "<REPO>", "worktree", "add", "-q", "<R>/l1r/wt", "-b", "side"]
      - ["git", "-C", "<REPO>", "worktree", "add", "-q", "<R>/l1r/wt2", "-b", "side2"]
      - { cwd: l1r/wt, argv: ["jigc", "setup"], exit: 0 }    # the hook is byte-identical after it
    other: ["<REPO>", l1r/wt2]
    hook: "<REPO>/.git/hooks/pre-commit"
    validate_by_hand: { exit: 0, stdout_json_contains: { blocking_probes: [] } }
repro:                                           # in each `other`, in order
  - write: "docs/a.md = '# a tracked file\\n'"
  - ["git", "add", "docs/a.md"]
  - ["git", "commit", "-q", "-m", "add docs/a.md"]                     # commit A
  - ["git", "mv", "docs/a.md", "docs/b.md"]
  - ["jigc", "validate", "--format", "json"]                           # by hand, cwd and GIT_* as the hook recorded them
  - ["sh", "-x", "<hook>"]                                             # the installed hook, unedited, same cwd and env
  - ["git", "commit", "-q", "-m", "move docs/a.md to docs/b.md"]       # commit B
repro_rename:                                    # L1r only: first in l1r/wt2, then in <REPO>
  - ["git", "mv", "VISION.md", "moved-VISION.md"]
  - ["jigc", "validate", "--format", "json"]
  - ["sh", "-x", "<hook>"]
  - ["git", "commit", "-q", "-m", "move VISION.md by hand"]
expect:
  commit_A: { exit: 0, stdout: "", stderr: "own-hook-ran\n" }          # L1r: stderr ""
  commit_B: { exit: 0, stdout: "", stderr: "own-hook-ran\n" }          # L1r: stderr ""
  hook_under_sh_x: { exit: 0, trace_lacks: ["git diff --cached", "exit 1"], trace_contains_where_the_hook_holds_the_block: ["+ moves="] }
  head_after_B: "`git show --stat -M HEAD` = docs/{a.md => b.md}; `git ls-tree -r --name-only HEAD` holds docs/b.md and no docs/a.md"
  files_changed_across_each_commit: "COMMIT_EDITMSG, index, logs/HEAD, the branch's ref and its log (under worktrees/<name>/ for the first three in a linked worktree) — no other path under the root, HOME included"
  files_changed_across_the_by_hand_runs: "none"
  files_removed_in_any_step: "none"
  hook_after_setup: "where setup exits 0 over the 106-byte own hook: 5218 bytes, mode 755, two marker lines, the last 96 bytes equal the own hook's; where setup exits 1 (L2, L3): unchanged"
  rename_in_main_checkout: { commit_exit: 1, stderr_contains: "out-of-band managed-doc rename staged in this commit", commits_after: 7, porcelain: "R  VISION.md -> moved-VISION.md" }
  rename_in_linked_worktree: { commit_exit: 0, stdout: "", stderr: "" }    # landed unrefused — Left open
observed: "<W>/c1-<layout>.log and <W>/c1-<layout>.<suffix>/ev/ — per step <name>.out and <name>.err, the traces <name>.commitA.trace and <name>.validate.trace, the recorded hook environment <name>.hookenv.A, and the manifests man.<step>; the drivers are <W>/env.sh, <W>/drive.sh, <W>/all.sh"
pinned-by: "UNPINNED: no suite found that commits in a second checkout or a second repository after a `setup` in the first; the near-miss is precommit_hook_acceptance::commit_is_silent_on_unconfigured_repo, which pins not-a-project → silent in ONE repository"
```

**Pinnable as it stands:** yes, with two things said. Every step is argv or a file write, and
the expectations are exits, exact strings and file lists; the one machine-dependent value, the
binary's absolute path inside the hook, is in no assertion, and the `sh -x` step is a tracing
aid a test can drop. (1) The `GIT_*` shape git hands a hook is this git's (2.54.0); a test
should let the real commit run the hook, as commits A and B do, and not rebuild that
environment. (2) `L2`, `L3`, `L3b`, `L3c` and `L5` pin what the candidate does in layouts row
(D) holds open, and `rename_in_linked_worktree` pins a gap: if any of them is ruled or fixed,
those cells change and the block changes with them. `L1`, `L1r`'s first two cells, `L4`,
`L4j`, `L4g` and `L6` pin behaviour the design states.

## The class

**Eleven layouts driven; not a derived bound.** The enumeration is by hand, from the three
ways git moves or shares a hooks directory (a common git dir behind several checkouts; a
relocated git dir; a hooks path or link the user's configuration shares), crossed with what
stands at `dirname(git-common-dir)`, which is where jigc asks. It is not read off a registry,
so a layout I did not think of is not in it. The mechanism has four call sites in
`crates/cli/src/setup.rs` by `command grep -n 'resolve_hooks_dir('` — the install, the
teardown's removal, the teardown's sentence about a hook it kept, and the link refusal — and
**I drove one of them, the install**; `jigc uninstall` in any of these layouts was not driven.
Not driven either: a `core.hooksPath` that points into another repository's work tree or into
a submodule; a relative, committed in-tree `core.hooksPath` reaching a clone; a per-worktree
`core.hooksPath` under `extensions.worktreeConfig`; a linked worktree of a
`--separate-git-dir` repository; a user-exported `GIT_DIR` (a declared bound of
`crates/cli/src/repo.rs`); the submodule (the earlier report's); another git version or
platform; the previous release.

## Left open — noticed, not pursued

- **`L5`: `jigc setup` at exit 0 commits into a repository it was not typed in.** Typed in a
  `--separate-git-dir` checkout whose git dir sits inside another repository's work tree, it
  resolves its home to the git dir's parent, writes nine install files there, and makes the
  commit `chore(jigc): install jigc workspace config` — those nine files, under `gitdirs/` —
  **in the enclosing repository** (1 commit before, 2 after), rewrites that repository's
  `pre-commit`, and leaves the checkout it was typed in with its one commit and no install.
  The ack names the git dir's parent *the main checkout this repository's jigc install …
  bind[s] to*. The block is `L5`'s `setup` list above; `git -C l5/enc log --format=%s
  --name-only` after it shows the commit. This is content committed in a repository the user
  did not run the command in, by the `setup` door itself — the mechanism row (D) and the
  earlier submodule finding name (`home = dirname(git-common-dir)`), reaching a third layout.
  I did not drive it on the previous release, did not look at how ordinary the layout is, and
  grade nothing of it.
- **`L1r`: a bare `git mv` of a managed doc committed in a linked worktree lands unrefused.**
  The same commit in the main checkout is blocked. The block's sweep reads the main checkout,
  where the doc still stands, so it reports no `reconciliation.rename` and the M35 block does
  not fire for a rename staged in a linked worktree. No byte is lost; what is open is whether
  the backstop is meant to see it.
- **Git exports `GIT_DIR` to the hook in every linked-worktree and `--separate-git-dir`
  commit**, and the block's `validate` then starts git children aimed at the home. What those
  children read under that variable — the worktree's index and `HEAD`, or the home's — was not
  examined. The module doc of `crates/cli/src/repo.rs` declares a *user-exported* `GIT_DIR`
  out of scope; this one is git's own.
- **`L2`, `L3`: `setup` exits 1 after writing eight install files into the git dir's parent**,
  and its route — *ensure the repo's git hooks directory is writable, then re-run* — names a
  directory that is writable (the own hook was written there); the directory git was asked
  about is another. Row (D) records the exit and the writes as pre-existing. I did not re-run
  `setup` as the route says.
- **`L3b`, `L3c`: `setup` exits 0 and installs at a directory that is no work tree**, with no
  install commit, while the ack calls it *the main checkout* and says the hook is *local to
  this checkout*. Row (D) territory.
- **Two installs, one hook file (`L4j`, `L6`, `L1`).** The block holds one `jigc='…'` line,
  regenerated by whichever `setup` ran last: with two different builds in two repositories
  that share a hooks directory, each `setup` repoints the other's backstop; and a
  `jigc uninstall` typed in one would be removing the block the other relies on. Neither was
  driven — both `setup`s here used one binary.
- **The ack's hook line says "local to this checkout"** of a file in a directory other
  repositories or checkouts share (`L1`, `L4`, `L4g`, `L5`, `L6`). A sentence of the surface.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/`. No commit, no stage, no
build. Under the scratch root I wrote this report, and inside `<W>` three driver scripts, their
logs and the roots they minted.

<!-- end of report -->
