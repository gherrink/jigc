# row-doc-list — the source pass (run canary-one, round 1, test, attempt 1)

Reporter `row-doc-list-source`. Read-only for code: no edit, no commit. The one file written is this report.

## What was reviewed, and on what

- **Unit:** the review row of ONE door, `jigc doc list` — the path from the verb's dispatch in `crates/cli/src/cli.rs` to everything it reads, at the candidate's commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` (label c1). The brief's question: does a verb that only reads write, remove or commit anything at exit 0?
- **Binary driven:** the candidate handed on the `BINARY:` line, `<scratch>/bin/c1.a1/jigc`. Asserted before the first drive by the one spelled call `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc`, which printed `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the line gives. Its directory was put first on `PATH` and `command -v jigc` printed `<scratch>/bin/c1.a1/jigc`. Nothing was built; nothing under `target/` was driven.
- **Previous release:** `<scratch>/bin/previous-91834b5e011d/jigc`, asserted the same way: `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"`, the hash the line gives. Driven by absolute path, only to establish whether a behaviour is also the previous release's.
- **Scratch:** one directory of this reporter's own, minted with `mktemp -d` under the scratch root: `<scratch>/row-doc-list-source.uGNDJj`. Every rig and throwaway repository below lives under it; rigs were built with `SCRATCH=<that dir> dev/jigc-rig --binary <binary> <state>`. Every drive ran with a home directory that is not the machine's, `GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1` and a synthetic git identity passed through the `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables.
- **Evidence is keyed by commit and hash**, never by the version string: the candidate prints `1.0.0-rc.24` (visible in the log record under S3 below).

## Verdict

**On the door's own path there is no write to the working tree, to git or to a home directory, no removal and no commit — at exit 0 or at any other exit.** The source trace (next section) finds no write primitive below `DocCommand::List`, and the drives agree byte for byte: a whole-tree snapshot of the repository (`.git/` and `.jigc/` included), of the linked worktree where there was one, and of the home directory — kind, mode, size, sha256 and mtime of every entry — is identical around the verb in every row of the table under *The driven table*.

**One write is reachable from this door, and it is not on the door's path but in `fn main`:** with the opt-in `invocation-log` knob switched on, every invocation — this read verb included — creates `.jigc/logs/` and appends one record to `.jigc/logs/invocations.jsonl` (S1). It is designed, opt-in and gitignored, it destroys nothing and commits nothing, and `git status --porcelain` does not move. It is returned as a finding because the registry that classifies this door as `Read` states a rule the log is not named in.

**The seeded claim (`canary-seeded-claim`) is REFUTED on the candidate**, and on the previous release: the verb exits 0 and `notes.md` stands, same bytes (repro block V1).

Nothing found here breaks `no-lost-files` as this reporter reads the clause's scope. Five findings and two leads follow; three of the findings are the previous release's behaviour too.

## The path, as read

Every row was read in full at the cited lines. "Touches" is what the function does to the outside world; a function not listed was read and does nothing but compute.

| step | where | touches | writes |
|---|---|---|---|
| `fn main` | `crates/cli/src/main.rs:33-126` | `probe_intercept` (argv only, `:133-143`); `route_fence::install` (stores a function pointer; `route_fence.rs` holds no filesystem call outside its tests); `invocation_log::enabled_logs_dir` (`:45`); an fd-level tee over stdout and stderr **only when the knob is on** (`:46`, pipes, nothing on disk); `cli.dispatch()` (`:56`); `invocation_log::log_invocation` **only when the knob is on** (`:116-124`) | **the log, under the knob — S1** |
| `enabled_logs_dir` | `crates/cli/src/invocation_log.rs:535-559` | reads cwd, `$HOME`, `.jigc/config/`, the pack and the cascade; returns `None` unless the scalar `invocation-log` is `true` | none |
| `log_invocation` → `append_record` | `invocation_log.rs:487-521`, `:621-661` | `create_dir(logs_dir)` (`:638`), then `OpenOptions::new().create(mint).append(true).open(...)` (`:656-659`) and one `write_all` | **yes, under the knob** |
| `Cli::dispatch` | `crates/cli/src/cli.rs:822-908` | `refuse_on_posture` (`:642-658`) returns at once for this door: its `BEHALF_DOORS` row is `ActsOnBehalf::Neither` (`cli.rs:2332-2335`) | none |
| `run_doc` | `cli.rs:1809-1815` | reads the process cwd | none |
| `DocCommand::dispatch` → `run_list` | `crates/cli/src/doc.rs:568-659`, `:4801-4977` | see the rows below | none |
| `ingest::require_project_layer` | `crates/cli/src/ingest.rs:1017-1026` | `repo::jigc_home` (`crates/cli/src/repo.rs:91-102`): a walk-up for `.git` (`:118-123`) and, only where `.git` is a file, `git -C <root> rev-parse --path-format=absolute --git-common-dir` (`:127-142`); then `is_dir` on `.jigc/config` | none |
| `make_pack` and the pack-load assertions | `crates/cli/src/pack.rs:2049` onward; the production half of the file is lines 1-2587 | `std::fs::read`, `read_to_string`, `read_dir`, `std::env` — a scan of those 2587 lines for `fs::write`, `fs::remove*`, `fs::rename`, `fs::create*`, `fs::copy`, `OpenOptions`, `File::create`, `set_permissions`, `symlink`, `hard_link`, `Command::new` has no hit | none |
| `start::resolve_severity_cascade`, `CascadeDefs::{new, all_schemas, declared_schemas}` | `crates/cli/src/start.rs:2465-2471`, `:3924-3992`, `:4534-4680` | reads `.jigc/config/manifest.yaml` and the project shadows | none |
| `pack::frozen_doctype_versions`, `pack::prior_doctype_schemas`, `migrate_corpus::prior_homes` | `pack.rs:170`, `:205`; `crates/cli/src/migrate_corpus.rs:2358-2390` | pack reads | none |
| `orphan::prior_home_instances` | `crates/cli/src/orphan.rs:338-411` | `engine::index::committed_instances`, `std::fs::read` (`:394`, a failed read is skipped) | none |
| `engine::index::committed_instances` | `crates/engine/src/index.rs:789-817` | `path.exists()`, `read_dir` | none |
| the row loop | `doc.rs:4854-4903` | `std::fs::read(&path)…?` (`:4866-4867`), then pure parsing (`is_unadopted_foreign`, `parse_sections`, `read_h1`) | none — and see S2 |
| the orphan rows (unfiltered listing only) | `doc.rs:4922-4964`; `orphan.rs:204-263`, `:600-634`, `:144-159` | `git ls-files -z` in the store's home through `task::git_capture` (`crates/cli/src/task.rs:10087-10103`), `read_to_string`, `std::fs::read` (`doc.rs:4953`) | none |
| `render_listing` | `doc.rs:5086-5105` | stdout | none |
| `staged_listing_hint` | `doc.rs:5136` onward | `engine::state::list_active_task_ids` (`crates/engine/src/state.rs:1725-1737`, `read_dir` + `symlink_metadata`), `task::staged_doc_ids` (`task.rs:1215-1234`, `read_dir` + `metadata`); one stderr line | none |
| `served_from_home_note` — **the round's change in this function** | `doc.rs:4467-4471` → `crates/cli/src/render.rs:2450-2459` → `CodeOnlyCheckout::of_reader` (`render.rs:2280-2288`) → `repo::posture_subject` (`repo.rs:1002-1023`), `CommitSite::differing` (`render.rs:2136-2151`, `canonicalize` and `git symbolic-ref -q HEAD` at `repo.rs:1334-1347`), `repo::home_is_a_checkout` (`repo.rs:167-197`, two `git rev-parse` reads), `engine::milestone::owning_milestone` (`crates/engine/src/milestone.rs:1006-1021`, `read_dir` and a task-list read) | one stderr line | none |
| the staged arm, `run_list_staged` (`--task <id>`) | `doc.rs:5006-5064` | `ActiveTask::resolve` (`doc.rs:6702-6759`: `task::require_task_area`, `task.rs:2072-2089`, `is_dir` + `symlink_metadata`), `committed_schemas`, `staged_doc_ids`, `std::fs::read` of each staged copy (`:5040`) | none |
| the refusal arms (`store.unknown-type`, not set up, no such task) | `doc.rs:637-658` → `invocation_log::operational_failure` (`invocation_log.rs:411-437`) | stderr only | none |

**What the door does NOT call**, checked by reading `run_list` and `run_list_staged` end to end: `engine::index::load_committed` (`index.rs:242-258`), the one read-side function in the neighbourhood that persists a cache (`EdgeIndex::save`, `index.rs:117-122`), and `engine::index::invalidate` (`index.rs:182-189`), the one `remove_file` in the modules this path enters. The drive confirms it: in the corpus of rows C1-C4 `.jigc/index/` holds only `edges.json.lock` before and after the verb — no `edges.json` appears.

**Every subprocess this door can spawn** — four git reads, none of which takes the index lock or writes a ref:

1. `git -C <root> rev-parse --path-format=absolute --git-common-dir` — `repo.rs:128-132` (only where `.git` is a file)
2. `git -C <dir> rev-parse --path-format=absolute --show-toplevel --git-common-dir` and the same with `--git-common-dir` alone — `repo.rs:170-175` (only where the store's home and the standing checkout differ)
3. `git symbolic-ref -q HEAD` — `repo.rs:1335-1339` (the same condition)
4. `git ls-files -z` — `orphan.rs:149` (the unfiltered listing only)

The engine crate spawns nothing: the two textual hits for `Command::new` under `crates/engine/src` are a test's `mkfifo` (`finalize.rs:4671`) and a doc comment (`state.rs:1785`).

## The driven table

The instrument, for every row: `git status --porcelain --untracked-files=all` before; a snapshot of every entry under the repository (`.git/` and `.jigc/` included), the home directory and, in D, the linked worktree — kind, mode, size, sha256 (link target for a link) and mtime; the verb; the snapshot again; the porcelain again. The snapshots bracket the verb and nothing else. (A first version took the second porcelain before the second snapshot and showed the `.git` directory's mtime moving in rows A and B — in row A, where the verb exits before any git call. That was the instrument's own `git status`; the order was corrected and both rows re-driven, and the corrected runs are what the table holds.)

| row | repository state | cwd | argv | exit | stdout | stderr | porcelain before = after | tree snapshot |
|---|---|---|---|---|---|---|---|---|
| A | `git init` + one empty commit, `jigc setup` never ran; untracked `notes.md` | repo root | `jigc doc list` | 1 | empty | ``this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)`` | `?? notes.md` both | IDENTICAL, 40 entries |
| N | a plain directory, no repository; `notes.md` in it | that directory | `jigc doc list` | 1 | empty | ``not inside a git repository (no `.git` found from <dir>) — run jigc from inside the target git repository; if this project isn't one yet, `git init` here first`` | not a repository | the directory holds `notes.md` and nothing else afterwards; the home directory is empty |
| B | the same, then `jigc setup` (exit 0), then untracked `notes.md` (one line) | repo root | `jigc doc list` | 0 | `jigc doc list — no committed docs` | empty | `?? notes.md` both | IDENTICAL, 87 entries |
| B3 | as B | repo root | `jigc doc list --format json` | 0 | `{ "docs": [] }` (pretty-printed) | empty | same | IDENTICAL |
| B4 | as B | repo root | `jigc doc list adr` | 0 | ``jigc doc list — no committed `adr` docs`` | empty | same | IDENTICAL |
| B5 | as B | repo root | `jigc doc list nope` | 1 | empty | ``blocking · store.unknown-type — unknown doctype `nope` `` with its route | same | IDENTICAL |
| B6 | as B | repo root | `jigc doc list --task nope` | 1 | empty | ``blocking · finalize.no-task — no task `nope` `` with its route | same | IDENTICAL |
| C1 | rig `refs-post-hoc` (five committed docs, one live task staging two), plus untracked `notes.md` and `scratchdir/untracked.md` | repo root | `jigc doc list` | 0 | header + five `managed` rows | the staged-listing note | `?? notes.md`, `?? scratchdir/untracked.md` both | IDENTICAL, 145 entries |
| C2 | as C1 | repo root | `jigc doc list --task ground-the-vision-in-research` | 0 | header + two rows (the task's commit doc, `vision:vision`) | empty | same | IDENTICAL |
| C3 | as C1 | repo root | `jigc doc list vision --format json` | 0 | one row, `fields` carried | the staged-listing note, narrowed to `vision` | same | IDENTICAL |
| C4 | as C1 | repo root | `jigc doc list --task ground-the-vision-in-research --format json` | 0 | two rows | empty | same | IDENTICAL |
| D1 | as C1, plus `git worktree add <rig>/wt -b side` and an untracked `wt-notes.md` in the worktree | the linked worktree | `jigc doc list` | 0 | the same five rows as C1 | the staged-listing note, then the round's new line: ``note: served from the main checkout at `<rig>/repo` — jigc's doc store has one home, not the linked worktree at `<rig>/wt` on branch `side` you are standing in`` | main checkout: same; worktree: `?? wt-notes.md` both | IDENTICAL, 181 entries (main checkout, its `.git/worktrees/wt`, the worktree, home) |
| D2 | as D1 | the linked worktree | `jigc doc list --task ground-the-vision-in-research` | 0 | the two staged rows | empty (the staged arm prints no served-from line) | same | IDENTICAL |
| D3 | as D1 | the linked worktree | `jigc doc list --format json` | 0 | the JSON document | both notes, on stderr | same | IDENTICAL |
| E1 | rig `fresh`, `jigc config set invocation-log true` (exit 0), untracked `notes.md` | repo root | `jigc doc list` | 0 | `jigc doc list — no committed docs` | empty | `?? .jigc/config/manifest.yaml`, `?? notes.md` both | **DIFFERS — S1:** `.jigc/logs/` and `.jigc/logs/invocations.jsonl` (176 bytes) are new, `.jigc`'s mtime moved; nothing else |
| E2 | as E1, after it | repo root | `jigc doc list nope` | 1 | empty | the `store.unknown-type` block | same | **DIFFERS — S1:** the log grew from 176 to 380 bytes; nothing else |

Row B on the previous release's binary: exit 0, `jigc doc list — no committed docs`, porcelain `?? notes.md` both, tree snapshot IDENTICAL (85 entries), `notes.md` still `one line`.

## The seeded claim

### V1 — the seeded claim: `jigc doc list` exits 0 and an untracked file in the root is gone

```yaml
claim: "in a repository `jigc setup` has run in, `jigc doc list` exits 0 and an untracked file in the repository's root is gone afterwards"
verdict: REFUTED
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:            # every step in a fresh directory under the scratch root, HOME a fresh empty directory
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]                      # in repo; exit 0
  - "write notes.md, one line (`one line`), and do not add it"
repro:
  - ["jigc", "doc", "list"]                # in repo
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  stderr: ""
  after:
    - "notes.md stands: `one line`, sha256 3887c2cd3bec16420dc71507a74cf7f0a5effdd361f6d9cbe27b77831de8f65f before and after"
    - "`git status --porcelain` is `?? notes.md` before and after"
    - "every entry under repo (with .git/ and .jigc/) and under HOME is unchanged in kind, mode, size, sha256 and mtime — 87 entries"
also-on-previous-release: "the same: exit 0, the file stands, the tree snapshot identical (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
pinned-by: "UNPINNED: the nearest standing test, read_verb_acts_nothing::no_read_verb_acts_over_the_revealing_state, asserts that the repository tree minus .git/ is byte-identical around a bare `jigc doc list` — but its fixture holds no untracked file at the repository's root, so it would not see this file vanish"
```

The source reason, in one line: nothing below `DocCommand::List` (`doc.rs:633-635`) reaches a removal — the path's file operations are `read`, `read_to_string`, `read_dir`, `exists`, `metadata`, `symlink_metadata` and `canonicalize`, and its four subprocesses are git reads (the table above).

## Findings

Severity is information for triage. `door` is in the door list's words where the finding is this row's door.

### S1 — under the opt-in `invocation-log` knob the read verb creates `.jigc/logs/` and appends to its log, and the read-verb rule names no such write

- **door:** `jigc doc list`
- **clause:** `none` — as this reporter reads the first clause's scope, an opt-in append to a gitignored file the operator asked for destroys no bytes and commits nothing.
- **severity:** low — the behaviour is designed (`design/measurement.md` → The in-repo invocation log); what is wrong is that two texts say otherwise and the fence cannot see it.
- **class:** a write a `Read` leaf makes that the read-verb rule does not admit.
- **count_derivation:** the write has ONE call site, `invocation_log::log_invocation(` at `main.rs:117` (`grep -rn "log_invocation(" crates/cli/src` — the definition and that call), reached by every leaf. `LogWrite::for_leaf` (`invocation_log.rs:470-479`) answers `AppendOnly` for exactly one leaf, `uninstall`, so 47 of the registry's 48 rows may create the directory and the file. Of the 48, 12 are `VerbKind::Read` (`sed -n '2033,2090p' crates/cli/src/cli.rs` counted for `VerbKind::Read`: 12; for `VerbKind::`: 48). So the class is the 12 `Read` rows, of which this row drove one.
- **evidence, source:** `append_record` — `std::fs::create_dir(logs_dir)` at `invocation_log.rs:638`, `.create(mint).append(true).open(logs_dir.join(LOG_FILE))` at `:656-659`. The rule it is not named in: `cli.rs:1992-1994`, a verb is `Read` only when it "mutates nothing — neither repo files, nor the git index/history, nor the `.jigc/` workbench", with "**One carve-out**" (`cli.rs:1999`), the derived edge index. The fence repeats both: `crates/cli/tests/read_verb_acts_nothing.rs:6-8` and `:26-27`, `DERIVED_CACHE_PREFIX = ".jigc/index/"` at `:56`. The suite never switches the knob on (`grep -n "invocation-log\|invocation_log\|logs/" crates/cli/tests/read_verb_acts_nothing.rs`: no hit), so a `Read` leaf writing `.jigc/logs/` is outside what it can observe.
- **evidence, driven:** rows E1 and E2 of the table. The two records written, verbatim:

```
{"timestamp":"2026-10-08T08:07:27Z","argv":["doc","list"],"exit_code":0,"duration_ms":61,"finding_codes":[],"output_bytes":36,"binary_version":"1.0.0-rc.24","error_code":null}
{"timestamp":"2026-10-08T08:07:27Z","argv":["doc","list","nope"],"exit_code":1,"duration_ms":23,"finding_codes":["store.unknown-type"],"output_bytes":200,"binary_version":"1.0.0-rc.24","error_code":null}
```

#### Repro S1

```yaml
claim: "with the invocation-log knob on, `jigc doc list` at exit 0 creates .jigc/logs/ and .jigc/logs/invocations.jsonl; `git status --porcelain` does not move"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:
  - fixture: fresh                          # dev/jigc-rig --binary <candidate> fresh
  - ["jigc", "config", "set", "invocation-log", "true"]      # exit 0
  - "write notes.md, one line, and do not add it"
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  porcelain_before_and_after: ["?? .jigc/config/manifest.yaml", "?? notes.md"]
  new_entries: [".jigc/logs/", ".jigc/logs/invocations.jsonl (one JSON line, argv [\"doc\",\"list\"], exit_code 0)"]
  nothing_else_changed: true                # every other entry under repo (with .git/) and HOME identical, mtime included
pinned-by: "UNPINNED: read_verb_acts_nothing drives every Read leaf with the knob off, so no standing test observes this write; invocation_log's own unit tests pin the writer, not the read-verb rule"
```

### S2 — the log's append follows a link at the log's path

- **door:** `jigc doc list`
- **clause:** `none` — a link planted at `.jigc/logs/invocations.jsonl` is a deliberately planted state, which the first clause's scope declares a bound; and the write is an append, so the target's bytes stand.
- **severity:** low.
- **class:** an opening write that follows a link.
- **count_derivation:** `instance, unbounded` — one site read and driven (`invocation_log.rs:656-659`, the `Mint` arm). The writers elsewhere in the crate were not enumerated; the run's range carries a suite named `replacing_writers_never_follow`, which this reporter did not read against this site.
- **evidence, source:** `append_record`'s doc comment states the never-written-through property for `AppendOnly` only (`invocation_log.rs:614-619`, "a dangling link at the log's path is not written through either"); under `Mint` the open carries `create(true)` and follows the link.
- **evidence, driven:** knob on; the log moved aside; `.jigc/logs/invocations.jsonl` made a link to a file outside the repository holding `user bytes`; `jigc doc list` exits 0; the target then holds `user bytes` followed by the JSON record.

#### Repro S2

```yaml
claim: "with the knob on and a link at .jigc/logs/invocations.jsonl, `jigc doc list` appends its record to the link's target at exit 0"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:
  - fixture: fresh
  - ["jigc", "config", "set", "invocation-log", "true"]
  - ["jigc", "doc", "list"]                 # starts the log
  - "move .jigc/logs/invocations.jsonl out of the repository"
  - "write <rig>/target.txt holding the one line `user bytes`"
  - ["ln", "-s", "<rig>/target.txt", ".jigc/logs/invocations.jsonl"]
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 0
  target_txt: "line 1 `user bytes` (unchanged), line 2 the invocation record for argv [\"doc\",\"list\"]"
pinned-by: "UNPINNED: no test read for this site; the AppendOnly arm's link case is pinned by invocation_log::tests::an_append_only_write_creates_nothing_and_still_appends_to_a_log_that_is_there, which asserts that arm creates nothing — a different arm"
```

### S3 — one unreadable entry named `*.md` at a located home fails the whole listing, with no route; a named pipe there hangs it; `jigc validate` over the same store says nothing

- **door:** `jigc doc list`
- **clause:** `none` — nothing is written or lost, and the previous release behaves identically, so it is no regression as the second clause's instrument counts one. Recorded for its tier: a report verb that one stray directory entry turns into exit 1 with zero rows.
- **severity:** medium for the verb's own job (the index read an agent is told to use instead of the filesystem), low against the closing condition.
- **class:** the census takes an entry by NAME and its consumer reads it unconditionally. `engine::index::committed_instances` admits every directory entry whose extension is `md` (`index.rs:807-814`) with no file-type check; `run_list` then does `std::fs::read(&path).with_context(...)?` (`doc.rs:4866-4867`), so the first entry that cannot be read ends the verb through `DocFailure::Orchestration` (`doc.rs:655-657`) — the flattened operational error, which carries an absolute path and no route.
- **count_derivation:** four shapes driven at ONE site in this door (`doc.rs:4866`): a directory named `notes.md`, a dangling link `dangling.md`, a mode-000 `locked.md`, a named pipe `pipe.md`. The door's two other hard reads were read and are guarded: `doc.rs:4953` reads only a path `carries_stamp` has just read (`orphan.rs:629-634`), and `doc.rs:5040` only an entry `staged_doc_ids` has stat-ed as a file (`task.rs:1227`). Beyond this door: `committed_instances(` has 14 textual call sites under `crates/cli/src` and `crates/engine/src`, in-file tests included (`cli.rs` 1, `doc.rs` 1, `orphan.rs` 4, `start.rs` 1, `engine/file_state.rs` 1, `engine/index.rs` 3, `engine/target_surface.rs` 1, `engine/validate.rs` 2); this reporter read four of them — `doc.rs:4862` (hard `?`), `orphan.rs:394` and `index.rs:214` (a failed read is skipped), `orphan.rs:608` (no read). The other ten were not examined: **an instance in this door, unbounded across the mechanism's consumers.**
- **evidence, driven** (rig `refs-post-hoc`, whose located home is `docs/research/`; identical on both binaries, each in its own rig):

| shape planted in `docs/research/` | argv | exit | stdout | stderr |
|---|---|---|---|---|
| none (baseline) | `jigc doc list` | 0 | header + five rows | the staged-listing note |
| a directory `notes.md` | `jigc doc list` | 1 | empty | `reading the committed doc at "<rig>/repo/docs/research/notes.md": Is a directory (os error 21)` |
| the same | `jigc doc list research` | 1 | empty | the same line |
| the same | `jigc doc list vision` | 0 | header + the `vision` row | — (another doctype's home is not walked) |
| a dangling link `dangling.md` | `jigc doc list` | 1 | empty | `reading the committed doc at "<rig>/repo/docs/research/dangling.md": No such file or directory (os error 2)` |
| the same | `jigc validate` | 0 | one unrelated advisory (`schema-conformance.repeatable-populated` on the decisions log) and the report-only trailer; the dangling entry is not mentioned | empty |
| a mode-000 file `locked.md` | `jigc doc list` | 1 | empty | `reading the committed doc at "<rig>/repo/docs/research/locked.md": Permission denied (os error 13)` |
| a named pipe `pipe.md` | `jigc doc list` under an 8-second alarm | 142 (killed by the alarm) | empty | empty — the verb blocks in the read |
| every plant removed | `jigc doc list` | 0 | byte-identical to the baseline | |

  The verb's own doc comment sets the standard it misses here: the row loop calls an unparseable instance "never a block: `doc list` is a report" (`doc.rs:4879-4882`), and the listing and the store sweep are to tell one story about one file (`doc.rs:4784-4797`). An entry the listing dies on and the sweep is silent about is two.

#### Repro S3

```yaml
claim: "a directory entry named *.md that cannot be read, at a located doctype's home, makes `jigc doc list` exit 1 with no rows and no route; `jigc validate` over the same store exits 0 without naming it"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "identical in all four shapes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d) — not a regression"
setup:
  - fixture: refs-post-hoc                  # dev/jigc-rig --binary <binary> refs-post-hoc
  - ["ln", "-s", "nowhere-target", "docs/research/dangling.md"]
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "validate"]
expect:
  - exit: 1
    stdout: ""
    stderr: "reading the committed doc at \"<repo>/docs/research/dangling.md\": No such file or directory (os error 2)"
  - exit: 0
    stdout_lacks: "dangling.md"
variants:
  - "mkdir docs/research/notes.md            -> exit 1, `Is a directory (os error 21)`"
  - "a mode-000 docs/research/locked.md      -> exit 1, `Permission denied (os error 13)`"
  - "mkfifo docs/research/pipe.md            -> no exit: the verb blocks reading the pipe (killed by an 8 s alarm, status 142)"
pinned-by: "UNPINNED: no test found for an unreadable entry at a located home on this door"
```

### S4 — where `.git` is a file and no second checkout exists, the listing is read at a directory that is no checkout, at exit 0 and with no note (on record; the previous release's behaviour too)

- **door:** `jigc doc list`
- **clause:** `none` for this door — it reads, and what it reads wrongly it only reports. (The write doors in the same layouts are lead L1.)
- **severity:** low here; returned so triage sees that the round's one change in this function, the served-from note, is silent exactly where the store's home is the surprising one — by design (`repo.rs:164-166`, `render.rs:2227-2234`).
- **class:** `repo::jigc_home` answers `dirname(git-common-dir)` wherever `.git` is a file (`repo.rs:98-101`).
- **count_derivation:** two of the three layouts the source names (`repo.rs:148-155`) were driven on both binaries — a submodule and a `--separate-git-dir` checkout; a worktree of a bare repository was not. The class is already on the fix pass's ledger with its reach stated as "every door in those layouts" (`completions/artifacts/M55/fix-pass-rc25/findings-ledger.md`, rows `4A-8`, `4B-5`, `4C-3`, `4E-2`) and is owed a ruling at `implementation/decisions-pending.md` → (D), trigger M57. This row adds the one door's driven behaviour, not a new class.
- **evidence, driven:**
  - **submodule** (`<super>/vendor/lib`, `.git` → `../../.git/modules/vendor/lib`): `jigc doc list` before any setup exits 1, *this project isn't set up*; `jigc setup` typed in the submodule exits 0; `jigc doc list` then exits 0 with `jigc doc list — no committed docs` and an empty stderr — served from `<super>/.git/modules/vendor/`, which is where `.jigc/` was put.
  - **`--separate-git-dir`** (`.git` → `<dir>/store/sep.git`): `jigc setup` exits 1 at `setup.install-hook` and leaves `<dir>/store/.jigc/` behind; `jigc doc list` then exits 0 with `jigc doc list — no committed docs`, served from `<dir>/store/`.

#### Repro S4

```yaml
claim: "in a submodule, after `jigc setup` there, `jigc doc list` exits 0 serving a store whose home is <super>/.git/modules/<parent> and prints no served-from note"
verdict: CONFIRMED
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "identical (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
setup:
  - ["git", "init", "-q", "lib"]
  - ["git", "-C", "lib", "commit", "-q", "--allow-empty", "-m", "lib-base"]
  - ["git", "init", "-q", "super"]
  - ["git", "-C", "super", "commit", "-q", "--allow-empty", "-m", "super-base"]
  - ["git", "-C", "super", "-c", "protocol.file.allow=always", "submodule", "add", "-q", "<abs>/lib", "vendor/lib"]
  - ["git", "-C", "super", "commit", "-q", "-m", "add submodule"]
  - ["jigc", "setup"]                       # cwd super/vendor/lib; exit 0
repro:
  - ["jigc", "doc", "list"]                 # cwd super/vendor/lib
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  stderr: ""
  where_the_store_is: "super/.git/modules/vendor/.jigc exists; super/vendor/lib holds only .git"
pinned-by: "linked_worktree_doc_home::a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard — cited by decisions-pending (D) as the pin of these layouts; this reporter did not open it to check that it asserts anything about a read"
```

### S5 — the standing read-verb fence observes this door in one form only

- **door:** `jigc doc list`
- **clause:** `none` — no defect is hidden behind it as far as these drives reach; it is a statement about what the fence would catch if one arrived.
- **severity:** low.
- **class:** input coverage of a fence over a multi-arm leaf.
- **count_derivation:** the fence's table has one row for this leaf, `(&["doc", "list"], &["doc", "list"])` (`crates/cli/tests/read_verb_acts_nothing.rs:398`), driven from the main checkout of one fixture. The leaf has two arms with different callee sets — `run_list` and, under `--task`, `run_list_staged` (`doc.rs:4807-4809`, `:5006`) — so one arm of two is swept. Its byte snapshot leaves `.git/` out (`:321-323`, compared through a HEAD/diff/commit-count summary instead) and never covers the home directory. The other eleven `Read` rows' tables were not read: **an instance, unbounded** across the registry's `Read` leaves.
- **what this pass adds:** rows B3-B6, C2-C4 and D1-D3 of the driven table drive the forms the fence does not — the staged arm, the doctype filter, JSON, a linked worktree — with `.git/` and the home directory inside the snapshot, and every one is identical.

#### Repro S5

```yaml
claim: "`jigc doc list --task <id>` — the arm the read-verb fence does not drive — writes nothing, .git/ and HOME included"
verdict: CONFIRMED (the arm is write-free; the fence does not hold it)
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging two docs
  - "write notes.md and scratchdir/untracked.md, and do not add them"
repro:
  - ["jigc", "doc", "list", "--task", "ground-the-vision-in-research"]
expect:
  exit: 0
  stdout_rows: ["commit:ground-the-vision-in-research  commit:ground-the-vision-in-research  managed", "vision:vision  VISION.md  managed"]
  porcelain_before_and_after: ["?? notes.md", "?? scratchdir/untracked.md"]
  tree_snapshot: "identical — 145 entries under repo (with .git/ and .jigc/) and HOME, mtime included"
pinned-by: "UNPINNED: read_verb_acts_nothing::no_read_verb_acts_over_the_revealing_state asserts byte-identity of the repository tree minus .git/ around the bare form only"
```

## Leads — noticed, not pursued

### L1 — `jigc setup` typed in a submodule installs into the superproject's git directory, and into the superproject's hooks

- **door:** `jigc setup` — unlisted (outside this row; in the round's scope it is an excluded door).
- **clause:** `no-lost-files`, as a question for whoever owns that door: this reporter saw the writes land and did not test whether any of them replaces bytes.
- **severity:** unknown here; the layout is on record as owed a support ruling (S4's references).
- **class / count_derivation:** `instance, unbounded` — seen once, while building the state for S4, on both binaries; not pursued.
- **what was seen** (candidate, exit 0): `<super>/.git/modules/vendor/` afterwards holds `.claude`, `.jigc`, `CLAUDE.md` beside the submodule's own git directory `lib`; the ack names the hook's home as `<super>/.git/hooks/pre-commit`, and that file exists there, executable — the SUPERPROJECT's hook, installed by a setup typed in the submodule; the submodule's work tree holds only `.git`; `git status --porcelain` is empty in both. Ledger row `4A-8` records "in a submodule it installs at `<super>/.git/modules`"; whether the hook's landing in the superproject's hooks directory, and what it does to a `pre-commit` hook the superproject already has, is on record anywhere was not checked.
- **the `--separate-git-dir` sibling, seen the same way:** `jigc setup` exits 1 at `setup.install-hook` and leaves `.jigc/.gitignore`, `.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml` and `.jigc/version` in the git directory's parent (`4E-2` records "exits 1 at its hook step … after writing").

#### Repro L1

```yaml
claim: "`jigc setup` typed in a submodule exits 0 having written .jigc/, CLAUDE.md and .claude/ into <super>/.git/modules/<parent>/ and a pre-commit hook into <super>/.git/hooks/"
verdict: OBSERVED, not pursued (a lead)
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup: "the six git steps of Repro S4"
repro:
  - ["jigc", "setup"]                       # cwd super/vendor/lib
expect:
  exit: 0
  stdout_contains: "pre-commit hook → <abs>/super/.git/hooks/pre-commit"
  after: ["super/.git/modules/vendor/ lists .claude, .jigc, CLAUDE.md, lib", "super/.git/hooks/pre-commit exists, mode 755", "super/vendor/lib lists only .git"]
pinned-by: "UNPINNED: not looked for"
```

### L2 — one tracked path that is not UTF-8 would empty the orphan walk, silently

- **door:** `jigc doc list`
- **clause:** `none` for this door.
- **severity:** low; a source reading, not driven.
- **class / count_derivation:** `instance, unbounded` — `orphan::committed_markdown` (`orphan.rs:144-159`) asks `git ls-files -z` through `task::git_capture`, which fails the whole capture unless the output is UTF-8 (`String::from_utf8(out.stdout)`, `task.rs:10100-10101`), and `committed_markdown` turns that failure into an empty listing (`orphan.rs:149-151`). Its consumers on this door are `orphaned_docs` (`doc.rs:4925`) and `orphaned_instances` (`doc.rs:4948`): with one such path anywhere in the index, every `orphaned` row would drop out of the listing at exit 0. The same enumerator feeds the store sweep, whose orphaned-instance finding is blocking (`orphan.rs:556-561`) — not this row's door, and not read further. **Not driven:** the filesystem these drives ran on refuses a file name that is not UTF-8, so the state could not be built here.

#### Repro L2

```yaml
claim: "with one tracked path whose name is not valid UTF-8, `jigc doc list` prints no `orphaned` row for a stamped file no doctype claims"
verdict: NOT DRIVEN (a lead from source: orphan.rs:149-151, task.rs:10100-10101)
setup:
  - fixture: fresh
  - "on a filesystem that admits it: commit a file whose name holds the byte 0xFF"
  - "commit a stamped .md inside the docs home that no resolved doctype claims"
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 0
  stdout_lacks: "orphaned"
pinned-by: "UNPINNED: not driven"
```

## What this pass did not examine

- The eleven other `Read` leaves and every write leaf: out of this row.
- The pack-load assertions beyond a scan for write primitives (`pack.rs:1-2587`): they were not read line by line for correctness, only for whether they can write.
- `git` itself: the four subprocesses are taken to be reads on the strength of what they are; the drives show `.git/` byte-identical around them under the git this machine has, with no global or system configuration. A repository carrying `core.fsmonitor`, an untracked cache or a filter driver was not driven.
- `JIGC_PACK_DIR`, a project pack list and a project-layer schema shadow: the listing was driven over the built-in packs only.
- A worktree of a bare repository, and jigc's own fan-out worktree (`of_reader` returns `None` there by design, `render.rs:2284-2286`) — read, not driven.
- Concurrency: nothing was driven against a second writer.

<!-- end of report -->
