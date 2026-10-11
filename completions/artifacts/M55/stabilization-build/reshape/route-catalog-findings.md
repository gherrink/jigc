# The route catalog's pilot — the findings its cases made, by key

What a case of the route catalog ([route_catalog.rs](../../../../../crates/cli/tests/route_catalog.rs), cases in [route_catalog/setup.rs](../../../../../crates/cli/tests/route_catalog/setup.rs)) came out red with, where the small canary's file ([canary-one/findings.md](../canary-one/findings.md)) has no row for it. A key here is in that file's grammar and is what the suite's expected-red list names; the classification is the frame's ([route_case.rs](../../../../../crates/cli/tests/support/route_case.rs)). **Nothing here is a ruling, a grade or a word about the previous release**: each row is what one binary did on one day — this tree's debug build, macOS arm64, 2026-10-11 — when the printed route was followed by its own words. Sandbox paths are written `<sandbox>/…`.

**No ledger holds these keys.** Until a stabilization run is opened they certify themselves, as the canary's do for the cases listed under them ([DECISIONS.md](../../../../../DECISIONS.md) → *The route catalog's frame*).

## 1. Keyed — a case is listed under each

### `r1-setup-untracked-link-repointed-draws-dirty-install-path`

- **Classification:** followed, another failure.
- **Cases:** `setup-host-link-pointed-at-a-committed-file` · `setup-host-link-pointed-at-a-new-file` · `setup-settings-link-pointed-at-a-new-file`.
- **State:** a repository with one commit and no `jigc setup`; `CLAUDE.md` (in the third case `.claude/settings.json`) is a link to an existing file outside the repository, and git does not track the link.
- **Refusing command:** `jigc setup` — exit 1, `setup.inject-reference` (third case: `setup.inject-allowlist`).
- **Printed route** (first two cases; the third prints the same words over `.claude/settings.json`):

  ```text
  route: remove the link at `CLAUDE.md`, or point it at a regular file inside this repository — committing the change where git tracks the link — then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link
  ```

- **What was done:** the route's second way out — the link pointed at a regular file inside the repository (the repository's own committed `README.md` in the first case, a file written a moment before in the other two); no commit, since git does not track the link; `jigc setup` again.
- **What went wrong:** the re-run exits 1 with `setup.dirty-install-path`, naming the link itself (and the new file, where there is one) as work in no commit — so on a born `HEAD` the route's second way out leads into a second refusal whenever the link is one git does not track, and the route's clause about committing does not reach that link.
- **Beside it, and green:** the first way out (remove the link) holds in every cause driven; the second holds where git tracks the link, the file it is pointed at is committed and the change is committed (`setup-host-committed-link-pointed-at-a-committed-file`); and it holds on an unborn `HEAD`, where the link rides the first commit (`setup-seeded-ignore-link-pointed-at-a-new-file`).

### `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path`

- **Classification:** followed, another failure. **Marked `doubt`** — the case rests on a reading the human has not given.
- **Case:** `setup-host-committed-link-pointed-at-a-new-file`.
- **State:** as above, and the link at `CLAUDE.md` is committed.
- **Refusing command:** `jigc setup` — exit 1, `setup.inject-reference`.
- **Printed route:** the one above.
- **What was done:** the link pointed at a regular file inside the repository that was written a moment before and is in no commit; the link's change committed, as the route says where git tracks the link; `jigc setup` again.
- **What went wrong:** the re-run exits 1 with `setup.dirty-install-path` over the file the link now leads to — the route says *a regular file inside this repository* and the install takes only one git already holds.
- **The two readings:** (a) any regular file in the checkout, one in no commit included — followed here, red; (b) a file the repository already holds committed — the case `setup-host-committed-link-pointed-at-a-committed-file`, green.

## 2. Seen while the cases were written — no case, no key

Each is in the suite as a stated reason beside the cases (`READINGS` in [route_catalog/setup.rs](../../../../../crates/cli/tests/route_catalog/setup.rs)); none is on the expected-red list, because none is a case. Whether either is a finding of a run is not decided here.

- **A FIFO at an install path `jigc setup` reads before it refuses holds the command.** At `.jigc/config/packs.yaml` and at `.claude/skills/jigc/SKILL.md`, in a repository with one commit and no install: `mkfifo <path>`, then `jigc setup` prints nothing and does not end. Written as a case, the first came out *not provoked* when the runner ended the command at its limit of two minutes; by hand, the second was held for ten seconds and ended when the FIFO was opened for writing. At the other four replaced paths (`.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/settings-entries.json`) the same FIFO is refused at once with a route that holds. Kin, at another door: the canary's `r1-p4-named-pipe-at-managed-home-holds-reading-doors`.
- **The root `.gitignore` as a link is one git itself does not read.** On an unborn `HEAD`, with the root `.gitignore` pointed at a regular file inside the repository as the route offers, `jigc setup` exits 0 and merges its secrets floor through the link; git (2.54.0) then warns `unable to access '.gitignore': Too many levels of symbolic links` and applies none of it — a `.env` written afterwards, which the floor lists, shows as untracked and `git check-ignore .env` exits 1. The case holds as the frame reads a route — the re-run exits 0 and nothing outside changed — and whether the floor takes effect is no check of it.
