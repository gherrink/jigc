# verify-real — `r1-p3-milestone-merged-area-writer-follows-a-link` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-milestone-merged-area-writer-follows-a-link`. One finding, handed over:
ledger key `r1-p3-milestone-merged-area-writer-follows-a-link`, door *unlisted — the milestone merged
area's writer, `crates/engine/src/milestone.rs:2323` (the reporter names no verb)*, the clause it is
said to break `no-lost-files`, triage's grade *unclear*. It has no block of its own: its source is
item 4 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-staging-area-writers-not-enumerated.a1.md`,
which says of itself *read, not driven with a plant*. That report was read because the prompt hands
it over as the finding; no other report was read, and nothing of triage's reasoning beyond the grade
and the re-drive it asks for.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** The defect is real. It is **not**
`does-not-reproduce`, and it stays a row of the ledger.

What triage asked, answered in its order:

1. **Can a jigc verb, or an ordinary act, leave a link under `.jigc/milestones/<id>/merged/docs/`
   before the write? None was found.** No production code in either crate creates a link; the one
   writer into that directory creates regular files; a link planted in a *sub-task's* area is not
   carried into the merged area (driven, S7); `.jigc/milestones/` is ignored by the working area's
   own `.gitignore`, so no git act delivers an entry there. The door's own inventories — after a
   join, a blocked finalize, a join over the standing area, a second blocked finalize — hold regular
   files and nothing else, on both binaries. The state was reached one way only: this verifier made
   the link by hand.
2. **A correction to the premise: `jigc milestone join` does not write there at all.** The writer's
   function, `engine::milestone::materialize`, has one production caller, `run_milestone_finalize`
   (`crates/cli/src/milestone.rs:6769`). Driven: after `jigc milestone join` the milestone's area has
   no `merged/` (S0), and a join over a planted link leaves the link and its target untouched at
   exit 0 (S1, S6). **The verb that reaches the writer is `jigc milestone finalize`**, and only it.
3. **With a link planted at the name of a body the join produces, the write lands at the link's
   target, and the boundary exits 0.** `jigc milestone finalize` — on the candidate and on
   `1.0.0-rc.24`, identically:
   - `clear_staged_bodies` runs first, as the source report says, and **skips the link**: it removes
     regular files only, by design (`crates/engine/src/milestone.rs:2386`). The `std::fs::write` at
     `:2323` then follows it.
   - **The target's bytes are replaced.** An untracked `notes.md` of 43 bytes that no git object
     holds is 225 bytes afterwards — the merged ADR body — and the line it held is gone (S1, S6).
   - **A dangling link makes a file where it points**, outside the repository included (S3).
   - **The doc is left out of the commit.** Every reader of `merged/docs/` skips an entry that is not
     a regular file, so `adr:alpha-choice` is not gated and not promoted: the commit holds 2 files
     where the control's holds 3, `docs/decisions/alpha-choice.md` does not exist, and no git object
     holds the body. The summary still says `alpha-part: 1 doc`, the record flips `alpha-part` to
     `joined`, and the sub-task's area — the one place jigc held that prose — is removed.
   - What the boundary does say: one note on stderr that *the working area held 1 entry jigc did not
     write*, and the link itself moved to `.jigc/displaced/link-probe/merged/docs/`.
   - **A blocked boundary writes through the link too**: with the slots left empty the verb exits 3
     and `notes.md` is replaced all the same (S2).
4. **Why it breaks no clause inside its scope.** The first clause's scope, in the closing
   condition's words (DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 1): *in a
   healthy repository used as documented … no jigc command at exit 0 destroys bytes no git object
   holds … Races against a non-jigc writer inside a millisecond window, and deliberately planted
   states, are declared bounds, written down with their reach.* The loss is at exit 0 and it is of
   bytes no git object holds — but the state it needs is a symbolic link, inside a directory git
   ignores, at the exact name of a staged body the join is about to produce. Nothing found makes
   that but a deliberate plant. That is the scope sentence's own excluded class.

**What this verdict does not settle, said plainly.** The scope sentence calls planted states
*declared bounds, written down with their reach*. **No written bound names this state**: the run's
opening declares none, and the one list with reach is still owed (`implementation/decisions-pending.md`
→ *The exit rule* → *The declared bounds, as one list with reach*). Declaring a bound is the human's
ruling; this report declares none and reads the clause's own text. It is item 1 of *Left open*.

`contested: false` — the finding argues against no settled decision. Two settled texts bear on it
and neither intends the behaviour: see *Against the design that owns the behaviour*.

No `regression` field is returned (the verdict is not `confirmed`). The fact was established anyway,
because triage asked for both binaries: the behaviour is the same on the previous release, cell for
cell.

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
  fresh`, stdout captured alone, the construction log to its own file, exit 0 each time — one rig
  per scenario, 8 per binary, plus one exploration rig on the candidate.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and
  after, `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user.** Nothing
was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-merged-link.bnVo0i` (written `<W>`). Under it: `<W>/explore/` (one rig, candidate
only, used to learn the verbs' spellings — nothing here is read from it), `<W>/tools/drive.sh`, and
the pair every number here is read from — `<W>/c.log` with `<W>/c/` (candidate) and `<W>/p.log` with
`<W>/p/` (previous release). Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. One driver, `<W>/tools/drive.sh <binary> <out dir> <clone>`: each cell runs
with stdin from `/dev/null` or a file, stdout and stderr to their own files, the exit status read
directly and never through a pipe. An *inventory* is `find <area> -mindepth 1` handed to
`stat -f '%HT|%Sp|%z|%N|->%Y'`, which reads an entry's shape **without following a link**. A file is
shown by its shape, size, link count, sha256 and bytes.

**One thing about the tooling, stated because it is a deviation of a kind.** The driver is a scratch
file written from the shell with a here-document, not with the file tool; it is in no repository.

## Is the state reachable — the enumeration

All lines are at eeffe347.

- **The writers into `merged/`: one site.** `command grep -rn 'MERGED_AREA\|"merged"'` over the two
  `src/` trees: 6 lines outside a test module, in 2 files — the constant
  (`engine/src/milestone.rs:2242`), the clear's and the writer's path (`:2302`, `:2322`), and the
  registry, its tree-member rule and the complement walk (`engine/src/state.rs:250`, `:290`,
  `:494`). The one write is `std::fs::write(&path, &body)` at `milestone.rs:2323`. Every other hit
  (19 lines) sits after its file's first `#[cfg(test)]`.
- **Its callers: one.** `command grep -n 'materialize('`, definitions and comments aside: 5 lines —
  `crates/cli/src/milestone.rs:6769` inside `run_milestone_finalize`, and 4 in the engine's test
  module (after `milestone.rs:3017`).
- **No production code creates a link.** `command grep -rnE '(symlink|soft_link|hard_link|symlink_file|symlink_dir)\('`
  over the two `src/` trees, comments and the `is_symlink(` / `symlink_metadata(` probes aside: 23
  lines in 9 files, **every one after its file's first `#[cfg(test)]`** (compared by line number,
  file by file).
- **No rename or copy has a destination under `merged/`**: taken from the source report's sweep of
  `fs::rename(` / `fs::copy(` (its section *The enumeration*, (d)) and from the `MERGED_AREA` lines
  above, which name every path built under it. Not re-swept here.
- **git**: the working area's own `.jigc/.gitignore` lists `milestones/` (read in a rig: `tasks/`,
  `index/`, `state/`, `milestones/`, `worktrees/`, `logs/`, `displaced/`), so no checkout, stash or
  clone puts an entry there.

**The bound of this enumeration.** It is textual and by reading; *after the first `#[cfg(test)]`* is
taken for *in a test module*, which holds only where the test module runs to the end of its file —
not checked brace by brace. The spawned `git` processes were not each read. What stands beside it is
the inventories: regular files only, after every jigc door driven.

**Ordinary acts considered, and what each gives.** A hard link at the body's name (a deduplicating
copy would leave one): the clear unlinks it and a fresh file is written — the other name keeps its
bytes (driven, S4). A link under a name the join does not produce: never written, moved aside with
a note (driven, S5). A link at the staged body in the sub-task's area: read through, a regular file
written (driven, S7). A copy or move of the repository carries a link and makes none. A `pre-commit`
hook is named by the engine's own comment as a writer into this area (`engine/src/state.rs`, the
`MILESTONE_AREA_FILES` row): a hook that makes a link at a staged body's name would plant this
state — that is a plant by another hand, and was not driven.

## What was driven — the candidate

Log `<W>/c.log`, per-cell files under `<W>/c/runs/`, one `fresh` rig per scenario. 95 cells: 86 exit
0, 9 exit 3 — every exit 3 is a `jigc milestone finalize` over ADRs whose required slots are empty,
blocked on `schema-conformance.required-slot-present`.

Every scenario opens with the same fan-out: `jigc milestone create "Link probe"` ·
`jigc milestone add-task link-probe "alpha part"` · `… "beta part"` ·
`jigc doc create adr --title "Alpha choice" --task alpha-part` ·
`jigc doc create adr --title "Beta choice" --task beta-part`. *Fill* is the six
`jigc doc set-slot adr:<x>-choice#<context|decision|consequences> --task <x>-part --from-file -`
calls the blocked finalize's own routes print. `notes.md` is an untracked file in the repository's
root, 43 bytes, sha256 `c5c99d10…7ba1b4`: `user notes - no git object holds this line`.

| scenario | the plant | what was run | exit | where the write landed |
|---|---|---|---|---|
| **S0** control | none | join · finalize · join · finalize · fill · finalize | 0 · 3 · 0 · 3 · 0 | after the first join: **no `merged/`**. After each blocked finalize: `merged/docs/` with two regular files, mode `-rw-r--r--`. The landed finalize: `promoted` both ADRs, `3 files committed`, the area gone, nothing displaced |
| **S1** | after a blocked finalize, `merged/docs/adr:alpha-choice.md` replaced by a symbolic link to `<repo>/notes.md` | fill · join · finalize | 0 · **0** | join: `notes.md` unchanged, the link still a link. Finalize: **`notes.md` is 225 bytes, sha256 `d2639ff7…0b5714` — the ADR body; its own line is gone.** `promoted docs/decisions/beta-choice.md` only, `2 files committed`, `sub-tasks: alpha-part: 1 doc …`; `docs/decisions/alpha-choice.md` absent; the link moved to `.jigc/displaced/link-probe/merged/docs/adr:alpha-choice.md`, named in a stderr note |
| **S2** | the same link; the slots left empty | finalize | **3** | **`notes.md` is 136 bytes, sha256 `4d1d693c…af49af` — the empty ADR skeleton.** The link stands; no commit |
| **S3** | the same name, a **dangling** link to `<rig>/outside/made-elsewhere.md` — outside the repository | fill · finalize | **0** | **that file now exists**, 225 bytes, sha256 `d2639ff7…0b5714`. The commit and the note as in S1 |
| **S4** | the same name, a **hard** link to `notes.md` (link count 2) | fill · finalize | 0 | `notes.md` unchanged, link count back to 1; both ADRs promoted, `3 files committed`, nothing displaced |
| **S5** | a symbolic link to `notes.md` under `adr:unrelated.md` — a name the join does not produce | fill · finalize | 0 | `notes.md` unchanged; both ADRs promoted; the link moved to `.jigc/displaced/…`, named in the note |
| **S6** | before any finalize: `merged/docs/` made by hand, the S1 link in it | join · fill · finalize | 0 · **0** | as S1: the join leaves it all untouched, the finalize replaces `notes.md` and leaves `adr:alpha-choice` out of the commit |
| **S7** | the **sub-task's** staged `adr:alpha-choice.md` replaced by a symbolic link to a copy of itself outside the repository | finalize | 3 | `merged/docs/adr:alpha-choice.md` is a **regular file**, 136 bytes; the copy outside is unchanged |

**What is left of the doc after S1** (read in the rig after the run): `.jigc/tasks/` is empty — both
sub-task areas are removed; the committed record reads `status: joined` for the milestone and for
`alpha-part`; `git grep -l "Alpha choice" HEAD` exits 1; under the repository the string is in
`notes.md` alone. The ADR's prose survives only as the bytes written through the link.

**The stderr note of S1, whole** — it names the link and says nothing of the write:

```
note: the working area held 1 entry jigc did not write, and removing it would have destroyed bytes no commit has a copy of — they were moved aside, not taken:
    .jigc/milestones/link-probe/merged/docs/adr:alpha-choice.md → .jigc/displaced/link-probe/merged/docs/adr:alpha-choice.md
```

## The same block on the previous release

Not the regression step — the verdict is not `confirmed` — but what triage asked for. Rigs built
with `--binary <scratch>/bin/previous-91834b5e011d/jigc`; the same driver; log `<W>/p.log`.

- 95 cells, 86 exit 0 and 9 exit 3, the same cell for cell.
- `diff` of the two whole logs — 1037 lines each — with the rig's name and the abbreviated commit
  ids normalised: **no difference** (exit 0). Every inventory, every file's size and sha256, every
  stdout and stderr line is the same, the S1 note included.

So on `1.0.0-rc.24` the planted link is written through in the same three places (S1, S2, S3, S6),
and the safe cells are safe in the same way (S0, S4, S5, S7).

## Against the design that owns the behaviour

- **The writer's own contract.** `engine::milestone::clear_staged_bodies`' doc comment
  (`crates/engine/src/milestone.rs:2351`): *jigc writes regular files, so a directory or a symlink
  wearing a staged identity's name is something else, and this removes bytes* — the reason the clear
  leaves a link alone. The same module's writer, 60 lines up, does not ask the shape and writes
  through it. The reader at the boundary gate says the same of the rule
  (`crates/cli/src/milestone.rs:8506`): *jigc writes regular files and never a directory or a link
  wearing a body's name*. The remover and the readers hold the shape rule; the writer does not.
- **`completions/artifacts/M53/settle-record.md` → D1.3**: *`materialize` clears what it wrote* —
  the selective clear that makes this cell reachable. Before it, an unconditional
  `remove_dir_all(merged/docs)` took any plant ahead of the write. The same record names the shape
  axis as the one that was missed once already (the suite `merged_area_selective_clear`, its
  `SHAPE_PLANTS`).
- **DECISIONS.md, 2026-10-04, the fix-pass entry for `104a7d4b`**: *a file jigc replaces is written
  as a regular file, never through a link* — one writer, `regular_file::replace`, shared with the
  promote sink. The merged area's writer is not routed through it. That entry's cells are links git
  can deliver (a committed link at an install member); this one is a link in an ignored directory.

None of the three intends a write through a link here, so the basis is not `intended`. None of them
puts a planted link in an ignored staging directory inside the first clause's scope either.

## Scope of what was verified

**Driven: the instance** — one writer (`engine/src/milestone.rs:2323`), reached through its one verb
(`jigc milestone finalize`), with a link at the name of a created ADR's body, on two binaries, on one
platform, in a docs-only fan-out of two sub-tasks.

**Enumerated: the writers into `merged/` and the writer's callers** — 1 and 1, by the method and
with the bound stated in *Is the state reachable*.

**Not enumerated: the class.** Other writers in jigc that open their destination with a following
write were not walked; for them this report is `instance, unbounded`.

## Repro M-1

```yaml
claim: "a symbolic link at `.jigc/milestones/<id>/merged/docs/<type>:<slug>.md`, under the name of a body the join produces, is written through by `jigc milestone finalize` at exit 0 — the target's bytes replaced, the doc left out of the commit"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; the behaviour below is what was OBSERVED, and it is a defect
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "identical in all 95 cells; the two logs differ in nothing once the rig's name and the commit ids are normalised (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: fresh
  - ["jigc", "milestone", "create", "Link probe"]
  - ["jigc", "milestone", "add-task", "link-probe", "alpha part"]
  - ["jigc", "milestone", "add-task", "link-probe", "beta part"]
  - ["jigc", "doc", "create", "adr", "--title", "Alpha choice", "--task", "alpha-part"]
  - ["jigc", "doc", "create", "adr", "--title", "Beta choice", "--task", "beta-part"]
  - ["jigc", "milestone", "finalize", "link-probe"]   # exit 3 on the empty required slots; leaves merged/docs/ with two regular bodies
  - write: "notes.md in the repository's root, untracked: `user notes - no git object holds this line\n` (43 bytes)"
  - plant: "replace .jigc/milestones/link-probe/merged/docs/adr:alpha-choice.md by a symbolic link to <repo>/notes.md"
  - fill: "for x in alpha, beta and s in context, decision, consequences: [\"jigc\", \"doc\", \"set-slot\", \"adr:<x>-choice#<s>\", \"--task\", \"<x>-part\", \"--from-file\", \"-\"] with one line of prose on stdin — exit 0 each"
repro:
  - ["jigc", "milestone", "join", "link-probe"]
  - ["jigc", "milestone", "finalize", "link-probe"]
expect:
  - exit: 0
    stdout_contains: "joined milestone:link-probe — 2 doc(s) merged"
    tree: "notes.md unchanged (43 bytes); the planted entry is still a symbolic link; nothing else under merged/docs/ changed"
  - exit: 0
    stdout_contains: ["promoted docs/decisions/beta-choice.md", "2 files committed", "alpha-part: 1 doc"]
    stdout_lacks: "promoted docs/decisions/alpha-choice.md"
    stderr_contains: "the working area held 1 entry jigc did not write"
    tree:
      - "notes.md is a regular file of 225 bytes holding the ADR `# Alpha choice` with its three filled slots; the line it held is gone"
      - "docs/decisions/alpha-choice.md does not exist; HEAD's commit changes 2 files (docs/decisions/beta-choice.md, docs/milestone-records/link-probe.md)"
      - ".jigc/milestones/link-probe/ is gone; .jigc/tasks/ is empty; the record reads `status: joined` for alpha-part"
      - ".jigc/displaced/link-probe/merged/docs/adr:alpha-choice.md is the symbolic link, still pointing at notes.md"
variants:
  - "S2 — the fill left out: the finalize exits 3 and notes.md is replaced all the same, by the 136-byte empty skeleton; no commit"
  - "S3 — the link dangling, to a path outside the repository: exit 0, and a 225-byte file is made there"
  - "S6 — merged/docs/ made by hand before any finalize, the link in it: as the spine"
  - "S4 — a hard link to notes.md at the same name: exit 0, notes.md unchanged, both ADRs promoted, 3 files committed"
  - "S5 — the symbolic link under `adr:unrelated.md`: exit 0, notes.md unchanged, both ADRs promoted, the link moved to .jigc/displaced/"
  - "S7 — the link at the sub-task's staged body instead: the finalize (exit 3) writes a regular file into merged/docs/"
control: "S0 — no plant: join leaves no merged/ at all; a blocked finalize leaves two regular files there; the landed finalize promotes both ADRs, 3 files committed, nothing displaced"
observed: "<W>/c.log with <W>/c/runs/ (S0-S7); the previous release: <W>/p.log with <W>/p/runs/"
pinned-by: "UNPINNED: found this round. The near-miss, read and not run: crates/cli/tests/merged_area_selective_clear.rs plants a symbolic link under merged/docs/ as `adr:a-link.md` (its SHAPE_PLANTS), a name its fixture's join does not produce (it stages adr:broken-policy, adr:code-policy, adr:doc-policy) — so the link there is never the writer's destination; crates/cli/tests/milestone_merged_complement.rs plants the same name. No suite was run by this verifier"
```

**Pinnable as it stands: in part, and not the spine.** The fixture is a named state of the shared
builder and every step but the plant is an argv; the plant is one `std::os::unix::fs::symlink` call,
so the block needs a Unix target. The control and the variants S4, S5 and S7 are true statements
about behaviour worth keeping and convert by hand to a test as they are, and so does the first
`expect` (the join writes nothing). **The spine and S2, S3, S6 describe a defect**: converted as
written they would pin the write through the link. They are the red test of a fix, should the row be
admitted — the plant under a name the fixture's join *does* produce is the one cell the existing
shape axis lacks — or the reach of a bound, should one be declared. The comparison with the previous
release is not a suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **No written bound covers the state this verdict rests on.** The first clause's scope excludes
   *deliberately planted states* as declared bounds *written down with their reach*; the run's
   opening declares no bound, and the list with reach is still owed. A grade of *out of scope* has
   no bound of the run's list to cite for this row. Who declares one, and whether a link planted
   inside jigc's own ignored staging directory is the kind the sentence means, is not a verifier's.
2. **Under the same plant, an authored doc is left out of the milestone's commit at exit 0 while
   the summary counts it.** Separate from where the bytes go: `adr:alpha-choice` is neither gated
   nor promoted, the summary says `alpha-part: 1 doc`, the record says `joined`, and the sub-task's
   area is removed. The one statement the boundary makes is the displacement note, which names the
   link and not the doc. Driven in S1, S3 and S6, the same on both binaries.
3. **A boundary that refuses has already written.** In S2 the finalize exits 3 and the link's target
   is replaced; the write sits ahead of the gate that blocks. The clause's sentence is about exit 0;
   this cell is recorded as hit.
4. **The shape rule is held by the remover and the readers of `merged/docs/` and not by its
   writer** (*Against the design that owns the behaviour*). Whether the writer should go through the
   no-follow writer the promote sink shares is a fixer's and a design question; nothing was changed.
5. **Not driven:** Linux; a root caller; a link whose target is given relative; a link at `merged`
   or at `merged/docs` themselves (a directory on the way, not the body's name); a body whose final
   address the join suffixed (`-2`); a body copied in from the committed store (`edited-from-base`);
   a fan-out with provisioned worktrees and staged code; a `pre-commit` hook as the planting hand.
6. **The class is not bounded**: other following writes in jigc were not walked (`instance,
   unbounded`), and the spawned `git` processes were not each read as writers into the area.

<!-- end of report -->
