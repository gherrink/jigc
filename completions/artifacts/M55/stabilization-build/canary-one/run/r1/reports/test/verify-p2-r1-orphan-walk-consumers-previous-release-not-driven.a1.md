# verify-real — `r1-orphan-walk-consumers-previous-release-not-driven`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier:
the three other consumers of the orphan walk (`jigc config set docs-root`,
`jigc config set placement-root`, `jigc relocate`) were driven on the candidate only, and how
they behave on the previous release was not known. Door: `jigc config set` and
`jigc relocate`. Clause it is said to break: `working-product`. Triage's grade: *unclear*.
Its block: `Repro VR-LD-2` and `The regression fact` of
`completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-non-utf8-path-other-orphan-walk-consumers.a1.md`.

## Verdict in one paragraph

**`refuted` — as a blocker, with the basis `breaks-no-clause`. It is not
`does-not-reproduce`: the behaviour is real, and it is the same on both binaries.** I drove
`Repro VR-LD-2` from nothing — thirteen cells, each on its own fresh rig built with the
binary under test — once on the candidate and once on the previous release. **Every exit
status, every standard output and every standard error is identical between the two, cell
for cell**, and the index after each door holds the same doc names in the same places. With
one index entry whose name holds the byte 0xFF, the previous release's three doors do what
the candidate's do: they exit 0, move nothing, say nothing about it, and the two
`config set` doors still land the knob. The second clause's instrument is *no command that
works on the previous release stops working, and every refusal's route works as printed*.
Nothing that worked on `1.0.0-rc.24` in these cells stopped working on the candidate, and
none of the three doors prints a refusal in any cell. **The regression fact for the three
doors: not a regression — red on the previous release, red on the candidate, in the 0xFF
state; green on both in the other two states.** The defect stays a row of the ledger.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` —
  the hash the prompt gives for the candidate (commit
  `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` —
  the hash the prompt gives for the previous release (`1.0.0-rc.24`).
- The candidate's directory went first on `PATH` and `command -v jigc` printed the
  candidate's path, before the first command. For the second run the previous release's
  directory went first and `command -v jigc` printed its path. The driver repeats the check
  at its start and again inside every rig, after the rig's `eval`, and stops if it fails;
  it also stops unless the rig's `$JIGC` is the binary it was handed. Neither stop fired in
  either run.
- Every rig was built with `dev/jigc-rig <state> --binary <the binary under test>`.
- **An independent sign that two different binaries ran:** every `refs-post-hoc` rig the
  candidate built tracks one path more than the matching rig the previous release built —
  `.jigc/settings-entries.json`, the file the candidate's `jigc setup` commits and the
  previous release's does not. It is the one difference between the two runs (below).
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was driven

One driver, written for this verification, run twice — the same script, the binary its one
varying argument. Every command whose exit status is read ran bare; its standard output, its
standard error and its exit status each went to a file of their own. Nothing of the
reporter's rigs was reused: every rig is minted under
`<scratch>/verify-p2-orphan-prev.q7QTJc/runs/`.

The three states, per door, as the block has them:

- **none** — the rig as built.
- **ascii** — the plant under the name `badXname.txt`.
- **ff** — the plant under `bad<byte 0xFF>name.txt`.

The plant, in each rig, after the door's setup:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,<name>"        # exit 0, both binaries' rigs

After the `ff` plant `git ls-files -z` exits 0 and its output holds the byte 0xFF and is not
valid UTF-8, read back in every `ff` cell of both runs; after the `ascii` plant it is valid.

The corpora are the block's: rig state `refs-post-hoc` for the two `config set` doors; for
`jigc relocate` the rig state `bare` plus the fixture of `crates/cli/tests/relocate.rs` as
the block spells it (a manifest-less `note` doctype named in `.jigc/config/packs.yaml`, one
committed note at `docs/legacy-notes/cache-benchmarks.md`).

**What I changed from the block as written: nothing in any command.** I added two cells so
that `jigc relocate` has its `--format json` form in the control state as well as in the
`ff` state (`R0j`, `R2j`), where the block's own log ran the JSON form in the `ff` state
only; and each `config set` cell runs the block's read-backs (`config get`, `git ls-files`
on the doc's two homes, `doc show`, `doc list`, `validate`) in the same rig after the door.

### The comparison, as a count

Each run left 411 files beside its 13 rigs, the same 398 names in both and 13 rig
construction scripts whose names are minted. The 398, compared file by file, with the rig's
own root path replaced by one token and a 40-character commit id by another:

- **320 stream and exit files — every `.out`, `.err` and `.rc` of every step of every
  cell: all 320 identical.**
- **26 index summaries** (after the plant and after the door, per cell): the 10 of the
  five `bare` cells are identical; **the 16 of the eight `refs-post-hoc` cells differ in
  one line each** — the first, which counts one entry more on the candidate (15 against
  14, or 16 against 15 with a plant). The lists of `.md` names below that line are
  identical in all 26.
- 26 raw index listings, compared as bytes: the same 10 identical, the same 16 differing
  by 28 bytes each — the one name `.jigc/settings-entries.json` and its terminator.
- 26 files are not compared: 13 rig construction logs and 13 rig root paths, which name
  their own directories.

The commit id is normalised because the two rigs' commits differ (one more tracked file);
within each cell `git rev-parse HEAD` printed the same id before the door and after the
read-backs, on both binaries, in all five `docs-root` cells.

### Door 1 — `jigc config set docs-root notes`

| cell | state | previous release: exit · stdout · stderr | candidate | index afterwards, both |
|---|---|---|---|---|
| D0 | none | 0 · `config: set docs-root = notes — written to .jigc/config/, uncommitted …` · `relocating 1 committed doc(s) stranded by the docs-root re-point …` and `- docs/research/context-loss.md → notes/research/context-loss.md` | the same three | `R docs/research/context-loss.md -> notes/research/context-loss.md` |
| D1 | ascii | 0 · the same line · the same two lines | the same | the same rename, beside `AD badXname.txt` |
| **D2** | **ff** | **0 · the same line · empty** | **the same** | **no rename; the doc still at `docs/research/context-loss.md`** |
| D0j | none, `--format json` | 0 · `"relocated": [ { "from": "docs/research/context-loss.md", "to": "notes/research/context-loss.md" } ]` · empty | the same | the rename |
| **D2j** | **ff, `--format json`** | **0 · `"relocated": []` · empty** | **the same** | **no rename** |

In jigc's quoted lines the inner backticks are dropped; the files hold them verbatim. After
the door, in the same rig, on **both** binaries alike:

| next command | D0 · D1 | D2 |
|---|---|---|
| `jigc config get docs-root` | exit 0, `docs-root = notes  (project)` | exit 0, the same |
| `git ls-files -- docs/research/context-loss.md notes/research/context-loss.md` | exit 0, `notes/research/context-loss.md` | exit 0, `docs/research/context-loss.md` |
| `jigc doc show research:context-loss` | exit 0, the doc | **exit 1**, `blocking · store.not-found — could not read research:context-loss at notes/research/context-loss.md: No such file or directory (os error 2)` |
| `jigc doc list` | exit 0, five rows | exit 0, **four rows — the research doc has none** |
| `jigc validate` | exit 0, one advisory (an empty repeatable section) | exit 0, the same one advisory, nothing about the stranded doc |

### Door 2 — `jigc config set placement-root notes`

| cell | state | previous release: exit · stdout · stderr | candidate | index afterwards, both |
|---|---|---|---|---|
| P0 | none | 0 · `config: set placement-root = notes — written to .jigc/config/, uncommitted …` · `relocating the committed doc(s) stranded by the placement-root re-point …`, `- docs/decisions-log.md → notes/decisions-log.md`, `- docs/roadmap.md → notes/roadmap.md` | the same | both renames staged |
| P1 | ascii | 0 · the same line · the same three lines | the same | both renames, beside `AD badXname.txt` |
| **P2** | **ff** | **0 · the same line · empty** | **the same** | **no rename; both docs still under `docs/`** |

After the door, on both binaries alike: `jigc config get placement-root` exits 0 with
`placement-root = notes  (project)` in all three cells. In P2 only,
`git ls-files -- docs/roadmap.md docs/decisions-log.md notes/` prints the two `docs/` names,
`jigc validate` exits 0 printing `no findings — the committed store validates clean`,
`jigc doc list` exits 0 with three rows and `jigc doc show roadmap` exits 1 with
`store.not-found … at notes/roadmap.md`. In P0 and P1 the listing has five rows, the read
exits 0 and the sweep prints its one advisory.

### Door 3 — `jigc relocate note --from docs/legacy-notes/`

| cell | state | previous release: exit · stdout | candidate | index afterwards, both |
|---|---|---|---|---|
| R0 | none | 0 · `freeze-exempt relocation: 1 moved, 0 displaced, 0 blocked` and `moved  docs/legacy-notes/cache-benchmarks.md -> docs/notes/cache-benchmarks.md` | the same | the rename staged |
| R1 | ascii | 0 · the same two lines | the same | the rename, beside `AD badXname.txt` |
| **R2** | **ff** | **0 · `freeze-exempt relocation: 0 moved, 0 displaced, 0 blocked`** | **the same** | **no rename; `docs/legacy-notes/cache-benchmarks.md`** |
| R0j | none, `--format json` | 0 · `"moved": [ [ "docs/legacy-notes/cache-benchmarks.md", "docs/notes/cache-benchmarks.md" ] ]`, `"blocked": []`, `"displaced": []` | the same | the rename staged |
| **R2j** | **ff, `--format json`** | **0 · `"moved": []`, `"blocked": []`, `"displaced": []`** | **the same** | **no rename** |

Standard error is empty in all five cells on both binaries.

## Is it what the finding says?

The finding says one thing: the previous release was not driven. It now is, and the answer
to what it left open is that **the two binaries do not differ at these doors in any of the
three states.**

The source agrees with the cells, read at both commits with `git grep` over
`crates/cli/src`: `orphan::committed_markdown` has the same body at `91834b5e` and at
`eeffe347` — `git ls-files -z` through `task::git_capture`, and any error turned into an
empty list — and the same five call sites (`orphan.rs:222`, `orphan.rs:616`,
`orphan.rs:1116` in both; `config.rs:1482` and `relocate.rs:630` at the previous release,
`config.rs:1512` and `relocate.rs:708` at the candidate). The behaviour was not introduced
between the two commits; it was in the previous release as published.

Whether it is a defect against the design is not reopened here: the report this block comes
from read `design/storage.md` → Placement against it and found no ruling that intends it. I
read the same two paragraphs (*A re-point moves what it would strand*; *What a root knob
refuses, and why the refusal is at the door*) and they say what that report quotes: a
landed knob with no move is named there as a state a root knob must not produce. Nothing in
this finding argues that a settled decision is wrong, so nothing is contested.

## Does it break `working-product`, inside that clause's scope?

No. **Basis: `breaks-no-clause`.**

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is, in the human's words, *a working product others can use and rely on*;
  its one instrument, under the second sharpening, is *no command that works on rc.24 in a
  supported layout stops working, and every refusal's route works as printed*. The run's
  opening names the clause `working-product` and gives it the regression set and the gate.
  `implementation/stabilization-workflow.md` → The regression set says the same in one
  sentence: *no command that works on the previous release stops working*.
- **Nothing that works on the previous release stops working.** In the `none` and `ascii`
  states all three doors work on `1.0.0-rc.24` and work identically on the candidate. In
  the `ff` state none of the three works on `1.0.0-rc.24` — and the candidate does exactly
  what it did. There is no cell that is green on the previous release and red on the
  candidate.
- **No refusal is printed, so no route is in play at these doors.** Each door exits 0 in
  every cell on both binaries and prints no refusal. The one refusal met on the way —
  `jigc doc show` exiting 1 with `store.not-found` after a `config set` door in the `ff`
  state — belongs to another door, is byte-identical on both binaries, and is left open
  below.
- **Whether a tracked name that is not UTF-8 is a *supported layout*** did not need
  settling: the comparison comes out the same whichever way that is read.

**What this verdict does not say.** It does not say the behaviour breaks no clause of the
rule: the report this block comes from left the reading against `no-lost-files` open, and
this verifier was not sent to grade it. It says the second clause is not what it breaks.

## The regression fact

Step 4 runs with `confirmed` only, so the structured return carries no `regression` field.
**But the fact triage asked for is established, because establishing it was the re-drive:**
the same block, on fresh rigs, with the previous release's binary by its absolute path
(`<scratch>/bin/previous-91834b5e011d/jigc`, hash asserted above). The door exists there
and the block runs there as written, so the two are comparable. **Red on the previous
release and red on the candidate in the `ff` state; green on both in the `none` and `ascii`
states. Were this row ever confirmed against another clause, its regression fact for these
three doors is `false`.**

## Class

**Driven: three doors, three states each, plus the JSON forms the block names — thirteen
cells per binary, twenty-six in all. `instance, unbounded` beyond them.** The consumers of
`committed_markdown` are enumerated at both commits by the grep above: five call sites
each, the same five. Of every other caller of `task::git_capture` I enumerated nothing and
give no count; nothing here says how any of them behaves on either binary.

## Coverage

The block I was handed says `UNPINNED`. I checked that against the suites at **both**
commits, with `git grep` over `crates/cli/tests`, `tooling-tests` and `crates/engine/tests`
for a 0xFF byte, a raw-bytes name, a name that is not UTF-8, or the `--cacheinfo` plumbing:

- at `eeffe347`, twelve test files hit; three of them name a root knob or `relocate`
  anywhere (`subtask_discard_record`, `uninstall_workbench_subject`,
  `version_stamp_rollback`), and at their hits none plants a tracked *name* that is not
  UTF-8: the first plants such bytes as a file's content, the second passes them as an
  argument to `jigc uninstall`, the third uses `--cacheinfo` under an ordinary name;
- at `91834b5e`, eight test files hit; one names a knob (`subtask_discard_record`, the same
  content plant).

So no standing test holds this behaviour at either commit, in either direction — which is
also why the regression set's first part, the previous release's own suite run against the
candidate's binary, could not have shown these doors either way.

## Repro VR-P2-ORPHAN-PREV

```yaml
claim: "the three other consumers of orphan::committed_markdown were not driven on the previous release — and what they do there differs from the candidate, which breaks the clause working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The behaviour reproduces on both binaries and is identical cell for cell; no command that works on the previous release stops working."
binaries:
  candidate: "c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a"
  previous: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, commit 91834b5e011de2c36e2be2b79e96c0b9f60a803c"
run: "the whole block once per binary, every rig built with that binary; the expectations below hold for BOTH"
plant:                      # after each door's setup; needs no file on disk
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
doors:
  - door: "jigc config set docs-root"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <the binary under test>
      - plant
    repro:
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]
      - ["jigc", "config", "get", "docs-root"]
      - ["git", "ls-files", "--", "docs/research/context-loss.md", "notes/research/context-loss.md"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 0
        stdout_json: { "committed": false, "key": "docs-root", "op": "config-set", "relocated": [], "value": "notes" }
        stderr: ""
      - exit: 0
        stdout: "docs-root = notes  (project)"
      - exit: 0
        stdout: "docs/research/context-loss.md"
      - exit: 1
        stderr_contains: "store.not-found"
    control: "without the plant, or with it under the name badXname.txt, on both binaries: relocated holds one pair, docs/research/context-loss.md -> notes/research/context-loss.md, and the read exits 0"
  - door: "jigc config set placement-root"
    setup:
      - fixture: refs-post-hoc
      - plant
    repro:
      - ["jigc", "config", "set", "placement-root", "notes"]
      - ["git", "ls-files", "--", "docs/roadmap.md", "docs/decisions-log.md", "notes/"]
      - ["jigc", "validate"]
    expect:
      - exit: 0
        stdout_contains: "config: set `placement-root` = `notes`"
        stderr: ""
      - exit: 0
        stdout: "docs/decisions-log.md\ndocs/roadmap.md"
      - exit: 0
        stdout_contains: "no findings — the committed store validates clean"
    control: "without the plant, or with it under the name badXname.txt, on both binaries: both docs are staged renames into notes/"
  - door: "jigc relocate"
    setup:
      - fixture: bare                    # dev/jigc-rig bare --binary <the binary under test>
      - "the fixture of crates/cli/tests/relocate.rs: .jigc/note-pack/schemas/note.yaml = `type: note / location: notes/ / id-from: title / sections: [ { id: body, slot: { hint: \"The note.\" } } ]`; .jigc/config/packs.yaml = `packs: [ <repo>/.jigc/note-pack ]`"
      - "write docs/legacy-notes/cache-benchmarks.md = `# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n`"
      - ["git", "add", "docs/legacy-notes/cache-benchmarks.md"]
      - ["git", "commit", "-q", "--no-verify", "-m", "strand the note"]
      - plant
    repro:
      - ["jigc", "relocate", "note", "--from", "docs/legacy-notes/", "--format", "json"]
      - ["git", "ls-files", "--", "docs/"]
    expect:
      - exit: 0
        stdout_json: { "moved": [], "blocked": [], "displaced": [] }
        stderr: ""
      - exit: 0
        stdout: "docs/legacy-notes/cache-benchmarks.md"
    control: "without the plant, or with it under the name badXname.txt, on both binaries: `1 moved`, the rename staged"
equality:                   # the half the verdict rests on
  claim: "for every step of every cell, the exit status, standard output and standard error of the previous release equal the candidate's, the rig's root path and commit ids aside"
  observed: "320 of 320 stream and exit files identical; the one difference between the runs is the candidate's extra tracked path .jigc/settings-entries.json in the refs-post-hoc rigs"
observed: "<scratch>/verify-p2-orphan-prev.q7QTJc — scripts/drive.sh (the driver), cand.log and prev.log (every command, its exit, its two streams, in order), runs/cand/ and runs/prev/ (one file per stream and per exit status per step, and the rigs)"
pinned-by: "UNPINNED: the door cells hold a defect as observed, on both binaries — see Pinnable"
```

**Pinnable as it stands: no.** Mechanically it converts, on Unix only — the plant is two git
plumbing calls with the name passed as raw bytes, and needs no file on disk. But a standing
test can hold one binary, the one it is built with; the half this verdict rests on is an
equality between two binaries, which is a fact for the regression instrument and not for a
suite. And pinned green on the candidate alone, the three `ff` cells would hold in place, as
expected output, a root knob landing over docs it left behind — the state the owning design
names as one a root knob must not produce. They are a fix's red test in waiting, each
`expect` inverted, if this row is ever scheduled.

## Left open

1. **The same behaviour, read against the first clause (`no-lost-files`)** — the reading
   the report this block comes from left open, first in its list. Not graded here. The one
   fact this verification adds to it: whatever that reading comes to, the behaviour is in
   the previous release as published, unchanged.
2. **`jigc doc show` on the stranded doc exits 1 with `store.not-found` and the route
   *create the referenced doc, or fix the reference to an existing one …*** — the doc
   exists, at its prior home, and the route names neither. It is a refusal with a printed
   route, which is the second half of this clause's instrument, at a door that is not this
   finding's. Identical on both binaries. The route was not run.
3. **`jigc validate` prints `no findings — the committed store validates clean`** after the
   `placement-root` door in the `ff` state, over a store two docs have dropped out of.
   Identical on both binaries. Seen from this door; not pursued.
4. **Whether a tracked name that is not UTF-8 is a supported layout** — not settled here,
   because the verdict does not turn on it. On this host the state exists as an index entry
   only.
5. **The other callers of `task::git_capture`** — not enumerated, not driven, on either
   binary.
6. **What follows once the entry is removed again** (the aftermath cells of the report this
   block comes from) was not re-driven on the previous release: triage asked for the three
   doors in the three states.

## Bounds — what this verification did not do

- One value per door (`notes`), one corpus per door; text and `--format json` for the
  `docs-root` and `relocate` doors and text only for `placement-root`, as the block has it.
- One macOS host, one git. Both binaries are release builds handed to me; I built nothing.
- The comparison of the two runs normalises the rig's root path and 40-character commit
  ids, and nothing else. A difference inside either of those would not show; the commit ids
  differ for the stated reason, and the paths are the rigs' own directories.
- It read the report the block lives in, the exit rule's entry, the run's opening and
  `design/storage.md` → Placement. It read no other finding's report, no other verifier's,
  and nothing of triage's reasoning beyond the grade and the one line that says what to
  re-drive.

## Where the evidence is

- `<scratch>/verify-p2-orphan-prev.q7QTJc/scripts/drive.sh` — the driver.
- `<scratch>/verify-p2-orphan-prev.q7QTJc/cand.log`, `prev.log` — every command, its exit
  status and its two streams, in order, one log per binary (763 lines each).
- `<scratch>/verify-p2-orphan-prev.q7QTJc/runs/cand/`, `runs/prev/` — one file per stream
  and per exit status per step, the raw index listings, and the rigs themselves.

<!-- end of report -->
