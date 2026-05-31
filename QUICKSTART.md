# Quickstart

`jigc` is a context compiler for coding agents: a deterministic CLI that
assembles exactly the instructions and document slices an agent needs for a
task, and is the sole channel through which the agent reads and writes managed
project documents. This walks the MVP loop on a real machine —
**install → start → finalize**.

> The product name is settled as `jigc` (`DECISIONS.md` 2026-05-31 → Product
> name). The binary is the same name.

## Install

Build the release binary from the workspace and put it on your `PATH`:

```sh
cargo build --release -p cli
# the binary lands at target/release/jigc
cp target/release/jigc /usr/local/bin/   # or anywhere on $PATH
```

Confirm it identifies itself:

```sh
jigc --version          # → jigc <version>
```

## 1. `jigc setup` — adapter install

Run once per project, from the repo root. `setup` generates the Claude Code
adapter from the embedded profile and installs it idempotently
(`design/assistant-adapter.md` → Generated, minimal, regenerated):

```sh
jigc setup
```

It does two things:

- injects a marker-fenced bootstrap line into `CLAUDE.md` (the static floor
  that tells the agent `jigc` is its interface to the project); and
- allowlists `jigc *` in `.claude/settings.json` (so the agent can call the CLI
  without a permission prompt).

Both edits are idempotent — re-running `jigc setup` leaves the files
byte-identical, so it is safe to run after every upgrade to re-apply the
adapter.

## 2. `jigc start "<intent>"` — mint and compose a task

Hand the agent (or yourself) a task by stating the intent. `start` mints the
task and composes the `single-task` workflow over the resolved cascade
(`design/write-commands.md` → Task origination):

```sh
jigc start "add per-client rate limit at the gateway"
```

This opens the task's working area under `.jigc/tasks/<id>/` and emits the
composed workflow — the instructions plus the document slices the task needs.
The agent reads its intent and the codebase, implements the change directly in
the working tree, and (when a decision is warranted) records an ADR in-task
through the workflow's `create` gate.

Bare `jigc start` (no intent) orients instead: it reports project state and the
next action, read-only.

## 3. `jigc task finalize <id>` — the commit boundary

When the work is done, finalize. This is the single transactional boundary
(`design/finalize.md`): it validates, renders the commit-doc into the git commit
message, promotes any created ADR into `decisions/`, and `git`-commits the code
changes plus promoted docs as one commit.

```sh
jigc task finalize <id>
```

If validation blocks (a dangling forward reference, a missing required slot, a
malformed value), finalize makes no commit and surfaces the findings with a
route for the next action. Fix and re-run.

You can preview what finalize will gate on at any time:

```sh
jigc task validate <id>
jigc task diff <id>
```

---

That is the whole loop: **`jigc setup`** once, then **`jigc start "<intent>"` →
implement → `jigc task finalize <id>`** per task. See
[`design/worked-examples.md`](design/worked-examples.md) for end-to-end flows
and [`implementation/roadmap.md`](implementation/roadmap.md) for the build
sequence.
