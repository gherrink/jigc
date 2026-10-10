# verify-real — `r1-p3-staged-doc-ids-orientation-and-milestone-callers` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-staged-doc-ids-orientation-and-milestone-callers`. One finding, handed over:
ledger key `r1-p3-staged-doc-ids-orientation-and-milestone-callers`, door `jigc start` (orientation's
staged list, `orient.rs:212`) and the milestone door (`milestone.rs:6099`, `milestone.rs:5897`), the
clause it is said to break `no-lost-files`, triage's grade *unclear*. It has no block of its own. Its
plants are those of `Repro V-2` in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-staged-doc-ids-other-callers-not-driven.a1.md`,
which the prompt handed over as this finding's source and which was read for the plants and for its
`Scope of what was verified`. No other report and nothing of triage's reasoning was read.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`. Neither door destroys anything without the
consent; two surface defects at the same doors are real, and stay rows of the ledger.**

- **`jigc start` (orientation) destroys nothing under either plant.** It is a read door and has no
  forced form. It exits 0 in every cell; nothing under `.jigc/` is removed or modified, `HEAD` and
  the porcelain set of tracked and untracked files do not move. The one change it makes is a derived
  cache it adds in some cells (`.jigc/index/edges.json`, 73 bytes), in the no-plant control as well.
- **`jigc milestone discard <id>` fails closed under both plants.** With each plant alone in a
  sub-task's staging area the unforced door exits 1 with a blocking finding that carries a route,
  and leaves `.jigc/`, the porcelain set, `HEAD`, the commit count and the worktree list as they
  were. Four cells on the candidate, the same four on the previous release, eight of eight.
- **The forced form destroys at exit 0, and that is the consent the design names.**
  `design/write-commands.md` → *Abandoning a milestone*: *It refuses on three subjects, and `--force`
  is the single consent for all of them.* The first clause's scope (DECISIONS.md, 2026-10-04, *The
  exit rule, revised*, sharpening 1) is destruction at exit 0 that the user did not ask for.
- **Defect 1, real: under the dangling link the forced milestone door does not name the staged doc
  it takes.** `jigc milestone discard cache-rework --force` prints the working-area warning for the
  link and **no staged-docs warning**, while `adr:low-policy` goes with the area. With an ordinary
  foreign file in place of the link (this verifier's control) it prints both warnings. In that cell
  the staged doc is named by nothing at all: the unforced refusal is `milestone.foreign-bytes`, asked
  first, and names only the link.
- **Defect 2, real: under the dangling link orientation says a task that stages docs stages
  nothing.** The row reads `staged:   nothing staged yet`, the JSON row `"staged": []`, and the
  abandon directive is printed without the consent — `jigc task discard <id>` — *abandon: removes the
  working area*. Run as printed, that command refuses (exit 1, `task-discard.foreign-bytes`), so the
  wrong line leads to a closed door and not to a loss.
- **Neither breaks the clause inside its scope.** No command at exit 0 destroys bytes without the
  consent, and nothing is committed that was not asked for. What is wrong is what two surfaces say.
  The previous release says the same, byte for byte.

`contested: false` — the finding does not argue that a settled decision is wrong.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (1.0.0-rc.24).
- The binary's directory went first on `PATH` in every driver call, and each call stops unless
  `command -v jigc` prints the binary it was handed. For the candidate that printed
  `<scratch>/bin/c1.a1/jigc`.
- No `cargo build`, nothing under `target/`. Every rig: `SCRATCH=<W>/rigs dev/jigc-rig --binary <binary> refs-post-hoc`,
  stdout captured alone, the construction log to its own file, exit status 0 each time.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and after,
  `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## What was reconstructed, and what was changed

The finding has no repro block, so the smallest setup that reaches each door was built:

- **Orientation.** The fixture of Repro V-2 as it stands (`refs-post-hoc`, the live task
  `ground-the-vision-in-research`, written `T`), the plant in `.jigc/tasks/T/docs/`, then
  `jigc start` and `jigc start --format json`. A second set puts the plant in a milestone
  sub-task's area, because orientation lists sub-tasks as well.
- **The milestone door.** No named state of the rig holds a milestone, so one was made on top of
  `refs-post-hoc` with the binary under test, four commands, each exit status 0:
  `jigc milestone create "Cache rework"`, `jigc milestone add-task cache-rework "Area low"`,
  `jigc milestone add-task cache-rework "Area zed"`, `jigc doc create adr --title "Low policy" --task area-low`.
  That leaves sub-task `area-low` staging `adr:low-policy` and sub-task `area-zed` staging nothing;
  the plant goes into `.jigc/tasks/area-low/docs/`. The door driven is
  `jigc milestone discard cache-rework`, bare and with `--force`: it is the one caller of both
  code sites the finding names (`refuse_over_subtask_staged_prose`, the guard, and
  `pending_staged_prose`, the forced narration). No worktree is provisioned in these cells; one
  pair of cells with `jigc milestone provision cache-rework` added shows the same answers.
- Nothing was written into `.jigc/` by hand except the plant itself.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user (uid 501).**
Nothing was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p3-orient-milestone.YOTyiW` (written `<W>`). **38 rigs were driven, 19 per binary**,
each from the rig tool's own `mktemp -d` under `<W>/rigs/`; two more were built first to learn the
milestone setup and are not evidence. Every cell that ends in a destruction has a rig of its own.
A second measured invocation in one rig happens only where the first changed nothing a later one
reads: the JSON form of orientation after the text form, and a refusal's routes after the refusal.
Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's repository. Driver `<W>/tools/cell.sh`:
build the rig (`<W>/tools/mkrig.sh`), make the one plant, snapshot, run the argv with stdin from
`/dev/null` and stdout and stderr to their own files, read the exit status directly (never through
a pipe), snapshot again. The snapshot (`<W>/tools/snap.sh`) is one line per entry under `.jigc/` —
kind, mode, size and the first 16 hex digits of its sha256, or the link's target, or `UNREADABLE`
for the mode-000 file — then `git status --porcelain --untracked-files=all --ignored`, `HEAD`,
`git rev-list --count HEAD` and `git worktree list --porcelain`. Follow-on commands went through
`<W>/tools/again.sh`, the same measurement without a new rig.

The plants, each alone, in the `docs/` directory of the task named:

- **dangling** — `ln -s /nonexistent/x.md research:dangling.md`
- **locked** — a 29-byte regular file `research:locked.md`, then `chmod 000`
- **foreign** (this verifier's control, not the finding's) — a regular file `notes.txt`, mode 644

## What was driven — the candidate

Per-cell files under `<W>/runs/cand-*/` (`stdout`, `stderr`, `exit`, `snap.before`, `snap.after`,
`snap.diff`, `build.log`, `argv`, `rig`).

### `jigc start` — the plant in the top-level task `T`

| cell | plant | exit | the row's `staged:` line | the abandon directive | `findings:` | tree |
|---|---|---|---|---|---|---|
| cand-orient-none | none | 0 | `commit:T, vision:vision` | `jigc task discard T --force` — *removes the working area and the doc(s) staged in it …* | 2 blocking, 1 advisory | one file added: `.jigc/index/edges.json`; nothing else |
| cand-orient-dangling | dangling | 0 | **`nothing staged yet`** | **`jigc task discard T`** — *abandon: removes the working area* | 2 blocking, 1 advisory | the same one file added; the link and both staged docs standing |
| cand-orient-locked | locked | 0 | `commit:T, research:locked, vision:vision` | `jigc task discard T --force` | `unknown — enumerating the task's code-anchor surface at "<absolute path of the task area>": Permission denied (os error 13)` | identical |
| cand-orient-foreign | foreign | 0 | `commit:T, vision:vision` | `jigc task discard T --force` | 2 blocking, 1 advisory | the same one file added |

stderr is empty in every cell. The text output of the foreign cell is byte-identical to the
control's once the rig's path and the base pin's short hash are normalised (`cmp`); the dangling
cell differs from the control in exactly the two lines the table shows (`diff`).

`jigc start --format json`, second in each rig: exit 0, tree identical in all four. The task's row
carries `"staged": ["commit:T", "vision:vision"]` (none, foreign), **`"staged": []`** (dangling), and
the three ids with `"findings": null` plus `findings_unavailable` (locked).

### `jigc start` — the plant in the sub-task `area-low`

| cell | plant | exit | `area-low`'s `staged:` line | its abandon directive | tree |
|---|---|---|---|---|---|
| cand-orient-sub-none | none | 0 | `adr:low-policy` | `jigc task discard area-low --force` | `.jigc/index/edges.json` added |
| cand-orient-sub-dangling | dangling | 0 | **`nothing staged yet`** | **`jigc task discard area-low`** | the same |
| cand-orient-sub-locked | locked | 0 | `adr:low-policy, research:locked` | `jigc task discard area-low --force` | the same |

The other two rows (`area-zed`: nothing staged; `T`: its two ids) are the same in all three cells.
The JSON form agrees: `"staged": ["adr:low-policy"]`, `[]`, `["adr:low-policy", "research:locked"]`.

### `jigc milestone discard cache-rework` — without the consent

| cell | plant | exit | stdout | stderr | `.jigc/`, porcelain, `HEAD`, commit count, worktrees |
|---|---|---|---|---|---|
| cand-ms-none-discard | none | 1 | empty | `blocking · milestone.staged-prose` — *1 sub-task(s) stage 1 doc(s)*: `area-low: adr:low-policy`; a route (661 bytes) | identical |
| cand-ms-dangling-discard | dangling | **1** | empty | `blocking · milestone.foreign-bytes` — *holds 1 path(s) jigc did not write*: `.jigc/tasks/area-low/docs/research:dangling.md`; a route (632 bytes) | identical, the link standing |
| cand-ms-locked-discard | locked | **1** | empty | `blocking · milestone.staged-prose` — *stage 2 doc(s)*: `area-low: adr:low-policy, research:locked`; a route (678 bytes) | identical, the file standing at mode 000 |
| cand-ms-foreign-discard | foreign | 1 | empty | `blocking · milestone.foreign-bytes` — `.jigc/tasks/area-low/docs/notes.txt`; a route (621 bytes) | identical |
| cand-msprov-dangling-discard | dangling, worktrees provisioned | 1 | empty | the same `milestone.foreign-bytes` text (632 bytes) | identical |

The dangling link does not reach the staged-docs probe at this door: the foreign-bytes guard is
asked first and classifies it foreign. The mode-000 file is not an error to the probe: the probe
asks for the entry's shape and not its bytes, so the file is listed as the staged identity
`research:locked`. Both are what `crates/cli/tests/destroying_door_sibling_surfaces.rs` states in its
header for this family of doors (*the foreign refusal answers first, every time*).

### `jigc milestone discard cache-rework --force` — with the consent

| cell | plant | exit | the ack (stdout) | stderr | after |
|---|---|---|---|---|---|
| cand-ms-none-discard-force | none | 0 | `discarded milestone:cache-rework (2 sub-task(s); workbench removed)` (141 bytes) | *discarding milestone:cache-rework discards the staged docs of 1 open task(s)*: `area-low: adr:low-policy` (231 bytes) | the milestone area, `area-low` and `area-zed` gone (17 entries); one commit, the record alone |
| cand-ms-dangling-discard-force | dangling | 0 | the same ack | *removing the working area .jigc/tasks/area-low discards work that is not in git*: `.jigc/tasks/area-low/docs/research:dangling.md` — **and no staged-docs warning** (228 bytes) | the same areas gone (18 entries): the link, **and `adr:low-policy`, named nowhere** |
| cand-ms-locked-discard-force | locked | 0 | the same ack | the staged-docs warning: `area-low: adr:low-policy, research:locked` (248 bytes) | the same areas gone (18 entries) |
| cand-ms-foreign-discard-force | foreign | 0 | the same ack | the staged-docs warning naming `adr:low-policy` **and** the working-area warning naming `notes.txt` (448 bytes) | the same areas gone (18 entries) |
| cand-ms-none-discard-force-json | none | 0 | `{"hook_output": "", "text": "discarded milestone:cache-rework (2 sub-task(s); workbench removed)"}` (105 bytes) | as the text cell | as the text cell |
| cand-ms-dangling-discard-force-json | dangling | 0 | the same JSON, byte for byte | the link's warning only | as the text cell |
| cand-msprov-dangling-discard-force | dangling, worktrees provisioned | 0 | the same ack | the link's warning only (228 bytes) | the areas and both worktrees gone |

In every forced cell: `HEAD` advances by exactly one commit, `chore(milestone): discard record for
milestone:cache-rework`, whose only path is `docs/milestone-records/cache-rework.md` — the settle of
the record the door is documented to make; `.jigc/state/file-state.json` is rewritten at the same
size; the unrelated task `T` keeps its area, its three `docs/` files unchanged. The JSON ack of
this door carries no list of what was taken in any cell, the control included — the names are on
stderr only.

### The printed routes, run as printed

Each in the rig of the cell that printed it, whose state that cell had left as it was.

| printed by | the route's command | exit | what it printed |
|---|---|---|---|
| orientation, dangling in `T` | `jigc task discard T` | **1** | `blocking · task-discard.foreign-bytes` naming the link, with a route (695 bytes); tree identical |
| orientation, dangling in `T` | `jigc start --task T` | 0 | the task's workflow, re-composed (7184 bytes) |
| orientation, dangling in `T` | `jigc task validate T` | 3 | the two blocking findings and the advisory of the control (1111 bytes) |
| orientation, dangling in `area-low` | `jigc task discard area-low` | **1** | `blocking · task-discard.foreign-bytes` naming the link (590 bytes); tree identical |
| the dangling refusal | the link deleted by hand, then `jigc milestone discard cache-rework` | 1 | `milestone.staged-prose` — byte-identical to cand-ms-none-discard (`cmp`) |
| the locked refusal | `jigc doc show research:locked --task area-low` | **1** | `blocking · store.not-staged` — *is not staged in this task and has no committed copy* (320 bytes). The refusal says the sub-task stages it; the read it routes at says it does not. |
| the locked refusal | `jigc doc show adr:low-policy --task area-low` | 0 | the staged doc (135 bytes) |
| the locked refusal | `jigc milestone finalize cache-rework` | 1 | `blocking · join.missing-provenance` — *staged doc `research:locked` … has no recorded provenance*; its route names no command (225 bytes); nothing removed |
| the control refusal | `jigc milestone finalize cache-rework` | 3 | three `required-slot-present` findings over `adr:low-policy`, each with its route (1122 bytes) |
| every refusal | its `--force` form | 0 | the cells of the table above |

Orientation's `jigc task finalize T` directive and the forced task door under these plants were
not run here; the forced task door is the earlier report's.

## Against the claim, and against the design that owns the behaviour

**The claim**, as triage put it: under each plant, does each door fail closed, and does a forced
form name what it takes.

- *Orientation* has nothing to fail closed over: it removes and commits nothing, and it has no
  forced form. What it gets wrong is its own report, under the link only.
- *The milestone door* fails closed in every cell. Its forced form names what it takes in three
  cells of four and does not under the dangling link.

**The design.**

- `design/write-commands.md` → *Abandoning a milestone*: the door refuses on a dirty worktree, on
  staged docs (`milestone.staged-prose`) and on bytes jigc did not write (`milestone.foreign-bytes`),
  *and `--force` is the single consent for all of them*. Driven as written.
- `design/team-ready-state.md` → *The lifecycle*, *The workbench is actually removed*: *the abandon
  now names every staged doc of **its own** sub-tasks before removing their areas, through the
  emitter `uninstall` already uses*. Under the link the forced door does not. The code's own
  contract for that emitter is the comment over `pending_staged_prose` in `crates/cli/src/setup.rs`:
  *Best-effort, like every narration: an in-scope area it cannot read yields no warning rather than
  failing a teardown the guards already cleared … the declared bound stays visible, not prevented.*
  It takes the probe's error as an empty list; in the dangling cell the bound is not visible.
- `design/bootstrap.md` → orientation, state 3: *Over a task staging nothing … the flag is not
  printed: a route this binary's own guard blocks is a route-floor defect … so the printed argv is
  the one that runs in each state.* Under the link orientation prints the argv without the flag
  over a task that stages two docs, and this binary's guard blocks it. The comment over
  `active_tasks` in `crates/cli/src/orient.rs` gives the rule the row is built under — *Nothing here
  can fail the door. A working area that will not answer degrades to the absent case for that
  fact* — and the staged list is taken with `.unwrap_or_default()`. For `findings` the same door
  renders *unknown* and says why; for `staged` the unreadable case is rendered as *nothing staged
  yet*.

This verifier found no dated decision that rules what either surface says when the probe errors on
one entry, so the basis is `breaks-no-clause` and not `intended`.

**The clause.** DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 1: *In a healthy
repository used as documented … no jigc command at exit 0 destroys bytes no git object holds or
commits content the user did not ask for. Where jigc cannot tell … it refuses before writing. …
deliberately planted states are declared bounds.* Read against the cells:

- orientation: exit 0, nothing destroyed, nothing committed; its wrong directive, run as printed,
  is refused before anything is written;
- the milestone door without the consent: exit 1, nothing destroyed, nothing committed;
- with the consent: the destruction the user asked for by the word the refusal printed, and one
  commit, the record's settle, which is what the verb is documented to commit;
- the missing names: a surface that under-reports a consented removal.

**The strongest reading against this verdict, stated so that triage can weigh it.** In the dangling
cell the consent is given to a refusal that named one path, the link, and the forced door then
takes a staged doc that neither the refusal nor the forced warning ever named. In the foreign-file
control the refusal is just as narrow — the foreign-bytes guard answers first there too — but the
forced door then names the staged doc. So the difference between the two cells is the narration
alone; the consent, and what it removes, are the same, and the route's own text says `--force`
*settles the record and tears the workbench down*. That is why the row is graded a narration
defect here and not an unconsented loss.

The verdict does not rest on the plants being *deliberately planted states*, and the run's opening
record declares no bound. Whether ordinary use can leave a link or an unreadable file in a staging
area was not driven here: both plants were made by hand.

## The same cells on the previous release

The verdict is not `confirmed`, so no `regression` field is returned. Every cell and every route
drive was run on the previous release for comparison — 19 rigs, 35 measured invocations, per-cell
files under `<W>/runs/prev-*/`.

- **Every exit status is the same**, in all 35.
- **The milestone door is byte-identical**, stdout and stderr, in all eleven of its cells and in
  its five route drives, the rig's path normalised (`cmp`): the same refusals, the same forced
  warnings, the staged-docs warning absent under the link there too.
- **Orientation is byte-identical** in all fourteen of its invocations once the rig's path and the
  base pin's hash (different in every rig) are normalised (`cmp`): `nothing staged yet`, `"staged": []`
  and the directive without `--force` under the link, on both binaries.
- The orientation route drives are byte-identical as well.

## Scope of what was verified

**Instance.** Driven: two doors — `jigc start` in its text and JSON forms, and
`jigc milestone discard`, bare and forced, text and JSON — two plants and two controls, the plant
in a top-level task and in a milestone sub-task, on two binaries, on one platform, plus the routes
above.

**The callers were counted, and with this report each has been driven once by some verifier of this
round or is named as not driven.** `grep -rn "staged_doc_ids" crates/cli/src` gives five call sites
outside comments and tests: `task.rs:1301` (inside the guard probe `staged_task_prose`),
`task.rs:1582`, `orient.rs:212`, `doc.rs:5023`, `doc.rs:5142`.
`grep -rn "staged_task_prose\|pending_staged_prose(" crates/cli/src` gives the guard probe's
consumers: `task.rs:1480`, `milestone.rs:6099`, `setup.rs:4903`, and the narration helper
`pending_staged_prose` (`setup.rs:6406`) with its two callers `setup.rs:6165` and `milestone.rs:5897`.
This report drove `orient.rs:212`, `milestone.rs:6099` and `milestone.rs:5897`; the source report says
which of the others it drove. One further reader was found on the way and driven with
orientation: the abandon directive in `crates/cli/src/render.rs`, which chooses its argv from the
same list. **The plants are not bounded**: a link to a directory, a link loop, a fifo, a directory
named as a staged doc and an unreadable `docs/` directory were not driven at these two doors —
`instance, unbounded` on that axis.

## Repro V-3

```yaml
claim: "with a dangling link or a mode-000 file named as a staged doc alone in a staging area, `jigc start` and `jigc milestone discard <id>` destroy the area or the plant at exit 0, or the milestone door does not fail closed"
verdict: REFUTED   # basis breaks-no-clause: orientation removes nothing, the milestone door refuses at exit 1 and changes nothing; the forced form destroys by consent and, under the dangling link, does not name the staged doc it takes; orientation reports that task as staging nothing
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in every cell; both doors byte-identical once the rig's path and the base pin's hash are normalised (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging commit:<id> and vision:vision
  - ["jigc", "milestone", "create", "Cache rework"]
  - ["jigc", "milestone", "add-task", "cache-rework", "Area low"]
  - ["jigc", "milestone", "add-task", "cache-rework", "Area zed"]
  - ["jigc", "doc", "create", "adr", "--title", "Low policy", "--task", "area-low"]
  - ["ln", "-s", "/nonexistent/x.md", ".jigc/tasks/area-low/docs/research:dangling.md"]
repro:                                      # 1 and 2 may share a fixture; 3 and 4 each from a fresh one
  - ["jigc", "start"]
  - ["jigc", "start", "--format", "json"]
  - ["jigc", "milestone", "discard", "cache-rework"]
  - ["jigc", "milestone", "discard", "cache-rework", "--force"]
expect:
  - exit: 0
    stderr: ""
    stdout_contains: ["Active task: area-low", "  staged:   nothing staged yet", "Run: `jigc task discard area-low`   — abandon: removes the working area"]   # TODAY's answer: the task stages adr:low-policy
    tree: "nothing under .jigc/ removed or modified (at most .jigc/index/edges.json added); porcelain of the rest and HEAD unchanged"
  - exit: 0
    stdout_json: { "tasks": [ { "id": "area-low", "milestone": "cache-rework", "staged": [] } ] }   # TODAY's answer, the same silence
    tree: "identical"
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · milestone.foreign-bytes", ".jigc/tasks/area-low/docs/research:dangling.md", "jigc milestone discard cache-rework --force"]
    tree: "every entry under .jigc/ identical before and after (kind, mode, size, link target, content); porcelain, HEAD and the commit count unchanged"
  - exit: 0
    stdout_contains: ["discarded milestone:cache-rework (2 sub-task(s); workbench removed)"]
    stderr_contains: [".jigc/tasks/area-low/docs/research:dangling.md"]
    stderr_lacks: ["discards the staged docs of"]   # TODAY's answer: adr:low-policy goes with the area and is not named
    tree: ".jigc/tasks/area-low/, .jigc/tasks/area-zed/ and .jigc/milestones/cache-rework/ gone; .jigc/tasks/ground-the-vision-in-research/ unchanged; HEAD one commit ahead, touching docs/milestone-records/cache-rework.md alone"
variants:        # each alone, in place of the `ln`
  - "a mode-000 research:locked.md  -> orientation: exit 0, staged lists `adr:low-policy, research:locked`, `findings: unknown`; bare door: exit 1, `milestone.staged-prose` naming both ids, tree unchanged; forced door: exit 0, the staged-docs warning names both"
  - "a regular notes.txt            -> bare door: exit 1, `milestone.foreign-bytes` naming notes.txt; forced door: exit 0, the working-area warning names notes.txt AND the staged-docs warning names adr:low-policy"
  - "the `ln` into .jigc/tasks/ground-the-vision-in-research/docs/ with no milestone -> orientation: exit 0, that task's row reads `nothing staged yet` and its directive is `jigc task discard ground-the-vision-in-research`, which exits 1 with `task-discard.foreign-bytes`"
control: "no plant -> orientation: `staged:   adr:low-policy`, directive with `--force`; bare door: exit 1, `milestone.staged-prose` over `area-low: adr:low-policy`; forced door: exit 0, the staged-docs warning names it"
observed: "<W>/runs/cand-*/ (stdout, stderr, exit, snap.before, snap.after, snap.diff, build.log); the previous release: <W>/runs/prev-*/"
pinned-by: "UNPINNED: found this round. A name search (`grep -rln 'nothing staged yet' crates/cli/tests tooling-tests`) gives no suite at all; the suites that name `milestone.staged-prose` or `milestone.foreign-bytes` and mention a symlink are milestone_merged_complement.rs, flow53_acceptance.rs and destroying_door_sibling_surfaces.rs, and the last states the dangling-link case in its header as driven by hand at the task door, not as a test. No suite was run by this verifier, and no test's assertions were read beyond that header."
```

**Pinnable as it stands: expectation 3 yes; expectations 1, 2 and 4 only as a statement of today's
behaviour.** The fixture is a named state of the shared builder plus four argv steps and one
filesystem call, so the block converts by hand. Conditions: a Unix target (a link, a mode); the
mode-000 variant needs a caller that is not root for orientation's `findings: unknown` line, while
the door's own answers do not read the file's bytes and are expected to hold for root as well —
not driven. Expectation 3, the refusal with the tree unchanged, is the half worth a standing test:
it is the property the clause asks for. Expectations 1, 2 and 4 pin a silence that a repair of
this row would change on purpose; whoever converts them should write them as the red test of that
repair — the row naming `adr:low-policy`, the directive carrying `--force`, the forced warning
naming the doc — and not as a fact to keep. What survives a repair unchanged in them is the exit
status and the `tree:` line. The comparison with the previous release is not a suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **Orientation reports a task that stages docs as staging nothing, and prints a route its own
   guard blocks.** Under the dangling link: `staged:   nothing staged yet`, `"staged": []`, and
   `jigc task discard <id>` without `--force`, which exits 1 with `task-discard.foreign-bytes`.
   Against `design/bootstrap.md` (*the printed argv is the one that runs in each state*). Identical
   on the previous release. It destroys nothing; whether it is the second clause's subject is
   triage's to say.
2. **The forced milestone door's missing names under the dangling link** — the defect this
   report's verdict leaves on the ledger: no staged-docs warning at
   `jigc milestone discard <id> --force` while a staged doc goes with the area, and in that cell the
   unforced refusal names only the link, so the doc is named neither before nor after the consent.
   Identical on the previous release. The same mechanism as the two doors of the source report.
3. **The milestone door's JSON ack names nothing it takes in any cell** — `text` and `hook_output`
   only, the control included; the names are on stderr. Not compared against
   `design/command-output-contract.md` by this verifier.
4. **A refusal names a staged doc that its own read route says is not staged, at the milestone door
   too.** Under the mode-000 plant `milestone.staged-prose` names `research:locked` and routes at
   `jigc doc show <address> --task <sub-task-id>`; that command exits 1 with `store.not-staged`.
   Identical on the previous release.
5. **`jigc milestone finalize <id>` under the mode-000 plant** exits 1 with `join.missing-provenance`,
   whose route — *re-stage the doc so its provenance is recorded* — names no command. It is a route
   the staged-prose refusal prints. Identical on the previous release.
6. **Orientation prints the machine's absolute path under the mode-000 plant** — `findings: unknown —
   enumerating the task's code-anchor surface at "<absolute path>": Permission denied (os error 13)`,
   in the text row and in `findings_unavailable`. Identical on the previous release.
7. **Other plants at these two doors** — a link to a directory, a link loop, a fifo, a directory
   named as a staged doc, an unreadable `docs/` directory — were not driven.
8. **Reach was not driven.** Whether any jigc verb, or ordinary use, can leave a link or an
   unreadable file in a staging area was not tested; the verdict does not depend on it.
9. **Linux, and a root caller, were not driven.**

<!-- end of report -->
