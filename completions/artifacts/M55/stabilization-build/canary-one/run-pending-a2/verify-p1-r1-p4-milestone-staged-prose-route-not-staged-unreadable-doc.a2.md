# verify-real — `r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc`. One finding, handed over:
ledger key `r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc`, door `jigc milestone discard`,
the clause it is said to break `working-product`, triage's grade *unclear*. Its repro is item 4 of
*Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-staged-doc-ids-orientation-and-milestone-callers.a1.md`,
which the prompt handed over as this finding's source and which was read whole for the plant and the
setup. No other report and nothing of triage's reasoning beyond the grade was read.

The finding, in the source's words: *a refusal names a staged doc that its own read route says is
not staged, at the milestone door too. Under the mode-000 plant `milestone.staged-prose` names
`research:locked` and routes at `jigc doc show <address> --task <sub-task-id>`; that command exits 1
with `store.not-staged`. Identical on the previous release.*

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`. The behaviour is real and reproduces exactly;
it is a row of the ledger, not a break of `working-product` inside that clause's scope.**

- **It reproduces, from nothing, on the candidate.** With a hand-written file `research:locked.md`
  at mode 000 in sub-task `area-low`'s staging area, `jigc milestone discard cache-rework` exits 1
  with `blocking · milestone.staged-prose`, names `area-low: adr:low-policy, research:locked`, and
  routes at `jigc doc show <address> --task <sub-task-id>`. Run as printed for the doc it named,
  `jigc doc show research:locked --task area-low` exits 1 with `blocking · store.not-staged` —
  *`research:locked` is not staged in this task and has no committed copy — nothing to read yet*,
  route *create or author the doc in this task first — a staged copy exists only after a write*.
  One refusal says the sub-task stages the doc; the read it routes at says it does not.
- **It is the unreadable bytes, and nothing else about the plant.** The same hand-written file at
  mode 644 (this verifier's control) is answered truthfully by the same read:
  `blocking · store.unparseable` — *`research:locked` at `.jigc/tasks/area-low/docs/research:locked.md`
  does not parse: required section heading `## question` is missing*. And a doc jigc itself staged,
  `adr:low-policy`, answers `store.not-staged` in the same words once its file is set to mode 000
  (this verifier's variant). So the contradiction is the read door's answer to a staged file whose
  bytes the caller cannot read.
- **Nothing that works on the previous release stops working.** All sixteen measured invocations
  have the same exit status on the previous release, and stdout and stderr are byte-identical in
  all sixteen once the rig's path is normalised.
- **Nothing is destroyed or written by the door or by the read route.** Every entry under `.jigc/`,
  the porcelain set, `HEAD`, the commit count and the worktree list are identical before and after
  the refusal and after each `jigc doc show` — the mode-000 file standing.
- **Why it breaks no clause inside its scope.** The second clause's measure is *no command that
  works on rc.24 in a supported layout stops working, and every refusal's route works as printed*
  (DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 2). Its first half holds here as a
  fact. Its second half fails in exactly one kind of cell: a staged file whose bytes the calling
  user cannot read. That state was made by hand, with `chmod 000` on a file inside jigc's own
  workbench; no jigc verb was seen to make it, documented use does not, and it is none of the
  ordinary configurations and layouts the clause's instrument is ruled over (DECISIONS.md,
  2026-10-06, *Ahead of the run's opening*, ruling 4, part 2: nine, each a git configuration or a
  repository layout). In every cell where the staged files are readable — the no-plant control and
  the mode-644 control — each printed route ran as printed.
- **What this verdict rests on, said so that triage and the human can overturn it.** It rests on
  reading *in a supported layout* as the reach of the whole measure, the route half included. That
  reading is this verifier's; the entry does not spell the second clause's reach over hand-made
  states, the only sentence that names *deliberately planted states* is the first clause's, and
  this run's opening record declares no bound. **If the route half is read without a reach term,
  this finding is `confirmed`, `regression: false`** — the previous release's side of that fact is
  established below and would stand.

`contested: false` — the finding does not argue that a settled decision is wrong, and no settled
decision was found that intends this answer (below), so the basis is `breaks-no-clause` and not
`intended`.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (1.0.0-rc.24).
- The binary's directory went first on `PATH` in every driver call, and each call stops unless
  `command -v jigc` prints the binary it was handed: `<scratch>/bin/c1.a2/jigc` for the candidate,
  `<scratch>/bin/previous-91834b5e011d/jigc` for the previous release.
- No `cargo build`, nothing under `target/`. Every rig:
  `SCRATCH=<W>/rigs dev/jigc-rig --binary <binary> refs-post-hoc`, stdout captured alone, the
  construction log to its own file, exit status 0 each of eight times.
- The clone: nothing of this verifier's was edited, staged or committed. `HEAD` is 126a8531 on
  `fix/canary-one` — the round's record commit, on top of the candidate's commit — and
  `git diff --stat` is empty; the untracked files under the run's `r1/reports/test/` are other
  reporters' and none is this one's.

## What was reconstructed, and what was changed

The finding's block is one item of another report's *Left open*, so the smallest setup that reaches
the door was built again, from the source report's own description of it:

- On top of the rig state `refs-post-hoc`, with the binary under test, four commands, each exit
  status 0: `jigc milestone create "Cache rework"`, `jigc milestone add-task cache-rework "Area low"`,
  `jigc milestone add-task cache-rework "Area zed"`,
  `jigc doc create adr --title "Low policy" --task area-low`. Sub-task `area-low` then stages
  `adr:low-policy`; `area-zed` stages nothing.
- **The plant**, as the source describes it: a regular file `research:locked.md` in
  `.jigc/tasks/area-low/docs/`, then `chmod 000`. **One change:** this verifier's file is 28 bytes
  (`# locked`, a blank line, `planted by hand.`, a blank line) where the source's was 29; the bytes
  are never read by the door, and the refusal is the same 678 bytes the source reports.
- **Added by this verifier**, each in a rig of its own: a control with no plant; a control with the
  same file left at mode 644; and a variant with no added file, where the doc jigc staged itself,
  `adr:low-policy.md`, is set to mode 000.
- Nothing else was written into `.jigc/` by hand.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user (uid 501).**
Nothing was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/vp1a2-ms-route.gJBK53` (written `<W>`). **Eight rigs, four per binary**, each from the
rig tool's own `mktemp -d` under `<W>/rigs/`; none was built before them and none is left out.
Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's repository. Driver `<W>/tools/run.sh`:
snapshot, run the argv with stdin from `/dev/null` and stdout and stderr to their own files, read
the exit status directly (never through a pipe), snapshot again. The snapshot (`<W>/tools/snap.sh`)
is one line per entry under `.jigc/` — kind, mode, size and the first 16 hex digits of its sha256,
or `UNREADABLE` for a mode-000 file, or a link's target — then
`git status --porcelain --untracked-files=all --ignored`, `HEAD`, `git rev-list --count HEAD` and
`git worktree list --porcelain`. The rig and the milestone are built by `<W>/tools/mkrig.sh`.

Several measured invocations share a rig where the earlier ones changed nothing a later one reads:
the refusal and the reads leave the tree identical; `jigc milestone finalize` and the forced discard
were run last in their rigs, in that order.

## What was driven — the candidate

Per-invocation files under `<W>/runs/cand-<cell>/<n>-<name>/` (`argv`, `stdout`, `stderr`, `exit`,
`snap.before`, `snap.after`, `snap.diff`), and `build.log`, `rig`, `binary` per cell.

### The finding's cell — `cand-locked`: the hand-written file at mode 000

| n | argv | exit | stdout | stderr | tree |
|---|---|---|---|---|---|
| 1 | `jigc milestone discard cache-rework` | **1** | empty | `blocking · milestone.staged-prose` — *1 sub-task(s) stage 2 doc(s) that no commit has a copy of*: `area-low: adr:low-policy, research:locked`; the route (678 bytes) | identical, the file standing at mode 000 |
| 2 | `jigc doc show research:locked --task area-low` | **1** | empty | `blocking · store.not-staged` — *`research:locked` is not staged in this task and has no committed copy — nothing to read yet*; route *create or author the doc in this task first — a staged copy exists only after a write* (320 bytes) | identical |
| 3 | `jigc doc show adr:low-policy --task area-low` | 0 | the staged doc (135 bytes) | empty | identical |
| 4 | `jigc doc show research:locked --task area-low --format json` | 1 | empty | the same finding as data: `"code": "store.not-staged"`, `"target": "research:locked"`, the same message and route (594 bytes) | identical |
| 5 | `jigc doc show research:locked` | 1 | empty | `blocking · store.not-found` — *could not read `research:locked` at `docs/research/locked.md`: No such file or directory (os error 2)*; its route names `jigc doc show research:locked --task <task-id>` (463 bytes) | identical |
| 6 | `jigc milestone finalize cache-rework` | 1 | empty | `blocking · join.missing-provenance` — *staged doc `research:locked` in sub-task `area-low` of milestone `cache-rework` has no recorded provenance*; route *re-stage the doc so its provenance is recorded* (225 bytes) | one file added, `.jigc/index/edges.json` (73 bytes); nothing else |
| 7 | `jigc milestone discard cache-rework` | 1 | empty | byte-identical to row 1 (`cmp`) | identical |
| 8 | `jigc milestone discard cache-rework --force` | 0 | `discarded milestone:cache-rework (2 sub-task(s); workbench removed)` (141 bytes) | the staged-docs warning naming `area-low: adr:low-policy, research:locked` (248 bytes) | the milestone area and both sub-task areas gone; `HEAD` one commit ahead |

The refusal's route, whole, as printed in row 1:

    route: read what is in them with `jigc doc show <address> --task <sub-task-id>` (the sub-task ids
    are listed above), or land the milestone with `jigc milestone finalize cache-rework` (which
    refuses, with its own route, while the milestone has nothing to land or a required slot is
    empty) — or, once you have confirmed the milestone holds nothing you need,
    `jigc milestone discard cache-rework --force` settles the record and tears the workbench down
    with them

(one line on the terminal; wrapped here). Rows 2 and 3 are its first command, once for each doc the
refusal named; row 6 its second; row 8 its third. Rows 4, 5 and 7 are this verifier's: the JSON form
of row 2, the task-less read, and the refusal once more after row 6.

Row 5 closes a loop that row 2 opens: the task-less read routes back at the `--task` read, which is
row 2. It was run once and not pursued.

### The controls and the variant

| cell | the state | `jigc milestone discard cache-rework` | `jigc doc show <the doc in question> --task area-low` | `jigc doc show adr:low-policy --task area-low` |
|---|---|---|---|---|
| cand-none | no plant | exit 1, `milestone.staged-prose`, *stage 1 doc(s)*: `area-low: adr:low-policy` (661 bytes); tree identical | — | exit 0, the staged doc (135 bytes) |
| cand-readable | the same hand-written file, mode 644 | exit 1, `milestone.staged-prose`, *stage 2 doc(s)*: `area-low: adr:low-policy, research:locked` (678 bytes); tree identical | `research:locked`: exit 1, `blocking · store.unparseable` — *at `.jigc/tasks/area-low/docs/research:locked.md` does not parse: required section heading `## question` is missing*; route *fix the staged working copy so it conforms to its schema* (329 bytes) | exit 0 (135 bytes) |
| cand-genuine-locked | no added file; `adr:low-policy.md`, staged by jigc, set to mode 000 | exit 1, `milestone.staged-prose`, *stage 1 doc(s)*: `area-low: adr:low-policy` (661 bytes); tree identical | `adr:low-policy`: **exit 1, `blocking · store.not-staged`** — *`adr:low-policy` is not staged in this task and has no committed copy — nothing to read yet* (318 bytes) | that same invocation |

In `cand-genuine-locked` the route's second command was run as well:
`jigc milestone finalize cache-rework` exits 1 with `blocking · milestone.area-io` — *could not read
a staged doc body for milestone `cache-rework`: Permission denied (os error 13)*, route *resolve the
underlying I/O condition (a disk or permissions problem on the `.jigc/` milestone area), then re-run
the command* (292 bytes). That door says what the state is. It also leaves three entries behind
while refusing: `.jigc/index/edges.json`, and the empty directories
`.jigc/milestones/cache-rework/merged/` and `.jigc/milestones/cache-rework/merged/docs/`.

## Against the claim, and against the design that owns the behaviour

**The claim holds as a description.** Exit statuses, codes and messages are what the finding says,
on a fresh rig, with each status read bare and each output read whole from its own file. The three
ways a repro goes wrong were each looked for and none applies: no state was reused across cells; no
output was piped or cut; and `command -v jigc` named the handed binary before every invocation.

**The mechanism, read after the drive.** `crates/engine/src/store.rs`, `read_slice_staged`: the
staged read hands `read_parse_slice` the handler `|_| not_staged_block(...)` for a failed read of
the staged file, and that handler discards the error. So every error of `std::fs::read_to_string` on
that path — *permission denied* among them — is rendered as the absent-instance block. The probe
the refusal is built from, `staged_doc_ids` in `crates/cli/src/task.rs`, asks for the entry's shape
(`std::fs::metadata(..).is_file()`) and never for its bytes, so it lists the file as staged. Two
doors, two questions, one file: that is the contradiction.

**The design.**

- `design/write-commands.md` → *Abandoning a milestone*: on a sub-task whose working area stages a
  doc no commit has a copy of, the door *blocks with `milestone.staged-prose` … names each sub-task
  and its staged identities, and routes read → land → consent — `jigc doc show <address> --task
  <sub-task-id>`, `jigc milestone finalize <id>`, then the consent*. The door does this in every
  cell. What the design does not say is what the read answers over a staged file it cannot read.
- `design/doc-read-surface.md`: *An **absent** staged instance blocks `store.not-staged`, routed on
  the real state*. The instance here is present and unreadable, not absent, and the route printed —
  *a staged copy exists only after a write* — is not on the real state. The doc is silent on the
  unreadable case; nothing in it intends this answer.
- A sibling of this shape was treated as a defect and repaired before this run: the comment over
  `staged_doc_ids` records a directory named `<type>:<slug>.md` that a refusal named and routed at a
  read that dead-ended — *four surfaces claiming a staged doc that no doc read can open*. The
  repair asked a shape question; a file's readability was not part of it.

No dated decision was found that rules what the staged read says over bytes it cannot read. So the
answer is a defect of what a surface says — against the read surface's own *routed on the real
state* — and not behaviour a decision intends.

**The clause.** DECISIONS.md, 2026-10-04, *The exit rule, revised*: the second clause is *a working
product others can use and rely on*; its instrument, *no command that works on rc.24 in a supported
layout stops working, and every refusal's route works as printed*; and *a finding blocks the 1.0.0
call only if it breaks a clause inside its scope; everything else is recorded with its tier*. The
run's opening record names the clause `working-product`, by reference to that entry. Read against
the cells:

- *No command that works on rc.24 stops working*: holds — sixteen of sixteen invocations identical
  on the previous release (below).
- *Every refusal's route works as printed*, where the staged files are readable: holds — in
  `cand-none` and `cand-readable` the read route serves the doc or says truthfully why it cannot,
  and the two other routes answer as the refusal says they will.
- *Every refusal's route works as printed*, where a staged file is mode 000 and the caller is not
  root: does not hold for that file. The read exits 1 on a statement that contradicts the refusal,
  and its own route points at a write. The other docs of the same refusal read at exit 0; the land
  route refuses with a route of its own, as the refusal's text says it may; the consent runs.

**The strongest reading against this verdict.** The route half of the measure is a universal and
carries no words of its own about reach; the layout driven is the plainest one there is, a single
checkout with default git configuration; a bound is the human's to declare and this run declares
none; and a route that sends its reader from *these docs are staged* to *nothing to read yet —
create or author the doc first* is not followable, whatever made the state. On that reading the
finding is confirmed and the human's list is where it belongs. This verifier reads the clause the
other way because the entry gives *why* it was revised — a criterion with no reach term *could not
converge* — and because the instrument ruled for this clause is run over ordinary configurations
and layouts, of which a hand-set mode on a workbench file is none. Which reading holds is not this
verifier's to settle beyond the verdict it was sent to give; it is flagged in *Left open*, item 1.

## The same cells on the previous release

The verdict is not `confirmed`, so no `regression` field is returned. Because the clause's first
half is itself a comparison with the previous release, every cell was run on it — four rigs, sixteen
measured invocations, files under `<W>/runs/prev-<cell>/`.

- **Every exit status is the same**, in all sixteen.
- **stdout and stderr are byte-identical in all sixteen**, the rig's path normalised (sha256 of each
  file after the replacement, compared pairwise): the refusal, both answers of the read route, the
  JSON form, the task-less read, both `finalize` refusals, the forced discard's ack and warning.
- The tree effects are the same: identical where the candidate's is identical; the same added
  entries after each `jigc milestone finalize`; the same areas gone after the forced discard.

So if this finding were confirmed, the fact is **`regression: false`** — red on both binaries, the
door and its route existing on both. The previous release's binary was hashed and driven; the field
is left out of the return only because the verdict is `refuted`.

## Scope of what was verified

**`instance, unbounded`.** Driven: one door, `jigc milestone discard <id>`, in its bare form, and
the three commands its refusal prints; one plant (a hand-written file at mode 000), two controls
(no plant; the same file at mode 644) and one variant (a jigc-staged doc at mode 000); two binaries;
one platform, one non-root caller. The consumers of the mechanism were **not** enumerated. One count
was taken and is that and no more: `grep -n "not_staged_block" crates/engine/src/store.rs` gives one
call site (`read_slice_staged`), a comment and the definition — the number of callers of the block,
not of the doors that reach the staged read, and not of the refusals that route at it.

## Repro V-1

```yaml
claim: "with a mode-000 file named as a staged doc in a sub-task's staging area, `jigc milestone discard <id>` refuses with milestone.staged-prose naming that doc, and the read route it prints — `jigc doc show <address> --task <sub-task-id>` — exits 1 with store.not-staged for it; said to break working-product"
verdict: REFUTED   # basis breaks-no-clause: the behaviour reproduces as described; nothing that works on the previous release stops working, and the route fails only over a staged file whose bytes the caller cannot read, a state made by hand
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in every invocation; stdout and stderr byte-identical once the rig's path is normalised (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux, not driven as root"
setup:
  - fixture: refs-post-hoc
  - ["jigc", "milestone", "create", "Cache rework"]
  - ["jigc", "milestone", "add-task", "cache-rework", "Area low"]
  - ["jigc", "milestone", "add-task", "cache-rework", "Area zed"]
  - ["jigc", "doc", "create", "adr", "--title", "Low policy", "--task", "area-low"]
  - write: { path: ".jigc/tasks/area-low/docs/research:locked.md", content: "# locked\n\nplanted by hand.\n\n" }
  - ["chmod", "000", ".jigc/tasks/area-low/docs/research:locked.md"]
repro:                                      # in order, in one fixture: none of the three changes the tree
  - ["jigc", "milestone", "discard", "cache-rework"]
  - ["jigc", "doc", "show", "research:locked", "--task", "area-low"]
  - ["jigc", "doc", "show", "adr:low-policy", "--task", "area-low"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · milestone.staged-prose", "1 sub-task(s) stage 2 doc(s)", "  area-low: adr:low-policy, research:locked", "`jigc doc show <address> --task <sub-task-id>`", "`jigc milestone finalize cache-rework`", "`jigc milestone discard cache-rework --force`"]
    tree: "every entry under .jigc/ identical before and after (kind, mode, size, content); porcelain, HEAD and the commit count unchanged"
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · store.not-staged", "`research:locked` is not staged in this task and has no committed copy — nothing to read yet", "create or author the doc in this task first"]   # TODAY's answer: the file is there and cannot be read
    tree: "identical"
  - exit: 0
    stdout_contains: ["# Low policy"]
    stderr: ""
    tree: "identical"
variants:        # each in a fixture of its own
  - "the same file left at mode 644 -> discard: exit 1, the same two ids; `jigc doc show research:locked --task area-low`: exit 1, `store.unparseable` naming `.jigc/tasks/area-low/docs/research:locked.md` and `## question`"
  - "no added file, `chmod 000 .jigc/tasks/area-low/docs/adr:low-policy.md` -> discard: exit 1, `stage 1 doc(s)`, `area-low: adr:low-policy`; `jigc doc show adr:low-policy --task area-low`: exit 1, `store.not-staged`; `jigc milestone finalize cache-rework`: exit 1, `milestone.area-io`, `Permission denied (os error 13)`"
control: "no plant -> discard: exit 1, `milestone.staged-prose`, `stage 1 doc(s)`, `area-low: adr:low-policy`; `jigc doc show adr:low-policy --task area-low`: exit 0"
observed: "<W>/runs/cand-*/ (argv, stdout, stderr, exit, snap.before, snap.after, snap.diff, build.log); the previous release: <W>/runs/prev-*/"
pinned-by: "UNPINNED: found this round. From a name search over the suites, not from the diff: four suites name `milestone.staged-prose` (staged_prose_consent_axis.rs, repo_relative_paths.rs, milestone_teardown_loss.rs, destroying_door_sibling_surfaces.rs) and nine name `store.not-staged` (staged_read_miss_arm.rs, spec_read_back_arms.rs, flow49_acceptance.rs, doc_list.rs, flow52_acceptance.rs, migrate_locus_axis.rs, doc_show_staged.rs, work_unit_unknown_envelope.rs, no_such_task_route.rs); a search of those thirteen for a mode being set (`0o000`, `from_mode`, `set_permissions`, `Permission denied`) finds four lines — three make a hook executable, and the one `0o000` (repo_relative_paths.rs) is set on a base pin, not on a staged doc. No suite was run, and no test's assertions were read beyond those lines."
```

**Pinnable as it stands: expectations 1 and 3 yes; expectation 2 only as a statement of today's
behaviour.** The fixture is a named state of the shared builder plus four argv steps, one file
write and one mode change, so the block converts by hand. Conditions: a Unix target, and **a caller
that is not root** — for root a mode-000 file reads, the plant does not bite, and the block's
second expectation is expected to turn into the mode-644 variant's answer (not driven). Expectation
1 with its `tree:` line, and expectation 3, are facts worth keeping: the door refuses, names what
it would take and changes nothing, and a readable staged doc is served. Expectation 2 pins a
misstatement that a repair of this row would change on purpose; whoever converts it should write it
as the red test of that repair — the read saying that the staged file is there and cannot be read —
and not as a fact to keep. What survives such a repair unchanged is its exit status being non-zero
and its `tree:` line. The comparison with the previous release is not a suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **The reach of the second clause's route half over a state made by hand is written nowhere this
   verifier found, and this run declares no bound.** The verdict above reads *in a supported layout*
   as covering it. If that reading is not the human's, this finding is `confirmed` with
   `regression: false`, on the evidence already in this report.
2. **The defect this verdict leaves on the ledger:** the staged read answers a present, unreadable
   staged file with `store.not-staged` — *nothing to read yet* — and routes at a write. Mechanism:
   `read_slice_staged` in `crates/engine/src/store.rs` maps every failed read to the
   absent-instance block. Identical on the previous release.
3. **The `store.not-staged` block's own route was not run in this state** — *create or author the
   doc in this task first*. What a write does to the mode-000 file standing at that path — refuses,
   or replaces it at exit 0 — was not driven, and would be the first clause's question, not this
   one's.
4. **Other failed reads through the same handler were not driven**: a staged file that is not
   UTF-8 (`read_to_string` fails on it as well), and a staged file owned by another user under a
   restrictive umask.
5. **Reach was not driven.** Whether any jigc verb, or ordinary use — a run under another uid, for
   one — can leave a staged file its next reader cannot read was not tested. The plant here was made
   by hand.
6. **The task-less read and the `--task` read route at each other in this state**: `store.not-found`
   names `jigc doc show research:locked --task <task-id>`, which is the read that answered
   `store.not-staged`. Run once, not pursued.
7. **`jigc milestone finalize <id>` under the hand-written plant** exits 1 with
   `join.missing-provenance`, whose route — *re-stage the doc so its provenance is recorded* — names
   no command. The source report's item 5, seen again here; identical on the previous release.
8. **`jigc milestone finalize <id>` leaves entries behind while refusing** — under the jigc-staged
   doc at mode 000 it exits 1 with `milestone.area-io` and has created `.jigc/index/edges.json` and
   the empty directories `.jigc/milestones/<id>/merged/` and `merged/docs/`. Nothing is removed.
   Identical on the previous release.
9. **Linux, and a root caller, were not driven.**

<!-- end of report -->
