# The 170 red route-catalog cases, by the kind of state that provokes the refusal

*What this file is: the count the human's ruling of 2026-10-11 on which states a route must work in was put on ([DECISIONS.md](../../../../../DECISIONS.md) → *Which states a route must work in*). It was made on 2026-10-11 by a read-only agent the orchestrating session launched, and is committed as that agent handed it over — this line added, no number changed. It counts and rules nothing; where the suite's own reading of the same states (`route_case::standing`) differs from its buckets is said in that entry.*

Counted on worktree `streams/p`, branch `fix/rc24-route-catalog`, commit `ae5454bf`. A count, not a judgement: nothing here says which states an ordinary user reaches.

## How it was counted

- **The number.** `EXPECTED_RED` in `crates/cli/tests/route_catalog.rs` holds **170 entries, 170 distinct case names**, and every one names a case of the tables (353 cases, 353 distinct names). Counted by compiling the two case tables and the list themselves — a throwaway `rustc` program in the scratchpad that includes `route_catalog/setup.rs` and `route_catalog/uninstall.rs` by path, so every macro is expanded by the compiler and no case is read off its name — and printing each `Case` with its `cause`, `state`, `env`, `from` and `refusing`. Key and classification of all 170 agree with `route-catalog-pilot-cases.tsv` (0 differences; the TSV has 170 red rows of 353).
- **The bucket.** A rule on the `cause` text picks the bucket; the state steps must then carry the step that bucket needs (a `chmod` for permission, a `Fifo`/`Link`/`Dir` step for file-kind, the variable for environment, …) or the case is reported as an error. 0 errors. The sub-bucket of a permission case is the path of its `chmod`.
- **How a permission is planted.** The vocabulary has no step for one: every permission case runs `chmod <mode> <path>` through a git alias (`shut!`). All 47 are a bit changed by a command, none by an umask, a mount or an ownership.
- **The check by reading.** A stride sample of 34 cases (every fifth of the 170) was labelled by hand from the dumped states *before* the script ran: **34 of 34 agree**. All 170 dumped states were then read once against the script's output; no further disagreement. What I was unsure of is a question of which bucket a state belongs in, not of what the state is — §4.
- **Mixed states.** Where a case stacks two kinds, the bucket is the one the `cause` names as what the site refuses over; the other is in the last column of §3 and counted in §1c.

## 1. Bucket × command × classification

| bucket | command | same refusal again | another failure | a witness changed | did not end | unsplittable line | total |
|---|---|---:|---:|---:|---:|---:|---:|
| 1a permission — inside `.jigc/` | setup | 7 | 4 | · | · | · | 11 |
| 1a permission — inside `.jigc/` | uninstall | 6 | 7 | · | · | · | 13 |
| **1a permission** | **both** | **13** | **11** | · | · | · | **24** |
| 1b permission — user-owned file in the repository, or its root | setup | 10 | · | · | · | · | 10 |
| 1b permission — user-owned file in the repository, or its root | uninstall | 6 | 4 | · | · | · | 10 |
| **1b permission** | **both** | **16** | **4** | · | · | · | **20** |
| 1c permission — outside the repository | setup | 1 | · | · | · | · | 1 |
| 1c permission — outside the repository | uninstall | 2 | · | · | · | · | 2 |
| **1c permission** | **both** | **3** | · | · | · | · | **3** |
| 2 file-kind | setup | 25 | 35 | 4 | 4 | · | 68 |
| 2 file-kind | uninstall | 5 | · | · | 3 | · | 8 |
| **2 file-kind** | **both** | **30** | **35** | **4** | **7** | · | **76** |
| 3 git-state | setup | 2 | 3 | · | · | 2 | 7 |
| 3 git-state | uninstall | 3 | · | · | · | 3 | 6 |
| **3 git-state** | **both** | **5** | **3** | · | · | **5** | **13** |
| 4 environment | setup | 2 | · | · | · | · | 2 |
| 4 environment | uninstall | 2 | · | · | · | · | 2 |
| **4 environment** | **both** | **4** | · | · | · | · | **4** |
| 5 content | setup | 21 | 1 | · | · | · | 22 |
| 5 content | uninstall | 6 | · | · | · | · | 6 |
| **5 content** | **both** | **27** | **1** | · | · | · | **28** |
| 6 other (jigc work in progress: an open task's staged doc) | setup | · | · | · | · | · | 0 |
| 6 other (jigc work in progress: an open task's staged doc) | uninstall | · | · | · | · | 2 | 2 |
| **6 other (jigc work in progress: an open task's staged doc)** | **both** | · | · | · | · | **2** | **2** |
| **all buckets** | **setup** | **68** | **43** | **4** | **4** | **2** | **121** |
| **all buckets** | **uninstall** | **30** | **11** | **0** | **3** | **5** | **49** |
| **all buckets** | **both** | **98** | **54** | **4** | **7** | **7** | **170** |

### 1a. The buckets, without the split by classification

| bucket | setup | uninstall | total |
|---|---:|---:|---:|
| 1 permission (1a+1b+1c) | 22 | 25 | 47 |
|   1a permission — inside `.jigc/` | 11 | 13 | 24 |
|   1b permission — user-owned file in the repository, or its root | 10 | 10 | 20 |
|   1c permission — outside the repository | 1 | 2 | 3 |
| 2 file-kind | 68 | 8 | 76 |
| 3 git-state | 7 | 6 | 13 |
| 4 environment | 2 | 2 | 4 |
| 5 content | 22 | 6 | 28 |
| 6 other (jigc work in progress: an open task's staged doc) | 0 | 2 | 2 |
| **total** | 121 | 49 | 170 |

### 1b. What the `file-kind` entry is, and where it stands

| what stands there | setup | uninstall | total | at which paths |
|---|---:|---:|---:|---|
| symlink to a file outside the repository | 17 | 0 | 17 | `.git/hooks/pre-commit` ×3, `.githooks/pre-commit` ×3, `outside/hooks/pre-commit` ×3, `CLAUDE.md` ×3, `.claude/settings.json` ×3, `.gitignore` ×2 |
| symlink, dangling (target outside the repository) | 11 | 0 | 11 | `CLAUDE.md` ×3, `.claude/settings.json` ×3, `.gitignore` ×2, `.git/hooks/pre-commit` ×1, `.githooks/pre-commit` ×1, `outside/hooks/pre-commit` ×1 |
| symlink to a directory inside the repository | 8 | 0 | 8 | `CLAUDE.md` ×3, `.claude/settings.json` ×3, `.gitignore` ×2 |
| symlink to a file inside git's own directory (.git/) | 8 | 0 | 8 | `CLAUDE.md` ×3, `.claude/settings.json` ×3, `.gitignore` ×2 |
| directory in a file's place | 5 | 3 | 8 | `.claude/settings.json` ×2, `CLAUDE.md` ×2, `.git/hooks/pre-commit` ×2, `.jigc/.gitignore` ×1, `.gitignore` ×1 |
| FIFO | 4 | 3 | 7 | `CLAUDE.md` ×2, `.git/hooks/pre-commit` ×2, `.jigc/config/packs.yaml` ×1, `.claude/skills/jigc/SKILL.md` ×1, `.claude/settings.json` ×1 |
| symlink to a file outside the repository (relative spelling) | 3 | 0 | 3 | `.git/hooks/pre-commit` ×3 |
| symlink to a directory outside the repository | 3 | 0 | 3 | `.git/hooks/pre-commit` ×3 |
| symlink to /dev/null | 3 | 0 | 3 | `.git/hooks/pre-commit` ×3 |
| regular file in a directory's place | 2 | 0 | 2 | `.jigc` ×1, `.jigc/config` ×1 |
| symlink to a file inside the repository | 2 | 0 | 2 | `.claude/settings.json` ×2 |
| regular file in a directory's place (hooks directory, via core.hooksPath) | 1 | 1 | 2 | `core.hooksPath names a file` ×2 |
| device in a directory's place (hooks directory = /dev/null, via core.hooksPath) | 1 | 1 | 2 | `core.hooksPath = /dev/null` ×2 |

Of the 76 `file-kind` cases, **55 are symbolic links** (all `setup`), 7 a FIFO, 8 a directory in a file's place, 6 a regular file or device in a directory's place. By where the link leads: outside the repository to a file 20, outside to a directory 3, dangling 11 (every dangling target is a path outside the repository), to `/dev/null` 3, inside the repository to a directory 8, inside to a file under `.git/` 8, inside to a regular tracked/ignored file 2.

The entry sits inside `.jigc/` in 4 of the 76 (`.jigc` or `.jigc/config` a regular file, `.jigc/.gitignore` a directory, `.jigc/config/packs.yaml` a FIFO); in the other 72 it is a file the user owns (`CLAUDE.md`, `.claude/…`, root `.gitignore`, the `pre-commit` hook, the hooks directory).

### 1c. Second conditions a case also stands on (not its bucket)

| second condition | cases | their buckets |
|---|---:|---|
| `HEAD` unborn (`git update-ref -d HEAD`) — where setup seeds the root `.gitignore`, by the table's own comment | 14 | file-kind 9, permission 1b 2, git-state 2, content 1 |
| `core.hooksPath` set in the repository's git config | 15 | file-kind 12, git-state 2, permission 1c 1 |
| `JIGC_ADAPTERS_DIR` set (the only way any profile site is reached) | 19 | content 17, environment 2 |
| a symlink the command follows to a read-only file outside | 2 | permission 1c 2 |
| the refusing command itself carries `--force` | 1 | permission 1a 1 |
| a `.jigc/` entry planted by the state's own file steps, with no `jigc setup` having run (setup cases only) | 20 | permission 1a 9, file-kind 4, content 4, permission 1b 2, git-state 1 |
| marked `doubt` in the case table (the route's prose has two readings — pilot §7) | 25 | file-kind 11, git-state 7, permission 1a 4, other 2, content 1 |

Every `uninstall` case starts from an install the binary under test made over a repository with the owner's `CLAUDE.md`, settings and hook committed; two `setup` cases start from an install too (§3).

## 2. Bucket × finding key

| key | 1a | 1b | 1c | file-kind | git-state | environment | content | other | total |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `r1-p3-setup-writability-route-printed-for-other-causes` | 7 | 9 | 1 | 10 | 1 | · | 7 | · | 35 |
| `r1-p3-uninstall-writability-route-not-driven` | 2 | 6 | 2 | 5 | 1 | · | 4 | · | 20 |
| `r1-setup-untracked-link-repointed-draws-dirty-install-path` | · | · | · | 16 | · | · | · | · | 16 |
| `r1-setup-install-hook-route-other-causes-unexamined` | · | · | · | 12 | · | · | · | · | 12 |
| `r1-setup-profile-clause-route-names-no-act` | · | · | · | · | · | · | 12 | · | 12 |
| `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | · | · | · | 8 | · | · | · | · | 8 |
| `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | · | · | · | 8 | · | · | · | · | 8 |
| `r1-setup-profile-route-says-reinstall-under-adapters-dir` | · | · | · | · | · | 2 | 5 | · | 7 |
| `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | 6 | · | · | · | · | · | · | · | 6 |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | · | 1 | · | 4 | · | · | · | · | 5 |
| `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | · | · | · | · | 3 | · | · | 2 | 5 |
| `r1-setup-fifo-at-install-path-holds-the-command` | · | · | · | 4 | · | · | · | · | 4 |
| `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | · | 4 | · | · | · | · | · | · | 4 |
| `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | · | · | · | 3 | · | · | · | · | 3 |
| `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | · | · | · | 3 | · | · | · | · | 3 |
| `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` | 3 | · | · | · | · | · | · | · | 3 |
| `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command` | · | · | · | 3 | · | · | · | · | 3 |
| `r1-setup-home-unset-route-says-run-from-inside-the-repository` | · | · | · | · | · | 2 | · | · | 2 |
| `r1-setup-workbench-made-writable-rerun-refuses-its-own-files` | 2 | · | · | · | · | · | · | · | 2 |
| `r1-setup-settings-record-refused-rerun-installs-without-the-record` | 2 | · | · | · | · | · | · | · | 2 |
| `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` | · | · | · | · | 2 | · | · | · | 2 |
| `r1-setup-unverified-route-opens-with-the-question-that-failed` | · | · | · | · | 2 | · | · | · | 2 |
| `r1-p3-uninstall-force-names-nothing-undecodable-listing` | · | · | · | · | 2 | · | · | · | 2 |
| `r1-setup-installs-outside-work-tree` | · | · | · | · | 1 | · | · | · | 1 |
| `r1-setup-force-over-skip-worktree-path-cannot-commit` | · | · | · | · | 1 | · | · | · | 1 |
| `r1-uninstall-worktree-route-move-aside-over-unreadable-path` | 1 | · | · | · | · | · | · | · | 1 |
| `r1-uninstall-worktrees-route-prints-git-worktree-remove-without-a-path` | 1 | · | · | · | · | · | · | · | 1 |
| **total (27 keys)** | **24** | **20** | **3** | **76** | **13** | **4** | **28** | **2** | **170** |

Columns: 1a · 1b · 1c permission, then file-kind, git-state, environment, content, other.

### 2a. The largest keys of each bucket

- **1 permission, all three** (47): `r1-p3-setup-writability-route-printed-for-other-causes` 17; `r1-p3-uninstall-writability-route-not-driven` 10; `r1-uninstall-force-over-unreadable-workbench-fails-part-way` 6; `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` 4
- **1a permission — inside `.jigc/`** (24): `r1-p3-setup-writability-route-printed-for-other-causes` 7; `r1-uninstall-force-over-unreadable-workbench-fails-part-way` 6; `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` 3; `r1-setup-workbench-made-writable-rerun-refuses-its-own-files` 2
- **1b permission — user-owned file in the repository, or its root** (20): `r1-p3-setup-writability-route-printed-for-other-causes` 9; `r1-p3-uninstall-writability-route-not-driven` 6; `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` 4; `r1-p3-setup-install-hook-undriven-producers-and-causes` 1
- **1c permission — outside the repository** (3): `r1-p3-uninstall-writability-route-not-driven` 2; `r1-p3-setup-writability-route-printed-for-other-causes` 1
- **2 file-kind** (76): `r1-setup-untracked-link-repointed-draws-dirty-install-path` 16; `r1-setup-install-hook-route-other-causes-unexamined` 12; `r1-p3-setup-writability-route-printed-for-other-causes` 10; `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` 8
- **3 git-state** (13): `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` 3; `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` 2; `r1-setup-unverified-route-opens-with-the-question-that-failed` 2; `r1-p3-uninstall-force-names-nothing-undecodable-listing` 2
- **4 environment** (4): `r1-setup-home-unset-route-says-run-from-inside-the-repository` 2; `r1-setup-profile-route-says-reinstall-under-adapters-dir` 2
- **5 content** (28): `r1-setup-profile-clause-route-names-no-act` 12; `r1-p3-setup-writability-route-printed-for-other-causes` 7; `r1-setup-profile-route-says-reinstall-under-adapters-dir` 5; `r1-p3-uninstall-writability-route-not-driven` 4
- **6 other (jigc work in progress: an open task's staged doc)** (2): `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` 2

## 3. The full list

Order: the case tables' own. `cls`: same = the same refusal again · other = another failure · witness = a witness changed · held = the command did not end · line = a printed line no shell can split. `d` = marked `doubt`.

| # | case | bucket | key | cls | the state | also |
|---:|---|---|---|---|---|---|
| 1 | `setup-hook-link-out-to-a-script-removed` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | witness | .git/hooks/pre-commit -> outside/pre-commit |  |
| 2 | `setup-hook-link-out-to-a-script-pointed-at-a-new-script` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .git/hooks/pre-commit -> outside/pre-commit |  |
| 3 | `setup-hook-link-out-to-a-script-pointed-at-a-committed-script` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .git/hooks/pre-commit -> outside/pre-commit |  |
| 4 | `setup-hook-link-to-nothing-pointed-at-a-new-script` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | other | .git/hooks/pre-commit -> outside/pre-commit | d |
| 5 | `setup-hook-relative-link-out-to-a-script-removed` | file-kind — symlink to a file outside the repository (relative spelling) | `r1-p3-setup-install-hook-undriven-producers-and-causes` | witness | .git/hooks/pre-commit -> ../../../outside/pre-commit |  |
| 6 | `setup-hook-relative-link-out-to-a-script-pointed-at-a-new-script` | file-kind — symlink to a file outside the repository (relative spelling) | `r1-p3-setup-install-hook-undriven-producers-and-causes` | same | .git/hooks/pre-commit -> ../../../outside/pre-commit |  |
| 7 | `setup-hook-relative-link-out-to-a-script-pointed-at-a-committed-script` | file-kind — symlink to a file outside the repository (relative spelling) | `r1-p3-setup-install-hook-undriven-producers-and-causes` | same | .git/hooks/pre-commit -> ../../../outside/pre-commit |  |
| 8 | `setup-hook-link-out-to-a-directory-removed` | file-kind — symlink to a directory outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .git/hooks/pre-commit -> outside/hooks |  |
| 9 | `setup-hook-link-out-to-a-directory-pointed-at-a-new-script` | file-kind — symlink to a directory outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .git/hooks/pre-commit -> outside/hooks |  |
| 10 | `setup-hook-link-out-to-a-directory-pointed-at-a-committed-script` | file-kind — symlink to a directory outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .git/hooks/pre-commit -> outside/hooks |  |
| 11 | `setup-hook-link-to-dev-null-removed` | file-kind — symlink to /dev/null | `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | same | .git/hooks/pre-commit -> /dev/null |  |
| 12 | `setup-hook-link-to-dev-null-pointed-at-a-new-script` | file-kind — symlink to /dev/null | `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | same | .git/hooks/pre-commit -> /dev/null |  |
| 13 | `setup-hook-link-to-dev-null-pointed-at-a-committed-script` | file-kind — symlink to /dev/null | `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | same | .git/hooks/pre-commit -> /dev/null |  |
| 14 | `setup-hooks-path-inside-link-to-nothing-pointed-at-a-new-script` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | other | .githooks/pre-commit -> outside/pre-commit | core.hooksPath='.githooks'; d |
| 15 | `setup-hooks-path-inside-link-out-to-a-script-removed` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | witness | .githooks/pre-commit -> outside/pre-commit | core.hooksPath='.githooks' |
| 16 | `setup-hooks-path-inside-link-out-to-a-script-pointed-at-a-new-script` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .githooks/pre-commit -> outside/pre-commit | core.hooksPath='.githooks' |
| 17 | `setup-hooks-path-inside-link-out-to-a-script-pointed-at-a-committed-script` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | .githooks/pre-commit -> outside/pre-commit | core.hooksPath='.githooks' |
| 18 | `setup-hooks-path-outside-link-to-nothing-pointed-at-a-new-script` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | other | outside/hooks/pre-commit -> outside/nothing | core.hooksPath='../outside/hooks'; d |
| 19 | `setup-hooks-path-outside-link-out-of-it-to-a-script-removed` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | witness | outside/hooks/pre-commit -> outside/pre-commit | core.hooksPath='../outside/hooks' |
| 20 | `setup-hooks-path-outside-link-out-of-it-to-a-script-pointed-at-a-new-script` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | outside/hooks/pre-commit -> outside/pre-commit | core.hooksPath='../outside/hooks' |
| 21 | `setup-hooks-path-outside-link-out-of-it-to-a-script-pointed-at-a-committed-script` | file-kind — symlink to a file outside the repository | `r1-setup-install-hook-route-other-causes-unexamined` | same | outside/hooks/pre-commit -> outside/pre-commit | core.hooksPath='../outside/hooks' |
| 22 | `setup-compose-marker-a-special-file` | file-kind — FIFO | `r1-setup-fifo-at-install-path-holds-the-command` | held | FIFO at .jigc/config/packs.yaml |  |
| 23 | `setup-guide-a-special-file` | file-kind — FIFO | `r1-setup-fifo-at-install-path-holds-the-command` | held | FIFO at .claude/skills/jigc/SKILL.md |  |
| 24 | `setup-host-link-to-nothing-pointed-at-a-committed-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> outside/nothing |  |
| 25 | `setup-host-link-to-nothing-pointed-at-a-new-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> outside/nothing |  |
| 26 | `setup-host-link-to-nothing-committed-and-pointed-at-a-new-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | CLAUDE.md -> outside/nothing | d |
| 27 | `setup-host-link-out-of-the-repository-pointed-at-a-committed-file` | file-kind — symlink to a file outside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> outside/theirs |  |
| 28 | `setup-host-link-out-of-the-repository-pointed-at-a-new-file` | file-kind — symlink to a file outside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> outside/theirs |  |
| 29 | `setup-host-link-out-of-the-repository-committed-and-pointed-at-a-new-file` | file-kind — symlink to a file outside the repository | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | CLAUDE.md -> outside/theirs | d |
| 30 | `setup-host-link-to-a-directory-pointed-at-a-committed-file` | file-kind — symlink to a directory inside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> docs |  |
| 31 | `setup-host-link-to-a-directory-pointed-at-a-new-file` | file-kind — symlink to a directory inside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> docs |  |
| 32 | `setup-host-link-to-a-directory-committed-and-pointed-at-a-new-file` | file-kind — symlink to a directory inside the repository | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | CLAUDE.md -> docs | d |
| 33 | `setup-host-link-into-gits-directory-pointed-at-a-committed-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> .git/theirs |  |
| 34 | `setup-host-link-into-gits-directory-pointed-at-a-new-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | CLAUDE.md -> .git/theirs |  |
| 35 | `setup-host-link-into-gits-directory-committed-and-pointed-at-a-new-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | CLAUDE.md -> .git/theirs | d |
| 36 | `setup-settings-link-to-nothing-pointed-at-a-committed-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> outside/nothing |  |
| 37 | `setup-settings-link-to-nothing-pointed-at-a-new-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> outside/nothing |  |
| 38 | `setup-settings-link-to-nothing-committed-and-pointed-at-a-new-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | .claude/settings.json -> outside/nothing | d |
| 39 | `setup-settings-link-out-of-the-repository-pointed-at-a-committed-file` | file-kind — symlink to a file outside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> outside/theirs |  |
| 40 | `setup-settings-link-out-of-the-repository-pointed-at-a-new-file` | file-kind — symlink to a file outside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> outside/theirs |  |
| 41 | `setup-settings-link-out-of-the-repository-committed-and-pointed-at-a-new-file` | file-kind — symlink to a file outside the repository | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | .claude/settings.json -> outside/theirs | d |
| 42 | `setup-settings-link-to-a-directory-pointed-at-a-committed-file` | file-kind — symlink to a directory inside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> conf |  |
| 43 | `setup-settings-link-to-a-directory-pointed-at-a-new-file` | file-kind — symlink to a directory inside the repository | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> conf |  |
| 44 | `setup-settings-link-to-a-directory-committed-and-pointed-at-a-new-file` | file-kind — symlink to a directory inside the repository | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | .claude/settings.json -> conf | d |
| 45 | `setup-settings-link-into-gits-directory-pointed-at-a-committed-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> .git/theirs |  |
| 46 | `setup-settings-link-into-gits-directory-pointed-at-a-new-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-untracked-link-repointed-draws-dirty-install-path` | other | .claude/settings.json -> .git/theirs |  |
| 47 | `setup-settings-link-into-gits-directory-committed-and-pointed-at-a-new-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | other | .claude/settings.json -> .git/theirs | d |
| 48 | `setup-seeded-ignore-link-to-nothing-pointed-at-a-new-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> outside/nothing | unborn HEAD |
| 49 | `setup-seeded-ignore-link-to-nothing-staged-and-pointed-at-a-new-file` | file-kind — symlink, dangling (target outside the repository) | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> outside/nothing | unborn HEAD |
| 50 | `setup-seeded-ignore-link-out-of-the-repository-pointed-at-a-new-file` | file-kind — symlink to a file outside the repository | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> outside/theirs | unborn HEAD |
| 51 | `setup-seeded-ignore-link-out-of-the-repository-staged-and-pointed-at-a-new-file` | file-kind — symlink to a file outside the repository | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> outside/theirs | unborn HEAD |
| 52 | `setup-seeded-ignore-link-to-a-directory-pointed-at-a-new-file` | file-kind — symlink to a directory inside the repository | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> docs | unborn HEAD |
| 53 | `setup-seeded-ignore-link-to-a-directory-staged-and-pointed-at-a-new-file` | file-kind — symlink to a directory inside the repository | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> docs | unborn HEAD |
| 54 | `setup-seeded-ignore-link-into-gits-directory-pointed-at-a-new-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> .git/theirs | unborn HEAD |
| 55 | `setup-seeded-ignore-link-into-gits-directory-staged-and-pointed-at-a-new-file` | file-kind — symlink to a file inside git's own directory (.git/) | `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | other | .gitignore -> .git/theirs | unborn HEAD |
| 56 | `setup-home-not-set-run-as-the-route-prints` | environment | `r1-setup-home-unset-route-says-run-from-inside-the-repository` | same | HOME unset, inside a repository | env HOME unset |
| 57 | `setup-profile-not-in-the-adapters-directory` | environment | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | JIGC_ADAPTERS_DIR -> empty directory | env JIGC_ADAPTERS_DIR set |
| 58 | `setup-profile-not-utf-8` | content | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | adapter profile: not UTF-8 | env JIGC_ADAPTERS_DIR set |
| 59 | `setup-profile-malformed` | content | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | adapter profile: no profile YAML | env JIGC_ADAPTERS_DIR set |
| 60 | `setup-profile-declares-no-reference` | content | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | adapter profile: no reference target | env JIGC_ADAPTERS_DIR set |
| 61 | `setup-launch-template-without-the-task-placeholder` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set |
| 62 | `setup-launch-template-without-the-workflow-placeholder` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set |
| 63 | `setup-launch-template-without-the-invocation` | content | `r1-setup-profile-clause-route-names-no-act` | other | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set; d |
| 64 | `setup-launch-template-of-three-lines` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set |
| 65 | `setup-launch-template-without-a-backticked-span` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set |
| 66 | `setup-launch-template-with-a-forbidden-marker` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set |
| 67 | `setup-launch-template-with-other-shell` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: launch template clause | env JIGC_ADAPTERS_DIR set |
| 68 | `setup-guide-target-without-a-path` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: guide target clause | env JIGC_ADAPTERS_DIR set |
| 69 | `setup-guide-target-absolute` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: guide target clause | env JIGC_ADAPTERS_DIR set |
| 70 | `setup-guide-target-climbing-out` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: guide target clause | env JIGC_ADAPTERS_DIR set |
| 71 | `setup-guide-target-declares-a-stamp-key` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: guide target clause | env JIGC_ADAPTERS_DIR set |
| 72 | `setup-guide-target-malformed-key` | content | `r1-setup-profile-clause-route-names-no-act` | same | adapter profile: guide target clause | env JIGC_ADAPTERS_DIR set |
| 73 | `setup-settings-a-directory-made-readable` | file-kind — directory in a file's place | `r1-p3-setup-writability-route-printed-for-other-causes` | same | directory at .claude/settings.json |  |
| 74 | `setup-compose-marker-packs-file-not-utf-8` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .jigc/config/packs.yaml not UTF-8 |  |
| 75 | `setup-compose-marker-packs-file-not-yaml` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .jigc/config/packs.yaml not YAML |  |
| 76 | `setup-compose-marker-packs-file-no-mapping` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .jigc/config/packs.yaml no mapping |  |
| 77 | `setup-compose-marker-packs-file-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 444 .jigc/config/packs.yaml |  |
| 78 | `setup-bootstrap-write-workbench-a-file` | file-kind — regular file in a directory's place | `r1-p3-setup-writability-route-printed-for-other-causes` | same | regular file at .jigc |  |
| 79 | `setup-bootstrap-write-root-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 . |  |
| 80 | `setup-bootstrap-write-workbench-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .jigc |  |
| 81 | `setup-reference-host-not-utf-8` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | CLAUDE.md not UTF-8 |  |
| 82 | `setup-reference-host-a-directory` | file-kind — directory in a file's place | `r1-p3-setup-writability-route-printed-for-other-causes` | same | directory at CLAUDE.md |  |
| 83 | `setup-reference-host-a-special-file` | file-kind — FIFO | `r1-setup-fifo-at-install-path-holds-the-command` | held | FIFO at CLAUDE.md |  |
| 84 | `setup-reference-host-absent-root-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 . |  |
| 85 | `setup-project-layer-config-a-file` | file-kind — regular file in a directory's place | `r1-p3-setup-writability-route-printed-for-other-causes` | same | regular file at .jigc/config |  |
| 86 | `setup-project-layer-no-config-workbench-read-only` | permission 1a | `r1-setup-workbench-made-writable-rerun-refuses-its-own-files` | other | chmod 555 .jigc |  |
| 87 | `setup-project-layer-marker-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 444 .jigc/config/.gitkeep |  |
| 88 | `setup-project-layer-no-marker-config-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .jigc/config |  |
| 89 | `setup-workbench-ignore-a-directory` | file-kind — directory in a file's place | `r1-p3-setup-writability-route-printed-for-other-causes` | same | directory at .jigc/.gitignore |  |
| 90 | `setup-workbench-ignore-not-utf-8` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .jigc/.gitignore not UTF-8 |  |
| 91 | `setup-workbench-ignore-absent-workbench-read-only` | permission 1a | `r1-setup-workbench-made-writable-rerun-refuses-its-own-files` | other | chmod 555 .jigc |  |
| 92 | `setup-workbench-ignore-short-and-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 444 .jigc/.gitignore |  |
| 93 | `setup-version-stamp-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 444 .jigc/version |  |
| 94 | `setup-seeded-ignore-not-utf-8` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .gitignore not UTF-8 | unborn HEAD |
| 95 | `setup-seeded-ignore-a-directory` | file-kind — directory in a file's place | `r1-p3-setup-writability-route-printed-for-other-causes` | same | directory at .gitignore | unborn HEAD |
| 96 | `setup-seeded-ignore-unreadable` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 000 .gitignore | unborn HEAD |
| 97 | `setup-seeded-ignore-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 444 .gitignore | unborn HEAD |
| 98 | `setup-settings-record-asked-of-the-reference-host` | file-kind — symlink to a file inside the repository | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .claude/settings.json -> ../CLAUDE.md |  |
| 99 | `setup-settings-merge-into-the-ignored-reference-host` | file-kind — symlink to a file inside the repository | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .claude/settings.json -> ../CLAUDE.md |  |
| 100 | `setup-settings-merge-no-file-directory-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .claude |  |
| 101 | `setup-settings-merge-no-directory-root-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 . |  |
| 102 | `setup-settings-record-read-only` | permission 1a | `r1-setup-settings-record-refused-rerun-installs-without-the-record` | other | chmod 444 .jigc/settings-entries.json |  |
| 103 | `setup-settings-record-absent-workbench-read-only` | permission 1a | `r1-setup-settings-record-refused-rerun-installs-without-the-record` | other | chmod 555 .jigc | state starts from an install the binary made |
| 104 | `setup-settings-record-to-remove-workbench-read-only` | permission 1a | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .jigc | state starts from an install the binary made |
| 105 | `setup-guide-write-no-directory-assistant-directory-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .claude |  |
| 106 | `setup-guide-write-no-file-directory-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .claude/skills/jigc |  |
| 107 | `setup-hook-write-hooks-path-empty` | git-state | `r1-p3-setup-writability-route-printed-for-other-causes` | same | core.hooksPath empty; git cannot resolve | core.hooksPath='' |
| 108 | `setup-hook-write-in-a-worktree-of-a-bare-repository` | git-state | `r1-setup-installs-outside-work-tree` | same | linked worktree of bare repository |  |
| 109 | `setup-hook-write-hooks-path-a-file` | file-kind — regular file in a directory's place (hooks directory, via core.hooksPath) | `r1-p3-setup-writability-route-printed-for-other-causes` | same | core.hooksPath names a file | core.hooksPath='hooks-file' |
| 110 | `setup-hook-write-hooks-path-dev-null` | file-kind — device in a directory's place (hooks directory = /dev/null, via core.hooksPath) | `r1-p3-setup-writability-route-printed-for-other-causes` | same | core.hooksPath = /dev/null | core.hooksPath='/dev/null' |
| 111 | `setup-hook-write-no-hooks-directory-parent-read-only` | permission 1c | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 ../outside/shut | core.hooksPath='../outside/shut/hooks' |
| 112 | `setup-hook-write-hook-not-utf-8` | content | `r1-p3-setup-writability-route-printed-for-other-causes` | same | .git/hooks/pre-commit not UTF-8 |  |
| 113 | `setup-hook-write-hook-a-directory` | file-kind — directory in a file's place | `r1-p3-setup-install-hook-undriven-producers-and-causes` | same | directory at .git/hooks/pre-commit |  |
| 114 | `setup-hook-write-hook-unreadable` | permission 1b | `r1-p3-setup-install-hook-undriven-producers-and-causes` | same | chmod 000 .git/hooks/pre-commit |  |
| 115 | `setup-hook-write-hook-a-special-file` | file-kind — FIFO | `r1-setup-fifo-at-install-path-holds-the-command` | held | FIFO at .git/hooks/pre-commit |  |
| 116 | `setup-hook-write-hook-read-only` | permission 1b | `r1-p3-setup-writability-route-printed-for-other-causes` | same | chmod 555 .git/hooks/pre-commit |  |
| 117 | `setup-change-hidden-by-skip-worktree-forced` | git-state | `r1-setup-force-over-skip-worktree-path-cannot-commit` | other | skip-worktree entry, file changed |  |
| 118 | `setup-unborn-staged-file-at-an-install-path-unstaged-as-printed-and-moved` | git-state | `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` | line | unborn HEAD, staged install path | unborn HEAD; d |
| 119 | `setup-unborn-install-path-staged-and-deleted-unstaged-as-printed` | git-state | `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` | line | unborn HEAD, staged install path | unborn HEAD; d |
| 120 | `setup-git-refuses-the-repository-asked-as-printed-and-resolved` | git-state | `r1-setup-unverified-route-opens-with-the-question-that-failed` | other | git fails: malformed .git/config | d |
| 121 | `setup-git-status-fails-asked-as-printed-and-resolved` | git-state | `r1-setup-unverified-route-opens-with-the-question-that-failed` | other | broken index (.git/index garbage) | d |
| 122 | `uninstall-home-not-set-run-as-the-route-prints` | environment | `r1-setup-home-unset-route-says-run-from-inside-the-repository` | same | HOME unset, inside a repository | env HOME unset |
| 123 | `uninstall-profile-not-in-the-adapters-directory` | environment | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | JIGC_ADAPTERS_DIR -> empty directory | env JIGC_ADAPTERS_DIR set |
| 124 | `uninstall-profile-not-utf-8` | content | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | adapter profile: not UTF-8 | env JIGC_ADAPTERS_DIR set |
| 125 | `uninstall-profile-malformed` | content | `r1-setup-profile-route-says-reinstall-under-adapters-dir` | same | adapter profile: no profile YAML | env JIGC_ADAPTERS_DIR set |
| 126 | `uninstall-worktree-path-unreadable-moved-out` | permission 1a | `r1-uninstall-worktree-route-move-aside-over-unreadable-path` | same | chmod 000 .jigc/worktrees/shut |  |
| 127 | `uninstall-worktree-path-unreadable-forced` | permission 1a | `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | other | chmod 000 .jigc/worktrees/shut |  |
| 128 | `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` | git-state | `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | line | untracked file in fan-out worktree | d |
| 129 | `uninstall-worktrees-root-unreadable-worktrees-removed-as-printed` | permission 1a | `r1-uninstall-worktrees-route-prints-git-worktree-remove-without-a-path` | same | chmod 000 .jigc/worktrees | d |
| 130 | `uninstall-worktrees-root-unreadable-forced` | permission 1a | `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | other | chmod 000 .jigc/worktrees |  |
| 131 | `uninstall-task-areas-unlistable-made-readable` | permission 1a | `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` | same | chmod 000 .jigc/tasks | d |
| 132 | `uninstall-task-areas-unlistable-forced` | permission 1a | `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | other | chmod 000 .jigc/tasks |  |
| 133 | `uninstall-workbench-unsearchable-made-readable` | permission 1a | `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` | same | chmod 000 .jigc | d |
| 134 | `uninstall-workbench-unsearchable-forced` | permission 1a | `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | other | chmod 000 .jigc |  |
| 135 | `uninstall-staged-doc-read-and-discarded-as-printed` | other | `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | line | open task's staged doc | d |
| 136 | `uninstall-staged-doc-finalized-as-printed` | other | `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | line | open task's staged doc | d |
| 137 | `uninstall-staged-docs-unsearchable-made-readable` | permission 1a | `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` | same | chmod 600 .jigc/tasks/write-a-note/docs | d |
| 138 | `uninstall-staged-docs-unsearchable-forced` | permission 1a | `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | other | chmod 600 .jigc/tasks/write-a-note/docs |  |
| 139 | `uninstall-untracked-workbench-file-added-as-printed` | git-state | `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | line | untracked file under .jigc/ | d |
| 140 | `uninstall-workbench-files-one-ignored-moved-out-and-one-added-as-printed` | git-state | `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | line | untracked file under .jigc/ | d |
| 141 | `uninstall-workbench-unlistable-forced` | permission 1a | `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | other | chmod 300 .jigc |  |
| 142 | `uninstall-git-index-unreadable-made-readable` | git-state | `r1-p3-uninstall-force-names-nothing-undecodable-listing` | same | broken index (.git/index garbage) |  |
| 143 | `uninstall-git-listing-undecodable-made-readable` | git-state | `r1-p3-uninstall-force-names-nothing-undecodable-listing` | same | index entry, non-UTF-8 path name |  |
| 144 | `uninstall-workbench-directory-read-only` | permission 1a | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 555 .jigc/config |  |
| 145 | `uninstall-workbench-under-a-read-only-root` | permission 1b | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 555 . |  |
| 146 | `uninstall-forced-workbench-directory-unreadable-made-writable` | permission 1a | `r1-p3-uninstall-writability-route-not-driven` | other | chmod 000 .jigc/worktrees | refusing command has --force |
| 147 | `uninstall-reference-host-read-only` | permission 1b | `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | other | chmod 444 CLAUDE.md |  |
| 148 | `uninstall-reference-host-a-directory` | file-kind — directory in a file's place | `r1-p3-uninstall-writability-route-not-driven` | same | directory at CLAUDE.md |  |
| 149 | `uninstall-reference-host-not-utf-8` | content | `r1-p3-uninstall-writability-route-not-driven` | same | CLAUDE.md not UTF-8 |  |
| 150 | `uninstall-reference-host-unreadable` | permission 1b | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 000 CLAUDE.md |  |
| 151 | `uninstall-reference-host-a-special-file` | file-kind — FIFO | `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command` | held | FIFO at CLAUDE.md |  |
| 152 | `uninstall-reference-host-a-link-to-a-read-only-file-outside` | permission 1c | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 444 ../outside/CLAUDE.md | reached through a symlink |
| 153 | `uninstall-settings-read-only` | permission 1b | `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | other | chmod 444 .claude/settings.json |  |
| 154 | `uninstall-settings-a-directory` | file-kind — directory in a file's place | `r1-p3-uninstall-writability-route-not-driven` | same | directory at .claude/settings.json |  |
| 155 | `uninstall-settings-not-utf-8` | content | `r1-p3-uninstall-writability-route-not-driven` | same | .claude/settings.json not UTF-8 |  |
| 156 | `uninstall-settings-not-json` | content | `r1-p3-uninstall-writability-route-not-driven` | same | .claude/settings.json not JSON |  |
| 157 | `uninstall-settings-unreadable` | permission 1b | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 000 .claude/settings.json |  |
| 158 | `uninstall-settings-a-special-file` | file-kind — FIFO | `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command` | held | FIFO at .claude/settings.json |  |
| 159 | `uninstall-session-hook-settings-read-only` | permission 1b | `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | other | chmod 444 .claude/settings.json |  |
| 160 | `uninstall-deny-floor-settings-read-only` | permission 1b | `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | other | chmod 444 .claude/settings.json |  |
| 161 | `uninstall-hook-hooks-path-empty` | git-state | `r1-p3-uninstall-writability-route-not-driven` | same | core.hooksPath empty; git cannot resolve | core.hooksPath='' |
| 162 | `uninstall-hook-hooks-path-a-file` | file-kind — regular file in a directory's place (hooks directory, via core.hooksPath) | `r1-p3-uninstall-writability-route-not-driven` | same | core.hooksPath names a file | core.hooksPath='../outside/hooks-file' |
| 163 | `uninstall-hook-hooks-path-dev-null` | file-kind — device in a directory's place (hooks directory = /dev/null, via core.hooksPath) | `r1-p3-uninstall-writability-route-not-driven` | same | core.hooksPath = /dev/null | core.hooksPath='/dev/null' |
| 164 | `uninstall-hook-a-directory` | file-kind — directory in a file's place | `r1-p3-uninstall-writability-route-not-driven` | same | directory at .git/hooks/pre-commit |  |
| 165 | `uninstall-hook-not-utf-8` | content | `r1-p3-uninstall-writability-route-not-driven` | same | .git/hooks/pre-commit not UTF-8 |  |
| 166 | `uninstall-hook-unreadable` | permission 1b | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 000 .git/hooks/pre-commit |  |
| 167 | `uninstall-hook-a-special-file` | file-kind — FIFO | `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command` | held | FIFO at .git/hooks/pre-commit |  |
| 168 | `uninstall-hook-read-only` | permission 1b | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 444 .git/hooks/pre-commit |  |
| 169 | `uninstall-hook-a-link-to-a-read-only-script-outside` | permission 1c | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 444 ../outside/pre-commit | reached through a symlink |
| 170 | `uninstall-guide-directory-read-only` | permission 1b | `r1-p3-uninstall-writability-route-not-driven` | same | chmod 555 .claude/skills/jigc |  |

## 4. The cases I was least sure of

None of these is a doubt about what the state *is* — the steps are in the tables. Each is a doubt about which bucket the state belongs in. The number in brackets is the count of red cases the doubt touches; § 3 numbers in parentheses.

1. **A link at the root `.gitignore`, on an unborn `HEAD` [8]** (48–55). Two kinds stacked: an unborn branch (git-state) and a symbolic link (file-kind). Put in `file-kind`: the site is `merged_through_links` and the cause's object is the link; the unborn `HEAD` is what makes setup reach for the root `.gitignore` at all. Counted the other way, file-kind is 68 and git-state 21. The same stacking, decided the same way, holds for (94) not-UTF-8 → content, (95) a directory → file-kind, (96, 97) `chmod` → 1b.
2. **The adapter profile [19]** (57–72, 123–125). All 19 are reached only with `JIGC_ADAPTERS_DIR` set to a directory of the case's own — the table's header says no other way reaches those sites. 17 went to `content` (the profile there is not UTF-8, is no profile, or a clause of it is refused — the brief names "a profile with a missing clause" under content) and 2 to `environment` (the directory holds no profile at all; "a missing file" would make them content too). Counted by how the state is reached, all 19 are environment: environment 21, content 11.
3. **`core.hooksPath` as the maker of the state [6]**. Empty string (107, 161) → `git-state`, read as "git failing": git refuses to resolve a hooks directory. It is a configuration value rather than something git reports about a path; `other: a git configuration value` would hold it as well. Naming a regular file (109, 162) or `/dev/null` (110, 163) → `file-kind`, read as "a path jigc expects as a directory is something else" — but nothing on disk was replaced; a `git config` line pointed the hooks directory at it. Nine more cases carry `core.hooksPath` only as an ordinary precondition (14–21 a link in the named directory; 111 a `chmod` outside) and are not in doubt.
4. **A link to a read-only file outside the repository [2]** (152, 169). Link and permission stacked. Put in `1c` because the cause says *may not be written* and the `chmod` is on the file outside. No case has the same link with a writable target, so the tables do not separate the two; by the link they would be file-kind (symlink to a file outside).
5. **The settings path a committed link to `CLAUDE.md` [2]** (98, 99). `file-kind`, symlink to a file inside the repository — the only two red links whose target is an ordinary file of the repository. The link itself is one the install follows; what the step refuses over is that two members of the install are then one file, which the reference step has already written. `other: two install paths are one file` would be as true.
6. **An open task's staged doc [2]** (135, 136) → `other`. The whole state beyond the install is one `jigc start --workflow decided-task "…"`: no permission, no odd file, nothing git reports (the doc is under `.jigc/tasks/`), no variable, no malformed content. "A dirty path" in a loose sense, but jigc's own, not git's.
7. **Work in no commit under `.jigc/` [3]** (128, 139, 140) → `git-state`, as untracked paths: a file written into a fan-out worktree the product provisioned (a linked worktree, made by three `jigc milestone` verbs), and one or two files written under `.jigc/config/`. They sit next to item 6 in kind — work in progress — and differ from it only in that git is who reports them.
8. **Git made to fail by a file planted inside `.git/` [4]** (120 a `.git/config` that does not parse; 121, 142 a `.git/index` that is no index; 143 an index entry whose path name is not UTF-8). `git-state` by the brief's own wording (*a broken or unreadable index, git failing*). As planted they are malformed content of files in git's directory.
9. **A regular file where a directory goes [2]** (78 `.jigc`, 85 `.jigc/config`) → `file-kind`. The brief's wording covers it; it is the reverse of the "directory in a file's place" it lists.
10. **Sub-bucket 1a is by path, not by what git ignores [24].** Every 1a case has its `chmod` on `.jigc` or below. In the setup cases the files there are *committed* by the state (`.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/…`, `.jigc/settings-entries.json`), so "the gitignored workbench" describes the path's role and not its status in those sandboxes. And **20 setup cases stand on a `.jigc/` the state plants by hand before any `jigc setup` has run** (9 of the 11 setup 1a cases, 4 file-kind, 4 content, 2 of 1b, 1 git-state); only two setup cases (103, 104) start from an install the binary made.
11. **The hook file under `.git/hooks/` [4]** (114, 116, 166, 168) → `1b`, because the brief lists the hooks directory there. It is in git's directory, not a tracked file.
12. **`setup-workbench-ignore-short-and-read-only` [1]** (92). Permission, with a content precondition: the file must lack an entry, or setup would not write it and the bit would not matter.
13. **`uninstall-forced-workbench-directory-unreadable-made-writable` [1]** (146). The refusing command is `uninstall --force`; the table's cause says the guards refuse the state from every other run. Bucket `1a` like its siblings; it is the one red case whose refusal needs a flag as well as a state.

**Where the name would have misfiled a case.** `uninstall-git-index-unreadable-made-readable` plants a garbage `.git/index` and changes no permission (git-state, not permission). `setup-settings-a-directory-made-readable` is a directory at the settings path (file-kind). The suffixes `-made-readable`, `-made-writable`, `-forced`, `-removed`, `-pointed-at-…` name the *act* that follows the route, never the state: of the 50 red cases whose name contains `made-readable`, `made-writable`, `read-only`, `unreadable`, `unlistable` or `unsearchable`, the state is a permission in 47 — all 47 permission cases — and something else in 3 (the two above and `uninstall-git-listing-undecodable-made-readable`) The names were a close guide here and still wrong three times; the buckets were read off the steps.
