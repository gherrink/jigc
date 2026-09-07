# Quickstart

`jigc` is a context compiler for coding agents: a deterministic CLI that
assembles exactly the instructions and document slices an agent needs for a
task, and is the sole channel through which the agent reads and writes managed
project documents. This walks the MVP loop on a real machine —
**install → start → finalize**.

> The name is **jigc** — pronounced "jig-see" — *a jig for coding agents*: the
> jig holds the work and guides the tool so the cut lands true (settled
> 2026-07-03, `DECISIONS.md` → The name is settled). The binary is the same name.

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
- allowlists `Bash(jigc:*)` in `.claude/settings.json` and adds a `SessionStart` hook
  that runs `jigc start` (so each session opens with orientation), so the agent
  can call the CLI without a permission prompt;
- writes jigc's own guides — this file and the migration field notes — to the path
  your assistant reads skills from (`.claude/skills/jigc/SKILL.md` for Claude Code),
  as **one file jigc owns**: it opens with a `jigc-version:` stamp naming the build
  that wrote it plus the hash of its own body, and every `jigc setup` rewrites it, so
  the guidance in your repo always matches the binary in your `PATH`. Re-run `setup`
  after upgrading and the copy follows;
- extracts the embedded `doc-code` probe beside the installed `jigc` (so the
  doc↔code probe resolves next to the binary — written if no sibling is present
  **or** if an existing sibling's bytes differ from the embedded copy, so a
  stale or corrupt probe self-heals; a byte-identical sibling is left untouched); and
- installs a `pre-commit` hook that runs `jigc validate` over the committed
  store and **warns** on doc↔code drift without blocking the commit — a
  backstop for edits made outside the loop, not a gate. It refuses exactly one
  thing: a commit that itself stages an out-of-band managed-doc **rename** (a
  bare `git mv` of a managed doc, both paths staged), because that silently
  breaks identity tracking — use `jigc rename` instead. Drift that landed in an
  earlier commit only warns; it never blocks a later, unrelated one.

`setup` **commits its own install** as a dedicated
`chore(jigc): install jigc workspace config` commit, so the install doesn't land in
your first feature commit. It stages **only the files it wrote inside the repo** —
never your working tree — so two of the things above are *not* in that commit: the
`doc-code` probe (it lives beside the `jigc` binary, not in your repo), and the
`pre-commit` hook whenever git keeps hooks outside your working tree — the usual
`.git/hooks/`, a `core.hooksPath` pointing elsewhere, or a linked worktree's shared
hooks dir — because git cannot track a file there. If your repo keeps hooks *in* the
tree (an in-repo `core.hooksPath`), the hook is an ordinary tracked file and `setup`
commits it with the rest — unless git will not take that path from your repo, in which
case `setup` leaves the hook out of its commit and commits the rest normally. That
covers a hooks dir belonging to **another** repository (shared hooks vendored as a
submodule — checked out or not — or an embedded repo), where it is that repo's file to
commit, not yours, and a hooks dir your **sparse-checkout** excludes. Either way the
hook is installed and works locally, it just isn't in the commit; note that a committed
hook carries **this** machine's `jigc`
path, so in someone else's clone it stays silent until they run `jigc setup`
themselves. These edits are idempotent — re-running `jigc setup` leaves the files
byte-identical (no new commit), so it is safe to run after every upgrade to re-apply
the adapter.

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

### Reading while the task is open

Managed docs are read through `jigc`, never off disk — three reads cover it, and
`--task <id>` turns the first two onto the working copies your open task has
staged but not yet committed:

```sh
jigc doc list --task <id>             # what this task stages, each by its address
jigc doc show adr:<slug> --task <id>  # the staged doc itself
jigc doc schema adr                   # the shape a write has to fill
```

Drop `--task` and the first two serve the committed store instead. `jigc doc
schema` needs no task — it projects the resolved schema: required slots and
fields, each field's enum members, the value grammar of any pack-declared field
type, and every address a write can take.

A `code-anchor` field — `adr.cites-code`, `spec.criteria/maps-to-test`,
`arch-doc.components/implemented-by` — takes `<repo-relative-path>[#<symbol>]`:
a path from the repository root, optionally `#` and the name of a unit *declared*
in that file (`src/router.ts#createRouter`). It is not `file:line` and not
`Class.method`; a bare path with no `#` is legal and asserts only that the file
exists. `jigc doc schema <doctype>` and `jigc doc set-field --help` both print the
grammar, so there is nothing to remember.

## 3. `jigc task finalize <id>` — the commit boundary

When the work is done, finalize. This is the single transactional boundary
(`design/finalize.md`): it validates, renders the commit-doc into the git commit
message, promotes any created ADR into `docs/decisions/`, and `git`-commits the
code changes plus promoted docs as one commit.

```sh
jigc task finalize <id>
```

If validation blocks (a dangling forward reference, a missing required slot, a
malformed value), finalize makes no commit and surfaces the findings with a
route for the next action. Fix and re-run.

Your own `pre-commit` / `commit-msg` hooks still run — jigc never passes
`--no-verify`, they are your policy — so a hook that rejects the commit stops
the finalize. Nothing is committed, the task survives — its staged docs still in
`.jigc/tasks/<id>/docs/`, anything you had `git add`-ed still in git's index —
and the message carries the hook's own output verbatim plus the line to re-run
once its complaint is fixed. Every jigc verb that commits on
your behalf answers a rejection that way, each stating what *its* rejection
left behind — the per-door detail is in [MIGRATING.md](MIGRATING.md) →
Reconciling and backing out.

A landed finalize prints a **manifest** — the file-set the commit carried (the
git index: what you staged plus the docs jigc promotes), with a distinct
**left-out** list naming any unstaged or untracked work the commit excluded (so
a stray `scratch.txt` stands out as left behind, never swept in). To see that
set *before* committing, run `jigc task finalize <id> --dry-run`: it prints the
manifest and stops, changing nothing. A change staged *before* the task existed
refuses to ride the commit (`finalize.carried-staged`) unless you declare it
with `--carry-staged`.

Changed your mind? **`jigc task discard <id>`** abandons the task — it removes
only the working area under `.jigc/tasks/`; no commit is made and the committed
store is untouched. It **refuses first if that area stages docs no commit has a
copy of** — one blocking `task-discard.staged-prose` naming each of them, so you
can read them back (`jigc doc show <address> --task <id>`) or land them
(`jigc task finalize <id>`) before deciding. **`--force` is the single consent**
that discards them along with the area. Minting a task stages its commit doc, so
expect that refusal on any task you have actually started. The full
reconcile/back-out ladder (the migration review hold, the carryover gate, the
hook rejection above, out-of-band edits) is in
[MIGRATING.md](MIGRATING.md) → Reconciling and backing out.

**What lands in your repo:** finalize promotes each managed doc under the
**`docs-root`** parent (default `docs/`) at its doctype's location —
`docs/decisions/` (ADRs), `docs/specs/`, `docs/prds/`, `docs/architecture/`
(arch-docs), `docs/milestone-records/` — as plain, human-reviewable Markdown
alongside your own `src/`. (The names are chosen for readability, not a uniform
`<type>s/` rule.) A few doctypes are **placed** rather than located: they live at
one literal path that `docs-root` does not move — the changelog is your
repo-root `CHANGELOG.md` ([design/storage.md](design/storage.md) → Placement).
Prefer a different parent, or the old flat repo-root layout? Set
`jigc config set docs-root <path>` (`.` or `""` for the old flat layout). Everything else jigc
writes lives under `.jigc/` (committed config + bootstrap; gitignored caches) —
see [design/storage.md](design/storage.md) → Repository layout / Config layout. If
you already have a same-named dir (say an existing `docs/architecture/`), run
`jigc ingest` first — it detects and routes existing content rather than
overwriting it.

You can preview part of what finalize will gate on at any time — this task's
content findings, the carryover gate, the `owner-artifact` causes that need no
staging, and the granted-but-unused changelog gate. The staged set, promotion
and the commit itself (your hooks included) are decided at `finalize`, so a clean `validate` means *nothing this
side of the commit blocks it*, not *this will commit*:

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
