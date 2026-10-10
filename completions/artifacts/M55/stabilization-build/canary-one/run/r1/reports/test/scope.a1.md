# canary-one, round 1, test stage - the scope step's report (attempt 1)

Status: **written**. The round's scope is on record at `completions/artifacts/canary-one/r1/scope.md`:
1 door included, 47 excluded, one registry, read as a whole list of 48 rows. 1 + 47 = 48, the
registry's own count.

## What was asked

- Run `canary-one`, round 1. Base `91834b5e011de2c36e2be2b79e96c0b9f60a803c` (the previous release,
  1.0.0-rc.24, as the run's record names it); tip `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` (label c1).
- The scope the stage was started with: **named doors, set by the human: `jigc doc list`.**
- No earlier round, so no fixer's or auditor's door list was an input.
- The units are the five rows of `completions/artifacts/canary-one/test-set.md`.

A scope the human set is the round's test set. The named door is included as named, every other door
of the registry it comes from is excluded as *outside this round's scope, as set*, and the doors my own
derivation reaches that the set scope leaves out are returned as `reached_but_excluded`. The scope was
neither narrowed nor widened.

## The two commits

    $ git rev-parse --verify 91834b5e011de2c36e2be2b79e96c0b9f60a803c^{commit}     -> resolves, exit 0
    $ git rev-parse --verify eeffe347324f83a51d1ae83d5f254e73c3f1ea3a^{commit}     -> resolves, exit 0
    $ git merge-base --is-ancestor 91834b5e... eeffe347...                          -> exit 0
    $ git rev-list --count 91834b5e..eeffe347                                       -> 145
    $ git rev-parse HEAD                                                            -> eeffe347... (branch fix/canary-one)
    $ git status --short                                                            -> only the untracked run directory r1/

The working tree is the tip for every tracked file, so a read of a file is a read at the tip.

## The registry

Neither the prompt nor the opening record names a registry, and no row of the test set names one (the
`registries` cell of all five rows is `-`). The named door is a leaf verb, so it was placed as a row of
the one registry of leaf verbs the code holds:

**`VERB_KINDS`, `crates/cli/src/cli.rs:2033`** - *"Every leaf verb's argv path with its VerbKind - total
over the clap tree by fence"*. The fence is `cli_parse::every_leaf_verb_is_classified` (same file), a
bijection with the clap tree's leaves, and it renders a row as `jigc ` + `path.join(" ")` - so
`jigc doc list` is the registry's own wording of the row `(&["doc", "list"], VerbKind::Read)`, and it is
also, letter for letter, the door of the test-set row `row-doc-list` and of the ledger row
`canary-seeded-claim`.

The registry's text in the scope is `VERB_KINDS - the leaf-verb registry, crates/cli/src/cli.rs`. It is
new with this round: a later round that names the same registry uses the same text.

Count, read off the symbol at both commits:

    $ git show <tip>:crates/cli/src/cli.rs  | awk '/^pub const VERB_KINDS:/{p=1;next} p&&/^\];/{exit} p&&/VerbKind::/' | wc -l   -> 48
    $ git show <base>:crates/cli/src/cli.rs | (the same)                                                                         -> 48
    $ diff of the two row lists                                                                                                  -> no difference, exit 0
    12 rows are VerbKind::Read, 36 are VerbKind::Write; 48 distinct door texts.

Rows 1-13 are the top level (lines 2035-2047), 14-24 `doc` (2049-2059), 25-31 `task` (2061-2067), 32-39
`config` (2069-2076), 40-48 `milestone` (2078-2086).

## The change, and what it reaches

    $ git diff --name-only <base>..<tip>                                     -> 371 files
    $ git diff --name-only <base>..<tip> -- crates Cargo.toml Cargo.lock rust-toolchain.toml   -> 115 files
        crates/cli/src 20 · crates/engine/src 9 · crates/cli/tests 83 · crates/cli/guides 2 · crates/cli/Cargo.toml 1
    $ git diff --stat <base>..<tip> -- crates/cli/src crates/engine/src crates/cli/Cargo.toml crates/cli/guides
        32 files changed, 18567 insertions(+), 1401 deletions(-)

### The named door, `jigc doc list` - reached directly

    $ git diff -U0 <base>..<tip> -- crates/cli/src/doc.rs | grep '^@@'      -> 1 hunk whose context is `fn run_list(`
        the hunk adds one line at the end of the handler: served_from_home_note(cwd, &jigc_home);
    $ git grep -c served_from_home_note <base> -- crates/                    -> no hit, exit 1  (the symbol is new)
    $ grep -n served_from_home_note crates/cli/src/*.rs                      -> 7 hits at the tip:
        cli.rs:1369 · doc.rs:4424 · doc.rs:4463 · doc.rs:4467 (the fn) · doc.rs:4469 · doc.rs:4975 (run_list) · render.rs:2450 (the pub(crate) fn)
    $ grep -n 'run_list(' crates/cli/src/doc.rs                              -> the fn at 4801 and 1 caller, line 634, under `DocCommand::List { doctype, task }`
    $ grep -n 'DocCommand::List' crates/cli/src/cli.rs                       -> cli.rs:564, `DocCommand::List { .. } => &["doc", "list"]` in `Command::leaf`

So the door the human named is one the change reaches by its own handler. The added line is the last
statement of the committed-store arm: it asks `crate::repo::discover_repo_root(cwd)` which checkout the
read was typed in and hands the answer to `render::served_from_home_note`. The staged arm (`--task`,
`run_list_staged`) returns before it, and so does every refusal above it in the handler.

The modules the handler calls into, and whether the change touched them (hunk contexts of
`git diff -U0`, which name the enclosing or the preceding item and are a pointer, not a proof):
`ingest.rs`, `orphan.rs`, `pack_builtin.rs`, engine `index.rs` and `parse.rs` - 0 of 5 changed;
`start.rs` - changed in `compose_core`, `compose_task_workflow`, `compose_minted_in_repo`,
`mint_migration_in_repo`, `Composition`; `pack.rs` - in `AMBUSH_CONTRACTS`; `rename.rs` - in
`RefusalKind`, `run`, `apply_and_commit`, `retitle_route`; `repo.rs` - 2 hunks, after
`operation_in_progress` and `git_common_dir_parent`; engine `validate.rs` - in `validate_task`,
`validate_store_families`, `schedule_doc_code`, `PriorHomeInstance` and two type aliases.

### Every door of the registry - reached, unbounded

    $ git diff --name-only <base>..<tip> -- crates/cli/src/main.rs           -> 1 file
    $ git diff -U0 <base>..<tip> -- crates/cli/src/main.rs | grep -c '^@@'   -> 4 hunks: one `use` line, three inside `fn main`
    $ grep -c '^fn main' crates/cli/src/main.rs                              -> 1
    $ grep -c 'cli.dispatch()' crates/cli/src/main.rs                        -> 1
    $ grep -c 'invocation_log::log_invocation(' crates/cli/src/main.rs       -> 1
    $ grep -c 'LogWrite::for_leaf(' crates/cli/src/main.rs                   -> 2  (the parsed command, and the node a rejected argv reached)
    $ git grep -c LogWrite <base> -- crates/                                 -> no hit, exit 1  (the type is new)
    arms of `Command::leaf` (cli.rs:538) that return a path                  -> 48

`fn main` is the one way into every leaf verb, and it changed: it now reads a `LogWrite` off the leaf of
the parsed command and passes it to `log_invocation`, whose signature changed with it. The consumers of
that change are every row of the registry and cannot be enumerated to a smaller set, so by the rule
*unbounded means inside* the derived delta reaches all 48 doors. Under the default scope the whole
registry would have been this round's test set.

Under the scope as set it is not. **47 doors are reached by the change and excluded by the set scope**;
each of the 47 says so in its derivation on record and all 47 are returned as `reached_but_excluded`.
None is excluded because its trace was too long.

## The two sides

Included (1):

| door | registry | row | kind |
|---|---|---|---|
| jigc doc list | VERB_KINDS | 24 of 48, cli.rs:2059 | Read |

Excluded (47), each *outside this round's scope, as set*, each reached by the change (unbounded, above):

- top level (13): jigc start · jigc workflow · jigc setup · jigc uninstall · jigc upgrade · jigc ingest ·
  jigc migrate · jigc migrate-corpus · jigc unmanage · jigc rename · jigc relocate · jigc describe ·
  jigc validate
- doc (10): jigc doc create · jigc doc add-item · jigc doc remove-item · jigc doc retitle-item ·
  jigc doc rename · jigc doc set-field · jigc doc set-slot · jigc doc author · jigc doc show ·
  jigc doc schema
- task (7): jigc task list · jigc task diff · jigc task validate · jigc task amend · jigc task discard ·
  jigc task finalize · jigc task bind
- config (8): jigc config set · jigc config insert-step · jigc config replace-step ·
  jigc config remove-step · jigc config fill · jigc config fork · jigc config get · jigc config list
- milestone (9): jigc milestone create · jigc milestone add-task · jigc milestone add-from-spec ·
  jigc milestone list-tasks · jigc milestone provision · jigc milestone execute · jigc milestone join ·
  jigc milestone finalize · jigc milestone discard

13 + 10 + 7 + 8 + 9 = 47, and 47 + 1 = 48.

The file was re-read against the registry before the script saw it: a check that parses the rows of
`VERB_KINDS` out of `crates/cli/src/cli.rs` and compares - 48 registry rows; 1 included, 47 excluded, 47
distinct; no door on both sides; the union equal to the registry's set, nothing missing and nothing that
is no row; every derivation's row number, line and kind equal to the registry's (0 mismatches); the
excluded side in the registry's own order.

## The write

    $ dev/stabilize-record scope-set --run canary-one --round 1 --scratch <scratch> --from <scratch>/scope/r1.a1.json
    {"scope": "completions/artifacts/canary-one/r1/scope.md", "included": 1, "excluded": 47}        exit 0

Called once. Read back afterwards:

    $ dev/stabilize-record state --run canary-one       -> doors: 1 included, 47 excluded; rounds[0].scope true; uncovered []; next test
    $ dev/stabilize-record item-doors --run canary-one --round 1 --item <each of the five>          -> [jigc doc list], exit 0, five times

## The resolved test set, by unit

| unit | kind | runs | how it reaches the round's door | doors |
|---|---|---|---|---|
| row-doc-list | review-row | in-scope | names the door by its text; the state now gives `selected: true` | jigc doc list |
| cross-cutting | audit-cross-cutting | every-candidate | names neither a door nor a registry, so it is handed all of the round's doors | jigc doc list |
| arm-control | trial-arm | every-candidate | the same rule | jigc doc list |
| gate | held-gate | every-candidate | the same rule; a held command, no agent hunts off it | jigc doc list |
| regression-set | held-regression | every-candidate | the same rule; a held command, no agent hunts off it | jigc doc list |

**Uncovered: none.** The one included door is named by `row-doc-list`, and the state's own `uncovered`
is empty.

The state's `never_selected` still lists `row-doc-list`. That list counts *tested* rounds only and
round 1 is not tested yet; the item's `selected` is true.

## Left open

Nothing here is a finding; each is something a reader of this scope should know.

1. **The registry is this step's choice.** No record of the run names one. A door renamed, or a registry
   renamed, between rounds is a different one to the script: later rounds use `jigc <path>` and
   `VERB_KINDS - the leaf-verb registry, crates/cli/src/cli.rs` as written here.
2. **Only the leaf-verb registry was read as a list of doors.** The code holds other registries - of
   committing doors, destroying doors, worktree doors, mint doors, finding codes and more. Their rows
   are on neither side of this scope, so a finding filed at one of them is `outside: unlisted`, which
   goes to the human exactly as `outside: excluded` does. Fifteen of those constants were compared
   between the two commits by their bodies. Thirteen are byte-identical: `BEHALF_DOORS`,
   `DOCTYPE_DOORS`, `WORK_UNIT_ID_DOORS`, `SLUG_DOORS` (cli.rs), `ROLLBACK_DOORS` (rollback.rs),
   `COMMITTING_DOORS`, `ERROR_CODE_REGISTRY` (invocation_log.rs), `BOUNDARY_DOORS` (render.rs),
   `DESTROYING_DOORS`, `WORKTREE_DOORS` (milestone.rs), `IGNORE_DOORS` (gitignore.rs), `MINT_DOORS`
   (engine state.rs), and `VERB_KINDS` itself. **Two changed:** `FINALIZE_FAMILY` (144 lines of body at
   the base, 155 at the tip) and `ENVELOPE_OWED_CODES` (6, then 12), both in `crates/cli/src/render.rs`.
   So the change adds rows to two registries of finding codes that this round, as set, does not look
   at. Their rows were not enumerated, and none of them is on a side.
3. **The control arm drives more than the round's door.** `completions/trial-driver/arms/walk/00-positive-control.sh`
   drives `jigc doc list --task` once (line 63, its output discarded) and beside it `setup`,
   `config set`, `start`, `doc create`, `doc set-slot`, `doc show`, `task diff` and `task list`. Anything
   it shows at one of those is at an excluded door, and is the human's.

## Repro

    cd <the clone>
    B=91834b5e011de2c36e2be2b79e96c0b9f60a803c; T=eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    git show "${T}:crates/cli/src/cli.rs" | awk '/^pub const VERB_KINDS:/{p=1;next} p&&/^\];/{exit} p&&/VerbKind::/' | wc -l    # 48
    git diff -U0 "$B..$T" -- crates/cli/src/doc.rs | grep '^@@' | grep -c 'fn run_list('                                  # 1
    git grep -c served_from_home_note "$B" -- crates/ ; echo $?                                                           # 1 (no hit)
    git diff -U0 "$B..$T" -- crates/cli/src/main.rs | grep -c '^@@'                                                       # 4
    grep -c 'cli.dispatch()' crates/cli/src/main.rs                                                                       # 1
    dev/stabilize-record state --run canary-one                                                                           # doors 1 / 47

<!-- end of report -->
