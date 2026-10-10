# verify-real — `r1-p4-finalize-refuses-tracked-link-home-previous-release-landed` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-finalize-refuses-tracked-link-home-previous-release-landed`. One finding,
handed over: ledger key `r1-p4-finalize-refuses-tracked-link-home-previous-release-landed`, door
`jigc task finalize`, the clause it is said to break `working-product`, triage's grade *unclear*.
Its repro is item 2 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-staging-area-writers-undriven-and-unread-remainder.a1.md`
(group MJ of that report's first driver). That report was read because the prompt hands it over as
the finding; no other report was read, and nothing of triage's reasoning beyond the grade and the
re-drive it asks for: *group MJ on both binaries, and the refusal's printed route to its end*.

## Verdict

**`refuted` — basis `intended`.**

What the finding observed is real and reproduces exactly: over a tracked link at a managed doc's
committed home, the candidate's `jigc task finalize <id>` and `jigc task finalize <id> --approve`
both exit 3 with `store.home-not-regular-file`, where the previous release holds (exit 4) and then
exits 0. It is not a defect, and it breaks neither half of the clause:

- **The refusal is a settled decision's.** `design/finalize.md` → 4. Promote: *a promote lands a
  regular file at exactly its canonical path, or the transaction refuses before anything is
  written* (the rc.24 fix pass, `(R6, D-7)`), under the one code ruled on 2026-10-06 (DECISIONS.md →
  *One code for a managed home that is not an ordinary file*). The cell this finding met is named
  there in so many words — *a doc the unit copied in through a live link and edited is
  `edited-from-base`, so it lands over the regular file itself, put where the link was* (**the
  live-link edit path**) — and the migration arm with it (*the in-place migration carve-out
  included*).
- **The route works as printed, to its end.** Driven twice on the candidate (the block as written,
  and the same block with an edit that changes the doc): the regular file put in the link's place,
  then the two commands the route prints, typed as printed — exit 4 (the review hold), then exit 0;
  the doc committed as a regular file at its home, the foreign original retired, the task's area
  gone, the tree clean, the file the link pointed at byte-identical to what it was.
- **What the previous release did at exit 0 is not a landing of the doc.** In the block as written
  the task's staged doc is byte-identical to the committed one (the driver authors it from the same
  payload), so the previous release's commit holds one thing: the deletion of the foreign original.
  With an edit that changes the doc, the previous release writes the edit **through the link** into
  the link's target, commits only the foreign original's deletion, and acknowledges `finalized …
  adopt … as a managed adr` with the target listed under `left-out` — the doc's new bytes are in no
  commit. That is the write-through `(R6, D-7)` closed; the candidate refusing here is the fix.
- **A tracked link at a managed home is not a supported layout.** `design/team-ready-state.md` →
  *Shape is part of membership*: *jigc writes regular files and real directories and never a link*.
  The link is the source verifier's plant (the committed doc moved aside with `git mv`, a link put
  in its place, both committed) — no jigc verb produces it.

`contested: false` — the finding records a difference between two binaries and argues against no
decision.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`. The drivers and the two route drives put the handed binary's directory
  first on `PATH` and stop (exit 90) unless `command -v jigc` prints the binary they were handed;
  every run passed it.
- No `cargo build`, nothing under `target/`. Four rigs, each `SCRATCH=<dir> dev/jigc-rig --binary
  <binary> refs-post-hoc`, stdout captured alone, the construction log to a file of its own, exit 0
  each time.
- The clone: `HEAD` 126a8531 on `fix/canary-one`, no tracked file modified before or after
  (`git diff --quiet HEAD` exit 0); the untracked entries are other reporters' reports under
  `completions/artifacts/canary-one/r1/reports/test/`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0 (Apple Git-157), a non-root caller (uid 501).
Nothing was driven on Linux and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-mj.9ss0me` (written `<W>`). Under it, four fresh rigs and what was read from them:

| rig | binary | driver | what it is |
|---|---|---|---|
| `<W>/c.rig/` → `<W>/c.run/`, `<W>/c.route/` | candidate | `<W>/tools/mj.sh` | group MJ as written, then the route |
| `<W>/p.rig/` → `<W>/p.run/` | previous release | `<W>/tools/mj.sh` | group MJ as written |
| `<W>/c2.rig/` → `<W>/c2.run/`, `<W>/c2.route/` | candidate | `<W>/tools/mj2.sh` | the same with an edit that changes the doc, then the route |
| `<W>/p2.rig/` → `<W>/p2.run/` | previous release | `<W>/tools/mj2.sh` | the same with an edit that changes the doc |

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. Each cell runs in a subshell under the umask the cell names, stdin from
`/dev/null` or a file, stdout and stderr to files of their own, the exit status read directly and
never through a pipe. A *snapshot* after each finalize reads `HEAD`, `git status --porcelain`, the
home's own entry without following a link (`stat -f '%HT|%Sp|%z|%N'`), `readlink`, `git ls-files
-s` of the home and of the link's target, and the task areas that exist.

**What was reconstructed, stated.** The finding's block is a group of a 242-cell driver and has no
standalone form in the report. The source verifier's driver was read for that group's commands
(`<scratch>/verify-p3.NursTz/tools/drive.sh`, lines 1 to 105 — a scratch file, not a report; none
of its rigs or outputs was used). `<W>/tools/mj.sh` keeps of it exactly what MJ needs: the plant of
two foreign notes, group MG — which lands the managed adr `adr:read-only-source` that MJ's link then
replaces — the three `git` commands of the link plant, and group MJ, each with the same argv, umask
and payload. Dropped: every other group, the 1-second sleep and the inode read around the refused
author, the staged listing cells (`jigc doc list --task`, plain and json) and the watch reads of
the rig's own live task — none of them is this finding's subject. `<W>/tools/mj2.sh` is `mj.sh` with
two lines changed: MJ's payload carries another decision sentence and its `set-slot` another line
of context, so that the task's staged doc differs from the committed one.

**A deviation, stated.** The two drivers were written from the shell into `<W>/tools/`; they are
scratch files in no repository. The file tool wrote this report and nothing else.

## 1 · Group MJ as written, on both binaries

The setup, identical on both: `legacy/ro-source.md` (mode 0444) and `legacy/dup-three.md` committed;
group MG under umask 077 — `jigc migrate legacy/ro-source.md --as adr`, `jigc doc author adr` under
the title *Read only source*, the hold, `--approve` (exit 0) — lands
`docs/decisions/read-only-source.md`, a regular file; then `git mv
docs/decisions/read-only-source.md legacy/ros-real.md`, `ln -s ../../legacy/ros-real.md
docs/decisions/read-only-source.md`, `git add`, `git commit` (exit 0 each). The home's entry is then
`Symbolic Link|lrwxr-xr-x|24`, tracked as mode 120000, the tree clean.

| cell (umask 022) | candidate | previous release |
|---|---|---|
| MJ.1 `jigc migrate legacy/dup-three.md --as adr` | exit 0, task minted | exit 0, the same task id |
| MJ.2 `jigc doc author adr --from-file <payload> --task <task>` (the title a managed adr holds — the create door copies in through the link) | exit 0, `adr:read-only-source`; the staged doc a regular file, 213 bytes | exit 0, the same |
| MJ.3 the same with `status: not-a-status` | exit 1 | exit 1 |
| MJ.4 `jigc doc set-slot adr:read-only-source#context --from-file - --task <task>` | exit 0; staged doc 209 bytes | exit 0; 209 bytes |
| MJ.5 `jigc doc list adr --task <task>` · `jigc doc show adr:read-only-source --task <task>` | exit 0 · exit 0 | exit 0 · exit 0 |
| MJ.6 `jigc task validate <task>` | exit 0 | exit 0 |
| **MJ.7 `jigc task finalize <task>`** | **exit 3**, `store.home-not-regular-file`, stdout empty | **exit 4**, *migration review required — nothing committed* |
| MJ.8 `jigc start --task <task>` | exit 0 | exit 0 |
| **MJ.9 `jigc task finalize <task> --approve`** | **exit 3**, the same refusal | **exit 0** |

That is the finding's observation, cell for cell.

**The candidate's refusal, whole** (stderr of MJ.7 and of MJ.9, byte-identical; stdout empty):

```
blocking · store.home-not-regular-file — `docs/decisions/read-only-source.md` is a symbolic link, not a regular file — jigc keeps a managed doc as a regular file at exactly its home, and neither writes through a link nor moves one, so this migration's doc `adr:read-only-source` is not promoted there
  at: docs/decisions/read-only-source.md
  route: nothing was committed and this task's staged docs are intact. jigc writes regular files only, so the link at `docs/decisions/read-only-source.md` is not one it put there, and what becomes of it is yours to decide: put the regular file itself at `docs/decisions/read-only-source.md` — for a link, a copy of the file it points at, in the link's place; then re-run `jigc task finalize migrate-adr-legacy-dup-three-7581e18534b7` to review the fidelity diff and `jigc task finalize migrate-adr-legacy-dup-three-7581e18534b7 --approve` to land it — this task's staged edit lands over it
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

After it, on the candidate: `HEAD` unmoved, `git status --porcelain` empty, the link standing with
the same target, the link's target byte-identical to what it was before MJ, and the task's area
intact with three regular entries. Every sentence of *nothing was committed and this task's staged
docs are intact* holds.

**What the previous release's exit 0 landed.** Its acknowledgement:

```
finalized d27a547 — docs(adr): adopt legacy/dup-three.md as a managed adr
  deleted legacy/dup-three.md
  1 file committed
```

`git show --stat HEAD`: one path, `legacy/dup-three.md`, 3 deletions. No `promoted` row. The home is
still the link, the link's target unchanged, the tree clean. **The reason nothing else moved: in
this block the task's staged doc is byte-identical to the committed doc** (209 bytes each; `cmp`
exit 0 on the candidate's rig, where the area survives) — MJ authors the adr from the payload MG
authored it from and sets the same context line. So the exit 0 the finding compares against is a
commit that retires the foreign note and changes no doc. The block as written cannot show what a
promote through the link does, on either binary; section 3 drives that.

## 2 · The refusal's printed route, to its end (candidate, the block as written)

On the candidate's rig, continuing from MJ.9. The route's first step is prose — *put the regular
file itself at the home; for a link, a copy of the file it points at, in the link's place* — and
was taken as a person would take it; the two commands it prints were then typed as printed.

| step | what was run | exit | what was read |
|---|---|---|---|
| 1 | `cp legacy/ros-real.md <copy>` · `rm docs/decisions/read-only-source.md` · `cp <copy> docs/decisions/read-only-source.md` | 0 · 0 · 0 | the home is `Regular File`, 209 bytes; `git status --porcelain`: ` T docs/decisions/read-only-source.md` |
| 2 | `jigc task finalize migrate-adr-legacy-dup-three-7581e18534b7` | **4** | *migration review required — nothing committed*, the fidelity diff; stderr empty; the status line unchanged |
| 3 | `jigc task finalize migrate-adr-legacy-dup-three-7581e18534b7 --approve` | **0** | below |

```
finalized caa59ec — docs(adr): adopt legacy/dup-three.md as a managed adr
  promoted docs/decisions/read-only-source.md
  deleted legacy/dup-three.md
  2 files committed
```

Afterwards: `git status --porcelain` empty; `git show --stat HEAD` names the home and the deleted
foreign note; the home is a regular file tracked as mode 100644; the task's area is gone; the rig's
own live task is the one area left. The route says *re-run … to review the fidelity diff and …
`--approve` to land it*, and that is what the two commands did.

## 3 · The same block with an edit that changes the doc, on both binaries

`<W>/tools/mj2.sh` on two more fresh rigs. Every cell before MJ.7 exits as in section 1 on both
binaries; the staged doc now differs from the committed one (`cmp` exit 1, 257 bytes against 209).

**Previous release** — MJ.7 exit 4, MJ.9 exit 0:

```
finalized 08639d2 — docs(adr): adopt legacy/dup-three.md as a managed adr
  deleted legacy/dup-three.md
  1 file committed
  left-out (unstaged/untracked — git add to include):
    legacy/ros-real.md
```

`git show --stat HEAD`: `legacy/dup-three.md`, 3 deletions, and nothing else. `git status
--porcelain`: ` M legacy/ros-real.md`. The link's target differs from what it was before MJ (`cmp`
exit 1; `git diff --stat`: 2 insertions, 2 deletions) — the task's edit was written through the
link into a file the task never named, and sits there uncommitted. The home is still the link. The
task's area is gone. The commit's subject says the note was adopted as a managed adr; the adr's new
bytes are in no commit. This is the behaviour `design/finalize.md` describes as what the fix
closed: *what a link at a home can never be is written through*.

**Candidate** — MJ.7 exit 3, MJ.9 exit 3, the refusal of section 1 word for word; `HEAD` unmoved,
the tree clean, the link's target byte-identical to what it was before MJ (`cmp` exit 0), the area
intact. Then the route, as in section 2:

| step | what was run | exit | what was read |
|---|---|---|---|
| 1 | the link replaced by a copy of the file it points at (the same three commands) | 0 · 0 · 0 | ` T docs/decisions/read-only-source.md` |
| 2 | `jigc task finalize <task>` | **4** | the review hold |
| 3 | `jigc task finalize <task> --approve` | **0** | `finalized 1dd8e49 …` · `promoted docs/decisions/read-only-source.md` · `deleted legacy/dup-three.md` · `2 files committed` |

Afterwards: the tree clean; the home a regular file, 257 bytes, byte-identical to the staged edit
saved before the route (`cmp` exit 0), and the blob at `HEAD:docs/decisions/read-only-source.md`
the same bytes; `legacy/ros-real.md` byte-identical to what it was before MJ (`cmp` exit 0); the
task's area gone. *This task's staged edit lands over it* — it did.

## Does it break the clause, inside its scope

The clause's instrument, in the closing condition's words (DECISIONS.md, 2026-10-04, *The exit
rule, revised*, sharpening 2): *no command that works on rc.24 in a supported layout stops working,
and every refusal's route works as printed.*

- **First half.** The command did not *work* on the previous release in this state: with an edit to
  land it committed a claim of adoption without the doc and wrote the doc's bytes into another
  tracked file, uncommitted (section 3); with nothing to land (the block as written) it changed no
  doc at all (section 1). And the state is not a supported layout: a link at a managed doc's home
  is an entry jigc writes at no door (`design/team-ready-state.md` → *Shape is part of
  membership*), planted here by hand through three `git` commands. The refusal that replaces the
  exit 0 is the design's stated contract (`design/finalize.md` → 4. Promote, `(R6, D-7)`; the code
  by the ruling of 2026-10-06; the registration at `design/validation.md`, the row
  `store.home-not-regular-file`).
- **Second half.** The route was driven to its end twice and landed both times (sections 2 and 3):
  exit 4, then exit 0, every sentence of the route borne out.

Neither half is broken. The verdict's basis is `intended` and not `breaks-no-clause`, because there
is no defect left standing for the ledger to carry as a non-blocker: the difference between the two
binaries is the fix of one.

**No regression fact is returned.** Step 4 belongs to `confirmed` only. The previous release was
driven because triage asked for both binaries, and what it showed is stated above as evidence, not
as a `regression` field.

## Scope of what was verified

**`instance, unbounded`.** Driven: one door (`jigc task finalize`, plain and `--approve`); a
migration task into `adr`; a home that is a **live, tracked** link to a tracked regular file inside
the repository; the doc copied in through it (`edited-from-base`), once with an edit that changes
nothing and once with one that does; two binaries; one platform; a non-root caller. Not driven: an
ordinary (non-migration) task over the same link on either binary; a dangling link, a link out of
the repository, a directory or a special file at the home; any other of the fifteen commands the
code's registration lists; any other doctype; `--format json`. The mechanism's consumers were not
enumerated by this verifier.

## Repro MJ-1

```yaml
claim: "over a tracked link at a managed doc's committed home, the candidate's `jigc task finalize` refuses where the previous release landed — a command that worked on the previous release stops working, or a refusal whose route does not work"
verdict: REFUTED   # basis intended — design/finalize.md → 4. Promote, (R6, D-7); DECISIONS.md 2026-10-06, One code for a managed home that is not an ordinary file
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the first finalize exits 4 and --approve exits 0; the commit holds only the foreign note's deletion, and with an edit that changes the doc the edit is written through the link into legacy/ros-real.md and left uncommitted"
platform: "macOS 26.6.2 arm64, git 2.54.0, a non-root caller; NOT driven on Linux, NOT driven as root"
setup:
  - fixture: refs-post-hoc
  - write: "legacy/ro-source.md — `# Read only source`, a blank line, one paragraph; legacy/dup-three.md — a title line, a blank line, one paragraph"
  - ["git", "add", "legacy"]
  - ["git", "commit", "-q", "-m", "docs: foreign notes for the migration probes"]
  - ["jigc", "migrate", "legacy/ro-source.md", "--as", "adr"]                     # prints `task minted: <first>`
  - ["jigc", "doc", "author", "adr", "--from-file", "<payload: title Read only source; context, decision, consequences slots>", "--task", "<first>"]
  - ["jigc", "task", "finalize", "<first>", "--approve"]                          # exit 0: docs/decisions/read-only-source.md, a regular file
  - ["git", "mv", "docs/decisions/read-only-source.md", "legacy/ros-real.md"]
  - symlink: "docs/decisions/read-only-source.md -> ../../legacy/ros-real.md"
  - ["git", "add", "docs/decisions/read-only-source.md"]
  - ["git", "commit", "-q", "-m", "docs: the adr home is now a link to the real file"]
  - ["jigc", "migrate", "legacy/dup-three.md", "--as", "adr"]                     # prints `task minted: <task>`
  - ["jigc", "doc", "author", "adr", "--from-file", "<payload: the same title, ANOTHER decision sentence>", "--task", "<task>"]   # exit 0: copied in through the link
repro:
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - replace: "docs/decisions/read-only-source.md — the link removed, a copy of legacy/ros-real.md put in its place (the route's first step)"
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
expect:
  - exit: 3
    stdout: ""
    stderr_contains: "blocking · store.home-not-regular-file — `docs/decisions/read-only-source.md` is a symbolic link, not a regular file"
    stderr_contains_also: "then re-run `jigc task finalize <task>` to review the fidelity diff and `jigc task finalize <task> --approve` to land it"
    tree: "HEAD unmoved; `git status --porcelain` empty; the home is still the link; legacy/ros-real.md holds the bytes it held; the task's area holds its three regular entries"
  - exit: 3
    stderr_contains: "blocking · store.home-not-regular-file"
    tree: "the same — in particular legacy/dup-three.md still exists"
  - tree: "`git status --porcelain` is ` T docs/decisions/read-only-source.md`"
  - exit: 4
    stdout_contains: "migration review required — nothing committed"
  - exit: 0
    stdout_contains: "promoted docs/decisions/read-only-source.md"
    stdout_contains_also: "2 files committed"
    tree: "`git status --porcelain` empty; the home is a regular file tracked as mode 100644 and holds the staged edit's bytes; legacy/ros-real.md holds the bytes it held before the task; legacy/dup-three.md is gone; the task's area is gone"
variants:
  - "the block as written by the source report — the second payload identical to the first, a `set-slot` of the same context line: the same five exits (3, 3, 4, 0), and the landing commit still carries the home's change from a link to a regular file"
control: "before the link plant, `jigc task finalize <first> --approve` exits 0 over a free home — the door lands where the home is free or a regular file"
observed: "<W>/c2.run/log with <W>/c2.run/cells/, and <W>/c2.route/; the block as written: <W>/c.run/ and <W>/c.route/; the previous release: <W>/p.run/ and <W>/p2.run/"
pinned-by: "UNPINNED: the exact cell — a migration whose authored doc is copied in through a live link — has no test of its own. Each of its two axes has one, read and not run by this verifier: promote_destination_shape::an_edit_of_a_doc_whose_home_is_a_live_link_is_refused_not_written_through (a record-decision task, a live tracked link, the route followed to a landing) and promote_destination_shape::a_migration_never_lands_through_a_link_at_its_home (a migration into `idea`, a dangling link, a minted doc, plain and --approve, the route's --approve followed)"
```

**Pinnable as it stands: yes, on a Unix target.** The fixture is a named state of the shared
builder, every step is an argv or one file-system act the neighbouring tests already perform
(`symlink`, `remove_file`, `copy`), and the assertions are exits, two stderr substrings and tree
facts. Two conditions: the migration's task id carries a suffix the test must read from the mint's
output and not hard-code (the suite's `migrate_task` helper does that), and the comparison with the
previous release is not a suite's to hold — a suite drives one binary. Whether the cell earns a
test of its own beside the two that cover its axes is the fixer's or triage's call.

## Left open

Not pursued; each is for triage like any finding.

1. **No single test drives this finding's exact cell** — a migration whose authored doc is copied
   in through a live link at a managed home. The two tests named under `pinned-by` cover its axes
   separately. Read from the suite's source, not from a run: this verifier ran no suite.
2. **The route leaves two tracked files with one doc's history.** After the route's landing the
   home is the regular file and `legacy/ros-real.md` — the file the link pointed at — is still
   tracked, holding the doc's earlier bytes. The route says *what becomes of it is yours to
   decide*, and the suite asserts that file is left as it was; recorded as an observation about
   what a person holds afterwards, not as a defect.
3. **Between the route's first step and its landing the home is an uncommitted change of type**
   (` T` in `git status --porcelain`), and the finalize lands over it without a word about the
   tree. That is the route as designed for a committing door (`design/finalize.md`: *lands over the
   regular file itself, put where the link was*); whether the finalize's dirty-tree handling is
   stated anywhere for a type change was not read.
4. **Not driven:** an ordinary (non-migration) task over the same link on the previous release —
   `design/finalize.md` says its commit was *rejected for holding nothing*, which is a different
   exit from the one this finding compares against; Linux; a root caller; `--format json`.

<!-- end of report -->
