# Row 1 · setup · the hook · install · release — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. Source of the row: M54's S18
axes *setup*, *the hook*, *install*, *release*. **No numbered-axis predecessor — first drive.**

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it (`env -u CLAUDECODE …`
  or `CLAUDECODE= …`). On this row that is expected, not a finding — row 10 owns it. Hold the
  variable constant across any two cells whose commit messages you compare.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R1, <id>)`.
- Under `.jigc/` use `command grep` with a before-control (the harness `grep` honours `.gitignore`).

## READ FIRST

- **Baseline: none — first drive.** The neighbouring numbered-axis rows on `setup`/`uninstall`
  (posture, destroying doors, the install commit's rollback) are *not* this row's; do not re-drive
  them.
- The contract: `implementation/release.md` (whole — especially *What the package carries*,
  *Installing*, *Verifying a publish*, *What agents may not do*, *Known gaps*);
  `design/project-setup.md` → *Idempotency & irreversibility* and → *The secrets-floor `.gitignore`*;
  `design/assistant-adapter.md` → *The adapter's owned artifacts*, → *`jigc setup` has
  assistant-neutral install responsibilities too*, → *The three responsibilities*;
  `design/validation.md` → the paragraph opening *The M19 pre-commit backstop stays doc↔code-keyed*;
  `crates/cli/guides/QUICKSTART.md` → *Install* and → *1. `jigc setup`*; `crates/cli/guides/MIGRATING.md`.
- What changed: `completions/artifacts/M54/VERDICT.md` (scenarios 8–13, *Not run*, *Declared
  bounds*), `completions/artifacts/M54/publish-proof.md`,
  `completions/artifacts/M55/crates-page-reread.md`, DECISIONS.md → *M54 settled* S9–S13, S21, S22.

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| `install_tracked_paths` — the install commit's pathspec | `crates/cli/src/setup.rs` | 7 unconditional members + 3 conditional (the fresh-repo root `.gitignore`, the guide artifact, the pre-commit hook) |
| `install_path_dispositions` / `InstallPathDisposition` | `crates/cli/src/setup.rs` | 2 dispositions; the exempt-when-jigc-owned members are `.jigc/version` and the guide |
| `BEHALF_DOORS` rows for `setup`, `uninstall` | `crates/cli/src/cli.rs` | 48 rows in all; `setup` = commits-on-behalf with one posture exemption (unborn HEAD); `uninstall` = neither |
| `IGNORE_DOORS` | `crates/cli/src/gitignore.rs` | 4 |
| the `setup.*` / `uninstall.*` finding codes | `crates/cli/src/*.rs` (grep — there is no registry) | 20 distinct `setup.*` literals, 13 distinct `uninstall.*` |
| `PrecommitTeardown` (what uninstall does to a hook) | `crates/cli/src/setup.rs` | 3 variants |
| `include` + `readme` | `crates/cli/Cargo.toml` | 6 patterns + `README.md` |
| `PUBLISHED_DIRS` | `crates/cli/tests/package_contents.rs` | 4 |
| the job list | the header comment and `jobs:` of `.github/workflows/ci.yml`, `.github/workflows/release.yml` | read, not counted |

**Doors:** `jigc setup` · `jigc uninstall` (only where it reverses what setup wrote) · the installed
pre-commit hook, through a real `git commit` · the install line · `cargo package --list -p jigc` ·
`cargo publish --workspace --dry-run` · `dev/unpublished-versions` · `dev/runner-faithful tarball` ·
`dev/runner-faithful registry '^1.0.0-rc.1'`. Only `setup` and `uninstall` are `VERB_KINDS` leaves;
the rest are recorded as rows and confer no leaf coverage.

## CELL SET

1. **Install shapes** — a fresh repo · an established repo with its own `CLAUDE.md`, `.gitignore`
   and `.claude/settings.json` · a re-run (byte-idempotent, no second commit) · an upgrade over an
   older `.jigc/version` stamp (`store-version.binary-mismatch` then the restamp) · HEAD unborn.
2. **The failed first run (S22)** — a read-only `core.hooksPath` · a commit step that fails (no git
   identity; a failing signing program) — then a **plain** re-run after the cause is removed, and
   the control: the same with a user edit to an install path in between (the guard must re-arm).
   For each: `git status --porcelain` before and after, and what the refusal says was written.
3. **The settings pre-check** — a malformed committed `.claude/settings.json` (trailing comma ·
   non-object root · a non-array `allow` · a scalar `permissions` · a scalar `hooks`): refused with
   nothing written; re-run clean after the fix; the user's own entries kept.
4. **The dirty-install guard** — each member of the pathspec dirty before the run, without and with
   `--force`; the forced-path advisory names what the consent was spent on.
5. **The hook** — jigc's own hook · a foreign hook present before setup (wrapped, its bytes kept) ·
   `core.hooksPath` inside and outside the repository · a repository path containing a space · the
   binary reached through a symlink · on a real commit: the doc↔code warning (non-blocking), the
   out-of-band rename **block** over a `git mv` of a location doctype and of each placement
   singleton, and silence on a commit that contains neither · `uninstall` restoring a wrapped
   foreign hook byte-for-byte.
6. **Install** — the QUICKSTART line (see the environment note: never bare on this host) · the
   unpinned `cargo install jigc` resolving nothing · one file in the install root's `bin/` ·
   `jigc --version`.
7. **Package** — `cargo package --list -p jigc` equal to the allowlist from both sides (nothing
   outside it listed; every git-tracked file under the four directories listed; no `tests/`); the
   `-p jigc-engine` listing recorded as it is (the engine publishes no allowlist — state what it
   carries); the packaged README is the generated crate README · the publish dry-run for both
   crates.
8. **Release** — `dev/unpublished-versions` against the registry (expected: nothing missing at this
   commit) · every `uses:` in both workflow files is a 40-hex pin (a read; record it as a
   classification row) · the two `dev/runner-faithful` acceptance modes.

## BASELINE ROWS TO RE-DRIVE

None. M54's e2e scenarios 8–13 and its *Not run* list are context, not a baseline in the review's
sense; say for each *Not run* entry whether this run drove it.

## RIG STATES

- **`bare`** — the only state `setup` itself can be probed from. Almost every cell starts here.
- **`dev/jigc-rig --git-state unborn`** (standalone form) or a plain `git init` in a fresh `mktemp -d`
  — the unborn-HEAD cell. Say which you used; the standalone form carries a hand-made project-layer
  marker.
- **`fresh`** — uninstall, and the hook on an installed repo.
- **`vendored`** — the hook's doc↔code warning (it carries an anchored symbol to drift).
- **`committed-singletons`** — the hook's rename block (four placement singletons; add a
  location-doctype doc through the binary for the control).
- `chatty-hooks` **replaces** jigc's hook by design, so it is not a fixture for jigc's own hook cells.

## ENVIRONMENT NOTES

- **Docker.** The orchestrator recorded Docker as *not running* when this row was approved; a probe
  made while this instrument was written answered. **Probe it yourself, once, before the release
  cells** (`docker version --format '{{.Server.Version}}'`, exit status read bare). If it answers,
  drive `dev/runner-faithful tarball` and `dev/runner-faithful registry '^1.0.0-rc.1'` (both run on a
  clean clone of the committed revision; each takes minutes). If it does not, mark every cell that
  needs it **NOT DRIVEN (Docker not running)** — never infer its result.
- **Never run the bare install line on this host.** Nine other drivers are using
  `~/.local/bin/jigc` and cargo's install root while you run. Drive the line into a private root:
  `R=$(mktemp -d "$SCRATCH/install.XXXXXX")` then
  `cargo install jigc --version '^1.0.0-rc.1' --locked --root "$R"`, and record that `--root` is the
  one token you added. The unmodified line's faithful home is `dev/runner-faithful registry`. It
  needs the network and a C compiler; if either is missing, mark it NOT DRIVEN with the reason.
- **`cargo publish` is dry-run only.** A real `cargo publish` uploads a permanent version — never
  run it, never drop `--dry-run`. Use the argv CI uses (`cargo publish --workspace --dry-run`) from
  the repository root with a private `CARGO_TARGET_DIR` under your scratch root. The workspace
  versions at this commit are already on crates.io: record verbatim whatever the dry-run says about
  that, and do not grade a registry-state message as a product defect.
- **The `package_contents` fence** is `cargo test -p jigc --test g_migrate package_contents::`. It
  builds and runs the **debug** test target, so it is a **fence row** — recorded, conferring no
  coverage, and never a substitute for the `cargo package --list` drive.
- **What you may not do at all** (`implementation/release.md` → *What agents may not do*): merge or
  push `main`, merge a pull request, push a tag, publish, approve a deployment, re-run the release
  workflow. Those cells are **NOT DRIVEN (agents may not)**; the published record
  (`completions/artifacts/M54/publish-proof.md`, `completions/artifacts/M55/crates-page-reread.md`)
  is what stands for them.
- Read-only on the repository: `cargo` writes only under the private target directory.
