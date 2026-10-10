# verify-real — `r1-p3-unreadable-entry-class-door-list-traced-by-hand` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-unreadable-entry-class-door-list-traced-by-hand`. One finding, handed over by triage
with the grade *unclear*: door *unlisted — every door an unreadable `*.md` at a managed home reaches*, the
clause it is said to break `working-product`, and for a repro block the heading *The consumers — enumerated,
and what each does with an entry it cannot read* of
`verify-p2-r1-doc-list-unreadable-entry-undriven-shapes-and-consumers.a1.md` — no block. Triage asked for: the
doors derived from the call graph of the fourteen `committed_instances` call sites and of the two read sites
outside it (`crates/engine/src/target_surface.rs:319`, `crates/cli/src/ingest.rs:991`); each door that
report's list lacks driven over the dangling link and the mode-000 file, with a no-plant control; any exit-0
write, commit or removal; and the block the finding lacks.

## Verdict

- **verdict:** `refuted` — **as a blocker**. What it rests on is real and stays a row of the ledger.
- **basis:** `breaks-no-clause` — the call graph of the sixteen sites reaches **15 of the 48 leaf verbs, and
  they are exactly the 15 the hand trace named**; the **22 leaf verbs that report did not drive** answer, over
  both plants and on both binaries, as they do with no plant, and none writes, commits or removes the plant;
  the one door outside the registry that the graph reaches and nobody had driven — the `pre-commit` hook
  `jigc setup` installs — **does** change under the mode-000 file, the same way on `1.0.0-rc.24`, and prints no
  refusal. It is **not** `does-not-reproduce`: the hook cell is a real effect of the plant at a door the list
  lacked. It is **not** `intended` for that cell: `design/validation.md` says such a commit *must* block.
- **contested:** `false`. The reading of the clause the verdict rests on is the one the report this finding
  grew from stated; it is restated under *Step 3* with what the other reading returns.
- **regression:** not returned (it goes with `confirmed` only). The fact was established all the same, because
  the clause's first half is a comparison of the two binaries: 249 paired cells, below.
- **class:** the mechanism's consumers **were enumerated, mechanically, and the count is given**: 14 functions
  hold the 16 sites; their reverse closure over a name-resolved call graph is 79 functions; 15 of the
  registry's 48 leaf verbs enter it. The bound of that method is stated under *The derivation*. Everything
  outside the 16 sites is `instance, unbounded` — four doors turned out to have read sites of their own.
- **platform:** **macOS only** — macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157), a user that is not root.
  Nothing was driven on Linux, and a mode-000 file refuses nobody who may read any file.

**What the drive found, in six lines.**

1. **The hand trace was complete at the level of leaf verbs.** Derived: `start`, `workflow`, `ingest`,
   `migrate`, `unmanage`, `rename`, `validate`, `doc show`, `doc list`, `task validate`, `task amend`,
   `task finalize`, `milestone execute`, `milestone join`, `milestone finalize`. That report's list: the same.
2. **The 22 leaf verbs it did not drive are unmoved by the plant.** 200 cells with a plant in place, each
   beside a control; 194 have the control's exit status, stdout and stderr; 4 differ in one stdout line that
   names the plant *left-out*; 2 are line 3.
3. **The installed `pre-commit` hook loses its one hard block.** With `docs/research/locked.md` at mode 000, a
   `git commit` that stages a bare `git mv` of a managed doc exits **0**; with no plant, and with the dangling
   link, it exits **1** under *out-of-band managed-doc rename staged in this commit … (commit blocked)*. Both
   binaries.
4. **Nothing of the user's is written, committed or removed at exit 0 by a jigc command on the candidate.**
   The plant was in place, unchanged, after every cell that did not address it; where a command's write lands
   on the plant's own path the candidate refuses with a code and a route, or parks the entry and says so.
5. **Every route such a refusal prints was run as printed and delivers** — four kinds, six runs.
6. **One route seen on the way does not** — `finalize.nothing-staged`'s `git add`, over any untracked file
   that cannot be read, at a managed home or not. It is at a door the hand trace *did* name, it needs no
   managed home, and it is *Left open* 2, with its block.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`).
- Before every sequence the driven binary's directory went first on `PATH` and `command -v jigc` was held to
  `<that directory>/jigc`; a rig was held to `$JIGC` being that same file (the driver exits 97 otherwise; it
  never did). No `cargo build`; nothing under `target/` was driven. Every rig was built with
  `dev/jigc-rig <state> --binary <that binary>`, stdout captured alone, the build's status read before the
  `eval`.
- The clone, after the drives: `git status --porcelain` reads `?? completions/artifacts/canary-one/r1/`, `HEAD`
  eeffe347, branch `fix/canary-one`. Nothing was built, edited, staged or committed there.

**What I read, said plainly.** The one report my prompt handed me, whole. Of triage's file, the one row a
search for this finding's key returned — its grade and the sentence the prompt already quotes. No other
verifier's report; their file names appeared in a directory listing and none was opened. The helper scripts
are in my own scratch directory and were written through the shell; the file tool wrote this report and
nothing else.

## The derivation — the doors, from the call graph

**The sites.** `grep -rn committed_instances crates/cli/src crates/engine/src` gives the definition
(`crates/engine/src/index.rs:789`) and the same **14 call sites** the earlier report lists; with the two read
sites outside the enumerator that makes 16 sites, in **14 functions** (`orphan.rs:352` and `:378` share
`prior_home_instances`; `target_surface.rs:274` and `:319` share `enumerate_committed_surface`).

**The graph.** A script of mine (`<scratch>/verify-p3-doors.6wDWZp/tools/callgraph.py`) reads the 72 `.rs`
files under `crates/cli/src` and `crates/engine/src` with comments and string literals blanked, takes every
`fn` outside a `#[cfg(test)]` module or a `#[test]` item — **2,475 functions** — and draws an edge for every
call or function-valued reference it can name: a path-qualified one to the module or the `impl` it names, a
bare one to the same file first and to the crate otherwise, a method call to **every** `impl` function of that
name. **12,807 edges.** The reverse closure of the 14 functions is **79 functions**.

**One edge was removed by hand, and it is named:** `render::code_fence` → `run`. `code_fence` has a local
variable called `run`; the tool took it for a function value and so reached `ingest::run`, which put
`migrate-corpus` and three other doors behind `read_candidate_bytes` through a route that prints a commit
rejection. With `CG_EXCLUDE=code_fence>run` the edge is gone; the first output is kept beside the second.

**The check on the closure.** Every token outside test code that spells the name of a function of the closure,
and whose containing function is *not* in the closure, was listed — **162** — and read: 115 are a module's
name inside a path (`engine::validate::…`, `engine::finalize::…`, `crate::validate::…`, `crate::finalize::…`),
14 are `use` or `pub mod` lines, 7 are calls of a *different* function of the same name (`describe::run`,
`setup::run`, `relocate::run`, `migrate_corpus::run`, `doc_code_probe::run`, the `config` module's own
`run_list`, `migrate_corpus`'s own `migrate_in_repo` — each a separate definition, checked with `grep`), and
26 are a struct field, a parameter or a local called `run`, `home`, `finalize`, `validate` or
`prior_home_instances`. **None is a call into the closure.**

**The entries.** Each of the registry's 48 rows (`VERB_KINDS`, `crates/cli/src/cli.rs:2033`) was mapped to the
function its `dispatch` arm calls, by reading the five `match` blocks (`Cli::dispatch`, and the `doc`, `task`,
`config` and `milestone` dispatchers); `start` has six arms and `workflow` two. The shared code in front of
every arm — `refuse_on_posture`, `invocation_log::log_invocation`, `invocation_log::operational_failure`,
`main`'s `probe_intercept`, `doc::served_from_home_note` — reaches none of the 14.

| reaches at least one of the 14 functions | leaf verbs |
|---|---|
| yes — **15** | `start` (five of its six arms; `--explain` reaches none) · `workflow` (both arms) · `ingest` · `migrate` · `unmanage` · `rename` · `validate` · `doc show` · `doc list` · `task validate` · `task amend` · `task finalize` · `milestone execute` · `milestone join` · `milestone finalize` |
| no — **33** | `setup` · `uninstall` · `upgrade` · `migrate-corpus` · `relocate` · `describe` · `doc create` · `doc add-item` · `doc remove-item` · `doc retitle-item` · `doc rename` · `doc set-field` · `doc set-slot` · `doc author` · `doc schema` · `task list` · `task diff` · `task discard` · `task bind` · the eight `config` verbs · `milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone list-tasks` · `milestone provision` · `milestone discard` |

The 15 are the earlier report's list, verb for verb. Of the 33, that report drove 11 as *unchanged by either
plant* (`describe`, `doc set-field`, `doc set-slot`, `doc schema`, `task list`, `task discard`, `milestone
create`, `add-task`, `list-tasks`, `provision`, `discard`). **The other 22 are the doors its list lacks**, and
they are what was driven here.

**Two doors the registry does not hold.** `jigc setup` installs two hooks. The `SessionStart` hook runs `jigc
start`, which that report drove. The git `pre-commit` hook runs `jigc validate --format json` and reads its
stdout — the door of line 3.

**The bound of the method.** Names, not types: a call the tool cannot spell — through a macro's expansion, or a
function stored in a field and called through it — is not an edge. None was seen in the 162. And the
derivation bounds the **16 sites**, not the class: `migrate-corpus`, `relocate`, `config set` of a root knob
and `doc create` each read or write a managed home through code of their own
(`crates/cli/src/migrate_corpus.rs:736`, `:2605`; `crates/cli/src/relocate.rs:791`, `:860`), which no trace of
`committed_instances` finds. They were reached here because the 22 were driven from the registry, not from
the graph.

## Step 1 — driven from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p3-doors.6wDWZp` (written `<W>`). Every root is under `<W>/roots`, minted by `mktemp -d` —
by `dev/jigc-rig` (`TMPDIR` pointed there) or by the driver. **One root per sequence, per plant, per binary**;
no root of any other reporter was read or reused.

Environment of every invocation: `HOME` the root's own `home/`, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables, stdin
from `/dev/null` or from a file where the argv reads `-`. Around each invocation, in this order: a manifest of
every entry under the root (kind, mode, size, sha256 or link target); the plant's own entry (kind, mode, size,
inode, mtime); `git status --porcelain --untracked-files=all`; the invocation, stdout and stderr each to a
file, **the exit status read from the bare command**; the three again, and `HEAD` with the commit count. No
pipe stands between a driven command and its exit status, and no output was cut.

**The plants**, as the finding has them:

- **L** — `ln -s nowhere-target docs/research/dangling.md`: an untracked dangling link at a located home.
- **M** — `docs/research/locked.md` holding `# x`, then `chmod 000`: an untracked file nobody may read. In
  every rig where it was still in place at the end, `cat` of it exits 1, *Permission denied* (29 of 29).
- **N** — no plant: the control of every sequence.

**The tally.** Through the driver, counted as evidence: **281 invocations on the candidate, 249 on the previous
release** — 237 / 31 / 11 / 2 at exit 0 / 1 / 3 / 128 on the candidate, 216 / 23 / 8 / 2 on the previous
release (the two 128s are `git add`). The 249 are paired one to one with candidate cells; the other 32
candidate cells are the routes of three refusals the previous release does not print. 102 roots. **Kept and
not counted:** one probe rig in which the argv of each door was found, and eight roots (24 cells per binary) of
a first pass of the last sequence, driven under a task id the mint did not give (`an-empty-task`; the mint
printed `empty-task`) — every cell of it answers `finalize.no-task`.

### The 22 doors, the plant beside them

Rig `refs-post-hoc` (five committed docs, one live task) unless said; the plant goes in before the first cell
unless said. Exit statuses, N / L / M; **the previous release has the same in every cell of this table**.

| door | argv as driven (the rig's task and the minted ids as they print) | N | L | M |
|---|---|---|---|---|
| `setup` | `jigc setup` · `jigc setup --format json` | 0 | 0 | 0 |
| `upgrade` | `jigc upgrade` · `jigc upgrade --format json` | 0 | 0 | 0 |
| `config get` | `jigc config get docs-root` · `jigc config get invocation-log` | 0 | 0 | 0 |
| `config list` | `jigc config list` | 0 | 0 | 0 |
| `task diff` | `jigc task diff <task>`, three tasks | 0 | 0 | 0 |
| `doc create` | `jigc doc create research --title Caches --task study-caches` · `jigc doc create decisions-log --title "Decisions Log" --task decide-on-a-cache` (copies the committed log in) | 0 | 0 | 0 |
| `doc rename` | `jigc doc rename research:caches --to "Cache study" --task study-caches` | 0 | 0 | 0 |
| `doc add-item` | `jigc doc add-item decisions-log:decisions-log#entries --title "Use an LRU cache" --task decide-on-a-cache` | 0 | 0 | 0 |
| `doc retitle-item` | `jigc doc retitle-item decisions-log:decisions-log#entries/use-an-lru-cache --title "Use an LFU cache" --task …` | 0 | 0 | 0 |
| `doc remove-item` | `jigc doc remove-item decisions-log:decisions-log#entries/use-an-lru-cache --task …` | 0 | 0 | 0 |
| `doc author` | `jigc doc author decisions-log --from-file <payload> --task …` · `jigc doc author spec --from-file <payload> --task plan-the-cache`, then that task's `task finalize` | 0 | 0 | 0 |
| `task bind` | a spec committed first, then the plant: `jigc task bind spec spec:cache-spec build-the-cache` | 0 | 0 | 0 |
| `milestone add-from-spec` | the same rig: `jigc milestone create "Wave one"`, `jigc milestone add-from-spec wave-one spec:cache-spec`, `jigc milestone list-tasks wave-one` | 0 | 0 | 0 |
| `config set` | `jigc config set invocation-log true` | 0 | 0 | 0 |
| `config fork` | `jigc config fork workflow:quick-fix#locate` | 0 | 0 | 0 |
| `config insert-step` | `jigc config insert-step --workflow quick-fix --after locate <root>/my-note.yaml` | 0 | 0 | 0 |
| `config replace-step` | `jigc config replace-step workflow:quick-fix#author-commit <root>/my-commit.yaml` | 0 | 0 | 0 |
| `config remove-step` | `jigc config remove-step workflow:quick-fix#implement-quick` | 0 | 0 | 0 |
| `config fill` | `jigc config fill step:implement#extra-guidance --from-file <file>` | 0 | 0 | 0 |
| `config set`, the relocating knob | `jigc config set docs-root documentation/` — moves the committed `docs/research/context-loss.md`; the plant stays where it is | 0 | 0 | 0 |
| `uninstall` | over the live task: `jigc uninstall` (`uninstall.staged-prose`, with and without a plant) | 1 | 1 | 1 |
| `uninstall` | `jigc uninstall --force` · after `jigc task discard <task> --force`: `jigc uninstall` | 0 | 0 | 0 |
| `migrate-corpus` | a fresh root: `git init`, one commit, `jigc setup`, the version-1 changelog of `crates/cli/tests/corpus_migration.rs` committed at `docs/changelog/changelog.md` — a corpus that folds: `jigc migrate-corpus --dry-run`, `jigc migrate-corpus` (*1 migrated*, one commit, `R docs/changelog/changelog.md → CHANGELOG.md`), again (*1 already current*) | 0 | 0 | 0 |
| `relocate` | the fixture of `crates/cli/tests/relocate.rs`: a manifest-less `note` doctype, one committed note at `docs/legacy-notes/`; the plant at the doctype's home, `docs/notes/dangling.md` or `docs/notes/locked.md`: `jigc relocate note --from docs/legacy-notes/` (*1 moved, 0 displaced, 0 blocked*), again | 0 | 0 | 0 |

Ten sequences of 55 cells. Five cells of one sequence run before the plant goes in, so 50 run with it: over L
and M, on both binaries, **200 cells with a plant in place.** Against the control of the same binary, after
replacing each root's name and each abbreviated commit id:

- **194** have the control's exit status, stdout and stderr.
- **4** differ in stdout only — the one `task finalize` of the table, L and M, both binaries: the same commit,
  and `left-out (unstaged/untracked — git add to include): docs/research/<plant>` printed twice.
- **2** differ in exit status — the hook cell below, M, both binaries.

And the tree: in **196 of the 200** the set of paths a cell created, removed or changed, `.jigc/` included, is
the control's; in the other 4 (`relocate`, L and M, both binaries) the control also creates the directory
`docs/notes`, which the plant's rig already had. **No cell touched a plant**: kind, mode, size, inode and
mtime are the same before and after each of the 200, and at the end of each of the 40 rigs the link reads
`-> nowhere-target` or the file is mode 0 and 4 bytes. `git log --name-status` of every one of those rigs
names no plant path.

### The hook — the door outside the registry

Rig `refs-post-hoc`; `jigc setup` has installed `.git/hooks/pre-commit`, which names the driven binary by its
absolute path. Two commits through it:

| step | N | L | M |
|---|---|---|---|
| `git add note.txt`, `git commit -m "add a note"` | 0 | 0 | 0 |
| `git mv docs/research/context-loss.md docs/research/renamed-loss.md`, then `git commit -m "bare rename of a managed doc"` | **1** | **1** | **0** |

Under N and L, stderr is the hook's one line — *jigc: out-of-band managed-doc rename staged in this commit — a
bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).* — and `HEAD` does
not move. Under M the commit is made: `1 file changed … rename docs/research/{context-loss.md =>
renamed-loss.md} (100%)`, stderr empty. **The same on the previous release**, byte for byte.

Why, driven in a second sequence (the rename staged, then the hook's own command by hand):

| `jigc validate --format json`, the rename staged | exit | stdout | stderr |
|---|---|---|---|
| N, and L | 1 | 2,417 bytes — the envelope: one `reconciliation.rename` finding at `docs/research/context-loss.md`, `blocking_probes` naming `reconciliation` | empty |
| M | 1 | **0 bytes** | `{ "error": "validating the committed store at \"<root>/repo\": Permission denied (os error 13)" }` |

The hook captures stdout and discards stderr (`report="$("$jigc" validate --format json 2>/dev/null)"`), and
its header says what follows: *If jigc is absent or fails to run, `report` is empty and nothing below matches
-> silent exit 0.* After the commit, `jigc validate` exits 1 with the same one line, so the sweep that would
name the out-of-band rename cannot run either, until the file is readable or gone.

### Where the plant stands on the path a door writes

Four of the 22 doors can be pointed at the plant's own path. That is past the finding's state — a plant
*beside* the docs — and it is where an exit-0 write or removal would be, so each was driven once more, the
plant *at* the destination. `D` marks a cell that differs from the previous release.

| door, and where the plant is | plant | candidate | previous release |
|---|---|---|---|
| `jigc doc create research --title Dangling --task <task>`, then the doc authored and `jigc task finalize` | L | create 0; finalize **3** `store.home-not-regular-file`, routed; the link stands, nothing is committed | create 0; finalize **0** — the doc is written through the link into `docs/research/nowhere-target`, which the commit then leaves out `D` |
| `jigc doc create research --title Locked --task <task>` | M | **1** — `could not read the existing research:locked: Permission denied (os error 13)`; no code, no route, the address and not the path | the same line |
| the three `jigc doc set-slot research:locked#… --task <task>` after it | M | 1 each — `could not copy … in for editing — nothing was staged: Permission denied (os error 13)` | 1 each — `could not read committed …: Permission denied (os error 13)` `D` (the sentence only) |
| `jigc config set docs-root documentation/`, the plant at `documentation/research/context-loss.md` | L, M | **1** `config.repoint-failed`, routed; the knob unchanged, the committed doc back at its home, the plant untouched | the same bytes |
| `jigc migrate-corpus` (and `--dry-run`), the plant at `CHANGELOG.md` | L | **1** — `blocked CHANGELOG.md`, `store.home-not-regular-file`, routed; nothing written | **0** — *1 migrated*; the link is replaced by the migrated file and committed `D` |
| the same | M | **1** — `blocked docs/changelog/changelog.md`, `migrate-corpus.destination-collision`, routed; nothing written | **0** — *1 migrated*; the 4-byte file nobody could read is overwritten and committed `D` |
| `jigc relocate note --from docs/legacy-notes/`, the plant at `docs/notes/cache-benchmarks.md` | L | 0 — *1 moved, 1 displaced*; the link itself is now `.jigc/displaced/cache-benchmarks.md`, still `-> nowhere-target`, and the ack names both paths | 0 — *0 moved, 1 blocked*, git's own *destination exists* `D` |
| the same | M | 0 — *1 moved, 1 displaced*; the file is now `.jigc/displaced/cache-benchmarks.md`, still mode 0 and 4 bytes | the same |

**On the candidate no door of this table destroys or overwrites the plant.** The three cells in which the
previous release did — a write through the link, and two overwrites by `migrate-corpus` at exit 0 — are the
ones the candidate now refuses.

### Every route those refusals print, run as printed

Candidate, each in a rig of its own, from the refusal on.

| refusal | the route, as printed | run | result |
|---|---|---|---|
| `store.home-not-regular-file` at `task finalize` (L) | *give this task's doc an id whose home is free (`jigc doc rename research:dangling --to "<title>" --task study-the-plant`) … then re-run `jigc task finalize study-the-plant`* | with `"Plant study"` | rename 0; finalize 0, `promoted docs/research/plant-study.md`; the link stands and is named *left-out* |
| the same | *or move the link out of the doc's home, so that `docs/research/dangling.md` is free; then re-run …* | `mv` to outside the repository | finalize 0, `promoted docs/research/dangling.md` |
| `store.home-not-regular-file` at `migrate-corpus` (L) | *move the link at `CHANGELOG.md` out of the way … then re-run `jigc migrate-corpus`* | as printed | 0 — *1 migrated*, one commit |
| `migrate-corpus.destination-collision` (M) | *make `CHANGELOG.md` readable (or move it out of the way if it is not this document), then re-run `jigc migrate-corpus`* | `chmod 644`: 1, a second refusal for a *different* document, with its own route; then moved out, as the parenthesis says | 0 — *1 migrated*, one commit |
| `config.repoint-failed` (L and M) | *Fix what this message names, then re-run `jigc config set docs-root documentation`* | unfixed: 1, the same refusal; the plant moved out, then as printed | 0 — the knob set, the doc moved (the same on the previous release, both plants) |

The refusals that print **no** route: `doc create` and `doc set-slot` over the mode-000 file (above). The
refusal that printed one I had not expected is *Left open* 2.

## Step 2 — is it what the finding says

The finding says one thing: the door list was a hand trace, so what a door outside it does is not known.

- **Against the list.** The call graph reaches the 15 leaf verbs the trace named and no sixteenth. The trace
  was right, and it is now a derivation with a count and a stated bound.
- **Against the doors outside it.** The 22 leaf verbs: nothing — the plant beside the docs changes no exit and
  no byte of output, except that a finalize names it as left out. The hook: the finding's suspicion holds
  there. The mode-000 file ends the hook's sweep before it can print, and the hook reads an empty report as
  *nothing to say*.
- **Against stale state, a pipe, a cut, another binary.** Every root was minted for this report and every
  sequence has a control built the same way; exit statuses are read bare and outputs are whole files; both
  hashes were asserted first and `PATH` held before each sequence.
- **Against a settled decision.**
  - The hook: `design/validation.md` → *The M19 pre-commit backstop stays doc↔code-keyed for content; M35
    adds a hard block on rename* — "a commit that itself stages a bare `git mv` of a managed doc **must**
    block". The paragraph before it (*Auto-firing the sweep (M19)*) says the hook is never driven by the exit
    code, and neither says what it does when the sweep cannot run. The hook's own comment does, for *jigc is
    absent or fails to run*. So the silence is written down in the script and nowhere ruled; the block that
    is ruled does not happen. **Not intended.**
  - The cells that differ from the previous release by refusing are intended, each by a ruling:
    `DECISIONS.md` → *2026-10-06 — One code for a managed home that is not an ordinary file* (the finalize
    cell; `design/finalize.md` → *4. Promote*, with its declared bound (4) for the create that still mints)
    and *2026-10-07 — `jigc migrate-corpus` refuses a doc whose home is not a regular file*, with
    `design/corpus-migration.md` on the destination ("an unreadable file answers under the same
    `migrate-corpus.destination-collision` … an entry that is no regular file … under
    `store.home-not-regular-file`").
  - `relocate` parking the entry is what `crates/cli/src/relocate.rs` says of itself at
    `displace_foreign_squatter` ("A link is a squatter like any other entry: it is parked below as the entry it
    is") and what `crates/cli/tests/relocate.rs` holds for a regular file. No bytes are lost and the ack names
    both paths.
  - No design section says what `doc create` owes an unreadable file at the slug's home; `design/finalize.md`'s
    bound (2) files "a read over an entry it cannot read" as "its own seam … filed with the review".

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We have a working product others can use
and relay on"; its instrument, "no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed"; and the rule, "A finding blocks the 1.0.0 call only if it breaks a clause
inside its scope; everything else is recorded with its tier." The run's opening declares no bound, and none is
used here.

**First half — no command that works on rc.24 stops working.** 249 paired cells, 747 files of exit status,
stdout and stderr, compared after replacing each root's name and masking abbreviated commit ids. **725 are
identical. 22 differ, in five groups:**

| files | cells | what differs | the plant's? |
|---|---|---|---|
| 6 | `uninstall --force` and `uninstall`, N, L and M | the warning's inventory of what `.jigc/` holds — one more tracked file and two more paragraphs on the candidate | no — the control differs the same way |
| 3 | the three `doc set-slot` after the refused create, M | the sentence of a code-less exit 1; exit 1 on both | wording only |
| 3 | `task finalize` of a doc created at the link's own slug, L | exit 0 → 3 | intended — the ruling of 2026-10-06 |
| 8 | `migrate-corpus` and its dry run, the plant at `CHANGELOG.md`, L and M | exit 0 → 1 | intended — the ruling of 2026-10-07 and `design/corpus-migration.md` |
| 2 | `relocate`, the plant at the destination, L | a `blocked` row → *1 moved, 1 displaced*; exit 0 on both | the code's stated intent |

In the 200 cells of the finding's own state — the plant beside the docs, at the 22 doors and the hook —
**nothing differs between the binaries**. The hook cell that exits 0 under M exits 0 on the previous release;
nothing that works there stops working. The five cells whose exit status changes are the ones in which the
previous release wrote through a link or over a file it had not read, and a ruling made each of them a
refusal. **Not broken.**

**Second half — every refusal's route works as printed.** At the doors the list lacked, the plant draws four
kinds of routed refusal, all with the plant on the path the door writes; each route was run as printed and
delivers (the table above). The refusals without a route have none to fail. The hook cell is **not a refusal**:
nothing is printed, so the instrument's second half has no sentence for it. **Not broken.**

*The reading this rests on, said so that it can be overruled.* It is the reading of the report this finding
grew from, and I kept it: *works as printed* is asked of a route in the state its refusal diagnoses, at the
door the finding is about. Two things would turn on another reading, and neither needs a second drive:

1. **The hook.** If *a working product others can … rely on* is read past its instrument — a guard that
   `design/validation.md` says must block, and that one unreadable file switches off without a word — then
   this finding is `confirmed`, with `regression: false`: the cell is byte-identical on `1.0.0-rc.24` (sha256
   accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d). The block is `Repro V-3b`.
2. **The `git add` route** (*Left open* 2). It is a printed route whose command fails. It is not counted
   against this finding for two reasons that are facts: it is printed by `jigc task finalize`, a door the hand
   trace named; and it fails the same way over an unreadable untracked file that is no `*.md` and stands at no
   managed home. It belongs to a row of its own, and under a reading on which a route that fails in any
   undeclared state breaks the clause, that row is the one it confirms — `regression: false`, byte-identical
   on the previous release. The block is `Repro V-3c`.

**The scope is not what the verdict leans on.** I did not excuse the mode-000 file as a planted state; whether
a tree holding a file its user cannot read is *a supported layout* the rule does not say.

**Verdict on the clause: `working-product` is not broken inside its scope by what a door outside the hand
trace does.** `refuted` as a blocker, basis `breaks-no-clause`.

## The coverage of what was driven

The finding carries no coverage claim. For the blocks' `pinned-by` only, from the suites at eeffe347: every
call under `crates/cli/tests`, `tooling-tests`, `crates/cli/src` and `crates/engine/src` that sets a mode was
listed (`from_mode`, `set_mode`, `set_readonly`, a spawned `chmod`, and the callers of the six helpers that
take the mode as an argument). Read permission is taken from six kinds of target: a `.jigc` directory, a
worktrees root, a task area, a task area's `docs` directory, a `locked.txt` inside a task area, and a task's
base pin. **None is a `*.md` at a doctype's home.** So no suite holds the mode-000 plant at any door. Whether a
suite holds a *dangling link beside* the docs at these 22 doors was not established:
`crates/cli/tests/home_shape_one_code.rs` plants links at a doc's own home, which is the other state.

## The repro blocks

### Repro V-3

```yaml
claim: "the door list of the unreadable-entry class was traced by hand, so a door outside it may do something with an unreadable `*.md` at a managed home that breaks the closing condition's `working-product` clause (ledger key r1-p3-unreadable-entry-class-door-list-traced-by-hand)"
verdict: REFUTED            # as a blocker — basis breaks-no-clause. The 22 leaf verbs the list lacked are unmoved; the hook is Repro V-3b.
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exit, stdout and stderr in every cell below, commit ids aside"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
setup:            # HOME a fresh directory; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1
  - fixture: refs-post-hoc                 # five committed docs, one live task
  - ["sh", "-c", "printf '# x\\n' > docs/research/locked.md && chmod 000 docs/research/locked.md"]
repro:            # one rig, this order — driven so
  - ["jigc", "start", "--workflow", "do-research", "study caches"]
  - ["jigc", "doc", "create", "research", "--title", "Caches", "--task", "study-caches"]
  - ["jigc", "doc", "rename", "research:caches", "--to", "Cache study", "--task", "study-caches"]
  - ["jigc", "start", "--workflow", "decided-task", "decide on a cache"]
  - ["jigc", "doc", "create", "decisions-log", "--title", "Decisions Log", "--task", "decide-on-a-cache"]
  - ["jigc", "doc", "add-item", "decisions-log:decisions-log#entries", "--title", "Use an LRU cache", "--task", "decide-on-a-cache"]
  - ["jigc", "doc", "retitle-item", "decisions-log:decisions-log#entries/use-an-lru-cache", "--title", "Use an LFU cache", "--task", "decide-on-a-cache"]
  - ["jigc", "doc", "remove-item", "decisions-log:decisions-log#entries/use-an-lru-cache", "--task", "decide-on-a-cache"]
  - ["jigc", "doc", "author", "decisions-log", "--from-file", "<payload>", "--task", "decide-on-a-cache"]   # sections: [{id: entries, items: [{title: Evict by recency, set: {why: "<<…>>"}}]}]
  - ["jigc", "task", "diff", "decide-on-a-cache"]
expect:
  - every: { exit: 0, stderr: "" }
  - stdout_first_line:
      - "task minted: study-caches"
      - "research:caches"
      - "research:cache-study (renamed to \"Cache study\" from research:caches)"
      - "task minted: decide-on-a-cache"
      - "decisions-log:decisions-log (already existed — copied in for update)"
      - "decisions-log:decisions-log#entries/use-an-lru-cache"
      - "retitled item decisions-log:decisions-log#entries/use-an-lru-cache to \"Use an LFU cache\" (anchor frozen)"
      - "removed item decisions-log:decisions-log#entries/use-an-lru-cache"
      - "decisions-log:decisions-log"
      - "# staged docs"
  - after: "docs/research/locked.md is still mode 000 and 4 bytes, the same inode; no path outside .jigc/ changed; HEAD unchanged"
controls: "the same ten with no plant, and with `ln -s nowhere-target docs/research/dangling.md` in place of the file: the same exits and, ids aside, the same bytes"
more-sequences:   # each a rig of its own, the plant first; every cell exit 0 under N, L and M unless said
  - "jigc setup · setup --format json · upgrade · upgrade --format json · config get docs-root · config list · task diff <rig task> · start --explain"
  - "start --workflow plan \"plan the cache\" · doc author spec --from-file <payload> --task plan-the-cache · the commit doc's type and summary · task finalize plan-the-cache  -> one commit, docs/specs/cache-spec.md; the plant printed as left-out"
  - "a spec committed, then the plant: start --workflow implement-from-spec \"build the cache\" · task bind spec spec:cache-spec build-the-cache · milestone create \"Wave one\" · milestone add-from-spec wave-one spec:cache-spec · milestone list-tasks wave-one"
  - "config set invocation-log true · config fork workflow:quick-fix#locate · config insert-step --workflow quick-fix --after locate <file> · config replace-step workflow:quick-fix#author-commit <file> · config remove-step workflow:quick-fix#implement-quick · config fill step:implement#extra-guidance --from-file <file> · config set docs-root documentation/"
  - "uninstall -> exit 1 uninstall.staged-prose in all three · uninstall --force -> 0, the plant standing"
  - "task discard <rig task> --force · uninstall -> 0, the plant standing"
  - "fresh root, `jigc setup`, a version-1 changelog committed at docs/changelog/changelog.md: migrate-corpus --dry-run · migrate-corpus (1 migrated, one commit) · migrate-corpus (1 already current)"
  - "the `note` fixture of crates/cli/tests/relocate.rs, the plant at docs/notes/: relocate note --from docs/legacy-notes/ (1 moved, 0 displaced, 0 blocked), twice"
the-plant-on-the-path-a-door-writes:   # candidate; the previous release differs where marked D
  - "doc create research --title Dangling, authored, task finalize, the link at docs/research/dangling.md -> exit 3 store.home-not-regular-file; route run as printed (doc rename … --to \"Plant study\", then finalize): 0 and 0   # D: rc.24 exits 0 and writes through the link"
  - "doc create research --title Locked, the file at docs/research/locked.md -> exit 1, `could not read the existing `research:locked`: Permission denied (os error 13)`, no route"
  - "config set docs-root documentation/, the plant at documentation/research/context-loss.md -> exit 1 config.repoint-failed; route (move it out, re-run as printed): 0"
  - "migrate-corpus, the plant at CHANGELOG.md -> exit 1; store.home-not-regular-file (link) or migrate-corpus.destination-collision (file); route (move it out, re-run): 0   # D: rc.24 exits 0 and replaces the plant"
  - "relocate note --from docs/legacy-notes/, the plant at docs/notes/cache-benchmarks.md -> exit 0, `1 moved, 1 displaced`, the entry now .jigc/displaced/cache-benchmarks.md, unchanged   # D for the link: rc.24 prints a blocked row at exit 0"
clause: "working-product — NOT broken: (a) 249 paired cells, 725 of 747 files identical, the 22 that differ accounted for in the report — none in the finding's own state; (b) every routed refusal's route delivers as printed. The reading (b) rests on is stated in the report."
observed: "<W>/out/{A1,A2,A3,A3S,A4,A5,A6,A7,A8,A8V,A9,B1,R1}-{N,L,M}-{cand,prev}/, <W>/out/{B2,R2}-{L,M}-{cand,prev}/, <W>/out/{RT1,RT1b,RT2}-L-cand/, <W>/out/RT3-M-cand/, <W>/out/RT4-{L,M}-{cand,prev}/"
pinned-by: "UNPINNED: no suite takes read permission from a `*.md` at a doctype's home (six kinds of target enumerated, none of them one); a dangling link beside the docs at these doors was not searched for"
```

**Pinnable as it stands: yes, for what it asserts** — a named fixture state, one filesystem call, literal
argv, exit statuses and first lines. What a converter needs to know:

1. **The `repro` list is one rig in the order driven**; each line of `more-sequences` is another rig. They
   were not driven joined.
2. **These are facts a fix must not break** — the plant beside the docs leaves these doors alone — so they pin
   as they are. The cells under `the-plant-on-the-path-a-door-writes` are the candidate's refusals; three of
   them are the fix pass's own and may already be held by `home_shape_one_code.rs` and the migration suites,
   which I did not read for it.
3. **The half the refutation rests on is a comparison of two binaries**, which no test of one binary holds.
4. **Platform edges.** Unix only; a mode-000 file refuses nobody who may read any file, so the M column is
   not reachable as root, a container that runs as root included.

### Repro V-3b — the hook, so that it is not re-derived

```yaml
claim: "with one `*.md` at a located home that cannot be read, the `pre-commit` hook `jigc setup` installs no longer blocks a commit that stages a bare `git mv` of a managed doc"
verdict: REFUTED            # as a break of `working-product` BY THIS FINDING — the effect is real; see Step 3 for the reading, and Left open 1
setup:
  - fixture: refs-post-hoc                 # `jigc setup` has installed .git/hooks/pre-commit, naming the binary by path
  - ["sh", "-c", "printf '# x\\n' > docs/research/locked.md && chmod 000 docs/research/locked.md"]
  - ["git", "mv", "docs/research/context-loss.md", "docs/research/renamed-loss.md"]
repro:
  - ["jigc", "validate", "--format", "json"]
  - ["git", "commit", "-m", "bare rename of a managed doc"]
  - ["jigc", "validate"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: "\"error\": \"validating the committed store at"
  - exit: 0                                # the commit is made
    stdout_contains: "rename docs/research/{context-loss.md => renamed-loss.md} (100%)"
    stderr: ""
  - exit: 1
    stdout: ""
    stderr_contains: "Permission denied (os error 13)"
control: "without the chmod line, and with `ln -s nowhere-target docs/research/dangling.md` in its place: validate --format json exits 1 with 2,417 bytes on stdout holding one `reconciliation.rename` finding; `git commit` exits 1, stderr `jigc: out-of-band managed-doc rename staged in this commit — … (commit blocked).`, HEAD unchanged"
also-on-previous-release: "byte-identical, all three plants"
design: "design/validation.md — 'a commit that itself stages a bare `git mv` of a managed doc **must** block'"
observed: "<W>/out/A8-{N,L,M}-{cand,prev}/, <W>/out/A8V-{N,L,M}-{cand,prev}/"
pinned-by: "UNPINNED: flow 37 holds the block with no plant (by the design text; the suite was not read)"
```

Pinnable as it stands, with one condition a converter must keep: the commit has to go through the hook the
rig's own `jigc setup` wrote, with `jigc` resolvable at the path that hook names.

### Repro V-3c — the `git add` route of *Left open* 2

```yaml
claim: "`finalize.nothing-staged` routes at `git add`, and over an untracked file that cannot be read the route does not deliver"
verdict: REFUTED            # as a break BY THIS FINDING — the door is one the hand trace named, and the state needs no managed home; left open as a row of its own
setup:
  - fixture: refs-post-hoc
  - ["sh", "-c", "printf 'x\\n' > scratch.bin && chmod 000 scratch.bin"]     # at the repository's root; no `*.md`, no managed home
  - ["jigc", "start", "--workflow", "do-research", "study the plant"]
  - ["jigc", "doc", "set-field", "commit:study-the-plant#header/type", "--value", "docs", "--task", "study-the-plant"]
  - ["jigc", "doc", "set-slot", "commit:study-the-plant#summary", "--from-file", "-", "--task", "study-the-plant"]   # stdin: "record nothing"
repro:
  - ["jigc", "task", "finalize", "study-the-plant"]
  - ["git", "add", "scratch.bin"]
  - ["jigc", "task", "finalize", "study-the-plant"]
expect:
  - exit: 3
    stdout: ""
    stderr_contains: "finalize.nothing-staged — you staged nothing — the working tree has changes but the index is empty"   # route: "`git add` your changes, then re-run `jigc task finalize`"
  - exit: 128
    stderr_contains: "error: unable to index file 'scratch.bin'"
  - exit: 3
    stderr_contains: "finalize.nothing-staged"
the-same-with: "docs/research/locked.md (mode 000) in place of scratch.bin — 3, 128, 3"
controls: "a readable scratch.txt: 3, then `git add` 0, then finalize 0 (`added scratch.txt`, one commit) · no untracked file at all: 3 `finalize.empty-commit`, another refusal with another route"
also-on-previous-release: "byte-identical, all four"
observed: "<W>/out/NS2-{N,R,X,M}-{cand,prev}/   (and <W>/out/NS-*: the voided first pass)"
pinned-by: "UNPINNED: not searched"
```

## What was driven, and what was not

- **Driven:** the call graph of the 16 sites over both crates, with its closure checked by name; each of the
  22 leaf verbs the earlier list lacked, over the dangling link and the mode-000 file beside the docs, each
  with a no-plant control, on both binaries; the installed `pre-commit` hook through two commits, and its sweep
  by hand; the four doors that can write the plant's own path, once more with the plant there; every route the
  refusals of those cells print; one route seen on the way, one step, to say whose it is.
- **Not driven:** Linux, and root. `--format json` anywhere but `setup`, `upgrade` and the hook's sweep. The
  22 doors from a linked or fan-out worktree. A plant at a home of a doctype whose committed instances carry
  code anchors (the committed spec here has none set). A plant at `placement-root`'s knob, or a re-point of
  that knob. A tracked managed doc made unreadable. A directory or a named pipe called `x.md`. The staged arm
  (`--task`) of the read verbs. The second route `migrate-corpus` prints once the plant has been made readable
  (it is then a *different document* at the destination, and no longer an entry nobody can read). What
  `jigc uninstall` or `jigc uninstall --force` does with an entry `relocate` parked under `.jigc/displaced/`.
- **Not established:** that no door exists outside the registry and the two hooks — a second binary, a script
  an adapter installs — and that no suite pins the dangling-link half.
- **Nothing was fixed, graded or decided.**

## Left open — seen on the way, not pursued

1. **The `pre-commit` hook's rename block is lost, without a word, while one `*.md` at a located home cannot
   be read.** `git commit` of a staged bare `git mv` of a managed doc: exit 1 in the control and over the
   dangling link, exit 0 over the mode-000 file; the hook discards the sweep's stderr and takes its empty
   stdout for a clean report. `design/validation.md` says the commit must block. Both binaries. `Repro V-3b`.
2. **`finalize.nothing-staged`'s route does not deliver over an untracked file that cannot be read.** The
   route is `git add` your changes; git answers 128, *unable to index file*, and the re-run repeats the
   refusal. Any such file — `docs/research/locked.md` and a `scratch.bin` at the root alike. Both binaries.
   `Repro V-3c`.
3. **`jigc doc create` over a mode-000 file at the slug's home: exit 1, one line, no code, no route, the
   address and not the path** — and the same shape from each `doc set-slot` at that address afterwards, in
   words that differ between the binaries. The task is left holding no doc, which is how line 2 was reached.
4. **`jigc relocate` moves an untracked entry it finds at a destination into `.jigc/displaced/`** — a dangling
   link and an unreadable file included, named in the ack. For the link the candidate does this where
   `1.0.0-rc.24` printed a `blocked` row; `design/finalize.md` words the store's rule as "neither writes
   through a link nor moves one", and `relocate.rs` says the opposite of this door on purpose. The two
   sentences were not reconciled here.
5. **After line 1's commit the store sweep cannot say what happened**: `jigc validate` exits 1 on the
   unreadable file before it can report the out-of-band rename it would otherwise name.
6. **Linux and root** — still driven by nobody for this class.

## Where the evidence is

`<W>` is `<scratch>/verify-p3-doors.6wDWZp`. Per sequence, plant and binary:
`<W>/out/<sequence>-<plant>-<cand or prev>/<NN>-<label>.{argv,rc,stdout,stderr,man.before,man.after,mandiff,plant.before,plant.after,porc.before,porc.after,head.after}`,
with `root` (the root it ran in), `final.log` (`git log --name-status`) and `final.plant`. The roots:
`<W>/roots/`. The derivation: `<W>/callgraph.paths2.txt` and `<W>/callgraph.closure2.txt` (the first outputs,
with the false edge, beside them without the `2`). The comparisons: `<W>/analysis3.txt`. The tools:
`<W>/tools/{callgraph.py,seeds.json,entries.json,lib.sh,drive.sh,runall.sh,snap.py,mandiff.py,analyze.py,pathsets.py}`
and the payload and stdin files beside them. Nothing was torn down, and the one file written for the
repository is this report.

<!-- end of report -->
