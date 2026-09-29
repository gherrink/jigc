# Release

**The home for how jigc is versioned, packaged, published and installed.** Settled at M54's planning, 2026-09-28, with the independent design review's findings accepted the same day ([DECISIONS.md](../DECISIONS.md) → *M54 settled*), and **built by M54** ([roadmap.md](roadmap.md) → M54): until M54 lands, nothing on this page runs — the version stamp is still the hand edit the [milestone close](milestone-completion-workflow.md) performs, and no crate has been published from this repository. The crate *architecture* this rides on is [module-layout.md](module-layout.md); the CI jobs are [dev-workflow.md](dev-workflow.md) → Gate; the machine that proves an install is `dev/runner-faithful`, which M54 commits beside [`dev/gate`](../dev/gate). This page states the rules and cross-references the rest.

**Two kinds of statement below, and each is marked.** *Settled* is the human's decision and changes only through a new one. **Pinned by the build** is an implementation pick M54 makes and records here when it lands — named so nobody mistakes an open detail for a decided one.

## Packages and names

| Package (crates.io) | Directory | Targets | Promise |
|---|---|---|---|
| **`jigc`** | `crates/cli/` | bin `jigc` (`default-run`); lib `cli` | The binary is the product. The `cli` lib is `#![doc(hidden)]` and declared **not an API, no semver promise** in the README and the crate docs — it exists as the pinning suites' enumeration seam ([module-layout.md](module-layout.md) → Crate topology). |
| **`jigc-engine`** | `crates/engine/` | lib `jigc_engine` | The deterministic core ([module-layout.md](module-layout.md)); 0.x, so no stability promise yet. |

*Settled.* The engine's `[lib] name` is `jigc_engine`, and `jigc` declares it as `engine = { package = "jigc-engine", version = "=<v>", path = "../engine" }` — the dependency is renamed back to `engine`, so none of the ~2,640 internal `engine::` paths moves (only the engine crate's own ~12 self-references do). **The rename lands in one commit** with everything that spells the old package names: `ci.yml`'s pinned `live_wiring` line (which `manifest_freeze_fence` fences), `.claude/agents/build-executor.md`, `.claude/agents/increment-validator.md` and `.claude/workflows/milestone-build.js`. The names are chosen to fit what is already on the record as coming: a future **`jigc-mcp`** depends on `jigc-engine`, and the frontend code it would share with `jigc` is extracted to its own crate **then**, not now; the built-in packs relocate to a pack crate at the seam [module-layout.md](module-layout.md) → *The dev pack's home* prepares. **No further names are reserved.**

## Versioning

*Settled:*

- **`jigc` is on the `1.0.0-rc.N` track** until the 1.0.0 call. The rc track **is the version string**: the release tool only increments `N`. Leaving the track — `release-plz set-version` by hand to `1.0.0` — **is** the human's 1.0.0 call, and nothing automated performs it.
- **`jigc-engine` is versioned independently** — 0.x, its own prerelease track starting at **`0.1.0-rc.1`**, bumped only by changes to its own paths. `jigc` pins it with **`=`**, so a `jigc` build names exactly one engine.
- **Each crate carries its own `version` field**, never `version.workspace = true`: release-plz silently fails to write an inherited one, so an inherited field is a bump that does not happen.
- **Versions are computed from conventional commits, by the paths each commit changed.** Docs-only commits never bump anything — this repository's doc-heavy history would otherwise release on every record edit. The filter that enforces it (the spike's `release_commits` regex admitted `feat`, `fix`, `perf`, `refactor`, `revert`) is **pinned by the build**.
- **The version the product reports is `jigc`'s.** The `.jigc/version` store stamp ([storage.md](../design/storage.md) → Store provenance) and the embedded packs' `pack_version` ([module-layout.md](module-layout.md) → The dev pack's home) both carry `jigc`'s version, never the engine's — the engine has no `CARGO_PKG_VERSION` use and gains none. So does the probe's build id ([module-layout.md](module-layout.md) → Probe boundary).
- **A bump moves no golden.** In the ten version-bearing compose goldens (`Pack: dev/<ver> | methodology/<ver>`), exactly the string equal to `env!("CARGO_PKG_VERSION")` is normalized to `<jigc-version>` — narrowly, so a regression to `0.1.0` or to the engine's version still diffs. The `env!("CARGO_PKG_VERSION")` test pins are left alone: they follow the bump by construction. `CLAUDE.md` stops carrying a pipeline-owned version number a fence pins; how `foldback_truth`'s *built and installed* arm is re-cut is **pinned by the build**.

## The release PR — how a version is proposed

*Settled:* the tool is **release-plz**. *Why not knope, which was spiked beside it:* knope attributes a change by commit **scope** rather than by the paths it touched, does not bump a dependent when its dependency moves, and so breaks the `=` pin above.

1. On every push to `main`, release-plz opens or updates **one workspace release PR** — not one per package (release-plz docs, `usage/release-pr.md`) — carrying every releasable package's version bump and changelog section. Its title comes from a **conditional** `pr_name` template: `{{ package }}` is populated only when a single package releases, and an unconditional use of it fails the run (`config.md`), so the template guards it with `{% if package and version %}`.
2. **The PR is opened with a GitHub App token**, not the workflow's own token. The App — created by the human — has **Contents: read/write and Pull requests: read/write**, is installed on `gherrink/jigc` — and, **for the rehearsal only**, on the private rehearsal repository (below), from which the human uninstalls it afterwards — and its ID and private key live in repository secrets. *Pinned by the build:* the secret names and the App's exact settings.
3. **Merging the release PR is the go.** The human merges it; nothing else starts a publish.

**Changelogs.** Each crate keeps its own generated changelog at `crates/<dir>/CHANGELOG.md`. **The root `CHANGELOG.md` is not written by release tooling**: it is reserved for jigc's own `changelog` doctype, and whether this repository's changelog is the generated one or a jigc-authored one is a fork keyed to M56's Settle ([decisions-pending.md](decisions-pending.md) → *The road to 1.0.0 and the port* → M56).

**release-plz knobs — pinned by the build, except what the spike proved.** Proven on the spike's throwaway workspaces: a per-crate `version` field is written and a workspace-inherited one silently is not. Everything else the spike's sketch set — `release_always = false`, `semver_check = false`, `dependencies_update = false`, the `release_commits` filter, per-package `changelog_path`, `git_tag_name`, `git_release_type = "auto"` (an `-rc` version marked prerelease), `publish_timeout`, `protect_breaking_commits`, the `pr_name` template above — is a starting point the build pins and records here, not a settled value.

## Publishing

*Settled:*

- **Workflow `release.yml`; the publish job runs in GitHub environment `release`** with the human as **required reviewer** and **`can_admins_bypass: false`** — agents run `gh` with the human's admin credentials, so an admin bypass would let an agent skip the approval (set at the first halt, with the reviewer rule, once the repository is public; the free plan's support for it is verified then). The environment's **deployment policy admits branch `main` only** (policy 61320798, set 2026-09-28; the `v*` tag policy 61287763, left from the superseded tag-triggered design, deleted), because the release job runs on the `main` push after the release-PR merge. The job does **not** key on a commit subject — the merge strategy is unchosen, and a default merge commit reads *Merge pull request #N*, not `chore(release)`. Instead a **preceding job outside the environment** checks whether any workspace package's version is missing from crates.io and outputs a flag; the environment-bound job `needs` it and runs **only when the flag is set**. So the approval prompt appears for a push that has something to publish and for no other.
- **crates.io Trusted Publishing** (OIDC): the job holds `id-token: write`, and **no `CARGO_REGISTRY_TOKEN` exists** in the repository. The trusted-publisher configuration on crates.io names repository `gherrink/jigc`, workflow `release.yml`, environment `release`, for both crates.
- **Publish order: `jigc-engine`, then wait until the index serves it, then `jigc`** — the `=` pin makes `jigc` unpublishable before its engine is resolvable.
- **Tags** are `<package>-v<version>` (`jigc-v1.0.0-rc.22`, `jigc-engine-v0.1.0-rc.1`), created by the release job; GitHub releases for an `-rc` version are marked prerelease.
- **Rehearsed dry before it runs for real — a required step.** Before the real rc.22 publish, the whole pipeline — release-pr, then release with `--dry-run` — runs on a **private throwaway repository, `gherrink/jigc-release-rehearsal`** (a mirror push of `main`), with the **same** GitHub App installed on it for the rehearsal, so the first run of the workflow's wiring is not the one that spends a version. It is **dry-run only**: Trusted Publishing is bound to `gherrink/jigc`, so nothing can publish from there. Afterwards the human uninstalls the App from it and **deletes the repository** — irreversible, so a human-confirmed step, never an agent's.
- **Real prerelease publishes, not dry runs.** M54 closes with a real publish of **`jigc 1.0.0-rc.22`** plus **`jigc-engine 0.1.0-rc.1`**; M55 publishes the next. A crates.io version is permanent — that is why the pipeline is exercised on prereleases before the version that matters. The human approves each one at the environment.
- **CI checks publishability and the lock on every push**: `cargo publish --workspace --dry-run` and a lock-current check (`cargo metadata --locked`) are CI jobs ([dev-workflow.md](dev-workflow.md) → Gate).

## What the package carries

*Settled:*

- **`jigc` publishes an `include` allowlist**, never the directory: `src/`, the packs, the adapter profiles, the guides at **`crates/cli/guides/`** (`QUICKSTART.md`, `MIGRATING.md` — their only copies; the root keeps none, and the root README links to them), `README.md` and the license pair (copied or declared into the crate). **`tests/` is not published.** *Pinned by the build:* how the README and the license files enter the crate.
- **`jigc-engine` publishes its in-source unit tests** with it. **Declared bound:** tests that read pack files cannot run from either tarball — the engine's own reach outside the engine crate for their pack root, and `jigc`'s integration suites are not shipped at all. The published crates are proven by building and installing them, not by testing from the tarball.

## Installing

*Settled:*

- **One static install line:** `cargo install jigc --version '^1.0.0-rc.1' --locked`. The requirement matches every `1.0.0-rc.N` and every later `1.x`, and never the `0.0.0` placeholder, so **no release rewrites it** and no release tooling touches a doc. **`QUICKSTART.md` owns the line**; the root README carries the one allowed copy, byte-identical and held so by a fence; every other place points at QUICKSTART. **Verify-before-rely, in two halves:** when the line lands, a test drives the `semver` crate — the library cargo matches requirements with — asserting `^1.0.0-rc.1` matches `1.0.0-rc.22`, `1.0.0` and `1.3.0` and **not** `0.0.0`; after the first publish, a real registry install with the line is the full proof ([Verifying a publish](#verifying-a-publish)).
- **Prerequisites:** Rust ≥ 1.95 (the workspace's `rust-version`), a **C compiler** (the `doc-code` probe's tree-sitter grammars are C and now build inside `jigc`), and a **Unix** host. `QUICKSTART.md` gains these at M54, inside the one guide batch ([assistant-adapter.md](../design/assistant-adapter.md) → *One batch, one hash move*).
- **One binary.** The `doc-code` probe ships **inside** `jigc` (self-exec — [module-layout.md](module-layout.md) → Probe boundary), so an install is exactly one file and `jigc setup` writes nothing outside the repository. **Upgrading from `1.0.0-rc.21` or earlier leaves an orphaned `doc-code` beside the old binary** (e.g. `~/.local/bin/doc-code`); nothing deletes it automatically, and `MIGRATING.md` says so.
- **A failed first `jigc setup` no longer wedges the repository.** Until M54, a setup that failed after its first write left its own writes dirty and the re-run refused them as the user's (`setup.dirty-install-path`), recoverable only by `--force`; M54 records the install footprint on every failure path, so a plain re-run completes ([project-setup.md](../design/project-setup.md) → Idempotency & irreversibility).
- **The two `0.0.0` placeholders** — `jigc@0.0.0` (2026-07-25) and `jigc-engine@0.0.0` (2026-09-28) — held the names and let Trusted Publishing be configured, which crates.io allows only on a crate that already exists. **Both are yanked after the first rc publish**, with the human's approval. From then on only prereleases are unyanked, and cargo never selects a prerelease unasked, so an *unpinned* `cargo install jigc` resolves nothing — which is why the install line carries the requirement.

## Verifying a publish

*Settled:* the proof of a release is an **install**, not a build — and the two installs M54 runs prove different things.

- **Before the publish**, the `dev/runner-faithful` image installs from the packaged tarball. **Declared bound on `--locked`:** cargo 1.95's `--locked` uses the lock as written and does not refuse an incomplete or stale one, and the path/patch overlay that lets the tarball resolve its unpublished engine bypasses the engine's lock entry. So this install proves only *the packaged lock was used* — checked by byte-comparing the lock and by a compiled-in version check — not that a registry install reproduces it.
- **After the publish**, the same image runs `cargo install jigc --version <v> --locked` against crates.io, then `jigc setup` in a throwaway repository and one flow that exercises the `doc-code` probe. **This is the first faithful registry proof.** The trial harness's **registry mode** installs by version the same way and is the blind trial's instrument ([completions/trial-harness/README.md](../completions/trial-harness/README.md)).

## What agents may not do

*Settled:* **agents never merge the release PR, never push a tag, and never approve a deployment**; the deployment-approval API is denied in the agents' permissions (agent definitions and settings). **This is adapter-enforced, not by construction** — agents run `gh` with the human's own credentials, so a permission entry is the only thing between an agent and a merge — the same *adapter-enforced, not sandboxed* bet [VISION.md](../VISION.md) principle #3 names for jigc itself. The real fix is a **restricted agent token** that cannot merge, approve or tag, owed at [decisions-pending.md](decisions-pending.md) → *The road to 1.0.0 and the port* → *The 1.0.0 call*.

## The one-time bootstrap

*Settled — the order inside M54:* a **pre-public audit** whose verdict the human signs off → **the human flips the repository public** and adds himself as the `release` environment's required reviewer, with `can_admins_bypass: false` → the *before* CI measurement → the restructure → the CI rework and the *after* measurement → release-plz, the GitHub App and this bootstrap → the dry-run rehearsal → **the rc.22 publish**.

**The audit.** A full-history secret scan (gitleaks) plus a targeted grep for tokens, keys, `.env` files and credentials. A real secret is rotated or revoked **first**, and history is rewritten only if something live is found. Personal data already in history (home paths, the work email, session transcripts) is accepted. Third-party or client data is **redacted at HEAD — or history is rewritten if it is sensitive.** A rewrite breaks citations, and that breakage is then handled deliberately, never discovered: the SHA-pinned `manifest_freeze_fence::historical` window (`2c5eee5~1 → 7ada302`) and the `ci.yml` comment naming it, every doc that cites a commit SHA, and the trial harness's build-by-sha.

**The bootstrap itself:** the human creates the App and its secrets; the first versions — `jigc 1.0.0-rc.22`, `jigc-engine 0.1.0-rc.1` — land with **short hand-seeded changelogs**, so release-plz does not walk the repository's ~2,530 prior commits into the first changelog.

## Known gaps

- **A publish whose tag step fails is not re-tagged by a re-run** — release-plz sees the version already on crates.io and skips it. The manual fix is the human's: create `<package>-v<version>` at the release merge commit, push it, and create the GitHub prerelease by hand.
- **The build id does not see a same-version rebuild.** It is `CARGO_PKG_VERSION`, which catches the realistic skew (an upgrade mid-run) and not a dev rebuild at one version; on Linux, spawning `/proc/self/exe` removes the replaced-binary case outright ([module-layout.md](module-layout.md) → Probe boundary).
- **The OIDC exchange cannot be rehearsed.** The trusted-publisher configuration names `gherrink/jigc`, so the rehearsal repository cannot exchange its token for a crates.io one and `--dry-run` does not need one: the first exchange is the real rc.22 publish. Acknowledged at M54's planning, not fixable by design.
- **No semver check runs on the rc track.** What the engine's compatibility promise is once `jigc` leaves the track is owed, not decided ([decisions-pending.md](decisions-pending.md) → *The road to 1.0.0 and the port*).
