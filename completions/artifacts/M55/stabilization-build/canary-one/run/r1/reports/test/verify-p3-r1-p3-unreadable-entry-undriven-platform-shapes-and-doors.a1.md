# verify-real — `r1-p3-unreadable-entry-undriven-platform-shapes-and-doors` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-unreadable-entry-undriven-platform-shapes-and-doors`. One finding, handed over by
triage with the grade *unclear*: door *unlisted* — the doors of the unreadable-entry class on Linux, with a
directory or a named pipe as the entry, at a code-anchor doctype's home, over a tracked managed doc, through
the pre-commit hook, and `jigc migrate-corpus` and `jigc relocate` over a corpus that folds; the clause it is
said to break `working-product`; and for a repro block the bullets *Not driven* and *Not established* of
`verify-p2-r1-doc-list-unreadable-entry-undriven-shapes-and-consumers.a1.md` — no block. Triage asked for the
cells that report lists as not driven.

## Verdict

- **verdict:** `refuted` — **as a blocker**. The defect is real, it reaches the doors the finding names, and
  it stays a row of the ledger.
- **basis:** `breaks-no-clause` — 443 cells driven on both binaries, in 90 rigs each: every exit status is the
  same on `1.0.0-rc.24` as on the candidate, and every byte of stdout and stderr is the same but abbreviated
  commit ids, so no command that works there stops working; the refusals the plants themselves cause print no
  route; and every route that *was* printed beside a plant behaves beside the plant as it does with no plant,
  one pair apart (*Step 3*). It is **not** `does-not-reproduce` — the class reaches five of the six families
  of cells named, and the sixth was not driven — and it is **not** `intended`: no settled decision says an
  unreadable entry should end these doors, or that a named pipe should hold them forever.
- **contested:** `false`. The verdict rests on **one reading of the clause's second half**, the same one the
  row this finding grew from was judged by; it is stated under *Step 3* with what the other reading returns.
- **regression:** not returned (it goes with `confirmed` only). The fact was established all the same:
  **not a regression** — 443 paired cells, none differing in exit status or in bytes other than commit ids.
- **class:** `instance, unbounded`. I enumerated no consumers; I drove cells. What was driven is listed under
  *What was driven, and what was not*.
- **platform:** **macOS only** — macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157), a user that is not root.
  **Linux was not driven, and this verdict does not cover it**: both binaries handed to me are Mach-O arm64
  executables, no Linux build of the candidate and no trial image is on my `BINARY:` line, and I build nothing.

**What the drive found, in eight lines.**

1. **A directory named `x.md`** at a located home ends **only the listing** — `jigc doc list`, three forms,
   exit 1, `Is a directory (os error 21)`. `validate`, `ingest`, `task validate`, `task finalize`, `milestone
   finalize`, `migrate-corpus`, `relocate` and a commit through the hook run as they do with no plant; the
   `validate` and `ingest` outputs are byte-identical to the control's.
2. **A named pipe named `x.md`** at a managed home makes **every door that reads the home wait forever** —
   twelve doors in seventeen cells, the **`git commit`** through the installed hook among them. Each cell was
   driven alone in a rig of its own, and in each the pipe accepted a writer while the command waited, which a
   pipe does only while something is reading it. `jigc relocate` does not wait.
3. **A tracked managed doc made mode 000** refuses like the untracked mode-000 file the earlier report drove,
   at the same doors and with the same lines — and at three doors that act on the doc itself: resuming the
   task that cites it (`jigc start --task`, exit 1), `jigc rename` of it (exit 1, one line, no route), and
   `jigc unmanage` of it (exit 0, the record dropped, the file untouched).
4. **At a code-anchor doctype's home with anchors present** (`docs/specs/` of the rig state `vendored`) the
   five plants behave as at `docs/research/`. Nothing there is new.
5. **Through the pre-commit hook** a plain commit lands at exit 0 over every plant but the pipe. **But the
   hook goes quiet over a mode-000 entry**: its `jigc validate` exits 1, the hook reads no report, and so the
   drift warning is not printed and **a commit that stages a bare `git mv` of a managed doc — refused at exit
   1 with no plant — lands at exit 0**.
6. **`jigc migrate-corpus` over a corpus that folds** migrates and commits the two docs over every plant but
   the pipe, exit 0, the plant untouched and in no commit. **A doc it cannot read is left out without a word**
   — exit 0, `1 migrated, 0 already current, 0 blocked`, the unreadable doc in no list.
7. **`jigc relocate` over a corpus that moves** moves the stranded doc over every plant, the pipe included.
   Over a stranded doc that is itself unreadable it prints `1 blocked` with the OS error, moves nothing, and
   exits 0.
8. **Nothing of the user's is written, committed or destroyed that the user did not ask for.** The plant's
   entry is the same before and after **every one of the 886 cells**, and every commit a cell made holds only
   the paths its door was asked for.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`).
- With the candidate's directory first on `PATH`, `command -v jigc` printed `<scratch>/bin/c1.a1/jigc`. The
  driver repeats that check before every invocation, for whichever binary the cell is on, and exits 97
  otherwise; it never did. No `cargo build`; nothing under `target/` was driven. Every rig was built with
  `dev/jigc-rig --binary <that binary>`, stdout captured alone.
- **The hook runs the binary its rig was built with.** `jigc setup` writes the absolute path of the binary
  that ran it into `.git/hooks/pre-commit` (`jigc='<scratch>/bin/c1.a1/jigc'` in the candidate's rigs,
  `jigc='<scratch>/bin/previous-91834b5e011d/jigc'` in the previous release's — read in four rigs), so a
  commit through the hook in a rig is a cell of that rig's binary.
- The clone after the drives: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/`, `HEAD`
  eeffe347, branch `fix/canary-one`. Nothing was built, edited, staged or committed there.

## Step 1 — driven from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p3.bLXp4m` (written `<W>`). Every rig got a fresh directory `<W>/r/<name>-<binary>` as its
`SCRATCH`, and `dev/jigc-rig` minted the root inside it; nothing was renamed, reset or torn down. No root of
any other reporter was read or reused. **Each plant, and each sequence that writes, got a rig of its own**;
the read-shaped doors of one plant share one rig. The seventeen pipe cells of the second pass got one rig per
cell.

Environment of every driven invocation: `HOME=<root>/home`, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables,
`JIGC_PACK_DIR` unset, stdin from `/dev/null` (a file for the one `--from-file -`). The driver, in this order:
`git status --porcelain --untracked-files=all --ignored`; `HEAD` and the commit count; a manifest of every
entry under the repository (kind, mode, size, sha256 or link target — a pipe is never opened); the invocation
in a session of its own, stdout and stderr each to a file, **the exit status read from the child directly**;
the three again. No pipe stands between a verb and its exit status, and no output was cut: what the logs
abbreviate is whole in the cell's file.

**The tally.** **443 invocations on the candidate and 443 on the previous release, paired one to one**, in
**90 rigs each**. Exit statuses on each binary: 259 at exit 0, 121 at exit 1, 2 at exit 3, and 61 that did not
return inside their limit (every one of them a cell with a pipe in the tree). Five more rigs were exploration
on the candidate (the control sequences and the two corpora, before the groups were scripted) and are not
counted.

**The plants.** `x.md` is the entry's name everywhere; the home varies with the group.

- **N** — no plant: the control of every sequence.
- **M** — `x.md` holding `# x`, then `chmod 000`: an untracked file nobody may read (the earlier report's
  plant; kept as the anchor of the hook, migration and relocation cells it did not drive).
- **L** — `ln -s nowhere-target x.md`: an untracked dangling link (kept for the same reason, in groups B, C, R).
- **D** — `mkdir x.md`: an empty directory.
- **F** — `mkfifo x.md`: a named pipe with no writer.
- **T** — `chmod 000` on a **tracked, committed, managed** doc.

### The exit matrix — candidate; the previous release is the same in every cell

`HANG` is *no exit inside the limit* (10 s in these rigs; a control cell takes 0.0–1.5 s). Rows are in the
order driven.

**A — rig `refs-post-hoc`, the plant at the located home `docs/research/`; T is `docs/research/context-loss.md`.**

| cell | N | M | D | F | T |
|---|---|---|---|---|---|
| `jigc doc list` · `--format json` · `doc list research` | 0 | 1 | **1** | **HANG** | **1** |
| `jigc validate` · `--format json` | 0 | 1 | **0** | **HANG** | **1** |
| `jigc doc show research:x` (T: `research:context-loss`) | 1 | 1 | 1 | **HANG** | **1** |
| the same with `--task <the live task>` | 1 | 1 | 1 | 1 | 1 |
| `jigc migrate docs/research/x.md --as research` (T: the doc) | 1 | 1 | 1 | **HANG** | 1 |
| `jigc start` (bare) | 0 | 0 | 0 | **HANG** | 0 |
| `jigc describe` · `jigc task list` · `jigc upgrade` · `jigc start --workflow dev-task "<intent>"` | 0 | 0 | 0 | 0 | 0 |
| `jigc ingest` · `--format json` | 0 | 1 | **0** | **HANG** | **1** |
| `jigc task validate <task>` — the task holds one staged code file | 0 | 1 | **0** | **HANG** | **1** |
| `jigc task finalize <task>` — the same task | 0 | 1 | **0** | **HANG** | **1** |
| `git commit` of one staged file, through the hook | 0 | 0 | 0 | **HANG** | 0 |
| `jigc task validate <sub-task>` from its worktree, code staged | 0 | 1 | 0 | **HANG** | **1** |
| `jigc milestone join wave-one` | 0 | 0 | 0 | **HANG** | 0 |
| `jigc milestone finalize wave-one` | 0 | 1 | **0** | **HANG** | **1** |
| `jigc doc list` afterwards | 0 | 1 | 1 | HANG | 1 |

**B — rig `vendored` (a spec and an arch-doc whose code anchors resolve), the plant at the spec home
`docs/specs/`; T is `docs/specs/padding.md`.**

| cell | N | M | L | D | F | T |
|---|---|---|---|---|---|---|
| `jigc doc list` · `--format json` | 0 | 1 | 1 | 1 | HANG | 1 |
| `jigc validate` · `--format json` | 0 | 1 | 0 | 0 | HANG | 1 |
| `jigc doc show spec:x` (T: `spec:padding`) | 1 | 1 | 1 | 1 | HANG | 1 |
| `jigc doc show arch-doc:padding-layer` | 0 | 0 | 0 | 0 | 0 | 0 |
| `jigc ingest` | 0 | 1 | 1 | 0 | HANG | 1 |
| `jigc task validate <task>` (code staged) | 0 | 1 | 0 | 0 | HANG | 1 |
| `jigc task finalize <task>` | 0 | 1 | 0 | 0 | HANG | 1 |
| `git commit` of one staged file, through the hook | 0 | 0 | 0 | 0 | HANG | 0 |
| `jigc validate` after the anchor's symbol is renamed in `src/pad.ts` | 0, the blocking `doc-code.symbol-exists` | 1 | 0, the finding | 0, the finding | HANG | 1 |
| `git commit` of that edit, through the hook | 0, **the drift warning on stderr** | 0, **stderr empty** | 0, warning | 0, warning | HANG | 0, **stderr empty** |
| `jigc validate` after a bare `git mv` of the arch-doc | 1, `reconciliation.rename` | 1, the one line | 1, the finding | 1, the finding | HANG | 1, the one line |
| `git commit` of that move, through the hook | **1, commit blocked** | **0, committed** | 1, blocked | 1, blocked | HANG | **0, committed** |

The last row was driven a second time by itself — rig `vendored`, the plant, the `git mv`, the commit, nothing
else (group H; N, M and T on both binaries): **1, blocked** with no plant; **0, committed, stderr empty** over
the mode-000 file and over the mode-000 spec.

**C — rig `fresh`, then a version-1 `changelog` at its prior home `docs/changelog/changelog.md` and a
version-1 `adr` at `docs/decisions/`, committed: a corpus that folds (the control migrates both and commits).
Plants N–F at the adr home `docs/decisions/`; T is the adr itself; the `c` columns are the same plants at the
prior home `docs/changelog/`, and Tc is the changelog itself.**

| cell | N | M | L | D | F | T | Mc | Lc | Dc | Fc | Tc |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `jigc validate` (before) | 1 | 1 | 1 | 1 | HANG | 1 | 1 | 1 | 1 | HANG | 1 |
| `jigc migrate-corpus --dry-run` | 0 | 0 | 0 | 0 | HANG | 0 | 0 | 0 | 0 | HANG | 0 |
| `jigc migrate-corpus` | 0 | 0 | 0 | 0 | HANG | **0** | 0 | 0 | 0 | HANG | **0** |
| `jigc migrate-corpus` again | 0 | 0 | 0 | 0 | HANG | 0 | 0 | 0 | 0 | HANG | 0 |
| `jigc doc list` (after) | 0 | 1 | 1 | 1 | HANG | 1 | 0 | 0 | 0 | HANG | 0 |
| `jigc validate` (after) | 0 | 1 | 0 | 0 | HANG | 1 | 0 | 0 | 0 | HANG | 0 |

In every column but F, Fc, T and Tc the migration prints `2 migrated, 0 already current, 0 blocked` and one
commit holds `docs/changelog/changelog.md` renamed to `CHANGELOG.md` and the adr modified — the control's
commit exactly. Under **T** it prints `1 migrated, 0 already current, 0 blocked`, names `CHANGELOG.md` only and
commits that one rename; under **Tc** the same with the adr only. In both the unreadable doc is still at
version 1, still mode 000, and in neither the *migrated* nor the *current* nor the *blocked* list. The first
`validate` exits 1 in every column because the corpus is below its schema version — with the finding and its
`jigc migrate-corpus` route in N, L, D, Mc, Lc, Dc and Tc (under Tc for the adr alone), and with the one
operational line and **no route** under M and T.

**R — rig `fresh`, a manifest-less `note` pack (`location: notes/`) listed in `.jigc/config/packs.yaml`, and
one note committed at `docs/legacy-notes/`: the corpus `crates/cli/tests/relocate.rs` builds, over a set-up
repository. Plants N–F at the prior home `docs/legacy-notes/`; T is the stranded note itself; the `d` columns
are the plants at the current home `docs/notes/`.** `jigc relocate` refuses every doctype both shipped packs
carry, so a stock corpus has no cell here at all.

| cell | N | M | L | D | F | T | Md | Ld | Dd | Fd |
|---|---|---|---|---|---|---|---|---|---|---|
| `jigc relocate note --from docs/legacy-notes/` | 0 | 0 | 0 | 0 | 0 | **0** | 0 | 0 | 0 | 0 |
| the same again | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `jigc doc list` (after) | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 1 | HANG |
| `jigc validate` (after) | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | HANG |

Every column but T prints `1 moved, 0 displaced, 0 blocked` and stages the one rename. **T** prints `0 moved,
0 displaced, 1 blocked`, the path, and `reading the stranded doc docs/legacy-notes/cache-benchmarks.md:
Permission denied (os error 13)` on stdout, at exit 0, with no route; nothing moves.

**E, P3 — the doors that act on the unreadable tracked doc itself (N and T).**

| cell | N | T |
|---|---|---|
| `jigc start --task <the live task>` — the task cites the doc | 0 | **1**, `store.not-found … Permission denied (os error 13)` |
| `jigc task validate <the live task>` | 3 (its unauthored commit doc) | 3, the same bytes |
| `jigc doc show vision:vision --task <task>` · `jigc doc list --task <task>` · `jigc doc schema research` | 0 | 0 |
| `jigc unmanage docs/research/context-loss.md` | 0 | 0 — the record dropped, the file left as it is |
| `jigc rename spec:padding --to padding-helper` (rig `vendored`, no task in flight) | 0, renamed and committed | **1**, `could not read the doc to rename at docs/specs/padding.md: Permission denied (os error 13)`, nothing moved |

### The pipe cells, each alone in a rig of its own

The matrix's F columns share a rig, and each cell there ended by a kill, so a second pass drove **seventeen
cells one per rig** on both binaries. After 8 s with no exit the driver offers the pipe a writer — a
non-blocking write-open, which a pipe refuses (`ENXIO`) unless something is reading it — closes it at once,
and repeats until the command exits.

| cell (the pipe at) | waited | a writer was accepted | then exit | on the previous release |
|---|---|---|---|---|
| `jigc doc list` (`docs/research/`) | yes | yes | 0 — the pipe listed as `research:x … unregistered` | the same |
| `jigc validate` | yes | yes | 1 | the same |
| `jigc doc show research:x` | yes | yes | 1, `store.unparseable` | the same |
| `jigc migrate docs/research/x.md --as research` | yes | yes | 1, `migrate.source-untracked` | the same |
| `jigc start` | yes | yes | 0 | the same |
| `jigc ingest` | yes | yes | 0 | the same |
| `jigc task validate <task>` (code staged) | yes | yes | 0 | the same |
| `jigc task finalize <task>` | yes | yes | 0, `cache.txt` committed | the same |
| `git commit`, through the hook | yes | yes | 0, committed | the same |
| `jigc task validate <sub-task>` from its worktree | yes | yes | 0 | the same |
| `jigc milestone join wave-one` | yes | yes | 0 | the same |
| `jigc milestone finalize wave-one` | yes | yes | 0, `cache.txt` and the record committed | the same |
| `jigc validate` (`docs/specs/`, rig `vendored`) | yes | yes | 1 | the same |
| `git commit` of a bare `git mv`, through the hook (`docs/specs/`) | yes | yes | 1, commit blocked | the same |
| `jigc migrate-corpus --dry-run` (`docs/decisions/`) | yes | yes | 0 | the same |
| `jigc migrate-corpus` (`docs/decisions/`) | yes | yes | 0, the control's commit | the same |
| `jigc migrate-corpus` (`docs/changelog/`) | yes | yes | 0, the control's commit | the same |

So the wait is at the pipe in every cell, and nothing else holds these commands. What follows the wait is what
the door does with an empty file, and is not a cell of this finding. **One number differs between the
binaries and is not theirs:** how many times my writer was accepted in the `milestone finalize` cell, 8 on the
candidate and 7 on the previous release — a count of how often a 0.25 s poll found the pipe open, which
depends on when it looked. The exit and the bytes of that cell are the same.

### The refusal lines the plants draw, whole

Each is one line on stderr (three for `--format json`, the single-key `{ "error": … }` envelope), stdout
0 bytes, no finding code, no route, no command:

    reading the committed doc at "<root>/repo/docs/research/x.md": Is a directory (os error 21)
    reading the committed doc at "<root>/repo/docs/research/context-loss.md": Permission denied (os error 13)
    could not read the candidate at "<root>/repo/docs/research/context-loss.md": Permission denied (os error 13)
    validating the committed store at "<root>/repo": Permission denied (os error 13)
    validating task at "<root>/repo/.jigc/tasks/add-a-cache": Permission denied (os error 13)
    validating the merged effective state under .jigc/milestones/wave-one/merged: Permission denied (os error 13)
    could not read the doc to rename at docs/specs/padding.md: Permission denied (os error 13)

The same lines with `docs/specs/…` at the code-anchor home, with `docs/decisions/…` and `docs/notes/…` in
groups C and R, and with `No such file or directory (os error 2)` for the link.

### Does any door write, commit or destroy at exit 0

**No**, held three ways over all 886 cells:

- **The plant.** The manifest before and after each cell was compared for every entry named `x.md` and every
  mode-000 file outside `.git/` and `.jigc/`: **0 cells of 886 changed one** — the directory is still an empty
  directory, the pipe still a pipe, the mode-000 files still mode 000 at their size.
- **The commits.** For every cell that moved `HEAD`, `git diff --name-status <before> <after>`: `cache.txt`
  (the code finalize) · `plain.txt` (the plain commit; `cache.txt` and `plain.txt` where the refused finalize
  had left `cache.txt` staged, which is git committing its index) · the milestone record (create, add-task) ·
  `cache.txt` and the record (milestone finalize) · `src/pad.ts` and the arch-doc's rename (the hook commits,
  each what was staged) · the corpus migration's two paths, or the one it could read · the spec's rename. No
  plant is in any of them.
- **What a refusing door leaves** is jigc's own: entries under `.git/` and `.jigc/` only, in every cell that
  exits 1 — never a file of the user's.

### Every printed route in a state holding a plant, run as printed

The refusals the plants cause at `doc list`, `ingest`, `validate`, `task validate`, `task finalize`,
`milestone finalize`, `rename` and (the blocked line of) `relocate` print **no route**. The routes that were
printed beside a plant, each run on both binaries with the same result:

| printed by | the route's command | result beside the plant | with no plant |
|---|---|---|---|
| `validate` — `schema-conformance.schema-version-current` (C: L, D, Mc, Lc, Dc, Tc) | `jigc migrate-corpus` | exit 0; every doc the finding named is migrated and committed | exit 0, the same |
| `validate` — `reconciliation.rename` (rig `vendored` after a bare `git mv`; L and D at `docs/specs/`) | `git -C <root>/repo mv docs/architecture/renamed-layer.md docs/architecture/padding-layer.md`, taken from the output and run from a file | exit 0; `validate` then exits 0, *validates clean* | exit 0, the same |
| the same finding, its other alternative | `jigc rename arch-doc:padding-layer --to "Renamed Layer"` (the placeholder filled) | **exit 1**, `store.not-found — no managed doc … to rename` | **exit 1, the same bytes** — not about the plant (*Left open*, 7) |
| `doc show research:context-loss` over T — `store.not-found` | `jigc task list` | exit 0, names the live task | exit 0 |
| the same | `jigc doc show research:context-loss --task <task>` | exit 1, `store.not-staged — … only its committed copy exists` | exit 1, the same bytes |
| that refusal — `store.not-staged` | `jigc doc show research:context-loss` | **exit 1**, the `store.not-found` it came from | exit 0, the doc |
| `start --task <task>` over T — `store.not-found` at `research:context-loss#findings` | `jigc doc show research:context-loss#findings --task <task>` | exit 1, `store.not-staged` | exit 1, the same bytes |
| that refusal | `jigc doc show research:context-loss#findings` | **exit 1**, `store.not-found … Permission denied` | exit 0, the slice |
| `migrate <the plant>` | re-run `jigc migrate <path> --as research` *with a readable file* | conditional — not a command for this file | the same refusal for a path that is not there |

The two rows that exit 1 beside the plant and 0 without it are the pair the earlier report left open as its
items 2 and 3, here at one more door (the resume) and over a tracked managed doc. *Step 3* says how they are
read.

## Step 2 — is it what the finding says

The finding is a list of cells nobody drove, with the suggestion that the class reaches them. Cell by cell:

- **A directory as the entry** — *it reaches the listing and nothing else.* Every other door skips an entry
  that is not a regular file; the listing reads it.
- **A named pipe as the entry** — *it reaches every door that reads the home, as a wait and not a refusal.*
  This is the widest cell: twelve doors, and through the hook any `git commit` in the repository.
- **A code-anchor doctype's home, anchors present** — *the same as a located home.* No door behaved
  differently at `docs/specs/` than at `docs/research/`.
- **A tracked managed doc made unreadable** — *the same lines at the same doors as the untracked file*, and
  three doors of its own (resume, rename, unmanage).
- **The pre-commit hook** — *a commit is never held by the plant, the pipe aside; the hook's two jobs are.*
- **`jigc migrate-corpus`, `jigc relocate`** — *both do their work over an untracked plant.* A doc of the
  corpus that is itself unreadable is skipped by the first without a word and reported blocked by the second.
- **Linux** — *not driven.*

Against stale state: every rig was minted for this report, and every sequence has a no-plant control built the
same way. Against a pipe or a cut: exit statuses read from the child, whole files. Against another binary: the
`PATH` check before every invocation, both hashes first, and the hook's own binary read from the hook. Against
a settled decision: `design/validation.md` → *The rc.24 fix pass registration — the home that is not a regular
file* settles one neighbouring state — a managed doc's **own home** whose entry is a link, a directory or a
special file is refused under `store.home-not-regular-file` by the commands that would **write, move or mint**
a doc there, and it says outright that a *read* is not that code's. None of these cells is that state (the
plant is a sibling entry, or a regular file nobody may read), and no cell drew that code.
`design/assistant-adapter.md` → *The doc↔code backstop* lists the five states in which the hook *prints
nothing and exits 0*; a sweep that ends on an operational error is not among them, and
`design/validation.md` → *The M19 pre-commit backstop stays doc↔code-keyed* says a commit that itself stages
a bare `git mv` of a managed doc **must** block. `crates/cli/src/migrate_corpus.rs:736` skips a candidate it
cannot read with the comment *read race: skip; the next run re-checks* — a comment, not a decision, and the
next run skips it again. So: real, not intended, and wider than the listing.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, in `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We have a working product others can
use and relay on", its instrument "**no command that works on rc.24 in a supported layout stops working, and
every refusal's route works as printed**", and the rule "A finding blocks the 1.0.0 call only if it breaks a
clause inside its scope; everything else is recorded with its tier." The run's opening declares no bound, and
none is used here.

**First half — no command that works on rc.24 stops working.** Every sequence was driven on the previous
release (sha256 accf3996…) in rigs of its own: 443 cells paired with the candidate's. **Exit statuses: 443 of
443 the same.** Stdout and stderr, after replacing each rig's root and the binary's path: **375 pairs
byte-identical; 68 differ, every differing line a seven-character commit id** (`finalized <id>`, `committed
<id>`, `[main <id>]`, `record commit: <id>`, `base <id>`, `shared base <id>` — 152 lines of eleven shapes,
listed in `<W>/runs/ids-only-diff.txt`); **0 differ otherwise**. The user's files each cell changed: the same
paths in every pair. Every cell that refuses or waits on the candidate refuses or waits on rc.24. **Not
broken.**

**Second half — every refusal's route works as printed.** The refusals this finding is about print no route,
and the wait prints nothing, so none fails as printed. Every route printed *beside* a plant does beside the
plant what it does with no plant — but for the `store.not-found` / `store.not-staged` pair over a doc that
exists and cannot be read.

*The reading this verdict rests on.* That pair is two refusals that route to each other: the first says
*create the doc, fix the reference, or read the staged copy*; the second, truthfully, that the task stages no
such doc, and that *the task-less read serves the committed copy* — which it does not, for a file nobody may
read. Each printed command is accepted and does what that verb does. I read *works as printed* as a property
of a route in the state its refusal diagnoses, not as a promise that the door it names survives a second,
independent fault — the reading the earlier report stated for the same pair, which it also showed failing
over a readable file with no unreadable entry anywhere. The pair is a defect of those two routes, it is left
open there and here, and it is not what this finding claims. **If it is read the other way** — a route that
does not deliver in any state without a declared bound breaks the clause — the verdict is `confirmed` with
`regression: false`: the cells are byte-identical on `1.0.0-rc.24`, and the block is *Repro V-3c* below. That
is a ruling on what the clause's second half reaches, and it is not mine.

**Two things I did not lean on.** *The scope:* whether a managed home holding a directory, a pipe or a file
its user cannot read is *a supported layout* the rule does not say, and I did not excuse a cell as a planted
state. *The wait:* a `git commit` that never returns is, in plain words, a product that does not work in that
repository — but the clause has one instrument, the instrument compares with rc.24 and reads routes, and on
both counts the wait is the same there and prints nothing. It is the first item of *Left open* so that it is
ruled on as itself.

**Verdict on the clause: `working-product` is not broken inside its scope.** `refuted` as a blocker, basis
`breaks-no-clause`.

## The coverage of what was driven

The finding's *Not established* bullet repeats the earlier report's — *that no suite pins these states* — and
it stays **not established** here: I searched no suite for these states and claim nothing about coverage. The
blocks below say `UNPINNED: not searched`.

## The repro blocks

### Repro V-3

```yaml
claim: "the unreadable-entry class reaches the cells nobody drove — a directory or a named pipe as the entry, a code-anchor doctype's home, a tracked managed doc, the pre-commit hook, `jigc migrate-corpus` and `jigc relocate` over a corpus that folds — and that breaks the closing condition's `working-product` clause (ledger key r1-p3-unreadable-entry-undriven-platform-shapes-and-doors)"
verdict: REFUTED            # as a blocker — basis breaks-no-clause. The class does reach these cells; no cell differs from the previous release.
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exit, stdout and stderr in every cell below, commit ids aside"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
env: "HOME a fresh directory; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1; a synthetic identity; stdin /dev/null"

a-directory-at-a-located-home:
  setup:
    - fixture: refs-post-hoc
    - ["mkdir", "docs/research/x.md"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "validate"]
    - ["jigc", "ingest"]
  expect:
    - { exit: 1, stdout: "", stderr_ends: "docs/research/x.md\": Is a directory (os error 21)\n" }
    - { exit: 0, stdout: "byte-identical to the same call with no plant" }
    - { exit: 0, stdout: "byte-identical to the same call with no plant; x.md is not named" }

a-tracked-managed-doc-nobody-may-read:
  setup:
    - fixture: refs-post-hoc                # $RIG_TASK cites research:context-loss
    - ["chmod", "000", "docs/research/context-loss.md"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "validate"]
    - ["jigc", "ingest"]
    - ["jigc", "start", "--task", "<RIG_TASK>"]                  # driven in a rig of its own
    - ["jigc", "unmanage", "docs/research/context-loss.md"]      # driven in a rig of its own
  expect:
    - { exit: 1, stdout: "", stderr_ends: "docs/research/context-loss.md\": Permission denied (os error 13)\n" }
    - { exit: 1, stdout: "", stderr_ends: "repo\": Permission denied (os error 13)\n" }
    - { exit: 1, stdout: "", stderr_contains: "could not read the candidate at" }
    - { exit: 1, stdout: "", stderr_contains: "store.not-found — could not read `research:context-loss#findings`" }
    - { exit: 0, stdout_contains: "unmanaged docs/research/context-loss.md (research:context-loss)", after: "the file still mode 000, 254 bytes" }
  rename:                                   # fixture vendored; chmod 000 docs/specs/padding.md
    - ["jigc", "rename", "spec:padding", "--to", "padding-helper"]
    - { exit: 1, stdout: "", stderr: "could not read the doc to rename at docs/specs/padding.md: Permission denied (os error 13)\n", after: "HEAD unchanged, docs/specs/ holds padding.md only" }

at-a-code-anchor-home:
  setup:
    - fixture: vendored                     # spec:padding and arch-doc:padding-layer, anchors resolving
    - ["sh", "-c", "printf '# x\\n' > docs/specs/x.md && chmod 000 docs/specs/x.md"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "validate"]
    - ["jigc", "doc", "show", "arch-doc:padding-layer"]
    - ["jigc", "ingest"]
  expect:
    - { exit: 1, stderr_ends: "docs/specs/x.md\": Permission denied (os error 13)\n" }
    - { exit: 1, stderr_contains: "validating the committed store at" }
    - { exit: 0 }
    - { exit: 1, stderr_contains: "could not read the candidate at" }
  with-a-directory-instead: "doc list 1 (`Is a directory (os error 21)`) · validate 0 `no findings` · ingest 0"

through-the-hook:                           # driven exactly so, in a rig of its own (group H)
  setup:
    - fixture: vendored
    - ["sh", "-c", "printf '# x\\n' > docs/specs/x.md && chmod 000 docs/specs/x.md"]
    - ["git", "mv", "docs/architecture/padding-layer.md", "docs/architecture/renamed-layer.md"]
  repro:
    - ["git", "commit", "-m", "move the arch doc by hand"]
  expect:
    - { exit: 0, stderr: "", after: "HEAD moved; the commit is the one rename" }
  control: "the same without the plant: exit 1, stderr is `jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).`"
  also: "with `chmod 000 docs/specs/padding.md` as the plant: exit 0, committed. With a dangling link or a directory at docs/specs/x.md (driven after a first commit that renamed the anchor's symbol): exit 1, blocked, as the control"

migrate-corpus-over-a-corpus-that-folds:
  setup:
    - fixture: fresh
    - "docs/changelog/changelog.md — a conformant changelog stamped `schema-version: 1` (the body `changelog_body(1)` of crates/cli/tests/migrate_corpus_home_pairs.rs)"
    - "docs/decisions/cache-sessions-in-memory.md — a conformant adr stamped `schema-version: 1` (`adr_body(1)` of the same suite)"
    - ["git", "add", "-A"]
    - ["git", "commit", "-q", "-m", "seed the corpus"]
    - ["sh", "-c", "printf '# x\\n' > docs/decisions/x.md && chmod 000 docs/decisions/x.md"]
  repro:
    - ["jigc", "validate"]
    - ["jigc", "migrate-corpus", "--dry-run"]
    - ["jigc", "migrate-corpus"]
  expect:
    - { exit: 1, stdout: "", stderr_contains: "validating the committed store at" }          # the control prints the finding and its `jigc migrate-corpus` route instead
    - { exit: 0, stdout_contains: "2 would migrate, 0 already current, 0 blocked" }
    - { exit: 0, stdout_contains: "2 migrated, 0 already current, 0 blocked", after: "one commit: docs/changelog/changelog.md -> CHANGELOG.md, the adr modified; x.md still mode 000, untracked" }
  with-the-adr-itself-mode-000: "migrate-corpus exit 0, `1 migrated, 0 already current, 0 blocked`, names CHANGELOG.md only; the adr still version 1 and in no list"

relocate-over-a-corpus-that-moves:
  setup:
    - fixture: fresh
    - ".jigc/note-pack/schemas/note.yaml — `type: note`, `location: notes/`, `id-from: title`, one slot section `body` (the fixture of crates/cli/tests/relocate.rs)"
    - "append to .jigc/config/packs.yaml: `packs:` and one item, the absolute path of .jigc/note-pack"
    - "docs/legacy-notes/cache-benchmarks.md — `# Cache Benchmarks`, `## Body`, one line"
    - ["git", "add", "docs/legacy-notes", ".jigc/config/packs.yaml", ".jigc/note-pack"]
    - ["git", "commit", "-q", "-m", "strand the note"]
    - ["mkfifo", "docs/legacy-notes/x.md"]
  repro:
    - ["jigc", "relocate", "note", "--from", "docs/legacy-notes/"]
  expect:
    - { exit: 0, stdout_contains: "1 moved, 0 displaced, 0 blocked", after: "the rename staged; x.md still a pipe" }
  with-the-note-itself-mode-000: "exit 0, `0 moved, 0 displaced, 1 blocked`, `reading the stranded doc docs/legacy-notes/cache-benchmarks.md: Permission denied (os error 13)`; nothing staged"

driven-as: "each section in a rig of its own, the doors in the order given with other read-shaped cells between them (the matrix above lists every one); `start --task`, `unmanage` and `rename` each in a rig of their own"
clause: "working-product — NOT broken: (a) 443 cells the same on the previous release; (b) the refusals the plants cause print no route, and the routes printed beside a plant do there what they do without it — the store.not-found / store.not-staged pair apart, read as stated in the report"
observed: "<W>/r/<group>-<plant>-{cand,prev}/<rig>.runs/<cell>.{argv,rc,stdout,stderr,porc.*,head.*,man.*,tree}; <W>/runs/group{A,B,C,R,E,H,P,Q}-{cand,prev}-<plant>.log; <W>/runs/matrix.txt; <W>/runs/compare-everything.txt"
pinned-by: "UNPINNED: not searched"
```

**Pinnable as it stands: yes, for what it asserts** — named fixture states, filesystem calls, literal argv,
and for expectations an exit status and the tail of one line. What a converter needs to know:

1. **Most of it would pin a defect's present shape.** A fix inverts the refusals; a pin written before the
   row's disposition is a characterization test and should say so. Three sections are the other kind — facts a
   fix must not break: the directory that leaves `validate` and `ingest` untouched, the migration that lands
   over an untracked plant, the relocation that moves past a pipe.
2. **The half the refutation rests on is a comparison of two binaries**, which no test of one binary holds.
3. **Platform edges.** Every shape is Unix-only. A mode-000 file refuses nobody who may read any file, so
   those sections do not hold for root — in a container that runs as root included. An absolute path in a
   message is the canonical one, so a pin matches on the tail.
4. **The hook section needs the hook to reach a `jigc`**: it holds the absolute path of the binary that ran
   `jigc setup`.

### Repro V-3b — the named pipe

```yaml
claim: "a named pipe named `x.md` at a managed home makes every door that reads the home wait until something writes to the pipe"
verdict: CONFIRMED-AS-A-FACT   # not a verdict on the finding: the same on the previous release, and no route is printed
setup:
  - fixture: refs-post-hoc
  - ["mkfifo", "docs/research/x.md"]
  - ["sh", "-c", "printf 'x\\n' > plain.txt && git add plain.txt"]
repro:                         # each in a rig of its own
  - ["jigc", "doc", "list"]
  - ["jigc", "validate"]
  - ["jigc", "ingest"]
  - ["jigc", "start"]
  - ["git", "commit", "-m", "a plain commit through the hook"]
expect:
  - each: "no exit within 8 s (the control exits in under 1 s); a non-blocking write-open of docs/research/x.md is ACCEPTED while the command waits; once a writer has opened and closed it the command exits — 0, 1, 0, 0, 0 — having read the pipe as an empty file"
driven-as: "plain.txt was staged in the commit cell's rig only, and that rig also held a minted task and a second staged file; the other four rigs held the pipe alone"
also: "doc show research:x · migrate docs/research/x.md --as research · task validate and task finalize of a task holding code · a sub-task's task validate · milestone join · milestone finalize · validate and a hook commit with the pipe at docs/specs/ (rig vendored) · migrate-corpus (--dry-run and applying) with the pipe at docs/decisions/ or docs/changelog/"
does-not-wait: "jigc relocate note --from docs/legacy-notes/ with the pipe in that directory; describe; task list; upgrade; start --workflow dev-task; doc show --task"
also-on-previous-release: "every cell, the same"
pinned-by: "UNPINNED: not searched"
```

**Not pinnable as it stands.** A test of a wait needs a limit, a second thread that offers the writer, and a
kill for the case where the door still waits; the block names the mechanism (`<W>/tools/drive.py`, `--fifo`)
but a suite would have to build it. Once the row has a disposition the pin is the inverse and is easy: the
door exits inside a second with the pipe unopened.

### Repro V-3c — the two routes over a doc that cannot be read, so that they are not re-derived

```yaml
claim: "over a committed managed doc nobody may read, the routes of store.not-found and store.not-staged lead to each other"
verdict: REFUTED            # as a break BY THIS FINDING — see Step 3; the pair is left open
setup:
  - fixture: refs-post-hoc                 # $RIG_TASK cites research:context-loss
  - ["chmod", "000", "docs/research/context-loss.md"]
repro:
  - ["jigc", "start", "--task", "<RIG_TASK>"]
  - ["jigc", "doc", "show", "research:context-loss#findings", "--task", "<RIG_TASK>"]
  - ["jigc", "doc", "show", "research:context-loss#findings"]
expect:
  - { exit: 1, stderr_contains: "store.not-found — could not read `research:context-loss#findings` at `docs/research/context-loss.md`: Permission denied (os error 13)", route: "… read it with `jigc doc show research:context-loss#findings --task <task-id>` (find the task id with `jigc task list`)" }
  - { exit: 1, stderr_contains: "store.not-staged — `research:context-loss#findings` is not staged in this task — only its committed copy exists", route: "`jigc doc show research:context-loss#findings` — the task-less read serves the committed copy" }
  - { exit: 1, stderr_contains: "store.not-found" }       # the first refusal again
control: "without the chmod: 0 (the workflow composes) · 1 store.not-staged, the same bytes · 0, the slice"
also-on-previous-release: "byte-identical"
pinned-by: "UNPINNED: not searched"
```

**Pinnable as it stands: yes** — a fixture, one `chmod`, three argv, three exits (not for root).

## What was driven, and what was not

- **Driven, on both binaries, each with a no-plant control:** a directory and a named pipe named `x.md` at a
  located home, at nineteen doors and forms beyond the listing (and at the listing, as the anchor); the five
  plants and a tracked unreadable spec at a code-anchor doctype's home with its anchors resolving; a tracked
  managed doc made mode 000 at those doors and at resume, rename and unmanage; a plain commit, a commit with a
  broken anchor and a commit with a bare `git mv`, each through the installed hook, over every plant;
  `jigc migrate-corpus` over a corpus that folds, with the plant at another doctype's home, at the folding
  doc's prior home, and as each folding doc itself; `jigc relocate` over a corpus that moves, with the plant
  at the prior home, at the current home, and as the stranded doc itself; every route printed beside a plant.
- **Not driven:** **Linux** (above). The directory with something in it — mine was empty. A pipe that has a
  writer. The plant at a placement home for these doors (the earlier report drove the listing there). The
  staged arm (`--task`) beyond the three reads in group E. `jigc relocate` with a pipe as the stranded doc.
  `jigc migrate-corpus --no-commit`. `jigc milestone finalize` over a tracked unreadable doc that the
  milestone itself changes. The hook under `core.hooksPath`. Root.
- **Not established:** that these are all the doors — I traced no call graph and enumerated no consumer; and
  that no suite pins these states.
- **Slips of mine, said plainly.** A first relocation probe appended a malformed line to `packs.yaml` and drew
  jigc's own parse refusal; it is an exploration rig, not counted. One exploration script globbed three
  directories into one path and ran nothing. The pipe group lists a `relocate` cell that its script never
  drove — two rigs with no cell in them, not counted; the relocation over a pipe is the matrix's R–F column.
  Two help texts were redirected into the system temp directory before I had minted `<W>`. Every helper
  script was written through the shell into `<W>/tools/`; the one file my file tool wrote is this report.
- **Nothing was fixed, graded or decided.** What happens to the row is triage's and the human's.

## Left open — seen on the way, not pursued

1. **A named pipe at a managed home holds every reading door, and through the hook every `git commit`,
   until something writes to it** (*Repro V-3b*). No message, no limit, no route; both binaries.
2. **The pre-commit hook goes quiet when its sweep cannot read an entry.** Over a mode-000 file at a managed
   home — untracked, or a tracked doc — the hook's `jigc validate` exits 1 with nothing on stdout, and the
   hook treats that as nothing to say: the blocking doc↔code warning is not printed, and a commit that stages
   a bare `git mv` of a managed doc lands at exit 0 where the control refuses it (*Repro V-3*,
   `through-the-hook`). `design/assistant-adapter.md` names five silent passes; this is a sixth. Both binaries.
3. **`jigc migrate-corpus` leaves out a doc it cannot read, at exit 0, in no list** — and when that doc sits
   at a prior home, `jigc doc list` and `jigc validate` afterwards exit 0 without naming it, so the store
   reads clean with a version-1 doc stranded in it. Both binaries.
4. **Under a mode-000 entry `jigc validate` cannot say the corpus is below its schema version.** The control
   prints the finding and its `jigc migrate-corpus` route; beside the plant it prints the one operational
   line — while `jigc migrate-corpus` itself runs clean over the same plant.
5. **`jigc relocate` reports a blocked doc at exit 0, with the OS error and no route.**
6. **`store.not-found` and `store.not-staged` route to each other over a doc that exists and cannot be read**
   — now also at the resume door, `jigc start --task`, for a tracked managed doc (*Repro V-3c*). The earlier
   report's items 2 and 3, one door further.
7. **A printed route that fails with no plant at all.** After a bare `git mv` of a managed doc (staged, not
   committed), `jigc validate` prints `reconciliation.rename` with two alternatives. The second, `git -C
   <root>/repo mv <new> <old>`, works. The first, `jigc rename arch-doc:padding-layer --to "<New Title>"` —
   run with a title in the placeholder — exits 1, `store.not-found — no managed doc arch-doc:padding-layer to
   rename (expected at docs/architecture/padding-layer.md)`, routed to `jigc describe`. Rig `vendored`, three
   rigs per binary (no plant, a link, a directory), the same bytes in all six. Not this finding's; a row of
   its own.
8. **A directory named `*.md` ends the listing and nothing else** — one more door where the listing's posture
   towards an entry differs from every other consumer's (the earlier report's item 6, a third shape).
9. **Three of the lines still do not name the file** — at `validate`, at the task doors, at `milestone
   finalize` — for a tracked doc as for an untracked one.
10. **Linux** — still driven by nobody. It needs a Linux build of the candidate handed over with its hash.

## Where the evidence is

`<W>` is `<scratch>/verify-p3.bLXp4m`. Per rig `<W>/r/<group>-<plant>-<cand|prev>/<rig root>`, and beside it
`<rig root>.runs/<cell>.{argv,rc,stdout,stderr,porc.before,porc.after,head.before,head.after,man.before,man.after,tree}`
(and `.hang` for a pipe cell of the second pass). The group logs: `<W>/runs/group{A,B,C,R}-{cand,prev}-<plant>.log`,
`groupE-…-{N,T}.log`, `groupF-…-<cell>.log`, `groupH-…-{N,M,T}.log`, `groupP-…-{N,T,L,D}.log`,
`groupQ-…-{N,L,D}.log`. The pairing of all 443: `<W>/runs/compare-everything.txt`; the matrix:
`<W>/runs/matrix.txt`; the commit-id lines: `<W>/runs/ids-only-diff.txt`. The fixtures: `<W>/fix/`. The tools:
`<W>/tools/{drive.py,lib.sh,groupA.sh,groupB.sh,groupC.sh,groupE.sh,groupF.sh,groupH.sh,groupP.sh,groupQ.sh,groupR.sh,runall.sh,runF.sh,compare.py,matrix.py,audit.py}`.
Nothing was torn down.

<!-- end of report -->
