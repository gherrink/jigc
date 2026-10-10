# verify-real — `r1-p3-doc-list-tracked-dangling-link-raw-os-error` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-doc-list-tracked-dangling-link-raw-os-error`. One finding, handed over:
ledger key `r1-p3-doc-list-tracked-dangling-link-raw-os-error`, door `jigc doc list`, the clause it
is said to break `working-product`, triage's grade *unclear*. It has no block of its own: its source
is item 2 of *Left open* (label K4.5-committed) in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-staging-area-writers-not-enumerated.a1.md`.
That report was read because the prompt hands it over as the finding; no other report was read, and
nothing of triage's reasoning beyond the grade and the re-drive it asks for.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** Not `does-not-reproduce`: the behaviour is
real, it reproduces on the candidate exactly as the finding words it, and it stays a row of the
ledger.

- **It reproduces.** Over a committed doc's home that is a tracked link whose target is gone,
  `jigc doc list` exits 1 with stdout empty and one line on stderr — `reading the committed doc at
  "<abs repo>/docs/research/context-loss.md": No such file or directory (os error 2)` — no finding
  code, no route, the machine's absolute path. `--format json` exits 1 with stdout empty and the
  same sentence as `{"error": …}` on stderr. One unreadable entry takes the whole unfiltered
  listing down, and the listing filtered to that doctype; a listing filtered to another doctype
  (`jigc doc list vision`) still answers at exit 0.
- **The listing did not answer on the previous release either.** The same block on a fresh rig of
  1.0.0-rc.24: the same exit status in all 35 cells, and all 30 jigc streams byte-identical (the
  rig's path normalised) — the failing ones included.
- **So nothing that works on the previous release stops working, and no printed route fails** —
  the refusal prints none. The state is a hand plant by a non-jigc writer, and the design that owns
  the behaviour already names this seam as a declared bound. See *Does it break the clause, inside
  its scope*.

`contested: false` — the finding argues against no settled decision. No `regression` field: the
verdict is not `confirmed`.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`, exit 0. The driver puts the handed binary's directory first on `PATH`
  and stops (exit 90) unless `command -v jigc` prints the binary it was handed, and stops (exit 95)
  unless the rig was built with that same binary; both runs passed both.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone, the construction log to a file of its own, exit 0 both
  times.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and
  after, `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user.** Nothing
was driven on Linux.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-dangling.I7AjQv` (written `<W>`). Under it: `<W>/c.rig/` and `<W>/p.rig/`, one
fresh rig per binary; `<W>/c.log` with `<W>/c.runs/` (candidate) and `<W>/p.log` with `<W>/p.runs/`
(previous release); `<W>/norm/`, the normalised copies the comparison read. Nothing was torn down.

Environment of every cell: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's repository. One driver, `<W>/tools/drive.sh
<binary> <rig env file> <out dir>`: each cell is one command with stdin from `/dev/null`, stdout and
stderr to files of their own, and its exit status read directly — no pipe anywhere a status is read.
**A deviation, stated:** the driver is a scratch file written from the shell; it is in no
repository.

**What was changed against the source cell.** The finding has no block; its source planted the link
at the home of an adr it had first landed through `jigc task finalize`. This re-drive reaches the
same door with a smaller setup: the rig state `refs-post-hoc` already commits a doc of a located
doctype, `research:context-loss` at `docs/research/context-loss.md`, and the link is planted at that
home. The door (`jigc doc list`, the committed arm), the shape (a tracked link at a managed home, its
target then removed, both steps committed) and the failing read are the same.

## What was driven — the candidate

Rig `<W>/c.rig/jigc-rig-refs-post-hoc-*`. 35 cells: 29 exit 0, 6 exit 1. The six: the block's own
probe `test -e` over the dangling link, and five listings — named below.

| step | what was done | what stands at the home | `jigc doc list` | `--format json` |
|---|---|---|---|---|
| C0 — control | nothing: the rig as built | `Regular File`, index mode `100644` | exit 0, header and 5 rows, `research:context-loss  docs/research/context-loss.md  managed` among them | exit 0, 5 objects |
| P1 then C1 — a live link | `mkdir notes` · `git mv docs/research/context-loss.md notes/context-loss-target.md` · `ln -s ../../notes/context-loss-target.md docs/research/context-loss.md` · `git add` of both · `git commit` — each exit 0; `git status --porcelain` empty | `Symbolic Link`, index mode `120000`; the target tracked at `100644` | exit 0, stdout **byte-identical to C0** — the read follows the link | exit 0, stdout byte-identical to C0 |
| P2 then C2 — the link dangles | `git rm -q notes/context-loss-target.md` · `git commit` — each exit 0; `git status --porcelain` empty | `Symbolic Link`, index mode `120000`; `readlink` prints `../../notes/context-loss-target.md`; `test -e` exits 1 | **exit 1, stdout empty**, stderr the one line below | **exit 1, stdout empty**, stderr the object below |
| R — recovery | `git revert --no-edit HEAD` (the target is back), exit 0 | the link, live again | exit 0, stdout byte-identical to C0 | exit 0, stdout byte-identical to C0 |

The plain refusal, whole (stderr, 297 bytes, one line; stdout 0 bytes):

```
reading the committed doc at "<abs repo>/docs/research/context-loss.md": No such file or directory (os error 2)
```

The json refusal, whole (stderr, 316 bytes; stdout 0 bytes):

```
{
  "error": "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": No such file or directory (os error 2)"
}
```

`<abs repo>` stands for the rig repository's absolute path on this machine, which both streams
print in full. Neither stream names a finding code, a route or a command: a search of the two for
`route`, `jigc ` with a trailing space, an arrow or `blocking` finds nothing.

**Five more cells over the dangling state, driven because they say what "the listing" covers:**

- `jigc doc list research` and `jigc doc list research --format json` — exit 1, the same two
  streams, byte for byte.
- `jigc doc list vision` and `jigc doc list vision --format json` — **exit 0**, the one `vision`
  row. A listing narrowed to a doctype that does not hold the entry is not taken down.
- `jigc doc list` a second time — exit 1, the same stream: the answer is stable, and
  `git status --porcelain` after the five refusals is empty — they wrote nothing the worktree
  shows.

**That the refusal is the dangling link's and nothing else's** is what the controls on either side
say: the same listing exits 0 with the same stdout over the regular file (C0), over the live link
(C1) and after the target is put back (R). Only the target's absence moves it.

## The same block on the previous release

Not the regression step — the verdict is not `confirmed` — but what triage asked for (*both
binaries … whether the listing answered there on the previous release*), and what the clause's
instrument is worded on. A fresh rig, `<W>/p.rig/jigc-rig-refs-post-hoc-*`, built with `--binary
<scratch>/bin/previous-91834b5e011d/jigc`; the same driver, with that binary's directory first on
`PATH`; log `<W>/p.log`.

- 35 cells, 29 exit 0 and 6 exit 1; the exit status is the same as the candidate's in every one
  (`cmp` of the two label-and-status lists, exit 0).
- The 30 streams of the 15 jigc cells (stdout and stderr each; the rig's path normalised to one
  token) are **byte-identical** between the two binaries — 30 same, 0 differing.
- **The listing did not answer there:** `jigc doc list` exit 1, stdout empty, the same line;
  `--format json` exit 1, stdout empty, the same object.
- What stood at the home was the same: `Symbolic Link`, index mode `120000`, the same link text.

The sentence is at one site in the candidate's source, `crates/cli/src/doc.rs:4867` (`std::fs::read`
of each path the committed enumerator yields, with that context string and a bare `?`), and the same
sentence is in the previous release's source at `crates/cli/src/doc.rs:4602` (read with `git show`
of the previous release's commit). Read, to say where the bytes come from; the verdict rests on the
drive.

## Against the design that owns the behaviour

- `design/doc-read-surface.md` → *`jigc doc list` — the fourth read surface*: the listing *is a
  report*, a row is emitted for every instance the enumerator yields, and an instance that does not
  parse counts 0 — *never a block*. An entry that cannot be **read** is not one of the cases that
  section settles; it says nothing of a home with no bytes behind it.
- `design/finalize.md` → *4. Promote managed docs* → **Declared bounds, (2)** names this seam in so
  many words: *Reads still follow a link at a home … and a read over an entry it cannot read is its
  own seam: `jigc doc list` answers a code-less exit 1 over one unreadable entry … Neither is a
  promote; both are filed with the review.* The observed behaviour is that sentence, driven.
- `DECISIONS.md` → *2026-10-06 — One code for a managed home that is not an ordinary file* → **Not
  this ruling's, and unchanged:** *a read through a link at a home*. `design/validation.md`, the
  `store.home-not-regular-file` row, says the same of that code: *a read through a link at a home
  (`jigc doc show`, a copy-in, the store sweep) follows it*.
- `design/surface-contract.md` → *The three laws*: law 1 holds *every printed path* to *repo-real
  or a typed identity*, and law 2 holds the surface that produces a state to naming its recovery.
  A refusal with no route and the machine's absolute path is what the behaviour falls short of
  there — the surface contract, not the closing condition's second clause.

So: no settled decision **intends** a raw OS error here — the basis is not `intended` — and none is
argued against. The design records the seam as known, open and filed.

## Does it break the clause, inside its scope

The clause, in the closing condition's words (`DECISIONS.md`, 2026-10-04, *The exit rule, revised*):
*a working product others can use and rely on*; its one instrument (sharpening 2): *no command that
works on rc.24 in a supported layout stops working, and every refusal's route works as printed.*

1. **No command that works on rc.24 stops working.** Over this state the command does not work on
   rc.24: exit 1, the same bytes. In the three states of the block where it does work on rc.24 —
   the regular file, the live link, the restored target — it works on the candidate, with
   byte-identical output. Nothing moved between the two releases at this door in this block.
2. **Every refusal's route works as printed.** The refusal prints no route, so there is no printed
   route to fail. *This is the instrument read as it is worded.* A reading under which a refusal
   that prints **no** route fails the instrument is not what the sentence says, and is not this
   verifier's to adopt; under that reading the finding would hold on rc.24 in the same bytes, and
   would still be no regression.
3. **In a supported layout.** The state was reached by two hand-made commits of a non-jigc writer:
   a link put at a managed doc's home, then its target removed. `design/finalize.md` (the paragraph
   above the declared bounds) states that *jigc writes regular files and real directories and never
   a link*, so an entry at a doc's home that is not a regular file is *a third party's entry*. The
   one route by a jigc verb into this state that the design records — a `docs-root` re-point
   carrying a relative link to a home where it dangles — is recorded there as closed at the
   relocation primitive, and `crates/cli/tests/store_door_home_shape.rs`,
   `a_root_repoint_refuses_to_carry_a_link_to_the_new_home`, asserts the refusal. **Both were read,
   not driven and not run by this verifier**, and that route starts from a link somebody else
   planted as well.

The defect is real: one unreadable entry denies the whole committed listing — the one sanctioned
route to the corpus — with a raw OS error, no code, no route and an absolute host path. It breaks
no clause inside its scope, because the previous release answers in the same bytes, no printed
route fails, and the state is planted. It is recorded with its row; the fix, if one is ruled, is a
surface-contract fix at a seam the design already files.

## Scope of what was verified

**`instance, unbounded`.** Driven: one located doctype (`research`), one doc, one shape (a tracked
link at the home, its target removed and the removal committed), the committed arm of `jigc doc
list` in both formats, unfiltered and under two doctype filters, on two binaries, on one platform.
The mechanism's consumers were not enumerated: the other readers of what the committed enumerator
yields, a placement doctype's home, the prior-home rows, and the other shapes of *an entry that
cannot be read* (a mode-000 file, a directory, a special file) are not counted here and were not
driven.

## Repro D-1

```yaml
claim: "over a committed doc's home that is a tracked link whose target has been removed, `jigc doc list` exits 1 with a raw OS error — no code, no route, an absolute host path — in both formats; said to break `working-product`"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; the behaviour itself reproduces
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d: the same exit status in all 35 cells, all 30 jigc streams byte-identical (the rig's path normalised)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc          # commits research:context-loss at docs/research/context-loss.md
  - ["mkdir", "notes"]
  - ["git", "mv", "docs/research/context-loss.md", "notes/context-loss-target.md"]
  - ["ln", "-s", "../../notes/context-loss-target.md", "docs/research/context-loss.md"]
  - ["git", "add", "docs/research/context-loss.md", "notes/context-loss-target.md"]
  - ["git", "commit", "-q", "-m", "plant: the research doc's home becomes a tracked link"]
control:
  - ["jigc", "doc", "list"]                         # the live link: exit 0, the research row present, stdout byte-identical to the listing before the plant
  - ["jigc", "doc", "list", "--format", "json"]     # exit 0, likewise
setup-2:
  - ["git", "rm", "-q", "notes/context-loss-target.md"]
  - ["git", "commit", "-q", "-m", "plant: the link's target is removed"]
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "--format", "json"]
  - ["jigc", "doc", "list", "research"]
  - ["jigc", "doc", "list", "vision"]
expect:
  - exit: 1
    stdout: ""
    stderr: "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": No such file or directory (os error 2)\n"
  - exit: 1
    stdout: ""
    stderr_json: { "error": "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": No such file or directory (os error 2)" }
  - exit: 1
    stdout: ""
    stderr_contains: "No such file or directory (os error 2)"
  - exit: 0
    stdout: "id  path  state\nvision:vision  VISION.md  managed\n"
  - tree: "after the refusals `git status --porcelain` is empty, and the home is still the link (`readlink` prints ../../notes/context-loss-target.md)"
recovery:
  - ["git", "revert", "--no-edit", "HEAD"]          # the target is back
  - ["jigc", "doc", "list"]                         # exit 0, stdout byte-identical to the listing before the plant
observed: "<W>/c.log with <W>/c.runs/ (candidate); <W>/p.log with <W>/p.runs/ (previous release)"
pinned-by: "UNPINNED: found this round. Searched: the sentence `reading the committed doc at` is in no file under crates/cli/tests, crates/engine/tests or tooling-tests (`command grep -rl`, one hit, the production site); `store_door_home_shape::a_root_repoint_refuses_to_carry_a_link_to_the_new_home` asserts the listing over a LIVE link and the relocation's refusal, not the listing over a dangling one. No suite was run by this verifier"
```

**Pinnable as it stands: yes, on a Unix target, with one thing to decide first.** The fixture is a
named state of the shared builder, and every step is an argv or one link creation
(`std::os::unix::fs::symlink` in a test), so the block converts by hand. What the converter has to
decide is *which half to pin*: the controls and the recovery (the listing answers over a live link
and again once the target is back) are facts to hold; the exit-1 cell pins a seam the design files
as open, and a standing test of it turns red the day the seam is closed — it then wants to be that
fix's red test, not a pinned fact. The comparison with the previous release is not a suite's to
hold: a suite drives one binary.

## Left open

Not pursued; each is for triage like any finding.

1. **The plain and the json refusal print the machine's absolute path** of the repository, where
   every other printed path of this door is repo-relative (the rows of the same listing). Seen in
   both formats on both binaries. It is part of this finding's own sentence and is named apart only
   so that a fix of the route does not leave it behind.
2. **Whether a jigc verb on the candidate can still carry a live link at a home into a dangling
   one** was not driven. `design/finalize.md` records the one known route (a `docs-root` re-point)
   as closed and a suite asserts it; this verifier read both and ran neither.
3. **Not driven:** Linux; a root caller; a placement doctype's home as a dangling link
   (`design/doc-read-surface.md` says the enumerator yields a placement instance on the file's
   existence, which a dangling link does not have, so the listing may drop the row at exit 0
   instead of refusing — read in the design, not driven); the prior-home rows; `jigc doc show`,
   `jigc validate` and the copy-in over the same state; a home that is unreadable for another
   reason (mode 000, a directory, a special file).

<!-- end of report -->
