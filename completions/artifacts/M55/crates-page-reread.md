# The rc.23 crates.io page, read back (2026-10-03)

The read owed by M55's Settle S15 ([settle-log.md](settle-log.md)) and by the graduated decisions-pending row *Before the next release PR is merged — the README crates.io shows*: after the first publish carrying the generated `crates/cli/README.md`, every link the crates.io page renders must answer 200 at its intended target. `1.0.0-rc.22`'s page broke four of six ([../M54/publish-proof.md](../M54/publish-proof.md) → The README as crates.io renders it).

**Published:** `jigc 1.0.0-rc.23` and `jigc-engine 0.1.0-rc.2`, crates.io `created_at` 2026-10-03T18:43Z, by release run `37144894365` on `main` at `60c53e16` (the release PR #2 merge, after the M55 PR #9 merge `8520c16c`), approved by the human.

**Read:** the rendered README from `https://crates.io/api/v1/crates/jigc/1.0.0-rc.23/readme`, every `href` extracted, each absolute one fetched with `curl -sL -o /dev/null -w '%{http_code}'`.

| link | status | target |
|---|---|---|
| `https://github.com/gherrink/jigc/blob/HEAD/crates/cli/guides/QUICKSTART.md` | 200 | the guide (in the crate) |
| `https://github.com/gherrink/jigc/blob/HEAD/crates/cli/guides/MIGRATING.md` | 200 | the guide (in the crate) |
| `https://github.com/gherrink/jigc/blob/HEAD/VISION.md` | 200 | the root document |
| `https://github.com/gherrink/jigc/blob/HEAD/WHY-JIGC.md` | 200 | the root document |
| `https://github.com/gherrink/jigc/blob/HEAD/LICENSE-APACHE` | 200 | the root license file (no longer the crate-local symlink page) |
| `https://github.com/gherrink/jigc/blob/HEAD/LICENSE-MIT` | 200 | the root license file (no longer the crate-local symlink page) |

The five remaining hrefs are in-page anchors (`#user-content-install`, `#user-content-guides`, `#user-content-license`, `#user-content-jigc`, `#user-content-the-cli-library-is-not-an-api`).

**Verdict: all six links resolve to their intended targets.** S15 is discharged. The links point at `blob/HEAD`, so they follow `main` rather than the published version; the version-pinned alternative is parked at [ideas/version-pinned-readme-links.md](../../../ideas/version-pinned-readme-links.md).

**Installed:** `cargo install jigc --version '^1.0.0-rc.1' --locked --root ~/.local` (QUICKSTART's line, with this machine's install root) replaced `1.0.0-rc.22` with `1.0.0-rc.23`; `jigc --version` → `jigc 1.0.0-rc.23`.
