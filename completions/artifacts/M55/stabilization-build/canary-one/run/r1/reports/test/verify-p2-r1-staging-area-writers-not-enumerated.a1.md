# verify-real — `r1-staging-area-writers-not-enumerated` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p2-r1-staging-area-writers-not-enumerated`. One finding, handed over: ledger key
`r1-staging-area-writers-not-enumerated`, door `jigc doc list`, the clause it is said to break
`working-product`, triage's grade *unclear*. It has no block of its own: its source is the section
*Does ordinary use reach the state* and item 2 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-doc-list-staged-arm-unreadable-entry.a1.md`.
That report was read because the prompt hands it over as the finding; no other report was read, and
nothing of triage's reasoning beyond the grade and the re-drive it asks for.

## Verdict

**`refuted` — basis `does-not-reproduce`.**

The finding is a hypothesis: that some writer into a task's staging area
(`.jigc/tasks/<id>/docs/`) that nobody had read can, under ordinary use, leave a link or a file its
owner cannot read there — the state in which `jigc doc list --task <id>` exits 1 with a raw OS
error. The writers are now enumerated and each one driven, and the hypothesis does not hold:

- **Three mechanisms write a staged-doc entry into a task's staging area, and no fourth was found.**
  The atomic temp-and-rename primitive; one `std::fs::rename` inside the area; one in-place
  `std::fs::write` that the source report did not see. The `std::fs::copy` the finding names writes
  into a scratch gate area, and a fourth raw write writes into a milestone's merged area — neither
  is a task's staging area. See *The enumeration*.
- **No production code in either crate creates a link.** Every link-creating call under the two
  `src/` trees — 17 of them — is inside a `#[cfg(test)]` module.
- **Driven on the candidate: every staged entry is a regular file its owner can read.** 21
  inventories taken after jigc's own verbs under umask 022 and 077 — over a read-only, a mode-000,
  a linked and a dangling-link committed home, through both copy-in doors, the in-area rename, the
  rollback write and two fan-outs — show 0 entries that are not a regular owner-readable file; the
  closing inventory holds 42 entries in 15 task areas. All 30 `jigc doc list --task` invocations
  over those areas exit 0.
- **The same block on a fresh rig of the previous release gives the same picture**: 89 cells, the
  same exit status in every one, the same shapes and modes in every inventory, and all 68 listing
  streams byte-identical (the rig's path normalised).

One route to the state through jigc's verbs alone exists, and it is reported as what it is — a
declared boundary, not ordinary use: a umask that masks the **owner's own read bit** (driven:
0477). Under it git itself stops working after one `git add`. It is item 1 of *Left open*, with
both binaries driven and identical.

`contested: false` — the finding argues against no settled decision.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`. The driver puts the handed binary's directory first on `PATH` and
  stops (exit 90) unless `command -v jigc` prints the binary it was handed; both runs passed it.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone, the construction log to its own file, exit 0 each time.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and
  after, `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user (uid 501).**
Nothing was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-writers.H4spQV` (written `<W>`). Under it: an exploration rig (`<W>/explore/`,
candidate only, used to learn the verbs' spellings — two side facts are read from it and each says
so where it is used), a first candidate run
(`<W>/cand.log`, superseded: see below), and the pair every number here is read from —
`<W>/c.log` with `<W>/c.runs/` (candidate) and `<W>/p.log` with `<W>/p.runs/` (previous release),
each on a rig of its own. Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. One driver, `<W>/tools/drive.sh <binary> <rig root> <out dir> <clone>
<tools>`: each cell runs in a subshell under the umask the cell names, stdin from `/dev/null` or a
file, stdout and stderr to their own files, the exit status read directly and never through a pipe.
An *inventory* is `find <area>/docs -mindepth 1` handed to `stat -f '%HT|%Sp|%z|%N'`, which reads
an entry's shape **without following a link**; an entry counts as anomalous when it is not a
regular file or its owner's read bit is off.

**Two things about the tooling, stated because they are deviations.** (1) The driver was written
with the file tool into `<W>/tools/` and later patched from the shell; the contract gives the file
tool the report alone. It is a scratch file, in no repository. (2) The first candidate run used the
unpatched driver, whose capture files were created under the cell's own umask and so came out
unreadable in the boundary group; the patch creates them before the umask applies and adds two
cells. The first run's ordinary groups read the same as the second's, cell for cell; it is kept and
not used.

## The enumeration

Method, and its counts. All lines are at eeffe347.

**(a) The 46 textual hits of `instance_path(`** in `crates/` (`command grep -rn`, 46 lines) — each
read, and classified production or test by the `#[cfg(test)]` module it does or does not sit in:
**24 production, 22 test** (`start.rs` 1 · `doc.rs` 4 · engine `finalize.rs` 9 · `store.rs` 3 ·
`state.rs` 3 · engine `milestone.rs` 2). The 24:

| site | what it does with the path |
|---|---|
| `crates/engine/src/state.rs:744` | the definition |
| `crates/engine/src/state.rs:1101` `provision_doc` | **writes** — `write_atomic` |
| `crates/engine/src/state.rs:1151` `copy_in` | **writes** — `write_atomic`, of bytes read from the committed home by `read_to_string` |
| `crates/cli/src/start.rs:645` `persist_provisioned_commit` | **writes** — `state::persist`, which is `write_atomic` |
| `crates/engine/src/state.rs:2496` `mint_instance` | builds the path a create then hands to `copy_in` or `provision_doc` — and that `CreatedDoc::rollback` writes (below) |
| `crates/cli/src/doc.rs:7429`, `:7433` `staged_path` | builds the path, behind the write-time barrier, that every edit verb reads, copies in to and persists at — and that the in-task rename moves from and to |
| `crates/engine/src/milestone.rs:2322` `materialize` | **writes, with `std::fs::write`** — but into `.jigc/milestones/<id>/merged/docs/`, not a task's area |
| `crates/cli/src/start.rs:760` | existence probe before the commit doc is provisioned |
| `crates/cli/src/task.rs:1783`, `:1867`, `:3732`, `:4939` | read |
| `crates/cli/src/doc.rs:3405`, `:5038` | read (`:5038` is the door's own read) |
| `crates/cli/src/doc.rs:4479`, `crates/cli/src/milestone.rs:8744` | `is_file()` probe |
| `crates/engine/src/store.rs:447`, `crates/engine/src/milestone.rs:2758` | read |
| `crates/engine/src/state.rs:1273`, `:1310`, `:3076`, `crates/engine/src/validate.rs:2188`, `crates/engine/src/target_surface.rs:181` | `is_file()` probe |

**(b) The two sites the finding names.**

- `crates/cli/src/doc.rs:3322`, `std::fs::rename(old_path, new_path)` in `move_staged_identity` —
  **a writer into the area**: it moves a staged entry to a new name inside the same `docs/`. A
  rename carries the entry as it is — shape and mode — so it can leave only what was there. What
  is there is a doc this task minted, written by `provision_doc`: a re-slug of a doc copied in from
  the committed store is refused before the move. Driven once, on the candidate, in the
  exploration rig: `jigc doc rename research:context-loss --to "Another title" --task <sub-task>`
  over a copied-in doc exits 1 with `write.identity-change`, with and without `--slug`, and the
  area's three entries keep their inodes.
- `crates/cli/src/milestone.rs:8553`, `std::fs::copy(&path, gate_docs.join(…))` — **not a writer
  into a task's area.** Its destination is `gate_docs`, the `docs/` of a `ScratchTree` made a few
  lines above for the join's gate; its source is the milestone's merged area, read only after
  `entry.file_type()` (not following a link) says *regular file*.

**(c) A third writer, which neither the finding nor its source names.**
`crates/engine/src/state.rs:2426`, `std::fs::write(&self.path, bytes)` in `CreatedDoc::rollback`:
when `jigc doc author` runs over a doc the task already stages and one of its leaves is refused,
the staged file's captured pre-image is written back **in place**. An in-place write keeps the
entry's inode, shape and mode; it creates a file only if the entry vanished meanwhile, and then a
regular one under the process's umask. Its one caller is `crates/cli/src/doc.rs:4299`.

**(d) The sweeps behind "no fourth was found"**, over production code of both crates:

- every `fs::rename(` / `fs::copy(` / `set_permissions` call: the two above, `write_atomic`'s own
  rename (`state.rs:1337`), `relocate.rs:926` (a squatter out of the committed store),
  `task.rs:7808` (foreign entries **out of** an area into the workbench), `task.rs:5818` and
  `setup.rs:1290` (permissions of a committed home and of a hook);
- every link-creating call (`fs::symlink`, `soft_link`, `hard_link`, `symlink_file`,
  `symlink_dir`): 17 under the two `src/` trees, **all in test modules**;
- every `fs::write(` / `File::create` / `OpenOptions` / `create_new` hit: read by line, and with
  its context wherever the target was not evident from the line. Besides (c) and the merged-area
  write, their targets are task-root files (the base pin, `intent`, `workflow`, the amend pin, the
  staged snapshot), commit-message temp files, scratch trees, the committed store, the adapter and
  hook files, the lock and log files. `File::options` has no hit;
- the callers of `persist` / `provision_doc` / `copy_in` / `rollback`: the staged-doc entry points
  of the atomic primitive are `state.rs:1103`, `state.rs:1153`, `start.rs:647` and the nine calls
  of `doc.rs`'s `persist` wrapper (`:746`, `:877`, `:1369`, `:1492`, `:2291`, `:2488`, `:2971`,
  `:3384`, `:4305`).

**The bound of this enumeration.** It is textual and by reading. A write reached through a spelling
none of those patterns match would be missed. Spawned programs in production code are `git` and
the probe's self-exec (`crates/cli/src/invoke.rs:218`); the 89 textual `Command::new("git")` sites
(test code included) were not each read. What stands against a missed writer is the inventories:
nothing but regular owner-readable files, after every door driven.

**What the atomic primitive can leave** (`write_atomic`, `crates/engine/src/state.rs:1328`): the
bytes go to a fresh sibling temp file by `std::fs::write`, which is renamed over the path. The
result is a new regular file whose mode is 0666 less the process's umask — whatever stood at the
path, a link included, is replaced and not written through, and nothing of the committed file's
mode or shape is carried.

## What was driven — the candidate

Rig `<W>/c.rig/jigc-rig-refs-post-hoc-*` (state `refs-post-hoc`), log `<W>/c.log`, per-cell files
under `<W>/c.runs/`. 89 cells: 76 exit 0, 11 exit 1, 2 exit 3 — every non-zero one is named below.

| group | umask | what jigc was asked to do | staged entries afterwards | `jigc doc list --task …` (plain and json) |
|---|---|---|---|---|
| T0 | — | nothing: the rig's own live task | 3, regular, `-rw-r--r--` | exit 0 |
| A | 022 | `jigc start --workflow single-task` (the commit doc is provisioned) · `doc create adr` · `doc rename … --to` (the `:3322` rename) · `doc author` with a refused leaf, exit 1 `write.malformed-value` (the `:2426` rollback) · `doc author` · `doc set-slot` on the committed `research:context-loss` (copy-in on first touch) | 4, regular, `-rw-r--r--` | exit 0, 3 rows |
| B | 077 | the same six acts, in a task of its own | 4, regular, `-rw-------` | exit 0, 3 rows |
| M | 022 | `milestone create` · `add-task` twice · `doc create adr` in each sub-task · a copy-in in the second · `milestone join` (exit 0, 3 docs merged) · `milestone finalize` (exit 3: the gate blocks on the adrs' empty required slots, so the areas stay) | 5 in the two sub-task areas, regular, `-rw-r--r--`; the merged area: 3 regular files | exit 0 for both sub-tasks |
| N | 077 | the same fan-out | 5, regular, `-rw-------`; merged: 3 regular `-rw-------` | exit 0 for both |
| F | 022 | lands one adr through `jigc task finalize` (exit 0), so that a committed home exists | — | — |
| K1 | 022 | the committed home made mode 0444; `doc create adr` under the same title (the create door's copy-in, `state.rs:2713`), and `doc set-slot` from a second task (the edit door's, `doc.rs:6958`) — both exit 0 | 6 in the two areas, regular, **`-rw-r--r--`**: the home's mode is not carried | exit 0 for both |
| K2 | 022 | the committed home made mode 000; the same two doors — both **exit 1**, `Permission denied (os error 13)`, nothing staged | 4 (each area's commit doc and manifest only), regular, `-rw-r--r--`; no temp residue | exit 0 for both |
| K3 | 022 | the committed home replaced by a tracked link to a real tracked file, committed; the same two doors — both exit 0, `copied in for update` | 6, **regular**, `-rw-r--r--`: the link is read through and not carried | exit 0 for both |
| K4 | 022 | the link's target moved away, committed, so the home dangles; `doc create adr` exits 0 and mints fresh; `doc set-slot` exits 1 (`no staged instance …`) | 5, regular, `-rw-r--r--` | exit 0 for both |

**The closing inventory of the ordinary groups** (label `ALL`, every entry of every task's `docs/`):
42 entries in 15 task areas — 33 `Regular File|-rw-r--r--`, 9 `Regular File|-rw-------`, nothing
else. Of the 23 inventories the run takes, the 21 outside the boundary cells report 0 anomalous
entries.

**That the two non-atomic writers were reached, not assumed.** The rename: the inventory before it
holds `adr:first-title.md`, the one after holds `adr:second-title.md` and no `adr:first-title.md`,
and the verb acks `adr:second-title (renamed to "Second title" from adr:first-title)`. The
rollback write: around the refused `doc author` the staged file keeps its inode and its mode and
its mtime moves by a little over a second (the driver sleeps one second before the cell) — the
file was rewritten in place by a command that exited 1. Both hold under 022 and under 077.

**Against the design that owns the behaviour.** `design/doc-read-surface.md` → the `--task` arm of
`jigc doc list`: *a staged working copy is jigc-written by construction, never a foreign squatter*.
The enumeration and the inventories are what that sentence rests on, and they bear it out for the
shape of what jigc writes: a regular file, readable by its owner, under every ordinary
configuration driven.

## The same block on the previous release

Not the regression step — the verdict is not `confirmed`, and no `regression` field is returned —
but what triage asked for (*on both binaries*), and the second clause is comparative by its own
wording. Rig `<W>/p.rig/jigc-rig-refs-post-hoc-*`, built with `--binary
<scratch>/bin/previous-91834b5e011d/jigc`; the same driver; log `<W>/p.log`.

- 89 cells; the exit status is the same as the candidate's in every one (76 · 11 · 2).
- `diff` of the two whole logs, the rig's path, the inodes, the mtimes and the commit sha
  normalised, differs in two things only: the **size of `provenance.json`** wherever a copy-in
  happened (the candidate's manifest records what was copied in; the same shape and mode), and the
  **wording of one refusal** — the edit door over the mode-000 home says `could not copy
  `adr:landed-choice#context` in for editing — nothing was staged: Permission denied (os error 13)`
  on the candidate and `could not read committed `adr:landed-choice#context`: Permission denied
  (os error 13)` on the previous release; exit 1 on both, nothing staged on both.
- Every inventory: the same entries, shapes and modes. The closing one: 42 entries in 15 areas, 33
  and 9, as on the candidate.
- All 68 listing streams (34 `doc list --task` invocations, stdout and stderr each) are
  byte-identical between the two binaries, the rig's path normalised.

## Does it break the clause, inside its scope

The clause's instrument, in the closing condition's words (DECISIONS.md, 2026-10-04, *The exit
rule, revised*, sharpening 2): *no command that works on rc.24 in a supported layout stops working,
and every refusal's route works as printed.* The finding needs a state for the door to fail in; in
a supported layout, under jigc's own verbs, that state was not reached on either binary, and the
door exits 0 in all 30 ordinary cells on both. There is nothing here for the clause to be broken
by. Where the state was forced (the boundary below), the door fails on the previous release in the
same bytes.

## Scope of what was verified

**Enumerated: the writers into a task's staging area — 3 mechanisms**, by the method and with the
bound stated in *The enumeration* (the atomic primitive through 12 staged-doc call sites; the
rename at `doc.rs:3322`; the rollback write at `state.rs:2426`). Driven: each of the three, on two
binaries, on one platform, under umask 022 and 077, with the committed home in five shapes.

**Not enumerated: the readers.** The other callers of the staged enumerator and the other readers
of a staged doc were not walked; for them this report is `instance, unbounded`.

## Repro W-1

```yaml
claim: "some writer into a task's staging area leaves, under ordinary use, a link or an owner-unreadable file there, so that `jigc doc list --task <id>` exits 1"
verdict: REFUTED   # basis does-not-reproduce
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in all 89 cells, the same inventories, all 68 listing streams byte-identical (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc
  - umask: "022, and the whole block again under 077 with other titles"
  - ["jigc", "start", "--workflow", "single-task", "alpha probe"]
repro:
  - ["jigc", "doc", "create", "adr", "--title", "First title", "--task", "alpha-probe"]
  - ["jigc", "doc", "rename", "adr:first-title", "--to", "Second title", "--task", "alpha-probe"]
  - ["jigc", "doc", "author", "adr", "--from-file", "<payload: title Second title, a context slot, then `status: not-a-status`>", "--task", "alpha-probe"]
  - ["jigc", "doc", "set-slot", "research:context-loss#question", "--from-file", "-", "--task", "alpha-probe"]   # stdin: one line of prose
  - ["jigc", "doc", "list", "--task", "alpha-probe"]
expect:
  - exit: 0
    stdout: "adr:first-title\n"
  - exit: 0
    stdout: "adr:second-title (renamed to \"Second title\" from adr:first-title)\n"
  - exit: 1
    stderr_contains: "blocking · write.malformed-value"
    tree: "the staged adr:second-title.md keeps its inode, is rewritten in place, and is a regular file with the mode it had"
  - exit: 0
    stdout_contains: "copied in for update"
  - exit: 0
    stdout: "id  path  state\nadr:second-title  docs/decisions/second-title.md  managed\ncommit:alpha-probe  commit:alpha-probe  managed\nresearch:context-loss  docs/research/context-loss.md  managed\n"
    stderr: ""
  - tree: "after every step, each entry of .jigc/tasks/alpha-probe/docs/ — read without following a link — is a regular file with its owner's read bit set: mode 0644 under umask 022, 0600 under 077"
variants:   # each in a task of its own; the staged entries are regular and owner-readable, and `doc list --task` exits 0, in every one
  - "a fan-out: milestone create, add-task twice, `doc create adr` in each, a copy-in in one, `milestone join` (exit 0), `milestone finalize` (exit 3 on the empty required slots) — under 022 and under 077"
  - "the committed home mode 0444: `doc create adr --title <its title>` and, from a second task, `doc set-slot <its address>#context` — both exit 0; the staged copies are mode 0644"
  - "the committed home mode 000 (a non-root caller): both doors exit 1 with `Permission denied (os error 13)`; nothing is staged and no temp file is left"
  - "the committed home a tracked link to a real file: both doors exit 0; the staged copies are regular files"
  - "the committed home a dangling tracked link: `doc create` exits 0 and mints a regular file; `doc set-slot` exits 1; nothing but regular files in either area"
control: "the rig's own live task before anything is done: 3 regular entries, `doc list --task` exit 0"
observed: "<W>/c.log with <W>/c.runs/ (groups T0, A, B, M, N, F, K1-K4); the previous release: <W>/p.log with <W>/p.runs/"
pinned-by: "UNPINNED: found this round. Searched: no file under crates/cli/tests, crates/engine/tests or tooling-tests names a umask (`command grep -rl umask`, no hit); no suite was run by this verifier"
```

**Pinnable as it stands: partly.** The fixture is a named state of the shared builder and every
step is an argv, so the umask-022 spine and the committed-home variants convert by hand to a test
whose assertion is the shape and mode of each entry of the task's `docs/`. Three conditions: it
needs a Unix target (a mode, a link); the mode-000 variant needs a caller that is not root; and the
077 half needs the test to run the binary under a umask of its own, which an argv alone cannot
say — a `pre_exec` or a shell wrapper in the test. The comparison with the previous release is not
a suite's to hold: a suite drives one binary, and that half is the regression set's.

## Left open

Not pursued; each is for triage like any finding.

1. **A umask that masks the owner's own read bit reaches the state through jigc's verbs alone —
   declared here as a boundary, not as ordinary use.** Under `umask 0477`, in a task whose area
   exists: `jigc doc create adr --title "Zeta choice" --task zeta-two` **exits 0** and leaves
   `adr:zeta-choice.md` and `provenance.json` at mode `--w-------`; `jigc doc set-slot
   research:context-loss#question --from-file - --task zeta-one` exits 1 with `no staged instance
   for `research:context-loss#question` — provision it first …` — a sentence that names the wrong
   cause, since the staged file is there and unreadable — and leaves the same two modes. `jigc doc
   list --task` over either area then exits 1, stdout empty, `reading the staged doc at "<abs
   repo>/.jigc/tasks/<id>/docs/<name>": Permission denied (os error 13)`, plain and json. **The
   previous release does the same, byte for byte** (the six streams compared, the rig's path
   normalised). Why it is a boundary: in a repository of its own, one `git add` under that umask
   leaves `.git/index` unreadable, and `git commit` and `git status` then exit 128 with `index file
   open failed: Permission denied` — the configuration is outside anything git supports.
   (Candidate: labels Z.1 to Z2.listj in `<W>/c.log`; the git check: `<W>/gitumask.*`.)
2. **The committed listing over a tracked dangling link at a managed home.** After the K4 plant —
   a tracked link at `docs/decisions/landed-choice.md` whose target is gone, made by this verifier
   — `jigc doc list` (no `--task`) exits 1, stdout empty, `reading the committed doc at "<abs
   repo>/docs/decisions/landed-choice.md": No such file or directory (os error 2)`: a raw OS error,
   no route, the machine's absolute path. Identical on the previous release (label K4.5-committed).
   One cell, recorded as hit; nothing further was driven.
3. **The copy-in doors over a mode-000 committed doc refuse with a raw OS error and no route**
   (group K2, both doors, exit 1; nothing staged). The edit door's sentence changed between the
   releases and the exit did not.
4. **The merged area's writer is not the atomic primitive.** `crates/engine/src/milestone.rs:2323`
   writes each merged body with `std::fs::write`, which follows a link at its destination. The
   destination is `.jigc/milestones/<id>/merged/docs/` — not a task's staging area, and not
   addressable by `jigc doc list --task` (asked with the milestone's id the door answers
   `finalize.no-task`, exit 1 — seen on the candidate, in the exploration rig) — and `clear_staged_bodies` runs before
   it. Read, not driven with a plant.
5. **Not driven:** Linux; a root caller; a fan-out that lands (its areas are removed by the landing,
   so there is nothing to list); `jigc migrate`'s authoring path and a re-opened (amend) task — both
   were read as reaching the same create and persist primitives, and neither was driven.
6. **The spawned `git` processes were not each read** as possible writers into a staging area; the
   inventories show no entry of that origin.

<!-- end of report -->
