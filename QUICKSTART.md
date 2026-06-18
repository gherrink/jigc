# Quickstart

`jigc` is a context compiler for coding agents: a deterministic CLI that
assembles exactly the instructions and document slices an agent needs for a
task, and is the sole channel through which the agent reads and writes managed
project documents. This walks the MVP loop on a real machine —
**install → start → finalize**.

> The product name is settled as `jigc` (`DECISIONS.md` 2026-05-31 → Product
> name). The binary is the same name.

## Install

`jigc` ships as a single binary. Install it from the workspace and put it on
your `PATH`:

```sh
cargo install --path crates/cli      # or: cargo build --release -p cli
cp target/release/jigc /usr/local/bin/   # if you built rather than installed
```

Just `jigc` — nothing else to copy. The doc↔code probe (`doc-code`) is
embedded in the binary and extracted beside it by `jigc setup` (below), so a
`cargo install` from a fresh machine is a supported install channel: no manual
probe copy, and code-anchor finalize / `jigc validate` work out of the box.

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

It does these things:

- writes the managed bootstrap file `.jigc/AGENT.md` and adds a bare
  `@.jigc/AGENT.md` import to `CLAUDE.md` (the static floor that tells the agent
  `jigc` is its interface to the project — a plain import line, no marker fence);
- allowlists `jigc *` in `.claude/settings.json` and adds a `SessionStart` hook
  that runs `jigc start` (so each session opens with orientation), so the agent
  can call the CLI without a permission prompt;
- extracts the embedded `doc-code` probe beside the installed `jigc` (so the
  doc↔code probe resolves next to the binary — written if no sibling is present
  **or** if an existing sibling's bytes differ from the embedded copy, so a
  stale or corrupt probe self-heals; a byte-identical sibling is left untouched); and
- installs a warn-only `pre-commit` hook that runs `jigc validate` over the
  committed store and **warns** on doc↔code drift. It never blocks the commit
  (always exits 0) — a backstop for edits made outside the loop, not a gate.

`setup` **commits its own install** as a dedicated `chore(jigc): install` commit
(only the files above, never your working tree), so the install doesn't land in
your first feature commit. These edits are idempotent — re-running `jigc setup`
leaves the files byte-identical (no new commit), so it is safe to run after every
upgrade to re-apply the adapter.

## 2. `jigc start "<intent>"` — route, then mint

Hand the agent (or yourself) a task by stating the intent. `start` composes
the cascade-default **router** (`design/write-commands.md` → Task
origination): it presents the selectable work-workflows with the situation
each fits, and emits the re-run line — nothing is minted yet:

```sh
jigc start "add per-client rate limit at the gateway"
# → the workflow menu, then the re-run line:
#   jigc start --workflow <chosen> "<intent>"
```

Pick the workflow whose situation fits the intent and re-run with that
choice — this is the call that mints:

```sh
jigc start --workflow single-task "add per-client rate limit at the gateway"
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

**What lands in your repo:** finalize promotes each managed doc to its doctype's
location at the **repo root** — `decisions/` (ADRs), `specs/`, `prds/`,
`architecture/` (arch-docs), `changelog/` — as plain, human-reviewable Markdown
alongside your own `src/`. (The names are chosen for readability, not a uniform
`<type>s/` rule.) Everything else jigc writes lives under `.jigc/` (committed
config + bootstrap; gitignored caches) — see
[design/storage.md](design/storage.md) → Repository layout. If you already have a
same-named dir (say an existing `architecture/`), run `jigc ingest` first — it
detects and routes existing content rather than overwriting it.

You can preview what finalize will gate on at any time:

```sh
jigc task validate <id>
jigc task diff <id>
```

---

That is the whole loop: **`jigc setup`** once, then **`jigc start "<intent>"` →
pick from the menu → `jigc start --workflow <chosen> "<intent>"` → implement →
`jigc task finalize <id>`** per task. See
[`design/worked-examples.md`](design/worked-examples.md) for end-to-end flows
and [`implementation/roadmap.md`](implementation/roadmap.md) for the build
sequence.
