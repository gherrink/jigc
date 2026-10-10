# verify-real — `r1-non-utf8-path-other-orphan-walk-consumers`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: the
lead `LD-2` of `completions/artifacts/canary-one/r1/reports/test/row-doc-list-reconciler.a1.md`
— *not driven* by its reporter, its block naming no command. Clause it is said to break:
`migration-works`. Triage's grade: *unclear*.

## Verdict in one paragraph

**`refuted` — as a blocker, with the basis `breaks-no-clause`. It is not `does-not-reproduce`:
the behaviour the lead guessed at is real, on all three doors.** With one index entry whose
name holds the byte 0xFF, `jigc config set docs-root`, `jigc config set placement-root` and
`jigc relocate` each read an empty committed listing, move nothing, say nothing about it and
exit 0 — and the two `config set` doors still land the knob, so the store then resolves the
docs at a home no doc is at. An index entry planted by the same plumbing under an ASCII name
changes nothing, so the cause is the byte and not the staged entry. **What does not survive is
the clause.** `migration-works` is the fourth clause of the exit rule — *the planned migration
for this project to use jigc will work* — and its scope is this repository's own migration,
rehearsed on a plain clone of it. This repository tracks 3352 paths at `eeffe347` and not one
of them holds a byte outside ASCII, so the state the three doors mis-handle is not in the
repository the clause is about. The defect stays a row of the ledger. **Whether the same
behaviour breaks the first clause was not what this verifier was sent to answer, and is left
open below, first in the list** — it is the reading that could make this row a blocker.

## The binary, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` —
  the hash the prompt gives for the candidate (commit
  `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` —
  the previous release's. **Nothing was driven on it** (see *The regression fact*).
- The candidate's directory went first on `PATH` and `command -v jigc` printed the
  candidate's path. Both driver scripts repeat that check and stop if it fails, and each rig
  was built with `dev/jigc-rig <state> --binary <the candidate>`; the scripts also stop
  unless the rig's `$JIGC` is that path.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## The three verbs, identified

`grep -rn "committed_markdown(" crates/cli/src` at `eeffe347`: 6 hits — the definition
(`orphan.rs:144`) and 5 call sites. Two are the listing's own (`orphan.rs:222`, `orphan.rs:616`)
and are another finding's. The three this finding names, each followed to its verb by a grep
for its enclosing function's name:

| call site | enclosing function | reached from | the verb |
|---|---|---|---|
| `orphan.rs:1116` | `docs_root_would_orphan` | `config.rs:1360`, in `route_docs_root_repoint_orphans`, called once at `config.rs:516` when the key is `docs-root` | `jigc config set docs-root <value>` |
| `config.rs:1512` | `route_placement_root_repoint_strands` | called once, at `config.rs:533`, when the key is `placement-root` | `jigc config set placement-root <value>` |
| `relocate.rs:708` | `relocate_stranded` | `relocate.rs:670` (`relocate_freeze_exempt`, from `relocate::run`, `cli.rs:1223`) and `config.rs:1543` (the `placement-root` sweep's move loop) | `jigc relocate <doctype> --from <prior home>` — and the second half of the `placement-root` door |

Under this state the `placement-root` door never reaches its move loop: the pre-check at
`config.rs:1512` already reads the empty listing and returns.

**The mechanism, read in the source and matched by every cell below.**
`orphan::committed_markdown` runs `git ls-files -z` through `task::git_capture`
(`task.rs:10087`), which returns an error unless git's whole standard output is UTF-8
(`String::from_utf8`), and `committed_markdown` turns any error into an empty list. git itself
exits 0 here. So one name that is not UTF-8, anywhere in the index, empties the listing for
every path — the `.md` files beside it included.

## What was driven

Every cell on its own fresh rig, minted under `<scratch>/verify-ld2.oRmkuX/runs/`; every
command whose exit status is read ran bare, its standard output and standard error each to a
file of its own. Three states per door:

- **none** — the rig as built: the control.
- **ascii** — the plant below under the name `badXname.txt`: the control for *a staged entry
  with no file behind it*.
- **ff** — the plant under `bad<byte 0xFF>name.txt`: the state of `Repro RC-8`.

The plant, in each rig, after it is built:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,<name>"        # exit 0

After the `ff` plant `git ls-files -z` exits 0, its output holds the byte 0xFF and is not
valid UTF-8 (read back in every `ff` cell); after the `ascii` plant it is valid UTF-8.

The corpus for the two `config set` doors is the rig state `refs-post-hoc`: five committed
managed docs, of which `docs/research/context-loss.md` is homed by a `location:` (the
`docs-root` door's subject) and `docs/roadmap.md` and `docs/decisions-log.md` by a nested
`placement:` (the `placement-root` door's). For `jigc relocate` the fixture is the standing
suite's own (`crates/cli/tests/relocate.rs`): the rig state `bare`, a manifest-less `note`
doctype named in `.jigc/config/packs.yaml`, and one committed note stranded at
`docs/legacy-notes/cache-benchmarks.md`. **That is what I changed from the block as handed:**
it named no command and no corpus beyond RC-8's index entry, and RC-8's own corpus — one
orphan file nobody claims — holds nothing any of the three doors would move.

In the tables below jigc's lines are quoted with their inner backticks dropped; the logs hold
them verbatim.

### Door 1 — `jigc config set docs-root notes`

| cell | state | exit | standard output | standard error | index afterwards |
|---|---|---|---|---|---|
| D0 | none | 0 | `config: set docs-root = notes — written to .jigc/config/, uncommitted …` | `relocating 1 committed doc(s) stranded by the docs-root re-point …` and `- docs/research/context-loss.md → notes/research/context-loss.md` | `R docs/research/context-loss.md -> notes/research/context-loss.md` |
| D1 | ascii | 0 | the same line | the same two lines | the same rename, beside `AD badXname.txt` |
| **D2** | **ff** | **0** | **the same line** | **empty** | **no rename; `docs/research/context-loss.md` still where it was** |
| D0j | none, `--format json` | 0 | `"relocated": [ { "from": "docs/research/context-loss.md", "to": "notes/research/context-loss.md" } ]` | empty | the rename |
| **D2j** | **ff, `--format json`** | **0** | **`"relocated": []`** | **empty** | **no rename** |

In every cell `.jigc/config/manifest.yaml` then holds `scalar:` / `docs-root: notes` and
`jigc config get docs-root` prints `docs-root = notes  (project)`. What follows in the same
rig, D0 and D1 against D2:

| next command | D0 · D1 | D2 |
|---|---|---|
| `jigc doc list` | exit 0, five rows, `research:context-loss  notes/research/context-loss.md  managed` among them | exit 0, **four rows — the research doc has no row** |
| `jigc doc show research:context-loss` | exit 0, the doc | **exit 1**, `blocking · store.not-found — could not read research:context-loss at notes/research/context-loss.md: No such file or directory (os error 2)` |
| `jigc validate` | exit 0, one advisory (an empty repeatable section) | exit 0, **the same one advisory and nothing about the stranded doc** |
| `jigc migrate-corpus` | exit 0, `0 migrated, 5 already current, 0 blocked` | exit 0, `0 migrated, 4 already current, 0 blocked` |

### Door 2 — `jigc config set placement-root notes`

| cell | state | exit | standard output | standard error | index afterwards |
|---|---|---|---|---|---|
| P0 | none | 0 | `config: set placement-root = notes — written to .jigc/config/, uncommitted …` | `relocating the committed doc(s) stranded by the placement-root re-point …`, `- docs/decisions-log.md → notes/decisions-log.md`, `- docs/roadmap.md → notes/roadmap.md` | both renames staged |
| P1 | ascii | 0 | the same line | the same three lines | both renames, beside `AD badXname.txt` |
| **P2** | **ff** | **0** | **the same line** | **empty** | **no rename; both docs still under `docs/`** |

In every cell the knob lands (`placement-root = notes  (project)`). Afterwards, in P2 only:
`jigc doc list` exits 0 with **three** rows (the roadmap and the decisions log have none),
`jigc doc show roadmap` exits 1 with `store.not-found … at notes/roadmap.md`, and
`jigc validate` exits 0 printing **`no findings — the committed store validates clean`**. In
P0 and P1 the listing has five rows, the read exits 0 and the sweep prints its one advisory.

### Door 3 — `jigc relocate note --from docs/legacy-notes/`

| cell | state | exit | standard output | index afterwards |
|---|---|---|---|---|
| R0 | none | 0 | `freeze-exempt relocation: 1 moved, 0 displaced, 0 blocked` and `moved  docs/legacy-notes/cache-benchmarks.md -> docs/notes/cache-benchmarks.md` | the rename staged |
| R1 | ascii | 0 | the same two lines | the rename, beside `AD badXname.txt` |
| **R2** | **ff** | **0** | **`freeze-exempt relocation: 0 moved, 0 displaced, 0 blocked`** | **no rename** |

Standard error is empty in all three. In the `ff` state `--format json` prints
`"moved": []`, `"blocked": []`, `"displaced": []` at exit 0 (cell A2). This door writes
nothing when it moves nothing: the status afterwards is the plant and the untracked project
layer, as before it ran.

### What stands afterwards (cells A1, A2)

- **No byte is destroyed and nothing is committed.** Across the `docs-root` door in the `ff`
  state: `git rev-parse HEAD` prints the same commit before and after;
  `git hash-object docs/research/context-loss.md` prints the same blob before and after;
  `git diff --quiet HEAD -- docs/research/context-loss.md` exits 0. The one thing written is
  the knob, in the untracked `.jigc/config/manifest.yaml`.
- **Once the entry is gone again** (`git update-index --force-remove -- <the name>`, exit 0)
  **the strand is seen, and nothing I ran moves it.** `jigc validate` exits 0 and now carries
  `advisory · file-state.orphaned-doc — committed doc docs/research/context-loss.md sits
  outside the resolved doctype roots — a docs-root change likely stranded it`, with the route
  *move it under the current resolved root (re-point `docs-root` to cover it) or drop it with
  `jigc unmanage`*. `jigc doc list` still has four rows. `jigc config set docs-root notes`
  again exits 0 and moves nothing — the value no longer changes the root. `jigc
  migrate-corpus` exits 0 with `4 already current`. That advisory's own route was not run
  (left open).
- **`jigc relocate` is re-runnable.** With the entry removed, the same command exits 0 with
  `1 moved` and stages the rename.

So, to the question triage put: **yes, the walk goes empty on all three; a re-point then lands
its knob over docs it left behind, and a relocation reports `0 moved` at exit 0.**

## Is it a defect, read against the design that owns it?

Yes — it is not behaviour a settled decision intends. `design/storage.md` → Placement owns
both re-point floors and states their purpose in one sentence: *a re-point moves what it would
strand*; and it names the state these cells end in as one of the two a root knob must not
produce — *a landed knob with no move leaves the store pointing at a home no doc is at*. The
code's own posture is narrower than what happened: the doc comment of `committed_markdown`
says *a `git` failure yields an empty listing (best-effort advisory)*, and the floors say *a
resolution/store-access hiccup returns silently* — but git did not fail here, it exited 0 and
listed every path. The same function's comment on `-z` shows that names outside ASCII were
meant to be read correctly.

What I searched for a ruling to the contrary: `design/storage.md`, `design/reconciliation.md`,
`design/corpus-migration.md`, `design/validation.md` and `DECISIONS.md`, for a name or output
that is not UTF-8. One declared bound about such a name exists — `DECISIONS.md` →
*2026-09-17 — M52 Increment 4 / T3*, *a non-UTF-8 entry name is narrated, not kept* — and it
is `jigc task finalize`'s, not these doors'; what it declares acceptable there is a failure
that is *visible rather than silent*. Nothing says a tracked name that is not UTF-8 should
empty this walk. So this is not `intended`, and nothing here is contested.

## Does it break `migration-works`, inside that clause's scope?

No. **Basis: `breaks-no-clause`.**

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  fourth clause is, in the human's words, *the planned migration for this project to use jigc
  will work*; its one instrument is *a rehearsal of this repository's migration on a copy of
  it with the candidate binary*. The run's opening names it `migration-works` and says the
  same. `implementation/decisions-pending.md` → *The exit rule* gives the rehearsal's ruled
  shape: a plain `git clone` of this repository, an isolated home and the candidate's binary.
  The clause is about one repository — this one.
- **The state is not in it.** In the clone at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`,
  `git ls-files -z` exits 0 and lists **3352** entries; the listing is valid UTF-8 and holds
  **0** bytes at or above 0x80 — every tracked name is ASCII. The enumeration is the index
  itself, read whole, and the count is derived from it, not from a diff. A clone carries the
  same names.
- **This host could not hold such a file.** `touch` of the 0xFF name in a scratch directory
  exits 1 with *Illegal byte sequence*: on this filesystem the state exists as an index entry
  only — by plumbing, or by cloning a repository that tracks such a name.
- **Read, not driven:** a doc jigc itself mints is named from a slug, and the slug grammar is
  lowercase `a-z0-9` words joined by `-` (`CLAUDE.md` → Branches states the grammar). I did
  not drive a migration to see what names it writes.

The lead said as much of itself — *nothing here establishes that* the fourth clause is
touched. Driven, the mechanism is real, and the clause it was filed under is not the one it
could break. Which steps the planned migration takes (whether it re-points a root knob at
all) did not need settling: the state the doors mis-handle is absent whatever they are.

**What this verdict does not say.** It does not say the behaviour breaks no clause of the
rule. It was graded against the clause the finding names; the first clause is the open item
below.

## The regression fact

**Not established, and not owed by this verdict**: step 4 runs with `confirmed` only. The
previous release's hash was asserted and its binary was not driven. If triage re-grades this
row against another clause and it comes back confirmed, the block below is what runs on the
previous release; how these doors behave there is **not known**.

## Class

**Driven: the three call sites handed to me, through one state each — `instance, unbounded`
beyond them.** Of the mechanism `committed_markdown` the consumers are enumerated: 5 call
sites by the grep above, 2 of them another finding's and 3 driven here. Of the wider mechanism
behind it — every caller of `task::git_capture`, each of which shares the UTF-8 step — I
enumerated nothing and give no count.

## Coverage

The lead's block says `UNPINNED: not driven`. Checked against the suites, not the diff: a grep
of `crates/cli/tests` and `tooling-tests` (the engine crate has no `tests/` directory) for a
0xFF byte, a non-UTF-8 name or the `--cacheinfo` plumbing hits 12 files. Nine of them name
neither root knob nor `relocate` anywhere. The three that do were read at their hits, and none
drives a tracked *path name* that is not UTF-8 at any of these doors:
`subtask_discard_record` plants such bytes as a file's *content*,
`uninstall_workbench_subject` as an *argument*, and `version_stamp_rollback` uses
`--cacheinfo` to stage a blob at a doc's destination under an ordinary name. The in-module
tests of `config.rs`, `orphan.rs` and `relocate.rs` name no such state. So: no standing test
holds this behaviour, in either direction.

## Repro VR-LD-2

```yaml
claim: "with one index entry whose name is not UTF-8, the three other consumers of orphan::committed_markdown see an empty listing — and that breaks the clause migration-works"
verdict: "REFUTED as a blocker — breaks-no-clause. The behaviour half reproduces on all three doors; the clause half fails on the clause's scope."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347"
plant:                      # after each door's setup; needs no file on disk
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
doors:
  - door: "jigc config set docs-root"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <candidate>
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
        stdout: "docs/research/context-loss.md"          # the doc did not move
      - exit: 1
        stderr_contains: "store.not-found"
    control: "without the plant, or with it under the name badXname.txt: relocated holds one pair, docs/research/context-loss.md -> notes/research/context-loss.md, and the read exits 0"
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
        stdout: "docs/decisions-log.md\ndocs/roadmap.md"   # neither moved
      - exit: 0
        stdout_contains: "no findings — the committed store validates clean"
    control: "without the plant, or with it under the name badXname.txt: both docs are staged renames into notes/"
  - door: "jigc relocate"
    setup:
      - fixture: bare                    # dev/jigc-rig bare --binary <candidate>
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
    control: "without the plant, or with it under the name badXname.txt: `1 moved`, the rename staged"
scope:                      # the half the verdict rests on
  - claim: "this repository, whose migration the clause is about, tracks no name outside ASCII"
    repro:
      - ["git", "ls-files", "-z"]         # in the clone, at eeffe347
    expect:
      exit: 0
      entries: 3352
      bytes_at_or_above_0x80: 0
observed: "<scratch>/verify-ld2.oRmkuX — cand.log (cells D0 D1 D2 D0j D2j P0 P1 P2 R0 R1 R2), cand-after.log (A1 A2); the three `git ls-files -- <paths>` read-backs were run on the D2j, P2 and R2 rigs after their cells"
pinned-by: "UNPINNED: the door cells hold a defect as observed — see Pinnable"
```

**Pinnable as it stands: no.** Mechanically it converts — the plant is two git plumbing calls
and needs no file on disk; the name has to be passed as raw bytes, so the test is Unix-only.
But pinned green, the three door cells would hold in place, as expected output, a root knob
landing over docs it left behind — the state the owning design names as the one the door
exists to prevent. They are a fix's red test in waiting, with each `expect` inverted, if
triage schedules one. The `scope` cell is the only part that states a fact this verdict wants
to stay true, and it is a fact about this repository, not about the product.

## Left open

1. **The same behaviour, read against the first clause (`no-lost-files`) — not graded here,
   and the reading that could make this row a blocker.** That clause's scope ends: *where
   jigc cannot tell (git fails, the index is unreadable) it refuses before writing*. Both
   `config set` doors could not read git's listing and wrote the knob at exit 0. Against
   that: no byte no git object holds was destroyed and nothing was committed (driven, cell
   A1), and git did not fail — jigc could not decode what git said. Two questions decide it
   and neither is a verifier's: whether that sentence binds on its own or only in service of
   the bytes it protects; and whether a tracked name that is not UTF-8 is a *healthy
   repository used as documented* or a *deliberately planted state*. As built here it was
   planted by plumbing.
2. **`jigc validate` says nothing while the entry stands** — after the `placement-root` door
   it prints `no findings — the committed store validates clean` over a store two docs have
   dropped out of (cell P2). The walk it would find them with is the same emptied listing.
   Seen here from a second door; not pursued.
3. **`jigc doc show` on the stranded doc exits 1 with `store.not-found`** and the route
   *create the referenced doc, or fix the reference to an existing one …* — the doc exists,
   at its prior home. A sentence for a human; not run.
4. **Whether any printed route lands the stranded doc at the new root.** With the entry
   removed, `file-state.orphaned-doc` routes to *re-point `docs-root` to cover it* or
   `jigc unmanage`; neither was run. The same `config set docs-root notes` again moves
   nothing, and `jigc migrate-corpus` does not carry the doc.
5. **The previous release** — not driven; the regression fact is unknown.
6. **Reach.** On a filesystem that admits the name it is an ordinary committed file, and a
   clone on this host of a repository tracking one would hold the index entry without the
   file. Neither was driven: one macOS host, one git.
7. **The other callers of `task::git_capture`** — each shares the UTF-8 step; not
   enumerated, not driven.

## Bounds — what this verification did not do

- It drove one value per door (`notes`), one corpus per door, text and `--format json` for
  the `docs-root` and `relocate` doors and text only for `placement-root`.
- The `refs-post-hoc` rig holds a live task; the controls show both `config set` doors work
  beside it, and no cell without one was driven.
- For `jigc relocate` the project layer is hand-made, as in the standing suite: the door
  refuses a manifest-frozen doctype, so the fixture is a pack of the project's own. Whether
  any shipped doctype is freeze-exempt was not checked.
- It read `design/storage.md` → Placement and the exit rule's entry; it read no other
  finding's report and no other verifier's. Of the reporter's report it read the lead and the
  one block the lead's setup points at.

## Where the evidence is

- `<scratch>/verify-ld2.oRmkuX/scripts/drive.sh` and `aftermath.sh` — the two drivers.
- `<scratch>/verify-ld2.oRmkuX/cand.log`, `cand-after.log` — every command, its exit status,
  its standard output and standard error, in order.
- `<scratch>/verify-ld2.oRmkuX/runs/cand/`, `runs/cand-after/` — one file per stream per
  step, and the rigs themselves.
- `<scratch>/verify-ld2.oRmkuX/this-repo-ls.bin` — this repository's `git ls-files -z`.

<!-- end of report -->
