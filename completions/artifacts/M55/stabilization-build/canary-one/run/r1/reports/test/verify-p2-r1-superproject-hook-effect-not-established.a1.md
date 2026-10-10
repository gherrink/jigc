# verify-real — `r1-superproject-hook-effect-not-established` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup`. Clause it is said to break:
`no-lost-files`. Triage's grade: *unclear*. The finding carries no block of its own: it is the
row of an earlier verifier's report that says the effect of the rewritten superproject hook was
*not established*. Triage asked for that effect to be driven, on both binaries.

## Verdict in one paragraph

**`refuted` as a blocker — basis `breaks-no-clause`. `contested: false`.** The effect is now
established, and it is nil. After `jigc setup` typed in a submodule, every commit in the
superproject does run jigc's managed block first, and the block does run
`jigc validate --format json` — at the top of the **superproject's** work tree, a directory
that holds no jigc install. There `validate` exits 1 with *this project isn't set up* on stderr
and nothing on stdout; the hook throws stderr away, so its `report` is the empty string, both of
its tests are false, it prints nothing, exits nowhere, and falls through to the user's own hook
body. A commit that adds a tracked file and a commit of a staged `git mv` of that file both land
at exit 0, with exactly the staged content, and the only bytes that change anywhere under the
root are the five files git itself rewrites on a commit. The previous release does the same,
cell for cell. So half of the relayed claim holds as a fact — *the superproject's commits now
run `jigc validate` first* — and nothing of it reaches the first clause: no byte is destroyed, no
content is committed that the user did not ask for, no commit is blocked or changed.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

For every candidate block the candidate's directory was first on `PATH` and `command -v jigc`
printed its path, checked at the top of the block (a mismatch would have stopped it). For the
previous release's block the binary was called by its absolute path, its own directory first on
`PATH` for that block only. Nothing was built, and nothing under `target/` was driven.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-superhook.p7ZSBn`. Two fresh roots inside it, minted the same way, one per
  binary: `sub-cand.wcu2xu` and `sub-prev.7KQx0V`. Neither is a reporter's nor another
  verifier's. `<R>` below is the root of the binary in question.
- Environment of both roots: `HOME` a fresh empty directory inside the root,
  `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null`, the four `GIT_AUTHOR_*` /
  `GIT_COMMITTER_*` variables set to a synthetic identity. Git 2.54.0 (Apple Git-157), macOS.
- Every exit status was read bare — `cmd > out 2> err; echo rc=$?` — never through a pipe.
- A manifest (sha256 of every file and link under the root, git object files and my own
  evidence directory left out) was taken before and after `setup`, before and after each
  commit, and before and after the by-hand runs.
- No rig: the layout is `Repro VR-1`'s variant, built with six git steps, and no state of
  `dev/jigc-rig` builds a submodule.
- **What I added to the block as written.** `Repro VR-1`'s variant ends at an empty commit in
  the superproject. Triage asked for a staged `git mv` of a tracked file, so after `setup` I
  created `docs/a.md` in the superproject and committed it (commit A — itself a commit through
  the hook), then `git mv docs/a.md docs/b.md` and committed that (commit B). The superproject's
  own hook is the same 41 bytes: `#!/bin/sh`, a newline, `echo superproject-own-hook-ran`, a
  newline.
- **How the block was traced, three ways.** (1) Both commits ran under `GIT_TRACE` and
  `GIT_TRACE_SETUP` pointed at one file. (2) Between staging the move and committing it, the
  installed hook file — unedited — was run by hand as `sh -x .git/hooks/pre-commit` from the
  superproject's top, under the environment git's trace shows it gives the hook
  (`GIT_EDITOR=:`, `GIT_INDEX_FILE=.git/index`). (3) The one jigc command the block holds was
  typed by hand in the same directory under the same environment, its stdout and stderr kept
  apart, where the hook sends stderr to the null device.

## What was observed — candidate (`<W>/sub-cand.wcu2xu`)

| step | cwd | exit | what |
|---|---|---|---|
| the six git steps of the layout, the 41-byte hook written | `<R>` | 0 each | superproject 2 commits, submodule 1 |
| `jigc setup` | `super/vendor/lib` | **0** | stderr empty; hook 41 → 5153 bytes, mode 755; eight files created under `super/.git/modules/vendor/`; both porcelains empty — the earlier verifier's cells, again |
| `git add docs/a.md`, `git commit -q -m …` (commit A) | `super` | **0** | stdout empty; stderr the one line `superproject-own-hook-ran`; the commit landed |
| `git mv docs/a.md docs/b.md` | `super` | 0 | `git diff --cached --name-status --find-renames` prints `R100 docs/a.md docs/b.md` |
| `jigc validate --format json`, by hand, the hook's environment | `super` | **1** | stdout 0 bytes; stderr below |
| `sh -x .git/hooks/pre-commit`, by hand, the hook's environment | `super` | **0** | stdout `superproject-own-hook-ran`; the trace below |
| `git commit -q -m …` (commit B, the staged move) | `super` | **0** | stdout empty; stderr the one line `superproject-own-hook-ran`; the commit landed |

`validate`'s stderr, typed in the superproject:

```text
{
  "error": "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
}
```

The hook under `sh -x`, whole (the binary's path shortened to its placeholder):

```text
+ jigc=<scratch>/bin/c1.a1/jigc
++ <scratch>/bin/c1.a1/jigc validate --format json
+ report=
+ printf %s ''
+ tr -d '\n'
+ grep -Eq '"blocking_probes"[[:space:]]*:[[:space:]]*\[[^]]*"doc-code"'
++ printf %s ''
++ grep -o 'git -C [^`]*'
+ moves=
+ '[' -n '' ']'
+ echo superproject-own-hook-ran
```

Read against the hook's text (`crates/cli/src/setup.rs`, the managed block and the rename
block): the warning is printed only when `report` names `doc-code` in `blocking_probes`; the
rename arm — the one place the block holds an `exit 1` — is entered only when `report` holds a
`git -C …` span. With `report` empty neither is reached, the `git diff --cached` inside the
rename arm is never run, and control passes the end marker into the user's line.

Git's own trace of both commits agrees: one `run_command: GIT_EDITOR=: GIT_INDEX_FILE=.git/index
.git/hooks/pre-commit`, then git's own `maintenance run --auto` — and **no git process in
between**. The hook's `jigc` started no git at all: typed by hand in the superproject under
`GIT_TRACE`, `validate` leaves no trace file. Typed in the submodule it does, and its git
children run at `<R>/super/.git/modules/vendor`.

**Which home.** So the block's `validate` asks about the superproject's own top, `<R>/super` —
the directory git puts a hook in — which has a `.git` directory and no `.jigc/config/`, and it
answers *not set up* without asking git anything. It never reaches the install `setup` made,
which sits at `super/.git/modules/vendor/`. For the record of the other direction:
`jigc validate --format json` typed in `super/vendor/lib` exits 0 with
`"blocking_probes": []` and `"findings": []`.

**What changed on disk.**

- Across commit A, and across commit B: `super/.git/COMMIT_EDITMSG`, `super/.git/index`,
  `super/.git/logs/HEAD`, `super/.git/logs/refs/heads/master`, `super/.git/refs/heads/master`,
  and object files. Nothing else under `<R>` — nothing in the superproject's work tree, nothing
  under `super/.git/modules/`, nothing in `HOME`, which is still empty.
- Across the two by-hand runs (`validate`, then the hook under `sh -x`): the manifests are
  identical (`diff` exit 0). The block's `validate` writes no file.
- After commit B: the superproject has 4 commits; `git show --stat -M HEAD` is
  `docs/{a.md => b.md} | 0`; `git ls-tree -r --name-only HEAD` is `.gitmodules`, `docs/b.md`,
  `vendor/lib`; `docs/b.md` holds the bytes written to `docs/a.md`; porcelain (untracked and
  ignored included) is empty in the superproject and in the submodule; the submodule still has
  its 1 commit and lists `.git` alone.
- The user's hook body: the last 31 bytes of the rewritten hook equal the last 31 of the
  original (`cmp` exit 0), the file opens with the same `#!/bin/sh` line, and it holds one end
  marker.

## What was observed — previous release (`<W>/sub-prev.7KQx0V`)

The same block, step for step, the binary called by its absolute path. `setup` exit **0**,
stderr empty; the hook 41 → 5169 bytes — sixteen more than the candidate's, the difference in
length between the two binaries' directory names, which the hook embeds — with one end marker
and the user's 31 bytes after it (`cmp` exit 0); the same eight files created under
`super/.git/modules/vendor/`. Commit A exit **0**, commit B exit **0**, each with stdout empty
and stderr the one line `superproject-own-hook-ran`. `validate` by hand in the superproject:
exit **1**, stdout 0 bytes, the same one-key error. The hook under `sh -x`: exit **0**, the same
eleven trace lines but for the binary's path. The same five git files changed across each
commit and nothing else; the by-hand manifests identical; `HEAD` holds `.gitmodules`,
`docs/b.md`, `vendor/lib`; both porcelains empty; the trace of both commits shows no git process
between the hook's start and git's own maintenance.

## The questions triage asked, one by one

| question | answer, on both binaries |
|---|---|
| does the managed block run `jigc validate` on a superproject commit | **yes** — `validate --format json`, by the absolute path of the binary that ran `setup`, once per commit |
| against which home | the superproject's own work tree top, which holds no install; never the submodule's install at `super/.git/modules/vendor/` |
| with what exit | **1**, *this project isn't set up*, stdout empty; the hook reads stdout only and never the exit |
| can it block a superproject commit | **not in this layout** — the block's one `exit 1` sits behind a `report` that holds a `git -C …` span, and `report` is empty; the commit of a staged `git mv` landed at exit 0 |
| can it change a superproject commit | **no** — the block holds no writing command, its `validate` wrote no file, and `HEAD` after commit B is the staged tree |
| what does it print | **nothing** — the commit's whole stderr is the user's own hook's line |

## The finding's claim against the design that owns the behaviour

`design/assistant-adapter.md` → *The doc↔code backstop (M19)*: the hook "prints nothing and
exits 0" on five cases, of which the third is "not-a-jigc-project" — "an unconfigured or
probe-less repo never breaks a commit". That is the case a superproject is in, and what was
observed is that sentence. The one arm that gates (`design/validation.md` → *The M19 pre-commit
backstop stays doc↔code-keyed for content; M35 adds a hard block on rename*) keys on a
`reconciliation.rename` finding of the sweep, and a sweep that answers *not set up* has none.
The install discipline's *non-destructive (preserves/wraps any existing `pre-commit` hook)*
holds for the user's bytes. No sentence of that section describes a hook that lands in another
repository's hooks directory; that is the earlier finding's subject
(`r1-setup-installs-outside-work-tree`) and the support question of
`implementation/decisions-pending.md` → the M57 list, row (D), and I grade neither here.

## Does it break the clause, inside its scope

The clause, as the run's opening names it and as `DECISIONS.md` → *2026-10-04 — The exit rule,
revised* sharpens it: "no jigc command at exit 0 destroys bytes no git object holds or commits
content the user did not ask for".

- **Bytes destroyed by the hook's effect: none.** Two commits and two by-hand runs of the block
  changed no file but git's own five. The user's hook body survives the rewrite byte for byte.
- **Content committed that the user did not ask for: none.** Commit A holds the one added file,
  commit B the one staged rename; the block stages nothing and its `validate` writes nothing.
- **A commit blocked or altered: none.** Both exit 0; the user's own hook still runs, after the
  block.

Whether a submodule is a layout inside the clause's scope is not settled by anything I can cite
— row (D) holds it open, the human's — and **the verdict does not turn on it**: read with the
layout inside the scope, the clause is still not broken.

**Why `breaks-no-clause` and not `does-not-reproduce`.** The relayed observation reproduces: the
superproject's commits do run `jigc validate` first, from a block `setup` typed elsewhere put
there. That is real, it costs each superproject commit one process that answers *not set up*,
and it stays a row of the ledger. What does not follow from it is a loss or a wrong write.

**Why not `contested`.** The finding does not argue that a settled decision is wrong, and my
verdict needs no ruling: it holds whichever way row (D) is answered.

## The previous release

The block is green on the previous release in the same way it is green on the candidate. With a
`refuted` verdict there is no regression fact to give; the run is recorded because triage asked
for both binaries.

## Coverage

The finding makes no coverage claim. For the block's `pinned-by` I looked, and only that far:
`command grep -rl submodule` over `crates/cli/tests`, `tooling-tests` and `crates/engine` names
eleven files, of which three also name `pre-commit` — `setup_install_pathspec_guard.rs`,
`setup.rs`, `crates/engine/src/finalize.rs`. Read at their `submodule` lines: the first drives
`setup` inside a submodule and asserts its exit and a clean porcelain; the second builds a
`core.hooksPath` inside a submodule, another shape; the third is a doc comment. None commits in
a superproject after a `setup` typed in its submodule. A suite that builds the layout without
the word would have been missed.

## Repro VR-2 — after `jigc setup` in a submodule, the superproject's hook runs `jigc validate` and it neither blocks, changes nor prints

```yaml
claim: "after `jigc setup` typed in a submodule, the managed block in the superproject's pre-commit hook can block or change a superproject commit, or lose bytes there"
verdict: REFUTED
basis: breaks-no-clause
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exits, outputs and writes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d); the hook is 5169 bytes there, 5153 here — the embedded binary path's length"
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, GIT_AUTHOR_NAME/EMAIL and GIT_COMMITTER_NAME/EMAIL set"
  - ["git", "init", "-q", "lib"]
  - ["git", "-C", "lib", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["git", "init", "-q", "super"]
  - ["git", "-C", "super", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["git", "-C", "super", "-c", "protocol.file.allow=always", "submodule", "add", "-q", "<abs>/lib", "vendor/lib"]
  - ["git", "-C", "super", "commit", "-q", "-m", "add submodule"]
  - write: "super/.git/hooks/pre-commit = '#!/bin/sh\necho superproject-own-hook-ran\n', mode 755"   # 41 bytes
  - cwd: super/vendor/lib
  - ["jigc", "setup"]                                    # exit 0
  - cwd: super
  - write: "docs/a.md = '# a tracked file of the superproject\n'"
  - ["git", "add", "docs/a.md"]
repro:
  - ["git", "commit", "-q", "-m", "add docs/a.md"]       # commit A; cwd super
  - ["git", "mv", "docs/a.md", "docs/b.md"]
  - ["jigc", "validate", "--format", "json"]             # the block's one jigc command, by hand; cwd super; env GIT_EDITOR=: GIT_INDEX_FILE=.git/index
  - ["sh", "-x", ".git/hooks/pre-commit"]                # the installed hook, unedited; same cwd and env
  - ["git", "commit", "-q", "-m", "move docs/a.md to docs/b.md"]   # commit B; cwd super
expect:
  commit_A: { exit: 0, stdout: "", stderr: "superproject-own-hook-ran\n" }
  validate_by_hand: { exit: 1, stdout: "", stderr_contains: "this project isn't set up" }
  hook_under_sh_x: { exit: 0, stdout: "superproject-own-hook-ran\n", trace_contains: ["+ report=", "+ moves="], trace_lacks: ["git diff --cached", "exit 1"] }
  commit_B: { exit: 0, stdout: "", stderr: "superproject-own-hook-ran\n" }
  head_after_B: "`git ls-tree -r --name-only HEAD` = .gitmodules, docs/b.md, vendor/lib; `git show --stat -M HEAD` = docs/{a.md => b.md}"
  files_changed_across_each_commit: "super/.git/COMMIT_EDITMSG, index, logs/HEAD, logs/refs/heads/master, refs/heads/master, and object files — no other path under the root, HOME included"
  files_changed_across_the_two_by_hand_runs: "none"
  porcelain: "`git status --porcelain --untracked-files=all --ignored` empty in super and in super/vendor/lib"
  user_hook_body: "the last 31 bytes of the rewritten hook equal the original's"
observed: "<W>/sub-cand.wcu2xu and <W>/sub-prev.7KQx0V — each with ev/setup1.out, ev/setup1.err, ev/hook.before, ev/commitA.{out,err,trace}, ev/commitB.{out,err,trace}, ev/validate-super.{out,err}, ev/shx.{out,err}, and the manifests ev/man.before, man.after-setup, man.before-commitA, man.after-commitA, man.before-byhand, man.after-byhand, man.before-commitB, man.after-commitB"
pinned-by: "UNPINNED: no suite found that commits in a superproject after a `setup` typed in its submodule (searched by the word `submodule` beside `pre-commit`; three files, none of this shape)"
```

**Pinnable as it stands:** yes. Every step is argv or a file write, the expectations are exits,
exact strings and file lists, and nothing in them depends on the machine: the one
machine-dependent value, the binary's absolute path inside the hook, appears in no assertion.
The `sh -x` step is a tracing aid; a test can drop it and keep the two commits, `validate`'s
exit, and the before-and-after file lists. It pins what both binaries do today — if row (D) is
ever ruled so that `setup` in a submodule installs elsewhere or refuses, the setup step's
`exit 0` changes and this block changes with it.

## The class

`instance, unbounded`. I drove one door's after-effect — the hook `jigc setup` rewrites — in one
layout, a submodule whose superproject is **not** itself a jigc project, with two commit shapes,
on two binaries. I did not enumerate the layouts in which a hook lands in a repository other
than the one `setup` was typed in, and derived no count. The mechanism that made the effect nil
is one fact — the directory git runs the hook in holds no install — and where that fact does not
hold, nothing here speaks.

## Left open — noticed, not pursued

- **A superproject that is itself a jigc project was not driven.** The two installs would share
  one hook file, with one block holding one `jigc='…'` line, regenerated by whichever `setup`
  ran last; and a `jigc uninstall` typed in the submodule would be removing a block of the
  superproject's hook. Whether either leaves the superproject without its own backstop, or with
  a block that names the other install's binary, is not established.
- **The submodule's own commits run no backstop.** `super/.git/modules/vendor/lib/hooks` holds
  no `pre-commit` after `setup` on either binary, while the ack lists a "pre-commit hook … (warn-only
  doc↔code drift backstop)" as installed. The hook it names is the superproject's, whose
  `validate` never reaches the submodule's install — so neither the drift warning nor the
  out-of-band rename block can fire for the repository `setup` was typed in. Not driven beyond
  listing the directory.
- **The ack's hook line says "local to this checkout"** of a hook that sits in another
  repository's hooks directory. A sentence of the surface, not graded here.
- **`jigc validate` typed in the submodule** exits 0 with no findings, its git children running
  at `super/.git/modules/vendor`, a directory that is no work tree. Row (D) already records that
  store reads are taken at the wrong place in this layout; I did not look at what that sweep can
  and cannot see.
- **The submodule has one commit after `setup`** and an empty porcelain: the install is in no
  commit and in no work tree. Row (D) territory, noted because it was in front of me.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. The one file I wrote is this report.

<!-- end of report -->
