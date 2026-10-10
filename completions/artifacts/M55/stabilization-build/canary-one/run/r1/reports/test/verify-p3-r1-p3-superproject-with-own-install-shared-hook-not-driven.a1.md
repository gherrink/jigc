# verify-real — `r1-p3-superproject-with-own-install-shared-hook-not-driven` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup` and `jigc uninstall`. Clause it is said
to break: `working-product`. Triage's grade: *unclear*. The finding carries no block of its own:
it is the first bullet under *Left open — noticed, not pursued* of an earlier verifier's report,
which says that a superproject that is itself a jigc project was not driven. Triage asked for
that layout to be driven on both binaries: the hook's one block and its `jigc=` line after
`setup` in the submodule, then `jigc uninstall` typed in the submodule, and whether the
superproject keeps a block that reaches its own install.

## Verdict in one paragraph

**`refuted` as a blocker — basis `breaks-no-clause`. `contested: false`.** Three things were
established, on both binaries, cell for cell. (1) `jigc setup` typed in the submodule leaves the
superproject's hook with **one** block and **one** `jigc=` line; where one binary made both
installs the hook is byte-identical before and after, and the block still reaches the
superproject's install — a commit that stages an out-of-band rename of the superproject's
managed doc is blocked at exit 1 before and after. (2) `jigc uninstall` **typed as the finding
says**, in the submodule, **refuses at exit 1** and changes no file: the superproject keeps its
block, and the same commit is still blocked. So the harm the finding supposes does not follow
from the command it names. (3) The refusal prints one route that does anything in this layout,
`jigc uninstall --force`; run as printed in the submodule it exits 0, removes the submodule's
install **and the block in the superproject's hook**, and the superproject — its own install
untouched and still standing — has no backstop: the commit that was blocked three times now
lands at exit 0, and neither `jigc validate` nor `jigc upgrade` typed in the superproject says
a word about the missing block. `jigc setup` typed in the superproject puts the block back,
byte for byte. That third fact is a real defect and it stays a row of the ledger. It does not
break the second clause inside its scope: the previous release does exactly the same at every
step, so no command that works there stops working, and the printed route does what it prints —
its ack lists *removed pre-commit hook* among what it took.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

For every candidate block the candidate's directory was first on `PATH` and `command -v jigc`
printed its path, checked at the top of the block (a mismatch stops the script). For the
previous release's block that binary's own directory was first on `PATH` for that block only,
under the same check. Nothing was built, and nothing under `target/` was driven.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-p3-shared-hook.7avT62`. Four fresh roots inside it, each minted the same
  way: `cand.4BNCxr` (candidate, the block as triage asked for it), `cand2.uj16iA` (candidate,
  the same block plus the refusal's printed route), `prev.L4xAlU` (previous release, the same
  block as `cand2`), `mixed.NDWGRN` (the one two-binary cell). None is a reporter's or another
  verifier's. `<R>` below is the root in question.
- Environment of every root: `HOME` a fresh empty directory inside the root,
  `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null`, the four `GIT_AUTHOR_*` /
  `GIT_COMMITTER_*` variables set to a synthetic identity, `JIGC_PACK_DIR` unset. Git 2.54.0
  (Apple Git-157), macOS.
- Every exit status was read bare — `( cd <cwd> && exec <argv> ) > out 2> err; rc=$?` — never
  through a pipe; stdout and stderr were kept apart.
- A manifest (sha256 of every file and link under the root, git object files and my own
  evidence directory left out) was taken before and after each `setup` and each `uninstall`.
- No rig: no state of `dev/jigc-rig` builds a submodule. The layout is `Repro VR-2`'s, built
  with its six git steps and its 41-byte hook. The superproject's one managed doc was made with
  the same commands the rig's `committed-singletons` state prints for its vision doc (read from
  `dev/jigc-rig --print-only`), typed by hand.
- **What I changed against `Repro VR-2`, and why.** (a) `jigc setup` is typed in the
  superproject **first** — what triage asked for. (b) After it the superproject gets one managed
  doc, `VISION.md`, created and finalized through jigc. `Repro VR-2`'s commit shapes prove
  nothing here: in a superproject with a clean store the block is silent whether or not it
  reaches the install. A managed doc gives the block something only the superproject's install
  knows — the store's record of `VISION.md` — so **the control** below tells the two apart.
  (c) The steps triage named: `jigc setup` in the submodule, then `jigc uninstall` in the
  submodule. (d) The refusal that `uninstall` printed names `jigc uninstall --force`; a route
  the binary prints is run as printed, so it was — in a fresh root, after the same steps.
  (e) `jigc setup` in the superproject once more, to see whether the block comes back.
- **The control — the same two commands, five times in a root.** In the superproject:
  `git mv VISION.md VISION2.md`, then `git commit -q -m …`. The hook's rename arm exits 1 with
  *commit blocked* only when its `jigc validate` reaches an install whose store records
  `VISION.md` — the superproject's. After a blocked commit `git mv VISION2.md VISION.md` put
  the index back (porcelain empty, checked each time); after a landed one the move was undone
  by a second commit.

## What was observed — candidate (`<W>/cand2.uj16iA`; the first seven rows also in `<W>/cand.4BNCxr`, identical)

| # | step | cwd | exit | what |
|---|---|---|---|---|
| 0 | the six git steps, the 41-byte hook written | `<R>` | 0 each | superproject 2 commits, submodule 1 |
| 1 | `jigc setup` | `super` | **0** | stderr empty; hook 41 → 5153 bytes, **one** start sentinel, **one** end marker, **one** `jigc=` line (line 9, the candidate's path); the user's line after the end marker; install commit made; porcelain empty |
| 2 | the managed doc: `start --workflow form-vision`, `doc create vision`, three `doc set-slot`, the commit doc's two fields and two slots, `task finalize` | `super` | 0 each | `finalized … docs(vision): form the project vision`, `promoted VISION.md`; hook output relayed: `superproject-own-hook-ran`; superproject 4 commits |
| 3 | **control 0** — `git mv`, `git commit` | `super` | mv 0, commit **1** | stderr the one line `jigc: out-of-band managed-doc rename staged in this commit — … (commit blocked).`; `HEAD` unmoved |
| 4 | `jigc setup` | `super/vendor/lib` | **0** | stderr empty; ack: `pre-commit hook → <R>/super/.git/hooks/pre-commit`, `installed at <R>/super/.git/modules/vendor`; eight files created under `super/.git/modules/vendor/` and nothing else changed — **the hook is byte-identical to row 1's (`cmp` exit 0)**; both porcelains empty; the submodule's own hooks directory holds git's samples only |
| 5 | **control 1** | `super` | commit **1** | *commit blocked*, the same line |
| 6 | **`jigc uninstall`** | `super/vendor/lib` | **1** | stdout empty; stderr the refusal below; **the manifests before and after are identical** — no file changed; hook still row 1's bytes |
| 7 | **control 2** | `super` | commit **1** | *commit blocked*, the same line — the superproject keeps a block that reaches its own install |
| 8 | **`jigc uninstall --force`** — the refusal's route, as printed | `super/vendor/lib` | **0** | stderr empty; the ack below; the eight files under `super/.git/modules/vendor/` gone but for `.claude/settings.json`, left with empty arrays; **`super/.git/hooks/pre-commit` back to 41 bytes, equal to the user's original (`cmp` exit 0) — no sentinel, no `jigc=` line**; nothing in the superproject's work tree changed, `super/.jigc/config/` and `super/CLAUDE.md` standing; both porcelains empty |
| 9 | `jigc validate --format json` | `super` | 0 | `"blocking_probes": []`, `"findings": []` |
| 10 | `jigc upgrade` | `super` | 0 | `no findings — …` |
| 11 | **control 4** | `super` | commit **0** | stderr the one line `superproject-own-hook-ran`; **the out-of-band rename landed**: `git show --stat -M` of it is `VISION.md => VISION2.md` |
| 12 | `jigc setup` (after the move was undone by a commit) | `super` | **0** | hook 5153 bytes again, **byte-identical to row 1's (`cmp` exit 0)**; porcelain empty |
| 13 | **control 3** | `super` | commit **1** | *commit blocked* again |

Row 6's stderr, whole:

```text
blocking · uninstall.untracked-workbench-file — cannot check `.jigc/` for files no index has a copy of, so removing it could destroy them: `git --no-optional-locks status --porcelain --untracked-files=no --no-renames -z -- .jigc` failed: fatal: this operation must be run in a work tree
  route: make sure `git` is on PATH and the `.jigc/` tree is readable, then re-run `jigc uninstall` — or, once you have confirmed it holds nothing you need, `jigc uninstall --force`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

Row 8's stdout, whole:

```text
jigc uninstall — repo-local install removed

  - removed .jigc/
  - unwired bootstrap reference ← CLAUDE.md
  - removed jigc allowlist permit ← .claude/settings.json
  - removed SessionStart hook ← .claude/settings.json
  - removed deny safety floor ← .claude/settings.json
  - removed pre-commit hook
  - removed jigc guide artifact
  removed at `<R>/super/.git/modules/vendor` — the main checkout this repository's jigc install and `.jigc/` workbench bind to, not the worktree you are standing in
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

**Why the two installs share one hook file.** Asked in the driven root:
`git -C super/vendor/lib rev-parse --git-common-dir` answers `<R>/super/.git/modules/vendor/lib`;
the home both verbs name in their ack is that directory's parent, `<R>/super/.git/modules/vendor`,
which is no work tree; and `git -C super/.git/modules/vendor rev-parse --git-path hooks` answers
`<R>/super/.git/hooks` — the superproject's — while the submodule's own hooks directory is
`<R>/super/.git/modules/vendor/lib/hooks`. So `setup` and `uninstall` typed in the submodule
both act on the superproject's hook file, which carries one block between jigc's two markers,
and that block is the superproject's as much as the submodule's.

## What was observed — previous release (`<W>/prev.L4xAlU`)

The same block, step for step. **Every exit in the table above is the same**: `setup` 0 in the
superproject and 0 in the submodule; controls 0, 1 and 2 blocked at exit 1 with the same line;
`jigc uninstall` in the submodule exit **1** under the same code, no file changed;
`jigc uninstall --force` exit **0** with the same seven-line ack; the hook back to the user's 41
bytes (`cmp` exit 0); `validate` and `upgrade` in the superproject silent; **control 4 landed at
exit 0**; `setup` in the superproject restored the hook byte for byte and control 3 blocked
again. Three differences, none of them a cell of the finding: the hook is 5169 bytes, sixteen
more than the candidate's — the length of the embedded binary path; the superproject's install
commit holds no `.jigc/settings-entries.json`, which the candidate added; and the refusal quotes
git's argv in another order (`status --porcelain -z --no-renames -- .jigc`).

## The two-binary cell (`<W>/mixed.NDWGRN`)

Asked because triage named the `jigc=` line, and with one binary it cannot differ. The
superproject set up, and its `VISION.md` made, by the **previous release**; then `jigc setup`
typed in the submodule with the **candidate**. Exit 0 each. The hook before and after differs in
exactly one line (`diff` prints one change, line 9):

```text
< jigc='<scratch>/bin/previous-91834b5e011d/jigc'
> jigc='<scratch>/bin/c1.a1/jigc'
```

Still one block, one end marker, one `jigc=` line. The control in the superproject is blocked at
exit 1 with the same line: the block now runs the binary that made the *submodule's* install,
at the superproject's top, and reaches the superproject's install. The other order was not
driven.

## The questions triage asked, one by one

| question | answer, on both binaries |
|---|---|
| how many blocks and `jigc=` lines after `setup` in the submodule | **one and one** — the block is regenerated in place, never doubled |
| what does the `jigc=` line name | the binary that ran `setup` **last**; with one binary on the machine the hook is byte-identical before and after |
| does the superproject keep a backstop after `setup` in the submodule | **yes** — the control is blocked before and after |
| `jigc uninstall` typed in the submodule | **refuses, exit 1**, `uninstall.untracked-workbench-file`; no file changes |
| does the superproject keep a block that reaches its own install | **yes after the command as typed** (control blocked); **no after the refusal's printed route `jigc uninstall --force`** — the block is cut, the user's 41 bytes restored, the control lands at exit 0 |
| does anything say so | the ack of the `--force` run lists `removed pre-commit hook`, with no path; nothing typed in the superproject afterwards (`validate`, `upgrade`) names it |
| is there a way back | **yes** — `jigc setup` in the superproject, exit 0, the same bytes as before |

## The finding's claim against the design that owns the behaviour

`design/assistant-adapter.md` → *Where it installs: jigc_home, from every cwd (M53)*: the
install is one per repository, and "the `pre-commit` hook lives in the repository-wide common
hooks dir whichever checkout installed it"; `setup` and `uninstall` "resolve their subject
through **jigc_home**". The same doc → *Install discipline*: the block is "**regenerated on
every `setup`**", and its declared bound (ii) says the body "embeds the **installing machine's
absolute `jigc` path**". `design/project-setup.md` → *Teardown / cleanup (G5)*: "`uninstall`
removes exactly the lines between jigc's two markers — whatever version wrote them, whatever
stands around them", and of `--force`: "the operator's explicit consent to delete".

Read against those: the one block and the last-`setup`-wins `jigc=` line are the design, as
written. The refusal at row 6 is the teardown's guard (c) failing closed where git cannot
answer. The cut at row 8 is the teardown removing the block of the hooks directory its home
resolves to. **What no sentence of either section describes is two installs behind one hooks
directory** — both assume the hook file belongs to one install. That state exists only because
the home of a submodule resolves to a directory inside the superproject's git directory, which
is the standing support question of `implementation/decisions-pending.md` → the M57 list, row
(D): "whether these three layouts are supported — and if they are not, a refusal that says so at
`setup`, rather than doors that half-work." So the behaviour is neither intended by a settled
decision nor argued against one: it is a consequence of a layout whose support is the human's
open call. I grade neither row (D) nor the earlier finding about where `setup` installs.

## Does it break the clause, inside its scope

The clause, as the run's opening names it — "a working product others can rely on", instrument
"the regression set, and here the gate" — and as `DECISIONS.md` → *2026-10-04 — The exit rule,
revised* sharpens it: "no command that works on rc.24 in a supported layout stops working, and
every refusal's route works as printed".

- **A command that works on the previous release and stops working on the candidate: none.**
  Every exit of the block is the same on both binaries, the refusal included, and the files
  each step writes and removes are the same but for the candidate's own settings record.
- **The refusal's route, as printed.** `jigc uninstall --force` exits 0 and removes what its
  ack lists, the hook's block among them. It works as printed. (The route's other half is a
  separate matter and is under *Left open*.)
- **What is real and is not a break of this clause.** After the `--force` run the superproject
  is a standing jigc install with no `pre-commit` block, and nothing it is asked afterwards
  says so. The out-of-band rename the block exists to stop lands at exit 0. This is reached
  only through the consent flag, in a layout row (D) holds open, and it is the same on the
  previous release.

Whether a submodule under a superproject that is itself a jigc project is a *supported layout*
is not settled by anything I can cite, and **the verdict does not turn on it**: with the layout
outside the scope the clause does not reach the finding, and with it inside, neither half of the
instrument is broken.

**Why `breaks-no-clause` and not `does-not-reproduce`.** Half of what the finding supposes does
not reproduce — `setup` in the submodule costs the superproject nothing, and `uninstall` as
typed refuses. The other half does: one printed command later the superproject has lost its
backstop while keeping its install. That is real, it is silent on the superproject's side, and
it stays a row of the ledger.

**Why not `contested`.** The finding does not argue that a settled decision is wrong, and the
verdict needs no ruling.

## The previous release

The block is the same on the previous release, cell for cell. With a `refuted` verdict there is
no regression fact to give; the run is recorded because triage asked for both binaries.

## Coverage

The finding makes no coverage claim. For the block's `pinned-by` I looked, and only that far:
`command grep -rl submodule` over `crates/cli/tests`, `tooling-tests`, `crates/engine` and
`crates/cli/src` names seventeen files, of which nine also hold the word `uninstall`; three of
those are suites — `git_status_own_flags.rs`, `setup_install_pathspec_guard.rs`, `setup.rs`.
Read at their `submodule` lines: the first names a git status knob and a doc comment; the second
drives `setup` inside a submodule and in a superproject with one, and never `uninstall` from
inside the submodule; the third builds a `core.hooksPath` inside a submodule, another shape.
None sets up a superproject and its submodule both, and none types `uninstall` in a submodule.
A suite that builds the layout without the word would have been missed.

## Repro VR-3 — a superproject and its submodule both set up share one hook block; `jigc uninstall` in the submodule refuses, and its printed `--force` route cuts the superproject's block

```yaml
claim: "with `jigc setup` run in a superproject and then in its submodule, the two installs share one `pre-commit` block; `setup` or `uninstall` typed in the submodule leaves the superproject without its own backstop, or with a block that names the other install's binary"
verdict: REFUTED
basis: breaks-no-clause
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exits, outputs and writes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d); the hook is 5169 bytes there, 5153 here — the embedded binary path's length; no `.jigc/settings-entries.json` there"
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, GIT_AUTHOR_NAME/EMAIL and GIT_COMMITTER_NAME/EMAIL set, JIGC_PACK_DIR unset"
  - ["git", "init", "-q", "lib"]
  - ["git", "-C", "lib", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["git", "init", "-q", "super"]
  - ["git", "-C", "super", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["git", "-C", "super", "-c", "protocol.file.allow=always", "submodule", "add", "-q", "<abs>/lib", "vendor/lib"]
  - ["git", "-C", "super", "commit", "-q", "-m", "add submodule"]
  - write: "super/.git/hooks/pre-commit = '#!/bin/sh\necho superproject-own-hook-ran\n', mode 755"   # 41 bytes
  - cwd: super
  - ["jigc", "setup"]                                                                  # exit 0; keep the hook's bytes as H1
  - ["jigc", "start", "--workflow", "form-vision", "form the project vision"]          # prints `task minted: <T>`
  - ["jigc", "doc", "create", "vision", "--title", "Vision", "--task", "<T>"]
  - ["jigc", "doc", "set-slot", "vision:vision#thesis", "--from-file", "<one line of prose>", "--task", "<T>"]
  - ["jigc", "doc", "set-slot", "vision:vision#invariants", "--from-file", "<one line of prose>", "--task", "<T>"]
  - ["jigc", "doc", "set-slot", "vision:vision#open-questions", "--from-file", "<one line of prose>", "--task", "<T>"]
  - ["jigc", "doc", "set-field", "commit:<T>#type", "--value", "docs", "--task", "<T>"]
  - ["jigc", "doc", "set-field", "commit:<T>#scope", "--value", "vision", "--task", "<T>"]
  - ["jigc", "doc", "set-slot", "commit:<T>#summary", "--from-file", "<one line of prose>", "--task", "<T>"]
  - ["jigc", "doc", "set-slot", "commit:<T>#body", "--from-file", "<one line of prose>", "--task", "<T>"]
  - ["jigc", "task", "finalize", "<T>"]                                                # exit 0; promotes VISION.md
control: &control                                                                      # cwd super
  - ["git", "mv", "VISION.md", "VISION2.md"]
  - ["git", "commit", "-q", "-m", "oob rename"]
  - on-exit-1: ["git", "mv", "VISION2.md", "VISION.md"]
repro:
  - *control                                                                           # control 0
  - cwd: super/vendor/lib
  - ["jigc", "setup"]                                                                  # the hook's bytes now: H2
  - *control                                                                           # control 1
  - cwd: super/vendor/lib
  - ["jigc", "uninstall"]
  - *control                                                                           # control 2
  - cwd: super/vendor/lib
  - ["jigc", "uninstall", "--force"]                                                   # the refusal's printed route
  - *control                                                                           # control 4
expect:
  control_0: { commit_exit: 1, stderr_contains: "out-of-band managed-doc rename staged in this commit", stderr_contains_2: "(commit blocked)" }
  setup_in_submodule: { exit: 0, stderr: "", stdout_contains: ["pre-commit hook → <abs>/super/.git/hooks/pre-commit", "installed at `<abs>/super/.git/modules/vendor`"] }
  hook_after_setup_in_submodule: "H2 == H1 byte for byte; one line `# jigc-managed pre-commit hook — end`; one line matching ^jigc=; the last 31 bytes are the user's"
  control_1: { commit_exit: 1, stderr_contains: "(commit blocked)" }
  uninstall_in_submodule: { exit: 1, stdout: "", stderr_contains: ["uninstall.untracked-workbench-file", "this operation must be run in a work tree", "`jigc uninstall --force`"] }
  files_changed_across_the_refused_uninstall: "none"
  control_2: { commit_exit: 1, stderr_contains: "(commit blocked)" }
  uninstall_force_in_submodule: { exit: 0, stderr: "", stdout_contains: ["jigc uninstall — repo-local install removed", "removed pre-commit hook", "removed at `<abs>/super/.git/modules/vendor`"] }
  hook_after_force: "super/.git/hooks/pre-commit == the 41 bytes written in setup; no sentinel, no `jigc=` line"
  superproject_install_after_force: "super/.jigc/config/ and super/CLAUDE.md present; `git -C super status --porcelain` empty"
  control_4: { commit_exit: 0, stdout: "", stderr: "superproject-own-hook-ran\n", head: "`git show --stat -M HEAD` = VISION.md => VISION2.md" }
observed: "<W>/cand2.uj16iA and <W>/prev.L4xAlU — each with ev/<step>.out and ev/<step>.err for every step (setup-super, finalize, ctl0…ctl4, setup-sub, uninstall-sub, uninstall-sub-force, val-super-0…3, upgrade-super, setup-super-2), the hook snapshots ev/hook.orig, hook.1…hook.5, and the manifests ev/man.0…man.7; <W>/cand.4BNCxr holds the block without the --force step; <W>/mixed.NDWGRN the two-binary cell; the scripts are <W>/drive.sh and <W>/mixed.sh, the logs <W>/cand.log, cand2.log, prev.log, mixed.log"
pinned-by: "UNPINNED: no suite found that sets up a superproject and its submodule both, or types `uninstall` in a submodule (searched by the word `submodule` beside `uninstall`; three suites, none of this shape)"
```

**Pinnable as it stands:** yes. Every step is argv or a file write, the expectations are exits,
exact strings, byte comparisons and file lists, and the one machine-dependent value — the
binary's absolute path inside the hook — appears in no assertion (`H2 == H1` compares two files
the same binary wrote). One caution for whoever pins it: `control_4` and `hook_after_force`
pin what both binaries do **today**, which is the defect; a fix, or a ruling on row (D) that
makes `setup` refuse in a submodule, turns those two expectations — and the setup step's
`exit 0` — and the block changes with it. The rows above them (one block, the refusal, the
controls that stay blocked) are the ones a fix must keep.

## The class

`instance, unbounded`. I drove one layout — a submodule at `vendor/lib` under a superproject
that is itself a jigc project and has a hook of its own — with the installs made in one order
(superproject first), on two binaries, plus one two-binary cell in one order. I did not
enumerate the layouts in which two installs resolve to one hooks directory, and derived no
count. The mechanism is one fact — the home of a submodule sits inside the superproject's git
directory, so git answers the superproject's hooks directory for it — and where a home resolves
elsewhere, nothing here speaks.

## Left open — noticed, not pursued

- **The refusal's first route does not clear the refusal in this layout.** It reads "make sure
  `git` is on PATH and the `.jigc/` tree is readable, then re-run `jigc uninstall`". With git on
  `PATH` and the tree readable, one more bare `jigc uninstall` in the submodule of
  `<W>/cand.4BNCxr` exits 1 with the identical stderr (`cmp` exit 0). The cause the refusal
  itself quotes is *this operation must be run in a work tree* — the home is no work tree — and
  the route names neither that nor a cure for it; `--force` is the only printed way out, on the
  candidate. Driven once, on the candidate only; the previous release prints the same route and
  was not re-run. A refusal's route is the second clause's own subject, so this wants its own
  triage.
- **`--force` in the submodule cuts a block another install depends on, and its ack names no
  path.** The line is `removed pre-commit hook`, where `setup`'s ack in the same directory
  printed the hook's absolute path inside the superproject. The fact is in the table above; the
  sentence of the surface is not graded here.
- **`jigc validate` and `jigc upgrade` in the superproject say nothing of a missing block.**
  Neither reads the hook, as far as this drive shows. Whether a door should is not mine to say.
- **The submodule's home keeps a `.claude/settings.json` after `--force`** on both binaries —
  an object with an empty `SessionStart` array and empty `allow` and `deny` arrays, under
  `super/.git/modules/vendor/`. The design keeps a user's own `.claude/` on purpose; here the
  file was created by `setup` and nothing else was ever in it.
- **The reverse order was not driven** — `setup` in the submodule first, then in the
  superproject — nor the two-binary cell in its other order, nor `uninstall` typed in the
  superproject while the submodule's install stands (which, by the same mechanism, would leave
  the submodule's install with no block; the earlier report already records that the block
  never reaches the submodule's install at all).
- **The ack of `setup` in the submodule** still says "local to this checkout" of a hook in
  another repository's hooks directory, and calls a directory that is no work tree "the main
  checkout". Both were already noted by the earlier report and by row (D).

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. The one file I wrote with the file tool is this report; the
two drive scripts and their evidence are under `<W>`, in the scratch root.

<!-- end of report -->
