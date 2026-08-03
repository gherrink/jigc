# Command catalog

The typed schema behind `{{cli.<id>}}` command-refs. A workflow names a command-ref; the cascade resolves it to an entry in the catalog; the renderer emits a literal shell-safe command line into the composed workflow. This is what makes "every other command the agent ever needs is learned **just-in-time**" ([bootstrap.md](bootstrap.md)) concrete: the agent never memorizes a CLI vocabulary, never improvises invocations — the workflow gives the exact command at the exact moment.

Builds on [workflow-dialect.md](workflow-dialect.md) (command-refs are one of three placeholder kinds; the data-value grammar), [overrides.md](overrides.md) (catalog entries resolve through the cascade like any pack content), [validation.md](validation.md) (`workflow-refs.command-ref-resolves` validates entries at compose-time), [write-commands.md](write-commands.md) (the verbs the catalog typically targets), and [bootstrap.md](bootstrap.md) (JIT learning of commands). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## What a command-ref is

A workflow embeds `{{cli.<id>}}` — e.g., `{{cli.set-commit-summary}}`. At compose-time, the CLI:

1. resolves `<id>` against the cascade-merged catalog (project > team > pack-default);
2. fills each `from:` arg from the workflow's data-value context;
3. leaves each `agent:` arg as a `<NAME>` marker in the rendered command;
4. shell-quotes args that need it;
5. emits the result inside a `Run: \`<cmd>\`` line per the [emitted format](workflow-dialect.md#emitted-format).

The agent receives a literal command to type. No improvisation, no memorized vocabulary.

## Catalog format

A catalog is a YAML list of command-refs, one entry per `id`. Each entry:

```yaml
- id: <string>            # the {{cli.<id>}} name
  command: <string>       # the executable (typically `jigc`)
  args: [<arg>, …]        # ordered list; each arg is literal, from:, or agent:
  stdin: <string>         # optional; documents what stdin carries (renderer ignores)
  hint: <string>          # one-line authored documentation of the command-ref; projected by `jigc describe` (M11, its first consumer)
```

### The three arg kinds

| arg form | meaning | renders as |
|---|---|---|
| `"<literal>"` (a plain string) | a fixed token | itself, shell-quoted if needed |
| `{ from: <data-value-path> }` | resolved at compose-time against the workflow's data-value context | the resolved value, shell-quoted if needed |
| `{ agent: <name>, hint: <string> }` | left for the agent to fill at run-time | `<NAME>` (uppercase identifier, single-angle brackets) |

The data-value-path in `from:` uses the workflow-dialect grammar ([workflow-dialect.md](workflow-dialect.md) → Leaves): `task.commit#summary` for an address, `@task.spec#criteria` for content, `task.id` for a scalar. The path resolves in the same context as a workflow-step placeholder, so the catalog can navigate any state the workflow has — task roots, store roots, pack-provided roots.

### Worked catalog

```yaml
# packs/dev/commands.yaml
commands:
  - id: set-commit-summary
    command: jigc
    args:
      - "doc"
      - "set-slot"
      - { from: "task.commit#summary" }
      - "--from-file"
      - "-"
    stdin: "the slot prose"
    hint: "Stage the commit summary slot from stdin."

  - id: validate-task
    command: jigc
    args: ["task", "validate", { from: "task.id" }]
    hint: "Preview part of the finalize gate — content findings, carryover, owner-artifact — without committing."

  - id: finalize-task
    command: jigc
    args: ["task", "finalize", { from: "task.id" }]
    hint: "Run validate + commit. One task → one commit."

  - id: create-adr
    command: jigc
    args:
      - "doc"
      - "create"
      - "adr"
      - "--title"
      - { agent: title, hint: "short declarative sentence describing the decision" }
    hint: "Create a new ADR in the current task."
```

### What the renderer emits

For `{{cli.set-commit-summary}}` composed in a task `add-rate-limiter` workflow:

```text
Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -`
```

For `{{cli.create-adr}}`:

```text
Run: `jigc doc create adr --title <TITLE>`
```

The `<TITLE>` is the agent-substitution marker; the surrounding workflow prose (typically Reason-class) carries the `hint` for what to fill.

## Three bracket families — distinct on purpose

The composed output has three bracket families, each filled by a different party at a different time:

| family | example | who fills | when |
|---|---|---|---|
| `{{...}}` placeholder | `{{task.commit#summary}}` | CLI | compose-time (gone from emitted text) |
| `<<...>>` slot-author | `<<author: commit:foo#summary>>` | LLM | write-path (through `jigc doc set-slot`) |
| `<...>` agent-substitution | `<TITLE>` | LLM | run-time (typed into shell) |

`<...>` is visually distinct from `<<...>>`: single-angle vs double, uppercase identifier with no payload vs `name: address` payload. They cannot be confused at a glance.

## Shell-safe rendering

Per-arg POSIX quoting, deterministic:

- An arg matching `[A-Za-z0-9._:#/=@+-]+` is **bare** — addresses (`commit:foo#summary`), identifiers, flag names (`--from-file`), single-dash stdin (`-`).
- An arg containing whitespace or any of `* $ ; & | < > ( ) { } " ' \\ ! ?` is **single-quoted**. An embedded `'` is closed-escaped-reopened (`'it'\''s'`).
- The `<NAME>` agent marker is **bare** (its character set is the bare-allowed set).

The emitted backticked command is copy-paste safe in any POSIX shell. The renderer's algorithm is deterministic and pure — same args in → same emitted line out, no environment variance.

## Validation

Extends `workflow-refs.command-ref-resolves` ([validation.md](validation.md) → Severity inventory) — no new probe. At compose-time, for every `{{cli.<id>}}` in the workflow:

- the catalog has an entry under `<id>` after cascade resolution (else: `orphaned`),
- every `from: <path>` arg's path resolves in the workflow's data-value context (else: a dangling-placeholder finding, same shape as a workflow-body `{{...}}` that fails),
- every `agent: <name>` arg has a non-empty `hint` (the agent needs to know what to substitute),
- the composed command renders cleanly (no impossible quoting — e.g., `from:` resolving to a binary blob).

All findings are intrinsic-blocking (locked) per the inventory: a broken command-ref means the workflow would emit a broken or ambiguous Run line, which is exactly the determinism boundary leak the system exists to prevent.

## Cascade override behavior

Standard cascade deltas through the [9-phase resolution algorithm](overrides.md#resolution-algorithm). Three common shapes:

```yaml
# project config — examples
deltas:
  # 1. Add a project-specific command-ref
  - kind: insert
    target: catalog
    content:
      - id: lint-staged
        command: jigc
        args: ["task", "validate", { from: "task.id" }, "--probe", "lint"]
        hint: "Run only the lint probe against staged changes."

  # 2. Replace a pack command-ref entirely (e.g., to point at a wrapper script)
  - kind: replace
    target: cli:finalize-task
    content:
      id: finalize-task
      command: ./scripts/finalize-wrapper
      args: [{ from: "task.id" }]
      hint: "Project's finalize wrapper — runs CI smoke before commit."

  # 3. Remove a command-ref no project workflow uses
  - kind: remove
    target: cli:create-adr   # any workflow referencing this now surfaces a workflow-refs finding
```

A `scalar-set` against a single arg is also possible (`scalar:` block, keyed `catalog.set-commit-summary.hint: "..."`) but most one-line tweaks are clearer as a `replace`.

## What the catalog does NOT do

- **No control flow.** Args are literals / data-value reads / agent markers. No conditionals, no loops, no environment-variable interpolation. The composed command is a single deterministic shape.
- **No multi-line commands.** Heredocs / line-continuations are out of scope. If a project needs them, it ships a script and references the script in `command:`.
- **No agent reasoning over alternatives.** A workflow names exactly one `{{cli.<id>}}` per Run line; the agent doesn't pick between catalog entries at compose-time. Selection (when needed) is a workflow-routing concern, not a catalog concern.
- **No LLM call.** Resolution is deterministic — same catalog + cascade + workflow data-value context → same emitted command line.

## Open questions

- **Per-workflow command-ref scoping** — catalogs are globally accessible by id today. Whether to introduce per-workflow imports (so a workflow declares which command-refs it can name) is a post-MVP question that becomes interesting only if catalogs grow large enough to risk namespace collisions.
- **`stdin:` as a workflow data-value binding** — currently the `stdin:` key is documentation only; the agent or surrounding workflow prose says what to pipe. A future extension could bind stdin to a data-value (so the workflow emits "feed in this slice"). Flagged as not-yet-needed.
- **Multi-target / variant commands** — the same logical operation against different doc-types (e.g., a generic `set-slot` that adapts its target type). Currently one command-ref per concrete invocation; abstracting over types is a post-MVP question.
