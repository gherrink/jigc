# The route catalog's pilot — the findings its cases made, by key

What a case of the route catalog ([route_catalog.rs](../../../../../crates/cli/tests/route_catalog.rs), cases in [route_catalog/setup.rs](../../../../../crates/cli/tests/route_catalog/setup.rs)) came out red with, where the small canary's file ([canary-one/findings.md](../canary-one/findings.md)) has no row for it. A key here is in that file's grammar and is what the suite's expected-red list names; the classification is the frame's ([route_case.rs](../../../../../crates/cli/tests/support/route_case.rs)). **Nothing here is a ruling, a grade or a word about the previous release**: each row is what one binary did on one day — this tree's debug build, macOS arm64, 2026-10-11 — when the printed route was followed by its own words. Sandbox paths are written `<sandbox>/…`.

**No ledger holds these keys.** Until a stabilization run is opened they certify themselves, as the canary's do for the cases listed under them ([DECISIONS.md](../../../../../DECISIONS.md) → *The route catalog's frame*).

**Three of the keys are about a state only a variable reaches** — `JIGC_ADAPTERS_DIR` for two, `$HOME` taken away for one. Whether a route owes anything there is not said here.

**The cases are those of every blocking site of `jigc setup`** — thirty-six sites, four of them with a reason and no case; `jigc uninstall` has none yet.

**A case's name says its cause and its way.** The cases of one cause are a group, and a name ends with the way the route was followed: `-removed`, `-pointed-at-a-committed-file`, `-pointed-at-a-new-file`, and each of those again with `-committed-and-` in front where the link was committed and the change was committed as the route's clause says.

## 1. Keyed — cases are listed under each

The three keys about a link at a file the install merges into share one printed route (over `CLAUDE.md`; the same words over `.claude/settings.json` and over the root `.gitignore`):

```text
route: remove the link at `CLAUDE.md`, or point it at a regular file inside this repository — committing the change where git tracks the link — then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link
```

The first way out — remove the link — holds at every end a link was driven at, with the link untracked and with it committed. Everything below is on the second.

### `r1-setup-untracked-link-repointed-draws-dirty-install-path`

- **Classification:** followed, another failure. **Sixteen cases:** `setup-host-link-<end>-pointed-at-a-committed-file` and `-pointed-at-a-new-file`, and the same two under `setup-settings-link-<end>-`, for each of the four ends: `to-nothing`, `out-of-the-repository`, `to-a-directory`, `into-gits-directory`.
- **State:** a repository with one commit and no `jigc setup`; `CLAUDE.md` or `.claude/settings.json` is a link the install will not follow, and git does not track the link.
- **Refusing command:** `jigc setup` — exit 1, `setup.inject-reference` or `setup.inject-allowlist`.
- **What was done:** the link pointed at a regular file inside the repository — the repository's own committed file, or one written a moment before; no commit, since git does not track the link; `jigc setup` again.
- **What went wrong:** the re-run exits 1 with `setup.dirty-install-path`, naming the link itself (and the new file, where there is one) as work in no commit — so on a born `HEAD` the route's second way out leads into a second refusal whenever git does not track the link, whatever the link led to before; the route's clause about committing does not reach that link.
- **Beside it, and green:** the same act where git tracks the link, the file it is pointed at is committed and the change is committed (`…-committed-and-pointed-at-a-committed-file`, eight cases).

### `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path`

- **Classification:** followed, another failure. **Marked `doubt`** — every case rests on a reading the human has not given. **Eight cases:** `setup-host-link-<end>-committed-and-pointed-at-a-new-file` and the same under `setup-settings-link-<end>-`, four ends each.
- **State:** as above, and the link is committed.
- **What was done:** the link pointed at a regular file inside the repository that was written a moment before and is in no commit; the link's change committed, as the route says where git tracks the link; `jigc setup` again.
- **What went wrong:** the re-run exits 1 with `setup.dirty-install-path` over the file the link now leads to — the route says *a regular file inside this repository* and the install takes only one git already holds.
- **The two readings:** (a) any regular file in the checkout, one in no commit included — followed here, red; (b) a file the repository already holds committed — `…-committed-and-pointed-at-a-committed-file`, green.

### `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor`

- **Classification:** followed, another failure — the re-run exits 0, and what must hold afterwards does not: git does not ignore `.env`, which the secrets floor lists. **Eight cases:** `setup-seeded-ignore-link-<end>-pointed-at-a-new-file` and `-staged-and-pointed-at-a-new-file`, four ends each.
- **State:** a repository whose `HEAD` is unborn — where `jigc setup` seeds the root `.gitignore` — and that file is a link the install will not follow. In the second four the link is staged.
- **Refusing command:** `jigc setup` — exit 1, `setup.secrets-gitignore`, the route above over `.gitignore`.
- **What was done:** the link pointed at a regular file inside the repository; in the second four the change committed, as the clause says of a link git tracks; `jigc setup` again — exit 0.
- **What went wrong, twice over.** Where the link is left uncommitted, `jigc setup` merges its secrets floor through the link into the file, and git does not read an ignore file through a link: it warns `unable to access '.gitignore': Too many levels of symbolic links` and ignores nothing the floor lists (git 2.54.0). Where the change is committed as the route says, that commit is the repository's first — `HEAD` is born, the root `.gitignore` is no member of the install any more, and no floor is seeded at all.
- **Beside it, and green:** the first way out. With the link removed, the re-run seeds a regular `.gitignore` and git ignores `.env` (four cases).
- **What the check is.** `git check-ignore -q .env` after the route is followed — asked of git, of a path the floor lists. No route prints it: it is the outcome the refused member exists for, as *the install's file is a regular file* is for the cases beside it.

### `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path`

- **Classification:** followed, another failure. **Marked `doubt`.** **Three cases:** `setup-hook-link-to-nothing-pointed-at-a-new-script`, `setup-hooks-path-inside-link-to-nothing-pointed-at-a-new-script`, `setup-hooks-path-outside-link-to-nothing-pointed-at-a-new-script`.
- **State:** the `pre-commit` hook — in git's default hooks directory, or where `core.hooksPath` names one inside or outside the repository — is a link to a path that holds nothing.
- **Refusing command:** `jigc setup` — exit 1, `setup.install-hook`.
- **Printed route** (default hooks directory):

  ```text
  route: remove the link at `<sandbox>/repo/.git/hooks/pre-commit`, or point it at a script inside this repository, then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link
  ```

- **What was done:** the link pointed at a script inside the repository that was written a moment before and is in no commit; `jigc setup` again.
- **What went wrong:** the re-run exits 1 with `setup.dirty-install-path` over the script — and this refusal comes **after** the install's writes: it says *the install files are written and staged, and no install commit was made*, and the script the link was pointed at now carries jigc's block. The hook's route has no clause about committing at all.
- **The two readings:** (a) any script in the checkout, one in no commit included — followed here, red; (b) a script the repository already holds committed — `…-pointed-at-a-committed-script`, green in all three.

### `r1-setup-fifo-at-install-path-holds-the-command`

- **Classification:** the command did not end. **Four cases:** `setup-compose-marker-a-special-file` (`.jigc/config/packs.yaml`), `setup-guide-a-special-file` (`.claude/skills/jigc/SKILL.md`), `setup-reference-host-a-special-file` (`CLAUDE.md`), `setup-hook-write-hook-a-special-file` (`.git/hooks/pre-commit`).
- **State:** a repository with one commit and no install; a FIFO at the path.
- **Refusing command:** `jigc setup` — it prints nothing and does not end; the runner ends it at its limit. No route is printed, so none is quoted.
- **What went wrong:** `jigc setup` reads each of the first two paths before it reaches the refusal that names a special file there, and the read of a FIFO with no writer does not return. At the other four replaced paths (`.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/settings-entries.json`) the same FIFO is refused at once, with a route that holds.
- **The third and the fourth are the same read at a file the install merges into**, where no refusal names a special file at all: `CLAUDE.md` is read by the reference step (`inject_reference`), after the bootstrap file is written, so the hold leaves `.jigc/AGENT.md` behind it; and the `pre-commit` hook is read by the last step of the write span (`install_precommit_hook`), so there the hold leaves every other install file written and none committed. Both sampled: the command held in `std::fs::read_to_string` under `write_install_span`.
- **Not under this key — a FIFO at an ignore file.** With `.jigc/.gitignore`, or the root `.gitignore` of a repository with no commit, a FIFO, `jigc setup` does not end either; there it is `git status` that does not return (git reads a file of that name as an ignore file), and `jigc setup` waits for it without a limit. No `git` command ends in such a repository. No case: each is a reason at its site.
- **Kin, at another door:** the canary's `r1-p4-named-pipe-at-managed-home-holds-reading-doors`.

### `r1-setup-home-unset-route-says-run-from-inside-the-repository`

- **Classification:** followed, the same refusal again. **One case:** `setup-home-not-set-run-as-the-route-prints`.
- **State:** a repository with one commit and no install; `$HOME` is not in the command's environment.
- **Refusing command:** `jigc setup`, run at the repository's root — exit 1, `setup.repo-root`: *cannot locate the repository root: cannot resolve the team config path: $HOME is not set*.
- **Printed route:**

  ```text
  route: run `jigc setup` from inside the target git repository
  ```

- **What was done:** `jigc setup`, from inside the repository — where it had been run — in the same environment.
- **What went wrong:** the same refusal: the code and the route are about where the command stands, and what is wrong is a variable, which neither names a way to set. The source declares it (`locate_finding`'s comment: *a law-1 wobble this task did not own*).

### `r1-setup-profile-route-says-reinstall-under-adapters-dir`

- **Classification:** followed, the same refusal again. **Four cases:** `setup-profile-not-in-the-adapters-directory`, `setup-profile-not-utf-8`, `setup-profile-malformed` (`setup.profile-load`); `setup-profile-declares-no-reference` (`setup.profile-incomplete`).
- **State:** a repository with one commit and no install; `JIGC_ADAPTERS_DIR` names a directory — `<sandbox>/outside/adapters` — that holds no `claude-code.yaml`, or one that is not UTF-8, or YAML that is no profile, or a profile with no reference among its inject targets. **No other state reaches these two refusals**: without the variable the profile is the binary's own, which loads and is complete.
- **Refusing command:** `jigc setup` — exit 1.
- **Printed routes:**

  ```text
  route: reinstall jigc — the embedded adapter profile is missing or malformed
  route: reinstall jigc — the embedded adapter profile is missing its bootstrap reference
  ```

- **What was done:** the command again. *Reinstall jigc* puts the same release's binary where the binary already is, and the route prints no command line; the profile's bytes are the case's witness and did not change.
- **What went wrong:** the same refusal. The profile that was refused is the directory's; the route blames the embedded one, which was not read, and neither the message nor the route names the variable or the file.

### `r1-setup-profile-clause-route-names-no-act`

- **Classification:** followed, the same refusal again — **eleven cases**: `setup-launch-template-without-the-task-placeholder`, `-without-the-workflow-placeholder`, `-of-three-lines`, `-without-a-backticked-span`, `-with-a-forbidden-marker`, `-with-other-shell` (`setup.spawn-template`); `setup-guide-target-without-a-path`, `-absolute`, `-climbing-out`, `-declares-a-stamp-key`, `-malformed-key` (`setup.guide-target`). And followed, another failure — **one case, marked `doubt`**: `setup-launch-template-without-the-invocation`.
- **State:** as above, and the profile in the directory loads; one clause of its launch template, or of its guide target, is refused. Reached through `JIGC_ADAPTERS_DIR` and no other way.
- **Refusing command:** `jigc setup` — exit 1.
- **Printed route** — the violated clause, which is also the end of the message above it; three of the twelve:

  ```text
  route: spawn template missing the required `{{task_id}}` placeholder
  route: the guide target's `file` must be repo-relative, not absolute
  route: spawn template missing the required `jigc workflow … --task …` invocation
  ```

- **What was done:** the command again, in eleven. In the twelfth the route's one backticked span is a `jigc` line, and it was run as printed: exit 1, `work-unit.malformed-id` over `…`.
- **What went wrong:** the route names no act, no file and no command. What a reader needs — which file holds the profile — is in neither line.
- **The two readings, of the twelfth:** (a) the backticked `jigc workflow … --task …` is a command to run — followed here, red; (b) it is the shape the template must hold, and the route prints nothing to run — then it is one more of the eleven.

### `r1-setup-workbench-made-writable-rerun-refuses-its-own-files`

- **Classification:** followed, another failure. **Two cases:** `setup-project-layer-no-config-workbench-read-only`, `setup-workbench-ignore-absent-workbench-read-only`.
- **State:** a repository with one commit and no install; `.jigc` holds a committed `.jigc/AGENT.md` (in the second case `.jigc/config/.gitkeep` as well) and may not be written — mode 555.
- **Refusing command:** `jigc setup` — exit 1, `setup.init-project-layer`: *Permission denied*.
- **Printed route:**

  ```text
  route: ensure `.jigc/` is writable, then re-run `jigc setup`
  ```

- **What was done:** `.jigc` given its owner's write bit; `jigc setup` again.
- **What went wrong:** the re-run exits 1 with `setup.dirty-install-path` over `.jigc/AGENT.md` and `CLAUDE.md` — *work `jigc setup` did not write* — which are the two files the refused run wrote before it reached the step that failed. A run that fails inside the write span records what it wrote so that the next run does not take it for the adopter's (`record_failed_install`), and that record is a file under `.jigc/state/`: where `.jigc` is what may not be written, it is not kept. So in the one state this route's words are true of, following it leads into a second refusal, whose own route says *commit or stash the work* over files nobody but jigc wrote.
- **Beside it, and green:** `setup-version-stamp-absent-workbench-read-only` — the same shut `.jigc` over an install that stands, where the steps before the stamp rewrite their files to the bytes git already holds and leave nothing to refuse.

### `r1-setup-settings-record-refused-rerun-installs-without-the-record`

- **Classification:** followed, another failure. **Two cases:** `setup-settings-record-read-only`, `setup-settings-record-absent-workbench-read-only`.
- **State:** a repository with one commit; the settings record, `.jigc/settings-entries.json`, cannot be written — a committed file there of mode 444 in the first case; in the second an install that stands with its settings file and its record taken out and that committed, and `.jigc` of mode 555.
- **Refusing command:** `jigc setup` — exit 1, `setup.inject-allowlist`: *cannot write `.jigc/settings-entries.json`, the record of the entries `jigc setup` added to `.claude/settings.json`: Permission denied*. By then the three merges have run: the entries are in the settings file.
- **Printed route:**

  ```text
  route: ensure `.jigc/settings-entries.json` is writable, then re-run `jigc setup`
  ```

- **What was done:** the file given its owner's write bit (in the second case there is no such file, and the act could not be done); `jigc setup` again — exit 0.
- **What went wrong:** the install the re-run completes has no settings record. The re-run finds every entry already in the settings file, so it adds none and records none — in the first case it removes the file that stood there and says *this run added none*. The entries were added by the run that was refused. What must hold is the file the refused step exists to write, as for every case beside these; without it `jigc uninstall` leaves jigc's entries in the settings file.
- **The source declares the half of this it chose** (`write_install_span`, step 4b′): *a run that fails between them and here leaves entries no record claims, which the teardown leaves in place and names — never the reverse*. Whether the route may then say *re-run* and nothing more is not said here.

### `r1-setup-force-over-skip-worktree-path-cannot-commit`

- **Classification:** followed, another failure. **One case:** `setup-change-hidden-by-skip-worktree-forced`.
- **State:** a repository with one commit more, which holds `.jigc/AGENT.md`; the file's index entry is flagged skip-worktree, and the file is changed.
- **Refusing command:** `jigc setup` — exit 1, `setup.dirty-install-path`: the path is *tracked, with the change hidden by its index entry*.
- **Printed route:**

  ```text
  route: let git see the change at `.jigc/AGENT.md` with `git -C <sandbox>/repo update-index --no-assume-unchanged -- .jigc/AGENT.md` and `git -C <sandbox>/repo update-index --no-skip-worktree -- .jigc/AGENT.md` (one call clears only one of the two flags), and commit or stash it — then re-run `jigc setup`; `jigc setup --force` is the single consent, and it lets the install run and commit those paths as it leaves them — which at a path jigc regenerates whole is jigc's own content, not yours
  ```

- **What was done:** the route's last way out — `jigc setup --force`.
- **What went wrong:** exit 1 with `setup.install-commit`: *the jigc install files were written, but `git add` was rejected* — git does not stage a path its sparse-checkout rules skip, and says so. The route says the consent *lets the install run and commit those paths*: the install ran — `.jigc/AGENT.md` holds jigc's text now, and the change that stood there is in no commit — and nothing was committed.
- **Beside it, and green:** the route's first way — both flag lines run, the change committed, the command again (`setup-change-hidden-by-skip-worktree-flags-cleared-and-committed`); and the same consent over an entry flagged assume-unchanged (`setup-change-hidden-by-assume-unchanged-forced`), which git stages.

### `r1-setup-unverified-route-opens-with-the-question-that-failed`

- **Classification:** followed, another failure. **Marked `doubt`** — both cases rest on a reading the human has not given. **Two cases:** `setup-git-refuses-the-repository-asked-as-printed-and-resolved`, `setup-git-status-fails-asked-as-printed-and-resolved`.
- **State:** a repository with one commit and no install, in which git cannot answer: its `.git/config` is no configuration file in the first case, its `.git/index` no index in the second.
- **Refusing command:** `jigc setup` — exit 1, `setup.unverified-install-path`, with git's own words quoted.
- **Printed route** (the first case; the second prints `git -C <sandbox>/repo status`):

  ```text
  route: run `git -C <sandbox>/repo rev-parse --is-inside-work-tree` to see what git reports, resolve that (or put `git` on `PATH`), then re-run `jigc setup`. `--force` does not change this: it consents to the paths git names, and git named none
  ```

- **What was done:** the printed `git` line run — exit 128, with the words the refusal had quoted; the file git's words name removed; `jigc setup` again — exit 0, the install complete.
- **What went wrong:** nothing but the first step. The frame holds every command a route prints to exit 0, and this one is the question that failed, printed so that the reader sees it fail.
- **The two readings:** (a) the printed `git` line is a step of the route like any other — followed here, red; (b) it is there to show git's words, and the route's steps are *resolve that* and the re-run — `…-resolved`, two cases, *followed, holds, operand implied*: what resolves the failure is in git's words and never in the route.

## 2. Under the canary's keys — the hook's link refusal

Not minted here; listed so that the count is in one place. The route of `setup.install-hook` names **where the link leads** as *the link* wherever the link leads anywhere: `display_hook_path` resolves the link before it words the path.

| the canary's key | cases | classification |
|---|---|---|
| `r1-setup-install-hook-route-other-causes-unexamined` | 3 — the link leads to a script outside the repository (default hooks directory; `core.hooksPath` inside the repository; `core.hooksPath` outside it), followed by removing what the route names | followed, a witness changed |
| | 9 — the same three by the route's second way, new script and committed; and a link to a directory outside the repository, all three ways | followed, the same refusal again |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | 1 — the link spelled relatively, removed | followed, a witness changed |
| | 2 — the same by the second way | followed, the same refusal again |
| `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | 3 — a link to `/dev/null`, all three ways: the runner does nothing outside its sandbox | followed, the same refusal again |

Where the link leads to nothing the route names the link itself, and removing it holds in all three hooks directories — and with the link committed, under a hooks directory inside the repository.

**And the hook's write, behind the link refusal** — the canary's own site for three of its keys. The route of every one is the one line, which prints no path: ``route: ensure the repo's git hooks directory is writable, then re-run `jigc setup` ``; the hooks directory git resolves was given its owner's bit, and the command run again. Each case is *followed, the same refusal again*.

| the canary's key | cases at the hook's write |
|---|---|
| `r1-p3-setup-writability-route-printed-for-other-causes` | 6 — the canary's four causes: `core.hooksPath` names `/dev/null` (`setup-hook-write-hooks-path-dev-null`), names a regular file (`-hooks-path-a-file`), the hook that stands may not be written in a hooks directory that may (`-hook-read-only`), is not UTF-8 (`-hook-not-utf-8`); and two the canary did not drive — `core.hooksPath` is the empty string, which git refuses to resolve (`-hooks-path-empty`), and there is no hooks directory under a directory that may not be written (`-no-hooks-directory-parent-read-only`) |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | 2 — a directory stands at the hook's path (`setup-hook-write-hook-a-directory`); the hook may not be read, mode 000 (`-hook-unreadable`) |
| `r1-setup-installs-outside-work-tree` | 1 — `jigc setup` run in a worktree of a bare repository (`setup-hook-write-in-a-worktree-of-a-bare-repository`): the install home is the folder that holds the repository, git resolves no hooks directory there, and the install's other files are written into that folder before the refusal |

**Green there:** `setup-hook-write-no-hook-hooks-directory-read-only` — no hook, and the hooks directory itself read-only: the one state the route's words are true of (*holds, operand implied*).

## 3. Under the canary's key — a route that asks for a permission where the cause is none

Not minted here. `r1-p3-setup-writability-route-printed-for-other-causes` is the canary's row for the hook's write: *ensure the repo's git hooks directory is writable* over causes that are no such thing. **The same defect at eleven more sites** — and at the hook's write six cases, in §2. First at two, five cases, each *followed, the same refusal again*: the entry the route names was given its owner's bit — it had it — and the command was run again.

| case | refusing — `jigc setup`, exit 1 | printed route | the cause |
|---|---|---|---|
| `setup-settings-a-directory-made-readable` | `setup.inject-allowlist`: *Is a directory* | ``route: ensure `.claude/settings.json` is readable, then re-run `jigc setup` `` | a directory stands at the settings file's path |
| `setup-compose-marker-packs-file-not-utf-8` | `setup.compose-marker`: *stream did not contain valid UTF-8* | ``route: ensure `.jigc/config/` is writable, then re-run `jigc setup` `` | the committed `packs.yaml` is not text |
| `setup-compose-marker-packs-file-not-yaml` | `setup.compose-marker`: a parser's line and column | the same | it is no YAML |
| `setup-compose-marker-packs-file-no-mapping` | `setup.compose-marker`: *is not a YAML mapping* | the same | it is YAML and no mapping |
| `setup-compose-marker-packs-file-read-only` | `setup.compose-marker`: *Permission denied* | the same | the file may not be written; the directory the route names may |

**Beside them, and green:** `setup-compose-marker-config-directory-read-only` — the directory itself read-only and no `packs.yaml` in it yet, the one state the route's words are true of. And the settings file's eight other causes, sixteen cases: *fix it so jigc can merge into it, commit the fix* holds for every shape the merge refuses.

**And at nine more sites — the other steps of the write span** (`write_install_span`), twenty-four cases, each *followed, the same refusal again*. Each step words every error of its callee under one route, which names one entry to make writable — for one of them readable. Where the cause is anything else — what stands at the path, what its bytes are, or the permission of another entry than the one named — the act changes nothing, or cannot be done, and the re-run is refused the same way.

| step — `jigc setup`, exit 1 | printed route | the causes it was printed for, a case each |
|---|---|---|
| `setup.write-bootstrap` | ``route: ensure `.jigc/AGENT.md` is writable, then re-run `jigc setup` `` | `.jigc` is a regular file (`setup-bootstrap-write-workbench-a-file`) · there is no `.jigc` and the repository root may not be written (`-root-read-only`) · there is no `.jigc/AGENT.md` and `.jigc` may not be written (`-workbench-read-only`) |
| `setup.inject-reference` | ``route: ensure `CLAUDE.md` is writable, then re-run `jigc setup` `` | the committed `CLAUDE.md` is not UTF-8 (`setup-reference-host-not-utf-8`) · a directory stands there (`-a-directory`) · there is none and the repository root may not be written (`-absent-root-read-only`) |
| `setup.init-project-layer` | ``route: ensure `.jigc/` is writable, then re-run `jigc setup` `` | `.jigc/config` is a regular file (`setup-project-layer-config-a-file`) · the committed `.gitkeep` may not be written (`-marker-read-only`) · there is none and `.jigc/config` may not be written (`-no-marker-config-read-only`) · a directory stands at `.jigc/.gitignore` (`setup-workbench-ignore-a-directory`) · it is not UTF-8 (`-not-utf-8`) · it lacks an entry and may not be written (`-short-and-read-only`) |
| `setup.version-stamp` | the same | the committed `.jigc/version` may not be written (`setup-version-stamp-read-only`) |
| `setup.secrets-gitignore` | ``route: ensure the repository root is writable, then re-run `jigc setup` `` | on an unborn `HEAD`, the root `.gitignore` is not UTF-8 (`setup-seeded-ignore-not-utf-8`) · a directory (`-a-directory`) · may not be read (`-unreadable`) · may not be written (`-read-only`) |
| `setup.inject-allowlist`, the read the record is computed from | ``route: ensure `.claude/settings.json` is readable, then re-run `jigc setup` `` | the settings path is a committed link to `CLAUDE.md`, to which the install has just appended its reference (`setup-settings-record-asked-of-the-reference-host`) |
| `setup.inject-allowlist`, the merge | ``route: ensure `.claude/settings.json` is writable, then re-run `jigc setup` `` | the same link where git ignores `CLAUDE.md` and no record is kept (`setup-settings-merge-into-the-ignored-reference-host`) · there is no settings file and `.claude` may not be written (`-no-file-directory-read-only`) · there is no `.claude` and the repository root may not be written (`-no-directory-root-read-only`) |
| `setup.inject-allowlist`, the record's write | ``route: ensure `.jigc/settings-entries.json` is writable, then re-run `jigc setup` `` | the run records nothing, a record stands, and `.jigc`, which it is to be taken out of, may not be written (`setup-settings-record-to-remove-workbench-read-only`) |
| `setup.write-guide` | ``route: ensure `.claude/skills/jigc/SKILL.md` is writable, then re-run `jigc setup` `` | there is no `.claude/skills` and `.claude` may not be written (`setup-guide-write-no-directory-assistant-directory-read-only`) · there is no guide and `.claude/skills/jigc` may not be written (`-no-file-directory-read-only`) |

Where the settings path leads to `CLAUDE.md`, the re-run is refused under the same code at another site — the question before the first write, with its own route, *fix it so jigc can merge into it*: the file is `CLAUDE.md` with the reference in it.

**Beside them, and green — the state each route's words are true of:** `setup-bootstrap-write-file-read-only`, `setup-reference-host-read-only`, `setup-settings-merge-file-read-only`, `setup-guide-write-file-read-only` (the named file may not be written); `setup-version-stamp-absent-workbench-read-only` (`.jigc` may not be written, over an install that stands); `setup-seeded-ignore-absent-root-read-only` (the root may not be written — *holds, operand implied*: the route prints no path for the root). Two more states the words are true of are red under keys of their own, in §1: a shut `.jigc` over a first install, and the record's own write.

**After each of the four refusals at the marker's write, the install is half written**: `.jigc/AGENT.md`, `.jigc/.gitignore`, `.jigc/config/.gitkeep` and `CLAUDE.md` are written and staged, and no install commit is made. The settings file's refusal comes before the first write and leaves nothing.
