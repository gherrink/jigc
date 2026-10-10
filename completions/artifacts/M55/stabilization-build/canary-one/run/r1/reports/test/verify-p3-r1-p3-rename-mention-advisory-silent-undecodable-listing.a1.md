# verify-real — `r1-p3-rename-mention-advisory-silent-undecodable-listing`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: door
`jigc rename`; the clause it is said to break, `working-product`; triage's grade, *unclear*; its
block, the first cell under `Observed, and not to be pinned green` in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-git-capture-other-callers-not-enumerated.a1.md`.
What triage asked to have re-driven: *a tracked file with a non-ASCII name (which git C-quotes
by default) holding a mention of the slug, default git configuration: does `jigc rename` report
the mention; both binaries.*

## Verdict in one paragraph

**`refuted` — as a blocker, with the basis `breaks-no-clause`. It is not `does-not-reproduce`:
the defect is real, it reproduces as written, and it reproduces under default git
configuration with nothing planted.** In a repository whose only unusual feature is one tracked
text file named with a non-ASCII letter, `jigc rename` exits 0, lands its commit, and its
advisory list of surviving mentions leaves that file out — the file holds the old slug before
and after, and the same file is listed the moment `core.quotePath` is `false`. The handed cell
(a name holding the byte 0xFF, `core.quotePath=false`) loses the whole advisory, the ASCII
file's line included. **The candidate and the previous release give the same exit status, the
same commit and byte-identical standard output in every one of 10 cells**, so no command that
works on the previous release stops working, and the door refuses nothing, so no route is
printed that could fail. That is the measure the closing condition gives its second clause. The
row stays a row of the ledger.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`,
  label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's. It was driven through the same script and the same cells, because triage
  asked for both binaries.
- The candidate's directory went first on `PATH` and `command -v jigc` printed the candidate's
  path. The driver repeats that check in every cell with the binary it was handed and stops if
  it fails; for the previous release's cells that release's directory is first, inside that
  cell's process only. Every rig was built with `dev/jigc-rig vendored --binary <that binary>`
  and the driver stops unless the rig's `$JIGC` is that path and `$REPO` is under my own
  scratch directory. No cell stopped: both logs hold zero `HALT` lines.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was read first

- The clause: `DECISIONS.md` → *2026-10-04 — The exit rule, revised*. Second clause, in the
  human's words: *We have a working product others can use and relay on.* Its instrument, the
  second sharpening: *no command that works on rc.24 in a supported layout stops working, and
  every refusal's route works as printed.* `implementation/stabilization-workflow.md` → *The
  regression set* restates it: *no command that works on the previous release stops working.*
  The run's opening names the clause `working-product`.
- The design that owns the behaviour: `design/write-commands.md` → `jigc rename` → *Boundary*:
  the CLI *reports … old-slug occurrences it finds in managed-doc prose and unmanaged files …
  scoped to tracked worktree files (respecting `.gitignore`), and is advisory (it never blocks
  and never rewrites)*. The build decision, `DECISIONS.md` → *2026-06-28 — M35 Inc-1 T5
  landed*: *Best-effort + advisory: a `git`/read failure or a binary file is skipped, never an
  error — the report never changes the verb's exit status.*
- The source at the site, `crates/cli/src/rename.rs`, `scan_prose_mentions` (line 1302): it
  captures `git ls-files` (no `-z`), returns an empty list if the capture is an error, and
  otherwise joins **each line as printed** onto the repository root and reads it, skipping a
  file it cannot read. The render arm, `crates/cli/src/render.rs:5153`, prints the advisory
  only when the list is not empty.
- I read the one report the prompt names, and no other finding's or verifier's.

## What was driven

10 cells per binary, each on a fresh `vendored` rig of its own under
`<scratch>/verify-rn.Y2RGcT/runs/<binary>/<cell>/`; every command whose exit status is read ran
bare, its standard output and standard error each to a file of its own. The door, in every
cell:

    jigc rename spec:padding --to "Padding Helper Two"        (two cells add --format json)

Every cell first writes `NOTES.txt` = `see padding for the helper` and commits it with a plain
`git commit --no-verify` — the handed block's own first step, and the ASCII control inside
every cell. The rig's arch-doc `docs/architecture/padding-layer.md` mentions the slug on line 9
as built.

The states:

- **none** — nothing more.
- **utf8** — *triage's question.* A file named `n\303\266tes.txt` (the letter o-umlaut,
  precomposed, UTF-8 bytes C3 B6), holding the same line, written with `printf`, `git add`ed
  and committed in the same commit as `NOTES.txt`. Default git configuration.
- **utf8q** — the same, then `git config core.quotePath false`.
- **utf8json**, **utf8qjson** — those two, with `--format json` at the door.
- **space** — the same line in a file named `my notes.txt`: a name that is unusual and that git
  does not quote.
- **sw-ffq** — *the handed block, as written*: an index entry `bad<byte 0xFF>name.txt` planted
  with `git update-index --add --cacheinfo`, committed with `git commit -q --no-verify -m
  foreign`, marked `git update-index --skip-worktree`, then `git config core.quotePath false`.
- **sw-ff**, **sw-asciiq**, **sw-ascii** — its three controls: the 0xFF name under default
  quoting, and the name `badXname.txt` with and without the knob.

**What I changed from the block as handed.** Nothing in `sw-ffq`: it ran as written. The four
`utf8` cells and `space` are mine, built to answer triage's question, which the handed report
states as *read and not driven*. They need no plumbing, no flag and no configuration.

**Read back in every cell, before the door.** `git config --show-origin --get core.quotePath`
exits 1 with no output in every cell that does not set it (the rig repoints `HOME`, and this
host has no system git configuration file), and names the repository's own configuration with
`false` in the cells that do. `git status --porcelain` is empty. A `command grep -r -l -w
padding` over the tree — the before-control that finds the plant — names `NOTES.txt`,
`docs/architecture/padding-layer.md` and, in the `utf8` cells, the o-umlaut file; the same grep
after the door names the same files, so nothing was rewritten and every mention still stands.
What `git ls-files` prints for the o-umlaut file:

| configuration | the line `git ls-files` prints | valid UTF-8 |
|---|---|---|
| default | `"n\303\266tes.txt"` — 18 ASCII bytes, the quotes and backslashes literal | yes |
| `core.quotePath=false` | the name itself, bytes C3 B6 raw | yes |

and for the 0xFF entry: `"bad\377name.txt"` under default quoting (valid UTF-8), the raw byte
under `core.quotePath=false` (the listing is **not** valid UTF-8).

### The cells

Both binaries, every row: exit status, whether `HEAD` moved, the landed commit and the advisory
are the same on the candidate and on the previous release.

| cell | exit | `HEAD` | landed commit | the advisory on standard output | standard error |
|---|---|---|---|---|---|
| none | 0 | moved | the one rename | `2 prose/unmanaged mention(s)`: `NOTES.txt:1`, `docs/architecture/padding-layer.md:9` | empty |
| **utf8** | **0** | moved | the one rename | **`2 prose/unmanaged mention(s)`: the same two lines — the o-umlaut file is not listed** | empty |
| utf8q | 0 | moved | the one rename | `3 prose/unmanaged mention(s)`: those two and `n\303\266tes.txt:1` (the name printed raw) | empty |
| **utf8json** | **0** | moved | the one rename | `"prose_mentions": ["NOTES.txt:1", "docs/architecture/padding-layer.md:9"]` | empty |
| utf8qjson | 0 | moved | the one rename | `"prose_mentions"` holds three entries, the o-umlaut file's the third | empty |
| space | 0 | moved | the one rename | `3 prose/unmanaged mention(s)`: those two and `my notes.txt:1` | empty |
| sw-ascii | 0 | moved | the one rename | `2 prose/unmanaged mention(s)`: `NOTES.txt:1`, `docs/architecture/padding-layer.md:9` | empty |
| sw-ff | 0 | moved | the one rename | the same two lines | empty |
| sw-asciiq | 0 | moved | the one rename | the same two lines | empty |
| **sw-ffq** | **0** | moved | the one rename | **no advisory line at all** — standard output is the `renamed …` line and the footer | empty |

*The one rename* is `R094 docs/specs/padding.md -> docs/specs/padding-helper-two.md`, and `git
status --porcelain` is empty after the door in all 20 runs.

### Candidate against previous release

Standard output of the door, with the short commit id in the JSON form normalised, hashes the
same on the two binaries in all 10 cells; so does `git show --name-status --format=%s HEAD`.
Standard error is empty in all 20 runs. No cell is green on one binary and red on the other.

## Is it what the finding says?

Yes, and under the conditions triage named it is wider than the handed cell.

- **The handed cell reproduces as written** (`sw-ffq`, both binaries): exit 0, the commit
  lands, and no advisory is printed although two tracked, readable ASCII files hold the old
  slug — the control `sw-asciiq` lists both. The capture of `git ls-files` cannot be decoded,
  the scan returns an empty list, and the render arm prints nothing for an empty list, so
  *could not list* and *nothing found* read alike.
- **Triage's question: no, it does not report the mention** (`utf8` and `utf8json`, both
  binaries). git exited 0 and its listing decoded; the line for the file is its C-quoted
  spelling, the scan joins that spelling onto the repository root as a path, no such path
  exists, and the file is skipped. The count the advisory states is 2 where 3 mentions stand.
  The cause is the quoting and nothing else: with `core.quotePath=false` the same file in the
  same commit is listed (`utf8q`), and a name git does not quote is listed under default
  configuration (`space`).

**Read against the design that owns it.** The report is *scoped to tracked worktree files*, and
the o-umlaut file is a tracked worktree file, readable and text. The build decision's
best-effort sentence covers a `git` failure, a read failure and a binary file; here git did not
fail and the file can be read — the read fails only because the door spells its path as git's
display form. That is not what the sentence describes, so the omission is a defect and not
behaviour a settled decision intends. The repository states the trap itself at a sibling door:
`design/project-setup.md` (the ingest candidate set) uses `git ls-files -z` *so a non-ASCII
filename is never `core.quotepath`-C-quoted into a bogus literal path*. In the handed cell the
capture is an error by the code's own classing, and the best-effort sentence can be read to
reach it; the lost lines there are those of files that were listed and readable, and I read
that as outside the sentence too, as the handed report did. Nothing here is contested: the
finding does not argue that a decision is wrong, and I found no decision that intends either
outcome.

## Does it break `working-product`, inside that clause's scope?

No. **Basis: `breaks-no-clause`.**

- **The clause's measure is comparative, and its two halves are both untouched.** *No command
  that works on rc.24 in a supported layout stops working*: in every cell `jigc rename` does on
  the candidate exactly what it does on the previous release — same exit status, same commit,
  byte-identical output. *Every refusal's route works as printed*: the door refuses nothing in
  any cell and prints no route.
- **This holds without leaning on the scope's edge.** I did not need to decide whether a
  tracked file with a non-ASCII name is a *supported layout* — I take it to be one, since the
  `utf8` cells are an ordinary file under default configuration — nor whether the handed cell's
  plumbing entry and `skip-worktree` flag are a planted state. Under the reading least
  favourable to the binary the two binaries still agree.
- **It is not the first clause's either, and I do not grade that**: the commit is the control's
  and no byte is destroyed; the handed report already found as much for this site.

**What this verdict rests on, said plainly.** I read the second clause by its instrument, as
the entry of 2026-10-04 and the stabilization workflow state it. Read by its plain words alone
— *a working product others can use and rely on* — a reader who relies on the advisory's count
is told 2 where 3 stand, at exit 0, under default configuration, and the previous release tells
him the same. Whether the clause is meant to reach a defect the previous release already has is
the rule's to say and the human's to rule, never this verdict's; if it is ruled to, this row is
the `utf8` cell below with its expectation inverted.

## The regression fact

**Not owed by this verdict**: step 4 runs with `confirmed` only. Both binaries were driven
through every cell because triage asked for it, and the result is above: the previous release
shows the same omission in the same cells, so had this been confirmed it would not have been a
regression.

## Class

**`instance, unbounded`.** Driven: one door, one corpus, one slug; one non-ASCII name (a
precomposed two-byte letter, at the repository root), one name with a space, one plumbing
entry holding the byte 0xFF. I enumerated neither the names git quotes nor the other consumers
of a line-oriented git listing, and give no count for either. The handed report's own table
classes 14 call sites of `task::git_capture` as line-oriented path listings; that count is its,
and I did not re-derive it.

## Coverage

The finding makes no coverage claim, and none is verified here. One fact read on the way, with
its bound: the advisory is named in two suite files, `crates/cli/tests/flow37_rename.rs` and
`crates/cli/tests/text_json_parity_axis.rs` (a grep for the function, the field and the printed
phrase over `crates`, `tooling-tests` and `dev`); a grep of those two files for `quotepath`,
`unicode` and `non-ascii`, in any case, finds nothing. That is a grep of two files, not a
classification of what the suites cover.

## Repro VR-RN-1

```yaml
claim: "jigc rename's advisory silently omits surviving mentions of the old slug — all of them when the listing cannot be decoded, and under default git configuration every one inside a tracked file whose name git C-quotes — and that breaks working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The omission reproduces on both binaries; the candidate does exactly what the previous release does, and the door refuses nothing."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; the previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, gives the same exit, commit and standard output in every cell"
cells:
  - cell: utf8                          # triage's question; rename.rs:1302, the per-line read
    setup:
      - fixture: vendored               # dev/jigc-rig vendored --binary <binary>
      - "write NOTES.txt = `see padding for the helper`"
      - "write a file named n<bytes C3 B6>tes.txt (o-umlaut, precomposed) = the same line"
      - ["git", "add", "--", "NOTES.txt", "n<bytes C3 B6>tes.txt"]
      - ["git", "commit", "-q", "--no-verify", "-m", "notes that mention the slug"]
      - "assert: git config --get core.quotePath exits 1; git status --porcelain is empty"
    repro:
      - ["jigc", "rename", "spec:padding", "--to", "Padding Helper Two"]
    observed:
      exit: 0
      stdout_contains: "2 prose/unmanaged mention(s) of `spec:padding` remain"
      stdout_lists: ["NOTES.txt:1", "docs/architecture/padding-layer.md:9"]
      stdout_lacks: "tes.txt:1"         # besides NOTES.txt:1 — the o-umlaut file's line
      stderr: ""
      landed: "R094 docs/specs/padding.md -> docs/specs/padding-helper-two.md, and nothing else"
    json_form: "with --format json: exit 0 and prose_mentions holds exactly those two entries"
    a_fix_would_expect: "3 mentions, the third the o-umlaut file's, line 1"
  - cell: utf8q                         # the control that isolates the quoting
    setup:
      - "as utf8, then:"
      - ["git", "config", "core.quotePath", "false"]
    repro:
      - ["jigc", "rename", "spec:padding", "--to", "Padding Helper Two"]
    expect:
      exit: 0
      stdout_contains: "3 prose/unmanaged mention(s) of `spec:padding` remain"
      stdout_lists: ["NOTES.txt:1", "docs/architecture/padding-layer.md:9", "n<bytes C3 B6>tes.txt:1"]
  - cell: space                         # an unusual name git does not quote
    setup:
      - fixture: vendored
      - "write NOTES.txt and `my notes.txt` = `see padding for the helper`; git add both; git commit -q --no-verify"
    repro:
      - ["jigc", "rename", "spec:padding", "--to", "Padding Helper Two"]
    expect:
      exit: 0
      stdout_contains: "3 prose/unmanaged mention(s) of `spec:padding` remain"
      stdout_lists: ["NOTES.txt:1", "docs/architecture/padding-layer.md:9", "my notes.txt:1"]
  - cell: sw-ffq                        # the handed block, as written; rename.rs:1303, the swallowed capture error
    setup:
      - fixture: vendored
      - "write NOTES.txt = `see padding for the helper`; git add; git commit -q --no-verify"
      - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
      - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]
      - ["git", "commit", "-q", "--no-verify", "-m", "foreign"]
      - ["git", "update-index", "--skip-worktree", "--", "bad<byte 0xFF>name.txt"]
      - ["git", "config", "core.quotePath", "false"]
    repro:
      - ["jigc", "rename", "spec:padding", "--to", "Padding Helper Two"]
    observed:
      exit: 0
      stdout_lacks: "prose/unmanaged mention(s)"
      stderr: ""
    control: "the same with badXname.txt: exit 0 and `2 prose/unmanaged mention(s)`, naming NOTES.txt:1 and docs/architecture/padding-layer.md:9"
observed-at: "<scratch>/verify-rn.Y2RGcT — cand.log and prev.log (every command and its exit, the door's two streams), runs/<binary>/<cell>/"
pinned-by: "UNPINNED: a verifier writes no test — and the two `observed` cells are not to be pinned green (below)"
```

**Pinnable as it stands: no.** The cells `utf8` and `sw-ffq` record a defect as observed;
pinned green they would hold it in place, and the fact that refutes the blocker — two binaries
agreeing — is not a fact one binary's suite can state. What converts as it stands: `utf8q` and
`space`, green today and wanted green. What converts with its expectation inverted, as a fix's
red test if one is scheduled: `utf8` (the line under `a_fix_would_expect`). It needs no
plumbing and no configuration, only a filesystem that holds a UTF-8 name; a test would write
the name precomposed, since this host's git sets `core.precomposeunicode` in the repository it
initialises. `sw-ffq` needs the name passed as raw bytes and is Unix-only.

## Left open

1. **Names git quotes under every configuration.** A name holding a double quote, a backslash
   or a control character is printed quoted by a line-oriented listing whatever
   `core.quotePath` says, so by reading the same scan skips such a file under any
   configuration. Read, not driven.
2. **A non-ASCII directory component, and a decomposed name.** `docs/<a non-ASCII
   directory>/x.md` reaches the same line-as-path read by reading; a name stored decomposed was
   not tried. Not driven.
3. **An empty advisory prints nothing.** Text form: no line at all when the list is empty, so a
   listing that could not be decoded and a tree with no mention read alike (`sw-ffq`). The JSON
   form of that cell was not driven.
4. **The other line-oriented listings.** The handed report's 14 class-C sites take path names
   from git output without `-z`; under default quoting each receives a C-quoted spelling for
   such a name. Which of them use the line as a path was not traced here. Another row's, if it
   is anyone's.

## Bounds — what this verification did not do

- One macOS host, one git, one filesystem. This filesystem holds the o-umlaut name and refuses
  the 0xFF one, so the `sw-` cells are an index entry with no file, as in the handed report.
- One corpus (`vendored`), one slug, one mention per file, one non-ASCII name. `--format json`
  was driven in the two `utf8` cells only.
- The notes file is committed with `--no-verify`, as the handed block does it; a commit through
  the rig's installed hook was not tried.
- It read the two `DECISIONS.md` entries named above, `design/write-commands.md` → `jigc
  rename`, one sentence of `design/project-setup.md`, `implementation/stabilization-workflow.md`
  → *The regression set*, and the source at the scan and its render arm.

## Where the evidence is

- `<scratch>/verify-rn.Y2RGcT/scripts/drive.sh` — one cell: the `PATH` check, the rig, the
  plant, the read-backs, the door, the read-backs again.
- `<scratch>/verify-rn.Y2RGcT/cand.log`, `prev.log` — every command with its exit status, and
  the door's standard output and standard error, in order.
- `<scratch>/verify-rn.Y2RGcT/runs/cand/<cell>/`, `runs/prev/<cell>/` — a file per stream per
  step (`door.out`, `door.err`, `ls-files-plain.out`, `ls-files-z.out`, `landed.out`, the two
  grep controls), and the rig itself.

<!-- end of report -->
