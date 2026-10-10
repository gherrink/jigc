# verify-real — `r1-doc-list-unreadable-entry-undriven-shapes-and-consumers` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p2-r1-doc-list-unreadable-entry-undriven-shapes-and-consumers`. One finding, handed over by
triage with the grade *unclear*: door `jigc doc list`, the clause it is said to break `working-product`, and
for a repro block the two bullets *Not enumerated* and *Not driven* of
`verify-p1-r1-doc-list-unreadable-entry-fails-listing.a1.md` — no block. Triage asked for: the listing over a
dangling link at a placement home; the orphan-row read over an entry that cannot be read; the other consumers
of `engine::index::committed_instances` and of the read at `crates/cli/src/doc.rs:4866`, each door they reach
driven over the same plant; any exit-0 write, commit or destruction; any exit that differs from the previous
release; every printed route run as printed.

## Verdict

- **verdict:** `refuted` — **as a blocker**. The defect is real, and it is wider than the row that named it:
  it stays a row of the ledger.
- **basis:** `breaks-no-clause` — over 277 cells driven on both binaries, every exit status, stdout and stderr
  is the same on `1.0.0-rc.24` as on the candidate (the only differing bytes are abbreviated commit ids), so
  no command that works there stops working; the refusals the plant itself causes print no route; and no door
  writes, commits or destroys anything of the user's at exit 0. It is **not** `does-not-reproduce` for the
  finding as a whole — two of the shapes triage named do not fail at all, the rest fail as described — and it
  is **not** `intended`: no settled decision says one unreadable entry should end a listing, an adoption scan,
  a store sweep or a commit boundary.
- **contested:** `false`. **One reading of the clause carries this verdict and is the human's to overrule** —
  it is stated under *Step 3*, with what the other reading would return, so that overruling it needs no
  second drive.
- **regression:** not returned (it goes with `confirmed` only). The fact was established all the same:
  **not a regression** — 277 paired cells, none differing.
- **class:** the mechanism's **call sites were enumerated — 14, by `grep`** (below); the **doors** they reach
  were derived by tracing callers by hand, which no registry fences, so at the level of doors this is
  `instance, unbounded`. And the class is **not bounded by the mechanism**: two of the read sites that end a
  verb here are not consumers of `committed_instances` at all (below).
- **platform:** **macOS only** — macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157), a user that is not root.
  Nothing was driven on Linux.

**What the drive found, in five lines.**

1. A **dangling link at a placement home** (`CHANGELOG.md`, `VISION.md`, `docs/roadmap.md`) does **not** fail
   the listing: exit 0, *no committed docs*, in both formats — the enumerator asks `path.exists()`, which is
   false for a link whose target is absent. A **mode-000 file** at the same home does: exit 1, the one line.
2. The **orphan-row read** (`doc.rs:4953`) **cannot be reached** by an entry that cannot be read: the orphan
   enumerator admits a path only after reading its stamp, so an unreadable stamped orphan is not an orphan to
   it. The row leaves the listing at exit 0, and the store sweep stops naming the file.
3. **Other doors end on the same plant.** `jigc ingest` exits 1 over the dangling link and over the mode-000
   file. `jigc validate` exits 1 over the mode-000 file. And — the widest cell — with the mode-000 file
   present, **a task that holds code cannot pass `jigc task validate` or `jigc task finalize`, and a milestone
   with a contributing sub-task cannot pass `jigc milestone finalize`**: exit 1, one line that names the task
   directory and not the file, no route.
4. **Nothing is written, committed or destroyed that the user did not ask for.** Every plant was in place,
   unchanged, after every drive (48 rigs checked); every commit a door made holds only the paths its task
   asked for; `task finalize` names the plant as *left-out*.
5. **Every cell is the same on the previous release.**

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`).
- Before every invocation the driven binary's directory went first on `PATH` and `command -v jigc` was held to
  `<that directory>/jigc` (the driver exits 97 otherwise; it never did). No `cargo build`; nothing under
  `target/` was driven. Every rig was built with `dev/jigc-rig --binary <that binary>`, stdout captured alone.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/`, `HEAD` eeffe347, branch
  `fix/canary-one`, after the drives. Nothing was built, edited, staged or committed there.

**A slip of mine, said plainly.** The first three helper scripts of this report (`snap.py`, `drive.sh`,
`mkfresh.sh`) were written with the file tool into my own scratch directory, before I had read back the
sentence that gives that tool one file. Every later script was written through the shell. None of them is in
the repository or the run's directory, and none was handed to the record script.

## Step 1 — driven from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p2-undriven.IJne7v` (written `<W>`). Every root under it was minted by `mktemp -d` — mine
directly, a rig's by `dev/jigc-rig` and then renamed in place to say what it holds. **Each shape, and each
door that writes, got a root of its own**; the read-shaped doors of one plant share one rig. No root of any
other reporter was read or reused.

Environment of every invocation: `HOME=<root>/home`, `GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a
synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables, stdin from `/dev/null` (a file
where the route as printed reads stdin). The driver, in this order: `git status --porcelain
--untracked-files=all --ignored`; `HEAD` and the commit count; a snapshot of every entry under the root (kind,
mode, size, mtime in nanoseconds, sha256 or link target); the invocation, stdout and stderr each to a file,
**the exit status read from the bare command**; the three again. No pipe stands between a verb and its exit
status, and no output was cut.

**The tally.** Through the driver: **292 invocations on the candidate, 277 on the previous release** — 207 /
76 / 9 at exit 0 / 1 / 3 on the candidate, 197 / 71 / 9 on the previous release. The 277 are paired one to one
with candidate cells. The other 15 candidate cells are first passes of three no-plant control sequences that I
ran again after correcting two of them (a `migrate` whose source I had not staged; a sub-task driven from the
main checkout instead of its worktree); they are kept and are not counted as evidence. 114 roots: 58
candidate, 55 previous release, one probe rig used to read `--help`.

**The plants.** In a fresh root (`git init`, one empty commit, `jigc setup`) or on the rig state
`refs-post-hoc` (five committed docs of five doctypes and one live task):

- **L** — `ln -s nowhere-target docs/research/dangling.md`: an untracked dangling link at a located home.
- **M** — `docs/research/locked.md` holding `# x`, then `chmod 000`: an untracked file nobody may read.
- **N** — no plant: the control of every sequence.

### A — the listing over a placement home (fresh roots)

| plant | argv | exit | stdout | stderr |
|---|---|---|---|---|
| none (control) | `jigc doc list` | 0 | `jigc doc list — no committed docs` | empty |
| `CHANGELOG.md` → `/nonexistent/nowhere.md` | `jigc doc list` | **0** | `jigc doc list — no committed docs` | empty |
| the same | `jigc doc list --format json` | **0** | `{ "docs": [] }` | empty |
| the same | `jigc doc list changelog` (and `--format json`) | **0** | ``no committed `changelog` docs`` / `{ "docs": [] }` | empty |
| the same | `jigc doc show changelog:changelog` | 1 | empty | `blocking · store.not-found — could not read … No such file or directory (os error 2)` and its route |
| the same | `jigc validate` | 0 | `no findings — the committed store validates clean` | empty |
| `VISION.md` and `docs/roadmap.md`, both dangling | `jigc doc list` (and json), `jigc validate` | **0** | the empty listing; *validates clean* | empty |
| `CHANGELOG.md` = `# Changelog`, mode 000 | `jigc doc list` | **1** | empty | `reading the committed doc at "<root>/repo/CHANGELOG.md": Permission denied (os error 13)` |
| the same | `jigc doc list --format json` | **1** | empty | the `{ "error": … }` envelope |
| the same | `jigc doc list changelog` | **1** | empty | the same line |
| the same | `jigc doc list vision` | 0 | ``no committed `vision` docs`` | empty |
| the same | `jigc validate` (and json) | **1** | empty | `validating the committed store at "<root>/repo": Permission denied (os error 13)` |

Tree IDENTICAL and porcelain unchanged around all 17 cells, on both binaries.

### O, P — the orphan-row read and the prior-home rows (fresh roots)

Two stamped files committed under `docs/archive/` (`old.md`, `kept.md` — inside the docs tree, at no doctype's
home: the state `crates/cli/tests/doc_list.rs` builds for its orphan row).

| state | `jigc doc list` | `jigc validate` |
|---|---|---|
| both readable (control) | 0 — two rows, `(none)  docs/archive/kept.md  orphaned` and `… old.md  orphaned` | 1 — `schema-conformance.orphaned-instance` on both |
| `old.md` set to mode 000 | **0 — one row, `kept.md`; `old.md` is gone from the listing** (json: one object) | 1 — the finding on `kept.md` only |
| `old.md` replaced by a dangling link (the path still tracked) | **0 — one row, `kept.md`** | 1 — `kept.md` only |
| a dangling link git tracks at `docs/archive/link.md`, nothing else | 0 — *no committed docs* | 0 — *validates clean* |

So the read at `doc.rs:4953` is not reached by an unreadable entry: `crate::orphan::orphaned_instances` keeps a
path only where `carries_stamp` could read it (`crates/cli/src/orphan.rs:616-634`), and what it cannot read it
calls unstamped.

The prior-home rows chain into the read at `doc.rs:4866` (`.chain(stale_homes)`). With a readable
version-1 `changelog` at the retired folder home `docs/changelog/changelog.md` as control — listed as
`changelog:changelog  docs/changelog/changelog.md  managed`, exit 0 — a dangling link and a mode-000 stamped
file beside it change nothing: **exit 0, the same one row**, both formats.
`crate::orphan::prior_home_instances` reads each candidate itself and skips one it cannot read
(`orphan.rs:394`). The same two plants in eight other retired-looking folders: exit 0, *no committed docs*.

All 23 cells of O and P: tree IDENTICAL, both binaries.

### The consumers — enumerated, and what each does with an entry it cannot read

`grep -rn "committed_instances" crates/cli/src crates/engine/src` prints 39 lines: the definition
(`crates/engine/src/index.rs:789`), 24 lines of comment or test-fixture text, and **14 call sites**:

| call site | in | reads the yielded path? | on a read failure |
|---|---|---|---|
| `crates/cli/src/doc.rs:4862` | `run_list` | yes, `:4866` | **propagates** — exit 1 (the row's defect) |
| `crates/engine/src/target_surface.rs:274` | `enumerate_committed_surface`, placement arm | yes, `:279` | **propagates** |
| `crates/engine/src/index.rs:213` | `rebuild_committed` | yes | skips |
| `crates/engine/src/index.rs:612` | `mention_resolves_store` | yes | skips |
| `crates/engine/src/file_state.rs:1405` | `detect_committed_store` | yes | skips |
| `crates/engine/src/validate.rs:944` | `schema_conformance_store` | yes | skips |
| `crates/engine/src/validate.rs:1117` | `hollow_surplus_store` | yes | skips |
| `crates/cli/src/orphan.rs:378` | `prior_home_instances` | yes, `:394` | skips |
| `crates/engine/src/index.rs:560` | `inverse_cardinality_store` | no — identities only | — |
| `crates/cli/src/start.rs:3545` | `committed_store` (the `{{store.…}}` feed) | no — identities only | — |
| `crates/cli/src/orphan.rs:352`, `:608`, `:810` | the *claimed* sets of three enumerators | no — paths only | — |
| `crates/cli/src/cli.rs:1431` | `unbaselined_identities` | no — the record lookup only | — |

**Two of fourteen propagate.** And **two read sites that end a verb over the same plant are not in this
table**, because they do not go through the enumerator: `crates/engine/src/target_surface.rs:319` — the same
function's *located* walk, which keeps its own `read_dir` (its doc comment says so), skips what is not a
regular file and reads every regular `*.md` with `?` — and `crates/cli/src/ingest.rs:991`, `jigc ingest`'s
candidate read. The first is why `jigc validate` exits 0 over the link and 1 over the mode-000 file; the
second is why `jigc ingest` exits 1 over both. **A fix that bounds the class by `committed_instances` would
miss both.**

Doors those functions are reached from, by tracing callers in `crates/cli/src` (`doc.rs`, `cli.rs`,
`start.rs`, `task.rs`, `rename.rs`, `unmanage.rs`, `ingest.rs`, `migrate.rs`, `milestone.rs`): `doc list` ·
`doc show` (`run_show`, through `reroute_unadopted`) · `validate` · `start` in its three composing forms ·
`workflow --preview` and `--task` · `migrate` · `task amend` · `task validate` · `task finalize` · `rename` ·
`unmanage` · `ingest` · `milestone execute`, `join`, `finalize`. Every one was driven over N, L and M on both
binaries. That a given run executed a given call site is read from the call graph and, where a message names
it, from the output — a release binary carries no trace.

### B, X, Y — every door over the plant (rig `refs-post-hoc`), candidate; the previous release is the same

Only the cells in which a plant changes anything against the no-plant control:

| door | N | L (dangling link) | M (mode 000) |
|---|---|---|---|
| `jigc doc list` (and after a finalize, a rename, a milestone finalize; from a sub-task worktree) | 0, the rows | **1**, stdout empty | **1**, stdout empty |
| `jigc validate`, `--format json` | 0, one unrelated advisory | 0, the same bytes — the link is not named | **1** — `validating the committed store at "<root>/repo": Permission denied (os error 13)` |
| `jigc ingest`, `--format json` | 0, five adopted | **1** — `could not read the candidate at "<root>/repo/docs/research/dangling.md": No such file or directory (os error 2)` | **1** — the same line, `locked.md`, `Permission denied (os error 13)` |
| `jigc task validate <task>` — the task holds **code** (one staged file) | 0, *validates clean* | 0 | **1** — `validating task at "<root>/repo/.jigc/tasks/add-a-cache": Permission denied (os error 13)` |
| `jigc task finalize <task>` — the same task | 0, `cache.txt` committed | 0, `cache.txt` committed, the link named *left-out* | **1** — the same line; nothing committed, the task still listed |
| `jigc task validate <sub-task>`, from its worktree, code staged | 0 | 0 | **1** — the same line |
| `jigc milestone finalize wave-one`, one contributing sub-task | 0, `cache.txt` and the record committed | 0, the same | **1** — `validating the merged effective state under .jigc/milestones/wave-one/merged: Permission denied (os error 13)` |
| `jigc task finalize <task>` — a task holding a doc only | 0, `VISION.md` committed | 0, the same, the link named *left-out* | 0, the same, the file named *left-out* |
| `jigc doc show research:<plant>` | 1, `store.not-found` (nothing is there) | 1, the same finding | 1, `store.not-found — could not read … Permission denied (os error 13)` |
| `jigc migrate docs/research/<plant>.md --as research` | 1, *could not read the foreign source* | 1, `migrate.source-untrackable` (a symlink) | 1, *could not read the foreign source* |

**Unchanged by either plant** — the same exit and, commit ids aside, the same bytes as the control: `jigc
start` (bare, with an intent, `--workflow dev-task`, `--workflow do-research`, `--task`), `jigc workflow
dev-task --preview`, `jigc workflow sub-task --task`, `jigc task amend`, `jigc task validate` on a doc-only
task, `jigc task list`, `jigc task discard --force`, `jigc rename research:context-loss --to …`, `jigc unmanage`
(of the sibling doc; of the plant itself: *no-op … is not managed*), `jigc migrate` of another source, `jigc
milestone create`, `add-task`, `list-tasks`, `execute`, `join`, `provision`, `discard`, the zero-contribution
refusal of `milestone finalize`, `jigc doc show` of a sibling, `jigc doc schema`, `jigc describe`, and the
`doc set-field` / `doc set-slot` writes of a commit doc.

**The five operational-error lines the plant draws, whole.** Each is one line on stderr (three for
`--format json`, the single-key `{ "error": … }` envelope), stdout 0 bytes, no finding code, no route, no
command:

    reading the committed doc at "<root>/repo/docs/research/locked.md": Permission denied (os error 13)
    could not read the candidate at "<root>/repo/docs/research/locked.md": Permission denied (os error 13)
    validating the committed store at "<root>/repo": Permission denied (os error 13)
    validating task at "<root>/repo/.jigc/tasks/add-a-cache": Permission denied (os error 13)
    validating the merged effective state under .jigc/milestones/wave-one/merged: Permission denied (os error 13)

The last three do not name the file that cannot be read.

### Does any door write, commit or destroy at exit 0

**No.** Held four ways:

- **The plants.** After every drive, in all 48 rigs that hold one (24 L, 24 M; both binaries): the link still
  reads `-> nowhere-target`, the file is still mode 000 and 4 bytes.
- **The commits.** `git diff --name-only <the rig's last commit> HEAD` in every rig a committing door ran in:
  `VISION.md` (doc-only finalize) · `cache.txt` (code finalize) · `docs/research/context-loss-renamed.md`
  (rename) · `docs/milestone-records/wave-one.md` (milestone create, add-task, discard) · `cache.txt` and the
  record (milestone finalize). `git log -- <the plant>` is empty in each. Under M the two finalize doors that
  refuse commit nothing: `HEAD` and the commit count are the same before and after.
- **`task finalize` says what it leaves.** Twice in its output: `left-out (unstaged/untracked — git add to
  include): docs/research/dangling.md`.
- **What a refusing door leaves behind** is jigc's own and ignored: `jigc ingest` and `jigc task validate`
  write the rebuildable `.jigc/index/edges.json` before they exit 1; `jigc milestone finalize` under M leaves
  `.jigc/milestones/wave-one/merged/docs/commit:add-a-cache.md` and one loose git object. The sub-task's
  staged `cache.txt` is still staged in its worktree.

### Every printed route, run as printed

The refusals the plant itself causes at `doc list`, `ingest`, `validate`, `task validate`, `task finalize` and
`milestone finalize` print **no route** (above). The routes that *were* printed in a state holding a plant,
each run as printed on both binaries:

| printed by | the route's command | as printed | result |
|---|---|---|---|
| `doc show research:<plant>` — `store.not-found` | `jigc task list` | yes | exit 0, names the live task |
| the same | `jigc doc show research:<plant> --task <task-id>` | with the id `task list` printed | exit 1, `store.not-staged` — true: the task stages no such doc |
| `task validate` (the commit doc unauthored — not about the plant) | `jigc doc set-field commit:<task>#header/type --task <task> --value <value>` | with `docs` | exit 0 |
| the same | `jigc doc set-slot commit:<task>#summary --task <task> --from-file -` | prose on stdin | exit 0; `task validate` then exits 0 |
| `milestone finalize` — `milestone.zero-contribution` (not about the plant) | `jigc milestone provision wave-one` | yes | exit 0 under N, L and M |
| the same | `jigc milestone discard wave-one` | yes | exit 0 under N, L and M |
| `migrate <plant M>` | re-run `jigc migrate <path> --as research` *with a readable file* | conditional | not a command for this file |
| `validate` — `schema-conformance.orphaned-instance` on `docs/archive/kept.md` | `jigc ingest` | yes | **control: exit 0. With the sibling `old.md` unreadable (mode 000, or a dangling link): exit 1**, `could not read the candidate at "<root>/repo/docs/archive/old.md": …` |
| the same | `jigc unmanage docs/archive/kept.md` | yes | exit 0 in all three (*no-op … is not managed*) |
| `validate` — `schema-version-current` on the prior-home changelog | `jigc migrate-corpus` | yes | exit 1 `migrate-corpus.fold-refused` — **the same bytes with and without the plants** (`cmp` exit 0; my hand-made version-1 file does not fold; not about the plant) |
| `doc show research:locked --task <task>` — `store.not-staged`, *only its committed copy exists* | `jigc doc show research:locked` | yes | **exit 1**, `store.not-found … Permission denied` — whose route is the `--task` read again |
| the same refusal over a **readable** file that is no research doc (`# x`, mode 644) | `jigc doc show research:plain` | yes | **exit 1**, `store.unparseable`, routed to adoption |
| the same refusal over a committed doc the task does not stage (control) | `jigc doc show research:context-loss` | yes | exit 0, the doc |

Two rows of this table carry a command that does not do what its route says. Neither is counted as this
finding breaking the clause; *Step 3* says why, and both are in *Left open*.

## Step 2 — is it what the finding says

The finding is a list of what nobody drove, with the suggestion that the row's defect reaches further. Shape
by shape:

- **A dangling link at a placement home** — *it does not fail.* Exit 0 and an empty listing, at three
  placement homes, both formats, both binaries. This is what `design/doc-read-surface.md` → *`jigc doc list` —
  the fourth read surface* describes: "`committed_instances` yields a **placement** doctype's instance on
  **mere file existence** … `path.exists()`". A link to nothing is no file.
- **The orphan-row read over an unreadable entry** — *it cannot happen.* The read is behind an enumerator that
  has already read the same file. What happens instead is a silent omission at exit 0, in the listing and in
  the sweep (*Left open*, 5).
- **The mechanism's other consumers** — *two of fourteen propagate; the other twelve skip or do not read.* The
  second propagating site ends `jigc validate` and, when a task holds code, the task and milestone commit
  boundaries — over a mode-000 file at a placement home through the enumerator, and at a located home through
  the sibling walk beside it.
- **Other verbs over the same state** — `jigc ingest` ends on either plant; the rest behave as they do with
  no plant.

Against stale state: every root was minted for this report, and every sequence has a no-plant control built
the same way. Against a pipe or a cut: bare exit statuses, whole files. Against another binary: the `PATH`
check before every invocation, both hashes first. Against a settled decision: the design section above owns
`doc list` and has no row and no sentence for an instance that cannot be read; `design/validation.md`,
`design/storage.md`, `design/project-setup.md` and `design/command-output-contract.md` were searched for
*unreadable*, *could not read*, *cannot be read* and *loud* and say nothing of an unreadable instance at
`ingest`, `validate` or a finalize. `design/command-output-contract.md` → *The two reject arms* settles the
**shape** an I/O failure takes — one `error` key, exit 1 — and every driven refusal has it. One code comment
leans the other way for one door: `enumerate_committed_surface` says a store-scope sweep "that cannot read a
managed location must be **loud**, never a clean-looking empty" — of a *directory* it cannot list; it is a
comment, not a decision, and it does not speak of the commit boundary. So: a real defect, not intended, wider
than the row.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, in `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We have a working product others can
use and relay on", its instrument "**no command that works on rc.24 in a supported layout stops working, and
every refusal's route works as printed**", and the rule "A finding blocks the 1.0.0 call only if it breaks a
clause inside its scope; everything else is recorded with its tier." The run's opening declares no bound, and
none is used here.

**First half — no command that works on rc.24 stops working.** Every sequence was driven on the previous
release (sha256 accf3996…) in roots of its own: 277 cells, 831 files of exit status, stdout and stderr,
compared after replacing each root's name. **797 files identical; 34 differ, each only in a seven-character
commit id** (a `finalized <id>`, a `record commit: <id>`, a `base: <id>`, an `amend-<id>` — a function of the
commit's timestamp); **0 differ otherwise.** Every cell that exits 1 over a plant on the candidate exits 1
with the same bytes on rc.24. Nothing stops working. **Not broken.**

**Second half — every refusal's route works as printed.** The refusals this finding is about — one unreadable
entry ending `doc list`, `ingest`, `validate`, `task validate`, `task finalize`, `milestone finalize` — print
no route, so none fails as printed. That is the reading the row this finding grew from was judged by, and it
is kept here.

*The reading this verdict rests on, said so that it can be overruled.* Two printed routes do not deliver in a
state that holds a plant:

1. The store sweep's route for a **different** file — an orphan at `docs/archive/kept.md` — names `jigc
   ingest`, and `jigc ingest` ends on the unreadable sibling. The route is right for the file it is about and
   works in the control; what fails is the door it names, for a second and independent fault, and that door's
   failure is the consumer cell judged under the first half (the same on rc.24). I read *works as printed* as
   a property of the route in the state its refusal diagnoses — not as a promise that the door it names
   survives every other fault in the tree. On the other reading, any route can be broken by planting a second
   fault beside it.
2. `store.not-staged` routes to the task-less read as one that "serves the committed copy", and that read
   refuses for the mode-000 file. **It refuses the same way for a readable file that is simply not a managed
   doc**, with no unreadable entry anywhere (the `plain.md` row above). So that route's over-promise is not
   this finding's defect: it is a property of `doc show --task` over any file the task-less read blocks on,
   and belongs to a row of its own.

**If either is read the other way** — a route that fails as printed in any state without a declared bound
breaks the clause — then the verdict is `confirmed` with `regression: false`: both cells are byte-identical on
`1.0.0-rc.24` (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d), and the blocks are
below. That is a ruling on what the clause's second half reaches, and it is not mine.

**The scope is not what the verdict leans on.** I did not excuse the mode-000 file as a planted state. A file
its user cannot read is an odd thing to find in a docs folder and an easy one to make; whether such a tree is
*a supported layout* the rule does not say.

**Verdict on the clause: `working-product` is not broken inside its scope.** `refuted` as a blocker, basis
`breaks-no-clause`. What stays true and should go onto the ledger with the row: one untracked file named
`*.md` that cannot be read, at a managed home, ends the listing, the adoption scan and the store sweep, and
**stops every commit boundary of a task that holds code** — with a message that names the task directory, not
the file, and no next step. On rc.24 as well.

## The coverage of what was driven

The finding carries no coverage claim, and none is verified here. For the block's `pinned-by` only, by the
text of the suites at eeffe347: `grep -rnF` under `crates` and `tooling-tests` for `could not read the
candidate at`, `reading the orphaned doc at` and `validating the merged effective state` — **one hit each, the
production line**. `validating the committed store at` and `validating task at` each have further hits that
speak of another cause (a path component too long; `git` not on `PATH`). The suites that set a mode or plant a
link and also call `validate`, `ingest` or `finalize` were listed and **not** read one by one, so *no test
drives this state at those doors* is **not** established by this report.

## The repro blocks

### Repro V-2

```yaml
claim: "the unreadable-entry defect of `jigc doc list` reaches further — a dangling link at a placement home, the orphan-row read, the mechanism's other consumers and the doors they reach — and that breaks the closing condition's `working-product` clause (ledger key r1-doc-list-unreadable-entry-undriven-shapes-and-consumers)"
verdict: REFUTED            # as a blocker — basis breaks-no-clause. The defect reproduces at five doors; two named shapes do not fail.
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exit, stdout and stderr in every cell below, commit ids aside"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
setup:            # HOME a fresh directory; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1
  - fixture: refs-post-hoc                 # five committed docs, one live task
  - ["sh", "-c", "printf '# x\\n' > docs/research/locked.md && chmod 000 docs/research/locked.md"]
  - ["jigc", "start", "--workflow", "dev-task", "add a cache"]          # exit 0, mints `add-a-cache`
  - ["sh", "-c", "printf 'cache\\n' > cache.txt && git add cache.txt"]
  - ["jigc", "doc", "set-field", "commit:add-a-cache#header/type", "--task", "add-a-cache", "--value", "feat"]
  - ["jigc", "doc", "set-slot", "commit:add-a-cache#summary", "--task", "add-a-cache", "--from-file", "-"]   # stdin: "add a cache"
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "ingest"]
  - ["jigc", "validate"]
  - ["jigc", "task", "validate", "add-a-cache"]
  - ["jigc", "task", "finalize", "add-a-cache"]
  - ["jigc", "task", "list"]
expect:
  - exit: 1
    stdout: ""
    stderr: "reading the committed doc at \"<abs repo>/docs/research/locked.md\": Permission denied (os error 13)\n"
  - exit: 1
    stdout: ""
    stderr: "could not read the candidate at \"<abs repo>/docs/research/locked.md\": Permission denied (os error 13)\n"
  - exit: 1
    stdout: ""
    stderr: "validating the committed store at \"<abs repo>\": Permission denied (os error 13)\n"
  - exit: 1
    stdout: ""
    stderr: "validating task at \"<abs repo>/.jigc/tasks/add-a-cache\": Permission denied (os error 13)\n"
  - exit: 1
    stdout: ""
    stderr_contains: "validating task at"
    after: "HEAD and the commit count unchanged; cache.txt still staged"
  - exit: 0
    stdout_contains: "add-a-cache"            # the task is still open
  - every_refusal: "one line, no `route`, no backtick, no `jigc <verb>`; docs/research/locked.md still mode 000, 4 bytes"
driven-as: "each door in a rig of its own, never the six in one — `ingest` was driven in a rig with no minted task; the block joins them for a converter, and the order above was not itself driven"
controls:         # the same sequences with no plant, each in a rig of its own
  - "doc list 0 (five rows) · ingest 0 (five adopted) · validate 0 · task validate 0 · task finalize 0, `cache.txt` committed alone"
with-a-dangling-link:   # `ln -s nowhere-target docs/research/dangling.md` in place of the file
  - "doc list 1 · ingest 1 (`… dangling.md\": No such file or directory (os error 2)`) · validate 0, the link not named · task validate 0 · task finalize 0, `cache.txt` committed alone, the link printed as left-out and still there"
shapes-that-do-not-fail:   # fresh root: git init, one commit, `jigc setup`
  - "ln -s /nonexistent/nowhere.md CHANGELOG.md     -> doc list 0 `no committed docs`; --format json 0 `{ \"docs\": [] }`; validate 0"
  - "two committed stamped files under docs/archive/, one then chmod 000 (or replaced by a dangling link) -> doc list 0 with ONE `orphaned` row; validate 1 naming the readable one only"
  - "a readable version-1 changelog at docs/changelog/changelog.md, a dangling link and a mode-000 file beside it -> doc list 0, the one `managed` row"
and-one-that-does:
  - "printf '# Changelog\\n' > CHANGELOG.md; chmod 000 -> doc list 1 `… CHANGELOG.md\": Permission denied (os error 13)`; validate 1"
milestone:        # fixture refs-post-hoc + the mode-000 file; `milestone create \"Wave one\"`, `add-task wave-one \"add a cache\"`, `provision wave-one`, one file staged in the sub-task worktree
  - "jigc milestone finalize wave-one -> exit 1, `validating the merged effective state under .jigc/milestones/wave-one/merged: Permission denied (os error 13)`; with no plant, and with the dangling link: exit 0"
clause: "working-product — NOT broken: (a) 277 cells the same on the previous release, so no command that works there stops working; (b) the refusals the plant causes print no route, so none fails as printed. The reading (b) rests on is stated in the report."
observed: "<W>/y-codetask-{N,L,M}-cand.*, <W>/b-{ro,ingest,finalize,rename,unmanage,mint,migrate,milestone}-{N,L,M}-cand.*, <W>/x-{msfull2,amend}-{N,L,M}-cand.*, <W>/a{1,2,3}-*-cand.*, <W>/o{1,2,3}-*-cand.*, <W>/p{1,2}-*-cand.*; the same names with -prev for the previous release"
pinned-by: "UNPINNED: three of the five messages have one hit under crates/ and tooling-tests/, the production line; whether any suite drives an unreadable instance at `ingest`, `validate` or a finalize was not established (the candidate suites were listed, not read)"
```

**Pinnable as it stands: yes, for what it asserts** — a named fixture state, two filesystem calls, literal
argv, and for expectations an exit status, an empty stdout and the tail of one stderr line. What a converter
needs to know:

1. **It would pin a defect's present shape.** A fix inverts the five refusals; a pin written before the row's
   disposition is a characterization test and should say so. The three *shapes that do not fail* are the
   other kind: facts a fix must not break, pinnable as they are.
2. **The half the refutation rests on is a comparison of two binaries**, which no test of one binary holds.
3. **The doors were driven one rig each** (`driven-as`, in the block). A test that runs the six in one repo
   should expect `ingest`'s cache write and the minted task to be present by the time `validate` runs; neither
   changed an exit in the cells where they did coexist (`task validate` ran after a mint in every Y rig).
4. **Platform edges.** Every shape is Unix-only. A mode-000 file refuses nobody who may read any file — the
   five refusals do not occur for root, in a container that runs as root included. The absolute path in a
   message is the canonical one, so a pin matches on the tail.

### Repro V-2b — the two routes of *Left open* 1 and 2, so that they are not re-derived

```yaml
claim: "in a state that holds an unreadable entry, a route a refusal prints does not deliver"
verdict: REFUTED            # as a break BY THIS FINDING — see Step 3; left open as two rows of their own
setup-a:          # fresh root: git init, one commit, `jigc setup`
  - "docs/archive/old.md and docs/archive/kept.md, each `---\\nschema-version: 1\\n---\\n\\n# An orphan\\n…`, committed"
  - ["chmod", "000", "docs/archive/old.md"]
repro-a:
  - ["jigc", "validate"]      # exit 1; `schema-conformance.orphaned-instance` at docs/archive/kept.md; route: "ask `jigc ingest`, which re-reads the file …"
  - ["jigc", "ingest"]        # exit 1; "could not read the candidate at \"<abs repo>/docs/archive/old.md\": Permission denied (os error 13)"
control-a: "without the chmod: validate 1 naming both files; `jigc ingest` exit 0"
setup-b:          # fixture refs-post-hoc; its live task is ground-the-vision-in-research
  - ["sh", "-c", "printf '# x\\n' > docs/research/locked.md && chmod 000 docs/research/locked.md"]
repro-b:
  - ["jigc", "doc", "show", "research:locked", "--task", "ground-the-vision-in-research"]   # exit 1; store.not-staged; route: "`jigc doc show research:locked` — the task-less read serves the committed copy"
  - ["jigc", "doc", "show", "research:locked"]                                               # exit 1; store.not-found … Permission denied; route: the `--task` read again
control-b: "a committed doc the task does not stage (research:context-loss): the same refusal, and its route exits 0. A readable non-doc (`# x`, mode 644, research:plain): the same refusal, and its route exits 1 `store.unparseable` — with no unreadable entry anywhere"
also-on-previous-release: "both, byte-identical"
pinned-by: "UNPINNED: not searched"
```

## What was driven, and what was not

- **Driven:** the listing over a dangling link and a mode-000 file at a placement home (three homes); the
  orphan-row read and the prior-home rows over both shapes, each with a readable control; the fourteen call
  sites read in the source; nineteen doors and forms over the dangling link and the mode-000 file at a located
  home of a populated store, each with a no-plant control, a task with a doc only and a task with code, a
  milestone refused for zero contribution and one taken through its commit boundary; every route a refusal
  printed in those states; both formats where the door has two; both binaries.
- **Not driven:** Linux. A directory named `x.md` and a named pipe at any door but the listing (the row's
  first verifier drove them there). A plant at a home of the dev pack's code-anchor doctypes with anchors
  present. The staged arm (`--task`), which another verifier holds. A tracked managed doc made unreadable. The
  `pre-commit` hook's `jigc validate` over the plant. `jigc migrate-corpus` and `jigc relocate` with a
  corpus that folds — `migrate-corpus` was run once, as a printed route, over a file of mine that does not.
  `jigc uninstall`, `jigc upgrade` and `jigc config`, which the enumerator's call graph does not reach.
- **Not established:** that the door list is complete — it is a hand trace; and that no suite pins these
  states.
- **Nothing was fixed, graded or decided.** What happens to the row is triage's and the human's.

## Left open — seen on the way, not pursued

1. **A route that names `jigc ingest` does not work while any `*.md` the scan reads cannot be read.** Driven:
   the store sweep's `schema-conformance.orphaned-instance` route, exit 0 in the control and exit 1 beside an
   unreadable sibling (`Repro V-2b`, a). `store.unparseable`'s adoption route names the same door and was not
   run in that state. Both binaries. Under the other reading of the clause's second half this is a route that
   fails as printed.
2. **`store.not-staged` promises a serve the task-less read may refuse.** Its route reads "`jigc doc show
   <address>` — the task-less read serves the committed copy" wherever a file exists at the home; that read
   exits 1 for a readable file that is no managed doc and for a file that cannot be read (`Repro V-2b`, b).
   The first needs no unreadable entry at all. Both binaries.
3. **`store.not-found` over a file that exists and cannot be read.** The code and its route say *create the
   doc, or read the staged copy* about a file whose only fault is its mode; with item 2 the two refusals
   route to each other.
4. **Three of the five lines do not name the file.** `jigc validate` names the repository root, `jigc task
   validate` and `jigc task finalize` the task directory, `jigc milestone finalize` the merged area. A user
   whose commit boundary is refused is not told which file to look at.
5. **An unreadable stamped file leaves the listing and the sweep without a word.** An orphan, or an instance
   at a retired home, that cannot be read is dropped at exit 0 from `jigc doc list`, and `jigc validate` stops
   naming it — over a tracked dangling link alone it says *the committed store validates clean*. The listed
   row and the blocking finding both return when the file is readable. Both binaries.
6. **`jigc validate` treats the two plants differently at a located home** — silent over the link, exit 1
   over the file — because its code-anchor walk keeps a directory read of its own beside the shared
   enumerator. Two posture rules for one family of entry.
7. **Refusing doors leave jigc's own files behind**: `.jigc/index/edges.json` from `jigc ingest` and `jigc
   task validate`; `.jigc/milestones/<id>/merged/` and a loose git object from `jigc milestone finalize`. All
   ignored or unreachable, none of the user's.
8. **`task finalize`'s hint `git add to include`** names a mode-000 file that `git add` itself refuses (exit
   128, *unable to index file*). git's refusal, under a hint that does not expect it.
9. **Linux** — still driven by nobody.

## Where the evidence is

`<W>` is `<scratch>/verify-p2-undriven.IJne7v`. Per root `<W>/<root>`:
`<W>/<root>.runs/<label>.{argv,rc,stdout,stderr,porc.before,porc.after,head.before,head.after,man.before,man.after}`.
The group logs: `<W>/group{A,O,P2,R}-{cand,prev}.log`, `<W>/groupB-{cand,prev}-{N,L,M}.log`,
`<W>/groupX2-…`, `<W>/groupY-…`, `<W>/groupZ-…`; the comparison of the first 236 pairs: `<W>/compare-all.txt`.
The tools: `<W>/tools/{snap.py,drive.sh,mkfresh.sh,groupA.sh,groupO.sh,groupP2.sh,groupB.sh,groupX.sh,groupY.sh,groupZ.sh,groupR.sh,compare.sh,compare2.sh}`.
Nothing was torn down, and the one file written for the repository is this report.

<!-- end of report -->
