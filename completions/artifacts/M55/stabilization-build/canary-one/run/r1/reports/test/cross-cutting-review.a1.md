# canary-one · round 1 · `test` — the cross-cutting pass (`cross-cutting-review`, attempt 1)

Item `cross-cutting` (kind `audit-cross-cutting`, clause `no-lost-files`). Read-only for code: no edit, no commit, no build. The one file written is this report.

## The question, and the answer

The brief bounds this pass to ONE question over the round's doors: *does any code path a door reaches write to the working tree or to git at exit 0 without the user having asked for it?* The round has one door, `jigc doc list`.

**Answer.** With the product's defaults, no: nothing the door reaches writes anything — not the working tree, not `.git/`, not `.jigc/`, not the home directory — and it spawns four git commands, all of them reads. The door reaches exactly **one** writer, and it is not the door's own: the per-invocation log append in `fn main`'s tail (`crates/cli/src/invocation_log.rs:621`, `append_record`), which runs only when the project's `invocation-log` knob is `true` (default `false`, `crates/cli/packs/dev/config/knobs.yaml:78-80`) and whose destination, `.jigc/logs/invocations.jsonl`, is gitignored (`crates/cli/src/gitignore.rs:38`). That write is asked for. **One finding sits on it (F1):** the append opens its destination through whatever entry is at that path, so a link there sends a read verb's record into the working tree at exit 0. It is the same on the previous release. Three leads are listed and were not pursued.

The verdict, for this door and this question: **the door holds in a repository used as documented; F1 is a planted-state defect on an opt-in path — as I read it outside the first clause's scope, on a bound this run has not declared.**

## What was driven, and its identity

- **Candidate:** label `c1`, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. `dev/stabilize-step hash --file bin/c1.a1/jigc` printed `content_sha256` `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash handed over — before any command was driven. Its directory went first on `PATH` and `command -v jigc` printed that path.
- **Previous release:** `1.0.0-rc.24`, commit `91834b5e011de2c36e2be2b79e96c0b9f60a803c`. The same call with `bin/previous-91834b5e011d/jigc` printed `content_sha256` `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`. Driven for F1's regression fact only.
- **No `cargo build`; nothing under `target/` was driven.** Rigs were built with `dev/jigc-rig fresh --binary <that path>`, `SCRATCH` pointed at a directory minted with `mktemp -d` under my own root `<scratch>/xcut.7rxkuS/`.
- **Tree at return:** branch `fix/canary-one` at `eeffe347`; `git status --short` shows the stage's own untracked `completions/artifacts/canary-one/r1/` and nothing else.
- Evidence files, all under `<scratch>/xcut.7rxkuS/`: `callgraph.py`, `cg-all.txt`, `reach.txt` (the source enumeration); `snap.py`, `drive.sh`, `drive-cand.out`, `drive-prev.out` (the driven block).

## How the answer was derived

### 1. The path, read by hand

`fn main` (`crates/cli/src/main.rs:33-126`) → `probe_intercept` (argv compare only, `crates/cli/src/invoke.rs:140-170`) → `route_fence::install` → `invocation_log::enabled_logs_dir` (`invocation_log.rs:535-559`: locate, load the pack, resolve the cascade, read the knob — no write) → `OutputTee::install` (fd-level `dup`/`pipe`, no file) → `Cli::try_parse` → `Cli::dispatch` (`cli.rs:822`) → `refuse_on_posture` (`cli.rs:642-652`: returns at `:650` because the `doc list` row is `ActsOnBehalf::Neither`, `cli.rs:2332-2335`, so no posture probe runs) → `run_doc` (`cli.rs:1809`) → `DocCommand::dispatch` (`doc.rs:633-635`) → `run_list` (`doc.rs:4801-4977`) or, under `--task`, `run_list_staged` (`doc.rs:5006-5064`) → back in `main`, `log_invocation` (`main.rs:116-124`).

`run_list` reads: `require_project_layer` → `locate` (`locate.rs:190-217`); `make_pack`; `resolve_severity_cascade`; `CascadeDefs::all_schemas`/`declared_schemas`; `orphan::prior_home_instances`; `engine::index::committed_instances`; `std::fs::read` of each listed doc (`doc.rs:4866`, `:4953`); `orphan::orphaned_docs`/`orphaned_instances`; `render_listing` (stdout); `staged_listing_hint` (`doc.rs:5136`, existence checks, stderr); `served_from_home_note` (`doc.rs:4467-4471`, stderr). It does not reach `engine::index::load_committed`, so the one write a `Read` verb is allowed — the derived edge index — is not on this door either.

The git commands on the path, each a read:

| where | command | when |
|---|---|---|
| `repo.rs:127` (`git_common_dir_parent`) | `git -C <root> rev-parse --path-format=absolute --git-common-dir` | only when `.git` is a file (`repo.rs:98`) |
| `repo.rs:167` (`home_is_a_checkout`) | `git -C <dir> rev-parse --path-format=absolute --show-toplevel --git-common-dir` and `… --git-common-dir` | the served-from-home note, via `render.rs:2450` |
| `repo.rs:1334` (`head_ref`) | `git symbolic-ref -q HEAD` | the same note |
| `orphan.rs:149` → `task.rs:10087` (`git_capture`) | `git ls-files -z` | an unfiltered listing (`doc.rs:4922-4925`) |

### 2. The enumeration behind "exactly one writer" — and its bounds

A name-based, over-approximating call graph over the non-test source of both product crates (`callgraph.py`): **2461** functions indexed; **142** of them hold a write primitive (`fs::write`, `fs::remove_file`/`remove_dir`/`remove_dir_all`, `fs::rename`, `fs::create_dir`/`create_dir_all`, `File::create`/`create_new`/`options`, `OpenOptions`, `fs::copy`, `set_permissions`, `symlink`, `hard_link`, `Command::new`, `set_len`, `set_modified`, `set_times`, `libc::unlink`/`rename`/`open`, `DirBuilder`). From the ten roots read off the path above (`run_list`, `run_list_staged`, `enabled_logs_dir`, `log_invocation`, `refuse_on_posture`, `cwd_or_refusal`, `route_fence::install`, `probe_argv`, `LogWrite::for_leaf`, `Command::leaf`) **509** functions are reachable and **13** of those hold a write primitive. Each of the 13, adjudicated at its source:

| function | primitive | adjudication |
|---|---|---|
| `invocation_log.rs:621` `append_record` | `fs::create_dir`, `OpenOptions` create+append | **the one writer** — knob-gated; F1 |
| `repo.rs:127` `git_common_dir_parent` | `Command::new("git")` | `rev-parse` — a read |
| `repo.rs:167` `home_is_a_checkout` | `Command::new("git")` | `rev-parse` — a read |
| `repo.rs:1334` `head_ref` | `Command::new("git")` | `symbolic-ref -q HEAD` — a read |
| `task.rs:10087` `git_capture` | `Command::new("git")` | reached through one caller, `orphan.rs:144` `committed_markdown`, with `["ls-files", "-z"]` — a read |
| `repo.rs:615` `index_has_unmerged_paths` | `Command::new("git")` | **not reached**: `refuse_on_posture` returns at `cli.rs:650-652` for `ActsOnBehalf::Neither` |
| `task.rs:9075` `head_is_unborn` | `Command::new("git")` | **not reached**, same return |
| `milestone.rs:4065` `Registration::read`, `milestone.rs:4158` `git_in` | `Command::new` | false edge: `owner.read(…)` at `pack.rs:1783` is `PackSource::read`; the graph matches every method named `read` |
| `setup.rs:4634` `git_output` | `Command::new` | false edge, same collision (`SettingsClaim::read`) |
| `engine/src/store.rs:210` `home_entry` | the `symlink` pattern | false edge (same collision), and the hit is `is_symlink()` on `symlink_metadata` — a read |
| `combine.rs:289` `TempIndex::new`, `task.rs:9765` `CombineIndex::new` | `fs::remove_file` | false edge: `OsStr::new` at `invoke.rs:141-148` matched every `new` |

**Control, so the instrument is not blind:** the same script rooted at a write verb's handler (`doc.rs` `run_set_slot`) reaches `engine/src/state.rs:1328` `write_atomic` and `state.rs:1548` `SaveLock::acquire`; neither is in `doc list`'s reach (the reach list of `reach.txt` has 0 lines naming `write_atomic`, `load_committed` or `SaveLock::acquire`).

**Bounds of the enumeration, stated rather than assumed.** (a) Calls through function pointers held in `const` tables are not edges: the tables are `render.rs:1046-1056` (`matches`/`witness`/`trailer`), `render.rs:4109` (`witness`) and `cli.rs:3811`/`:3850`/`:4119` (`tip`) — their values are string and finding builders, read by eye, and none is on this door's success path. (b) `Drop` impls are not edges; the six in the non-test source were listed and none of their types is constructed in the reach (`TempIndex`, `CombineIndex`, `ScratchTree` are named only by their own `new`, reached by the `OsStr::new` false edge; `RecordFlipGuard`, `DedicatedWorktree`, `SaveLock` are named by no reachable function). (c) A write primitive imported under a bare name would be missed: a grep for unqualified `remove_file(`, `remove_dir_all(`, `remove_dir(`, `rename(`, `create_dir_all(`, `create_dir(`, `copy(`, `hard_link(`, `symlink(`, `set_permissions(` in both `src/` trees has 0 production calls — its hits are two functions *named* `rename` (`render.rs:5124`, `engine/src/finalize.rs:1096`), a test helper's calls to one, and `symlink(` calls inside `mod tests`. (d) The three `macro_rules!` in the crates (`pack_path` twice, `id_newtype`) build paths and newtypes. (e) What git itself does inside its four read commands is git's, not read here; the driven cell below watched `.git/` for it. (f) The question is asked at exit 0: refusal paths were not enumerated.

### 3. The driven block — no more than the finding and its control need

`drive.sh <label> <binary>`, once per binary. Cell A is the control for the knob-off answer; cells C and D are F1's. `snap.py` records, for every entry under the repository (its `.git/` and `.jigc/` included) and the rig's home: kind, mode, size, mtime in nanoseconds, inode, and a regular file's sha256.

| cell | state | candidate `c1` | previous `rc.24` |
|---|---|---|---|
| A | rig `fresh`, knob off, an untracked `notes.md` in the root; `jigc doc list` | exit 0; stdout `jigc doc list — no committed docs`; stderr 0 bytes; snapshot **identical** (94 entries); `git status --porcelain` `?? notes.md` before and after; `notes.md` bytes intact | the same (92 entries) |
| A2 | the same repository, from a linked worktree (`git worktree add`), `jigc doc list --format json` | exit 0; one stderr line `note: served from the main checkout at …`; snapshot of repository, home and the worktree **identical** | exit 0; stderr empty (the note is new); snapshot identical |
| B | `jigc config set invocation-log true`, then `jigc doc list` | exit 0; the diff is `.jigc/logs/` and `.jigc/logs/invocations.jsonl` (plus `.jigc`'s own mtime); `git check-ignore -v` answers `.jigc/.gitignore:6:logs/` | the same |
| C | knob on and committed; a **dangling** link `.jigc/logs/invocations.jsonl -> ../../minted-by-a-read.md`; `jigc doc list` | exit 0; `git status --porcelain` goes from empty to `?? minted-by-a-read.md`; the file holds the run's JSON record | the same |
| D | knob on and committed; a link `.jigc/logs/invocations.jsonl -> ../../TRACKED.md`, a tracked file; `jigc doc list` | exit 0; `git status --porcelain` goes from empty to ` M TRACKED.md`; `git diff --stat` `1 insertion(+)`; the JSON record is the file's new last line | the same |

A first run of cell A showed `.git`'s mtime moving; that was the harness's own `git status` between the two snapshots, and the cell was re-run with the status taken outside them — the table is the re-run.

**Observed in cell C's setup, not a finding:** with the knob on, the plain `git commit` of the config change left one record in the log, `["validate","--format","json"]` — the installed `pre-commit` hook's own jigc call. That is the knob's stated reach (one record per jigc invocation) and the file is gitignored.

**On the ledger's seeded row (`canary-seeded-claim`):** not mine to grade, and cell A is not its block as written (the rig's `fresh` state stands in for the block's first three steps). What this pass can say: no `fs::remove_*` is reachable from the door by source (the two by-name hits are the `OsStr::new` false edges above), and in cell A an untracked `notes.md` in the root of a set-up repository was there, byte-identical, after `jigc doc list` exited 0, on both binaries.

## Findings

### F1 — with the log knob on, `jigc doc list` writes its record through a link at the log's path, into the working tree, at exit 0

- **Severity:** low (information for triage). **Door:** `jigc doc list`. **Clause:** `no-lost-files`, as I read it — *nothing incorrect is written or updated* — with the scope caveat below.
- **Where:** `crates/cli/src/invocation_log.rs:656-660` — `OpenOptions::new().create(mint).append(true).open(logs_dir.join(LOG_FILE))`, then `write_all`. No `O_NOFOLLOW`, and the open handle is never asked what it is. `mint` is `true` for every leaf but the teardown (`LogWrite::for_leaf`, `:470-479`), so a dangling link is created *through*. Every failure is swallowed (`:510`, `let _ =`), so nothing is said either way.
- **What happens:** cells C and D above. A read verb exits 0 having minted an untracked, un-ignored file in the repository root, or having appended a JSON line to a tracked file. No byte is destroyed (the open appends), and this door commits nothing.
- **Why it reads as a defect and not as design:** the fix pass this candidate carries made *never through a link* a rule for the writers that put a doc at a home (`regular_file::open_no_follow`, `crates/cli/src/regular_file.rs:182`, the crates' only `O_NOFOLLOW` open, with 2 callers: `task.rs:5815`, `regular_file.rs:166`), and the same pass edited this very open (`git diff 91834b5e..eeffe347 -- crates/cli/src/invocation_log.rs`: `create_dir_all` → `create_dir`, `.create(true)` → `.create(mint)`). Its doc comment (`:614-619`) argues link-safety for one case only — a *dangling* link under `AppendOnly` — and is silent on the `Mint` arm, where the dangling link is exactly the one that is written through.
- **Scope, as I read the rule** (`DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 1): the first clause binds *in a healthy repository used as documented*, and *deliberately planted states are declared bounds, written down with their reach*. A link at a path inside the gitignored workbench, which no jigc door creates, is a planted state — so this is a bound to declare, not a break inside the scope, **unless** triage reads a relocated log as ordinary use. This run declares no bound, so the row needs one or a ruling.
- **Regression fact:** **not a regression** — cells C and D give the same result on `1.0.0-rc.24`.
- **Class:** a non-replacing writer that opens its destination through whatever entry is there.
- **Count derivation:** by the mechanism's consumers, enumerated. `append_record` has **1** production caller (`log_invocation`, `invocation_log.rs:510`; `grep -n "append_record(" crates/cli/src/*.rs` gives 7 hits — the definition, that call, and 5 inside `mod tests` at `:740`, `:759`, `:797`, `:1015`, `:1027`). `log_invocation` has **1** call site (`main.rs:117`; `grep -rn "log_invocation(" crates/cli/src crates/engine/src` gives the definition and that line), which every invocation passes. `LogWrite::for_leaf` answers `Mint` for **47 of the 48** rows of `VERB_KINDS` and for an argv that reaches no verb. So: 1 writer, 1 call site, every leaf but `uninstall` — and **1 door** of this round's list. **Driven:** 1 door, 2 link shapes, 2 binaries. **Not driven:** the other 46 `Mint` leaves (one call site by source; an instance as far as driving goes). As a member of the wider class across the crates — every appending or in-place writer that follows a link — this is **an instance, unbounded**: only the writers reachable from this door were enumerated (1), not the 142 write-holding functions.
- **Repro:** the block under *Repro F1*, below.

#### Repro F1

```yaml
claim: "with invocation-log on, `jigc doc list` exits 0 and writes its log record through a link at .jigc/logs/invocations.jsonl, into the working tree"
verdict: CONFIRMED
binary: "candidate c1, commit eeffe347, content_sha256 dded1fac…beadfc — and the same on the previous release 1.0.0-rc.24, content_sha256 accf3996…ab5d"
setup:
  - fixture: fresh                      # dev/jigc-rig fresh --binary <binary>; HOME is the rig's
  - ["jigc", "config", "set", "invocation-log", "true"]
  - ["git", "add", "-A", ".jigc"]
  - ["git", "commit", "-q", "-m", "switch the log on"]
  - "move .jigc/logs/invocations.jsonl out of the repository"   # the commit's pre-commit hook logged its own `jigc validate`
  - ["mkdir", "-p", ".jigc/logs"]
  - ["ln", "-s", "../../minted-by-a-read.md", ".jigc/logs/invocations.jsonl"]   # dangling: the target is absent
repro:
  - ["git", "status", "--porcelain"]    # prints nothing
  - ["jigc", "doc", "list"]
  - ["git", "status", "--porcelain"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  status_after: "?? minted-by-a-read.md"
  minted_file: '{"timestamp":"<t>","argv":["doc","list"],"exit_code":0,"duration_ms":<n>,"finding_codes":[],"output_bytes":36,"binary_version":"1.0.0-rc.24","error_code":null}'
variant:                                # cell D — the link's target is a tracked file
  setup_instead:
    - "write TRACKED.md (three lines), git add -A ., git commit"
    - ["ln", "-s", "../../TRACKED.md", ".jigc/logs/invocations.jsonl"]
  expect:
    exit: 0
    status_after: " M TRACKED.md"
    diff_stat: "TRACKED.md | 1 +"
    last_line_of_TRACKED: "the same JSON record"
control: "cell B — no link: the record lands in .jigc/logs/invocations.jsonl, ignored by .jigc/.gitignore:6"
pinned-by: "UNPINNED: no suite plants a link at the log's path — grep for symlink together with invocations|logs over crates/cli/tests/*.rs has 0 hits"
```

## Leads — noticed, not pursued

### Lead L1 — whether a committing door carries F1's redirected record into a commit

Cell D leaves a tracked file modified and cell C an untracked one, at exit 0 and unannounced. A door that stages the working tree on the user's behalf (`jigc task finalize` commits a task's code changes) could then commit a line nobody wrote — *content the user did not ask for*, in the first clause's own words. Not read, not driven. **Door:** `jigc task finalize` — unlisted. **Clause:** `no-lost-files`. Instance, unbounded.

### Lead L2 — `.jigc/logs` itself as a link to a directory

`append_record` answers `AlreadyExists` from `create_dir` as success (`invocation_log.rs:638-642`) without asking what is there, so with `.jigc/logs` a link to a directory the record is written into that directory, wherever it is. Read at the source, not driven; a user who relocated the log this way would want exactly that, so it may be no defect. **Door:** `jigc doc list`. **Clause:** `none`. Instance, unbounded.

### Lead L3 — the teardown's append-only arm writes through a *live* link

Under `LogWrite::AppendOnly` the open is `.create(false).append(true)` (`invocation_log.rs:656-659`): the doc comment covers the dangling link, which fails; a link whose target exists is appended through. Read at the source, not driven. **Door:** `jigc uninstall` — unlisted. **Clause:** `none`. Instance, unbounded.

## Held — with its repro block

#### Repro H1

```yaml
claim: "`jigc doc list` writes to the working tree or to git at exit 0 without the user having asked (the invocation-log knob at its default, off)"
verdict: REFUTED
binary: "candidate c1, commit eeffe347, content_sha256 dded1fac…beadfc — and the same on the previous release"
setup:
  - fixture: fresh                      # dev/jigc-rig fresh --binary <binary>
  - "write notes.md (one line) in the root, not added"
  - "snapshot every entry under the repository (.git/ and .jigc/ included) and the rig's home: kind, mode, size, mtime_ns, inode, sha256"
repro:
  - ["jigc", "doc", "list"]
  - "snapshot again"
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs"
  stderr_bytes: 0
  snapshots: "identical"
  status: "?? notes.md, before and after"
second_cell: "from a linked worktree, `jigc doc list --format json`: exit 0, snapshots of repository, home and worktree identical"
pinned-by: "read_verb_acts_nothing::no_read_verb_acts_over_the_revealing_state"
```

What that test asserts, read at `crates/cli/tests/read_verb_acts_nothing.rs:439-543`: the `doc list` row (`:398`) is driven through the real binary over a copy of the fixture, and the whole tree outside `.git/` must be byte-identical afterwards save paths under `.jigc/index/`, with `HEAD`, the commit count and `git status --porcelain` unchanged. **Its bounds against this claim:** the knob is off in its fixture; `.git/`'s bytes are not compared (git's state is, through three commands); it drives the bare form only — not `doc list <doctype>`, not `--task <id>`, not a linked worktree; and its walker panics on a link, so it could not hold F1 as written.

## Not examined

- Every path that does not exit 0.
- `doc list <doctype>` and `doc list --task <id>` by driving — read at the source only (`doc.rs:4820-4824`, `:5006-5064`, `:6702-6759`).
- A corpus with committed docs, prior-home instances or orphan rows by driving — the rig's `fresh` state has none; those arms are `std::fs::read` plus parsing at the source (`doc.rs:4862-4963`).
- The debug-only route fence — it is not in a release binary.
- A team layer under the home's `.config/jigc` setting the knob.
- What git does inside its own four read commands under configuration such as fsmonitor or the untracked cache.
- The other 47 leaf verbs: this round's door list has one door.

<!-- end of report -->
