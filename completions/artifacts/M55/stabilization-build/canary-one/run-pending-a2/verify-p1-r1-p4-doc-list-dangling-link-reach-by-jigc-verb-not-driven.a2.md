# verify-real — `r1-p4-doc-list-dangling-link-reach-by-jigc-verb-not-driven` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-doc-list-dangling-link-reach-by-jigc-verb-not-driven`. One finding, handed
over: ledger key `r1-p4-doc-list-dangling-link-reach-by-jigc-verb-not-driven`, door `jigc doc list`,
the clause it is said to break `working-product`, triage's grade *unclear*. It has no block of its
own: its source is item 2 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-doc-list-tracked-dangling-link-raw-os-error.a1.md`
— *whether a jigc verb on the candidate can still carry a live link at a home into a dangling one
was not driven; the design records the one known route (a `docs-root` re-point) as closed and a
suite asserts it; read, not run*. That report was read because the prompt hands it over as the
finding. No other report was read, and nothing of triage's reasoning beyond the grade.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** Not `does-not-reproduce`: the question the
finding asks has two answers, and one of them is *yes*.

- **The recorded route is closed on the candidate, and its printed route lands.** A `docs-root`
  re-point one directory deeper, over a located doc whose home is a tracked live relative link, is
  refused at exit 1 with nothing moved, the knob unchanged and the listing byte-identical to the
  listing before; the sibling knob, `placement-root`, refuses the same way over a placement doc's
  linked home. At both, the route the refusal prints was driven as printed and ends in a landed
  re-point, a regular file at the new home and a listing that answers.
- **On the previous release both re-points carried the link.** `docs-root`: exit 0, the link
  dangling at its new home, and `jigc doc list` then exit 1 for the whole store with the raw OS
  error. `placement-root`: exit 0, the link dangling, and the listing then exit 0 **with the
  `roadmap` row gone and nothing said**.
- **A jigc verb on the candidate can still take a live link at a home into a dangling one — by
  moving the link's target, not the link.** `jigc rename` of a doc that a tracked link at a
  *sibling* home points at exits 0, commits, and leaves that link dangling; `jigc doc list` then
  exits 1 for the whole store with the raw OS error. **The same block on the previous release: the
  same exit status in every cell and all 20 streams byte-identical.**
- **So no clause breaks inside its scope.** Where the two binaries differ, the candidate is the one
  that refuses and routes, by a ruling that intends it; where the dangling state is still
  reachable, the previous release reaches it in the same bytes; and no printed route fails. See
  *Does it break the clause, inside its scope*.

`contested: false` — the finding argues against no settled decision. No `regression` field: the
verdict is not `confirmed`.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`, exit 0. The driver repeats the check for the binary it is handed and
  stops (exit 90) unless `command -v jigc` prints it, and stops (exit 95) unless the rig's own
  `$JIGC` is that binary; all seven runs passed both.
- No `cargo build`, nothing under `target/`, no suite run. Rigs: `SCRATCH=<dir> dev/jigc-rig
  --binary <binary> refs-post-hoc`, stdout captured alone, the construction log to a file of its
  own, exit 0 seven times (one exploratory, six driven).
- The clone: `HEAD` 126a8531 on `fix/canary-one` before and after; its only untracked entries are
  other reporters' files under `completions/artifacts/canary-one/r1/reports/test/`, none of them
  written by this verifier. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user.** Nothing
was driven on Linux.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-reach.ubCalQ` (written `<W>`). Under it, one fresh rig per binary and scenario
(`<W>/c.rig.A`, `c.rig.D`, `c.rig.T`, `p.rig.A`, `p.rig.D`, `p.rig.T`), and one directory of
streams with its log per run (`<W>/c.runs.A`, `c.runs.D`, `c.runs.DR`, `c.runs.T`, `p.runs.A`,
`p.runs.D`, `p.runs.T`); `<W>/norm/` holds the normalised copies the comparison read. Nothing was
torn down.

Environment of every cell: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, a synthetic author identity in the environment, the working directory the
rig's repository. One driver, `<W>/tools/drive.sh <binary> <rig env file> <out dir> <scenario>`:
each cell is one command with stdin from `/dev/null`, stdout and stderr to files of their own, and
its exit status read directly — no pipe anywhere a status is read. After a step that changes the
tree, a `facts` record says what stands at the old home, the new home and the target (link text,
whether the target exists), their index modes, `git status --porcelain` and `HEAD`.

**Deviations, stated.** (1) The driver is a scratch file written from the shell; it is in no
repository. A second copy, `<W>/tools/drive2.sh`, adds scenario DR and changes nothing else. (2)
One exploratory rig (`<W>/rigs/`) was driven by hand first, to learn the refusal's shape and its
route; nothing below is taken from it but the two `jigc relocate` cells, which are marked. (3) The
finding has no block. The setup is the source report's plant on the rig state `refs-post-hoc` —
`research:context-loss` at `docs/research/context-loss.md`, its home turned into a tracked live
link — and the suite the design cites drives an `adr` on the `fresh` state instead; the door and
the shape are the same.

## What was driven

### Scenario A — the recorded route: `jigc config set docs-root handbook/sub`

The plant, each step exit 0 on both binaries: `mkdir notes` · `git mv
docs/research/context-loss.md notes/context-loss-target.md` · `ln -s
../../notes/context-loss-target.md docs/research/context-loss.md` · `git add` of both · `git
commit`. After it: the home is a symbolic link, index mode `120000`, its target exists; `git status
--porcelain` empty.

| step | candidate (28 cells: 27 exit 0, 1 exit 1) | previous release (19 cells: 14 exit 0, 5 exit 1) |
|---|---|---|
| A0 — `jigc doc list`, the rig as built | exit 0, 5 rows | exit 0, stdout byte-identical to the candidate's |
| A1 — `jigc doc list` over the live link | exit 0, stdout byte-identical to A0 | exit 0, stdout byte-identical to A0 |
| A2 — `jigc config set docs-root handbook/sub` | **exit 1**, stdout empty, the refusal below | **exit 0**, `relocating 1 committed doc(s)`, `docs/research/context-loss.md → handbook/sub/research/context-loss.md`, `config: set docs-root = handbook/sub` |
| what stands after A2 | the link at its old home, target exists; nothing at the new home; `git status --porcelain` empty; `HEAD` unmoved; `docs-root = docs/ (pack-default)` | nothing at the old home; **a symbolic link at `handbook/sub/research/context-loss.md`, same link text, target does not exist**; the move staged; `docs-root = handbook/sub (project)` |
| A3 — `jigc doc list`, and `--format json` | exit 0 both, stdout byte-identical to A0 | **exit 1 both, stdout empty**, the raw OS error below |
| after `git add -A` and `git commit` | (the route first — below) | exit 1 still: `jigc doc list`, `--format json`, `jigc doc list research`; `jigc doc list vision` exit 0 |

The candidate's refusal, whole (stderr, 1538 bytes; stdout 0 bytes):

```
relocating 1 committed doc(s) stranded by the `docs-root` re-point to `handbook/sub` (every committed doc under a doctype's prior resolved `location:` directory, managed or not — a placement doctype's file is not carried: it homes at its declared `placement.file`, which resolves through `placement-root`; each move is a staged `git mv` — commit it with your next commit):
blocking · config.repoint-failed — `docs-root` was not set to `handbook/sub`: blocking · store.home-not-regular-file — `docs/research/context-loss.md` is a symbolic link, not a regular file — jigc keeps a managed doc as a regular file at exactly its home, and neither writes through a link nor moves one, so it is not moved to `handbook/sub/research/context-loss.md`
  at: docs/research/context-loss.md
  route: nothing was written through `docs/research/context-loss.md` and it stands exactly as it was. jigc writes regular files only, so the link at `docs/research/context-loss.md` is not one it put there, and what becomes of it is yours to decide: put the doc itself at `docs/research/context-loss.md` as a regular file — for a link, a copy of the file it points at, in the link's place — and commit that; then re-run this command — the re-point was undone
  at: docs/research/context-loss.md
  route: `docs-root` is unchanged and every doc this re-point moved is back at its prior home. Fix what this message names, then re-run `jigc config set docs-root handbook/sub`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The previous release's listing after its re-point, whole (stderr, one line; stdout 0 bytes):

```
reading the committed doc at "<abs repo>/handbook/sub/research/context-loss.md": No such file or directory (os error 2)
```

`<abs repo>` stands for the rig repository's absolute path on this machine, which the stream prints
in full. The json form is the same sentence as `{"error": …}` on stderr, stdout empty.

**The candidate's route, driven as printed** (the same rig, straight after A3): a copy of the file
the link points at, taken (`cp`), the link removed (`unlink`), the copy put in its place (`cp`),
`git add` of the home, `git commit` — each exit 0; the home is then a regular file at index mode
`100644`, `git status --porcelain` empty. Then the command the route names, typed as it stands:

- `jigc config set docs-root handbook/sub` — **exit 0**: `relocating 1 committed doc(s)`,
  `docs/research/context-loss.md → handbook/sub/research/context-loss.md`, `config: set docs-root =
  handbook/sub`. A **regular file** stands at `handbook/sub/research/context-loss.md`, index mode
  `100644`; `docs-root = handbook/sub (project)`.
- `jigc doc list`, and `--format json` — exit 0, the row `research:context-loss
  handbook/sub/research/context-loss.md  managed`.
- After `git add -A` and `git commit` (exit 0 each; `git status --porcelain` empty): `jigc doc
  list`, `--format json`, `jigc doc list research`, `jigc doc list vision` — exit 0, all four.

### Scenario D — the sibling knob: `jigc config set placement-root handbook/sub`

The plant, each step exit 0 on both binaries: `mkdir notes` · `git mv docs/roadmap.md
notes/roadmap-target.md` · `ln -s ../notes/roadmap-target.md docs/roadmap.md` · `git add` of both ·
`git commit`. The listing over the live link: exit 0, 5 rows, on both.

| step | candidate | previous release |
|---|---|---|
| `jigc config set placement-root handbook/sub` | **exit 1**, stdout empty: `config.repoint-failed` wrapping `store.home-not-regular-file`, keyed at `docs/roadmap.md`, the same two routes with `placement-root` in the command to re-run. The stream first prints `docs/decisions-log.md → handbook/sub/decisions-log.md`, then the refusal that says the move was undone | **exit 0**: both `docs/decisions-log.md` and `docs/roadmap.md` moved, the knob written |
| what stands after it | the link at `docs/roadmap.md`, target exists; `git status --porcelain` **empty** — the one move it made is back; `placement-root` unchanged | **a symbolic link at `handbook/sub/roadmap.md`, link text `../notes/roadmap-target.md`, target does not exist**; `placement-root = handbook/sub (project)` |
| `jigc doc list`, and `--format json` | exit 0, 5 rows, `roadmap:roadmap  docs/roadmap.md  managed` among them | **exit 0, 4 rows — no `roadmap` row**, in both formats, before and after the move is committed; nothing on either stream says a doc is missing |

Cells: the candidate 15, of which 13 exit 0 and 2 exit 1 — the re-point, and the driver's closing
`git commit`, which had nothing to commit because the refusal left the tree clean; the previous
release 15, all exit 0.

**The candidate's route, driven as printed** (scenario DR, the rig scenario D left refused; 12
cells, all exit 0): the copy put where the link was and committed, then `jigc config set
placement-root handbook/sub` — exit 0, both singletons moved, a regular file at
`handbook/sub/roadmap.md`; `jigc doc list` before and after the commit — exit 0, 5 rows,
`roadmap:roadmap  handbook/sub/roadmap.md  managed` among them.

### Scenario T — one cell beyond the recorded route: the verb moves the link's target

Driven because the finding asks whether a jigc verb can *still* reach the state, and a refusal at
the door that moves the link answers only half of that. One shape, one verb, both binaries.

Setup, each step exit 0 on both: `jigc task discard ground-the-vision-in-research --force` (the
rig's live task; `jigc rename` refuses while a task is in flight) · `ln -s context-loss.md
docs/research/context-loss-alias.md` · `git add` of it · `git commit`. After it the alias is a
symbolic link at index mode `120000`, its target exists, and the listing answers at exit 0 with 6
rows — `research:context-loss-alias  docs/research/context-loss-alias.md  managed` is one of them:
the store reads the link as a doc at a home.

| step | candidate (10 cells: 8 exit 0, 2 exit 1) | previous release (10 cells: 8 exit 0, 2 exit 1) |
|---|---|---|
| `jigc rename research:context-loss --to "Context Decay"` | **exit 0**: `renamed research:context-loss -> research:context-decay (docs/research/context-loss.md -> docs/research/context-decay.md), repointed 0 referrer(s)`; stderr empty | exit 0, the same bytes |
| what stands after it | `docs/research/context-decay.md` a regular file; **`docs/research/context-loss-alias.md` a symbolic link whose target does not exist**; `git status --porcelain` empty — the rename committed | the same |
| `jigc doc list` | **exit 1, stdout empty**, stderr one line: `reading the committed doc at "<abs repo>/docs/research/context-loss-alias.md": No such file or directory (os error 2)` | exit 1, the same bytes |
| `jigc doc list --format json` | **exit 1, stdout empty**, the same sentence as `{"error": …}` on stderr | exit 1, the same bytes |

Between the two binaries: the label-and-status lists compare equal (`cmp`, exit 0), and the 20
streams of the 10 cells (stdout and stderr each, the rig's path normalised to one token) are
**byte-identical** — 20 same, 0 differing.

## Against the design that owns the behaviour

- `design/finalize.md` → *4. Promote managed docs* → *The doors that write a committed home in
  place* → **The relocation primitive**: *`git mv` moves a link as the entry it is, relative target
  and all: a `docs-root` re-point one directory deeper exited 0, left the link dangling at its new
  home, and `jigc doc list` then answered a code-less exit 1 for the whole store. A relocation
  lands a regular file or refuses, keyed at the source.* Scenario A on the previous release is the
  first sentence, driven; scenario A on the candidate is the second. The route paragraph below it —
  *the doc itself is put at its home as a regular file and committed, and the door's own command is
  then re-run as printed* — is what was driven at both knobs.
- `DECISIONS.md` → *2026-10-06 — One code for a managed home that is not an ordinary file*: the
  refusal's code, `store.home-not-regular-file`, at every command that meets the shape — both root
  knobs are named there. The candidate's exit 1 at A2 and at D's re-point is that ruling, built.
- The same paragraph of `design/finalize.md` lists the homes `jigc rename` asks about: *the doc's
  own, every referrer's, and the destination.* A link at **another** doc's home that points at the
  doc being renamed is none of the three, and the alias is no referrer (`repointed 0
  referrer(s)`). So scenario T is not a guard that failed: it is a route the design does not
  record. **Declared bounds, (2)** records what it ends in — *`jigc doc list` answers a code-less
  exit 1 over one unreadable entry … filed with the review* — and says nothing of which verbs can
  produce such an entry.
- `design/finalize.md`, the paragraph above the declared bounds: *jigc writes regular files and
  real directories and never a link*, so an entry at a doc's home that is not a regular file is *a
  third party's entry*. In all three scenarios the link is a hand plant by a non-jigc writer.

No settled decision **intends** the raw OS error in scenario T — the basis is not `intended` — and
the finding argues against none.

## Does it break the clause, inside its scope

The clause, in the closing condition's words (`DECISIONS.md`, 2026-10-04, *The exit rule,
revised*): *a working product others can use and rely on*; its one instrument (sharpening 2): *no
command that works on rc.24 in a supported layout stops working, and every refusal's route works as
printed.*

1. **No command that works on rc.24 stops working.**
   - *The listing.* In every state of the three scenarios where `jigc doc list` answers on the
     previous release, it answers on the candidate. The candidate answers in two more: after
     scenario A's re-point (the previous release exits 1) and after scenario D's (the previous
     release drops a row at exit 0). In scenario T it does not answer on either, in the same
     bytes.
   - *The two re-points.* Each exits 0 on the previous release and 1 on the candidate. Read on the
     exit status alone that is a command that stopped working; read on what it did, the previous
     release's exit 0 is the defect — it left the store unlistable, or one doc short without a
     word — and the candidate's refusal is the ruled behaviour (the entry of 2026-10-06 and the
     relocation primitive's paragraph, both above). A change a ruling intends is not this clause's
     break. Whether the regression set's list of intended changes holds a row for it was **not
     checked** by this verifier; it is named here so that the list's owner can.
   - *The rename.* Exit 0 on both, the same bytes.
2. **Every refusal's route works as printed.** Two refusals print routes, and both were driven as
   printed to a landed end state (scenario A's route, scenario DR). The listing's exit 1 in
   scenario T prints no route, so there is none to fail — the instrument read as it is worded, the
   reading the source report states and this one does not widen.
3. **In a supported layout.** Not needed for the verdict, and not settled by it: points 1 and 2
   hold whichever way a hand-made link in a doctype's directory is classed. For the record — the
   design calls the entry a third party's, and it also serves it: the listing prints the alias as a
   `managed` row at exit 0 while its target stands.

**What stays a row.** A committing jigc verb at exit 0 (`jigc rename`) can leave a tracked link at
a managed home dangling, and one such entry then denies the whole committed listing with a raw OS
error — no code, no route, an absolute host path. It is real on the candidate, it is the same on
the previous release, and it is reached only over a link jigc did not write. It breaks no clause
inside its scope; the fix, if one is ruled, belongs with the read seam the design already files.

## Scope of what was verified

**`instance, unbounded`.** Driven: the recorded route at `docs-root` (one located doctype,
`research`, one doc) and at `placement-root` (one placement doc, `roadmap`), each with its printed
route on the candidate; and one target-moving cell (`jigc rename` of a doc a sibling link points
at) — on two binaries, on one platform. The verbs that can move or remove a link's target were
**not enumerated**; the count of them is not known to this verifier, and `jigc rename` is one
instance taken because it was the nearest.

Of the four commands the design puts behind the relocation primitive, two were driven as carriers
(`config set docs-root`, `config set placement-root`). `jigc relocate` could not be reached with a
shipped doctype — `jigc relocate research --from legacy/` and `jigc relocate vision --from legacy/`
both answer `relocate.frozen-doctype` at exit 1 on the exploratory rig (candidate only), before any
move — and `jigc rename` over a link at the doc's **own** home was not driven: a rename keeps the
directory, so a relative link carried by it would not dangle.

## Repro E-1

The recorded route. Refuted on the candidate: the re-point refuses and its route lands.

```yaml
claim: "a `docs-root` re-point on the candidate carries a live relative link at a located doc's home to a new home where it dangles, and `jigc doc list` then exits 1 for the whole store; said to break `working-product`"
verdict: REFUTED   # the re-point refuses, nothing moves, the listing answers, and the printed route lands
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d: the re-point exits 0, the link dangles at handbook/sub/research/context-loss.md, and `jigc doc list` exits 1 with `reading the committed doc at \"<abs repo>/handbook/sub/research/context-loss.md\": No such file or directory (os error 2)`"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc          # commits research:context-loss at docs/research/context-loss.md
  - ["mkdir", "notes"]
  - ["git", "mv", "docs/research/context-loss.md", "notes/context-loss-target.md"]
  - ["ln", "-s", "../../notes/context-loss-target.md", "docs/research/context-loss.md"]
  - ["git", "add", "docs/research/context-loss.md", "notes/context-loss-target.md"]
  - ["git", "commit", "-q", "-m", "plant: the research doc's home becomes a tracked link"]
control:
  - ["jigc", "doc", "list"]                          # exit 0, the research row at docs/research/context-loss.md
repro:
  - ["jigc", "config", "set", "docs-root", "handbook/sub"]
  - ["jigc", "config", "get", "docs-root"]
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "--format", "json"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["config.repoint-failed", "store.home-not-regular-file", "`docs/research/context-loss.md` is a symbolic link", "re-run `jigc config set docs-root handbook/sub`"]
  - exit: 0
    stdout: "docs-root = docs/  (pack-default)\n"
  - exit: 0
    stdout: "byte-identical to the control's"
  - exit: 0
  - tree: "`git status --porcelain` is empty, HEAD is unmoved, docs/research/context-loss.md is still the link and its target exists, nothing stands at handbook/sub/research/context-loss.md"
route:                                               # the refusal's, as printed
  - "copy notes/context-loss-target.md; unlink docs/research/context-loss.md; put the copy at docs/research/context-loss.md"
  - ["git", "add", "docs/research/context-loss.md"]
  - ["git", "commit", "-q", "-m", "route: the doc itself, where the link was"]
  - ["jigc", "config", "set", "docs-root", "handbook/sub"]   # exit 0; handbook/sub/research/context-loss.md is a regular file, index mode 100644
  - ["jigc", "doc", "list"]                                   # exit 0, the row `research:context-loss  handbook/sub/research/context-loss.md  managed`
sibling:                                             # the same block at the other root knob, on a rig of its own
  - "plant: docs/roadmap.md becomes a tracked link to ../notes/roadmap-target.md, committed"
  - ["jigc", "config", "set", "placement-root", "handbook/sub"]   # exit 1, the same two codes, keyed at docs/roadmap.md; `git status --porcelain` empty
  - ["jigc", "doc", "list"]                                        # exit 0, 5 rows, the roadmap row at docs/roadmap.md
  - "the route, as above, then the same command: exit 0, a regular file at handbook/sub/roadmap.md, the listing exit 0 with 5 rows"
observed: "<W>/c.runs.A/log, <W>/c.runs.D/log and <W>/c.runs.DR/log with their streams (candidate); <W>/p.runs.A/log and <W>/p.runs.D/log (previous release)"
pinned-by: "store_door_home_shape::a_root_repoint_refuses_to_carry_a_link_to_the_new_home — read assertion by assertion, NOT run by this verifier: it asserts exit 1, both codes, `a symbolic link`, the home, HEAD and status unmoved, the link untouched, `jigc doc list` succeeding, the emitted re-point landing once the home is a regular file, regular files at the new homes and the listing naming the doc there. The near-miss: it drives an `adr` on the `fresh` state at `docs-root`; this block drives `research` on `refs-post-hoc`, and the `placement-root` half is not that test's — `home_shape_one_code` drives `config set placement-root` over a link (seen at the file's lines, not read whole, not run)"
```

**Pinnable as it stands: yes, on a Unix target — and its `docs-root` half is pinned already** by the
test named, for another doctype. What is not held by a test this verifier read: the
`placement-root` route end to end, and the previous release's half, which no suite can hold — a
suite drives one binary.

## Repro E-2

The route that still reaches the state. Identical on both binaries.

```yaml
claim: "a jigc verb at exit 0 leaves a tracked link at a managed home dangling, and `jigc doc list` then exits 1 for the whole store with a raw OS error"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; the behaviour itself reproduces, on both binaries, in the same bytes
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d: the same exit status in all 10 cells, all 20 streams byte-identical (the rig's path normalised)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc
  - ["jigc", "task", "discard", "ground-the-vision-in-research", "--force"]   # the rig's live task; `jigc rename` refuses while one is in flight
  - ["ln", "-s", "context-loss.md", "docs/research/context-loss-alias.md"]
  - ["git", "add", "docs/research/context-loss-alias.md"]
  - ["git", "commit", "-q", "-m", "plant: a tracked link beside the research doc, pointing at it"]
control:
  - ["jigc", "doc", "list"]                          # exit 0, 6 rows, `research:context-loss-alias  docs/research/context-loss-alias.md  managed` among them
repro:
  - ["jigc", "rename", "research:context-loss", "--to", "Context Decay"]
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "--format", "json"]
expect:
  - exit: 0
    stdout_contains: "renamed research:context-loss -> research:context-decay (docs/research/context-loss.md -> docs/research/context-decay.md), repointed 0 referrer(s)"
    stderr: ""
  - exit: 1
    stdout: ""
    stderr: "reading the committed doc at \"<abs repo>/docs/research/context-loss-alias.md\": No such file or directory (os error 2)\n"
  - exit: 1
    stdout: ""
    stderr_json: { "error": "reading the committed doc at \"<abs repo>/docs/research/context-loss-alias.md\": No such file or directory (os error 2)" }
  - tree: "`git status --porcelain` is empty after the rename; docs/research/context-loss-alias.md is a symbolic link whose target does not exist; docs/research/context-decay.md is a regular file"
observed: "<W>/c.runs.T/log with its streams (candidate); <W>/p.runs.T/log (previous release); <W>/norm/ for the comparison"
pinned-by: "UNPINNED: found this round. Searched: the sentence `reading the committed doc at` is in no file under crates/ or tooling-tests/ but its one production site, crates/cli/src/doc.rs:4867 (`command grep -rln`, one hit). No suite was run by this verifier"
```

**Pinnable as it stands: yes, on a Unix target, with the source report's caveat.** The fixture is a
named state and every step is an argv or one link creation. The exit-1 cells pin a seam the design
files as open: a standing test of them turns red the day the seam is closed, so they want to be
that fix's red test and not a pinned fact. What is a fact to hold either way is the control — the
listing answers over a live link at a home.

## Left open

Not pursued; each is for triage like any finding.

1. **`jigc rename` leaves a link at another doc's home dangling, at exit 0** (Repro E-2). The
   rename asks about the doc's own home, its referrers' and the destination; a link elsewhere in
   the store that points at the renamed file is none of them. The same on the previous release.
   Whether the rename should see it, or the listing should survive it, is a ruling and not this
   verifier's.
2. **The other verbs that move or remove a file a link at a home may point at were not driven and
   not counted** — `jigc migrate` retiring a foreign original a link at a home points at,
   `jigc migrate-corpus`'s relocation arm, a root re-point whose link stays behind while its target
   moves (a link at a placement home pointing into the `docs-root` tree), `jigc task discard`,
   `jigc uninstall`. Named from the command surface, not from a drive.
3. **On the previous release a `placement-root` re-point over a linked placement home drops the
   doc from the listing at exit 0, in both formats, with nothing said** (scenario D). Closed on the
   candidate by the refusal. Whether the candidate's listing drops the row the same way over a
   placement home that is made a dangling link **by hand** was not driven.
4. **The first route sentence of the re-point's refusal ends `then re-run this command — the
   re-point was undone`**, and names no command; the second route names it. Driven as the second
   prints it, and it lands. Wording only.
5. **Whether the regression set's list of intended changes carries the two re-points** (exit 0 on
   the previous release, exit 1 on the candidate, over a link at a home) was not checked.
6. **Not driven:** Linux; a root caller; `jigc relocate` over a freeze-exempt doctype (no shipped
   doctype is one); `jigc rename` over a link at the doc's own home; `jigc doc show`, `jigc
   validate` and the copy-in over the dangling state of scenario T.

<!-- end of report -->
