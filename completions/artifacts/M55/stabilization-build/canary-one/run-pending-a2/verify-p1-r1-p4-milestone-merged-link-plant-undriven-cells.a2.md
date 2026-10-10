# verify-real — `r1-p4-milestone-merged-link-plant-undriven-cells` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-milestone-merged-link-plant-undriven-cells`. One finding, handed over:
ledger key `r1-p4-milestone-merged-link-plant-undriven-cells`, door `jigc milestone finalize`, the
clause it is said to break `no-lost-files`, triage's grade *unclear*. It has no block of its own. Its
source is item 5 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-milestone-merged-area-writer-follows-a-link.a1.md`,
a list of eight cells that report says it did not drive: Linux; a root caller; a link whose target is
given relative; a link at `merged` or at `merged/docs` themselves; a body whose final address the
join suffixed (`-2`); a body copied in from the committed store (`edited-from-base`); a fan-out with
provisioned worktrees and staged code; a `pre-commit` hook as the planting hand. That report was read
because the prompt hands it over as the finding; no other report was read, and nothing of triage's
reasoning beyond the grade.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** Not `does-not-reproduce`: six of the eight
cells were driven, the write through the link happens in every one of them that reaches the writer,
and it stays a row of the ledger. Two cells were **not driven** and are not covered by this verdict
(Linux, a root caller — see *What was not driven*).

The question an *unclear* grade sends a verifier to answer is whether one of the undriven cells puts
the loss inside the first clause's scope — that is, whether one of them reaches the state without a
deliberate plant. **None does.** In its order:

1. **Every driven cell that loses bytes needs a symbolic link that a hand other than jigc put
   inside `.jigc/milestones/<id>/merged`** — a directory git ignores. In the three cells that vary
   what the *join* produces (a suffixed body, a copied-in body, provisioned worktrees with staged
   code) jigc makes no link of its own: `find <repo>/.jigc -type l` counts **0** after the fan-out
   and a blocked finalize in each, on both binaries, provisioned worktrees included.
2. **The cells vary where the bytes go and what is left out of the commit; none varies who made
   the link.** A relative target is followed like an absolute one (U1). A suffixed body is written
   through at its suffixed name (U3). A copied-in body is written through, and the edit to the
   committed doc is left out of the commit (U4). Staged code lands; the doc under the link does not
   (U5).
3. **A link at `merged/docs` or at `merged` reaches further than the handed finding's spine**
   (U2a, U2b): the clear runs *through* the directory link, so in the linked directory an untracked
   regular file under any `<type>:<slug>.md` name is **deleted** at exit 0, one under a produced
   name is replaced, and jigc's bodies are left behind there. The docs themselves are promoted. This
   is new against the source report, and it is still a planted link. A **dangling** link at either
   name refuses at exit 1 and writes nothing (U2c, U2d).
4. **A `pre-commit` hook reaches the state only when it is written to.** jigc runs the hooks in a
   dedicated worktree, so a hook that names the merged area relative to its own directory plants
   nothing (the first drive's hook cells — kept, and void as plants). A hook that names the main
   checkout's merged area by its absolute path, makes the link and **succeeds** loses nothing: both
   docs are promoted and the link is moved aside (U6a). The same hook **refusing** the commit leaves
   the link in the standing area, and the next `jigc milestone finalize` writes through it at exit 0
   (U6b). That is a hook whose author makes a link at a staged body's name inside jigc's ignored
   staging directory — a plant by another hand, and a plant.
5. **Why that breaks no clause inside its scope.** The first clause's scope, in the closing
   condition's words (DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 1): *in a
   healthy repository used as documented … no jigc command at exit 0 destroys bytes no git object
   holds … Races against a non-jigc writer inside a millisecond window, and deliberately planted
   states, are declared bounds, written down with their reach.* Every loss driven here is at exit 0
   and of bytes no git object holds, and every one needs the link first. The undriven cells were
   the places a non-planted route could have been hiding; in the six driven, there is none.

**What this verdict does not settle.** The scope sentence calls planted states *declared bounds,
written down with their reach*. The run's opening declares no bound, and no written bound names a
link inside the merged area. This report reads the clause's own text and declares nothing; it is
item 1 of *Left open*, as it was the source report's.

`contested: false` — the finding argues against no settled decision.

No `regression` field is returned (the verdict is not `confirmed`). The same blocks were run on the
previous release all the same, because the cells are new facts about the door: the behaviour is the
same there, cell for cell (*The same blocks on the previous release*).

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`, exit 0. Both drivers put the handed binary's directory first on `PATH`
  and stop (exit 90) unless `command -v jigc` prints the binary they were handed; all four runs
  passed it.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  fresh`, stdout captured alone, the construction log to its own file, exit 0 each time — one rig
  per scenario: 12 and 3 per binary, plus one exploration rig on the candidate.
- The clone: `HEAD` 126a8531 on `fix/canary-one`; `git status --porcelain` lists only untracked
  files under `completions/artifacts/canary-one/r1/reports/test/`. Nothing was edited, staged or
  committed. Source lines cited here were read at that `HEAD`; `git diff --stat eeffe347 HEAD --
  crates/ tooling-tests/ dev/` is empty, so they are the candidate's.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user (uid 501).**

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-undriven.fZ2Mlx` (written `<W>`). Under it: `<W>/explore/` (one rig, candidate
only, used to learn the verbs — nothing here is read from it); `<W>/tools/drive.sh` and
`<W>/tools/drive-hook.sh`; and the logs every number here is read from — candidate `<W>/c.log` with
`<W>/c/`, and `<W>/ch.log` with `<W>/ch/` (the hook cells); previous release `<W>/p.log` with
`<W>/p/`, and `<W>/ph.log` with `<W>/ph/`. Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. Each cell runs with stdin from `/dev/null` or a file, stdout and stderr to
their own files, the exit status read directly and never through a pipe. An *inventory* is `find
<path>` handed entry by entry to `stat -f '%HT|%Sp|%z|nlink %l|%N|->%Y'`, which reads an entry's
shape **without following a link**. A file is shown by its shape, size, link count, sha256 and bytes.

**Two things about the tooling, stated because they are deviations of a kind.** The two drivers are
scratch files written from the shell with a here-document, not with the file tool; they are in no
repository. And the first driver's two hook scenarios named the merged area relative to the hook's
directory, so their plant never fired: they are in `<W>/c.log` and `<W>/p.log` under the labels
`U6a` and `U6b`, they prove only what item 4 above says of them, and the hook cells this report
counts are the second driver's.

## What was driven — the candidate

`<W>/c.log`: 164 cells — 155 exit 0, 6 exit 3, 3 exit 1. `<W>/ch.log`: 38 cells — 36 exit 0, 2 exit
1. Every exit 3 is a `jigc milestone finalize` over ADRs whose required slots are empty, blocked on
`schema-conformance.required-slot-present`. The exit 1 cells are U2c, U2d, U6b's and U6c's first
finalize, and — in the first driver's void hook scenario — a second finalize over a milestone that
had already landed (`milestone.terminal`).

Every scenario opens with the same fan-out unless the table says otherwise: `jigc milestone create
"Link probe"` · `jigc milestone add-task link-probe "alpha part"` · `… "beta part"` · `jigc doc
create adr --title "Alpha choice" --task alpha-part` · `jigc doc create adr --title "Beta choice"
--task beta-part`. *Fill* is the three `jigc doc set-slot <address>#<context|decision|consequences>
--task <task> --from-file -` calls per ADR that the blocked finalize's own routes print. `notes.md`
is an untracked file in the repository's root, 43 bytes, sha256 `c5c99d10…7ba1b4`: `user notes - no
git object holds this line`. Every plant is one `ln -sfn <target> <name>`.

| scenario | the cell of item 5 | the plant | what was run | exit | what happened |
|---|---|---|---|---|---|
| **C0** | control | none | finalize · fill · finalize | 3 · 0 | after the blocked finalize `merged/docs/` holds two regular files and `.jigc` holds **0** links; the landed finalize promotes both ADRs, `3 files committed`, nothing displaced, `notes.md` unchanged |
| **C1** | the handed spine, driven again as the baseline | after a blocked finalize, `merged/docs/adr:alpha-choice.md` replaced by a link to `<repo>/notes.md` (absolute) | fill · finalize | **0** | **`notes.md` is 276 bytes, sha256 `8bf97615…7ca951` — the ADR body; its own line is gone.** `promoted docs/decisions/beta-choice.md` only, `2 files committed`, `alpha-part: 1 doc`; the link moved to `.jigc/displaced/link-probe/merged/docs/`, named in a stderr note |
| **U1** | a link whose target is given relative | the same name, target `../../../../../notes.md` | fill · finalize | **0** | as C1, byte for byte: `notes.md` is the 276-byte ADR body, sha256 `8bf97615…7ca951`; the link is moved aside with its relative target as it was |
| **U2a** | a link at `merged/docs` itself | before any finalize: `merged/` made by hand, `merged/docs` a link to `<repo>/userdir/`, an untracked directory holding `keep.md` (30 bytes), `adr:alpha-choice.md` (53 bytes, the user's) and `adr:unrelated.md` (61 bytes, the user's) | fill · finalize | **0** | **`userdir/adr:unrelated.md` is gone** — no git object holds it. **`userdir/adr:alpha-choice.md` is 276 bytes, the ADR body**; `userdir/adr:beta-choice.md` now exists (269 bytes); `keep.md` unchanged. Both ADRs promoted, `3 files committed`. The note names `merged/docs` as the one entry moved aside and says nothing of `userdir/`; `notes.md` unchanged |
| **U2b** | a link at `merged` itself | `merged` a link to `<repo>/userdir2/`, untracked, holding `keep.md`, `docs/keep2.md` and `docs/adr:alpha-choice.md` (53 bytes, the user's) | fill · finalize | **0** | **`userdir2/docs/adr:alpha-choice.md` is 276 bytes, the ADR body**; `userdir2/docs/adr:beta-choice.md` now exists; both `keep` files unchanged. Both ADRs promoted, `3 files committed`; the link moved to `.jigc/displaced/link-probe/merged` |
| **U2c** | the same, dangling | `merged` a link to `<rig>/outside/nowhere`, which does not exist | fill · finalize | **1** | `blocking · milestone.area-io — could not open the parent staging area … File exists (os error 17)`; no commit, nothing made under `<rig>/outside/`, both sub-task areas intact, the link stands |
| **U2d** | the same, dangling | `merged/docs` a link to `<rig>/outside/nodocs`, which does not exist | fill · finalize | **1** | as U2c |
| **U3** | a body whose final address the join suffixed | both sub-tasks create an ADR titled `Shared choice`; after a blocked finalize, `merged/docs/adr:shared-choice-2.md` replaced by a link to `notes.md` | fill · join · finalize | 0 · **0** | join: `adr:shared-choice-2 (created · from beta-part) ← suffixed -2 on collision`. Finalize: **`notes.md` is 277 bytes — beta-part's ADR**; `promoted docs/decisions/shared-choice.md` only, `2 files committed`, `beta-part: 1 doc`; `shared-choice-2.md` is in no commit |
| **U4** | a body copied in from the committed store | first a `single-task` task commits `docs/decisions/base-choice.md`; then the fan-out, and `jigc doc set-slot adr:base-choice#context --task alpha-part` (*copied in for update*; provenance `edited-from-base`); after a blocked finalize, `merged/docs/adr:base-choice.md` replaced by a link to `notes.md` | fill · finalize | **0** | **`notes.md` is 271 bytes — `base-choice` with alpha-part's edit.** Both new ADRs promoted, `3 files committed`, `alpha-part: 2 docs`; **`docs/decisions/base-choice.md` is unchanged** (281 bytes, the same sha256 as before the fan-out) and `git grep` for the edit's line over `HEAD` exits 1 — the edit is in no commit; alpha-part's area is removed |
| **U5** | provisioned worktrees and staged code | `jigc milestone provision link-probe`; one file written and `git add`-ed in each sub-task worktree; after a blocked finalize (`.jigc` holds **0** links, worktrees included) the C1 link | fill · finalize | **0** | **`notes.md` is the 276-byte ADR body.** `added alpha-code.txt`, `added beta-code.txt`, `promoted docs/decisions/beta-choice.md`, `4 files committed`, `alpha-part: 1 doc, 1 code file`; `docs/decisions/alpha-choice.md` absent; the worktrees removed |
| **U6a** | a `pre-commit` hook as the planting hand; the hook succeeds | lines of another hand put ahead of jigc's block in `.git/hooks/pre-commit`: where the main checkout's `merged/docs` exists (named by its absolute path), `ln -sfn <repo>/notes.md <that>/adr:alpha-choice.md` | fill · finalize | **0** | the hook ran in `<repo>/.jigc/worktrees/.combine-<n>` and planted; **`notes.md` unchanged**; both ADRs promoted, `3 files committed`; the link moved to `.jigc/displaced/`, named in the note |
| **U6b** | the same; the hook plants and refuses the commit | the same lines, then `exit 1` | fill · finalize · (hook made quiet) · finalize | **1** · **0** | first: `` `git commit` was rejected (no commit was made) ``, `notes.md` unchanged, the area stands with **the link at `merged/docs/adr:alpha-choice.md`** and beta's regular body. Second: as C1 — **`notes.md` is the 276-byte ADR body**, `promoted docs/decisions/beta-choice.md` only, `2 files committed` |
| **U6c** | control for U6b | a hook that refuses and writes nothing | fill · finalize · (hook made quiet) · finalize | 1 · 0 | after the refusal `merged/docs/` holds two regular files; the second finalize promotes both ADRs, `3 files committed`, `notes.md` unchanged |

**What U2a's deletion is, mechanically (read, at eeffe347).** `materialize` calls
`clear_staged_bodies(…, merged/docs)` and then `create_dir_all` (`crates/engine/src/milestone.rs:2303`,
`:2304`). The clear lists the directory with `read_dir`, which follows a link *at the directory's own
name*, and removes every entry whose name reads as a staged identity and whose shape is a regular
file (`:2369`–`:2392`). Its shape rule (`:2386`) is asked of each entry and never of the directory.
So through a link at `merged/docs` — or one level up, at `merged` — the clear takes regular files
that live in somebody else's directory, and the write at `:2323` then lands there.

**Who made the link, cell by cell.** C1, U1, U2a–U2d, U3, U4, U5: this verifier, by `ln -sfn`.
U6a, U6b: a hook this verifier wrote to make it. No cell reached the state by a jigc verb or by an
act of git: the four inventories of `.jigc` taken before a plant (C0, U3, U4, U5) hold no link.

## What was not driven

- **Linux.** The handed candidate is a macOS binary and the prompt hands no trial image; nothing
  was built. Not driven.
- **A root caller.** Not driven; this verifier has no root.

Neither cell changes who makes the link, and this verdict rests on that alone — but that is an
argument, not a drive. For these two the finding stays as it was handed: *not driven*.

## The same blocks on the previous release

Not the regression step — the verdict is not `confirmed`. Rigs built with `--binary
<scratch>/bin/previous-91834b5e011d/jigc`; the same two drivers; logs `<W>/p.log` and `<W>/ph.log`.

- 164 and 38 cells, the same exits in the same cells (155 · 6 · 3, and 36 · 2).
- `diff` of each pair of logs, with the rig's name, the binary's path, the abbreviated commit ids
  and the dedicated worktree's name normalised: **two differences, neither in a cell's outcome.**
  (a) In U4, the sub-task's `provenance.json` carries a `copied-in` record on the candidate (289
  bytes) and none on the previous release (97 bytes) — the file as each binary writes it. (b) The
  hook's own log differs in its sha256 and not in its size, because it holds the dedicated
  worktree's name.

So on `1.0.0-rc.24` the write through the link lands in the same cells (C1, U1, U2a, U2b, U3, U4,
U5, U6b), U2a deletes the same file, and the safe cells are safe in the same way (C0, U2c, U2d,
U6a, U6c).

## Against the design that owns the behaviour

- **`crates/engine/src/milestone.rs`, the writer and its clear** (`:2260`–`:2267`, `:2348`–`:2368`):
  *jigc writes regular files, so a directory or a symlink wearing a staged identity's name is
  something else* — the rule the clear holds per entry. Nothing there says a link at `merged` or at
  `merged/docs` is followed; nothing says it is refused.
- **`crates/engine/src/state.rs:226`–`:240`, the milestone area's registry row**: under `merged/`
  jigc's set is the `docs/` *directory* `materialize` writes. The complement walk treats a link
  named `docs` as foreign and does not descend into it — which is why U2a's and U2b's notes name the
  link and nothing behind it.
- **`crates/cli/src/task.rs:9614`–`:9627`**: the milestone boundary's commit runs the hooks in a
  dedicated detached worktree. That is the settled reason a hook's relative path does not reach the
  main checkout's merged area (item 4).
- **DECISIONS.md, 2026-10-04, the fix-pass entry on replacing writers** — *a file jigc replaces is
  written as a regular file, never through a link* — as the source report read it: its cells are
  links git can deliver; these are links in an ignored directory.

None of these intends a write or a removal through a link here, so the basis is not `intended`. None
of them puts a planted link in the ignored staging directory inside the first clause's scope.

## The suites — read, not run

The finding makes no coverage claim; this is for the block's `pinned-by`. At eeffe347, read:

- `crates/cli/tests/merged_area_selective_clear.rs` plants a link under `merged/docs/` as
  `adr:a-link.md` (its `SHAPE_PLANTS`), a name its fixture's join does not produce.
- `crates/cli/tests/milestone_merged_complement.rs:400`–`:421` plants a link **named `docs`** under
  `merged/`, to a directory holding `adr:theirs.md`, and asserts that
  `engine::state::foreign_area_paths` returns `merged/docs` whole and undescended. It calls the
  enumeration only; it does not run `materialize` over that state. That is the near-miss of U2a: the
  same plant, and the walk that is fenced is not the one that deletes. Its other link is
  `adr:a-link.md` again (`:431`).
- `crates/cli/tests/flow53_acceptance.rs` plants one link, `link-to-notes.md`, at an area's root
  (its `FOREIGN_SHAPES`).
- How the three were found: `command grep -rln merged crates/cli/tests tooling-tests`, then
  `command grep -ln symlink` over the hits — 9 files. The other six hold no line that names a
  `milestones/` path (`command grep -c`, 0 in each), so none of them plants in a milestone's area;
  they were read no further than that count.

**The bound of this reading.** Textual, over the files that grep returned; no suite was run.

## Scope of what was verified

**Driven: the instance, in six further cells** — one writer (`crates/engine/src/milestone.rs:2323`)
and its clear (`:2369`), reached through their one verb (`jigc milestone finalize`), on two binaries,
on one platform, as a non-root caller.

**Not enumerated: the class.** Other writers and removers in jigc that list or open a directory
through a link at the directory's own name were not walked: `instance, unbounded`. The writers into
`merged/` and the writer's callers were enumerated by the source report (1 and 1); that enumeration
was not repeated here.

## Repro U-1

```yaml
claim: "one of the cells the source report left undriven reaches the write through a link under `.jigc/milestones/<id>/merged` without a deliberate plant, and so breaks `no-lost-files` inside its scope"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; what is below was OBSERVED, and the write and the removal through the link are a defect
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exits and outcomes in all 202 cells (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d); the logs differ only in U4's provenance.json (a `copied-in` record the candidate writes) and in the dedicated worktree's name inside the hook's log"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux, not driven as root"
setup:
  - fixture: fresh
  - ["jigc", "milestone", "create", "Link probe"]
  - ["jigc", "milestone", "add-task", "link-probe", "alpha part"]
  - ["jigc", "milestone", "add-task", "link-probe", "beta part"]
  - ["jigc", "doc", "create", "adr", "--title", "Alpha choice", "--task", "alpha-part"]
  - ["jigc", "doc", "create", "adr", "--title", "Beta choice", "--task", "beta-part"]
  - write: "notes.md in the repository's root, untracked: `user notes - no git object holds this line\n` (43 bytes)"
  - write: "userdir/ in the repository's root, untracked: keep.md (30 bytes), adr:alpha-choice.md (53 bytes), adr:unrelated.md (61 bytes) — each one line of the user's"
  - assert: "`find .jigc -type l` prints nothing — jigc has made no link"
  - plant: "mkdir .jigc/milestones/link-probe/merged; a symbolic link .jigc/milestones/link-probe/merged/docs -> <repo>/userdir"
  - fill: "for x in alpha, beta and s in context, decision, consequences: [\"jigc\", \"doc\", \"set-slot\", \"adr:<x>-choice#<s>\", \"--task\", \"<x>-part\", \"--from-file\", \"-\"] with one line of prose on stdin — exit 0 each"
repro:
  - ["jigc", "milestone", "finalize", "link-probe"]
expect:
  exit: 0
  stdout_contains: ["promoted docs/decisions/alpha-choice.md", "promoted docs/decisions/beta-choice.md", "3 files committed"]
  stderr_contains: ".jigc/milestones/link-probe/merged/docs → .jigc/displaced/link-probe/merged/docs"
  tree:
    - "userdir/adr:unrelated.md does not exist"
    - "userdir/adr:alpha-choice.md is a regular file of 276 bytes holding the ADR `# Alpha choice` with its three filled slots; the line it held is gone"
    - "userdir/adr:beta-choice.md exists, 269 bytes; userdir/keep.md is unchanged; notes.md is unchanged"
    - ".jigc/displaced/link-probe/merged/docs is the symbolic link, still pointing at userdir; .jigc/milestones/link-probe/ is gone"
variants:
  - "U1 — after a blocked finalize, merged/docs/adr:alpha-choice.md replaced by a link with the RELATIVE target ../../../../../notes.md; fill; finalize: exit 0, notes.md is the 276-byte ADR body, `promoted docs/decisions/beta-choice.md` only, 2 files committed"
  - "U2b — `merged` itself a link to an untracked userdir2/ holding docs/adr:alpha-choice.md: exit 0, that file replaced by the ADR body, docs/adr:beta-choice.md made there, both ADRs promoted, 3 files committed"
  - "U2c, U2d — `merged`, or `merged/docs`, a DANGLING link to a path outside the repository: exit 1 `milestone.area-io` (File exists, os error 17), no commit, nothing made at the target"
  - "U3 — both sub-tasks create `Shared choice`; the link at merged/docs/adr:shared-choice-2.md: exit 0, notes.md is beta-part's ADR (277 bytes), `promoted docs/decisions/shared-choice.md` only, 2 files committed"
  - "U4 — a committed docs/decisions/base-choice.md edited by alpha-part (edited-from-base); the link at merged/docs/adr:base-choice.md: exit 0, notes.md is the edited doc (271 bytes), docs/decisions/base-choice.md unchanged, the edit in no commit, `alpha-part: 2 docs`"
  - "U5 — `jigc milestone provision`, one file staged in each sub-task worktree; the link at merged/docs/adr:alpha-choice.md: exit 0, both code files and beta's ADR committed (4 files), notes.md is the ADR body"
  - "U6a — a pre-commit hook that makes that link by the merged area's ABSOLUTE path and exits 0: exit 0, notes.md unchanged, both ADRs promoted, the link moved to .jigc/displaced/"
  - "U6b — the same hook exiting 1: the finalize exits 1 (`git commit` was rejected), the link stands; with the hook quiet the next finalize exits 0 and is the spine of the source report"
control: "C0 — no plant: a blocked finalize leaves two regular files under merged/docs/ and no link under .jigc; the landed finalize promotes both ADRs, 3 files committed, nothing displaced. U6c — a hook that refuses and writes nothing: the next finalize promotes both ADRs"
observed: "<W>/c.log with <W>/c/runs/ (C0, C1, U1–U5) and <W>/ch.log with <W>/ch/runs/ (U6a–U6c); the previous release: <W>/p.log, <W>/ph.log"
pinned-by: "UNPINNED: found this round. The near-miss, read and not run: crates/cli/tests/milestone_merged_complement.rs plants a link named `docs` under merged/ to a directory holding adr:theirs.md and asserts only that engine::state::foreign_area_paths returns it undescended — it never runs materialize over that state; crates/cli/tests/merged_area_selective_clear.rs plants its link as adr:a-link.md, a name its fixture's join does not produce"
```

**Pinnable as it stands: no — in part.** The fixture is a named state of the shared builder and
every step but the plant is an argv; the plant is one `std::os::unix::fs::symlink` call, so the
block needs a Unix target, and U6 needs a hook file written by the test. **The block's `expect` and
the variants U1, U2b, U3, U4, U5 and U6b describe a defect**: converted as written they would pin
the removal and the write through the link. They are the red test of a fix, should the row be
admitted, or the reach of a bound, should one be declared. What converts as it stands, because it is
behaviour worth keeping: the `assert` of the setup (no link under `.jigc` after a fan-out, a blocked
finalize and a provision), the two controls, U2c and U2d (a dangling link refuses and writes
nothing — though see *Left open*, item 4, on its route), and U6a (a link a succeeding hook makes
costs nothing). The comparison with the previous release is not a suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **No written bound covers the state this verdict rests on.** The first clause's scope excludes
   *deliberately planted states* as declared bounds *written down with their reach*; the run's
   opening declares no bound. Whether a link planted inside jigc's own ignored staging directory is
   the kind that sentence means, and who writes the bound, is not a verifier's.
2. **Through a link at `merged/docs`, or at `merged`, `jigc milestone finalize` removes files in
   the linked directory at exit 0.** U2a: an untracked `adr:unrelated.md` in the user's directory —
   a name the join does not even produce — is gone afterwards, a same-named file is replaced, and
   jigc's bodies are left behind there. The boundary's one statement is the note about the link;
   nothing names the directory behind it. The same on both binaries. The enumeration half of this
   state is fenced (`milestone_merged_complement.rs`, the link named `docs`); the clear and the
   write are not.
3. **Under the plant, an edit to a committed doc is left out of the milestone's commit at exit 0
   while the summary counts it.** U4: `alpha-part: 2 docs`, `docs/decisions/base-choice.md`
   unchanged, the edit in no git object, the sub-task's area removed. The source report's item 2,
   for an `edited-from-base` body. And U3: the suffixed doc is in no commit while the summary says
   `beta-part: 1 doc`.
4. **The refusal over a dangling link names a cause that is not the cause.** U2c, U2d:
   `milestone.area-io … File exists (os error 17)`, route *resolve the underlying I/O condition (a
   disk or permissions problem on the `.jigc/` milestone area), then re-run the command*. The entry
   is a link; neither the finding nor the route says so. Not driven further.
5. **Not driven:** Linux; a root caller. And, beyond item 5 of the source report: a link at `merged`
   or `merged/docs` made *after* a blocked finalize (here both were made before any finalize); a
   link at `.jigc/milestones/<id>` or higher; a `merged/docs` link whose target is a directory the
   caller cannot write.
6. **The class is not bounded**: other places where jigc lists or opens a directory through a link
   at the directory's own name were not walked (`instance, unbounded`).

<!-- end of report -->
