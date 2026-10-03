# version-pinned README links — the crates.io page links the tree it was built from

**Status: parked 2026-10-02.** Parked by the M55 Settle, S15 ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; design of record [design/findings-channel.md](../design/findings-channel.md) → 8). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

M55 fixes the crates.io README's broken links by generating a committed crate README from the root one at commit time, every relative link rewritten to the repository URL at **`blob/HEAD/<path>`** — the base crates.io itself uses, so a link reads the same on GitHub and on crates.io. `HEAD` is the default branch. While every published version is a release candidate of one line and the default branch is that line, *the current tree* is the right target. Once a supported published line and the default branch diverge, a user reading an older version's crates.io page lands on guides for a different version.

## The direction

Pin each published README's links to the release tag — `blob/<tag>/<path>` — so a version's page links the tree that version was built from. The generator already owns the rewrite; it would take the ref as a parameter.

## Why parked

The pinned form must be regenerated **inside the release PR** with the version's tag, which makes release tooling touch a doc — against the settled rule that no release rewrites the install line or any doc ([release.md](../implementation/release.md) → Installing). Today the cost buys nothing: there is one line.

## What it would cost

- A generator ref parameter and a release-PR step that regenerates and commits the crate README, with the byte-for-byte fence comparing against the tag-parameterized transform.
- An explicit revision of the *no release tooling touches a doc* rule, with its reasoning engaged.
- The post-publish 200-check M55 runs once, run on every release against the tag.

## Trigger

The default branch and a supported published line genuinely diverge — a 2.x on `main` while 1.x is still supported.
