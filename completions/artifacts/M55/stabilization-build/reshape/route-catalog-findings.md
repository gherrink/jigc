# The route catalog's pilot — the findings its cases made, by key

What a case of the route catalog ([route_catalog.rs](../../../../../crates/cli/tests/route_catalog.rs), cases in [route_catalog/setup.rs](../../../../../crates/cli/tests/route_catalog/setup.rs) and [route_catalog/uninstall.rs](../../../../../crates/cli/tests/route_catalog/uninstall.rs)) came out red with, where the small canary's file ([canary-one/findings.md](../canary-one/findings.md)) has no row for it. A key here is in that file's grammar and is what the suite's expected-red list names; the classification is the frame's ([route_case.rs](../../../../../crates/cli/tests/support/route_case.rs)). **Nothing here is a ruling, a grade or a word about the previous release**: each row is what one binary did on one day — this tree's debug build, macOS arm64, 2026-10-11 — when the printed route was followed by its own words. Sandbox paths are written `<sandbox>/…`.

**No ledger holds these keys.** Until a stabilization run is opened they certify themselves, as the canary's do for the cases listed under them ([DECISIONS.md](../../../../../DECISIONS.md) → *The route catalog's frame*).

**Three of the keys are about a state only a variable reaches** — `JIGC_ADAPTERS_DIR` for two, `$HOME` taken away for one. **Whether a route owes anything there is ruled** ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-11 — Which states a route must work in*): a state only `JIGC_ADAPTERS_DIR` reaches is a declared bound of the catalog — a red case that stands on one is recorded, stays red, and does not block; `$HOME` taken away is no bound.

**The cases are those of every blocking site of `jigc setup`** — thirty-six sites, four of them with a reason and no case — **and of every blocking site of `jigc uninstall`** (§4), eighteen, each with a case: the door, the four guards over what `.jigc/` holds the only copy of, and the seven steps that remove.

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

### `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line`

- **Classification:** followed, a printed line no shell can split. **Marked `doubt`** — both cases rest on a reading the human has not given. **Two cases:** `setup-unborn-staged-file-at-an-install-path-unstaged-as-printed-and-moved`, `setup-unborn-install-path-staged-and-deleted-unstaged-as-printed`.
- **State:** a repository with no commit yet and no install, in which a file is staged at an install path (`CLAUDE.md`) — still in the working tree in the first case, deleted from it in the second, where the refusal comes behind the install's writes.
- **Refusing command:** `jigc setup` — exit 1, `setup.dirty-install-path`.
- **Printed route** (the first case; the second ends its commit clause at *before the install runs over the path*):

  ```text
  route: move the file(s) out of those path(s) — `git -C <sandbox>/repo rm --cached -- <path>` first where you had staged one — then re-run `jigc setup`; or commit them and re-run, so git holds your copy before the install runs over the path — a first commit also ends the fresh-repository install, so `jigc setup` will not seed its secrets-floor root `.gitignore` afterwards (it writes that only where there is no commit yet; moving the file out instead keeps it); `jigc setup --force` is the single consent, and it lets the install run and commit those paths as it leaves them — which at a path jigc regenerates whole is jigc's own content, not yours
  ```

- **What was done:** the first backticked line handed to a shell as printed — it is not run: `<path>` is a redirection with nothing behind it, and the shell refuses the line whole (`syntax error near unexpected token`). Then the rest of the way: the file moved out of its path (the first case), and `jigc setup` again.
- **What went wrong:** the one line the route prints for a staged file cannot be run as it stands; and with the file still staged, the re-run refuses under the same code in both cases. The repository's path is printed in full in the same line; the staged file's, which the refusal lists two lines above, is not.
- **The two readings:** (a) the backticked line is a command to run as printed, like every other line a route prints between backticks — followed here, red; (b) it is a shape, and the reader fills in the path — then this route prints no line that can be run as it stands, and the catalog has no case for it: a filled-in line would be a line of the case's own, which the frame does not run.
- **Beside it, and green:** the route's other two ways from the same two states — the file committed, and the consent (`…-committed`, `…-forced`).
- **[2026-10-11 — read by the surface contract, and no case stands under this key any more.** [surface-contract.md](../../../../../design/surface-contract.md) → *The route fence* says of every backticked `git …` / `jigc …` span that *a `<placeholder>` the reader fills is legal bare or as a quoted span*: reading (b) is the design's, and the line is a shape. So the frame fills it — with the path the refusal lists above its route, by index, never a path of the case's ([DECISIONS.md](../../../../../DECISIONS.md) → 2026-10-11, *The route catalog's first doubt, read by the surface contract*) — and each case was followed again:
  - `setup-unborn-staged-file-at-an-install-path-unstaged-as-printed-and-moved` — **followed, holds, operand implied**: `git -C <sandbox>/repo rm --cached -- CLAUDE.md` exits 0, the file is moved out of its path, and `jigc setup` installs. (*Operand implied* is the move: the route says *those path(s)* and prints none.)
  - `setup-unborn-install-path-staged-and-deleted-unstaged-as-printed` — **red, under the key below**: the filled line is refused by git.**]**

### `r1-setup-unborn-dirty-route-unstaging-line-refused-behind-the-installs-writes`

*Minted 2026-10-11, with the reading above.*

- **Classification:** followed, the same refusal again. **One case:** `setup-unborn-install-path-staged-and-deleted-unstaged-as-printed`. Not marked `doubt`: the contract gives the reading.
- **State:** a repository with no commit yet and no install; a file was staged at an install path (`CLAUDE.md`) and then deleted from the working tree.
- **Refusing command:** `jigc setup` — exit 1, `setup.dirty-install-path`, **behind the install's writes**: *the install files are written and staged, and no install commit was made*. The index holds the staged bytes at `CLAUDE.md`; the working tree holds the file the install wrote there.
- **Printed route** — the unborn route of the key above, without its clause about the first commit:

  ```text
  route: move the file(s) out of those path(s) — `git -C <sandbox>/repo rm --cached -- <path>` first where you had staged one — then re-run `jigc setup`; or commit them and re-run, so git holds your copy before the install runs over the path; `jigc setup --force` is the single consent, and it lets the install run and commit those paths as it leaves them — which at a path jigc regenerates whole is jigc's own content, not yours
  ```

- **What was done:** the unstaging line with its placeholder filled by the one path the refusal lists — `git -C <sandbox>/repo rm --cached -- CLAUDE.md`; then `jigc setup` again.
- **What went wrong:** git refuses the line, exit 1 — *the following file has staged content different from both the file and the HEAD: `CLAUDE.md` (use -f to force removal)* — because the refused run wrote its own `CLAUDE.md` over the path before it refused. Nothing is unstaged, and the re-run refuses under the same code. The one line the route prints for a staged file does not work in the state this arm of the refusal is raised in; what does is not printed (`-f`).
- **The ground:** the exit rule's sentence, *every refusal's route works as printed*, with the contract's reading of the line as a shape whose one value the refusal lists. The contract asks nothing more of the line: it is in the aimed form the printed-path fence states for a `git` span with a path operand, and `<path>` is a placeholder it declares not derivable from a finding's target.
- **Beside it, and green:** the same line in the state where the refusal comes before any write (`…-unstaged-as-printed-and-moved`, above); and from this state, the file committed and the consent (`…-committed`, `…-forced`).

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

## 4. `jigc uninstall` — keyed, cases listed under each

Every case starts from one state: somebody's repository — their `CLAUDE.md`, their settings file with a permit of their own, their `pre-commit` hook, a file jigc never writes, all committed — and an install the binary under test made over it. (Three start beside it: two from a settings file that already held jigc's own entries, one from a repository with no hook of its own.) **What every case holds still:** a file outside the repository, and inside it the files the command has no business touching (`README.md`, `notes.md`); where a route leads to no teardown at all, every file of the install as well. **What must hold where a route leads to a teardown:** `.jigc/` and the guide are gone, jigc's lines are out of `CLAUDE.md`, the settings file and the hook, and their owner's lines are still in each.

**No case of this section came out *a witness changed*.**

### Under keys that stand — the same function, handed this door's code

- **`r1-setup-home-unset-route-says-run-from-inside-the-repository`** — *the same refusal again*, 1 case: `uninstall-home-not-set-run-as-the-route-prints`. *Why the same defect and no neighbour:* it is the same arm of the same function — `locate_finding`'s second, which `run_uninstall` hands `uninstall.repo-root` as `setup::run` hands it `setup.repo-root`; the route is ``run `jigc uninstall` from inside the target git repository``, where it was run.
- **`r1-setup-profile-route-says-reinstall-under-adapters-dir`** — *the same refusal again*, 3 cases: `uninstall-profile-not-in-the-adapters-directory`, `-not-utf-8`, `-malformed`. *Why the same defect:* the same loader (`load_profile`) refused the same file, and `run_uninstall` words it with the sentence `setup::run` uses — *reinstall jigc — the embedded adapter profile is missing or malformed* — over a profile read from the directory `JIGC_ADAPTERS_DIR` names. Reached through that variable and no other way.

### `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id`

- **Classification:** followed, a printed line no shell can split. **Marked `doubt`.** **Three cases:** `uninstall-staged-doc-read-and-discarded-as-printed`, `uninstall-staged-doc-finalized-as-printed` (`uninstall.staged-prose`); `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` (`uninstall.dirty-worktree`).
- **State:** an open task that stages a doc (`jigc start --workflow decided-task "write a note"`); or a fan-out — a milestone, one sub-task, its worktree provisioned — with a file in no commit in the worktree.
- **Refusing command:** `jigc uninstall` — exit 1.
- **Printed routes:**

  ```text
  route: read what is in them with `jigc doc show <address> --task <task-id>`, then throw the task away with `jigc task discard <task-id> --force` — or, once its doc is complete, land it with `jigc task finalize <task-id>` (which refuses while a required slot is empty) — then re-run `jigc uninstall`; `jigc uninstall --force` deletes them with the install
  route: get the work out of those paths first (commit, stash, or copy it), then re-run `jigc uninstall` — or abandon the milestone with `jigc milestone discard <milestone-id> --force`, which destroys the uncommitted work in the path(s) registered here, and re-run `jigc uninstall`; `jigc uninstall --force` deletes them with the install
  ```

- **What was done:** each backticked line handed to a shell as printed. The shell refuses the lines that hold a word in angle brackets — it reads `<task-id` as a redirection from a file of that name, and there is none — so none of them is run; `jigc uninstall`, run next as printed, refuses again under the same code. The staged doc and the worktree's file are witnesses and did not change.
- **The two readings:** (a) a command to run as printed — followed here, red; (b) a shape to fill in with the id the refusal lists two lines above its route — then the route prints no line a reader can run as it stands. **By hand, filled in, both hold**: `jigc task discard write-a-note --force` and `jigc milestone discard fan-out --force` exit 0, and `jigc uninstall` then exits 0. The frame runs no line a case fills in.
- **Beside them, and green:** the consent from the same states (`…-forced`), and the fan-out worktree's work moved out of the path (`…-moved-out`).
- **Kin:** `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` — another route, another function, the same question for the human.
- **[2026-10-11 — read by the surface contract; one case is left under this key, and it is still marked.** The contract reads such a line as a shape the reader fills (the note under the kin key, §1), and the frame fills a placeholder with a value the refusal lists, by index. Followed again:
  - `uninstall-staged-doc-read-and-discarded-as-printed` — **followed, holds**. The refusal lists `write-a-note: commit:write-a-note`; `jigc doc show commit:write-a-note --task write-a-note` and `jigc task discard write-a-note --force` exit 0, and `jigc uninstall` tears the install down.
  - `uninstall-staged-doc-finalized-as-printed` — **followed, holds**, from a state the way's own words ask for: the route says *once its doc is complete, land it*, and that the line *refuses while a required slot is empty*. So the case's state is now the task with its doc complete and a change of its own staged; `jigc task finalize write-a-note` exits 0 and the teardown follows. From the doc as minted the filled line is refused, as the route says it will be — driven by hand, and no case: that is a way the route does not offer that state.
  - `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` — **still red, *a printed line no shell can split*, and still marked `doubt`**. The refusal lists the worktree's path and what git reports of it, and **no milestone**: `<milestone-id>` cannot be filled from what was printed. The contract decides that the line is a shape and declares `<milestone-id>` not derivable from a finding's target; it does not say where a reader takes such an id from. **The two readings now:** (a) from the refusal's own output, as every other value a route needs — then this route owes the id and is red as it stands; (b) from somewhere else the reader knows of — then the frame has no case. The contract's nearest line is its reason for `<workflow-id>`, the one placeholder it says anything of the kind about: *the set it must be filled from is named in the message instead, which is the only place the caller's choice can come from*.**]**

### `r1-uninstall-force-over-unreadable-workbench-fails-part-way`

- **Classification:** followed, another failure. **Five cases:** `uninstall-worktree-path-unreadable-forced` (`uninstall.dirty-worktree`, the held path); `uninstall-worktrees-root-unreadable-forced` (the same code, the guard that could not look); `uninstall-task-areas-unlistable-forced`, `uninstall-workbench-unsearchable-forced` (`uninstall.foreign-bytes`); `uninstall-staged-docs-unsearchable-forced` (`uninstall.staged-prose`).
- **State:** the install, and one directory of the workbench that may not be read or searched: `.jigc/worktrees/shut` or `.jigc/worktrees` or `.jigc/tasks` or `.jigc` itself of mode 000, or an open task's `docs/` of mode 600.
- **Refusing command:** `jigc uninstall` — exit 1; each of the four routes ends in the consent, three of them in these words:

  ```text
  … `jigc uninstall --force` deletes them with the install
  ```

- **What was done:** `jigc uninstall --force`, as printed.
- **What went wrong:** exit 1 with `uninstall.remove-jigc` — *cannot remove the repo-local `.jigc/` tree: Permission denied*, routed *ensure `.jigc/` is writable* — **after part of `.jigc/` was removed**. Driven by hand over `.jigc/worktrees` of mode 000: all six files of the install under `.jigc/` are gone — the settings record among them — and `.jigc/worktrees` stands; over a task's `docs/` of mode 600: `.jigc/AGENT.md` is gone and the rest stands. The files it took are in git's index. The consent the route offers does not delete *them with the install*: it leaves the install half removed and refuses under another code, whose own route names `.jigc/`.
- **Not the canary's `r1-uninstall-refuses-and-force-half-removes-no-work-tree-home`:** that row is `--force` refusing at the hook's step, *after `.jigc/` is removed*, in a layout whose home is no work tree. This is the removal of `.jigc/` itself, failing part-way, in a plain checkout. A neighbour.
- **Every state here is a permission taken away by hand inside the ignored workbench.** **Whether a route owes anything there is ruled** ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-11 — Which states a route must work in*): a permission bit changed by hand is a declared bound of the catalog, so a red case that stands on one is recorded, stays red, and does not block — unless the human admits its key by name, as that ruling does one.

### `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable`

- **Classification:** followed, the same refusal again. **Marked `doubt`.** **Three cases:** `uninstall-task-areas-unlistable-made-readable`, `uninstall-workbench-unsearchable-made-readable` (`uninstall.foreign-bytes`); `uninstall-staged-docs-unsearchable-made-readable` (`uninstall.staged-prose`).
- **State:** as above — `.jigc/tasks` of mode 000; `.jigc` of mode 000; an open task's `docs/` of mode 600.
- **Printed routes:**

  ```text
  route: make sure the `.jigc/` tree is readable, then re-run `jigc uninstall` — or, once you have confirmed it holds nothing you need, `jigc uninstall --force`
  route: make sure `.jigc/tasks/` is readable, then re-run `jigc uninstall` — or, once you have confirmed the open tasks hold nothing you need, `jigc uninstall --force` deletes them with the install
  ```

- **What was done:** the entry the route names given its owner's read bit; `jigc uninstall` again.
- **What went wrong:** the same refusal. The named entry had the bit (`.jigc/` over an unlistable `.jigc/tasks`; `.jigc/tasks/` over an unsearchable `docs/` two levels down), or the bit is not what was missing (`.jigc` of mode 000 made 400 still may not be searched). The staged-doc refusal's message names the directory at fault; the foreign-byte refusal's names none.
- **The two readings:** (a) the entry the route names, its owner's read bit — followed here; (b) every directory under it, readable and searchable — under that reading each route is expected to hold; not driven, and the frame has no act for a tree.
- **Beside them, and green:** `uninstall-worktrees-root-unreadable-made-readable` — the sibling guard's route names `.jigc/worktrees/`, which is the entry at fault, and the listing needs the read bit alone.

### `r1-uninstall-worktree-route-move-aside-over-unreadable-path`

- **Classification:** followed, the same refusal again. **One case:** `uninstall-worktree-path-unreadable-moved-out`.
- **State:** the install, and `.jigc/worktrees/shut` — a directory holding a file — of mode 000.
- **Refusing command:** `jigc uninstall` — exit 1, `uninstall.dirty-worktree`: *unknown — could not read the leftover directory … nothing could be read there*.
- **Printed route:**

  ```text
  route: get the work out of those paths first (commit, stash, or copy it), then re-run `jigc uninstall` — abandoning the milestone will not clear them: a milestone teardown removes only the worktrees this repository has registered, and none of these paths is (a path listed above as a file, or as one nothing could be read at, is not a worktree at all, so no worktree removal clears it — look at it and move it aside or delete it yourself); `jigc uninstall --force` deletes them with the install
  ```

- **What was done:** the directory moved out of the way — *move it aside*; `jigc uninstall` again.
- **What went wrong:** the move is refused, *Permission denied*: a directory that may not be written cannot be given another parent, and one that may not be read cannot be looked at or emptied. The route says what to do with the path and not what must be done to it first. The re-run refuses again; and the consent from the same state is red under the key above.

### `r1-uninstall-worktrees-route-prints-git-worktree-remove-without-a-path`

- **Classification:** followed, the same refusal again. **Marked `doubt`.** **One case:** `uninstall-worktrees-root-unreadable-worktrees-removed-as-printed`.
- **State:** the install, and an empty `.jigc/worktrees` of mode 000.
- **Refusing command:** `jigc uninstall` — exit 1, `uninstall.dirty-worktree`: *cannot check `.jigc/worktrees/` for uncommitted fan-out work*.
- **Printed route:**

  ```text
  route: make sure `.jigc/worktrees/` is readable — this failure is a `read_dir` of that directory, not a git fault — then re-run `jigc uninstall`; or remove the worktrees yourself (`git worktree list`, then `git worktree remove`) and re-run — or, once you have confirmed the fan-out worktrees hold nothing you need, `jigc uninstall --force` deletes them with the install
  ```

- **What was done:** the route's second way — `git worktree list`, exit 0, listing the main checkout alone; `git worktree remove`, exit 129, git's usage line; `jigc uninstall` again.
- **What went wrong:** the second line is no command as printed, and no worktree removal reaches the cause the route's own first clause names — the directory cannot be listed. The re-run refuses the same way.
- **The two readings:** (a) two command lines to run as printed — followed here; (b) the names of two git verbs, the second to be run for each path the first lists — then the route prints no line, and in this state there is nothing for it to be run on.
- **Beside it, and green:** the route's first way, `uninstall-worktrees-root-unreadable-made-readable`.

### Under the canary's `r1-p3-uninstall-writability-route-not-driven` — the seven steps that remove

Not minted here. The canary's row is the hook's step: *ensure the repo's git hooks directory is writable* while that path is already writable — under `core.hooksPath` naming a file or `/dev/null`, a read-only wrapped hook, a non-UTF-8 hook, a hook linked to a read-only outside script. **The same defect at six more steps**: each of the seven steps of `uninstall` words every error of its callee under one route, *ensure … is writable* over one entry it names, whatever the error was. *Why the same defect and no neighbour:* it is the canary's own site with four more causes, and its six sibling `map_err`s in the same function, shaped the same way. Twenty cases, each ***the same refusal again*** — the entry the route names was given its owner's write bit, and the command was run again:

| step — `jigc uninstall`, exit 1 | printed route | the causes it was printed for, a case each |
|---|---|---|
| `uninstall.remove-jigc` | ``route: ensure `.jigc/` is writable, then re-run `jigc uninstall` `` | a directory under `.jigc` may not be written while `.jigc` may (`uninstall-workbench-directory-read-only`) · the repository root may not be written (`uninstall-workbench-under-a-read-only-root`) |
| `uninstall.unwire-reference` | ``route: ensure `CLAUDE.md` is writable, then re-run `jigc uninstall` `` | a directory stands at `CLAUDE.md` (`uninstall-reference-host-a-directory`) · it is not UTF-8 (`-not-utf-8`) · it may not be read (`-unreadable`) · it is a link to a file outside the repository that may not be written (`-a-link-to-a-read-only-file-outside`) |
| `uninstall.remove-allowlist` | ``route: ensure `.claude/settings.json` is writable, then re-run `jigc uninstall` `` | a directory stands at the settings file's path (`uninstall-settings-a-directory`) · it is not UTF-8 (`-not-utf-8`) · it is no JSON (`-not-json`) · it may not be read (`-unreadable`) |
| `uninstall.remove-precommit` | ``route: ensure the repo's git hooks directory is writable, then re-run `jigc uninstall` `` | the canary's five — `core.hooksPath` names a regular file (`uninstall-hook-hooks-path-a-file`), names `/dev/null` (`-hooks-path-dev-null`), the wrapped hook may not be written in a hooks directory that may (`uninstall-hook-read-only`), is not UTF-8 (`-not-utf-8`), is a link to a read-only script outside (`-a-link-to-a-read-only-script-outside`) — and three it did not drive: `core.hooksPath` is the empty string (`-hooks-path-empty`), a directory stands at the hook's path (`-a-directory`), the hook may not be read (`-unreadable`) |
| `uninstall.remove-guide` | ``route: ensure `.claude/skills/jigc/SKILL.md` is writable, then re-run `jigc uninstall` `` | the directory that holds the guide may not be written (`uninstall-guide-directory-read-only`) — a file's removal is its directory's permission, so this route's words are true of no state |

**And one *another failure*, under the same key:** `uninstall-forced-workbench-directory-unreadable-made-writable`. `jigc uninstall --force` over a `.jigc/worktrees` of mode 000 refuses under `uninstall.remove-jigc`, routed *ensure `.jigc/` is writable, then re-run `jigc uninstall`*: it is writable, and the re-run — which the route prints without the consent — refuses under `uninstall.dirty-worktree`.

**Every refusal behind the first step finds `.jigc/` gone.** The first step removes the workbench; a refusal at the reference, the settings file, the hook or the guide leaves the rest of the install standing beside no `.jigc/`.

**Beside them, and green — the state each route's words are true of:** `uninstall-workbench-read-only` (`.jigc` itself may not be written) and `uninstall-hook-of-jigcs-own-hooks-directory-read-only` (an install over a repository with no hook of its own, and a hooks directory that may not be written — *holds, operand implied*: the route prints no path for the directory). Two more states the words are true of are red under the next key.

### `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries`

- **Classification:** followed, another failure — the re-run exits 0, and what must hold afterwards does not: the settings file still holds jigc's entries. **Four cases:** `uninstall-reference-host-read-only` (`uninstall.unwire-reference`), `uninstall-settings-read-only` (`uninstall.remove-allowlist`), `uninstall-session-hook-settings-read-only` (`uninstall.remove-hook`), `uninstall-deny-floor-settings-read-only` (`uninstall.remove-deny`).
- **State:** the install, and the file the step writes may not be written — `CLAUDE.md` of mode 444, or the settings file of mode 444. For the last two, their settings file held jigc's permits (and the session hook) before `jigc setup`, so the install's record lists none of them and the step named is the first to write the file.
- **Refusing command:** `jigc uninstall` — exit 1, with `.jigc/` already removed.
- **Printed routes:**

  ```text
  route: ensure `CLAUDE.md` is writable, then re-run `jigc uninstall`
  route: ensure `.claude/settings.json` is writable, then re-run `jigc uninstall`
  ```

- **What was done:** the named file given its owner's write bit — these are the states the route's words are true of; `jigc uninstall` again — exit 0.
- **What went wrong:** the teardown the re-run completes leaves every entry of jigc's in the settings file — 25 of them in the first two cases, the permits, the deny floor and the session hook — and says so: *left 25 entries … no record git tracks lists what `jigc setup` added … so jigc cannot tell its entries from yours and removed none*. The record that says which entries are jigc's is `.jigc/settings-entries.json`; the refused run read it and then removed `.jigc/`, and the re-run has none to read. So a refusal behind the first step turns *re-run `jigc uninstall`* into a teardown that takes less than the one that was refused would have.
- **Kin:** `r1-setup-settings-record-refused-rerun-installs-without-the-record` — the install's side of the same record; and the canary's `r1-uninstall-refuses-and-force-half-removes-no-work-tree-home`, another refusal behind the removal of `.jigc/`.

### `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command`

- **Classification:** the command did not end. **Three cases:** `uninstall-reference-host-a-special-file` (`CLAUDE.md`), `uninstall-settings-a-special-file` (`.claude/settings.json`), `uninstall-hook-a-special-file` (`.git/hooks/pre-commit`).
- **State:** the install, and a FIFO where the file stood.
- **Refusing command:** `jigc uninstall` — it does not end; the runner ends it at its limit. No refusal and no route is printed.
- **What went wrong:** each step reads its file with no limit, and the read of a FIFO with no writer does not return — **behind the removal of `.jigc/`**: the command holds with the workbench gone, and at the hook with the reference and the settings entries taken out as well.
- **Why a key of its own and not `r1-setup-fifo-at-install-path-holds-the-command`:** another command and other reads — `unwire_reference`, the teardown's `read_settings`, `remove_precommit_hook` — and what the hold leaves behind is a half-removed install, not a half-written one. Kin.

### Under the canary's `r1-p3-uninstall-force-names-nothing-undecodable-listing` — the fourth guard, where git has no answer

Not minted here. The canary's row is the fourth guard over a listing of git's that cannot be decoded: its route says *make sure `git` is on PATH and the `.jigc/` tree is readable*, and both already hold. **Two cases, each *the same refusal again*** — `.jigc/` given its owner's read bit, the command again:

| case | refusing — `jigc uninstall`, exit 1, `uninstall.untracked-workbench-file` | the cause |
|---|---|---|
| `uninstall-git-listing-undecodable-made-readable` | *`git` produced non-UTF-8 output* | the canary's: the index holds an entry under `.jigc/` whose name is not UTF-8 |
| `uninstall-git-index-unreadable-made-readable` | *`git ls-files …` failed: … index file smaller than expected* | the same route over another answer git could not give — the same site, the same arm, the same words: `.git/index` is no index |

**The half of the canary's row the catalog does not see:** that `--force` then destroys an untracked file and names nothing. The consent from both states exits 0 and the teardown is whole (`…-forced`, green); what it says while it does so is no case's business.

**Beside them, and green:** `uninstall-workbench-unlistable-made-readable` — `.jigc` may be searched and not listed, the one state the route's second clause is true of.

### More cases under two keys of this section

- **`r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id`** — 2 more, *a printed line no shell can split*, marked `doubt`: `uninstall-untracked-workbench-file-added-as-printed` and `uninstall-workbench-files-one-ignored-moved-out-and-one-added-as-printed`. The fourth guard's route prints `` `git -C <sandbox>/repo add -- <path>` is enough ``, with the repository's path in full and `<path>` where the listed file's belongs. *Why the same defect:* the same shape — a backticked line with a bracketed word whose value the refusal lists above its route — at another site. **By the route's other words the same state holds**: the file committed (*put them where they can be recovered*), or deleted (`…-committed`, `…-deleted`, green). **[2026-10-11: both hold, filled.** The refusal lists each file's path on a line of its own, and `git -C <sandbox>/repo add -- .jigc/config/local.yaml` — in the second case `… add -- .jigc/config/other.yaml`, its second listed path, the first being the one git ignores — exits 0; `jigc uninstall` then tears the install down, the index keeping the copy. Neither is listed red any more.**]**
- **`r1-uninstall-force-over-unreadable-workbench-fails-part-way`** — 1 more, *another failure*: `uninstall-workbench-unlistable-forced`, the consent over a `.jigc` that may be searched and not listed, from the fourth guard's route.

### Seen, and no case — nothing refuses, so there is no route to follow

- **`jigc uninstall` writes through a link, to a file outside the repository, at exit 0.** With `CLAUDE.md` a link to a file outside that holds the reference, the teardown takes the reference out of *that file*; with the `pre-commit` hook a link to a script outside that holds jigc's block, it writes the script back without the block. Driven by hand: both files' bytes change, the link at `CLAUDE.md` stays, and the summary says *unwired bootstrap reference ← CLAUDE.md* and *removed pre-commit hook*. What is written is the file less jigc's lines. Where the outside file may not be written, the step refuses — the two `…-a-link-to-a-read-only-…-outside` cases above — and its route says to make `CLAUDE.md`, or the hooks directory, writable; a reader who does so through the link has the outside file rewritten by the re-run. The frame gives no permission through a link, so no case shows that, and the outside files of both cases are witnesses that did not change.
- **After a clean `jigc uninstall` the settings file is not what it was before the install**: an emptied `"SessionStart": []` under `"hooks"` and an empty `"deny": []` stay behind in a file that had neither.

## 5. The gate — `task finalize`, the sites the fix pass touched

*Added 2026-10-12, with the cases of [route_catalog/task_finalize.rs](../../../../../crates/cli/tests/route_catalog/task_finalize.rs): seven sites of the family, taken in by the human's ruling of 2026-10-11 ([DECISIONS.md](../../../../../DECISIONS.md) → *Which families the route catalog takes in*). Every case here stands at a site the fix pass touched; what the previous release's binary does with each is not said in this file.* Each state starts from `jigc setup` and an open task whose `commit` doc is complete, made by the binary under test.

### `r1-finalize-nothing-staged-route-prints-finalize-without-its-id`

- **Classification:** followed, another failure. **Three cases, all marked `doubt`:** `finalize-nothing-staged-a-tracked-file-modified-added-and-finalized-as-printed`, `…-a-new-file-…`, `…-a-tracked-file-deleted-…`.
- **State:** the working tree holds a change — a tracked file modified, a file git does not track, a tracked file deleted — and nothing is staged.
- **Refusing command:** `jigc task finalize write-a-note` — exit 3, `finalize.nothing-staged`.
- **Printed route:**

  ```text
  route: `git add` your changes, then re-run `jigc task finalize`
  ```

- **What was done:** the two printed lines, as printed: `git add` — exit 0, *Nothing specified, nothing added* — and `jigc task finalize`.
- **What went wrong:** exit 2 — *error: the following required arguments were not provided: `<ID>`*. The command requires the task's id and the route prints the command without it. The refusal one step on, `finalize.empty-commit`, prints `jigc task finalize write-a-note`.
- **The doubt:** whether the second span is a line to run or the name of the command the reader has just run. Driven by hand under the second reading — the change added by its path, the finalize run with its id — it exits 0 with one commit.

### `r1-finalize-stage-failed-route-names-a-lock-and-three-paths-and-the-docs-home-is-ignored`

- **Classification:** followed, the same refusal again. **One case:** `finalize-stage-failed-the-docs-home-ignored-re-run-as-printed`.
- **State:** the root `.gitignore`, committed, holds `docs/`; the task creates the decisions log, which promotes to `docs/decisions-log.md`.
- **Refusing command:** `jigc task finalize write-a-note` — exit 3, `finalize.stage-failed`; the message quotes git: *The following paths are ignored by one of your .gitignore files: docs*.
- **Printed route:**

  ```text
  route: `jigc task finalize write-a-note` once the embedded git failure is resolved — a stale `.git/index.lock` removed, or the `.gitignore` rule dropped that covers `.jigc/config`, `.jigc/.gitignore` or `.jigc/version`: jigc commits those three with the work, and git will not add a path it ignores — the task survives intact, so the same re-run lands the commit
  ```

- **What was done:** no lock stands and no rule covers any of the three paths, so nothing the route names is there to act on; the printed re-run.
- **What went wrong:** the same refusal. The route names two causes and a third refused: the `git add` it embeds also stages the promoted doc, and the rule is over the doc's home. Where the rule is over one of the three paths the route names, the route holds (`finalize-stage-failed-the-stamp-ignored-and-in-no-index-ignore-file-removed`).

### `r1-finalize-amend-staged-doc-route-says-the-start-line-mints-and-it-presents-the-workflows`

- **Classification:** followed, another failure. **One case, marked `doubt`:** `finalize-amend-staged-doc-an-edit-refused-and-the-front-door-run-as-printed`.
- **State:** a committed decisions log, and an amend task minted over the commit that landed it.
- **Refusing command:** `jigc doc add-item decisions-log:decisions-log#entries --task fix-the-message --title "A decision"` — exit 1, `finalize.amend-staged-doc`.
- **Printed route:**

  ```text
  route: `jigc start "<intent>"` mints an ordinary task, whose finalize commits the promoted doc; this amend task keeps its own job, repairing `HEAD`'s message
  ```

- **What was done:** the printed line, with the placeholder's own bytes as the intent — the refusal lists no value for it.
- **What went wrong:** exit 0, and no task was minted: under the shipped default the front door presents the workflows to pick one from. The route says the line *mints an ordinary task*. The linked-worktree refusal's route says of the same line *the front door there, which mints a task or presents the workflows to pick one from*.
- **The doubt:** whether *mints* is said of the printed line or of the door the reader then goes through.

### `r1-linked-doc-route-cites-a-command-that-is-itself-refused`

- **Classification:** followed, the same refusal again (one case) · followed, another failure (one case). **Both marked `doubt`.**
- **State:** a task minted in the main checkout that may create the decisions log; a linked worktree of the user's beside the repository. (1) the task already stages the log, staged from the main checkout; (2) the worktree's `HEAD` is detached.
- **Refusing command:** a `jigc doc` write of the log, asked from the worktree — exit 1, `finalize.linked-worktree-doc`.
- **Printed route**, its last sentence in each:

  ```text
  (1) … Task `write-a-note` already stages a managed doc, so it does not finalize from here — `jigc task validate write-a-note` names where it lands
  (2) … This worktree's HEAD is detached, so task `write-a-note` commits nothing from here until it is on a branch — `jigc task finalize write-a-note` names that route
  ```

- **What was done:** the cited line, run where the case stands.
- **What went wrong:** (1) `jigc task validate write-a-note` exits 3 under `finalize.linked-worktree-doc` — it does name where the task lands, in that refusal's route; (2) `jigc task finalize write-a-note` exits 1 under `repo.head-detached`, whose route is the one meant. The frame holds every printed command to exit 0.
- **The doubt:** whether a line a route cites for what it prints owes an exit of 0. The pilot's fourth question is the same one, over a `git` line (`r1-setup-unverified-route-opens-with-the-question-that-failed`).
- **Cases `linked-doc-second-write-refused-the-preview-asked-where-it-lands`, `linked-doc-write-refused-on-a-detached-head-the-finalize-asked-for-its-route`.**

### `r1-finalize-provenance-io-route-says-disk-or-permissions-over-a-manifest-of-another-kind`

- **Classification:** followed, the same refusal again. **Three cases:** `finalize-provenance-manifest-not-json-area-made-readable`, `…-a-directory-…`, `…-a-link-to-itself-…`.
- **State:** the open task's `docs/provenance.json` holds text that is no JSON · is a directory · is a symbolic link to itself. None is a permission.
- **Refusing command:** `jigc task finalize write-a-note` — exit 3, `finalize.provenance-io`; the message ends with the reader's own error — *expected ident at line 1 column 2*, *Is a directory (os error 21)*, *Too many levels of symbolic links*.
- **Printed route:**

  ```text
  route: resolve the read fault on the task provenance manifest under `<sandbox>/repo/.jigc/tasks/write-a-note` (a disk or permissions problem), then re-run the finalize
  ```

- **What was done:** the one entry the route prints made readable to its owner, which it was; the finalize again.
- **What went wrong:** the same refusal. The route names one kind of fault for every error of the read. It is the canary's writability pattern (`r1-p3-setup-writability-route-printed-for-other-causes`) at a step of another command — a neighbour of that row, not the row.

### `r1-task-working-area-io-route-says-disk-or-permissions-over-an-entry-of-another-kind`

- **Classification:** followed, the same refusal again. **Three cases:** `working-area-io-tasks-directory-a-file-workbench-made-writable`, `working-area-io-roles-record-not-json-…`, `working-area-io-a-directory-at-the-staged-docs-path-…`.
- **State:** a regular file at `.jigc/tasks` · the task's `roles.json` no JSON · a directory at the path the task stages a doc under. None is a permission.
- **Refusing command:** `jigc start --workflow decided-task "write a note"` · `jigc doc create decisions-log --title 'Decisions Log' --task write-a-note` — exit 1, `task.working-area-io`: *could not open the working area* (*Not a directory*), *could not read the bound roles*, *could not read the staged instance* (*Is a directory*).
- **Printed route:**

  ```text
  route: resolve the underlying I/O condition (a disk or permissions problem on the `.jigc/` task working area), then re-run the command
  ```

- **What was done:** `.jigc/` made writable to its owner, which it was; the command again.
- **What went wrong:** the same refusal — one route for ten I/O calls, as the key above. At the create door the message names the doc's address as the task: *for task `decisions-log:decisions-log`*.

### `r1-doc-create-refused-over-the-manifest-stages-the-body-and-the-re-run-exits-0`

- **Classification:** followed, another failure. **Two cases:** `working-area-io-manifest-a-directory-at-a-create-workbench-made-writable`, `working-area-io-manifest-a-directory-at-a-copy-in-workbench-made-writable`.
- **State:** a directory stands where a task's `docs/provenance.json` was — in a task that creates the decisions log, and in a second task once the log is committed.
- **Refusing command:** `jigc doc create decisions-log --title 'Decisions Log' --task <task>` — exit 1, `task.working-area-io`: *could not provision the instance* · *could not copy in the committed instance*.
- **Printed route:** the one of the key above.
- **What was done:** `.jigc/` made writable, which it was; the create again.
- **What went wrong:** the create exits 0 — *already existed — copied in for update* — with the manifest still a directory: **the refused run had staged the doc's body**, and the second finds it. What must hold once a create has gone through is that the task's manifest is a file; it is not. Without that check the case held.

### `r1-conformance-block-in-a-migration-minted-over-the-edit-revert-draws-conflict-block`

- **Classification:** followed, another failure. **One case:** `conformance-hand-broken-doc-migrated-in-place-nothing-authored-edit-reverted`.
- **State:** the committed decisions log with its section's heading removed by hand, and `jigc migrate docs/decisions-log.md --as decisions-log` minted over it; nothing authored.
- **Refusing command:** `jigc task finalize <the migration>` — exit 3, `reconciliation.conformance-block`.
- **Printed route:**

  ```text
  route: fix the file to restore conformance, or revert the edit — this is the one case a managed file is yours to hand-edit: the damage was made out-of-band, so it is repaired where it happened
  ```

- **What was done:** the edit reverted — the file's bytes put back as the index holds them — and the finalize again.
- **What went wrong:** exit 3 under `reconciliation.conflict-block`: *this path is the source the task is migrating, and it no longer holds what the migration recorded when it was minted*. The migration was minted over the edit, so the route's second way changes the source under it.

### Seen, and no case

- **`jigc validate` prints `file-state.hash-matches` as *blocking (gates at finalize)* and exits 0**, so the route it prints there is followed by no case (`drift_store_finding` has a reason).
- **A finding of `task.working-area-io` can print two `route:` lines** — where the file-state record cannot be read during a copy-in, the inner fault's route, `rm <the record>`, stands above the `at:` line and this site's below it. Seen by the helper's probe; no case.
- **`finalize.stage-failed` over an ignored doc home names the doc's path in git's quoted words and in no word of jigc's**: the route's two causes are the only ones it names.
- **The backstop of the linked-worktree guard is not reached from jigc's own fan-out worktree**: `jigc task finalize` and `jigc task validate` of a task that stages a doc exit 1 there under `repo.head-detached` first, whose route — *give this work a branch of its own with `git switch -c <new-branch>`* — would put a commit of that task's on a new branch in the sub-task's worktree. Not followed.
