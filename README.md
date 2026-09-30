# jigc

`jigc` — pronounced "jig-see" — is a **context compiler for coding agents**: a
deterministic CLI that assembles exactly the instructions and document slices an
agent needs for a task, just-in-time, and is the sole channel through which the
agent reads and writes a project's managed documents (commit messages, ADRs,
specs, PRDs, architecture docs, the changelog).

The idea it rests on: take every *structural* operation away from the LLM and
give it to the CLI, and leave the LLM only the prose. The CLI decides which
documents exist, their structure, their cross-references and where every write
lands, and validates the result at the commit boundary; the agent fills named
slots with text. A jig holds the workpiece and guides the tool so the cut lands
true — the CLI is the jig, the LLM is the tool.

## Install

```sh
cargo install jigc --version '^1.0.0-rc.1' --locked
```

This is a copy of the line [QUICKSTART.md](crates/cli/guides/QUICKSTART.md) owns,
kept byte-identical by a test. The prerequisites, what the `--version`
requirement admits, and how an upgrade works are stated there, once.

## Guides

- [QUICKSTART.md](crates/cli/guides/QUICKSTART.md) — install, then the core loop
  on a real machine: `jigc setup` → `jigc start "<intent>"` → `jigc task finalize <id>`.
- [MIGRATING.md](crates/cli/guides/MIGRATING.md) — adopting an existing project,
  and moving a managed corpus across jigc versions.

Both ship inside the `jigc` crate, and `jigc setup` installs them, concatenated,
as the agent's guide in the repository it sets up (`.claude/skills/jigc/SKILL.md`).
Why the design is shaped this way, and what has and has
not been measured about it, is in [VISION.md](VISION.md) and
[WHY-JIGC.md](WHY-JIGC.md).

## The `cli` library is not an API

The `jigc` package builds the `jigc` binary, which is the product, and a `cli`
library beside it. That library exists so this repository's test suites can
enumerate the binary's surfaces. **It is not an API and carries no semver
promise:** any release may change or remove anything in it. Depend on the
binary's documented commands and its pinned `--format json` output, never on the
library.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT), at your option.
