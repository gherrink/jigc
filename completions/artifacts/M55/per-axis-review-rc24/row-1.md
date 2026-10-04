<!-- Reconciled ROW 1 file (setup · the hook · install · release), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis1` in the body means ROW 1 of this run, not numbered axis 1. -->

> **Reconciled row 1 (rc.24) — this row HAS a source pass** (`axis1-codex.md`, exit 0; a second higher-effort pass ended on the provider's usage limit and produced no output). The driver's record follows **unchanged — no row was demoted**; the reconciler's work is the *Reconciliation ledger* after it. Outcome: 5 confirmed findings — (R1, F1) **tier 1**, (R1, F2), (R1, F3), (R1, F4), (R1, F5) tier 3; F4 and F5 are new, reached by driving Codex leads.

# Row 1 · setup · the hook · install · release — the driver's record (rc.24)

Partial per-axis re-review of the published `jigc 1.0.0-rc.24`. Driver: Opus, independent of the build.
`axis1` here is **row 1 of this run**, not numbered axis 1. No baseline (first drive). New findings are keyed
`(R1, <id>)`.

## Standing facts

- **Binary:** `~/.local/bin/jigc`; `jigc --version` printed `jigc 1.0.0-rc.24` before anything else ran.
  Release posture. Its embedded source paths are registry paths (18 strings under
  `registry/src/<index>/jigc-1.0.0-rc.24/`, 18 under `…/jigc-engine-0.1.0-rc.2/`, none under a workspace
  `crates/` path), so it is a registry build. No `doc-code` file sits beside it.
- **Host:** Darwin arm64, `git version 2.54.0 (Apple Git-157)`, `cargo 1.95.0`, `rustc 1.95.0`.
- **Repository:** read at `bffa6667` (branch `work/rc24-gate`); `git diff --stat 91834b5e HEAD` touches only
  `completions/trial-driver/`, so `crates/` and `dev/` are the tagged release's. The working repository's
  `git status --porcelain` was empty before and after this run, and its HEAD did not move.
- **Environment (names only):** `CLAUDECODE` set — every install commit below carries
  `Co-Authored-By: Claude <noreply@anthropic.com>`, expected on this row and held constant. `GITHUB_OUTPUT`,
  `JIGC_DOC_CODE_PROBE`, `JIGC_PACK_DIR`, `CARGO_TARGET_DIR` unset in the session; `CARGO_TARGET_DIR` was set
  per cargo command to a private directory under the scratch root; `SCRATCH` was set for one rig (5.15) to a
  scratch root whose name contains a space.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`
  — two steps, stdout only, the guard before any git command, plus a refusal when `$REPO` is under the working
  repository. A fresh rig per cell; no teardown. States used: `bare` (most cells), `fresh`, `vendored`,
  `committed-singletons`, and the standalone `--git-state unborn` form once (1.7). **The unborn-HEAD cells used
  a plain `git init` in a fresh `mktemp -d` under a `bare` rig's `$RIG`** (so `HOME` is the rig's empty home),
  with a repo-local synthetic identity (`trial@example.com`). The rig's standalone unborn form was driven once
  beside it; its hand-made `.jigc/config/` marker is an empty directory.
- **Nothing was written into a `.jigc/` workbench by hand.** Where a cell needs user bytes at an install path
  (`.jigc/AGENT.md`, `.jigc/config/packs.yaml`, `.jigc/.gitignore`, `.jigc/version`) that *is* the cell: the
  bytes are the adopter's, planted before `jigc setup`, each with a before-control (`command grep -c`).
- **Docker:** probed once — `docker version --format '{{.Server.Version}}'` answered `29.5.2`, exit 0. The
  container cells were driven.

## Door set — derived from the code, count beside the instrument's

| registry | file | instrument's count | **read by this driver** | same? |
|---|---|---|---|---|
| `install_tracked_paths` | `crates/cli/src/setup.rs` | 7 unconditional + 3 conditional | 7 unconditional (`CLAUDE.md`, `.claude/settings.json`, `.jigc/AGENT.md`, `.jigc/.gitignore`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`) + 3 conditional (root `.gitignore` iff setup seeded it, the guide, the hook) | yes |
| `InstallPathDisposition` / `install_path_dispositions` | `setup.rs` | 2 dispositions; exempt-when-jigc-owned = `.jigc/version`, the guide | 2 (`Refuses`, `ExemptWhenJigcOwned`); the same two exempt members | yes |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs` | 48 rows; `setup` commits-on-behalf, 1 posture exemption; `uninstall` neither | 48 `BehalfDoor {` rows; `setup` = `CommitsOnBehalf` with one `PostureExemption` (`HeadUnborn`, `SETUP_UNBORN_EXEMPTION`); `uninstall` = `Neither` | yes |
| `IGNORE_DOORS` | `crates/cli/src/gitignore.rs` | 4 | 4 (`jigc setup`, `task finalize` (+`milestone finalize`), `milestone create`, `milestone provision`) | yes |
| `setup.*` / `uninstall.*` literals (grep) | `crates/cli/src`, `crates/engine/src` | 20 / 13 | 20 / 13 distinct quoted literals | yes |
| `PrecommitTeardown` | `setup.rs` | 3 variants | 3 (`NotOurs`, `RemoveFile`, `RestoreForeign`) | yes |
| `include` + `readme` | `crates/cli/Cargo.toml` | 6 patterns + `README.md` | 6 (`src/**`, `packs/**`, `adapters/**`, `guides/**`, `LICENSE-MIT`, `LICENSE-APACHE`) + `readme = "README.md"` | yes |
| `PUBLISHED_DIRS` | `crates/cli/tests/package_contents.rs` | 4 | 4 (`src`, `packs`, `adapters`, `guides`) | yes |
| job list | `ci.yml`, `release.yml` | read, not counted | `ci.yml`: 10 jobs (`hygiene`, `fmt`, `clippy`, `build`, `test`, `unit`, `manifest-freeze`, `publish-dry-run`, `lock-current`, `ci-ok`); `release.yml`: 3 (`release-pr`, `check`, `release`) | — |

**No count differs.** One datum beside the grep census: three codes that are **not** spelled `setup.*` are
minted at the `setup` door and were reached here — `adapter-guide.user-modified` (advisory),
`repo.head-detached` (the posture refusal) and, read back at `validate`, `store-version.binary-mismatch`. A
census keyed on the prefix does not see them.

**`setup.*` codes reached by a driven row: 6 of 20** — `dirty-install-path`, `forced-install-path`,
`install-commit`, `install-hook`, `inject-allowlist`, `write-guide`. **`uninstall.*` codes reached: 0 of 13**
(every `uninstall` driven here exited 0 with no finding; the refusing arms are numbered axis 3's and were not
re-driven, per the scope).

**Doors:** `jigc setup` · `jigc uninstall` (leaves) · the installed pre-commit hook through a real
`git commit` · the install line · `cargo package --list` · `cargo publish --workspace --dry-run` ·
`dev/unpublished-versions` · `dev/runner-faithful tarball` · `dev/runner-faithful registry`. Only `setup` and
`uninstall` confer leaf coverage.

## Findings (defects), in proposed-tier order

| key | title | door | proposed tier |
|---|---|---|---|
| **(R1, F1)** | On an unborn HEAD, `jigc setup` overwrites untracked user bytes at a whole-rewrite install path at exit 0 with nothing said | `setup` | **1** — exit-0 loss through a committing door, bytes in no object (shown below); see the two stated caveats |
| (R1, F2) | `setup --force` over a dirty hook spends the consent without naming it | `setup` | 3 — the advisory says less than the binary did; no loss |
| (R1, F3) | A standalone jigc hook written with another `jigc` path (or another build's template) is wrapped as *foreign*, not regenerated; `uninstall` then leaves a jigc hook behind while saying it removed it | `setup`, `uninstall`, the hook | 3 — three surfaces say something the binary does not do; no loss |

### (R1, F1) — unborn HEAD × untracked user bytes at a whole-rewrite install path

**Contract.** `crates/cli/guides/QUICKSTART.md` → *1. `jigc setup`*: *"If any of them already carries changes
that are in no commit — staged, unstaged or untracked — it stops with one blocking `setup.dirty-install-path`
naming each such path and installs **nothing** … your bytes are still exactly where you left them — including
at the files jigc regenerates whole (`.jigc/AGENT.md`, `.jigc/config/packs.yaml`)"*. `design/validation.md` →
the `setup.dirty-install-path` row: *"Forgetting a new install path now costs a loud false alarm `--force`
clears, never a silent sweep."*

**The stated rule that makes the cell green, and what it does not cover.** The same `validation.md` row says
*"The exclusion survives on an unborn `HEAD` … there is no `HEAD` for a pre-existing file to differ from and
`setup` owns minting the repo's first commit"* — a stated door rule for the `??` class on a repository with
no commits. Its rationale is about the **merged-into** paths (the bytes ride the first commit). It says
nothing about the two paths the install **rewrites whole**, which is the class the row itself calls
*"destroyed"* on a born HEAD. QUICKSTART's sentence carries no unborn carve-out at all.

**Repro A — the direct cell.**

```text
setup   bare rig (for the isolated HOME); F=$(mktemp -d "$RIG/unborn.XXXXXX"); cd "$F"
        git init -q . ; git config user.name Trial ; git config user.email trial@example.com
        mkdir -p .jigc/config
        printf '# Team notes for agents — USERMARK-A written by a human\nAlways run the linter.\n' > .jigc/AGENT.md
        printf '# USERMARK-P why we pin dev only: see the team wiki\npacks:\n- dev\n' > .jigc/config/packs.yaml
before  git rev-list --count --all            → 0
        git status --porcelain -uall          → ?? .jigc/AGENT.md
                                                ?? .jigc/config/packs.yaml
        command grep -rc USERMARK .jigc       → .jigc/AGENT.md:1   .jigc/config/packs.yaml:1      (before-control)
argv    jigc setup
exit    0     stdout: "jigc setup — adapter installed … install commit → f1125db"; no finding, no advisory
after   git status --porcelain -uall          → (empty)
        git log --format='%h %s'              → f1125db chore(jigc): install jigc workspace config
        command grep -rc USERMARK .jigc CLAUDE.md → every file :0
        git log --all -S USERMARK             → 0 commits
        git grep -c USERMARK HEAD             → 0 files
        git fsck --unreachable                → 0 objects        git stash list → 0
        .jigc/AGENT.md                        → jigc's generated bootstrap text
        .jigc/config/packs.yaml               → "packs:\n- dev\ncompose-embedded-methodology: true"   (the comment gone)
```

**Control — the same two files, the same bytes, in a repository with one commit** (`bare` rig):
`jigc setup` exits **1**, `setup.dirty-install-path` names `.jigc/AGENT.md` and `.jigc/config/packs.yaml`,
*"nothing was installed"*, and `command grep -rc USERMARK .jigc` still answers `1` and `1`.

**Repro B — the route through M54's S22 cell (a failed first run, then a user edit).**

```text
setup   same unborn repository, but no identity: git config user.useConfigOnly true (no user.name/user.email)
argv    jigc setup                    → exit 1, setup.install-commit ("Author identity unknown … written and staged")
        git status --porcelain -uall  → A  on all nine install paths (root .gitignore included)
        (cause removed: identity set, useConfigOnly unset)
arm 1   printf '\n# USERMARK-7f3a edit after the failure\n' >> .jigc/AGENT.md        (status: AM)
        jigc setup                    → exit 1, setup.dirty-install-path names .jigc/AGENT.md; mark still 1      ← re-arms
arm 2   git rm -r -q --cached .       (the user unstages what the failed run staged; status: ?? on all nine)
        printf '\n# USERMARK-7f3a edit after the failure\n' >> .jigc/AGENT.md        before-control: mark = 1
        jigc setup                    → exit 0, install_commit 672e939, findings: []
        mark in .jigc/AGENT.md = 0 · git log --all -S USERMARK-7f3a → 0 commits · status empty                    ← does not re-arm
```

**The neighbouring unborn cells, for the boundary** (same construction, one planted file each):

| planted before `jigc setup` (unborn HEAD) | exit | code | what happened to the bytes |
|---|---|---|---|
| untracked `CLAUDE.md` | 0 | none | merged into; the user's bytes ride the install commit (mark in HEAD = 1) — the stated door rule |
| untracked `.jigc/AGENT.md` | 0 | none | **gone** — worktree 0, HEAD 0, `git log --all -S` 0 |
| untracked `.jigc/config/packs.yaml` (a comment line) | 0 | none | **comment gone** — same three zeros |
| **staged** `CLAUDE.md` | 1 | `setup.dirty-install-path` | preserved, nothing installed |
| **staged** `.jigc/AGENT.md` | 1 | `setup.dirty-install-path` | preserved, nothing installed |

**Proposed tier: 1** — exit 0, a committing door, and user bytes that exist in no worktree file, no commit
and no object afterwards. **Two caveats the reconciler should weigh, stated rather than buried:** (i) the
precondition is narrow — a repository with zero commits holding a human-authored file at a path only jigc
normally writes (a copied `.jigc/` from a template, or repro B's unstage-then-edit); (ii) the unborn `??`
exclusion is a stated door rule in `design/validation.md`. If that rule is read as covering the whole-rewrite
paths too, what remains is tier 3: QUICKSTART's sentence is false on an unborn HEAD.

### (R1, F2) — `setup --force` over a dirty hook does not name it

**Contract.** `design/validation.md` → `setup.dirty-install-path` row: *"`--force` is consent, not
preservation — it lets the install run over those bytes, and says so through the advisory
`setup.forced-install-path` … naming every path the consent was spent on."* The scope's cell 4: *"the
forced-path advisory names what the consent was spent on."*

```text
setup   bare rig; mkdir .githooks; printf '#!/bin/sh\necho "FOREIGN ran" >&2\n' > .githooks/pre-commit; chmod 755 …
        git add .githooks; git commit -qm "chore: team hook"; git config core.hooksPath .githooks
        printf 'echo "USERMARK-7f3a uncommitted line" >&2\n' >> .githooks/pre-commit      (status:  M .githooks/pre-commit)
argv    jigc setup --force --format json
exit    0     {"findings": [], "hook_committed": true, "hook_file": ".githooks/pre-commit", "install_commit": "08fd559", …}
after   git show HEAD:.githooks/pre-commit | command grep -c USERMARK-7f3a  → 1    (the uncommitted line is in the install commit)
        no setup.forced-install-path finding

control same, with CLAUDE.md ALSO carrying an uncommitted line
argv    jigc setup --force --format json
exit    0     setup.forced-install-path: "`--force` consented over 1 install path(s) …:  `CLAUDE.md`"
after   mark in HEAD:.githooks/pre-commit = 1 · mark in HEAD:CLAUDE.md = 1          (two paths consumed, one named)
```

The non-forced arm is right: the same dirty hook without `--force` exits 1 with `setup.dirty-install-path`
naming `.githooks/pre-commit` (the backstop ask), HEAD untouched. **Proposed tier: 3** — the advisory under-states
what the consent was spent on; the hook writer preserves the foreign bytes verbatim and they ride the commit,
so nothing is lost.

### (R1, F3) — a jigc hook from another path or another build is wrapped as foreign

**Contract.** `design/assistant-adapter.md` → *`jigc setup` has assistant-neutral install responsibilities
too*: the hook install is *"**idempotent** (a sentinel-marked block, re-runnable, and **regenerated on every
`setup`** like the managed `.jigc/AGENT.md`)"*, and the declared bound (ii): *"on another clone the tracked hook
no-ops until that developer's own `jigc setup` **rewrites it to their path** (which setup then commits)."*
QUICKSTART → *Install*: a binary placed elsewhere by hand is supported, with `jigc setup` re-run after.
`jigc uninstall`'s own summary line: *"removed pre-commit hook"* / `"precommit": true`.

```text
setup   fresh rig (hook written by ~/.local/bin/jigc; 65 lines, 1 start sentinel, line 9: jigc='~/.local/bin/jigc')
        BIN=$(mktemp -d "$RIG/cargo-bin.XXXXXX"); ln -s ~/.local/bin/jigc "$BIN/jigc"      (the same build at a second path)
argv    "$BIN/jigc" setup                         → exit 0, summary lists the hook as installed
after   .git/hooks/pre-commit: 129 lines, 2 start sentinels, 1 end marker
        line  9: jigc='$RIG/cargo-bin.…/jigc'      line 73: jigc='~/.local/bin/jigc'       (the old hook kept as "foreign")
argv    setup again from a THIRD path             → still 2 blocks; line 9 follows, line 73 stays at the first path
argv    ~/.local/bin/jigc setup (the first path)  → still 2 blocks, both now naming the first path
argv    jigc uninstall --format json              → exit 0, "removed": {… "precommit": true …}
after   .git/hooks/pre-commit STILL PRESENT: a standalone jigc hook, 1 sentinel, jigc='~/.local/bin/jigc'
argv    jigc uninstall  (a second time)           → exit 0, "- removed pre-commit hook"; the file is gone
```

Three more arms of the same seam:

- **Template drift, same path.** `fresh` rig; one comment line of the installed hook reworded (standing in
  for a hook body written by a build whose template differs); `jigc setup` → exit 0; the hook has 2 start
  sentinels, 129 lines, and the reworded line is still in it. *(The template is byte-identical at the
  `jigc-v1.0.0-rc.22`, `-rc.23` and `-rc.24` tags — read, 192 source lines, one hash — so no published
  version-to-version step triggers this arm today; the path arm is the live one.)*
- **A committed hook met by another machine's `jigc`.** `bare` rig, `git config core.hooksPath .githooks`,
  `jigc setup` (hook committed); then `"$BIN/jigc" setup --format json` from a second path → exit 0,
  `install_commit` made, `hook_committed: true`; `git show HEAD:.githooks/pre-commit` has **2** sentinels,
  line 9 the second path and line 73 the first. The commit adds 64 lines to the hook; it rewrites nothing.
- **Observable effect on a commit.** `vendored` rig, setup re-run through a symlinked binary, the anchored
  symbol renamed and committed with plain git: exit 0, and the drift warning prints **twice** (one sweep per
  block).

**Proposed tier: 3** — *regenerated on every setup*, *rewrites it to their path* and *removed pre-commit
hook* are each something the binary does not do when the path or the template differs; no bytes are lost
and both blocks stay warn-only. The stale block keeps running whatever binary still sits at the old path.

## The (door, cell) table

Route kind: **H** = a human route on the finding · **M** = a mechanical command · **I** = informational ·
none. `sdip` = `setup.dirty-install-path`, `sfip` = `setup.forced-install-path`. Every row below ran on the
installed binary (or, for the non-leaf doors, the named tool) and has its repro block in the next section.

### Cell 1 — install shapes (door `setup`)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 1.1 | established repo, first run (`bare`) | `jigc setup` | 0 | none | none | `install commit → <sha>`; 8 files in the commit; status empty | matches |
| 1.2 | re-run | `jigc setup`, `jigc setup --format json` | 0 | none | none | HEAD unchanged; `"install_commit": null` | matches |
| 1.3 | established with its own `CLAUDE.md`, `.gitignore`, `.claude/settings.json` | `jigc setup --format json` then re-run | 0 / 0 | none | none | user `allow`, `deny`, `SessionStart` and `model` kept; root `.gitignore` hash unchanged; re-run byte-identical over 10 worktree files + the hook | matches |
| 1.4 | HEAD unborn (plain `git init`) | `jigc setup --format json` | 0 | none | none | 9 files in the first commit, secrets floor seeded | matches |
| 1.5 | unborn, re-run | `jigc setup --format json` | 0 | none | none | HEAD unchanged, `.gitignore` hash unchanged | matches |
| 1.6 | unborn with a pre-existing untracked root `.gitignore` | `jigc setup --format json` | 0 | none | none | user lines kept, floor block appended after them | matches (never clobbers) — see O6 |
| 1.7 | the rig's standalone `--git-state unborn` form | `jigc setup --format json` | 0 | none | none | 9-file first commit | matches |
| 1.8 | upgrade over an older **committed** `.jigc/version` | `jigc validate --format json` · `jigc setup --format json` · `jigc validate` | 0 / 0 / 0 | `store-version.binary-mismatch` (advisory) then none | H | restamp commit touches `.jigc/version` only; the advisory is gone after | matches |
| 1.9 | older stamp present but uncommitted (jigc-shaped → exempt) | `jigc setup --format json` | 0 | none | none | restamped, status empty | matches |

### Cell 2 — the failed first run (S22) (door `setup`)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 2.1 | read-only in-repo `core.hooksPath`; then plain re-run | `jigc setup` ×2 | 1 / 0 | `setup.install-hook` / none | H | failure leaves 8 paths `A `; re-run commits 9 files incl. `hooks/pre-commit`, no `--force` | matches — see O1 |
| 2.2 | 2.1 + a user edit to `CLAUDE.md` and `.jigc/AGENT.md` before the re-run | `jigc setup --format json` | 1 | `sdip` | H | names exactly the two edited paths; marks kept; HEAD unmoved | matches (re-arms) |
| 2.3 | commit step fails: no identity; plain re-run | `jigc setup` ×2 | 1 / 0 | `setup.install-commit` / none | H | *"written and staged, but `git commit` was rejected"*; re-run lands the commit | matches |
| 2.4 | 2.3 + user edit to `.jigc/AGENT.md` | `jigc setup --format json` | 1 | `sdip` | H | names `.jigc/AGENT.md`; mark kept | matches (re-arms) |
| 2.5 | commit step fails: failing signing program; plain re-run | `jigc setup` ×2 | 1 / 0 | `setup.install-commit` / none | H | git's `gpg failed to sign the data` quoted; re-run lands | matches |
| 2.6 | 2.5 + user edit | `jigc setup --format json` | 1 | `sdip` | H | as 2.4 | matches (re-arms) |
| 2.7 | **unborn HEAD**, no identity; plain re-run | `jigc setup` ×2 | 1 / 0 | `setup.install-commit` / none | H | 9 paths `A `; re-run mints the first commit | matches |
| 2.8 | 2.7 + user edit (index kept) | `jigc setup --format json` | 1 | `sdip` | H | names `.jigc/AGENT.md`; mark kept | matches (re-arms) |
| 2.9 | 2.7 + the user unstages everything, then edits `.jigc/AGENT.md` | `jigc setup --format json` | **0** | none | none | `findings: []`, install commit made, the edit in no object | **DEFECT — (R1, F1) repro B** |
| 2.10 | read-only `.claude/` directory; plain re-run | `jigc setup` ×2 | 1 / 0 | `setup.write-guide` / none | H | failure leaves 6 `A ` + `M  .claude/settings.json`; re-run lands; user entry kept | matches |
| 2.11 | read-only `.claude/settings.json`; plain re-run | `jigc setup` ×2 | 1 / 0 | `setup.inject-allowlist` / none | H | re-run lands | matches |
| 2.12 | detached HEAD at a first run; re-attach; re-run | `jigc setup` ×2 | 1 / 0 | `repo.head-detached` / none | M | nothing written at the refusal (status empty) | matches |
| 2.13 | after a failed run: user bytes staged at `.jigc/AGENT.md` (index = worktree) | `jigc setup --format json` | 1 | `sdip` | H | staged blob and worktree keep the mark | matches (index axis) |
| 2.14 | …index-only (staged user blob, worktree back to jigc's bytes) | same | 1 | `sdip` | H | staged blob keeps the mark | matches (the worse loss class refused) |
| 2.15 | …worktree overwritten in place, same size | same | 1 | `sdip` | H | mark kept | matches (bytes, not size) |
| 2.16 | …user deletes `CLAUDE.md` after the failure | same | 1 | `sdip` | H | fires at the backstop (*"written and staged"*) | matches (loud, not lossy) |
| 2.17 | a failed `--force` run (dirty `CLAUDE.md` + read-only hooks), then a plain re-run | `jigc setup --force` · `jigc setup --format json` | 1 / 1 | `setup.install-hook` / `sdip` (6 paths) | H | the forced failure staged nothing (`??`); the plain re-run refuses | matches (*a failed `--force` run records nothing*) |
| 2.18 | failed first run, then unrelated work staged, plain re-run | `jigc setup` | 0 | none | none | the install commit carries no user path; `A  staged.txt`, ` M README.md` still there | matches |

### Cell 3 — the settings pre-check (door `setup`)

Each row: the body committed as `.claude/settings.json` on a `bare` rig; `jigc setup --format json`; then the
file fixed to `{"permissions":{"allow":["Bash(mine:*)"]}}`, committed, and `jigc setup --format json` again.

| # | committed body | exit | code | message (the parser's own) | nothing written? | re-run | user entry kept | verdict |
|---|---|---|---|---|---|---|---|---|
| 3.1 | trailing commas | 1 | `setup.inject-allowlist` | `trailing comma at line 1 column 42` | yes — no `.jigc`, no `CLAUDE.md`, no hook, status empty | 0 | yes | matches |
| 3.2 | a JSON array root | 1 | same | `settings root is not a JSON object` | yes | 0 | yes | matches |
| 3.3 | `permissions.allow` a string | 1 | same | `` `permissions.allow` is not a JSON array `` | yes | 0 | yes | matches |
| 3.4 | `permissions: 5` | 1 | same | `` `permissions` is not a JSON object `` | yes | 0 | yes | matches |
| 3.5 | `hooks: 3` | 1 | same | `` `hooks` is not a JSON object `` | yes | 0 | yes | matches |
| 3.6 | `hooks.SessionStart` a string | 1 | same | `` `hooks.<event>` is not a JSON array `` | yes | 0 | yes | matches |
| 3.7 | `permissions.deny` a string | 1 | same | `` `permissions.deny` is not a JSON array `` | yes | 0 | yes | matches |
| 3.8 | an empty file | 1 | same | `EOF while parsing a value at line 2 column 0` | yes | 0 | yes | matches |
| 3.9 | `null` | 1 | same | `settings root is not a JSON object` | yes | 0 | yes | matches |
| 3.10 | `{"hooks":{"SessionStart":[5]}}` | 0 | none | — | — | — | the `5` kept, jigc's entry appended | matches (merged around) |
| 3.11 | `{"hooks":{"SessionStart":[{"hooks":"x"}]}}` | 0 | none | — | — | — | kept, appended | matches |
| 3.12 | `{"permissions":{"allow":[5,{"a":1}]}}` | 0 | none | — | — | — | kept, appended | matches |

Route on every refusal: H — *"fix `.claude/settings.json` so jigc can merge into it, commit the fix, then
re-run `jigc setup`"*.

### Cell 4 — the dirty-install guard (door `setup`)

**4a — an installed repo (`fresh`), one member carrying an unstaged edit; plain, then `--force`.**

| # | member | plain: exit · code · bytes | `--force`: exit · code · what the consent cost | verdict |
|---|---|---|---|---|
| 4.1 / 4.2 | `CLAUDE.md` | 1 · `sdip` · preserved, HEAD unmoved | 0 · `sfip` names it · merged-into, the edit rides the commit | matches |
| 4.3 / 4.4 | `.claude/settings.json` | 1 · `sdip` · preserved | 0 · `sfip` names it · the edit rides the commit | matches |
| 4.5 / 4.6 | `.jigc/AGENT.md` | 1 · `sdip` · preserved | 0 · `sfip` names it · **regenerated; the edit is in no object** — the advisory says so | matches (consent, declared) |
| 4.7 / 4.8 | `.jigc/.gitignore` | 1 · `sdip` · preserved | 0 · `sfip` names it · the extra line rides the commit | matches |
| 4.9 / 4.10 | `.jigc/version` (prose appended → not jigc's shape) | 1 · `sdip` · preserved | 0 · `sfip` names it · restamped, edit gone, said | matches |
| 4.11 / 4.12 | `.jigc/config/.gitkeep` | 1 · `sdip` · preserved | 0 · `sfip` names it · emptied, said | matches |
| 4.13 / 4.14 | `.jigc/config/packs.yaml` | 1 · `sdip` · preserved | 0 · `sfip` names it · comment gone, said | matches |
| 4.15 / 4.16 | the guide `.claude/skills/jigc/SKILL.md` | 0 · `adapter-guide.user-modified` (advisory) · left byte-identical, dropped from the commit | 0 · same advisory · still left alone | matches (refuse-to-clobber) |

**4b — an established, never-installed repo (`bare`), one untracked user file at the member's path; plain.**

| # | member | exit | code | bytes | verdict |
|---|---|---|---|---|---|
| 4.17 | `CLAUDE.md` | 1 | `sdip` | preserved, nothing installed | matches |
| 4.18 | `.claude/settings.json` | 1 | `sdip` | preserved | matches |
| 4.19 | `.jigc/AGENT.md` | 1 | `sdip` | preserved | matches |
| 4.20 | `.jigc/config/packs.yaml` | 1 | `sdip` | preserved | matches |
| 4.21 | `.jigc/.gitignore` | 1 | `sdip` | preserved | matches |
| 4.22 | `.jigc/version` (prose) | 1 | `sdip` | preserved | matches |
| 4.23 | the guide path | 0 | `adapter-guide.user-modified` | left untracked and untouched; the rest installs and commits | matches — see O2 |
| 4.24 | root `.gitignore` (not a member on an established repo) | 0 | none | untouched, still untracked | matches (*never touch an existing project's `.gitignore`*) |

**4c — unborn HEAD, one planted file.**

| # | planted | exit | code | bytes | verdict |
|---|---|---|---|---|---|
| 4.25 | untracked `CLAUDE.md` | 0 | none | merged into, ride the first commit | matches the stated unborn door rule |
| 4.26 | untracked `.jigc/AGENT.md` | **0** | none | **gone, in no object** | **DEFECT — (R1, F1)** |
| 4.27 | untracked `.jigc/config/packs.yaml` | **0** | none | **comment gone, in no object** | **DEFECT — (R1, F1)** |
| 4.28 | staged `CLAUDE.md` | 1 | `sdip` | preserved | matches |
| 4.29 | staged `.jigc/AGENT.md` | 1 | `sdip` | preserved | matches |
| 4.30 | the F1 control: the same two files on a born HEAD | 1 | `sdip` (2 paths) | preserved | matches |

**4d — the hook member (in-repo `core.hooksPath`, a foreign tracked hook carrying an uncommitted line).**

| # | cell | argv | exit | code | surface asserted | verdict |
|---|---|---|---|---|---|---|
| 4.31 | dirty foreign hook, plain | `jigc setup --format json` | 1 | `sdip` names `.githooks/pre-commit` | *"the install files are written and staged, and no install commit was made"*; 8 paths `A `, hook ` M`; the user's line still in the file | matches (the backstop) — see O3 |
| 4.32 | 4.31, then `--force` | `jigc setup --force --format json` | 0 | **none** | `findings: []`; the user's line is in HEAD's hook | **DEFECT — (R1, F2)** |
| 4.33 | dirty foreign hook, `--force` as the first run | `jigc setup --force --format json` | 0 | **none** | as 4.32 | **DEFECT — (R1, F2)** |
| 4.34 | dirty hook **and** dirty `CLAUDE.md`, `--force` | same | 0 | `sfip` — *"1 install path(s)"*, `CLAUDE.md` only | both edits in HEAD | **DEFECT — (R1, F2)** |
| 4.35 | staged-only difference on the foreign hook, plain | `jigc setup --format json` | 1 | `sdip` names the hook | status `MM` on the hook: the staged blob is held back | matches |
| 4.36 | unrelated staged + unstaged + untracked work at a first run | `jigc setup` | 0 | none | the install commit names none of the three; all three still in `git status` | matches (the pathspec bounds the commit) |

### Cell 5 — the hook (doors: `setup`, `uninstall`, the installed hook through `git commit`)

| # | cell | argv | exit | code | surface asserted | verdict |
|---|---|---|---|---|---|---|
| 5.1 | jigc's own hook | `jigc setup` (1.3) · `sh -n .git/hooks/pre-commit` | 0 / 0 | none | standalone, 65 lines, executable, absolute `jigc=` path, summary names `.git/hooks/pre-commit` and says *"not in the install commit; a clone gets no drift backstop"*; JSON `hook_committed: false` | matches |
| 5.2 | a foreign hook present before setup | `jigc setup --format json` | 0 | none | shebang kept, jigc block (start + end marker) before the foreign body, foreign lines verbatim, mode `rwxr-xr-x` | matches |
| 5.3 | commit through the wrapped hook, foreign passes | `git commit -m "feat: a"` | 0 | — | `FOREIGN-HOOK ran`; commit lands | matches |
| 5.4 | commit the foreign hook rejects | `git commit -m "feat: forbidden"` | 1 | — | `FOREIGN: forbidden file`; the foreign hook still owns the exit | matches |
| 5.5 | re-run over a wrapped hook | `jigc setup` | 0 | none | hook hash unchanged, 1 start sentinel | matches (idempotent) |
| 5.6 | `uninstall` restoring the wrapped foreign hook | `jigc uninstall --format json` | 0 | none | `cmp` against the pre-setup copy: byte-identical, mode kept; `"precommit": true` | matches (`RestoreForeign`) |
| 5.7 | `uninstall` over jigc's standalone hook | `jigc uninstall` | 0 | none | the file is gone | matches (`RemoveFile`) |
| 5.8 | hook replaced by a purely foreign one after setup | `jigc uninstall --format json` | 0 | none | `"precommit": false`; file hash unchanged | matches (`NotOurs`) |
| 5.9 | `core.hooksPath` **outside** the repository | `jigc setup` · `git commit` · `jigc uninstall` | 0 / 0 / 0 | none | summary prints the absolute resolved path and the *local to this checkout* clause; the commit carries no hook; uninstall empties the outside dir | matches |
| 5.10 | `core.hooksPath` **inside** (`.githooks`) | `jigc setup --format json` · re-run | 0 / 0 | none | `hook_file: .githooks/pre-commit`, `hook_committed: true`, mode `100755` in the index; re-run moves nothing | matches |
| 5.11 | in-repo hooksPath holding a clean foreign tracked hook | `jigc setup --format json` | 0 | none | wrapped and committed (`hook_committed: true`) | matches |
| 5.12 | `uninstall`, in-repo hooksPath, jigc's own tracked hook | `jigc uninstall` | 0 | none | ` D .githooks/pre-commit` | matches |
| 5.13 | `uninstall`, in-repo hooksPath, wrapped tracked foreign hook | `jigc uninstall` | 0 | none | worktree file byte-identical to its pre-setup commit | matches |
| 5.14 | a repository path containing a space **and** a single quote | `jigc setup` · `git commit` · `sh -n` | 0 / 0 / 0 | none | summary path relative; a clean commit prints nothing | matches |
| 5.15 | the rename block under a repository path containing a space (`committed-singletons` under a spaced scratch root) | `git mv docs/roadmap.md docs/plan.md; git commit` | 1 | — | *"out-of-band managed-doc rename staged in this commit … (commit blocked)"*; a clean unrelated commit after `reset --hard` exits 0 silently | matches |
| 5.16 | the binary reached through a symlink | `"$BIN/jigc" setup` · `git commit` | 0 / 0 | none | the hook embeds the **symlink** path as invoked | matches |
| 5.17 | the binary resolved from `PATH` through that symlink dir | `PATH="$BIN:$PATH" jigc setup` | 0 | none | the hook embeds the resolved symlink path, absolute | matches |
| 5.18 | a hook written through the symlink reaches the probe (`vendored`, drift) | `"$BIN/jigc" setup` · `git commit` | 0 / 0 | — | the drift warning prints — **twice** | reaches the probe; the doubling is **(R1, F3)** |
| 5.19 | real commit, no drift, no rename (`vendored`) | `git commit -m "chore: note"` | 0 | — | no `jigc:` line | matches (silence) |
| 5.20 | real commit that drifts an anchored symbol | `git commit -m "refactor: rename pad"` | 0 | — | *"jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked)."*; `validate`: `blocking_probes: ["doc-code"]`, `doc-code.symbol-exists` | matches (warn, non-blocking) |
| 5.21 | a later unrelated commit over that drift | `git commit -m "chore: note 2"` | 0 | — | the same warning, commit lands | matches |
| 5.22 | rename block — location doctype `spec` | `git mv docs/specs/padding.md docs/specs/padding-v2.md; git commit` | 1 | — | *"… (commit blocked)."*; HEAD unmoved | matches |
| 5.23 | rename block — location doctype `arch-doc` | `git mv docs/architecture/padding-layer.md docs/architecture/pad-layer.md; git commit` | 1 | — | same | matches |
| 5.24 | rename block — placement singleton `vision`, root → root | `git mv VISION.md VISION-old.md; git commit` | 1 | — | same; `validate` exit 1 with `reconciliation.rename` | matches |
| 5.25 | …`vision`, root → `docs/` | `git mv VISION.md docs/vision.md; git commit` | 1 | — | same | matches |
| 5.26 | …`changelog` | `git mv CHANGELOG.md HISTORY.md; git commit` | 1 | — | same | matches |
| 5.27 | …`roadmap` | `git mv docs/roadmap.md docs/plan.md; git commit` | 1 | — | same | matches |
| 5.28 | …`decisions-log` | `git mv docs/decisions-log.md docs/decisions.md; git commit` | 1 | — | same | matches |
| 5.29 | a move landed earlier with `--no-verify`, then an unrelated commit | `git commit -m "chore: unrelated"` | 0 | — | *"an out-of-band managed-doc rename exists in the committed tree … (not staged in this commit; commit not blocked)."* | matches (warn arm) |
| 5.30 | a plain `mv` left unstaged in the tree, then an unrelated commit | same | 0 | — | the same warn line | matches |
| 5.31 | a `git mv` **plus a one-line edit** of `VISION.md`, both paths staged (`R093`) | `git commit -m "docs: move+edit"` | 0 | — | no `jigc:` line at all; the commit lands | matches the design's letter (weak signal, `design/reconciliation.md`) — see O4 |
| 5.32 | the hook when its `jigc` path no longer exists | `git commit -m "feat: a"` | 0 | — | silent | matches |
| 5.33 | the `jigc` path changes between two setups | `"$BIN/jigc" setup` over a `~/.local/bin/jigc` hook | 0 | none | 2 jigc blocks, the old one kept at the old path | **DEFECT — (R1, F3)** |
| 5.34 | a standalone jigc hook whose template differs, same path | `jigc setup` | 0 | none | 2 jigc blocks, the old body kept | **DEFECT — (R1, F3)** |
| 5.35 | `uninstall` after 5.33 | `jigc uninstall` ×2 | 0 / 0 | none | first: `"precommit": true` and a jigc hook still on disk; second: removed | **DEFECT — (R1, F3)** |
| 5.36 | a committed hook met by a second `jigc` path | `"$BIN/jigc" setup --format json` | 0 | none | the install commit adds a second block to the tracked hook | **DEFECT — (R1, F3)** |
| 5.37 | `IGNORE_DOORS` row `jigc setup`: an older committed `.jigc/.gitignore` entry set | `jigc setup` | 0 | none | *"`.jigc/.gitignore` → appended displaced/   (jigc's transient-runtime entries; every line already in the file was kept)"*; the commit touches that one file | matches (the append is named) |
| 5.38 | the same under `--format json` | `jigc setup --format json` | 0 | none | the same line on stderr beside the envelope | matches |

### `uninstall` — only where it reverses what setup wrote

| # | cell | argv | exit | code | surface asserted | verdict |
|---|---|---|---|---|---|---|
| U.1 | established repo with its own host files, setup then uninstall | `jigc uninstall --format json` | 0 | none | `CLAUDE.md` byte-identical to its pre-setup commit; `.claude/settings.json` equal as JSON to the pre-setup file (re-serialized, so not byte-equal); `.jigc/`, the skill dir and the hook gone; stderr names the 5 tracked files `.jigc/` removal takes and how to bring each back | matches |
| U.2 | a repository setup itself birthed (secrets floor seeded) | `jigc uninstall --format json` | 0 | none | the seeded root `.gitignore` stays; everything else reversed | matches |
| U.3 | uninstall after a **failed** first setup | `jigc uninstall` | 0 | none | the index is left with `AD`/`AM` entries for the removed files | declared open — owed at M57 (`design/project-setup.md` → *Idempotency & irreversibility*) |

### Cell 6 — install (non-leaf doors; no leaf coverage)

| # | cell | argv | exit | surface asserted | verdict |
|---|---|---|---|---|---|
| 6.1 | the QUICKSTART line into a private root | `cargo install jigc --version '^1.0.0-rc.1' --locked --root "$R"` — **`--root "$R"` is the one token added** | 0 | `Installing jigc v1.0.0-rc.24` · 67 crates compiled · `Installed package jigc v1.0.0-rc.24 (executable jigc)` | matches |
| 6.2 | one file in the install root's `bin/` | `ls "$R/bin"` · `cargo install --list --root "$R"` | 0 | exactly `jigc`; the list names one executable | matches |
| 6.3 | the installed binary identifies itself | `"$R/bin/jigc" --version` | 0 | `jigc 1.0.0-rc.24` | matches |
| 6.4 | the unpinned line resolves nothing | `cargo install jigc --locked --root "$R2"` | 101 | ``error: could not find `jigc` in registry `crates-io` with version `*` ``; the root stays empty | matches |

### Cell 7 — package (non-leaf doors)

| # | cell | argv | exit | surface asserted | verdict |
|---|---|---|---|---|---|
| 7.1 | `jigc`'s listing equals the allowlist from both sides | `cargo package --list -p jigc` | 0 | 195 paths: 4 cargo-generated + `README.md`, `LICENSE-MIT`, `LICENSE-APACHE` + `src/` 37, `packs/` 148, `adapters/` 1, `guides/` 2. Outside the allowlist: none. Under `tests/`: none. `git ls-files` under the four directories: 188, every one listed, none listed that is not tracked | matches — see O5 |
| 7.2 | `jigc-engine`'s listing, as it is | `cargo package --list -p jigc-engine` | 0 | 107 paths: `src/` 93, `proptest-regressions/` 7, and at the root `.cargo_vcs_info.json`, `CHANGELOG.md`, `Cargo.lock`, `Cargo.toml`, `Cargo.toml.orig`, `LICENSE-APACHE`, `LICENSE-MIT`. No README (declared). No allowlist | recorded |
| 7.3 | the packaged README is the generated crate README | `dev/crate-readme --stdout` · `cmp` | 0 | generator output == committed `crates/cli/README.md` == `README.md` inside the **published** `jigc-1.0.0-rc.24.crate`; all six link targets absolute under `…/blob/HEAD/` | matches |
| 7.4 | the published crates against the local listing | `curl` the two `.crate` files from `static.crates.io`, unpack, `diff` | 0 | SHA-256 of each equals its index `cksum`; published `jigc` file list == `cargo package --list -p jigc` (195); published engine list == local (107); `src/`, `packs/`, `guides/` `diff -rq`-equal to the tree; license pair byte-equal to the root's, regular files; no `tests/`; `Cargo.toml` pins `jigc-engine` `=0.1.0-rc.2`; `.cargo_vcs_info.json` → `91834b5e…` (`crates/cli`) and `bd6e1a44…` (`crates/engine`) | matches |
| 7.5 | the publish dry-run, both crates | `cargo publish --workspace --dry-run` (repository root, private `CARGO_TARGET_DIR`) | 0 | verbatim: ``warning: crate jigc-engine@0.1.0-rc.2 already exists on crates.io index`` · ``warning: crate jigc@1.0.0-rc.24 already exists on crates.io index`` · `Packaged 107 files` · `Packaged 195 files` · twelve ``ignoring test `g_*` `` warnings · both `Verifying` builds finish · `Uploading jigc-engine …` then `Uploading jigc …`, each `aborting upload due to dry run` | matches (the registry-state warnings are not graded) |

### Cell 8 — release (non-leaf doors)

| # | cell | argv | exit | surface asserted | verdict |
|---|---|---|---|---|---|
| 8.1 | nothing missing at this commit | `dev/unpublished-versions` | 0 | `jigc@1.0.0-rc.24 published` · `jigc-engine@0.1.0-rc.2 published` · `missing=false` | matches |
| 8.2 | the registry's own state | `curl https://index.crates.io/ji/gc/jigc` and `…/jigc-engine` | 0 (HTTP 200) | `jigc`: `0.0.0` yanked, `1.0.0-rc.22`, `-rc.23`, `-rc.24` unyanked; `jigc-engine`: `0.0.0` yanked, `0.1.0-rc.1`, `-rc.2` unyanked | matches |
| 8.3 | the tarball acceptance | `dev/runner-faithful tarball` (commit `bffa6667`, `linux/aarch64`, 8 CPUs, git 2.43.0, rustc 1.95.0) | 0 | `ok` on `package`, `lock-compare`, `install`, `version` (`jigc 1.0.0-rc.24`), `setup`, `probe-override`, `control-finalize` (exit 0), `drift-finalize` (exit 3, `doc-code.symbol-exists`, HEAD unchanged) | matches |
| 8.4 | the registry acceptance | `dev/runner-faithful registry '^1.0.0-rc.1'` (`linux/aarch64`) | 0 | `ok install: cargo install jigc --version ^1.0.0-rc.1 --locked installed jigc 1.0.0-rc.24`, then the same five post-install steps `ok` | matches |
| 8.5 | the registry acceptance, unpinned | `dev/runner-faithful registry` | 1 | `FAIL install: cargo install jigc --locked (unpinned) exited 101` · ``could not find `jigc` in registry `crates-io` with version `*` `` | matches (*resolves nothing*) |
| 8.6 | the tarball acceptance on four CPUs | `dev/runner-faithful --cpus 4 tarball` | 0 | `cpus 4`; every step `ok` | matches |
| 8.7 | the registry acceptance on the runner's architecture | `dev/runner-faithful --platform linux/amd64 registry '^1.0.0-rc.1'` | 0 | `platform linux/x86_64`; every step `ok` | matches |

### Classification and fence rows — recorded, not driven, no coverage

| # | row | how it was established | result |
|---|---|---|---|
| C.1 | the installed binary is a registry build | `strings` over the binary, counted | registry paths only (see *Standing facts*) |
| C.2 | every `uses:` in both workflows is a 40-hex pin with its tag comment | a read of `.github/workflows/ci.yml` and `release.yml` | 23 `uses:` lines, 23 match `<action>@<40 hex> # v<tag>`; 4 distinct actions — `actions/checkout@fbc6f399… # v5.1.0` ×12, `Swatinem/rust-cache@6323deb1… # v2.9.2` ×8, `release-plz/git-config@59144859… # v0.1.2` ×2, `actions/create-github-app-token@bcd2ba49… # v3.2.0` ×1 — the four rows of `implementation/release.md` → *Action pins* |
| C.3 | the `package_contents` fence | `cargo test -p jigc --test g_migrate package_contents::` (debug test target, private target dir) | exit 0, 5 passed |
| C.4 | the release as GitHub shows it | `gh release view`, `git rev-parse <tag>^{commit}`, `gh run list` (reads) | `jigc-v1.0.0-rc.24` → `91834b5e…`, prerelease, not draft; `jigc-engine-v0.1.0-rc.2` → `bd6e1a44…`, prerelease; the Release and CI runs on `aa6666cb` concluded `success` |

## Repro blocks

Paths are `$REPO` / `$RIG` / `<tmp>` / `~`. Every `jigc` below is `~/.local/bin/jigc` unless a symlink is
named. The F1, F2 and F3 blocks are in *Findings* above and are not repeated.

**Cell 1.**

```text
1.1  bare rig.  jigc setup → exit 0
     "- pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
        local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there"
     "- install commit → 9d68ad2"
     git show --stat HEAD → .claude/settings.json, .claude/skills/jigc/SKILL.md, .jigc/.gitignore, .jigc/AGENT.md,
                            .jigc/config/.gitkeep, .jigc/config/packs.yaml, .jigc/version, CLAUDE.md   (8 files)
     git status --porcelain → empty
1.2  jigc setup → exit 0, no "install commit" line, HEAD unchanged.   jigc setup --format json → exit 0
     {"allowlist_file": ".claude/settings.json", "findings": [], "guide_file": ".claude/skills/jigc/SKILL.md",
      "hook_committed": false, "hook_file": ".git/hooks/pre-commit", "install_commit": null, "line_file": "CLAUDE.md"}
1.3  bare rig; CLAUDE.md ("# My project … House rules"), .gitignore ("node_modules/\n.env"),
     .claude/settings.json (allow "Bash(npm test:*)", deny "Read(./secrets/**)", a SessionStart "echo mine", "model": "opus")
     committed.  jigc setup --format json → exit 0, install_commit 2958ccf
     CLAUDE.md = the user's three lines + "## Project interface" + "@.jigc/AGENT.md"
     settings: the user's allow/deny/hook/model all present, jigc's appended after them
     .gitignore shasum unchanged; not in the commit
     re-run: shasum over every worktree file + the hook, before == after (10 files + the hook); HEAD unchanged; status empty
1.4  bare rig for HOME; F=$(mktemp -d "$RIG/unborn.XXXXXX"); git init; local identity.
     git rev-parse --verify HEAD → "fatal: Needed a single revision"
     jigc setup --format json → exit 0, install_commit b56f55b; 9 files (the 8 + .gitignore, 13 lines: the secrets floor)
1.5  re-run → exit 0, install_commit null, HEAD unchanged, .gitignore shasum OK
1.6  same, with an untracked .gitignore "target/\n.env" first → exit 0; the file is the two user lines, a blank
     line, then the 13-line floor block
1.7  rig=$(dev/jigc-rig --git-state unborn --binary ~/.local/bin/jigc) || exit; eval "$rig"
     (.jigc/config/ exists and is empty)   jigc setup --format json → exit 0, install_commit 875ab28, 9 files
1.8  fresh rig; printf 'jigc-version: 1.0.0-rc.21\n' > .jigc/version; git commit -qam …
     jigc validate --format json → exit 0, store-version.binary-mismatch (advisory):
       "store last written by jigc 1.0.0-rc.21; you are running 1.0.0-rc.24 — align versions or re-run `jigc setup`"
     jigc setup --format json → exit 0, install_commit 2a36548;  git show --stat → .jigc/version | 2 +-  (that file only)
     jigc validate --format json → exit 0, findings: []
1.9  fresh rig; the same stamp written, NOT committed ( M .jigc/version)
     jigc setup --format json → exit 0, findings [], install_commit null; the file reads rc.24; status empty
```

**Cell 2.**

```text
2.1  bare rig; mkdir hooks; a tracked hooks/.keep; git config core.hooksPath hooks; chmod 555 hooks
     jigc setup → exit 1
       "blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: Permission denied (os error 13)
          route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`"
     status before: empty.   status after: "A  " on .claude/settings.json, .claude/skills/jigc/SKILL.md, .jigc/.gitignore,
       .jigc/AGENT.md, .jigc/config/.gitkeep, .jigc/config/packs.yaml, .jigc/version, CLAUDE.md
     chmod 755 hooks;  jigc setup --format json → exit 0, install_commit 1828deb, hook_committed true; 9 files incl. hooks/pre-commit
2.2  same failure; chmod 755 hooks; a marker line appended to CLAUDE.md and to .jigc/AGENT.md (AM on both)
     jigc setup --format json → exit 1, setup.dirty-install-path, "2 path(s) …  `.jigc/AGENT.md`  `CLAUDE.md`
       nothing was installed and no install commit was made"; both markers still present; HEAD unmoved
2.3  bare rig; git config --unset user.name; --unset user.email; git config user.useConfigOnly true
     jigc setup → exit 1
       "blocking · setup.install-commit — the jigc install files were written and staged, but `git commit` was rejected (no install commit was made):
        Author identity unknown … fatal: no email was given and auto-detection is disabled
          route: tell git who you are — set `git config user.email "you@example.com"` and `git config user.name "Your Name"` — then re-run `jigc setup` to commit the staged install files"
     status after: the same 8 "A " paths.   identity restored;  jigc setup --format json → exit 0, install_commit f80df95
2.4  2.3, a marker appended to .jigc/AGENT.md → exit 1, sdip names it; marker kept
2.5  bare rig; git config commit.gpgsign true; git config gpg.program false
     jigc setup → exit 1, setup.install-commit, "error: gpg failed to sign the data: … fatal: failed to write commit object
       route: resolve the refusal `git commit` reports above — it names the path or setting git declined — then re-run `jigc setup`"
     both keys unset;  jigc setup --format json → exit 0, install_commit 5b3bd6f
2.6  2.5 + marker → exit 1, sdip
2.7  unborn repo (git init), git config user.useConfigOnly true, no identity
     jigc setup → exit 1, setup.install-commit; status: 9 "A " paths (root .gitignore among them)
     identity set;  jigc setup --format json → exit 0, install_commit 40c4715
2.8  2.7 + marker (AM) → exit 1, sdip names .jigc/AGENT.md
2.9  see (R1, F1) repro B
2.10 bare rig; a committed .claude/settings.json with the user's allow; chmod 555 .claude
     jigc setup → exit 1, "setup.write-guide — cannot write the adapter guide artifact `.claude/skills/jigc/SKILL.md`: Permission denied (os error 13)"
     status: "M  .claude/settings.json" + 6 "A " paths.  chmod 755;  jigc setup --format json → exit 0, install_commit c1fb58d; user entry kept
2.11 same with chmod 444 on the file → exit 1, "setup.inject-allowlist — cannot merge the allowlist into `.claude/settings.json`: Permission denied (os error 13)"
     status: 6 "A " paths.  chmod 644;  re-run → exit 0, install_commit 0565d2b
2.12 bare rig; git switch -q --detach.  jigc setup → exit 1
       "blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no branch …
          route: re-attach HEAD with `git switch <branch>`, then re-run this command"
     status: empty.  git switch main;  jigc setup --format json → exit 0, install_commit b6b4a88
2.13–2.16  each: bare rig, the 2.3 failure, identity restored, then
     2.13 marker appended + git add                         → exit 1, sdip; mark in index blob 1, worktree 1
     2.14 marker appended + git add, worktree restored      → exit 1, sdip; mark in index blob 1, worktree 0 (unchanged)
     2.15 first 13 bytes of .jigc/AGENT.md overwritten      → exit 1, sdip; mark in worktree 1
     2.16 rm CLAUDE.md (AD)                                 → exit 1, sdip names CLAUDE.md, "the install files are written and staged"
2.17 bare rig; a committed CLAUDE.md and hooks/; core.hooksPath hooks; chmod 555 hooks; a marker appended to CLAUDE.md
     jigc setup --force → exit 1, setup.install-hook.   status:  M CLAUDE.md + 7 "??" install paths (nothing staged)
     chmod 755 hooks;  jigc setup --format json → exit 1, sdip, "6 path(s)": .claude/settings.json, .jigc/.gitignore,
       .jigc/AGENT.md, .jigc/config/.gitkeep, .jigc/config/packs.yaml, CLAUDE.md   (.jigc/version and the guide exempt)
2.18 bare rig, the 2.3 failure, identity restored; echo > staged.txt; git add staged.txt; echo >> README.md
     jigc setup → exit 0; git show --stat HEAD names neither file; status: " M README.md", "A  staged.txt"
```

**Cell 3.** `bare` rig per body; `mkdir -p .claude; printf '%s\n' "$body" > .claude/settings.json; git add .claude;
git commit -qm "chore: settings"; jigc setup --format json`. Observed per row in the table; after each refusal
`HEAD` is unchanged, `.jigc`, `CLAUDE.md` and `.git/hooks/pre-commit` do not exist and `git status --porcelain`
is empty. Then the file is replaced by `{"permissions":{"allow":["Bash(mine:*)"]}}`, committed, and
`jigc setup --format json` exits 0 with an install commit; `command grep -c 'Bash(mine:\*)'` and
`'Bash(jigc:\*)'` each answer 1.

**Cell 4a.** `fresh` rig per member; `printf '\n# USERMARK-7f3a user prose\n' >> <member>` (for the settings
file: one more `allow` entry carrying the mark, written through a JSON round-trip). Before-control:
`command grep -c USERMARK-7f3a <member>` → 1. `jigc setup --format json`, then `jigc setup --force --format json`.

```text
plain (members 4.1 … 4.13): exit 1, stderr envelope {"findings":[…], "schema_version":…}
  setup.dirty-install-path, blocking: "1 path(s) in the install footprint carried changes that were in no commit before this run, so
  committing the install would sweep work `jigc setup` did not write into `chore(jigc): install jigc workspace config`:  `<member>`
  nothing was installed and no install commit was made — `HEAD` is untouched, so every path listed above still has its pre-run bytes there"
  route: "commit or stash the work at those path(s) — `git stash -u` where git does not track them yet — then re-run `jigc setup`;
  `jigc setup --force` is the single consent, and it lets the install run and commit those paths as it leaves them — which at a path
  jigc regenerates whole is jigc's own content, not yours"
  HEAD unchanged; shasum of the member unchanged; status " M <member>"
--force: exit 0, setup.forced-install-path (advisory): "`--force` consented over 1 install path(s) that carried changes in no commit
  before this run — the install has run over them, so what it merges into rides the install commit as it stands and what it regenerates
  whole is now jigc's own content, not what you wrote:  `<member>`"
  route: "check each path — `git -C $REPO show HEAD -- <path>` where the install commit carries it; where jigc regenerated the path,
  git never held a copy of what was there, so recovery is your own backup or nothing"
  CLAUDE.md, .claude/settings.json, .jigc/.gitignore: install commit made, mark in HEAD 1
  .jigc/AGENT.md, .jigc/version, .jigc/config/.gitkeep, .jigc/config/packs.yaml: no commit (the file is back to HEAD's bytes),
    mark in worktree 0, in HEAD 0, `git log --all -S USERMARK-7f3a` 0 commits
guide (4.15/4.16): exit 0 both ways, adapter-guide.user-modified (advisory): "`.claude/skills/jigc/SKILL.md` no longer carries the bytes
  jigc wrote, so jigc left it untouched rather than clobber your edits — it is no longer version-matched to jigc 1.0.0-rc.24";
  the file stays " M", byte-identical
```

**Cell 4b.** `bare` rig per member; the member created as an untracked file carrying the mark;
`jigc setup --format json`. 4.17–4.22: exit 1, `sdip` naming the member, HEAD unchanged, shasum unchanged,
status `?? <member>`. 4.23: exit 0, install commit made, `adapter-guide.user-modified`, the file still `??`
and byte-identical. 4.24: exit 0, install commit made, `.gitignore` still `??` and byte-identical.

**Cell 4c.** The unborn construction of 1.4, one file planted (and `git add`-ed for the staged arms), then
`jigc setup --format json`. Results in the table; 4.26/4.27 are F1.

**Cell 4d.** 4.31–4.34 are in the F2 block. 4.35: the F2 setup with the marker line appended **and staged**
(`M  .githooks/pre-commit`); `jigc setup --format json` → exit 1, `sdip` names the hook; status `MM` on the
hook and `A ` on the eight others. 4.36: `bare` rig; `echo > staged.txt; git add staged.txt;
echo >> README.md; echo > untracked.txt`; `jigc setup` → exit 0; `git show --stat HEAD` names none of the
three; status unchanged (` M README.md`, `A  staged.txt`, `?? untracked.txt`).

**Cell 5.**

```text
5.2–5.6  bare rig; .git/hooks/pre-commit = '#!/bin/sh\n# my team hook\necho "FOREIGN-HOOK ran" >&2\n
           if git diff --cached --name-only | grep -q "^forbidden.txt$"; then echo "FOREIGN: forbidden file" >&2; exit 1; fi\nexit 0\n'
         (mode 755; a copy kept at <tmp>/foreign.orig)
         jigc setup --format json → exit 0, install_commit 3a042c6
         hook: "#!/bin/sh" / "# jigc-managed pre-commit hook (doc<->code backstop) …" … "# jigc-managed pre-commit hook — end" /
               blank / "# my team hook" … "exit 0"
         git commit -m "feat: a"          → exit 0, "FOREIGN-HOOK ran"
         git commit -m "feat: forbidden"  → exit 1, "FOREIGN-HOOK ran" / "FOREIGN: forbidden file"
         jigc setup → exit 0; shasum of the hook unchanged; 1 start sentinel
         jigc uninstall --format json → exit 0, "removed": {allowlist, deny, guide, hook, jigc_dir, precommit, reference: all true}
         cmp .git/hooks/pre-commit <tmp>/foreign.orig → identical; mode rwxr-xr-x
5.7      fresh rig; jigc uninstall → exit 0; .git/hooks/pre-commit absent
5.8      fresh rig; hook replaced by '#!/bin/sh\necho mine\n'; jigc uninstall --format json → exit 0, "precommit": false; shasum unchanged
5.9      bare rig; OUT=$(mktemp -d "$RIG/outside-hooks.XXXXXX"); git config core.hooksPath "$OUT"
         jigc setup → exit 0, "- pre-commit hook → $RIG/outside-hooks.…/pre-commit" + the "local to this checkout" clause
         git show --stat HEAD names no pre-commit; a plain commit exits 0; jigc uninstall → exit 0; "$OUT" is empty
5.10     bare rig; git config core.hooksPath .githooks; jigc setup --format json → exit 0, hook_file ".githooks/pre-commit",
         hook_committed true; git ls-files -s → mode 100755; re-run → exit 0, HEAD unchanged, status empty
5.11     bare rig; a committed .githooks/pre-commit ('#!/bin/sh\necho "FOREIGN ran" >&2\n'); core.hooksPath .githooks
         jigc setup --format json → exit 0, install_commit d2ab015, hook_committed true; the commit adds 64 lines to the hook
5.12     the 5.10 repo; jigc uninstall → exit 0; status " D .githooks/pre-commit"
5.13     the 5.11 repo; jigc uninstall → exit 0; cmp against `git show <pre-setup>:.githooks/pre-commit` → identical
5.14     SP=$(mktemp -d "$RIG/my proj it's.XXXXXX"); git init; identity; one commit
         jigc setup → exit 0, "- pre-commit hook → .git/hooks/pre-commit"; git commit -m "feat: a" → exit 0, no "jigc:" line; sh -n → 0
5.15     BASE=$(mktemp -d "<tmp>/sp ace.XXXXXX"); SCRATCH="$BASE" rig committed-singletons   ($REPO contains a space)
         git mv docs/roadmap.md docs/plan.md; git commit -m "docs: move" → exit 1
           "jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked)."
         git reset -q --hard; an unrelated commit → exit 0, no "jigc:" line
5.16     bare rig; BIN=$(mktemp -d "$RIG/bin.XXXXXX"); ln -s ~/.local/bin/jigc "$BIN/jigc"
         "$BIN/jigc" --version → jigc 1.0.0-rc.24;  "$BIN/jigc" setup → exit 0;  hook line 9: jigc='$RIG/bin.…/jigc'
5.17     bare rig; PATH="$BIN:$PATH" jigc setup → exit 0; hook line 9: jigc='$RIG/bin.…/jigc'
5.18     vendored rig; "$BIN/jigc" setup → exit 0; sed 's/function pad(/function padLeft(/' src/pad.ts; git add; git commit → exit 0,
         the drift warning line printed twice
5.19–5.21 vendored rig
         echo x > note.txt; git add; git commit -m "chore: note" → exit 0, no "jigc:" line
         src/pad.ts: `function pad(` → `function padLeft(`; git add; git commit -m "refactor: rename pad" → exit 0
           "jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked)."
         jigc validate --format json → exit 0, blocking_probes ["doc-code"], doc-code.symbol-exists blocking
         echo y >> note.txt; git add; git commit -m "chore: note 2" → exit 0, the same warning
5.22–5.23 vendored rig per pair; git mv <old> <new>; git commit -m "docs: move" → exit 1, the "(commit blocked)" line;
         `git log -1 --format=%s` still the rig's last subject
5.24–5.28 committed-singletons rig per pair; git mv <old> <new>; git commit -m "docs: move" → exit 1, the same line, HEAD unchanged;
         jigc validate --format json → exit 1, reconciliation.rename (blocking), route
         "adopt it as a CLI-owned rename …: `jigc rename <type>:<type> --to "<New Title>"`; or revert the move: `git -C $REPO mv …`"
5.29     committed-singletons; git mv docs/roadmap.md docs/plan.md; git commit -q --no-verify -m …   (exit 0)
         echo x > note.txt; git add; git commit -m "chore: unrelated" → exit 0
           "jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for details (not staged in this commit; commit not blocked)."
5.30     committed-singletons; mv VISION.md VISION-old.md (not staged); an unrelated commit → exit 0, the same warn line
5.31     committed-singletons; git mv VISION.md VISION-old.md; printf '\nextra line one\n' >> VISION-old.md; git add -A
         git diff --cached --name-status --find-renames → R093 VISION.md VISION-old.md
         jigc validate --format json (what the hook reads) → exit 1; reconciliation.rename (blocking) "tracked managed doc vision:vision
           (VISION.md) is missing", route "restore VISION.md, or confirm the deletion …: `jigc unmanage VISION.md`" — no `git -C … mv` span
         git commit -m "docs: move+edit" → exit 0, no "jigc:" line
         after: jigc doc list no longer lists vision; jigc validate → exit 1, closing line "out-of-band rename detected — a
           structural-identity change this commit introduced; the sweep exits non-zero"; the next unrelated commit → exit 0, silent
5.32     bare rig; setup through a symlink; the symlink renamed away; git commit → exit 0, no output from the hook
5.37     fresh rig; the last line of .jigc/.gitignore ("displaced/") deleted and committed
         jigc setup → exit 0; stderr: ".jigc/.gitignore → appended displaced/   (jigc's transient-runtime entries; every line already in the file was kept)"
         git show --stat HEAD → .jigc/.gitignore | 1 +
5.38     the same under --format json → exit 0, install_commit 68a822a, findings []; the same line on stderr
```

**`uninstall`.**

```text
U.1  bare rig; the user's CLAUDE.md, .gitignore, .claude/settings.json committed (PRE); jigc setup → 0; jigc uninstall --format json → 0
     stderr: "warning: removing `.jigc/` also removes 5 tracked file(s) under it: … note: each is in the index, so
              `git -C $REPO checkout -- <path>` brings it back."
     cmp <(git show PRE:CLAUDE.md) CLAUDE.md → identical;  json(PRE:.claude/settings.json) == json(worktree) → True
     status:  M .claude/settings.json,  M CLAUDE.md (vs the install commit),  D on the guide and the five .jigc files
U.2  unborn repo; jigc setup → 0; jigc uninstall --format json → 0; ls -A → .claude .git .gitignore
U.3  the 2.1 failure (hooks dir made writable again, no re-run); jigc uninstall → exit 0, the ordinary removal summary;
     status: AM .claude/settings.json, AD on the other seven
```

**Cells 6–8.** The argv and observed output are in the tables. Two notes on construction: the install root
and both cargo target directories were `mktemp -d` directories under the scratch root; the published crates
were fetched to a `mktemp -d` directory and unpacked there. `cargo install` needed the network and a C
compiler and had both.

## Observations — recorded, not graded as defects

- **O1 · what a failed first run says was written.** `setup.install-commit` says it: *"the jigc install files
  were written and staged"*. `setup.install-hook`, `setup.write-guide` and `setup.inject-allowlist` say
  nothing about the six to eight paths they leave staged (2.1, 2.10, 2.11); the route is still followable (a
  plain re-run completes). S22 declared *no contract change*, so this is a datum, not a defect.
- **O2 · the guide advisory's wording on a file jigc never wrote.** On a never-installed repo with the user's
  own untracked file at the guide path (4.23) the advisory reads *"no longer carries the bytes jigc wrote"* and
  *"no longer version-matched"*. The behaviour (left alone, dropped from the commit) is the contract's.
- **O3 · the backstop refusal's sentence at the hook.** 4.31 says *"every path listed above still has its
  pre-run bytes there"*; at that point the hook file in the worktree has been wrapped (the jigc block inserted
  above the foreign body). The user's bytes are all still in it, verbatim.
- **O4 · a move plus an edit passes the hook silently (5.31).** By `design/reconciliation.md` this is the
  *weak* signal (no content-hash match), and the hook's block and warn are both keyed on the strong one, so
  the letter of the contract holds, and QUICKSTART says *"a bare `git mv`"*. The datum for **row 3**: the sweep
  the hook runs exits 1 with `reconciliation.rename` on that same staged set, and after the commit `jigc
  validate` closes with *"out-of-band rename detected — a structural-identity change this commit introduced"*
  — a line the hook's own decision did not treat as a rename. A lead, not a finding of this row.
- **O5 · a dated count.** `implementation/release.md` → *What the package carries* says the `jigc` listing is
  184 files, pinned at M54 Increment 6. On rc.24 it is 195 (M55's doctypes and workflows). The equality with
  the allowlist, which is the contract, holds from both sides.
- **O6 · the secrets floor over a pre-existing fresh-repo `.gitignore` (1.6).** `design/project-setup.md` says
  setup *"appends the missing floor lines"*; the binary appended the whole 13-line block, so a `.env` the
  user already listed appears twice. Nothing is clobbered and the re-run is byte-stable. M36 behaviour, outside
  what M54, M55 and the trailer changed.
- **O7 · the settings pre-check's code.** Every pre-check refusal, including the `hooks`-shape ones (3.5,
  3.6), carries `setup.inject-allowlist`; `setup.inject-hook` exists as a separate literal.
- **O8 · where the tag points.** `jigc-v1.0.0-rc.24` peels to `91834b5e`, the release-prepare commit, not the
  merge `aa6666cb`; the published crate's `.cargo_vcs_info.json` names the same `91834b5e`.
  `jigc-engine-v0.1.0-rc.2` peels to `bd6e1a44`, the engine crate's own vcs sha.
- **O9 · the unmodified install line never ran on the host**, by the scope's rule. Its faithful drives are 8.4
  and 8.7 (in the container, unmodified) and 6.1 (on the host, with `--root`).

## M54's *Not run* list — whether this run drove each entry

| M54 *Not run* entry | this run |
|---|---|
| The genuine concurrent assistant Task-tool spawn | **NOT DRIVEN** — not this row's subject |
| The Linux replaced-binary-while-running case (`/proc/self/exe`) | **NOT DRIVEN** — no drive on this host; the container runs show Linux self-exec in the normal case only (8.3, 8.4, 8.6, 8.7: `drift-finalize` exit 3 with `JIGC_DOC_CODE_PROBE` unset) |
| The ≤ 15 min green-push claim and the before/after CI runtime | **NOT DRIVEN** — a GitHub-runner fact; only read: the CI and Release runs on `aa6666cb` concluded `success` (C.4) |
| The release pipeline, release-plz, the rehearsal, the App and the yank | **NOT DRIVEN (agents may not)** — what was read: both `0.0.0` yanked, three `jigc` and two `jigc-engine` prereleases unyanked (8.2), `missing=false` (8.1), the GitHub releases marked prerelease (C.4) |
| The 4-CPU-limited `dev/runner-faithful` variant | **DRIVEN** — 8.6, `--cpus 4 tarball`, exit 0, every step `ok` |

M54's e2e scenarios 8–13, for context: 8 → rows 2.1, 2.2, 2.5; 9 → cell 3; 10 → 1.8; 11 → 7.1, 7.5;
12 → 8.3; 13 → 6.1–6.4, 8.1, 8.2, 8.4, 8.5.

## Not driven — every pair this row could have held and did not drive

1. **Merge or push `main`, merge a pull request, push a tag, publish, approve or reject a deployment, re-run
   the release workflow** — NOT DRIVEN (agents may not). `completions/artifacts/M54/publish-proof.md` and
   `completions/artifacts/M55/crates-page-reread.md` stand for them; neither covers rc.24's own publish, whose
   outcome was only read (8.1, 8.2, 7.4, C.4).
2. **The bare install line on the host** — NOT DRIVEN (it would replace the binary the other drivers use).
3. **The rc.24 crates.io page as rendered** — NOT DRIVEN here; the packaged README's bytes were compared
   (7.3), not the page.
4. **`dev/runner-faithful --platform linux/amd64 tarball`** — NOT DRIVEN; amd64 was driven for the registry
   mode only (8.7).
5. **The Linux replaced-binary arm and the Task-tool spawn** — see the table above.
6. **The hook from a linked worktree** (the common hooks dir) — NOT DRIVEN; M47/M53 behaviour outside the
   range.
7. **A hooks dir owned by another repository (submodule, checked out or not; embedded repo) and a
   sparse-checkout that excludes it** — NOT DRIVEN; so the *soft member* retry and the `Stage` arm of
   `setup.install-commit` (a refused `git add`) have no row here.
8. **The posture cells other than detached HEAD at a first run** (merge, rebase, bisect in progress) —
   NOT DRIVEN; numbered axis 2's, excluded by the scope.
9. **`uninstall`'s refusing arms** (`uninstall.dirty-worktree`, `.staged-prose`, `.untracked-workbench-file`,
   `.foreign-bytes`, and `--force`) — NOT DRIVEN; numbered axis 3's, excluded by the scope. No `uninstall.*`
   code was reached.
10. **Fourteen `setup.*` codes** — `compose-marker`, `guide-target`, `init-project-layer`, `inject-deny`,
    `inject-hook`, `inject-reference`, `pack-load`, `profile-incomplete`, `profile-load`, `repo-root`,
    `secrets-gitignore`, `spawn-template`, `version-stamp`, `write-bootstrap` — NOT DRIVEN; no cell of the
    scope reaches them and each needs a fault this run did not plant.
11. **Cell 4 × the staged and index-only classes for every member** — driven for `CLAUDE.md`,
    `.jigc/AGENT.md` and the hook only (4.28, 4.29, 4.35, 2.13, 2.14); the other members were driven on the
    unstaged and untracked classes.
12. **Cell 4b × `--force`** (the untracked class under consent) — NOT DRIVEN; `--force` was driven on the
    unstaged class for all eight members (4a) and on the hook (4d).
13. **The doc↔code warning from inside `jigc task finalize`'s own commit** (the success relay) — NOT DRIVEN;
    the hook was driven through plain `git commit` only, as the scope's cell says.
14. **A hook template that really differs between two published versions** — cannot be driven: the template
    is identical at all three release tags, so 5.34 uses a one-line edit as the stand-in and says so.
15. **`jigc setup` from a build other than rc.24 writing the first hook** — NOT DRIVEN (no other build may be
    used); 5.33 uses the same build at a second path.

## Baseline rows: CLOSED / STILL-OPEN

**None.** This row has no baseline (first drive), and the scope names no row to re-drive. The one declared
open item met on the way is recorded at U.3 — `jigc uninstall` after a failed first setup, owed at M57 —
and it is **STILL-OPEN (expected)** as declared, not a baseline row.

## Counts

- Driven rows: **132** — cell 1: 9 · cell 2: 18 · cell 3: 12 · cell 4: 36 · cell 5: 38 · `uninstall`: 3 ·
  cell 6: 4 · cell 7: 5 · cell 8: 7. Of these, 116 have `setup` or `uninstall` as their door or drive the hook
  those doors installed; 16 are the non-leaf doors of cells 6–8.
- Classification and fence rows (no coverage): **4** (C.1–C.4).
- Not driven: **15** enumerated entries.
- Defects: **3** — (R1, F1) proposed tier 1 · (R1, F2) tier 3 · (R1, F3) tier 3.
- Leaves covered: `setup`, `uninstall`.

---

# Reconciliation ledger — row 1 (rc.24)

Reconciler: Opus, a third party — author of neither the driver record above nor the code. Binary
`~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` asserted before the first drive. Repository read at
`bffa6667` (`work/rc24-gate`); `git status --porcelain` empty before and after, HEAD unmoved, `git stash list`
empty afterwards. Every rig: `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig";
[ -n "$REPO" ] || exit`, stdout only, plus a refusal when `$REPO` or its git toplevel is under the working
repository. A fresh rig per cell, no teardown, every scratch root from `mktemp -d`. Unborn cells: a plain
`git init` in `mktemp -d "$RIG/unborn.XXXXXX"` under a `bare` rig (so `HOME` is the rig's), repo-local
identity `trial@example.com`. Nothing was written into a `.jigc/` workbench by hand except the adopter bytes
that *are* the cell (F1). Environment, names only: `CLAUDECODE` set in the session (held constant except in the
S4 cell, which sets / unsets / empties it); `JIGC_DOC_CODE_PROBE` unset except in the D3 cell; `GITHUB_PATH`
unset. Docker answered (`29.5.2`).

**The rule applied** (`completions/artifacts/M51/acceptance-design.md` → *The reconciliation rule*): a claim by
one that the other cannot reproduce is a lead, not a finding. Every Codex claim was entered as a lead and
driven; every driver defect was re-driven once.

## Source-pass status

- `codex/axis1-codex.md` exists (6.4 KB), `axis1.rc` = 0. **This is the pass reconciled against.**
- A second, higher-effort pass (`axis1-high-stdout.log`, `axis1-high.rc` = 1) ended on the provider's usage
  limit and wrote **no** output file. It contributes no claim. Nothing was inferred from its log.
- The pass reports **zero defect claims** (*"No grounded completeness defect found"*). Its body is still a
  set of positive claims about what the binary does — five M54-finding dispositions, five *Not run*
  dispositions, nine census bullets and one schema bound — and each is a lead here. Silence was not treated
  as refutation and a "nothing found" was not treated as a completeness result.

## Checks on the driver record

**Door-set count against the registries — re-read by the reconciler; every count holds.**

| registry | driver | reconciler's read |
|---|---|---|
| `install_tracked_paths` | 7 + 3 conditional | 7 unconditional pushes + `seeded_gitignore`, `guide`, `hook` — and the ten-member commit was **driven** (S1 below: 10 files) |
| `BEHALF_DOORS` | 48 | 48 indented `BehalfDoor {` literals (the 49th `BehalfDoor {` in the file is the struct definition); `setup` = `CommitsOnBehalf` with one `PostureExemption` (`HeadUnborn`); `uninstall` = `Neither` |
| `IGNORE_DOORS` | 4 | 4 `IgnoreDoor {` |
| `setup.*` / `uninstall.*` literals | 20 / 13 | 20 / 13 distinct quoted literals under `crates/cli/src` + `crates/engine/src` |
| `PrecommitTeardown` | 3 | `NotOurs`, `RemoveFile`, `RestoreForeign(String)` |
| `include` + `readme` | 6 + `README.md` | 6 patterns, `readme = "README.md"` |
| `PUBLISHED_DIRS` | 4 | `["src", "packs", "adapters", "guides"]` |
| jobs | 10 / 3 | `ci.yml` 10, `release.yml` 3, the same names |
| `VERB_KINDS` | `setup`, `uninstall` leaves | `(&["setup"], Write)`, `(&["uninstall"], Write)` |

**Row count.** 9 + 18 + 12 + 36 + 38 + 3 + 4 + 5 + 7 = 132, as stated.

**Rows marked driven with no repro block — none demoted, with one ruling stated.**

- Cells 1–5 and `uninstall`: every row is covered by a numbered block, a per-cell construction block (3, 4a,
  4b, 4c, 4d) or one of the three finding blocks. 5.1 has no block of its own; its facts sit in the 1.1, 1.2
  and F3 blocks (summary line, `hook_committed: false`, 65 lines / 1 sentinel / line 9) and the reconciler
  re-drove them (`fresh` rig: 65 lines, 1 start sentinel, 0 end markers, `sh -n` exit 0).
- **Cells 6–8 carry no separate block** — the record says *"The argv and observed output are in the tables."*
  Ruling: for a fixture-less door (the fixture is the repository at `bffa6667` or the registry) the table row
  *is* argv + exit + observed output, so the row stands as driven **where the argv is a runnable command**.
  7.3 and 7.4 give a description (`curl` … unpack … `diff`), not a command line; rather than demote them the
  reconciler re-drove both and supplies the block below. Re-driven by the reconciler: 6.1, 6.2, 6.3, 6.4, 7.1,
  7.2, 7.3, 7.4, 7.5, 8.1, 8.2, 8.4, 8.5, 8.6 — **every one reproduced**. Not re-driven (standing on the
  driver's row alone): 8.3 (the 8-CPU tarball mode; its 4-CPU twin 8.6 was re-driven) and 8.7 (`linux/amd64`
  under emulation).

```text
6.4  R2=$(mktemp -d …); cargo install jigc --locked --root "$R2"          → exit 101
       error: could not find `jigc` in registry `crates-io` with version `*`        ; "$R2" has 0 entries
6.1  R=$(mktemp -d …);  cargo install jigc --version '^1.0.0-rc.1' --locked --root "$R"   (--root is the one added token) → exit 0
       "Installing jigc v1.0.0-rc.24" · 67 "Compiling" lines · "Installed package `jigc v1.0.0-rc.24` (executable `jigc`)"
6.2  ls "$R/bin" → jigc            cargo install --list --root "$R" → "jigc v1.0.0-rc.24:" / "    jigc"
6.3  "$R/bin/jigc" --version → jigc 1.0.0-rc.24
7.1  cargo package --list -p jigc (private CARGO_TARGET_DIR) → exit 0, 195 paths: packs/ 148, src/ 37, guides/ 2, adapters/ 1,
       README.md, LICENSE-MIT, LICENSE-APACHE, Cargo.toml, Cargo.toml.orig, Cargo.lock, .cargo_vcs_info.json
       outside the allowlist: 0 · under tests/: 0
       git -C crates/cli ls-files src packs adapters guides → 188; tracked-not-listed 0; listed-not-tracked 0
7.2  cargo package --list -p jigc-engine → exit 0, 107 paths: src/ 93, proptest-regressions/ 7, CHANGELOG.md, the license pair,
       Cargo.toml, Cargo.toml.orig, Cargo.lock, .cargo_vcs_info.json; no README
7.3  dev/crate-readme --stdout > <tmp>/readme.gen → exit 0;  cmp <tmp>/readme.gen crates/cli/README.md → exit 0
7.4  curl -fsSL https://static.crates.io/crates/jigc/jigc-1.0.0-rc.24.crate          → exit 0
     curl -fsSL https://static.crates.io/crates/jigc-engine/jigc-engine-0.1.0-rc.2.crate → exit 0
     shasum -a 256 of each == the index `cksum` of that version (first 12 hex: fb67d12319f6 / 945b7052fc47)
     tar xzf both; find . -type f | sort  vs  the 7.1 / 7.2 listings → 195 / 107 files, diff 0 lines each
     diff -rq <published>/{src,packs,guides,adapters} crates/cli/{…} → exit 0 ×4, no output
     cmp <published>/README.md crates/cli/README.md → 0 ; LICENSE-MIT, LICENSE-APACHE vs the root pair → 0, 0
     [dependencies.engine] version = "=0.1.0-rc.2", package = "jigc-engine"
     .cargo_vcs_info.json → 91834b5e… (crates/cli) · bd6e1a44… (crates/engine); README links not `](https://` : 0
7.5  cargo publish --workspace --dry-run (repository root, private CARGO_TARGET_DIR) → exit 0
       both "already exists on crates.io index" warnings · "Packaged 107 files" · "Packaged 195 files" · 12 "ignoring test" warnings ·
       two "Verifying" builds finish · two "Uploading …" each followed by "aborting upload due to dry run"
8.1  dev/unpublished-versions → exit 0: "jigc@1.0.0-rc.24 published" / "jigc-engine@0.1.0-rc.2 published" / "missing=false"
8.2  curl -fsS https://index.crates.io/ji/gc/jigc → 0.0.0 yanked; 1.0.0-rc.22, -rc.23, -rc.24 unyanked
     curl -fsS https://index.crates.io/ji/gc/jigc-engine → 0.0.0 yanked; 0.1.0-rc.1, -rc.2 unyanked
8.5  dev/runner-faithful registry → exit 1: "FAIL install: cargo install jigc --locked (unpinned) exited 101" (linux/aarch64, 8 CPUs)
8.4  dev/runner-faithful registry '^1.0.0-rc.1' → exit 0: "ok install: … installed jigc 1.0.0-rc.24", then version, setup,
       probe-override, control-finalize, drift-finalize (exit 3, doc-code.symbol-exists, HEAD unchanged) all "ok"
8.6  dev/runner-faithful --cpus 4 tarball → exit 0: "cpus 4", git 2.43.0, rustc 1.95.0; package, lock-compare, install,
       version (jigc 1.0.0-rc.24), setup, probe-override, control-finalize, drift-finalize all "ok"
```

## Driver defects — re-driven

| key | status | tier | door | why that tier |
|---|---|---|---|---|
| **(R1, F1)** | **CONFIRMED** — repro A, the born-HEAD control and repro B all reproduced | **1** | `setup` | both halves shown, through a committing door — see below |
| (R1, F2) | **CONFIRMED** | 3 | `setup` | the advisory names fewer paths than the consent was spent on; exit 0, the user's line is in the install commit, nothing lost |
| (R1, F3) | **CONFIRMED** — path arm, template arm, committed-hook arm, the `uninstall` arm and the doubled warning all reproduced | 3 | `setup`, `uninstall`, the hook | three surfaces say something the binary does not do; a second `uninstall` removes the leftover, so no dead end, and no byte is lost |

### (R1, F1) — re-drive, and the tier argued both ways

```text
A       bare rig (HOME); F=$(mktemp -d "$RIG/unborn.XXXXXX"); git init -q; local identity; mkdir -p .jigc/config
        printf '# Team notes for agents — USERMARK-A written by a human\nAlways run the linter.\n' > .jigc/AGENT.md
        printf '# USERMARK-P why we pin dev only: see the team wiki\npacks:\n- dev\n' > .jigc/config/packs.yaml
before  git rev-list --count --all → 0 ;  git status --porcelain -uall → ?? .jigc/AGENT.md / ?? .jigc/config/packs.yaml
        command grep -rc USERMARK .jigc → .jigc/AGENT.md:1  .jigc/config/packs.yaml:1                       (before-control)
argv    jigc setup
exit    0   "jigc setup — adapter installed … install commit → 5e11e8e"; stderr empty; no finding
after   git status --porcelain -uall → (empty) ;  git log → 5e11e8e chore(jigc): install jigc workspace config
        command grep -rc USERMARK .jigc CLAUDE.md → every file :0
        git log --all -S USERMARK → 0 commits ;  git grep -c USERMARK HEAD → exit 1 (no file)
        git fsck --unreachable → 0 objects ;  git stash list → 0
        every blob of `git cat-file --batch-all-objects` read for the mark → none carries it
        .jigc/AGENT.md → jigc's generated bootstrap ;  .jigc/config/packs.yaml → "packs:\n- dev\ncompose-embedded-methodology: true"

control the same two files, the same bytes, on a `bare` rig (1 commit)
        jigc setup → exit 1, setup.dirty-install-path names both paths, "nothing was installed";
        command grep -rc USERMARK .jigc → 1 and 1 ; status still ?? on both ; 1 commit

B       unborn repo, git config user.useConfigOnly true, no identity
        jigc setup → exit 1, setup.install-commit ("written and staged, but `git commit` was rejected"); 9 paths "A "
        identity set, useConfigOnly unset
arm 1   (index kept)  printf '\n# USERMARK-7f3a edit after the failure\n' >> .jigc/AGENT.md   (AM)
        jigc setup --format json → exit 1, setup.dirty-install-path ; mark 1 ; 0 commits                     ← re-arms
arm 2   git rm -r -q --cached .   (9 paths ??) ; the same append ; before-control: mark = 1
        jigc setup --format json → exit 0, {"findings": [], "install_commit": "215fbec", …}
        mark in .jigc/AGENT.md 0 · git log --all -S USERMARK-7f3a → 0 · fsck --unreachable 0 · blob scan: none  ← does not re-arm

control for arm 2 on a BORN HEAD  (bare rig; the same failure; identity restored; git rm -r -q --cached .claude .jigc CLAUDE.md → 8 paths ??)
        no edit:   jigc setup --format json → exit 0, install commit made (the plain re-run completes)
        the edit:  jigc setup --format json → exit 1, setup.dirty-install-path names .jigc/AGENT.md ; mark 1   ← re-arms

boundary (reconciler, unborn):  untracked CLAUDE.md → exit 0, no code, mark in HEAD 1 (merged into — the stated rule)
        staged CLAUDE.md → exit 1, sdip, 0 commits, no .jigc ;  staged .jigc/AGENT.md → exit 1, sdip, mark 1, 0 commits
```

**Tier 1, upheld.** *Half one — exit 0:* shown, twice (A and B arm 2), with an empty finding list. *Half two —
the loss:* a before-control that finds the bytes (`:1`, `:1`), then their absence from the worktree, from
`HEAD`, from every commit (`log --all -S`), from the unreachable set and from every object in the database.
*The door:* `setup` is `CommitsOnBehalf` in `BEHALF_DOORS`. Neither half is missing.

**Argued downward, and why it does not hold.** `design/validation.md` states the unborn `??` exclusion as a
door rule, so the cell is green *by design*. But the rule's own stated rationale is that setup *merges into*
the host files so the adopter's bytes ride the first commit — true of `CLAUDE.md` (driven: mark in `HEAD`),
false of the two paths the install rewrites whole, which the same row calls *"destroyed"* on a born HEAD. The
tier scale grades what the binary did, not whether a doc sanctioned it; a sanctioned exit-0 loss is still an
exit-0 loss, and the born-HEAD control shows the product already refuses the identical bytes one commit later.
What the caveat changes is the fix's home (the door rule itself has the hole, and QUICKSTART's sentence carries
no unborn carve-out), and the precondition is narrow: a zero-commit repository holding human bytes at
`.jigc/AGENT.md` or `.jigc/config/packs.yaml`. Repro B is the less exotic route — it starts from jigc's own
failed first run. Both caveats travel with the finding; neither removes a half.

### (R1, F2) — re-drive

```text
setup   bare rig; .githooks/pre-commit ('#!/bin/sh\necho "FOREIGN ran" >&2\n', 755) committed; git config core.hooksPath .githooks
        printf 'echo "USERMARK-7f3a uncommitted line" >&2\n' >> .githooks/pre-commit                    ( M .githooks/pre-commit)
plain   jigc setup --format json → exit 1, setup.dirty-install-path names `.githooks/pre-commit`; HEAD unmoved;
        8 paths "A ", the hook " M"; mark in the worktree hook 1                                         (the backstop — as designed)
force   jigc setup --force --format json → exit 0
        {"findings": [], "hook_committed": true, "hook_file": ".githooks/pre-commit", "install_commit": "3465af9", …}; stderr empty
        git show HEAD:.githooks/pre-commit | command grep -c USERMARK-7f3a → 1 ; the commit: 9 files, .githooks/pre-commit +65
control the same, CLAUDE.md also committed and carrying an uncommitted marked line ( M on both)
        jigc setup --force --format json → exit 0, setup.forced-install-path:
          "`--force` consented over 1 install path(s) …:\n  `CLAUDE.md`"
        mark in HEAD:.githooks/pre-commit 1 · mark in HEAD:CLAUDE.md 1                                   (two consumed, one named)
```

**Tier 3.** Argued upward: an uncommitted user line is swept into `chore(jigc): install jigc workspace config`
— but under `--force`, whose declared meaning is exactly that, and the bytes are preserved in the commit. No
loss, no dead end; the defect is the advisory's silence about the hook member.

### (R1, F3) — re-drive, with one more live trigger

```text
path    fresh rig: hook 65 lines · 1 start sentinel · 0 end markers · line 9 jigc='~/.local/bin/jigc'
        BIN=$(mktemp -d "$RIG/cargo-bin.XXXXXX"); ln -s ~/.local/bin/jigc "$BIN/jigc"; a third dir the same way
        "$BIN/jigc" setup → exit 0   → 129 lines · 2 start sentinels · 1 end marker · line 9 = $RIG/cargo-bin.…/jigc · line 73 = ~/.local/bin/jigc
        third-path setup  → exit 0   → 2 sentinels · line 9 = the third path · line 73 unchanged
        ~/.local/bin/jigc setup → 0  → 2 sentinels · lines 9 and 73 both ~/.local/bin/jigc
        jigc uninstall --format json → exit 0, "precommit": true ; the hook is STILL PRESENT: 65 lines, 1 sentinel
        jigc uninstall               → exit 0, "- removed pre-commit hook" ; the file is gone
drift   fresh rig; the third comment line of the hook reworded ("# REWORDED committed store …"); jigc setup → exit 0
        → 129 lines · 2 sentinels · the reworded line still present
commit  bare rig; git config core.hooksPath .githooks; jigc setup --format json → hook_committed true
        "$BIN/jigc" setup --format json → exit 0, install_commit fdaf2cd, hook_committed true
        git show HEAD:.githooks/pre-commit → 2 sentinels; line 9 the second path, line 73 the first; the commit: 1 file, 64 insertions
double  vendored rig; setup through a symlinked binary (2 blocks); `function pad(` → `function padLeft(`; git commit → exit 0,
        the drift warning printed twice
append  (reconciler) a jigc hook with ONE user line appended ('# USERMARK-7f3a hook edit'); jigc setup --format json → exit 0,
        no finding; the hook now has 2 start sentinels and the user's line                               ← a user adding a line is enough
```

**Tier 3.** The last arm widens the trigger past "another path or another build": any byte of a standalone
jigc hook that differs from this build's template — including one line a user adds to it — makes the next
`setup` treat the whole file as foreign and wrap it. Still no loss, and a second `uninstall` clears it.

## New findings reached by driving Codex leads

Both were found by the reconciler while driving a claim of the source pass; the source pass asserted the
opposite or the neighbouring fact. Origin is recorded as `codex` (a lead driven to a repro) with that said.

| key | title | door | tier | origin |
|---|---|---|---|---|
| **(R1, F4)** | A `jigc` whose absolute path contains a single quote installs a hook whose quoting is broken: no drift warning, **no rename block**, and three `command not found` lines on every commit — while `setup` reports the backstop installed | `setup`, the hook | 3 | lead(codex, S6 — *"quoted for spaces"*) |
| (R1, F5) | From a linked worktree the hook's sweep asks about the **main checkout**: drift committed in the worktree draws no warning and a bare `git mv` of a managed doc committed there is not blocked | the hook (through `git commit`) | 3 | lead(codex, S6 — *"honors `core.hooksPath` and worktrees"*) |

### (R1, F4) — the embedded path is single-quoted and not escaped

**Contract.** `setup`'s summary: *"pre-commit hook → … (warn-only doc↔code drift backstop)"*. QUICKSTART → the
hook *blocks* a commit that itself stages an out-of-band managed-doc rename. The writer's own comment
(`precommit_hook_body`, `crates/cli/src/setup.rs`): the absolute installing path is embedded, *"quoted in the
script so a path with spaces survives"* — as `jigc='{jigc}'`, the value interpolated raw between single quotes.

```text
space   vendored rig; BS=$(mktemp -d "$RIG/my tools.XXXXXX"); ln -s ~/.local/bin/jigc "$BS/jigc"
        "$BS/jigc" setup → exit 0 ; line 9: jigc='$RIG/my tools.…/jigc' ; sh -n → 0
        drift an anchored symbol; git commit → exit 0, the drift warning prints                          (a space survives — control)

quote   BQ=$(mktemp -d "$RIG/it's bin.XXXXXX"); ln -s ~/.local/bin/jigc "$BQ/jigc"; "$BQ/jigc" --version → jigc 1.0.0-rc.24
Q0      bare rig; "$BQ/jigc" setup → exit 0, "- pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)",
        "- install commit → e28534f" ; line 9: jigc='$RIG/it's bin.…/jigc' ; sh -n → 0 (the quotes re-balance further down)
        echo x > note.txt; git add; git commit -m "chore: note" → exit 0, the commit lands, and stderr carries
          .git/hooks/pre-commit: line 9: blocking_probes: command not found
          .git/hooks/pre-commit: line 9: doc-code: command not found
          .git/hooks/pre-commit: line 9: bin.…/jigc⏎⏎# Run the sweep, capturing stdout only. … (one multi-line word made of the
            script's own following text, run as a command name)
Q1      vendored rig; the installed hook file removed; "$BQ/jigc" setup → exit 0 (1 sentinel)
        `function pad(` → `function padLeft(`; git add; git commit → exit 0 ; the three noise lines ; NO "jigc: doc<->code drift" line
        control: jigc validate --format json → "blocking_probes": ["doc-code"]                           (the drift is real)
Q2      committed-singletons rig; hook removed; "$BQ/jigc" setup → exit 0
        git mv docs/roadmap.md docs/plan.md; git commit -m "docs: move" → exit 0, HEAD MOVED ; the three noise lines ; no block
        control (same state, hook from ~/.local/bin/jigc): the same commit → exit 1,
          "jigc: out-of-band managed-doc rename staged in this commit … (commit blocked)." ; HEAD unmoved
Q3      bare rig; a foreign hook that rejects forbidden.txt (control before setup: commit exit 1, "FOREIGN: forbidden file")
        "$BQ/jigc" setup → exit 0 ; a commit staging forbidden.txt → exit 1, "FOREIGN-HOOK ran" / "FOREIGN: forbidden file", HEAD unmoved
                                                                                                         (the wrapped foreign hook still runs)
```

**Tier 3.** `setup` says a backstop was installed; the hook it wrote neither warns nor blocks and prints shell
noise on every commit. Argued upward: the rename the hook exists to block lands at exit 0 — but through `git
commit`, not a jigc committing door, nothing is lost (`jigc validate` still reports it), and the wrapped
foreign hook keeps its verdict (Q3), so neither tier-1 half and no dead end. Argued downward: the precondition
is an apostrophe in the directory `jigc` is installed in — narrow. One thing the reconciler did **not** drive:
a path carrying shell metacharacters after the quote (the unquoted remainder is executed as words — the noise
lines above are that); it is the installing user's own path, recorded as an open lead below, not graded.

### (R1, F5) — the hook from a linked worktree

**Contract.** QUICKSTART lists *"a linked worktree's shared hooks dir"* among the hook's homes and says the hook
warns on drift and blocks a commit that itself stages a bare `git mv` of a managed doc.
`design/assistant-adapter.md` → *Where it installs: jigc_home, from every cwd (M53)* binds the **store** to the
main checkout. No doc read here says what the hook therefore does on a commit made in a linked worktree.

```text
resolve vendored rig; WT=$(mktemp -d "$RIG/wt.XXXXXX"); git worktree add -q "$WT/tree" -b wt-branch → exit 0; cd "$WT/tree"
        git rev-parse --git-path hooks → $RIG/repo/.git/hooks                                            (the common dir)
        jigc setup --format json → exit 0, "hook_file": ".git/hooks/pre-commit", "hook_committed": false, "install_commit": null
WT1     in the worktree: `function pad(` → `function padLeft(` in src/pad.ts; git add; git commit → exit 0 ; 0 "jigc:" lines
        jigc validate --format json (cwd = the worktree, the drift now committed there) → exit 0, "blocking_probes": [], "findings": []
        control: the main checkout's src/pad.ts still has `function pad(`; jigc validate there → the same clean report
        inverse (a second rig): the worktree's own sweep clean as above; then the same drift made and committed in the MAIN
          checkout (the warning prints there); then the worktree's commit → exit 0 and the drift warning prints — about main's tree
WT2     committed-singletons rig; a linked worktree the same way; in it: git mv docs/roadmap.md docs/plan.md; git commit -m "docs: move"
        → exit 0, HEAD MOVED, no "jigc:" line
        control: the same `git mv` + commit in a main checkout of the same state → exit 1, "(commit blocked)"   (F4's Q2 control; driver 5.27)
```

**Tier 3**, with its range stated: this is the M53 jigc_home binding meeting the M35 hook, not something M54,
M55 or the trailer changed, and the driver listed the linked-worktree hook as *not driven* (entry 6). It is
filed because it was driven and it reproduces; the triage may route it to the numbered axis that owns the hook
and the worktree binding. No loss — the rename is reported by `jigc validate` once the branch is merged.

## Codex claims — every one a lead, every one driven or left open with the reason

Claim ids are the reconciler's: `K` the headline, `D1–D5` the M54-finding dispositions, `N1–N5` the *Not run*
dispositions, `S1–S9` the census bullets in order, `B1` the schema bound.

| id | lead(codex, …) | status | datum |
|---|---|---|---|
| K | *No missing door, undispositioned install path, unrecorded post-write error return, unsafe teardown classification, package/allowlist mismatch, or release-path bypass* | **REFUTED** in three limbs; two limbs hold | *undispositioned install path*: the hook member is consumed under `--force` and named by no advisory (F2). *unsafe teardown classification*: a jigc hook is classified foreign and `uninstall` reports a removal that left a jigc hook on disk (F3). And the headline *no defect*: F1, F4, F5. Holding: *package/allowlist mismatch* (7.1, 7.4 re-driven: equal from both sides) and *release-path bypass* (nothing driven contradicts it; the gated publish itself is an open lead, S9) |
| D1 | hygiene denylist: patterns compiled, failures propagated | **CONFIRMED** (driven) | block D1 below: clean → 0, hit → 1, malformed pattern → 2 *"the scan did not run"*, empty → 2, unreadable → 2 |
| D2 | every `uses:` a full SHA; release-plz installed through the digest-checked script | **CONFIRMED** (read + driven) | 23 `uses:` lines, 23 match `@<40 hex> # v…` (a read — classification); `dev/install-release-plz --archive <bogus> --dest <tmp>` → exit 1, *"SHA-256 mismatch …"*, the dest dir stays empty. The workflow fence test was not run; the genuine asset download was not driven |
| D3 | a probe failure names the attempted program | **CONFIRMED** (driven) | `vendored` rig, drifted; `JIGC_DOC_CODE_PROBE` set to a path under `$RIG` that names no file; `jigc validate --format json` → exit 1, *"`doc-code` probe not found at "$RIG/no-such-probe" — `JIGC_DOC_CODE_PROBE` names no file; point it at a probe executable, or unset it to run the probe built into `jigc`"* |
| D4 | failed release-PR publication recovery is documented and deliberately unpinned | **OPEN LEAD** | the behaviour is remote state (a rejected or expired deployment) — agents may not drive it. Read only: the recovery text is in `implementation/release.md` → *Known gaps* (*"A release-PR merge whose deployment does not publish is not retried by any later push"*); the pass's citation `release.md:172-187` lands in *What agents may not do*, not on that text |
| D5 | `SaveDegrades` / `save_degrades` public but doc-hidden | **CONFIRMED** (a read of the **published** artifact — classification) | `jigc-engine-0.1.0-rc.2.crate` as served by crates.io, `src/state.rs`: `#[doc(hidden)]` directly above `pub struct SaveDegrades` and above `pub fn save_degrades` |
| N1 | the genuine Task-tool spawn is still a declared bound | **OPEN LEAD** | not this row's subject and not drivable from this session |
| N2 | Linux replaced-binary-while-running still not covered; relocated and symlinked self-exec are fenced | split: **OPEN LEAD** (the Linux replaced-binary arm — no drive) · **CONFIRMED** (relocated and symlinked self-exec, driven on macOS) | `vendored` rig, drifted; `cp ~/.local/bin/jigc "$RIG/reloc.…/jigc"`; with `JIGC_DOC_CODE_PROBE` unset, `"$RIG/reloc.…/jigc" validate --format json` → exit 0, `doc-code.symbol-exists`, `"blocking_probes": ["doc-code"]`. Symlinked: F4's *space* block (the warning prints through a symlinked binary). Linux, normal case: 8.4 and 8.6 `drift-finalize` exit 3 |
| N3 | the cold CI runtime at this tree is unmeasured | **OPEN LEAD** | a GitHub-runner fact; nothing here measures it |
| N4 | the real release pipeline / App / yank is a remote-state bound | **OPEN LEAD** | agents may not; read only (8.1, 8.2) |
| N5 | the four-CPU `dev/runner-faithful` is *still a declared non-run variant* | **REFUTED** (stale) | driven by the driver (8.6) and re-driven here: `dev/runner-faithful --cpus 4 tarball` → exit 0, `cpus 4`, every step `ok` |
| S1 | the committed class is one registry: 7 + root `.gitignore` + guide + trackable hook; ten members | **CONFIRMED** (driven) | block S1: 8 files (born, default hooks dir) · 9 (unborn) · 9 (born, in-repo hooksPath) · **10** (unborn + in-repo hooksPath) |
| S2 | the dirty-path refusal occurs before writes; malformed settings are parsed before writes | split: **CONFIRMED** for the enumerated members on a born HEAD and for the settings pre-check · **REFUTED** as a universal | holds: F1's born-HEAD control (*nothing was installed*, status only the two `??`), block S2. Refuted: (a) the hook member is refused at the *backstop*, after the install is written and staged (F2 plain arm: 8 paths `A `) — design-declared; (b) on an unborn HEAD an untracked install path draws no refusal at all (F1) |
| S3 | every write failure is recorded; **post-failure user edits re-arm both worktree and index checks**; later runs retain only soft members | split: **CONFIRMED** on a born HEAD, on the index axis and for the soft member · **REFUTED** on an unborn HEAD once the staged entries are gone | holds: F1 arm 1, F1's born-HEAD control for arm 2, block S3. Refuted: F1 repro B arm 2 — a post-failure edit, exit 0, the edit in no object. The three non-commit failure sites (hook, guide, allowlist) stand on the driver's 2.1, 2.10, 2.11, not re-driven |
| S4 | the install commit carries the trailer only under the adapter's non-empty environment predicate | **CONFIRMED** (driven) | block S4: `CLAUDECODE` set → 1 trailer line; unset → 0; empty → 0; the subject identical in all three |
| S5 | `setup` commits on behalf with exactly the unborn exemption; `uninstall` does not commit | **CONFIRMED** (driven) | block S5: unborn → exit 0 (F1, S1); detached → exit 1 `repo.head-detached`, status empty; merge in progress → exit 1 `repo.operation-in-progress`, no `.jigc`; `uninstall` on a detached HEAD → exit 0, no finding |
| S6a | the hook embeds the *resolved* running executable | **CONFIRMED with a correction** | absolute, yes; **not** symlink-resolved — line 9 is the symlink path as invoked (F3, F4 *space*; the driver's 5.16 says the same) |
| S6b | …*quoted for spaces* | **CONFIRMED** for a space · the quoting **REFUTED** as sufficient | a space survives; a single quote does not → **(R1, F4)** |
| S6c | hooks-directory resolution honors `core.hooksPath` and worktrees | **CONFIRMED** for the resolution | in-repo and out-of-repo hooksPath (driver 5.9, 5.10; S1, F2 here); a linked worktree resolves the common dir (F5 *resolve*). What the hook then asks about from a worktree → **(R1, F5)** |
| S6d | rename blocking uses both staged operands | **CONFIRMED** (driven) | both staged → exit 1 blocked (F4's Q2 control); only the old path staged (`D docs/roadmap.md`, the new one `??`) → exit 0 with the *"not staged in this commit; commit not blocked"* warn line |
| S6e | teardown distinguishes standalone, wrapped and foreign bytes | **CONFIRMED** for the three clean cases · **REFUTED** at the edge | `RemoveFile` and `RestoreForeign` (F3's two `uninstall` calls; driver 5.6, 5.7), `NotOurs` (re-driven: `"precommit": false`, hash unchanged). Edge: a jigc hook from another path, another template or with one appended line is classified foreign (F3) |
| S7 | the package is the allowlist, equal in both directions | **CONFIRMED** (driven) | 7.1 and 7.4 re-driven above, local listing and the published crate |
| S8 | the install line is identical in three carriers; caret semantics | **CONFIRMED** (read + driven) | `cargo install jigc --version '^1.0.0-rc.1' --locked` at `QUICKSTART.md:21`, `README.md:19`, `crates/cli/README.md:20`; it installs rc.24 (6.1, 8.4) and the unpinned line resolves nothing (6.4, 8.5) |
| S9 | publication needs a main push, a missing version, the `release` environment approval and `release_always = false`; CI's publish is dry-run only | split: **OPEN LEAD** (the gated publish — agents may not) · **CONFIRMED** for the readable limbs | the only `cargo publish` in the workflows is `cargo publish --workspace --dry-run` (`ci.yml`); `release_always = false`; `dev/unpublished-versions` → `missing=false` |
| B1 | no schema-boundary violation; the two M55 findings doctypes sit at schema-version 1 | **CONFIRMED** (driven) | `jigc doc schema jigc-feedback --format json` and `… inconsistency …` → exit 0, `"schema-version": 1` each; every rig above loaded both packs clean |

Tally: 25 leads — 13 confirmed outright (D1, D2, D3, D5, S1, S4, S5, S6a, S6c, S6d, S7, S8, B1), 6 split
(N2, S2, S3, S6b, S6e, S9 — confirmed in part, refuted or open in part), 2 refuted (K, N5), 4 open in whole
(D4, N1, N3, N4).
**No Codex claim was a defect claim, so none became a finding by confirmation; two findings (F4, F5) came out
of driving its positive claims.**

### Repro blocks for the driven leads

```text
S1   bare rig (HOME); unborn repo; git config core.hooksPath .githooks; jigc setup --format json → exit 0
       "hook_committed": true, "hook_file": ".githooks/pre-commit", "install_commit": "6e2b33d"
       git show --name-only HEAD → 10 files: .claude/settings.json, .claude/skills/jigc/SKILL.md, .githooks/pre-commit, .gitignore,
       .jigc/.gitignore, .jigc/AGENT.md, .jigc/config/.gitkeep, .jigc/config/packs.yaml, .jigc/version, CLAUDE.md ; status empty
S2   bare rig; .claude/settings.json = '{"permissions":{"allow":["Bash(mine:*)",],},}' committed
     jigc setup --format json → exit 1, setup.inject-allowlist ; HEAD unmoved, status empty, no .jigc, no CLAUDE.md, no hook
S3   (soft member) bare rig; mkdir sub; an embedded repository in sub/ with one commit and sub/hooks/; git config core.hooksPath sub/hooks
     jigc setup → exit 0 ; "- pre-commit hook → sub/hooks/pre-commit … local to this checkout — git cannot track this path, so the
       hook is not in the install commit" ; the install commit: the 8 files, no hook ; outer status "?? sub/" ; the hook present and executable
     re-run: jigc setup --format json → exit 0, install_commit null, hook_committed false, no finding, HEAD unmoved
     then a marked line appended to .jigc/AGENT.md → exit 1, setup.dirty-install-path ; mark kept
S4   three bare rigs; CLAUDECODE set | removed with env -u | set to the empty string; jigc setup → exit 0 each
     git log -1 --format=%B: subject "chore(jigc): install jigc workspace config" in all three;
     lines equal to "Co-Authored-By: Claude <noreply@anthropic.com>": 1 | 0 | 0
S5   dev/jigc-rig bare --git-state detached → jigc setup --format json → exit 1, "blocking · repo.head-detached — HEAD is detached …
       route: re-attach HEAD with `git switch <branch>` …" ; status empty
     dev/jigc-rig bare --git-state merge → jigc setup --format json → exit 1, "blocking · repo.operation-in-progress — a merge is in
       progress … route: conclude it with `git merge --continue` … or abandon it with `git merge --abort` …" ; no .jigc
     dev/jigc-rig fresh --git-state detached → jigc uninstall --format json → exit 0, no finding ; .jigc gone
S6d  committed-singletons rig; mv docs/roadmap.md docs/plan.md; git rm -q --cached docs/roadmap.md
     git diff --cached --name-status --find-renames → "D docs/roadmap.md" (docs/plan.md untracked)
     git commit -m "docs: drop roadmap" → exit 0, "jigc: an out-of-band managed-doc rename exists in the committed tree — run
       `jigc validate` for details (not staged in this commit; commit not blocked)."
S6e  fresh rig; the hook replaced by '#!/bin/sh\necho mine\n'; jigc uninstall --format json → exit 0, "precommit": false ; hash unchanged
D1   four denylist files under <tmp>; dev/hygiene-scan <file> HEAD~3..HEAD
       one pattern matching nothing            → exit 0, "hygiene: denylist clean (1 pattern(s))"
       the same + a line "(unclosed[bracket"   → exit 2, "denylist <tmp>/bad line 2 is not a valid extended regex -- the scan did not run"
       comment + blank only                    → exit 2, "denylist <tmp>/empty holds no pattern"
       a path that does not exist              → exit 2
     hit control: a phrase from HEAD's own commit subject, range HEAD~1..HEAD → exit 1,
       "hygiene: 1 denylist hit(s) -- by location only, never by content:" / "  message  bffa6667ceee"
D2   printf 'not the asset\n' > <tmp>/bogus.tar.gz; dev/install-release-plz --archive <tmp>/bogus.tar.gz --dest <tmp>/dest  (GITHUB_PATH unset)
       → exit 1, "SHA-256 mismatch for release-plz-x86_64-unknown-linux-gnu.tar.gz: expected 1455106d… , got 1995d385… — not …" ; dest: 0 entries
D3, N2, B1 — in the table.
U.3  (the driver's declared-open row, re-driven) bare rig; tracked hooks/; core.hooksPath hooks; chmod 555 hooks; jigc setup → exit 1,
       setup.install-hook, 8 paths "A " ; chmod 755; jigc uninstall → exit 0 ; status: "AM .claude/settings.json" + 7 "AD"
```

## Baseline rows: CLOSED / STILL-OPEN

**None.** The row has no baseline (first drive) and the scope names no row to re-drive. The one declared-open
item, U.3 (`uninstall` after a failed first `setup` leaves `AD`/`AM` index entries — owed at M57), was
re-driven and is **STILL-OPEN (expected, declared)**; it is not a baseline row.

## Open leads

1. **lead(codex, D4)** — recovery of a release-PR merge whose deployment did not publish. Agents may not drive
   the release workflow; the doc text exists, the behaviour is unverified here.
2. **lead(codex, N1)** — the genuine assistant Task-tool spawn. Not this row's subject; not drivable here.
3. **lead(codex, N2, Linux half)** — the replaced-binary-while-running `/proc/self/exe` case. Not driven: it
   needs a binary replaced under a running process in the container, which no rig or `dev/runner-faithful`
   mode builds.
4. **lead(codex, N3)** — the cold CI runtime at this tree. A GitHub-runner fact.
5. **lead(codex, N4 / S9)** — the real release pipeline: the `release` environment approval, Trusted
   Publishing, the yank. Agents may not; `publish-proof.md` and `crates-page-reread.md` stand for earlier
   versions, and rc.24's own publish was only read (8.1, 8.2, 7.4).
6. **lead(codex, D2, the download half)** — `dev/install-release-plz` fetching the genuine asset and matching
   the pinned digest. Only the mismatch refusal was driven.
7. **lead(reconciler, beside F4)** — a `jigc` path carrying shell metacharacters after a single quote: the
   unquoted remainder of the hook is executed as words. Not driven (it would mean planting a hostile path);
   the three `command not found` lines in F4 are the observable form of it.
8. **Rows standing on the driver alone** — 8.3 and 8.7 (not re-driven), and S3's three non-commit failure
   sites (2.1, 2.10, 2.11). Not leads against the driver; recorded so the re-drive coverage is not overstated.

## Doors covered

Clap leaves that are the door of at least one driven row with a repro block in this file, `VERB_KINDS`
spelling:

- `setup` — cells 1–4, most of cell 5; F1, F2, F3, F4, and S1–S5.
- `uninstall` — 5.6–5.8, 5.12, 5.13, 5.35, U.1–U.3; S5, S6e.
- `validate` — only as the door of reconciler-driven lead rows (D3, N2, and F5's read-back); it confers
  nothing on this row's own cell set, whose leaves are the two above.
- `doc schema` — only as the door of B1.

Non-leaf doors (recorded, no leaf coverage): the installed pre-commit hook through `git commit` · the install
line · `cargo package --list` · `cargo publish --workspace --dry-run` · `dev/unpublished-versions` ·
`dev/runner-faithful tarball` · `dev/runner-faithful registry` · `dev/hygiene-scan` · `dev/install-release-plz`.

## Counts after reconciliation

- Confirmed findings: **5** — (R1, F1) tier 1 · (R1, F2) tier 3 · (R1, F3) tier 3 · (R1, F4) tier 3 ·
  (R1, F5) tier 3. Three from the driver (all reproduced), two from driving Codex leads.
- Driver rows demoted: **0**. Driver defects demoted to leads: **0**.
- Codex claims refuted in whole or part: K, N5, S2, S3, S6b, S6e. Open: 8 entries above.
- Tier 1: one finding, both halves shown, through `setup`.
