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

`jigc` ships as a single binary, and there is **one** install command. It is run
from a **clone of the jigc repository** — `crates/cli` is a path in *that* tree, so
the line fails from the project you are adopting jigc into, which has no such
directory. This is the only place this guide states it, so an upgrade is the same
line again:

```sh
# cwd: a clone of the jigc repository, not your own project
cargo install --path crates/cli
```

That builds the release binary and puts `jigc` in cargo's own install root —
`~/.cargo/bin` unless you have moved `CARGO_HOME` — which is already on your
`PATH` if cargo is. Placing a built binary somewhere else by hand works, but then
that copy is yours to keep current, and every instruction below to re-run `jigc
setup` after an upgrade means replacing it first.

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
  that wrote it plus the hash of its own body, and `jigc setup` rewrites it whenever
  the body still hashes to that stamp, so the guidance in your repo matches the binary
  in your `PATH`. Re-run `setup` after upgrading and the copy follows — **until you
  edit it**. An edited copy is yours: `setup` leaves it byte-identical, raises the
  `adapter-guide.user-modified` advisory instead of overwriting it, and drops it from
  the rest of the install's commit — the other bullets here still run — so from then on it stops tracking the binary until you delete it and
  re-run (`jigc upgrade` reports the same state without replacing anything);
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

**It refuses that commit rather than sweeping your work into it.** Before writing
anything, `setup` compares every path in its own install footprint against `HEAD`.
If any of them already carries changes that are in no commit — staged, unstaged or
untracked — it stops with one blocking `setup.dirty-install-path` naming each such
path and installs **nothing**: `HEAD` is untouched, and your bytes are still exactly
where you left them — including at the files jigc regenerates whole (`.jigc/AGENT.md`,
`.jigc/config/packs.yaml`), which is why the question is asked before the first write
rather than before the commit. Commit or stash them (`git stash -u` where git does not
track them yet) and re-run, or pass `jigc setup --force`, the single consent — which
lets the install run and commit those paths as it leaves them, and *says* which paths
it was spent on, since at a regenerated path what it leaves is jigc's content and not
yours. This is the same rule as the carryover gate at `jigc task
finalize` below — a door committing paths it does not own says so instead of
sweeping them in — at its other door.

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
next action and changes nothing in your repository. That holds for every *writes
nothing* claim in these guides, under one jigc-wide exception: with the opt-in
`invocation-log` knob on, **every** jigc run — these reads included — appends one
record to the gitignored `.jigc/logs/invocations.jsonl`. A run that claims to
write nothing writes that record and nothing more.

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

Your own `pre-commit` / `commit-msg` hooks still run: no jigc commit of *your*
work passes `--no-verify`, so they are your policy, and a hook that rejects the
commit stops the finalize. Nothing is committed, the task survives — its staged
docs still in `.jigc/tasks/<id>/docs/`, anything you had `git add`-ed still in git's index —
and the message carries the hook's own output verbatim plus the line to re-run
once its complaint is fixed. Every jigc verb that commits on
your behalf answers a rejection that way, each stating what *its* rejection
left behind — the per-door detail is in [MIGRATING.md](MIGRATING.md) →
Reconciling and backing out.

The `--no-verify` sentence above is about the commits that carry *your* work, and
it has exactly one exception, which carries none of it: `jigc setup`'s own install
commit passes `--no-verify`, so that the warn-only `pre-commit` hook `setup` has
just written cannot fire on the commit installing it. That commit carries only the
bytes `setup` itself wrote — it refuses outright rather than committing anything
else (§1 above) — so there is no work of yours inside it for a hook of yours to
have policy over.

A landed finalize prints a **manifest** — the file-set the commit carried (the
git index: what you staged plus the docs jigc promotes), with a distinct
**left-out** list naming any unstaged or untracked work the commit excluded (so
a stray `scratch.txt` stands out as left behind, never swept in; each path is
tagged by how it enters, and [MIGRATING.md](MIGRATING.md) → Reconciling and
backing out names the whole tag vocabulary once).

Finalize then clears the task's working area — but it is not a delete. The commit
carries the promoted docs and your index, nothing out of `.jigc/tasks/<id>/`, so
anything under there that jigc did not write (a scratch note, an analysis file,
something you dropped beside the staged docs) is **moved** first to
`.jigc/displaced/<id>/` at the same relative path, and every `from → to` pair is
named on stderr and on the landed `--format json` envelope's `committed.displaced`.
`jigc milestone finalize` does the same for each sub-task's area **and for the
milestone's own area**, `.jigc/milestones/<id>/` — the join's `merged/` tree
included — each area's bytes parked under `.jigc/displaced/` at that area's own
unit id, the sub-task's or the milestone's. Nothing clears
`.jigc/displaced/` — those bytes are yours to read and remove with your own `rm` —
and `jigc uninstall` refuses while it holds anything (`uninstall.foreign-bytes`), so
they never go out with the workbench.

A byte jigc **cannot** move is not taken instead. If the parking path is occupied
or unwritable, or a hook wrote into the area while the commit was running, that
byte stays exactly where it is and the working area is **left standing** around it
rather than removed. The commit still landed and the run still exits 0 — a kept
byte is not a failed finalize — and jigc says so with one `finalize.foreign-bytes`
advisory per area it left standing: it names what is still in there (or says it
could not read the area to tell you, rather than claiming an absence it did not
check), plus the reason a move failed where there was one. The advisory rides
`jigc task finalize`'s `findings`; at `jigc milestone finalize` it is printed on
stderr. What is in such an area is no more jigc's to judge than what is in
`.jigc/displaced/`: keep what you need out of it and delete the rest yourself.

To see that set *before* committing, run `jigc task finalize <id> --dry-run`. On a
task that would otherwise commit cleanly it prints the manifest and stops,
committing nothing. It forecasts no green it would refuse: over a state finalize
blocks — an unfilled required slot, a carried-over staged path — the dry run emits
those blocking findings instead of a manifest and takes the refusing exit, so a
manifest coming back from `--dry-run` is itself the news that nothing this side of
the commit blocks.

**Landed it with a wrong message? `jigc task amend`.** It mints a task pinned to the
commit at `HEAD` and provisions an empty `commit` doc, which you author through the
same `jigc doc` verbs as any other — then `jigc task finalize <id>` renders it and
rewrites that commit's message with `git commit --amend`, leaving every byte of the
committed **tree** as it is. So it repairs the *message*, never the *change*: a wrong
change is a new task. You re-author the message from scratch rather than editing the
old one — jigc does not read the landed message back into the doc, because a body
paragraph shaped `Refs: <value>` is indistinguishable from a trailer on the way back,
and guessing wrong there would corrupt the one thing this verb exists to repair. Two
things to know before you run it: **the index must be empty**, because
`git commit --amend` rewrites the commit *from* the index and would otherwise fold
your staged work into a commit that never carried it (the amend refuses instead — one
blocking `finalize.amend-index-dirty` per staged path, and there is no flag that
declares that carry deliberate); and jigc **cannot tell which commit it is** — it
amends `HEAD`, whatever `HEAD` is — so the mint prints `HEAD`'s current subject line
before the instructions, and if that subject is a milestone boundary, a bookkeeping
commit jigc wrote for itself, or a commit jigc did not make, stop. If the commit has
already been pushed, amending it rewrites shared history; jigc has no reliable way to
know whether it has, so that is your call, not a gate.

A change staged *before* the task existed refuses to ride the commit — one blocking
`finalize.carried-staged` per carried path — unless you declare it with
`--carry-staged`. That is the narrower of two members of one rule: **a door
committing paths it does not own says so instead of sweeping them in**, and `jigc
setup`'s install commit is the other (§1 above). It is not a universal over every
jigc commit — `jigc rename`, `jigc migrate-corpus` and the milestone record-only
doors commit without asking this question at all.

Changed your mind? **`jigc task discard <id>`** abandons the task — it removes
only the working area under `.jigc/tasks/`; for an ordinary task no commit is made
and the committed store is untouched. (One task shape is different, and it is the
only one: a **milestone sub-task** is named by a committed milestone record, so
discarding it settles that one record item to `discarded` and commits the record on
its own — before the area is removed, so a refused commit leaves both intact — and
the ack names the sha it landed on a `record commit:` line. Nothing of yours is
committed either way; only the record moves.) It **refuses first if that area
stages docs no commit has a copy of** — one blocking `task-discard.staged-prose`
naming each of them, so you can read them back (`jigc doc show <address> --task <id>`) or land them
(`jigc task finalize <id>`) before deciding. It refuses over the working area's **other**
population too — every path there jigc did not write, one blocking
`task-discard.foreign-bytes` naming each, since `.jigc/` is gitignored whole and
nothing else has a copy. **`--force` is the single consent**
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

You can preview part of what finalize will gate on at any time — the repository
posture `finalize` refuses under (an un-concluded merge, rebase or pick; a
detached HEAD), this task's content findings, the carryover gate, the
`owner-artifact` causes that need no staging, and the granted-but-unused
changelog gate. The staged set, promotion
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
