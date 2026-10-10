# verify-real — `r1-setup-installs-outside-work-tree` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup`. Clause it is said to break:
`working-product`. Triage's grade: *unclear*.

## Verdict in one paragraph

**`confirmed`, `regression: false`, `contested: true`.** Every cell of `Repro RC-6` holds as
written on the candidate: `jigc setup` typed in a worktree of a bare repository writes seven
files into the folder that holds the bare repository, appends to a `CLAUDE.md` there, exits 1
at `setup.install-hook`, and the route it prints, followed as printed, exits 1 again with the
same refusal. The submodule variant holds too: exit 0, the install inside the superproject's
git directory, the superproject's `pre-commit` hook rewritten with the user's hook body kept
after jigc's block. The previous release does the same in both layouts, so nothing regressed.
What I tried and could not do is refute it: it reproduces, no settled decision calls the
behaviour right, and the looping route is the letter of the clause's second measure, *every
refusal's route works as printed*. **Whether that breach is inside the clause's scope is not
derivable from the record** — `implementation/decisions-pending.md`, the M57 list's (D) row on
these layouts, holds the support question open and names it the human's call — and any fix
would take that decision and flip cells a standing test pins as *what the previous release does
here*. That is a fork for the human, and I do not settle it.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

For every candidate run the candidate's directory was first on `PATH` and `command -v jigc`
printed its path, checked at the top of each command block. For the previous release's runs the
binary was called by its absolute path, with its own directory first on `PATH` for that block
only. Both binaries print `jigc 1.0.0-rc.24`; the evidence here is keyed by the hashes above.
Nothing was built, and nothing under `target/` was driven.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-setup-outside.Ri5wMy`. Four fresh roots inside it, one per layout and
  binary, each minted the same way: `bare-cand.L4toSz`, `bare-prev.Ci4dkh`, `sub-cand.eOKbce`,
  `sub-prev.2QiX3e`. None is a reporter's.
- Environment of every root: `HOME` a fresh empty directory inside the root,
  `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null`, the four `GIT_AUTHOR_*` /
  `GIT_COMMITTER_*` variables set to a synthetic identity. Git is 2.54.0 (Apple Git-157), macOS.
- Every exit status was read bare — `cmd > out 2> err; echo rc=$?` — never through a pipe.
- A manifest (sha256 of every file and link under the root, git object files left out) was taken
  before the first `setup`, after it, and after the second.
- No rig: the block builds its own layout with git, and the rig's states do not include these
  layouts. I ran the driver's `Repro 4` step for step (it is `Repro RC-6` with its setup spelled
  out), and the source's `Repro S4` six git steps for the submodule.
- **What I changed:** for the submodule variant the reports give the superproject's hook as
  "41 bytes, `#!/bin/sh` and one `echo`" and not its bytes; I wrote
  `#!/bin/sh` + newline + `echo superproject-own-hook-ran` + newline, which is 41 bytes.

## What was observed

### A worktree of a bare repository — candidate (`<W>/bare-cand.L4toSz`)

| step | exit | what |
|---|---|---|
| `git init -q --bare store.git` (cwd `proj`) | 0 | |
| `git -C store.git worktree add -q ../wt -b main` | 0 | |
| write `proj/CLAUDE.md`, 80 bytes | - | three lines, in no git repository |
| in `proj/wt`: write `README.md`, `git add`, `git commit -q -m base` | 0 | |
| `jigc setup` (cwd `proj/wt`), first | **1** | stdout empty; stderr below |
| `jigc setup` (cwd `proj/wt`), the route as printed | **1** | stdout empty; stderr byte-identical |

stderr, both times (the path shortened to its placeholder):

```text
blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: git could not resolve the hooks dir for `<W>/bare-cand.L4toSz/proj`: fatal: not a git repository (or any of the parent directories): .git
  route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

Manifest of `proj`, before against after the first run — seven files created and one changed,
all in `proj`, which is no work tree:

- created: `.claude/settings.json`, `.claude/skills/jigc/SKILL.md`, `.jigc/.gitignore`,
  `.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`, `.jigc/version`
- changed: `CLAUDE.md`, 80 → 119 bytes — the three original lines, then a blank line,
  `## Project interface`, a blank line, `@.jigc/AGENT.md`
- nothing under `store.git` changed (object files aside, which the manifest leaves out); no
  `pre-commit` file exists in `store.git/hooks`

After the second run the manifest is identical to the one after the first. `proj/wt` holds
`.git` and `README.md`, its `git status --porcelain --untracked-files=all --ignored` is empty,
and it has one commit.

**The route's precondition already holds, and the route still fails.** The route says *ensure
the repo's git hooks directory is writable*. Asked from the worktree,
`git rev-parse --git-path hooks` answers `proj/store.git/hooks` at exit 0, and that directory is
writable (`drwxr-xr-x`, mine; `[ -w … ]` true). Asked at `proj` — the directory the refusal
names — the same question exits 128 with git's *not a git repository*. So nothing a reader can
do to the hooks directory changes the answer: the refusal is about where the question is asked,
and its route is about something else.

### A worktree of a bare repository — previous release (`<W>/bare-prev.Ci4dkh`)

The same steps, the binary called by its absolute path. First `setup` exit **1**, second exit
**1**; stdout empty both times; stderr the same three lines, the two runs byte-identical to each
other. The same seven files created and `CLAUDE.md` 80 → 119 bytes; every created file has the
hash the candidate's has **except `.claude/skills/jigc/SKILL.md`** (the embedded guides differ
between the builds). The second run changes nothing. `proj/wt`: porcelain empty, one commit.

### A submodule — candidate (`<W>/sub-cand.eOKbce`)

Setup: `git init -q lib`, an empty commit; `git init -q super`, an empty commit;
`git -C super -c protocol.file.allow=always submodule add -q <abs>/lib vendor/lib`; commit;
then the 41-byte hook written to `super/.git/hooks/pre-commit`, mode 755. All git steps exit 0.

| step | exit | what |
|---|---|---|
| `jigc setup` (cwd `super/vendor/lib`) | **0** | stderr empty; the ack below |
| `git commit -q --allow-empty -m …` (cwd `super`), afterwards | 0 | stderr is the one line `superproject-own-hook-ran`; the commit landed (3 commits) |
| `jigc setup` again (cwd `super/vendor/lib`) | 0 | the hook is the same 5153 bytes, one end marker |

- The ack's hook line names `<W>/sub-cand.eOKbce/super/.git/hooks/pre-commit`, and its last line
  says the install is at `…/super/.git/modules/vendor` — "the main checkout this repository's
  jigc install and `.jigc/` workbench bind to, not the worktree you are standing in".
- Manifest of `super`: eight files created under `.git/modules/vendor/` — the seven above and
  `CLAUDE.md` — and one changed, `.git/hooks/pre-commit`.
- `super/.git/modules/vendor/` lists `.claude`, `.jigc`, `CLAUDE.md`, `lib`. `super/vendor/lib`
  lists `.git` alone. Porcelain (with untracked and ignored) is empty in the submodule and in
  the superproject.
- **The superproject's hook:** 41 → 5153 bytes, still mode 755. It opens with `#!/bin/sh` and
  jigc's managed block, the block closes with `# jigc-managed pre-commit hook — end`, and the
  file's last 31 bytes are the user's `echo` line, byte for byte (`cmp` exit 0). The submodule's
  own hooks directory, `super/.git/modules/vendor/lib/hooks`, holds no `pre-commit`.
- The hook holds the absolute path of the binary that wrote it, which is why its size is not the
  reporter's 5187: the size moves with the length of that path and with nothing else I saw.

### A submodule — previous release (`<W>/sub-prev.2QiX3e`)

The same steps. `setup` exit **0**, stderr empty, the same two ack lines. The same eight files
created under `super/.git/modules/vendor/`, all with the candidate's hashes but `SKILL.md`. The
superproject's hook 41 → 5169 bytes — sixteen more than the candidate's, the difference in
length between the two binaries' directory names — ending with the same end marker and the
user's line, byte for byte.

## The finding's claims, one by one

| claim | where | observed |
|---|---|---|
| `setup` in a worktree of a bare repository writes its install into the folder that holds the bare repository | RC-6, driver Repro 4 | **holds** — the seven files of the block's `created_in_proj`, exactly |
| it appends to a `CLAUDE.md` there, 80 → 119 bytes, the three lines kept | RC-6 | **holds** |
| it exits 1, stdout empty, at `setup.install-hook` | RC-6 | **holds** |
| the route, re-run as printed, exits 1 again | RC-6 | **holds** — and the route's precondition was already true |
| `proj/wt`: porcelain empty, one commit, `README.md` alone | RC-6 | **holds** |
| "identical on the previous release but for the size of the installed skill file" | RC-6 | **holds** — I compared hashes, not sizes: `SKILL.md` is the one file that differs |
| a submodule: exit 0; `.claude`, `.jigc`, `CLAUDE.md` in `super/.git/modules/vendor/` | RC-6 variant, source L1 | **holds** |
| the superproject's hook is rewritten, the managed block first, the user's body after it, no byte of it lost | RC-6 | **holds**; its size is path-dependent (5153 here, 5187 there) |
| "the superproject's commits now run `jigc validate` first" | RC-6 | **not established here** — a commit in the superproject exits 0 and prints the user's line; I did not trace what the managed block ran |
| `--separate-git-dir`: exit 1 at the same code, after the same install | RC-6 variant | **not driven** — triage asked for two layouts |
| "UNPINNED" | RC-6, Repro 4, L1 | **does not hold** — see *The coverage claim* |

## What the record holds

**`implementation/decisions-pending.md` → the M57 list, the row "(D) A worktree of a bare
repository answers *isn't set up* at every door — the human's call".** As recorded on 2026-10-04
it said jigc is unusable in that layout. Its bracketed correction of 2026-10-05 calls that a
false record and replaces it: after the `jigc setup` the message routes at, the previous release
creates and finalizes a doc in a worktree of a bare repository, in a `--separate-git-dir`
checkout and in a submodule. It then lists *what is partial there, pre-existing and still open*,
and this finding is on that list in so many words: "`jigc setup` exits 1 at its hook step in the
sibling-bare and `--separate-git-dir` layouts, after writing". It also records that `setup` does
not ask its pre-write dirty question in any of the three, and that its ack calls the directory
*the main checkout*. **What it says is owed:** "whether these three layouts are supported — and
if they are not, a refusal that says so at `setup`, rather than doors that half-work."
**Trigger: M57**, the 1.x fix pass. So (D) holds three things: the behaviour is known and
written down; nothing rules the layouts supported and nothing rules them unsupported; and the
ruling is the human's. The deferral to M57 is recorded as the linked-worktree fixer's, not as a
ruling of the human's — unlike the next row of the same list, which says *ruled tier 2 for 1.x
by the human*.

**The design section that owns the behaviour** — `design/assistant-adapter.md`, the doc↔code
backstop's install discipline: the hook install "must resolve the real hooks dir (honoring
`core.hooksPath` and the git-worktree `.git`-is-a-file case)", and is non-destructive, wrapping
an existing `pre-commit` hook. *Where it installs: jigc_home, from every cwd* describes the home
as "the main checkout". No sentence of it describes a home that is no checkout. So the design
does not intend an exit 1 here: the hook step asks git at a directory that is no repository,
which is the opposite of resolving the real hooks dir. The non-destructive half holds in the
submodule.

**The fix pass's own decision, and the test that pins it.** `DECISIONS.md` → the round-4
as-built entry, *Area B — the linked-worktree guard*: in these three layouts "every door does
what it did on rc.24". The standing suite holds `setup` to exactly that, at the candidate's
commit: `setup_install_pathspec_guard::git_that_answers_is_never_refused_as_git_that_does_not`
drives `jigc setup` twice — "the first run", "the re-run" — in a `--separate-git-dir` checkout
and in a worktree beside a bare repository and asserts exit 1 with `setup.install-hook` both
times, and exit 0 inside a submodule and behind a bare `.git`; its assertion message is "the
door does what `1.0.0-rc.24` does here". The candidate's behaviour is therefore held on
purpose, as parity — not called right.

**The clause.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, the second clause's
measure: "no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed". The human's ruling on the instrument's shape (`DECISIONS.md`
→ *2026-10-06*, the regression set's shape, part 2) names "a worktree of a bare repository · a
separate git directory · a submodule" among "the nine ordinary configurations and layouts" in
which what succeeds on the previous release must succeed on the candidate, and leaves a route
that is a sentence for a human to the review rows, "which run every printed route".

## Does it break the clause, inside its scope

- **The first measure — not broken.** `setup` does not work in the bare-worktree layout on the
  previous release either, and works the same in the submodule. Nothing that worked stopped.
- **The second measure — broken on its letter, in one of the two layouts driven.** In the
  worktree of a bare repository a refusal prints a route, the route was followed as printed, and
  it ends in the same refusal. In the submodule there is no refusal: `setup` exits 0, so that
  variant by itself breaks neither measure of this clause. What it does — a `setup` typed in a
  submodule writing into the superproject's hooks — is a question of the first clause's shape,
  which the finding does not claim and I did not grade.
- **The scope — not settled by anything I can cite.** *In a supported layout* stands in the
  first half of the sentence; the second half carries no layout term. Read strictly, the breach
  is in scope whatever the layout. Read with the term carried across, it is in scope only if the
  layout is supported — and (D) holds that question open, the human's. The ruling on the
  regression set calls all three layouts *ordinary*, which leans one way; (D)'s trigger, after
  the 1.0.0 call, leans the other.

**Why `confirmed`.** Each refutation I tried failed. *Does not reproduce:* it reproduces from
nothing, on both binaries. *Intended:* no decision says exit 1 after writing, with a route that
loops, is the right behaviour — the round-4 decision holds the layouts at the previous release's
behaviour, and (D) calls that behaviour partial. *Breaks no clause:* it breaks the second
measure's letter, and I can show no ruling that puts the layout outside its scope.

**Why `contested`.** A fix cannot be written without deciding what (D) reserves for the human.
Making `setup` succeed there declares the layout supported; a refusal that says the layout is
unsupported declares the opposite, and would stop doors the correction to (D) records as
working on the previous release; and either one flips cells the standing test above pins as
parity. As a blocker under `working-product`, the finding argues that this is decided before the
call and not at M57. That is the fork: admit it now, declare it a bound, or leave it at its
trigger.

## The regression fact

`regression: false`. The same block on a fresh root with the previous release's binary
(`accf3996…`, called by its absolute path) is red in the same way: exit 1, exit 1, the same
refusal and route, the same writes. The submodule variant is exit 0 on both.

## The coverage claim — "UNPINNED"

Verified from the suites, not from the finding. `command grep` for the three layouts'
constructions (`--bare`, `separate-git-dir`, `submodule`) over `crates/cli/tests`,
`tooling-tests` and `crates/engine` names fourteen files; `setup.install-hook` is asserted in
four suites. Opened, by what each asserts:

- `setup_install_pathspec_guard::git_that_answers_is_never_refused_as_git_that_does_not` —
  **pins the exit and the code of this finding's block, on the first run and the re-run**, for a
  worktree beside a bare repository and a `--separate-git-dir` checkout; and exit 0, clean
  porcelain, on two runs inside a submodule. It asserts nothing about the route's text, about
  where the install lands, about the `CLAUDE.md` beside the bare repository, or about the
  superproject's hook.
- `linked_worktree_doc_home::a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard`
  — runs `setup` in all three layouts and says in its doc comment that "its exit is not the
  cell's claim"; it pins that a doc lands afterwards.
- `setup_failed_first_run` and `replacing_writers_never_follow` assert `setup.install-hook` under
  a read-only hooks directory and at a link — other causes, in an ordinary checkout.

So "nothing pins this" is wrong for the exit and the code, which a standing test holds as
intended parity, and right for the four things listed as unasserted. I searched by the layouts'
constructions and by the code; a suite that builds one of these layouts by another spelling
would have been missed, and I looked for none.

## Repro VR-1 — `jigc setup` in a worktree of a bare repository: writes, exits 1, and its route loops

```yaml
claim: "`jigc setup` typed in a worktree of a bare repository writes its install into the folder that holds the bare repository, appends to a CLAUDE.md there, exits 1 at setup.install-hook, and its route re-run as printed exits 1 again"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exits, stderr and writes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d); .claude/skills/jigc/SKILL.md is the one file whose bytes differ"
regression: false
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null, GIT_AUTHOR_NAME/EMAIL and GIT_COMMITTER_NAME/EMAIL set"
  - ["mkdir", "proj"]
  - cwd: proj
  - ["git", "init", "-q", "--bare", "store.git"]
  - ["git", "-C", "store.git", "worktree", "add", "-q", "../wt", "-b", "main"]
  - write: "CLAUDE.md = '# my folder-level agent rules\n\nkept beside the bare store, in no git repository\n'"   # 80 bytes
  - cwd: proj/wt
  - write: "README.md = '# readme\n'"
  - ["git", "add", "README.md"]
  - ["git", "commit", "-q", "-m", "base"]
repro:
  - ["jigc", "setup"]                       # cwd proj/wt
  - ["jigc", "setup"]                       # the route, as printed; cwd proj/wt
expect:
  exit: 1                                   # both runs
  stdout: ""                                # both runs
  stderr_contains:
    - "blocking · setup.install-hook"
    - "git could not resolve the hooks dir for `<abs>/proj`"
    - "route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`"
  stderr_second_run: "byte-identical to the first"
  precondition_of_the_route: "`git rev-parse --git-path hooks` in proj/wt exits 0 naming proj/store.git/hooks, which is writable"
  created_in_proj: [".claude/settings.json", ".claude/skills/jigc/SKILL.md", ".jigc/.gitignore", ".jigc/AGENT.md", ".jigc/config/.gitkeep", ".jigc/config/packs.yaml", ".jigc/version"]
  proj_CLAUDE_md: "80 -> 119 bytes: the three original lines, then '\n## Project interface\n\n@.jigc/AGENT.md\n'"
  proj_store_git_hooks: "no pre-commit file"
  proj_wt: "`git status --porcelain --untracked-files=all --ignored` empty; one commit; .git and README.md alone"
  second_run_changes: "no file's bytes"
variants:
  - layout: "a submodule (super/vendor/lib), the superproject holding a 41-byte pre-commit hook of its own"
    repro: [["jigc", "setup"]]              # cwd super/vendor/lib
    expect: "exit 0, stderr empty; .claude, .jigc, CLAUDE.md created in super/.git/modules/vendor/; super/.git/hooks/pre-commit rewritten — the managed block, its end marker, then the user's hook body byte for byte; super/vendor/lib holds .git alone; both porcelains empty; a later commit in super exits 0 and prints the user's hook line"
    on-previous-release: "the same"
    breaks: "neither measure of working-product by itself: no refusal, no regression"
observed: "<W>/bare-cand.L4toSz, <W>/bare-prev.Ci4dkh, <W>/sub-cand.eOKbce, <W>/sub-prev.2QiX3e — each with ev/setup1.err, ev/setup2.err, ev/man.before, ev/man.after1 (and ev/man.after2 in the two bare roots; ev/hook.before in the two submodule roots)"
pinned-by: "setup_install_pathspec_guard::git_that_answers_is_never_refused_as_git_that_does_not — pins the exit and the code on both runs, as parity with the previous release. UNPINNED for the rest: the route's text, where the install lands, the appended CLAUDE.md, the superproject's hook"
```

**Pinnable as it stands:** yes, as a statement of what the binary does — every step is argv, the
expectations are exits, substrings and file lists, and the one machine-dependent value is the
absolute path inside stderr. **As a fix's red test it waits for the ruling:** what the second
run should do instead — succeed, or never be routed at — is the fork, and the test named above
would have to change with it.

## The class

`instance, unbounded`. I drove one door, `jigc setup`, in two of the three layouts the finding
names, on two binaries. I did not drive `--separate-git-dir`, did not enumerate the consumers of
the home resolution, and derived no count.

## Left open — noticed, not pursued

- The finding's blocks say *UNPINNED*; a standing test pins the exit and the code of this very
  block as intended parity. Whoever takes the fork should know that a fix flips it.
- The submodule variant breaks no measure of `working-product`. Whether a `setup` typed in a
  submodule may write into the superproject's hooks directory — rather than the submodule's own,
  which stays empty — is a question of the first clause's shape that nobody has graded.
- "The superproject's commits now run `jigc validate` first" (RC-6) was not established: I saw a
  superproject commit exit 0 and print the user's hook line, and did not trace the managed block.
- The `setup.install-hook` route names writability for a refusal whose cause is that git was
  asked at a directory that is no repository. Whether the same route is printed, and is as wrong,
  for other causes of that code was not looked at.
- The `--separate-git-dir` layout was not driven here.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. The one file I wrote is this report.

<!-- end of report -->
