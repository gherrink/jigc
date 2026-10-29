# Brief — the byte table, task B7: the `milestone` family (2026-10-18)

Two builders, one after the other, each followed by a review: **part A** (rulings and frame) and **part B** (the
family). You are told which part is yours. Do only yours.

## Rules (both parts)

Worktree `~/projects/gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`. Start from its HEAD
(`a81af2bb` for part A — `git log -3`; a dirty tree stops you). Every commit behind the full `dev/gate` run there (in
the background, read unpiped, never `| tail`, never an exit code through a pipe), each by `git commit -F <file>` with
the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Messages in the form of the branch's byte-table commits, ending "(the byte table, task B7, the first/second/… of its
commits)". At the end `git push origin fix/rc24-route-catalog` by name (never main, never force, never a tag). Never
`git stash`, never `rm -rf` on a variable path (`mktemp -d`, `find <dir> -mindepth 1 -delete`). Public repo: no host
path, person's name, session or agent id in a file. Do not edit `.claude/agents/`, `.claude/settings.json` or
`CLAUDE.md`. Kill processes only by pid. Red first wherever a test can show it. The previous release is the rc.24
rebuild from `91834b5e` (sha256 `accf3996…ab5d`), never the `jigc` in `~/.local/bin`. A ruling's product or doc
change is a finding of the run's ledger, not a change in this lane (the form `b7ef80e6` used).

## Read first, yourself

- `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md` — §8 names B7: `milestone`, the eight
  leaves in three parts; "a state that holds worktrees is never copied — a cell provisions after the copy".
- Beside it: `byte-table-acceptance-rows.md` (§6 the questions; the `M` lines incl. **M4**, the `squash: false`
  spelling; the doc-only arm's sentence near its line 368), `byte-table-readings.md`, `byte-table-findings.md` (§14:
  leads for B7 — the git-claims refusal in `task.rs`, the unanswered-pin finding, four milestone sentences),
  `byte-table-doc-family.md` (§4: the `W1` limit), `byte-table-install-family.md`.
- `DECISIONS.md` from the top down to the B5 rulings (newest first): every entry of task B6 and its two rounds of
  review fixes — what the frame's words mean now (`Plant::InPlaceOf`, `Plant::Deleted`, `May::CopiedFrom`,
  `May::ReSlugged`, the base-pin rule removed).
- The two reviews of B6 and what they caught — the kind of mistake to avoid:
  `~/.claude/projects/<project>/stabilization-briefs/b6a-review.md`,
  `b6b-review.md`.
- `crates/cli/tests/support/byte_cell.rs`, `crates/cli/tests/byte_table.rs`, `crates/cli/tests/byte_table/`
  (`owed.rs`: B7's leaves — milestone create, add-task, add-from-spec, provision, execute, join, discard; and
  `milestone finalize`, whose cells are owed).
- `implementation/dev-workflow.md` → Gate; `implementation/public-hygiene.md`; `implementation/pinning.md`.

## Part A — the rulings and the frame

**Commit 1 — the human's three rulings of 2026-10-18**, in the form `b7ef80e6` used (`DECISIONS.md` at the top; the
records that state each question as open updated; cells whose classification a ruling moves re-read and named):

1. **Entry 14 → A.** A `doc` write refuses **before** its copy-in when the bytes at the doc's home do not parse as
   its doctype: it stages and binds nothing, and names what stands there (a squatter to adopt, or a file to
   restore). A file that parses but does not yet conform is still copied in, as now — the human's item-23 ruling
   stands untouched (an uncommitted edit, a hand-started draft and the live-link edit path all parse).
   `doc-read-surface.md`'s "never a foreign squatter" becomes true. Why: the line falls at bytes that can be no doc
   of the type at all.
2. **Entry 15 → A.** Every `doc` write that refuses leaves the task's area as it found it: the copy, its
   provenance, its witness and any role it bound are rolled back — one rule with `author`'s
   (`write-commands.md:43,45`). Why: the rule the docs state for every refusal they name; it makes "nothing was
   persisted" true (closes findings §12 and the fix pass's row `(R3, F3)`) and removes §13's dead end.
3. **The two spellings → A.** B7 builds a switch by which a cell names a spelling of its leaf, in its own commit
   before the milestone rows, with cells for **both**: the doc-only arm of `task finalize` (a task whose workflow
   includes `step:finalize-doc-only` commits its own paths and never the index — the user's staged bytes must stay as
   they were) in the task family, and `finalize.fan-out.squash: false` (M4: one commit per code-carrying sub-task in
   task-id order, then the aggregate) for `milestone finalize`. Why: the doc-only arm is where a user's staged change
   meets a commit that skips the index, and the switch costs the same for one spelling or two.

**Commit 2 — a line may cover only its own unit's area.** `W1` and `W4` cover `.jigc/tasks/*` — any task's area —
while `write-commands.md:240` says a write cannot stage into a sibling's area; so a sibling's staged doc *rewritten*
by a write is excused unseen (doc-family record §4). The milestone family is where sibling areas are everywhere.
Give the frame the narrowest grounded way to say "the acting unit's own area" (and, for a milestone, its sub-tasks'
areas where a doc says so), red first with a fence case (a sibling's staged doc rewritten → not excused), re-read every
line that covers `.jigc/tasks/*`, and say whether any classification moved.

**Commit 3 — the spelling switch and its cells** (ruling 3). The doc-only arm's commit line replaces T3/T4 for its
spelling (`byte-table-acceptance-rows.md` near 368); M4 is `squash: false`'s line. Cells for the needs those lines
make, plants red first.

Stop after commit 3, push, and return (below). Do not start the milestone rows.

## Part B — the family, on the plan's pattern

Rows from the docs in a commit of their own before any cell of them runs, then the cells in the three parts §8 names
(choose the split by the leaves' shared docs and say it), then the numbers record in the form of
`byte-table-doc-family.md` (+ its `.tsv`) — **including a committed whole-table print on the previous release** on the
named-binary arm, as B6 did, so no previous-release claim rests on an uncommitted run. A state that holds worktrees is
never copied: each cell provisions after the copy. Use both plants (`InPlaceOf`, `Deleted`) and part A's own-area word
where the rows name a need they answer. Drive findings §14's B7 leads and §15's part-B leads (a change staged before `milestone create` meeting the
carryover gate; a plain-spelling `M10` cell that builds the edge index first; `M3`/`M4` and a pending config delta —
the aggregate commits `.jigc/config/manifest.yaml` when a knob is set and not committed): each becomes a key, a cell,
an answered question with its sentence, or stays a lead with why. Part A and its five review fixes
(`7f71f44b`..`fff27d7b`) changed the frame: read every one of their `DECISIONS.md` entries before the rows — the
places `<unit>`, `<sub-task>`, `<its-milestone>`, `<docs-root>`; `Holding::TheWorktreesOneByOne`; `May::Invalidated`;
needs keyed by spelling; and `cargo fmt` now reaches the family files (a new family file is declared beside the
`families!` call, not inside it — `tooling-tests/fmt_reach.rs` fences it). `owed.rs` loses B7's leaves. Cells per commit by the gate you measure, said in
the commit, as B5 and B6 did.

**Where the docs are silent or say two things:** a line only over a quoted sentence; a question becomes an entry of
`byte-table-acceptance-rows.md` §6 with its *Until ruled* reading — and before you call anything a silence, search
every design doc for it and quote what you find (B6's entries 13 and 14 were each answered or contested by sentences
the builder had not read). Do not rule; do not stop for it unless a cell cannot be built at all without the answer.

## Return

Compact: each commit's sha and its gate's `passed=` line; the pushed remote head; what moved; for part B the family's
numbers in five lines; then one block per new question (the sentences on each side quoted, a robust answer and why,
the reading used until ruled). Anything left open, with why. Full detail goes into the records, not the return.
