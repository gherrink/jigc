# verify-real — `r1-doc-list-unreadable-entry-fails-listing` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p1-r1-doc-list-unreadable-entry-fails-listing`. One finding, handed over by triage with the
grade *unclear*: door `jigc doc list`, the clause it is said to break `working-product`, its repro
block `Repro RC-1` (`row-doc-list-reconciler.a1.md`), with `Repro S3` (`row-doc-list-source.a1.md`)
and `Repro 2` (`row-doc-list-driver.a1.md`) for the same defect.

## Verdict

- **verdict:** `refuted` — **as a blocker**. The defect is real and reproduces exactly; it stays a row of the ledger.
- **basis:** `breaks-no-clause` — the block reproduces as written on the candidate (exit 1, stdout
  empty, one raw OS-error line, no route; a named pipe blocks the verb), but every cell is
  byte-identical on `1.0.0-rc.24`, so no command that works there stops working, and the refusal
  prints no route, so no route fails as printed. It is **not** `does-not-reproduce`, and it is
  **not** `intended`: no settled decision says an unreadable entry should end the listing.
- **contested:** `false` — the finding does not argue that a decision is wrong. One reading of the
  clause was needed to reach the verdict; it is stated under *Step 3* so that it can be overruled.
- **regression:** not returned (it goes with `confirmed` only). The fact that would fill it was
  established all the same, because the clause's first half asks it: **not a regression**, 21
  cells on the previous release, each the same exit, stdout and stderr as the candidate's.
- **class:** `instance, unbounded` — one read site in this door was driven (`doc.rs:4866`); the
  mechanism's other consumers were not enumerated by this reporter.
- **platform:** **macOS only** — macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157). Nothing here was
  driven on Linux: the binary handed over is a macOS binary and a verifier builds none. The
  reconciler's lead LD-3 (*one platform*) is as open after this report as before it.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (`1.0.0-rc.24`).
- For every invocation the driven binary's directory went first on `PATH` and `command -v jigc`
  was compared with `<that directory>/jigc` before the verb ran (the driver exits 97 otherwise; it
  never did). No `cargo build`; nothing under `target/` was driven.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` and `HEAD`
  eeffe347 after the drives. Nothing was built, edited, staged or committed.

## Step 1 — driven again from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p1-doc-list-unreadable.79zAdR` (written `<W>`). **Every shape got a root of its
own**, minted with `mktemp -d` under `<W>`, so no plant was ever removed to make room for the next
(the one `unlink` is the control of the rig group, and is named there). No root of any reporter
was read or reused.

Environment of every invocation: `HOME=<root>/home` (a fresh directory), `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*`
variables, stdin from `/dev/null`.

The driver (`<W>/tools/drive.sh`), in this order: `git status --porcelain --untracked-files=all
--ignored`; a snapshot of every entry under the root (kind, mode, size, mtime in nanoseconds,
sha256 or link target, path); the invocation, stdout and stderr each to its own file, **the exit
status read directly from the bare command**; the snapshot again; the porcelain again. No pipe
stands between the verb and its exit status, and no output was cut: byte counts are of the whole
files.

### The block as written — `Repro RC-1` (and the driver's `Repro 2`, the same setup)

Setup, step for step: `git init -q repo` (exit 0) · `git -C repo commit -q --allow-empty -m base`
(exit 0) · `jigc setup` in `repo` (exit 0, *adapter installed*) · `mkdir -p docs/decisions` ·
`ln -s /nonexistent/nowhere.md docs/decisions/dangling.md`. Driven **twice on the candidate**, in
two roots (`<W>/a-dangling-cand.62pDUo`, `<W>/v1-dangling-cand.p9SOg2`), and once on the previous
release (`<W>/v1-dangling-prev.GPcwHW`).

| argv | exit | stdout | stderr | tree |
|---|---|---|---|---|
| `jigc doc list` before the plant (control) | 0 | `jigc doc list — no committed docs` (36 bytes) | empty | IDENTICAL |
| `jigc doc list` | **1** | **empty** (0 bytes) | one line: `reading the committed doc at "<root>/repo/docs/decisions/dangling.md": No such file or directory (os error 2)` | IDENTICAL |
| `jigc doc list --format json` | **1** | empty | `{ "error": "reading the committed doc at \"<root>/repo/docs/decisions/dangling.md\": No such file or directory (os error 2)" }` — three lines, one key | IDENTICAL |
| `jigc doc list adr` | **1** | empty | the same line as the unfiltered listing | IDENTICAL |
| `jigc doc list vision` | 0 | ``jigc doc list — no committed `vision` docs`` (45 bytes) | empty | IDENTICAL |
| `jigc validate` | 0 | `no findings — the committed store validates clean`, then the trailer (125 bytes) | empty | IDENTICAL |

The block's three expectations hold to the word. `<root>` in the message is the machine's absolute
path, spelled out in full.

### The variants — each alone, in a root of its own

Triage asked for one; all four of the block's were driven, on both binaries.

| shape, in place of the `ln` | argv | exit | stdout | stderr | root (candidate) |
|---|---|---|---|---|---|
| `mkdir CHANGELOG.md` — a directory at a **placement** home | `jigc doc list` | **1** | empty | `reading the committed doc at "<root>/repo/CHANGELOG.md": Is a directory (os error 21)` | `<W>/v2-dir-placement-cand.0XvJ1k` |
| `mkdir -p docs/research/a-dir.md` — a directory named `x.md` at a located home | `jigc doc list` | **1** | empty | `reading the committed doc at "<root>/repo/docs/research/a-dir.md": Is a directory (os error 21)` | `<W>/v3-dir-located-cand.hsG5Ol` |
| the same | `jigc doc list --format json` | **1** | empty | the `{ "error": … }` envelope around the same message | the same |
| `docs/ideas/locked.md` = `# x`, then `chmod 000` (`ls -l` read `----------`) | `jigc doc list` | **1** | empty | `reading the committed doc at "<root>/repo/docs/ideas/locked.md": Permission denied (os error 13)` | `<W>/v4-mode000-cand.epPEYa` |
| `mkfifo docs/research/pipe.md` (`ls -l` read `prw-r--r--`) | `jigc doc list` under `perl -e 'alarm shift; exec @ARGV' 8 jigc doc list` | **142** — killed by the alarm after 8 s | empty | empty — the verb printed nothing and did not return | `<W>/v5-fifo-cand.RRPaUe` |

Tree IDENTICAL and porcelain unchanged around every one, the killed one included.

### A store that holds docs — `Repro S3`'s setup, and the cell the round's change reaches

`Repro RC-1`'s store is empty, so *the whole listing fails* is there a listing of nothing. The
source's block runs over a populated store; it was driven as written, on a rig built with
`SCRATCH=<W> dev/jigc-rig --binary <binary> refs-post-hoc`, stdout captured alone
(`<W>/jigc-rig-refs-post-hoc-smPtHN` for the candidate, `<W>/jigc-rig-refs-post-hoc-qj0OPw` for
the previous release). A linked worktree was added first (`git worktree add -q <rig>/wt -b feature`),
because the round's one change in `fn run_list` is the served-from note of a linked worktree.

| state | argv, cwd | exit | stdout | stderr |
|---|---|---|---|---|
| no plant | `jigc doc list`, main checkout | 0 | header + **five** `managed` rows (257 bytes) | the staged-listing note |
| no plant | `jigc doc list`, the linked worktree | 0 | the same 257 bytes | the staged-listing note, then — **candidate only** — the served-from note |
| `ln -s nowhere-target docs/research/dangling.md` | `jigc doc list`, main checkout | **1** | **empty — none of the five rows** | `reading the committed doc at "<rig>/repo/docs/research/dangling.md": No such file or directory (os error 2)` |
| the same | `jigc doc list --format json` | **1** | empty | the `{ "error": … }` envelope |
| the same | `jigc doc list research` | **1** | empty | the same line |
| the same | `jigc doc list vision` | 0 | header + the `vision` row | the staged note, narrowed |
| the same | `jigc validate` | 0 | one unrelated advisory (`schema-conformance.repeatable-populated` on the decisions log) and the report-only trailer; `dangling.md` is not named | empty |
| the same | `jigc doc show vision:vision --format json` | 0 | the doc, 396 bytes | the staged-read note |
| the same | `jigc doc list`, the linked worktree | **1** | empty | the same one line — **no served-from note on either binary** |
| `unlink docs/research/dangling.md` | `jigc doc list`, main checkout | 0 | byte-identical to the first row (`cmp` exit 0, stdout and stderr) | |

So one untracked dangling link in one doctype's folder takes all five rows of four other doctypes
out of the unfiltered listing, in both formats; a filter for another doctype and `doc show` by
address still answer.

### Does any cell print a route

**No.** Over the stderr file of each of the fourteen exit-1 cells driven on the candidate: one line
(three for json), **0** lines holding the word `route`, **0** holding a backtick, **0** holding
`jigc <verb>`, **0** holding a `blocking ·` / `advisory ·` finding line; stdout is 0 bytes in each.
The named-pipe cell printed nothing at all. The message carries no finding code and no key.

### The tally

**48 invocations: 27 on the candidate, 21 on the previous release.** On the candidate 14 exit 1,
1 killed by the alarm (142), 12 exit 0; on the previous release 11 exit 1, 1 killed, 9 exit 0.
**Tree IDENTICAL, mtime included, and porcelain unchanged around all 48** (88 to 93 entries in a
fresh root, 176 to 180 in a rig).

## Step 2 — is it what the finding says

**Yes, in every cell.** The attempts to make it not so, and what each found:

- *Stale state* — every root was minted for this report; the control before the plant exits 0, and
  in the rig the control after the `unlink` is byte-identical to the control before the plant. The
  exit 1 follows the plant and nothing else.
- *Read through a pipe, or cut* — the exit status is the bare command's; stdout and stderr are
  whole files, counted in bytes.
- *Another binary* — the `PATH` check ran before every invocation; both hashes were asserted first.
- *Behaviour a settled decision intends* — **not found.** What the owning docs say:
  - `design/doc-read-surface.md` → *`jigc doc list` — the fourth read surface*: "**A row is emitted
    for every instance the enumerator yields, carrying its state**", and, of an instance that does
    not parse, "never a block: `doc list` is a report". The section's table of row values has rows
    for *managed, parses* · *managed, does not parse* · *unregistered* · *orphaned* · *staged*. **It
    has no row, no state and no sentence for an instance that cannot be read.** The driven
    behaviour — an enumerated instance for which no row is emitted, and the listing ended — is not
    what the first sentence says, and no sentence of the section says a read fault ends the listing.
  - `design/command-output-contract.md` → *The two reject arms*: "An operational error carries the
    single key `error` … exit 1 — and nothing else", and its exit table lists "a git/IO failure"
    under exit 1. That settles the **shape** an I/O failure takes once a verb gives up — `{error}`,
    exit 1, no route key — and the driven json matches it. It does not settle **that** this verb
    gives up on one entry.
  - `design/validation.md` → *Every finding carries a route — two kinds, never `null`*: the route
    floor is over **findings**. The driven refusal is not a finding.
  - The standing suites lean the same way as the first bullet and say nothing of this state:
    `crates/cli/tests/flow52_acceptance.rs` asserts of an orphan state "`doc list` is a report and
    stays exit 0".

  So this is a defect against the door's own stated posture, in a state its design does not
  describe. It is not `intended`.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, in `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We have a working product
others can use and relay on", with its one instrument (the second sharpening): "**no command that
works on rc.24 in a supported layout stops working, and every refusal's route works as printed**".
And the rule: "A finding blocks the 1.0.0 call only if it breaks a clause inside its scope;
everything else is recorded with its tier."

**First half — no command that works on rc.24 stops working.** Driven on the previous release
(sha256 accf3996…), the block and all four variants in fresh roots and the ten rig cells: 21 cells.
A `diff` of the two whole logs, with the root names and the binary's path normalised, differs in
**the entry count of the snapshot** (the candidate's `setup` installs two entries more) and in
**one stderr**: the no-plant listing from the linked worktree, where the candidate adds the
served-from note — the round's change, in a cell that exits 0 on both. **Every exit status, every
stdout and every other stderr is the same**, the four refusal cells of the rig byte-identical after
normalising the rig's name (sha256 prefix `ff4c7a1d603299bc` plain, `f8e69dd323cc2a1d` json, on
both binaries). In each of these states the command does not work on rc.24 either. Nothing stops
working. **Not broken.**

**Second half — every refusal's route works as printed.** The refusal prints no route (above), so
there is no printed route to run and none that fails. The reading taken here: the sentence is
about the routes refusals **print**. Two things support it — the fix pass's own brief words the
same check as "every route a refusal prints is run as printed on a built binary" (`DECISIONS.md`,
the round-4 entry), and the contract that owns the `{error}` arm gives it one key and no route.
**Not broken.**

*The reading this verdict rests on, said so that it can be overruled.* If the sentence were read as
*every refusal has a route, and it works*, all fourteen exit-1 cells would break it — and so would
every other `{error}`-arm refusal of the binary, which is why this reporter does not read it so.
The named pipe is no refusal at all: the verb does not return. That is a command that does not
work in that state — on rc.24 as well, so the first half is where it is judged, and it does not
break it.

**The scope does not excuse the state, and is not what the verdict leans on.** The layout is an
ordinary checkout — a supported one. A dangling link, a directory named `x.md` and a file the user
cannot read are untracked accidents, not planted states; the named pipe is the one shape that is
hard to arrive at by accident. No declared bound was used: the run's opening declares none.

**Verdict on the clause: `working-product` is not broken inside its scope.** `refuted` as a
blocker, basis `breaks-no-clause`. What stays true and stays on the ledger: one stray entry turns
the index read an agent is told to use into exit 1 with zero rows, the machine's absolute path,
the words "the committed doc" for an entry git never held, and no next step — while `jigc validate`
calls the same store clean.

## Step 5 — the coverage claim inside the finding

The block says `pinned-by: "UNPINNED: no test found for an unreadable entry at a managed home on
this door (the source's search; not repeated here)"`. **Verified here from the suites, and it
holds:**

- `grep -rn "reading the committed doc at" crates tooling-tests` — **1 hit**, the production line
  `crates/cli/src/doc.rs:4867`. No test names the message.
- The door's own suite, `crates/cli/tests/doc_list.rs`: eleven tests, read by name and the two
  refusing ones by body — an unknown doctype (`store.unknown-type`) and an unknown task id. None
  plants an entry that cannot be read.
- Files under `crates/cli/tests`, `crates/engine/tests` and `tooling-tests` that drive
  `"doc", "list"`: **43**. Of those, **13** also hold a token of a planted shape (`symlink(`,
  `set_permissions`, `mkfifo`, `from_mode(0o000)`, `os error`). Each of the 13 was read at its
  `doc list` call: every one asserts **success** of the listing, or a refusal with another cause
  (`unreadable_project_layer.rs` — an unreadable `.jigc/`, the project layer and not an instance).
  The nearest, `store_door_home_shape.rs`, names this very state in its header — a mover that left
  a dangling link, "after which `jigc doc list` could not enumerate the store" — and closes it at
  the **writing** door: its two `doc list` calls assert that the store still enumerates once the
  mover refuses. The listing is never run over the link.
- A scan of every suite for a `doc list` call with a failure asserted within ten lines flagged 12
  windows; all 12 were read, and none is this state.

No test was run by this reporter: a verifier builds nothing. This is a statement about the text of
the suites at eeffe347.

## The repro block

### Repro V-1

```yaml
claim: "one untracked entry named *.md that cannot be read as a file, at a home `jigc doc list` enumerates, makes the listing exit 1 with no rows and no route — and that breaks the closing condition's `working-product` clause (ledger key r1-doc-list-unreadable-entry-fails-listing)"
verdict: REFUTED            # as a blocker — basis breaks-no-clause. The behaviour itself reproduces in every cell.
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exit, stdout and stderr in every cell below"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157); Linux NOT driven"
setup:            # HOME a fresh empty directory; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]                       # in repo; exit 0  (the same state as `fixture: fresh`)
  - control: ["jigc", "doc", "list"]        # exit 0, "jigc doc list — no committed docs"
  - ["mkdir", "-p", "docs/decisions"]
  - ["ln", "-s", "/nonexistent/nowhere.md", "docs/decisions/dangling.md"]
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "list", "--format", "json"]
  - ["jigc", "doc", "list", "adr"]
  - ["jigc", "doc", "list", "vision"]
  - ["jigc", "validate"]
expect:
  - exit: 1
    stdout: ""
    stderr: "reading the committed doc at \"<abs repo>/docs/decisions/dangling.md\": No such file or directory (os error 2)\n"
    stderr_lacks: ["route", "`", "jigc "]   # no route, no command, no finding code
  - exit: 1
    stdout: ""
    stderr_json: { "error": "reading the committed doc at \"<abs repo>/docs/decisions/dangling.md\": No such file or directory (os error 2)" }
  - exit: 1
    stdout: ""
    stderr_contains: "docs/decisions/dangling.md\": No such file or directory (os error 2)"
  - exit: 0
    stdout: "jigc doc list — no committed `vision` docs\n"
  - exit: 0
    stdout_contains: "no findings — the committed store validates clean"
  - every_invocation: "nothing under the root changes, mtime included; `git status --porcelain --untracked-files=all --ignored` the same before and after"
variants:         # each alone, in a fresh root, in place of the `ln`
  - "mkdir CHANGELOG.md                                -> exit 1, stdout empty, `… CHANGELOG.md\": Is a directory (os error 21)`"
  - "mkdir -p docs/research/a-dir.md                   -> exit 1, stdout empty, `… a-dir.md\": Is a directory (os error 21)`"
  - "printf '# x\\n' > docs/ideas/locked.md; chmod 000  -> exit 1, stdout empty, `… locked.md\": Permission denied (os error 13)`"
  - "mkfifo docs/research/pipe.md                      -> no exit: killed by an 8 s alarm, status 142, stdout and stderr empty"
populated-store:  # fixture: refs-post-hoc, then `ln -s nowhere-target docs/research/dangling.md`
  - "jigc doc list                 -> exit 1, stdout empty: none of the five committed rows"
  - "jigc doc list vision          -> exit 0, the `vision` row"
  - "jigc doc list (linked worktree) -> exit 1, the same one line"
  - "after `unlink` of the plant   -> exit 0, byte-identical to the listing before it"
clause: "working-product — NOT broken: (a) every cell behaves the same on the previous release, so no command that works there stops working; (b) the refusal prints no route, so no route fails as printed"
observed: "<W>/a-dangling-cand.62pDUo, <W>/v1-dangling-cand.p9SOg2, <W>/v2-dir-placement-cand.0XvJ1k, <W>/v3-dir-located-cand.hsG5Ol, <W>/v4-mode000-cand.epPEYa, <W>/v5-fifo-cand.RRPaUe, <W>/jigc-rig-refs-post-hoc-smPtHN; the same names with -prev, and <W>/jigc-rig-refs-post-hoc-qj0OPw, for the previous release"
pinned-by: "UNPINNED: verified from the suites at eeffe347 — the message has one hit under crates/ and tooling-tests/, the production line; doc_list.rs holds no unreadable-entry test; the 13 suites that both drive `doc list` and plant such a shape assert the listing's success or another refusal"
```

**Pinnable as it stands: yes, for what the block asserts** — the setup is `fixture: fresh` and two
filesystem calls, the argv are literal, and the expectations are an exit status, an empty stdout
and substrings of one stderr line. Three things a converter needs to know:

1. **It would pin a defect's present shape.** The row stays on the ledger; a fix inverts the first
   three expectations (a row, or a skipped entry, at exit 0). A pin written before the row's
   disposition is a characterization test, and should say so.
2. **The half the refutation rests on is not pinnable as a test of one binary**: *the same on the
   previous release* is a comparison of two. The regression set is the instrument that holds it,
   and no test of the previous release's suite reaches this state (the coverage check above).
3. **Platform edges.** Every shape is Unix-only (`symlink`, `mkfifo`, a mode). The mode-000 cell
   does not refuse for a user who can read any file (a container running as root), and the
   named-pipe cell is a timing test — it needs an alarm, and is the one cell not pinnable as it
   stands. The absolute path in the message is the canonical one (on macOS it carries the
   `/private` prefix of the temp root), so a pin matches on the tail.

## What was driven, and what was not

- **Driven:** one read site of this door, `crates/cli/src/doc.rs:4866` (`std::fs::read(&path)…?`
  over `engine::index::committed_instances`), read in the source to confirm it is the line the
  message comes from; five shapes at it (a dangling link at a located home, a directory at a
  placement home, a directory at a located home, a mode-000 file, a named pipe); an empty store and
  a populated one; the main checkout and a linked worktree; both formats; both binaries.
- **Not enumerated:** the mechanism's other consumers. `instance, unbounded` — no count is given
  here, and none of the three reports' counts was re-derived.
- **Not driven:** Linux; a dangling link at a *placement* home; the staged arm (`--task`); the
  orphan-row read; any other verb over the same state beyond the two the block names and one
  `doc show`.
- **Nothing was fixed, graded or decided.** What happens to the row is triage's and the human's.

## Left open — seen on the way, not pursued

1. **`design/doc-read-surface.md` does not say what an instance that cannot be read does to the
   listing**, and its sentence "A row is emitted for every instance the enumerator yields" is false
   of the driven binary in these states. A doc/binary inconsistency of its own, whichever way the
   defect is settled.
2. **`jigc validate` and `jigc doc list` tell two stories about one entry**: over the same store
   the sweep exits 0 with *the committed store validates clean* while the listing cannot
   enumerate it. It is inside the finding's block as an expectation; it is listed here because the
   door it speaks of, `jigc validate`, is not this round's.
3. **The same section's line citations are stale**: it cites `index.rs:718-752` and `:724-731` for
   `committed_instances`, which stands at `crates/engine/src/index.rs:789-817` at eeffe347.
4. **Linux** — the reconciler's LD-3, still not driven by anyone.

## Where the evidence is

`<W>` is `<scratch>/verify-p1-doc-list-unreadable.79zAdR`. Per root `<W>/<root>`:
`<W>/<root>.runs/<label>.{stdout,stderr,porc.before,porc.after,man.before,man.after}`. The whole
logs: `<W>/variants-cand.log`, `<W>/variants-prev.log`, `<W>/rig-cand.log`, `<W>/rig-prev.log`, and
their normalised forms (`.norm`) that the two `diff`s read. The tools:
`<W>/tools/{env.sh,snap.py,mkfresh.sh,drive.sh,variants.sh,rigcells.sh}`. Nothing was torn down,
and the one file written for the repository is this report.

<!-- end of report -->
