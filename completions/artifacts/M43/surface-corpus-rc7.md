# jigc CLI surface corpus — 1.0.0-rc.7 (captured 2026-07-17)

## $ jigc --version

```
jigc 1.0.0-rc.7
```

## $ jigc --help

```
The `jigc` CLI — a context compiler for coding agents

Usage: jigc [OPTIONS] <COMMAND>

Commands:
  start           Start or resume work. Bare `jigc start` orients (read-only); an `<intent>` composes the cascade's `default-workflow` — shipped as the `router`, a `creates-task: false` selection pass that routes the intent to a work-workflow and mints **nothing**; `jigc start --workflow <X> "<intent>"` composes `<X>` and mints the task iff `<X>` declares `creates-task: true`; `--task <id>` resumes an existing task and re-composes it
  workflow        Re-enter a milestone sub-task as a fanned sub-agent — compose the named sub-workflow `<W>` for sub-task `<id>`. `<W>` must equal the sub-task's recorded mint workflow, else the command is rejected. Distinct from `jigc start --task`, which recomposes a top-level task's own workflow
  doc             Read and write managed docs — the `jigc doc <verb>` surface
  task            The task lifecycle surface — `jigc task <verb> <id>` over a named task's working area
  config          The cascade-authoring surface — `jigc config <verb>` records deltas into the project layer (`.jigc/config/`)
  milestone       The milestone work-unit surface — `jigc milestone <verb>` mints a milestone, builds its sub-task list, and executes, joins, and finalizes it — or, when the work is abandoned, discards it
  setup           Install the Claude Code adapter — writes the `.jigc/AGENT.md` bootstrap and a reference into `CLAUDE.md`, initializes the project layer (`.jigc/config/`), allowlists `Bash(jigc:*)`, and installs the `SessionStart` and warn-only git `pre-commit` hooks. Idempotent
  uninstall       Reverse this project's jigc install — removes `.jigc/`, unwires the `CLAUDE.md` reference, and drops the `Bash(jigc:*)` permit from `.claude/settings.json`. Leaves the machine-global `doc-code` probe (shared across repos) in place. Idempotent and non-destructive: a second run is a clean no-op, and your own file content is preserved byte-for-byte
  upgrade         Re-check every recorded config delta against the current pack and report what needs attention. Report-and-route only — it changes nothing, and exits non-zero if any finding blocks
  ingest          Scan the project for existing markdown docs, adopt every conformant one (register-only — never moving or rewriting a file), and print a triage report. Misplaced or non-conformant files are flagged for a human; exits 0. A flagged non-conformant file is rewritten into managed shape with `jigc migrate <path> --as <doctype>`
  migrate         Rewrite a foreign, non-conformant document into managed shape — `jigc migrate <path> --as <doctype>`. Mints a migration task and composes the `migrate-<doctype>` workflow so the agent re-authors the content through the write verbs. Works for any doctype with a shipped `migrate-<doctype>` workflow — see the error message for the live set. An already-conformant file needs no rewrite: adopt it with `jigc ingest` instead
  migrate-corpus  Migrate the committed managed corpus onto the current schema — `jigc migrate-corpus` re-parses every committed doc of a frozen persisted doctype, applies the deterministic v0→v1 transform (the schema-version stamp + any structural splice) byte-stable, and writes each doc back only on a clean conformance gate (the stamp flips last). A prose-needing change is routed to the agent to author, then re-run. Detect-with `jigc validate`; this migrates. Disjoint from `jigc migrate` (foreign adoption) and `jigc upgrade` (config). The verb **lands its own migration** in a pathspec-limited commit; `--no-commit` leaves the writes unstaged, `--dry-run` writes nothing at all
  unmanage        Drop a single managed doc from jigc's index and state — `jigc unmanage <path>` removes its edges and file-state baseline, leaving the file bytes on disk (the inverse of `ingest`'s adopt). Idempotent: a re-run is a clean no-op
  rename          Rename a managed doc — `jigc rename <type>:<slug> --to "<New Title>"` is the CLI-owned identity refactor: it derives the new slug from the title, repoints every persisted referrer old→new, rewrites the moved doc's H1, `git mv`s it, and commits as one atomic transaction (rolling back cleanly on any failure). `--to` is required; `--slug` (only valid alongside `--to`) overrides the derived slug
  relocate        Relocate a **freeze-exempt** doctype's pre-existing committed instance(s) from a human-supplied prior home to its current schema home — `jigc relocate <type> --from <prior-home>`. The sibling of the version-gated `jigc migrate-corpus` relocation for frozen doctypes: a freeze-exempt doctype carries no prior-home snapshot, so the prior home is supplied by hand and each stranded instance is `git mv`d byte-faithful to the current home (never silently stranded). A frozen doctype is refused (use `migrate-corpus`)
  describe        Print a prose tour of what's available — every workflow and doc-type with their descriptions, plus command-ref hints — reflecting the resolved cascade. The output is a human menu, not a stable API; don't parse it
  validate        Re-check every committed doc's code anchors against the codebase and report drift — `jigc validate` is the store-wide, read-only sweep (distinct from `jigc task validate <id>`, which gates one task). Detect-and-report
  help            Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
  -V, --version          Print version
```

## $ jigc start --help

```
Start or resume work. Bare `jigc start` orients (read-only); an `<intent>` composes the cascade's `default-workflow` — shipped as the `router`, a `creates-task: false` selection pass that routes the intent to a work-workflow and mints **nothing**; `jigc start --workflow <X> "<intent>"` composes `<X>` and mints the task iff `<X>` declares `creates-task: true`; `--task <id>` resumes an existing task and re-composes it

Usage: jigc start [OPTIONS] [INTENT]

Arguments:
  [INTENT]  Optional task intent. Absent → orient (read-only); present → compose the cascade's `default-workflow` with `{{task.intent}}` = `<intent>`, minting a task iff that workflow declares `creates-task: true` (the shipped default, the `router`, does not — it routes to the workflow that does)

Options:
      --format <FORMAT>      Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
      --workflow <WORKFLOW>  Compose a named workflow explicitly, bypassing the cascade default. Mints a task iff the workflow declares `creates-task: true`. Combines with the `<intent>` positional; mutually exclusive with `--task`
      --task <TASK>          Resume an existing task by id: re-compose its default workflow over the task's persisted state (its base + bound context roles), so the agent picks up context bound since the task was minted (e.g. an ADR created in-task, now reachable as `task.<role>`). Mutually exclusive with `<intent>`
      --explain              Show the resolution tree instead of composing — the cascade provenance, the resolved include list with each step's winning layer, and the `replaces … at position` annotations — without minting a task. Combines with the `<intent>` positional and `--workflow <X>`; mutually exclusive with `--task`
      --slug <SLUG>          Override the minted task id (only meaningful when a task is minted). Taken **verbatim** and validated as a well-formed slug — a malformed value is rejected, never silently re-slugified. Mutually exclusive with `--task` (a resume mints nothing); inert on a non-minting compose (`design/write- commands.md` → `jigc rename`'s `--slug` precedent)
  -h, --help                 Print help
```

## $ jigc task --help

```
The task lifecycle surface — `jigc task <verb> <id>` over a named task's working area

Usage: jigc task [OPTIONS] <COMMAND>

Commands:
  list      Enumerate the active tasks (id + minting workflow + intent); no id needed
  diff      Show the working changeset vs base (code diff + staged managed docs)
  validate  Run `validate(task)` and render the findings; exit non-zero iff any blocks
  discard   Abandon the task — remove its working area `.jigc/tasks/<id>/`
  finalize  The commit boundary — validate, render, stage, `git commit`, post-commit
  bind      Bind an already-committed doc to one of the task's declared context roles, so `task.<role>` resolves to it on the resume re-compose
  help      Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc doc --help

```
Read and write managed docs — the `jigc doc <verb>` surface.

The write verbs (`create`/`add-item`/`set-field`/`set-slot`/`author`/…) work the active task's working area; the read verbs (`show`/`schema`/`list`) serve the committed store.

Usage: jigc doc [OPTIONS] <COMMAND>

Commands:
  create        Mint a new managed instance (agent-initiated, create-gated)
  add-item      Mint a repeatable item into a section, id-slugged from `--title`
  remove-item   Remove a repeatable item (top-level or nested) without discarding the task
  retitle-item  Retitle a repeatable item's heading — its `{#id}` anchor stays frozen
  set-field     Set a field leaf's value (inline, adjudicated at write time)
  set-slot      Set a slot leaf's prose (multi-line, via stdin or a file)
  author        Author a whole instance from one declarative payload — the batch verb.
  show          Read a managed doc, or an addressed `#section`/item/leaf slice of it
  schema        Project a doctype's **resolved** schema (the cascade-composed shape as loaded)
  list          List the **committed** store surface by identity, slug-sorted
  help          Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc create --help

```
Mint a new managed instance (agent-initiated, create-gated).

The title the slug is minted from is supplied inline with `--title` — always literally `--title`, whatever the doctype's `id-from` field is named; the CLI mints + places per the schema.

Usage: jigc doc create [OPTIONS] --title <TITLE> <TYPE>

Arguments:
  <TYPE>
          The doctype to create (e.g. `adr`)

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --title <TITLE>
          The title the slug is minted from — the flag is always literally `--title`, whatever the doctype's `id-from` field is named (`design/write-commands.md` → The argument convention)

      --slug <SLUG>
          Override the minted doc slug (`<type>:<slug>`), decoupling the id from the title. Taken **verbatim** and validated as a well-formed slug — a malformed value is rejected, never silently re-slugified (`design/write-commands.md` → `jigc rename`'s `--slug` precedent). Inert for a singleton doctype (its slug is fixed to the type id)

      --task <TASK>
          The active task to scope the write to. Optional: explicit wins; else the single active task; else (zero / more-than-one) the write rejects

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc author --help

```
Author a whole instance from one declarative payload — the batch verb.

The doctype-general batch write: applies the equivalent `create` plus N
`add-item` / `set-slot` / `set-field` over a single buffer, persisting once.
In the payload, a slot value is a YAML block scalar wrapped in literal
`<<…>>` markers (`summary: |` then, indented beneath it, `<<the prose>>`) —
required syntax that tags the value as slot prose, not a fill-me
placeholder to delete; the block scalar keeps multi-paragraph and bulleted
prose intact where a quoted flow scalar would fold the line breaks. An
inline field takes a bare value (wrapping one is rejected).

Reach for `author` to write a whole instance in one shot (a migration, or any
many-leaf doc) — it collapses what would be a `create` + N follow-up calls.
Use `create` then `set-slot`/`set-field`/`add-item` for incremental,
one-leaf-at-a-time authoring instead.

Payload shape (YAML; `--from-file`), mirroring the document's structure:

    title: <the create id-source>
    sections:
      - id: <section-id>
        set:                    # doc-level leaves: fields + slots
          status: accepted      # inline field — a bare value
          summary: |
            <<the slot prose>>   # slot — <<…>>-wrapped block scalar
        items:                  # repeatable rows under this section
          - title: <item id-source>
            set:
              date: 2026-07-11
            sections:           # nested repeatable level, parented by the item
              - id: <nested-section-id>
                set: { note: <<inline slot>> }

Usage: jigc doc author [OPTIONS] --from-file <FROM_FILE> <DOCTYPE>

Arguments:
  <DOCTYPE>
          The doctype to author (e.g. `changelog`) — minted through the create-gate

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --from-file <FROM_FILE>
          The payload source: a path, or `-` for stdin (the whole-doc payload is large, so it arrives the same way slot prose does — never inline)

      --task <TASK>
          The active task to scope the write to (see `Create::task`)

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc set-field --help

```
Set a field leaf's value (inline, adjudicated at write time).

For a list-cardinality (`0..*`) ref, set ALL values in one call with the inline-list form `--value "[a, b, c]"` — repeated single-value calls replace the whole list (and are rejected once it is populated, to prevent silently dropping prior entries).

Usage: jigc doc set-field [OPTIONS] <ADDR>

Arguments:
  <ADDR>
          The leaf address — `<type>:<slug>#<field>` (or `#<section>/<field>`)

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --value <VALUE>
          The new value (inline — fields are short + escaping-safe). A list-cardinality (`0..*`) ref takes the inline-list form `"[a, b, c]"` to set multiple values in one call. Exactly one of `--value` / `--unset` is required

      --unset
          Clear the field entirely — remove its line/bullet (an optional field re-conforms absent). Refused for author-required / defaulted / CLI-`set:` fields

      --task <TASK>
          The active task to scope the write to (see `Create::task`)

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc set-slot --help

```
Set a slot leaf's prose (multi-line, via stdin or a file)

Usage: jigc doc set-slot [OPTIONS] --from-file <FROM_FILE> <ADDR>

Arguments:
  <ADDR>  The slot address — `<type>:<slug>#<slot>`

Options:
      --format <FORMAT>        Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
      --from-file <FROM_FILE>  The prose source: a path, or `-` for stdin (prose never inline)
      --task <TASK>            The active task to scope the write to (see `Create::task`)
  -h, --help                   Print help
```

## $ jigc doc add-item --help

```
Mint a repeatable item into a section, id-slugged from `--title`.

The section is addressed `<type>:<slug>#<section>`; the CLI mints the `{#id}` anchor + appends the item block.

Usage: jigc doc add-item [OPTIONS] --title <TITLE> <ADDR>

Arguments:
  <ADDR>
          The section address — `<type>:<slug>#<section>` (the repeatable section the item is minted into)

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --title <TITLE>
          The item id-source — slugged to the `{#id}` anchor (the MVP surface; the item's slot/fields are filled by later `set-slot`/`set-field` writes)

      --task <TASK>
          The active task to scope the write to (see `Create::task`)

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc remove-item --help

```
Remove a repeatable item (top-level or nested) without discarding the task.

Addresses a top-level item (`<type>:<slug>#<section>/<id>`) or a nested one (`#<section>/<parent>/.../<nested-section>/<id>`) — a general recovery verb over the engine's `remove_item` / `remove_nested_item`.

Usage: jigc doc remove-item [OPTIONS] <ADDR>

Arguments:
  <ADDR>
          The item address — top-level `<type>:<slug>#<section>/<id>` or the nested section-qualified chain `#<section>/<parent>/.../<nested-section>/<id>`

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --task <TASK>
          The active task to scope the write to (see `Create::task`)

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc retitle-item --help

```
Retitle a repeatable item's heading — its `{#id}` anchor stays frozen.

The retitle-without-reslug invariant's verb at item level. Addresses the same item forms as `remove-item`: top-level `<type>:<slug>#<section>/<id>` or the nested section-qualified chain. An item whose id derives from an **enum** field (e.g. a changelog change-group's `category`) refuses unconditionally — a member change is an identity change — routing to `remove-item` + `add-item` under the target category.

Usage: jigc doc retitle-item [OPTIONS] --title <TITLE> <ADDR>

Arguments:
  <ADDR>
          The item address — top-level `<type>:<slug>#<section>/<id>` or the nested section-qualified chain `#<section>/<parent>/.../<nested-section>/<id>`

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --title <TITLE>
          The new heading title (the item's `{#id}` anchor stays frozen)

      --task <TASK>
          The active task to scope the write to (see `Create::task`)

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc show --help

```
Read a managed doc, or an addressed `#section`/item/leaf slice of it.

Served through the canonical parse/render path. **Committed by default**: task-less, it reads the **committed** store — the view a fresh session or a teammate on a clone sees. A doc still staged in an open task is not committed yet; read it with `--task <id>`, which serves that task's **staged** working copy through the identical parse/slice path (the read-back of an in-flight write) — including a **transient** doc's staged copy (`commit:<task-id>`, which never commits to a repo file). Plain text is the canonical render; `--format json` is the pinned stable shape (`design/doc-read-surface.md`): a whole-doc object `{ type, slug, fields, sections }` — a staged serve adds the one `staged` key carrying the task id — where a slot section serializes to its prose string and a repeatable section to its item array; a `#section` slice returns that section's value (item array / slot prose), an `#section/<id>` slice the item object, an `#section/<id>/<leaf>` slice the leaf.

Usage: jigc doc show [OPTIONS] <ADDR>

Arguments:
  <ADDR>
          The doc address — `<type>:<slug>`, or a `#section`/item/leaf slice of it

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

      --task <TASK>
          Read this open task's **staged** working copy instead of the committed store (same address, identical parse/slice path)

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc schema --help

```
Project a doctype's **resolved** schema (the cascade-composed shape as loaded).

Injected stamp field included — the third read surface, next to `describe` (the non-contractual menu) and `doc show` (the committed-content read). `--format json` is the separately-pinned, explicitly versioned contract (`contract-version: 3` — `design/doc-read-surface.md` → Why json is a contract here); plain text is a non-contractual human listing. Task-less — a schema projection is never task-scoped.

Usage: jigc doc schema [OPTIONS] <DOCTYPE>

Arguments:
  <DOCTYPE>
          The doctype whose resolved schema to project (e.g. `adr`)

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc doc list --help

```
List the **committed** store surface by identity, slug-sorted.

The fourth read surface, next to `describe` (the menu), `doc show` (the content read) and `doc schema` (the schema read). Every instance of one doctype (or of every persisted doctype) carries its `<type>:<slug>` identity, its repo-relative path, and its **registration state**: `managed` (jigc's own doc) or `unregistered` (a file at a managed home jigc never adopted — adopt it with `jigc ingest` / `jigc migrate <path> --as <doctype>`). `--format json` is the pinned shape `{"docs":[{id, path, state}]}` (no in-band version integer — `design/doc-read-surface.md` → the fourth read surface). Task-less: it reads the committed store, never an open task's staged buffer.

Usage: jigc doc list [OPTIONS] [DOCTYPE]

Arguments:
  [DOCTYPE]
          The doctype to list (optional — omit to list every persisted doctype)

Options:
      --format <FORMAT>
          Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal)
          
          [default: agent]
          [possible values: agent, json, human]

  -h, --help
          Print help (see a summary with '-h')
```

## $ jigc migrate --help

```
Rewrite a foreign, non-conformant document into managed shape — `jigc migrate <path> --as <doctype>`. Mints a migration task and composes the `migrate-<doctype>` workflow so the agent re-authors the content through the write verbs. Works for any doctype with a shipped `migrate-<doctype>` workflow — see the error message for the live set. An already-conformant file needs no rewrite: adopt it with `jigc ingest` instead

Usage: jigc migrate [OPTIONS] --as <AS> <PATH>

Arguments:
  <PATH>  The repo-relative path of the foreign document to migrate (e.g. `CHANGELOG.md`)

Options:
      --as <AS>          The target managed doctype the foreign document is rewritten into — any doctype with a shipped `migrate-<doctype>` workflow — see the error message for the live set
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
      --slug <SLUG>      Override the migrated doc's slug (`<doctype>:<slug>`), decoupling the id from the authored title. Taken **verbatim** and validated as a well-formed slug before any task is minted — a malformed value is rejected, never silently re-slugified (`jigc doc create --slug`'s discipline, recorded in the migration task and applied when the agent authors the target doc)
  -h, --help             Print help
```

## $ jigc milestone --help

```
The milestone work-unit surface — `jigc milestone <verb>` mints a milestone, builds its sub-task list, and executes, joins, and finalizes it — or, when the work is abandoned, discards it

Usage: jigc milestone [OPTIONS] <COMMAND>

Commands:
  create         Mint a milestone work-unit (id = frozen slug from the title), opening its gitignored area with one shared base pinned at HEAD and an empty task list
  add-task       Mint a sub-task under an existing milestone (pinned to the milestone's shared base, isolated `tasks/<sub>/` area) and append it to the task list
  add-from-spec  Seed a milestone's task list from a committed spec: mint one sub-task per repeatable `criterion` of the spec (criterion text as intent). A spec with zero criteria blocks with `milestone.no-criteria` ("nothing to seed from")
  list-tasks     Emit a milestone's sub-task ids in canonical id-sorted order — the deterministic order the by-task-id join enumerates
  provision      Provision the milestone's fan-out worktrees: add one **detached** `git worktree` per sub-task at the milestone's recorded **base pin** under the gitignored `.jigc/worktrees/<sub-task-id>` path, so each fanned sub-agent gets an isolated code checkout. Idempotent — reuses a live worktree, clears a stale leftover from a crashed run. Run as a `Run:` step before the fan-out (`design/storage.md` → repository layout)
  execute        Compose the `milestone-execution` workflow over the milestone, feeding its id-sorted sub-task list into `{{milestone.tasks}}` so the `fan-out` step resolves it (one `Spawn:` directive per sub-task). Mints nothing — the milestone and its sub-tasks already exist; an unknown milestone is rejected
  join           Merge the milestone's sub-task areas into the parent working overlay by the by-task-id join — enumerate sub-areas by sorted task id, disjoint-union their staged docs (collision-suffixing distinct created instances), and report the merged outcome. Commits nothing. A blocking finding (a same-doc clash, an unknown milestone) surfaces on stderr with its route and exits non-zero
  finalize       The milestone commit boundary — run the by-task-id join, materialize its suffix-resolved doc bodies into the parent staging area, and commit them as one logical boundary with a CLI-synthesized message. A blocking join finding (a same-doc clash, an unknown milestone) routes to stderr and commits nothing
  discard        Abandon the milestone: settle its committed record to the `discarded` terminal (a genuinely **joined** sub-task stays `joined` — it really did land) in one record-only commit, then tear the workbench down (the sub-task areas, the registered fan-out worktrees, and `.jigc/milestones/<id>/`). Refuses when any sub-task worktree holds uncommitted work, unless `--force`
  help           Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc validate --help

```
Re-check every committed doc's code anchors against the codebase and report drift — `jigc validate` is the store-wide, read-only sweep (distinct from `jigc task validate <id>`, which gates one task). Detect-and-report

Usage: jigc validate [OPTIONS]

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc ingest --help

```
Scan the project for existing markdown docs, adopt every conformant one (register-only — never moving or rewriting a file), and print a triage report. Misplaced or non-conformant files are flagged for a human; exits 0. A flagged non-conformant file is rewritten into managed shape with `jigc migrate <path> --as <doctype>`

Usage: jigc ingest [OPTIONS]

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc describe --help

```
Print a prose tour of what's available — every workflow and doc-type with their descriptions, plus command-ref hints — reflecting the resolved cascade. The output is a human menu, not a stable API; don't parse it

Usage: jigc describe [OPTIONS]

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc rename --help

```
Rename a managed doc — `jigc rename <type>:<slug> --to "<New Title>"` is the CLI-owned identity refactor: it derives the new slug from the title, repoints every persisted referrer old→new, rewrites the moved doc's H1, `git mv`s it, and commits as one atomic transaction (rolling back cleanly on any failure). `--to` is required; `--slug` (only valid alongside `--to`) overrides the derived slug

Usage: jigc rename [OPTIONS] --to <TO> <type:slug>

Arguments:
  <type:slug>  The `<type>:<slug>` address of the doc to rename (e.g. `adr:single-node-cache`)

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
      --to <TO>          The new title — the moved doc's H1, and the slug source unless `--slug` overrides. Required
      --slug <SLUG>      Override the derived slug (only valid alongside `--to`)
  -h, --help             Print help
```

## $ jigc config --help

```
The cascade-authoring surface — `jigc config <verb>` records deltas into the project layer (`.jigc/config/`)

Usage: jigc config [OPTIONS] <COMMAND>

Commands:
  set           Record a `scalar-set` delta — set a closed-surface knob in the project layer's `manifest.yaml` `scalar:` block. Adjudicated at write time against the pack's declared knob (`check_value`): an undeclared key or a wrong-type value is rejected non-zero with a routed finding
  insert-step   Splice a native step into a workflow's include list, anchored `--after` or `--before` an existing step. The native step takes its id from the source `<file>`'s basename and is written to `.jigc/config/steps/<basename>.yaml`. Write-time adjudicated: a basename colliding with an existing step id, or an anchor absent from the current resolution, is rejected non-zero with no write. (Unlike `replace-step`/`remove-step`, which take a `workflow:<id>#<step-id>` address, insert needs an anchor step *and* a side — which the `#` address can't express — hence the `--workflow` + `--after`/`--before` form.)
  replace-step  Swap which step appears at a position in a workflow's include list, addressed `workflow:<id>#<step-id>`. The replacement is the native step the `<file>` registers (id = file basename, written to `.jigc/config/steps/<basename>.yaml`). Write-time adjudicated: a basename colliding with an existing step id, or a target step-id absent from the current resolution, is rejected non-zero with no write
  remove-step   Drop the step at a position in a workflow's include list, addressed `workflow:<id>#<step-id>`. No native file (nothing to add). Write-time adjudicated: a target step-id absent from the current resolution is rejected non-zero with no write
  fill          Inject content into a `{{fill:<fill-id>}}` extension point a step body anticipates, addressed `step:<id>#<fill-id>`. The content arrives via stdin / `--from-file` (prose, never inline) and is written to `.jigc/config/fills/<fill-id>.md`. Two write-time checks (both before any write): the `{{fill:<fill-id>}}` point must exist in the resolved step body, and the content must contain no nested `{{fill:}}`. A rejection exits non-zero. (Distinct from `doc set-slot`: `fill` injects into a *workflow step's* extension point — cascade-authoring; `set-slot` fills a managed *document's* slot — the write path.)
  fork          Copy a resolved step's body into a native file that shadows it, recording the pinned ancestor (`base-version` + the blake3 `base-hash` of the copied bytes), addressed `workflow:<id>#<step-id>`. The body bytes are copied verbatim to `.jigc/config/steps/<step-id>.yaml`, so at compose time the fork is just a file shadow. Write-time adjudicated: the target step-id must resolve, and a unit already forked is a collision — each is rejected non-zero. The recorded basis is what a later upgrade reconciliation reads
  help          Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc setup --help

```
Install the Claude Code adapter — writes the `.jigc/AGENT.md` bootstrap and a reference into `CLAUDE.md`, initializes the project layer (`.jigc/config/`), allowlists `Bash(jigc:*)`, and installs the `SessionStart` and warn-only git `pre-commit` hooks. Idempotent

Usage: jigc setup [OPTIONS]

Options:
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
```

## $ jigc setup   # in a fresh repo (git init + first commit)

```
jigc setup — adapter installed

jigc is now wired into this project; setup installed:
  - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
  - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
  - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
  - install commit → ba8b481   (setup's install files are committed on their own, off your first feature commit)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ cat .jigc/AGENT.md   # the installed adapter bootstrap, verbatim

```
`jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.

Managed docs are exactly the `jigc doc list` set; read one with `jigc doc show <doc>`. Everything else — source, tests, any file not in that set — you read freely.

`jigc` is a context compiler: it assembles the workflow steps for your task plus the doc slices that workflow declares (a quick fix may declare none), and owns every structural write — placement, cross-references, commits. You author only the prose.

Read every command's output; a non-zero exit means stop and follow what the output says — never retry blindly.
```

## $ jigc start   # bare — the orientation/catalog

```
jigc — orientation

Pack: dev/1.0.0-rc.7 | methodology/1.0.0-rc.7 · Project config: /home/maurice/.claude/jobs/0c0984fb/tmp/repoA/.jigc/config

Available workflows:
  - architecture-documentation — document the architecture of a part of the system, tying its components to the code that implements them
  - decided-task — implement one scoped change test-first, recording the design decision it makes
  - dev-task — implement one scoped change test-first, recording no decision
  - do-research — you need to investigate or gather evidence before forming a vision or making a decision
  - form-vision — you're forming or revising the project's vision from research
  - implement-from-spec — a committed spec already covers the intent, with acceptance criteria to build against
  - park-idea — a shaped-but-unscheduled idea occurs mid-work and is worth keeping
  - plan — draft the specification for upcoming work before writing any code
  - project-setup — bootstrap a brand-new project by developing the idea into its first product requirements
  - quick-fix — apply a small commit-only fix that touches no documented code and records no decision
  - single-task — implement one scoped change end-to-end, recording the decisions it makes

Run: `jigc start "<intent>"`   — presents the workflows above; pick one, then re-run with `--workflow <chosen>` to compose it
Run: `jigc start --workflow planning`   — plan a milestone — decompose it into increments and tasks
Run: `jigc start --workflow ingest-existing`   — bring an existing repo's docs under management
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc start "improve the error message when config is missing"   # the router compose

```
These are the selectable work-workflows, each with the situation it fits:

- architecture-documentation — document the architecture of a part of the system, tying its components to the code that implements them
- decided-task — implement one scoped change test-first, recording the design decision it makes
- dev-task — implement one scoped change test-first, recording no decision
- do-research — you need to investigate or gather evidence before forming a vision or making a decision
- form-vision — you're forming or revising the project's vision from research
- implement-from-spec — a committed spec already covers the intent, with acceptance criteria to build against
- park-idea — a shaped-but-unscheduled idea occurs mid-work and is worth keeping
- plan — draft the specification for upcoming work before writing any code
- project-setup — bootstrap a brand-new project by developing the idea into its first product requirements
- quick-fix — apply a small commit-only fix that touches no documented code and records no decision
- single-task — implement one scoped change end-to-end, recording the decisions it makes

Pick the workflow whose situation best fits the intent, then re-run with that
choice and the original intent:

jigc start --workflow <chosen> "<intent>"
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc describe

```
jigc describe — a tour of what this project lets you compose and author.

The workflows you can compose here. architecture-documentation is Authors living architecture documentation for one part of the system and commits it. Reach for it when a part of the system needs a durable, code-checked description so a later reader can orient without reverse-engineering it.

completion is The milestone-completion loop (audit → triage → fix → re-verify), with the human-gated triage and fix-round halts composed as structural Checkpoints, then authors the per-milestone completion-record (verdict + owner-artifact + findings) and appends the triage decisions to the running decisions-log. Reach for it when you are closing a milestone — auditing the assembled whole, triaging and fixing its findings, then recording the verdict and the genuine-audit owner-artifact — and you want the phase walk, its human gates, and the record authoring made visible. It is hidden from the router catalog: invoked by name when a milestone closes (`jigc start --workflow completion "<milestone>"`) — the completion audit follows the milestone lifecycle, not an intent the router disambiguates.

decided-task is The dev-workflow plus a decision-recording step — scope, implement test-first, run the gate, record the task's design decision on the running decisions log, then land one commit. The lightweight sub-milestone path for a task that decides something worth keeping, without milestone-planning ceremony. Reach for it when the work is one coherent change you can carry to a single commit AND it makes a design decision worth recording, and you want the test-first discipline plus a managed decision record without invoking milestone planning.

dev-task is The reduced-linear dev-workflow — scope the task, implement test-first, run the project's own gate, and land exactly one commit, all as one pass. Reach for it when the work is one coherent change you can carry from intent to a single commit, and you want the test-first discipline kept visible.

do-research is Investigate one question and record what it found — author a standalone research doc (question, findings, sources) and land it as one commit, so a later vision or decision can rest on committed evidence. Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and you want that investigation carried through the binary to one committed research record.

form-vision is Form or revise the project's vision from committed research — create the vision singleton, ground it in the research it rests on, and author its thesis, invariants, and open questions, then land it as one commit. A re-entry flow — set `grounded-in`, re-compose, then author against the grounding research now in view. Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, and you want it formed from and traceable to the committed research behind it.

implement-from-spec is An implementation pass driven by a committed spec — locate against it, implement, optionally record a decision, and commit. Reach for it when a spec already exists and you are turning its agreed criteria into code.

increment is The increment-workflow's outer loop (plan → execute → validate → fix), with its three human-gated halts composed as structural Checkpoint directives. Reach for it when you are working a whole roadmap increment, not a single task, and you want the plan/execute/validate/fix phases and their halt points made visible.

ingest-existing is An onboarding pass that scans an existing repo's documents and routes each to a verdict for jigc management. Reach for it when you are adopting jigc on a repo that already has docs and want to bring them under management rather than starting fresh.

migrate-adr is Migrate a foreign architectural decision record into a managed `adr` at `decisions/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant ADR (a MADR or Nygard file under `docs/adr/` or similar) and `jigc migrate <path> --as adr` is rewriting it into a managed `adr`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as adr`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-arch-doc is Migrate a foreign architecture document into a managed `arch-doc` at `architecture/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant architecture document (an arc42, a C4 model, or a README "Architecture" section under `docs/` or similar) and `jigc migrate <path> --as arch-doc` is rewriting it into a managed `arch-doc`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as arch-doc`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-changelog is Migrate a foreign `CHANGELOG.md` into the managed `changelog` singleton — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant `CHANGELOG.md` and `jigc migrate <path> --as changelog` is rewriting it into the managed changelog. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as changelog`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-completion-record is Migrate a foreign milestone-close record into a managed per-milestone `completion-record` at `completions/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs (resolving the required `owner-artifact` by the artifact ladder), and the task finalizes. Reach for it when an existing project carries a non-conformant milestone-close record (a GSD completion file, a "M3 done" write-up) and `jigc migrate <path> --as completion-record` is rewriting it into a managed completion record. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as completion-record`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-decisions-log is Migrate a foreign decisions log into the managed `decisions-log` singleton at `docs/decisions-log.md` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant decisions log (a DECISIONS.md, a running "choices we made" file) and `jigc migrate <path> --as decisions-log` is rewriting it into the managed decisions log. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as decisions-log`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-deferral-ledger is Migrate a foreign record of deferred decisions and parked ideas into the managed `deferral-ledger` singleton at `docs/deferral-ledger.md` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant owed-and-when record (a deferred-decisions list, a "later" backlog kept as prose) and `jigc migrate <path> --as deferral-ledger` is rewriting it into the managed deferral ledger. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as deferral-ledger`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-idea is Migrate a foreign parked-idea note into a managed `idea` at `ideas/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant parked-idea note (a someday/maybe file, a backlog entry kept as prose) and `jigc migrate <path> --as idea` is rewriting it into a managed idea. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as idea`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-prd is Migrate a foreign product requirements document into a managed `prd` at `prds/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant PRD (a free-form vision-and-requirements file under `docs/` or similar) and `jigc migrate <path> --as prd` is rewriting it into a managed `prd`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as prd`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-research is Migrate a foreign research or investigation note into a managed `research` doc at `research/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant research note (loose investigation notes, an evidence dump under `research/` or similar) and `jigc migrate <path> --as research` is rewriting it into a managed research doc. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as research`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-roadmap is Migrate a foreign roadmap or milestone plan into the managed `roadmap` singleton at `docs/roadmap.md` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant roadmap (a milestone plan, a phase list, a GSD ROADMAP.md) and `jigc migrate <path> --as roadmap` is rewriting it into the managed roadmap. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as roadmap`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-spec is Migrate a foreign specification into a managed `spec` at `specs/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant spec (a free-form requirements or acceptance-criteria file under `docs/specs/` or similar) and `jigc migrate <path> --as spec` is rewriting it into a managed `spec`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as spec`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-vision is Migrate a foreign vision/charter document into the managed `vision` singleton at the repo-root `VISION.md` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant vision document (a free-form charter/thesis/direction file) and `jigc migrate <path> --as vision` is rewriting it into the managed vision. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as vision`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

milestone-execution is A parallel execution of a planned milestone's sub-tasks, joined and committed as one boundary. Reach for it when a milestone is decomposed into independent sub-tasks you want run together and landed as a single commit.

park-idea is Park one shaped-but-unscheduled direction — author a standalone idea doc (the direction and the trigger that would bring it back) and land it as one commit, so a mid-work thought has a durable home cheaper than losing it. Reach for it when a direction is worth keeping but not worth scheduling now, and you want it carried through the binary to one committed idea record with the trigger that would revisit it.

plan is A planning pass that authors the specification for upcoming work and commits it. Reach for it when the work needs its goal and acceptance criteria agreed up front, before any code is written.

planning is The milestone-planning loop (scope → detect-gaps → settle → review → decompose), with the human-gated Settle composed as a structural Checkpoint, then authors the three running working-docs (roadmap / deferral-ledger / decisions-log) the loop maintains. Reach for it when you are opening a milestone — scoping it against the roadmap, settling its gaps, and cutting it into increments — and you want the phase walk, its human gate, and the running-doc authoring made visible. It is hidden from the router catalog: invoked by name with the milestone in hand (`jigc start --workflow planning "<milestone>"`) — opening a milestone is a deliberate lifecycle act, not an intent the router disambiguates.

project-setup is A bootstrap pass that develops a project idea into its first product requirements document and commits it. Reach for it when you are at the very start of a brand-new project, before any spec or code, and want to turn an idea into shared requirements.

quick-fix is A small commit-only fix — locate, implement, and commit, with no decision record. Reach for it when the change is a quick, low-stakes fix that needs nothing preserved beyond the commit itself, and changes no code a managed doc describes — if it renames or removes a documented symbol, the arch-doc anchoring it needs updating too, so pick single-task instead.

record-change is Record a change on the changelog — create-or-update the singleton, author a release (or a staged change-group) with its nested category groups, and commit. Reach for it when a user-facing change needs recording on the changelog — staged now, or cut into a versioned release. It is hidden from the router catalog: reached by name for the deliberate record-a-change/cut-a-release pass (`jigc start --workflow record-change`); routine change recording already rides `single-task`'s record-changelog step, so a catalog line would duplicate it.

record-dogfood is The measured-dogfood recording spine — transcribes a completed measured run's organic fact counts and seeded instrument checks from the committed capture into a per-run dogfood-record, with the judged verdict and the owner-artifact holding the raw capture, then finalizes. Reach for it when a measured dogfood run has completed and its capture (transcript + raw hook log/tally + manifest) is in hand — the run's facts, instrument checks, verdict, and capture artifact need a durable record authored through the binary. It is hidden from the router catalog: invoked by name once a measured run's capture is in hand (`jigc start --workflow record-dogfood "<run>"`) — the recording follows the dogfood protocol, not an intent the router disambiguates.

router is A selection pass that presents the available work-workflows and routes an intent to the right one. Reach for it when you have an intent but are unsure which work-workflow fits, and want the catalog to choose from.

single-task is An end-to-end scoped change — locate, implement, optionally record a decision, and commit, all as one task. Reach for it when the work is one coherent change you can hold in your head and carry from intent to commit in a single pass.

sub-task is One sub-task of a milestone — locate, implement, and author a commit into an isolated working area for a later join. Reach for it when a milestone execution needs the unit it fans out to; it is not selected directly. It is hidden from the router catalog: spawned by a milestone execution's fan-out and joined at the parent's finalize — it has no commit boundary of its own, so a router pick could never land.

The doc-types you can author. adr is A dated architectural decision record, capturing the context a choice was made in, the choice itself, and its consequences, with an optional link to the decision it supersedes. Reach for it when a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

arch-doc is Living architecture documentation for one part of the system — its overview, its components and the code that implements them, and the decisions behind it. Reach for it when a part of the system is worth a durable description that stays honest against the code, so a later reader (or agent) can orient without reverse-engineering it.

changelog is A Keep-a-Changelog singleton — staged unreleased changes plus the cut releases, each grouped by category, maintained over the life of the project. Reach for it when a user-facing change lands and the project keeps a human-readable record of what changed, staged now and cut into versioned releases over time.

commit is A Conventional-Commits message — a typed, scoped header over a subject line, an optional body, and trailers — rendered into the git commit rather than persisted as a repo file. Reach for it when you need to record what a change does and why at the moment it lands, in the form git and reviewers already read.

completion-record is The per-milestone completion record — the audit verdict, the owner-artifact the genuine audit produced, and each triaged finding with its disposition. Reach for it when a milestone reaches its completion audit, so its verdict, the audit's owner-artifact, and the triaged findings need a durable per-milestone record.

decisions-log is The running log of decisions made — each with the date it was settled and the reasoning behind it. Reach for it when the planning loop reaches Settle (or completion reaches triage) and a decision is made worth preserving with its rationale, dated, on the record.

deferral-ledger is The running ledger of deferred decisions and parked ideas, each keyed to the milestone trigger that resurfaces it. Reach for it when the planning loop reaches Settle and a decision is deferred or an idea parked rather than made now, so it needs a durable owed-and-when record.

dogfood-record is The per-run measured-dogfood record — the run's case and pinned binary, the transcribed organic fact counts, the seeded instrument checks, the judged verdict, and the owner-artifact holding the raw capture. Reach for it when a measured dogfood run completes, so its transcribed facts, seeded instrument checks, verdict, and capture artifact need a durable per-run record.

idea is One shaped-but-unscheduled direction, with the trigger that would bring it back. Reach for it when a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it.

milestone-record is The per-milestone team-ready state record — the shared base-SHA pin, the ordered sub-task list, and each sub-task's intent and status, resumable from a fresh clone. Reach for it when a milestone is executing and its in-flight state (base, sub-tasks, statuses) must be committed and legible so a teammate or fresh session can continue it, not stranded in the gitignored workbench.

prd is A product requirements document — the vision, the requirements, and the context for a piece of work, the first managed document a fresh project develops its idea into. Reach for it when you are at the start of a new project and want to turn an idea into a shared statement of what to build and why, before any spec or code.

research is One investigation and what it found — the evidence a vision or design is formed from. Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds.

roadmap is The running milestone spine — each milestone with what it proves and the prose decomposition of its increments. Reach for it when the planning loop reaches Decompose and a milestone's breakdown is worth recording on the durable roadmap.

spec is A specification of what a task implements — its goal, its context, and a set of testably-phrased acceptance criteria. Reach for it when you need to pin down what a piece of work must deliver before building it, so the criteria are agreed up front and code can be checked against them.

vision is The project's charter — its thesis, the invariants it holds, and its open questions — grounded in the research it was formed from. Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, formed from and traceable to the research behind it.

And the commands jigc hands you along the way. bind-spec Bind the spec this work implements to the task's spec role. create-adr Create a new ADR in the current task. create-arch-doc Create a new arch-doc in the current task. create-changelog Create-or-update the running changelog singleton in the current task. create-prd Create a new prd in the current task. create-spec Create a new spec in the current task. finalize-task Run validate + commit. One task → one commit. milestone-finalize The milestone commit boundary — validate the merged join + commit per the squash knob. milestone-provision Provision one detached base-pin worktree per sub-task before the fan-out. recompose-task Re-compose the task to pick up the freshly bound slice. run-ingest Scan the repo, classify candidate docs, and report the triage verdicts. set-commit-summary Stage the commit summary slot from stdin. set-commit-type Set the required Conventional-Commits type. validate-task Preview validation findings without committing.

— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc doc schema adr

```
doctype: adr (schema-version 2)
fields (* = author-required):
  - status: enum [proposed|accepted|superseded] (section: status) (default: proposed) (set-field: adr:<slug>#status/status)
  - date: date (section: status) (set: on-create)
  - supersedes: ref (section: status) (set-field: adr:<slug>#status/supersedes)
  - cites-code: code-anchor (section: status) (set-field: adr:<slug>#status/cites-code)
  - schema-version: int (section: status) (set: schema-version)
sections:
  - context: slot (set-slot: adr:<slug>#context)
  - options: slot (optional) (set-slot: adr:<slug>#options)
  - decision: slot (set-slot: adr:<slug>#decision)
  - consequences: slot (set-slot: adr:<slug>#consequences)
```

## $ jigc doc schema commit

```
doctype: commit (schema-version 1)
fields (* = author-required):
  - type: enum [feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert] (section: header) (set-field: commit:<slug>#header/type) *
  - scope: string (section: header) (set-field: commit:<slug>#header/scope)
  - implements: ref (section: header) (set-field: commit:<slug>#header/implements)
sections:
  - summary: slot (set-slot: commit:<slug>#summary)
  - body: slot (optional) (set-slot: commit:<slug>#body)
  - trailers: repeatable (add-item: commit:<slug>#trailers)
    - key: string *
    - value: string (set-field: commit:<slug>#trailers/<id>/value) *
```

## $ jigc doc list   # on the empty store

```
jigc doc list — no committed docs
```

## $ jigc start --workflow single-task "add a rate limiter"   # full composed output

```
task minted: add-a-rate-limiter

Reason about the change. The intent is:
add a rate limiter

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged. When done, set the
required Conventional-Commits type — your editorial call on what this change
does — then stage the summary prose:

Run: `jigc doc set-field commit:add-a-rate-limiter#type --value <COMMIT_TYPE> --task add-a-rate-limiter`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:add-a-rate-limiter#summary --from-file - --task add-a-rate-limiter`
<<author: commit:add-a-rate-limiter#summary>>

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:add-a-rate-limiter#scope --value <area> --task add-a-rate-limiter
jigc doc set-slot commit:add-a-rate-limiter#body --from-file - --task add-a-rate-limiter

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:add-a-rate-limiter#trailers --title Co-Authored-By --task add-a-rate-limiter
jigc doc set-field commit:add-a-rate-limiter#trailers/<id>/value --value "Name <email>" --task add-a-rate-limiter

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task add-a-rate-limiter`

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects):

jigc doc set-slot adr:<slug>#context --from-file - --task add-a-rate-limiter
jigc doc set-slot adr:<slug>#decision --from-file - --task add-a-rate-limiter
jigc doc set-slot adr:<slug>#consequences --from-file - --task add-a-rate-limiter

The `options` slot is optional — fill it only when alternatives were genuinely
weighed; omit it when the call was obvious:

jigc doc set-slot adr:<slug>#options --from-file - --task add-a-rate-limiter

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

If the change is user-facing — a feature, a fix, or a behaviour a user would
notice — record it on the changelog:

jigc doc create changelog --title Changelog --task add-a-rate-limiter

If your decision supersedes an earlier one, set `supersedes` on the ADR; the
superseded decision then appears below for reference, so your consequences can
explain what changes (nothing appears if it supersedes none).

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate add-a-rate-limiter` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize add-a-rate-limiter`
resume: `jigc start --task add-a-rate-limiter`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-a-rate-limiter`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task add-a-rate-limiter` is the explicit override and wins when several are active
create-gates: adr, changelog
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc start --workflow dev-task "add request logging to the API"   # in a fresh repo

```
task minted: add-request-logging

Scope the task before touching code. The intent is:

add request logging to the API

Restate that intent in your own words, then name the single observable
done-criterion — a test, a command that exits cleanly, or a behaviour you can
point at. If your restatement reveals a different problem than the intent
asked for, stop and check with the human before proceeding: a clarifying
question costs less than solving the wrong problem.

Implement the change test-first. Write the failing test first and confirm it
fails for the right reason — the assertion you care about, not an incidental
compile error standing in for it. Only then write the minimal implementation
that makes it pass. Refactor while green, touching only what this task needs.

`git add` your code edits as you work — test and implementation both. finalize
commits only what you have staged, so anything you leave unstaged is silently
left out of the commit.

This ordering is yours to police: nothing here enforces that the test was
observed failing before the implementation. Hold the discipline yourself.

Run your project's own test, lint, and build gate — the commands this project
already uses to prove a change is sound — and confirm every one passes before
you finalize. Use whatever the project's configured gate is; do not assume a
particular toolchain. A green gate is what separates a finished change from one
that merely compiles in your head.

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:add-request-logging#type --value <TYPE> --task add-request-logging`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:add-request-logging#scope --value <SCOPE> --task add-request-logging`

Set the subject line:

Run: `jigc doc set-slot commit:add-request-logging#summary --from-file - --task add-request-logging`

Set the body — why this change:

Run: `jigc doc set-slot commit:add-request-logging#body --from-file - --task add-request-logging`

To see what's left before committing, run `jigc task validate add-request-logging` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize add-request-logging`
resume: `jigc start --task add-request-logging`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-request-logging`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task add-request-logging` is the explicit override and wins when several are active
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc start --workflow decided-task "switch the cache to LRU eviction"   # in a fresh repo

```
task minted: switch-the-cache-to-lru

Scope the task before touching code. The intent is:

switch the cache to LRU eviction

Restate that intent in your own words, then name the single observable
done-criterion — a test, a command that exits cleanly, or a behaviour you can
point at. If your restatement reveals a different problem than the intent
asked for, stop and check with the human before proceeding: a clarifying
question costs less than solving the wrong problem.

Implement the change test-first. Write the failing test first and confirm it
fails for the right reason — the assertion you care about, not an incidental
compile error standing in for it. Only then write the minimal implementation
that makes it pass. Refactor while green, touching only what this task needs.

`git add` your code edits as you work — test and implementation both. finalize
commits only what you have staged, so anything you leave unstaged is silently
left out of the commit.

This ordering is yours to police: nothing here enforces that the test was
observed failing before the implementation. Hold the discipline yourself.

Run your project's own test, lint, and build gate — the commands this project
already uses to prove a change is sound — and confirm every one passes before
you finalize. Use whatever the project's configured gate is; do not assume a
particular toolchain. A green gate is what separates a finished change from one
that merely compiles in your head.

If this task made a design decision worth keeping — a choice with a rationale a
future reader would otherwise have to reverse-engineer — record it on the running
decisions log. (A task that decided nothing of consequence skips this step.)
Create-or-update the log singleton — safe whether or not it already exists (an
existing committed log is copied in for append):

Run: `jigc doc create decisions-log --title Decisions-Log --task switch-the-cache-to-lru`

Mint one entry per decision and author its `why` — the reasoning, in a sentence or
two. The `date` is CLI-set on create:

jigc doc add-item decisions-log:decisions-log#entries --title "<the decision>" --task switch-the-cache-to-lru
jigc doc set-slot decisions-log:decisions-log#entries/<id>/why --from-file - --task switch-the-cache-to-lru

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-entry verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write. Over an already-committed log it copies the committed doc in and appends —
existing entries are untouched:

jigc doc author decisions-log --from-file - --task switch-the-cache-to-lru

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:switch-the-cache-to-lru#type --value <TYPE> --task switch-the-cache-to-lru`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:switch-the-cache-to-lru#scope --value <SCOPE> --task switch-the-cache-to-lru`

Set the subject line:

Run: `jigc doc set-slot commit:switch-the-cache-to-lru#summary --from-file - --task switch-the-cache-to-lru`

Set the body — why this change:

Run: `jigc doc set-slot commit:switch-the-cache-to-lru#body --from-file - --task switch-the-cache-to-lru`

To see what's left before committing, run `jigc task validate switch-the-cache-to-lru` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize switch-the-cache-to-lru`
resume: `jigc start --task switch-the-cache-to-lru`   — re-composes this workflow if context is lost
what's-left: `jigc task validate switch-the-cache-to-lru`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task switch-the-cache-to-lru` is the explicit override and wins when several are active
create-gates: decisions-log
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc start --workflow quick-fix "fix the typo in the usage banner"   # in a fresh repo

```
task minted: fix-the-typo

Reason about the change. The intent is:
fix the typo in the usage banner

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged. When done, set the
required Conventional-Commits type — your editorial call on what this change
does — then stage the summary prose (`scope` and `body` are optional):

Run: `jigc doc set-field commit:fix-the-typo#type --value <COMMIT_TYPE> --task fix-the-typo`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:fix-the-typo#summary --from-file - --task fix-the-typo`
<<author: commit:fix-the-typo#summary>>

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate fix-the-typo` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize fix-the-typo`
resume: `jigc start --task fix-the-typo`   — re-composes this workflow if context is lost
what's-left: `jigc task validate fix-the-typo`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task fix-the-typo` is the explicit override and wins when several are active
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc migrate docs/adr/0003-use-postgres.md --as adr   # foreign Nygard-style ADR, committed

```
task minted: migrate-adr-docs-adr-0003

Migrate the foreign ADR into a managed `adr`. Below is the foreign source the CLI
staged for you (read-only context — you author the canonical record through the write
verbs, you never edit this file or place anything yourself):

# 3. Use PostgreSQL for the primary datastore

Date: 2025-11-02

## Status

Accepted

## Context

We need a relational database for the order pipeline. The team already
operates MySQL for the legacy billing system, but its JSON support is weak
and our new schema leans on JSONB-style columns and partial indexes.

## Decision

We will use PostgreSQL 16 as the primary datastore for all new services.

## Consequences

Operations must learn Postgres backup tooling. The legacy billing system
stays on MySQL until its planned retirement; the two will coexist for at
least a year. New hires can be onboarded against one modern SQL dialect.


The target schema and its batch payload, both generated from the resolved `adr`
schema, follow — the CLI creates the record and places every field and prose slot
over a single staged buffer:

The `adr` schema — each instance a managed file at `docs/decisions/<slug>.md`, its `<slug>` minted from `title`.

- `status` (front-matter fields):
    - `status`: enum, one of: proposed | accepted | superseded — defaults to `proposed` unless authored
    - `date`: date — CLI-stamped (on-create) unless authored
    - `supersedes`: ref -> adr (0..*) — optional
    - `cites-code`: code-anchor — optional; adjudicated at finalize
    - `schema-version`: int — CLI-stamped (schema-version) unless authored
- `context`: prose slot — Why a decision was needed — the forces at play.
- `options`: prose slot (optional) — Alternatives weighed and why they lost — omit when the call was obvious.
- `decision`: prose slot — What we decided, in a sentence or two.
- `consequences`: prose slot — Tradeoffs and follow-on effects.

Author the whole document in ONE `jigc doc author` batch payload — fill each `<…>` value. The `<<…>>` wrapping on slot prose is REQUIRED literal syntax: keep the `<<`/`>>` markers and replace only the text between them (an inline field takes a bare value — wrapping one is rejected). Inside slot prose, headings must sit at `####` depth or deeper — `##`/`###` are schema-reserved, and Setext headings are rejected. An entry marked `# optional` may be omitted entirely. Pipe the payload on stdin:

jigc doc author adr --from-file - --task migrate-adr-docs-adr-0003 <<'EOF'
title: "<the title>" # the id-source — slugged lowercase-kebab into the doc id, capped at the first 5 words / 50 chars
sections:
  - id: context
    set:
      context: |-
        <<Why a decision was needed — the forces at play.>>
  - id: options # optional — omit this entry if unused
    set:
      options: |-
        <<Alternatives weighed and why they lost — omit when the call was obvious.>>
  - id: decision
    set:
      decision: |-
        <<What we decided, in a sentence or two.>>
  - id: consequences
    set:
      consequences: |-
        <<Tradeoffs and follow-on effects.>>
EOF

Map the foreign ADR's headings onto the schema above:

  - the foreign Status / state line maps to the `status` field (see the enum map below);
  - the foreign Context / Background / Problem heading maps to the `context` slot;
  - the foreign Options / Alternatives / Considered-options heading maps to the
    optional `options` slot (omit the entry when the foreign source weighed none);
  - the foreign Decision heading maps to the `decision` slot;
  - the foreign Consequences / Outcome / Tradeoffs heading maps to the `consequences` slot.

The `status` field is authored under the `status` section's `set:` block. Map the
foreign status onto the nearest enum member — `Proposed`/`Draft` to proposed,
`Accepted` to accepted, `Superseded`/`Deprecated`/`Rejected` to superseded — and record
the foreign status word verbatim in the `context` prose when it carries nuance the
enum drops (`Accepted (revised)`, `Rejected`). A value outside the enum is rejected at
the write:

  - id: status
    set:
      status: "<the mapped member>"

The `date` records WHEN the decision was adopted. TRANSCRIBE the foreign source's date
(its `Date:` line or dated heading) verbatim into the `status` section's `set:` block as
`date: "<YYYY-MM-DD>"` — an authored date overwrites the CLI's on-create stamp and
survives the migration. OMIT `date` ONLY when the foreign source carries no date: in
migration the CLI does NOT fabricate today's date, so a dateless source stays dateless
(no false decision history).

SUPERSEDES edges — wire only an IN-SET supersession. When the foreign ADR supersedes a
prior decision you ARE also migrating in this pass, author the edge: add a `supersedes`
field to the `status` section's `set:` block as a typed bracket-list of the target slug(s) —

  - id: status
    set:
      status: "<...>"
      supersedes: "[adr:<slug-of-the-superseded-decision>, …]"

Each element is `adr:<slug>` (the per-title slug the target migrated under). A bare slug,
a wrong-type ref, or the unbracketed comma form is rejected at the write.

Dependency-ordering contract: a `supersedes` forward-ref resolves against the COMMITTED
store. So migrate and finalize the superseded target ADR FIRST,
before the ADR that supersedes it — otherwise the superseding ADR's finalize blocks on an
unresolved ref.

Out-of-set drop: a `supersedes` whose target is NOT (and will not be) a migrated ADR — it
predates the corpus, or is deliberately left foreign — has no edge to resolve and would
block finalize forever. Fold that reference into the `context` or `consequences` prose
instead; never author it as a dangling `supersedes` ref. The same drop-to-prose posture
applies to "Related ADRs", "See also", and plain links: only an in-set supersession
becomes a structural edge. (The `cites-code` field is a separate cross-code pass.)

Finalizing a migration adds a review hold: a plain finalize commits NOTHING —
it renders the foreign source against the canonical rewrite and holds (exit 4),
because the CLI guarantees structure, never content-faithfulness. Review that
fidelity diff, then re-run the same finalize with `--approve` to write the
canonical doc, RETIRE the foreign original, and commit — approval is the one
destructive gate. A source already sitting at its canonical destination is
rewritten in place on approval — the committed file is modified, nothing is
retired. A destination already committed when the doc was authored was copied
in as the edit base (the create said so), so approval updates that committed
file in place — review the fidelity diff before approving. Finalize blocks
(finalize.promote-clobber) only when a file appeared at the destination after
the doc was created: resolve that collision, or retitle the doc so it slugs
differently, then finalize again.
Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate migrate-adr-docs-adr-0003` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize migrate-adr-docs-adr-0003`
resume: `jigc start --task migrate-adr-docs-adr-0003`   — re-composes this workflow if context is lost
what's-left: `jigc task validate migrate-adr-docs-adr-0003`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task migrate-adr-docs-adr-0003` is the explicit override and wins when several are active
create-gates: adr
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc doc create adr --title "Use SQLite"   # in the single-task repo, task inferred

```
adr:use-sqlite
```

## $ jigc doc set-field commit:add-a-rate-limiter#type --value feat --task add-a-rate-limiter

```
set commit:add-a-rate-limiter#type = feat
```

## $ jigc doc set-field commit:add-a-rate-limiter#type --value bogus --task add-a-rate-limiter   # wrong enum value

```
blocking · write.malformed-value — write rejected: "bogus" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  route: `jigc doc schema <doctype>` to see the field's declared type and members, then re-run the write with a conformant value
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 1)

## $ echo "some prose" | jigc doc set-slot adr:use-sqlite#context --from-file - --task add-a-rate-limiter

```
set slot adr:use-sqlite#context (11 chars)
```

## $ jigc task validate add-a-rate-limiter   # while the commit doc is incomplete

```
advisory · file-state.staged-copy — staged copy of `docs/decisions/use-sqlite.md` — this task's in-flight version of the doc
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
blocking · schema-conformance.required-slot-present — `docs/decisions/use-sqlite.md`: required slot in section `decision` is empty
  route: `jigc doc set-slot <address> --from-file -` to fill the empty slot (this finding's target is the address)
blocking · schema-conformance.required-slot-present — `docs/decisions/use-sqlite.md`: required slot in section `consequences` is empty
  route: `jigc doc set-slot <address> --from-file -` to fill the empty slot (this finding's target is the address)
blocking · schema-conformance.required-slot-present — `commit:add-a-rate-limiter`: required slot in section `summary` is empty
  route: `jigc doc set-slot <address> --from-file -` to fill the empty slot (this finding's target is the address)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc task finalize add-a-rate-limiter   # a finalize attempt that blocks

```
blocking · schema-conformance.required-slot-present — `docs/decisions/use-sqlite.md`: required slot in section `decision` is empty
  route: `jigc doc set-slot <address> --from-file -` to fill the empty slot (this finding's target is the address)
blocking · schema-conformance.required-slot-present — `docs/decisions/use-sqlite.md`: required slot in section `consequences` is empty
  route: `jigc doc set-slot <address> --from-file -` to fill the empty slot (this finding's target is the address)
blocking · schema-conformance.required-slot-present — `commit:add-a-rate-limiter`: required slot in section `summary` is empty
  route: `jigc doc set-slot <address> --from-file -` to fill the empty slot (this finding's target is the address)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc doc show adr:use-sqlite   # task-less read of a doc staged in an open task

```
blocking · store.not-found — could not read `adr:use-sqlite` at `/home/maurice/.claude/jobs/0c0984fb/tmp/repoA/docs/decisions/use-sqlite.md`: No such file or directory (os error 2)
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show adr:use-sqlite --task <task-id>`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 1)

## $ jigc doc show nonexistent:thing

```
blocking · store.unknown-type — unknown doctype `nonexistent` for `nonexistent:thing`
  route: list the available doctypes with `jigc describe`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 1)

## $ jigc task finalize no-such-task

```
no task `no-such-task` — list live tasks with `jigc task list`
```

(exit code: 1)

## $ jigc task status add-a-rate-limiter   # unknown subcommand

```
error: unrecognized subcommand 'status'

Usage: jigc task [OPTIONS] <COMMAND>

For more information, try '--help'.
tip: `jigc task list` enumerates the active tasks (id + minting workflow + intent); `jigc task validate <task-id>` previews the finalize gate for one task — what still blocks
```

(exit code: 2)

## $ jigc task finalize add-a-rate-limiter   # after completing the docs minimally — the success manifest

```
advisory · file-state.staged-copy — staged copy of `docs/decisions/use-sqlite.md` — this task's in-flight version of the doc
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it — `jigc doc create changelog --title Changelog --task add-a-rate-limiter`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task add-a-rate-limiter` (after a landed commit: `jigc start --workflow record-change "<what changed>"`); if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized 0f897e8 — feat: add a rate limiter
  promoted docs/decisions/use-sqlite.md
  1 file committed
```

## $ jigc task finalize fix-a-broken-link   # with an unrelated file pre-staged before the task was minted

```
blocking · finalize.carried-staged — `notes.txt` was already staged before this task existed — refusing to let a pre-task staged change silently ride this task's commit
  route: unstage it (`git restore --staged -- notes.txt`) if it is not this task's work, or re-run the finalize with `--carry-staged` to declare the carry-over deliberate
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc task finalize fix-a-broken-link --carry-staged   # the carry-staged landing

```
finalize — committing the index; carrying over (staged before this task existed — declared with `--carry-staged`):
  carried-over notes.txt
no findings — the task validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized c986d52 — docs: docs: fix the readme link
  carried-over notes.txt
  1 file committed
```

## Capture notes

- Binary: `~/.local/bin/jigc` — version confirmed `jigc 1.0.0-rc.7` (first capture).
- 49 captures, all verbatim stdout+stderr; non-zero exit codes noted per capture. No command failed to capture.
- Repos used (all throwaway, under `/home/maurice/.claude/jobs/0c0984fb/tmp/`): `repoA` (setup / orientation / router / single-task / acks / findings / finalize / misuse), `repoB-dev`, `repoC-decided`, `repoD-quickfix` (one compose each), `repoE-migrate` (foreign Nygard ADR migrate compose), `repoF-staged` (carried-staged refuse + `--carry-staged` landing).
- The `docs: docs:` doubling in repoF's finalized subject line comes from the capture operator's own summary input ("docs: fix the readme link") combined with the `type` field — operator input, recorded verbatim.
