# The route catalog's pilot — the findings its cases made, by key

What a case of the route catalog ([route_catalog.rs](../../../../../crates/cli/tests/route_catalog.rs), cases in [route_catalog/setup.rs](../../../../../crates/cli/tests/route_catalog/setup.rs)) came out red with, where the small canary's file ([canary-one/findings.md](../canary-one/findings.md)) has no row for it. A key here is in that file's grammar and is what the suite's expected-red list names; the classification is the frame's ([route_case.rs](../../../../../crates/cli/tests/support/route_case.rs)). **Nothing here is a ruling, a grade or a word about the previous release**: each row is what one binary did on one day — this tree's debug build, macOS arm64, 2026-10-11 — when the printed route was followed by its own words. Sandbox paths are written `<sandbox>/…`.

**No ledger holds these keys.** Until a stabilization run is opened they certify themselves, as the canary's do for the cases listed under them ([DECISIONS.md](../../../../../DECISIONS.md) → *The route catalog's frame*).

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

- **Classification:** the command did not end. **Two cases:** `setup-compose-marker-a-special-file` (`.jigc/config/packs.yaml`), `setup-guide-a-special-file` (`.claude/skills/jigc/SKILL.md`).
- **State:** a repository with one commit and no install; a FIFO at the path.
- **Refusing command:** `jigc setup` — it prints nothing and does not end; the runner ends it at its limit. No route is printed, so none is quoted.
- **What went wrong:** `jigc setup` reads each of these two paths before it reaches the refusal that names a special file there, and the read of a FIFO with no writer does not return. At the other four replaced paths (`.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/settings-entries.json`) the same FIFO is refused at once, with a route that holds.
- **Kin, at another door:** the canary's `r1-p4-named-pipe-at-managed-home-holds-reading-doors`.

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
