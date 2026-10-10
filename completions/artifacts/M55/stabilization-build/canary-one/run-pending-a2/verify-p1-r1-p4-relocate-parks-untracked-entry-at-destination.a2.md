# verify-real — `r1-p4-relocate-parks-untracked-entry-at-destination` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-relocate-parks-untracked-entry-at-destination`. One finding, handed over by triage
with the grade *unclear*: door `jigc relocate`, the clause it is said to break `no-lost-files`, and for a repro
block *Left open*, item 4 of `verify-p3-r1-p3-unreadable-entry-class-door-list-traced-by-hand.a1.md` — a
paragraph, no block. Triage asked for: `jigc relocate` over an untracked entry at a destination — what moves,
what the ack names, both binaries.

## Verdict

- **verdict:** `refuted` — **as a blocker**. What it describes is real and stays a row of the ledger.
- **basis:** `breaks-no-clause` — in twelve states of the destination and the source, on both binaries, no
  byte of an untracked entry is destroyed by the candidate and none is committed: the entry is renamed, the
  same inode, into `.jigc/displaced/`. Where the stranded doc's move **lands**, that is the behaviour
  `design/reconciliation.md` → *Relocation collisions* rules, and the ack names both paths. Where the move
  then **fails**, the entry is parked all the same and the ack does **not** name it — a real defect, the same
  on `1.0.0-rc.24`, which destroys nothing either. It is **not** `does-not-reproduce`: the park happens in
  every cell. It is **not** `intended` as a whole: the un-named park is ruled by no decision and contradicts
  two design sentences.
- **contested:** `false`. The finding does not argue that a decision is wrong. The reading of the clause the
  verdict rests on is stated under *Step 3*, with what the other reading returns, so that it can be overruled.
- **regression:** not returned (it goes with `confirmed` only). The two binaries were compared all the same:
  24 paired invocations of the door, below.
- **class:** `instance, unbounded`. Twelve states were driven at one door. The mechanism's consumers were not
  enumerated; what was read of the code is said where it is used, and marked as read.
- **platform:** **macOS only** — macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157), a user that is not root.
  Nothing was driven on Linux, and a mode-000 file refuses nobody who may read any file.

**What the drive found, in six lines.**

1. **The park is as the finding says, where the doc's move lands.** An untracked regular file, a dangling
   link, a live link, a mode-000 file, a directory with two files in it and a file git ignores: each is moved
   whole into `.jigc/displaced/<basename>` — the same inode, mode, size and mtime, the same sha256 where it can
   be read — the stranded doc lands at its home, the exit is 0, and the ack prints
   `displaced docs/notes/cache-benchmarks.md -> .jigc/displaced/cache-benchmarks.md (foreign squatter → workbench)`.
2. **Nothing of the entry is committed.** The door makes no commit; it stages one rename. The commit a user
   makes next holds that rename and nothing else, and no path under `.jigc/displaced/` is in `git ls-files`.
3. **The candidate never replaces a file already parked.** Two doctypes, each with an untracked `todo.md` at
   its destination: the second is parked at `.jigc/displaced/todo.md.2`. **`1.0.0-rc.24` replaces the first**
   — 41 bytes no git object holds, gone at exit 0 — so here the candidate is the safer of the two.
4. **The dangling link is the one cell where the candidate does more than `1.0.0-rc.24`.** The previous
   release prints a `blocked` row carrying git's own *destination exists* and moves nothing; the candidate
   parks the link, as the link it is, and lands the doc. That change is ruled.
5. **Where the move then fails, the park is not named.** The stranded doc moved by hand with a plain `mv`
   (so it *is* the untracked entry at the destination): `0 moved, 0 displaced, 1 blocked` at exit 0, and the
   doc — 85 bytes, an edit in them that no git object holds — is now `.jigc/displaced/cache-benchmarks.md`, a
   path git ignores, which nothing printed. The same with the source at mode 000. **Both binaries, byte for
   byte.** It is *Left open* 1, with its block.
6. **In a repository that never ran `jigc setup`, the parking home is not ignored.** The suite's own fixture:
   the parked file shows as `??`. *Left open* 2.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`).
- Before every rig the driven binary's directory went first on `PATH` and `command -v jigc` was held to
  `<that directory>/jigc` (the driver exits 97 otherwise; it never did), and each rig's `$JIGC` was held to
  the same file. No `cargo build`; nothing under `target/` was driven. Every rig was built with
  `dev/jigc-rig <state> --binary <that binary>`, stdout captured alone, the build's status read before the
  `eval`.
- The clone, after the drives: branch `fix/canary-one`, `HEAD` 126a8531 (the record commit on eeffe347;
  `git diff --stat eeffe347 HEAD -- crates design dev` is empty). Its untracked files are other reporters'
  reports under `completions/artifacts/canary-one/r1/reports/test/`. Nothing was built, edited, staged or
  committed there by this reporter.

**What I read, said plainly.** The one report my prompt handed me. The run's opening record. The clause in
`DECISIONS.md` → *2026-10-04 — The exit rule, revised*. `design/reconciliation.md` → *Relocation collisions*,
`design/command-output-contract.md` on `jigc relocate`, `design/storage.md` on `.jigc/displaced/`,
`design/finalize.md` → *4. Promote*, the `DECISIONS.md` entry of 2026-10-06 on `store.home-not-regular-file`,
`crates/cli/src/relocate.rs`, and `crates/cli/tests/relocate.rs`. No other verifier's report and nothing of
triage's file. The driver scripts are in my own scratch directory and were written through the shell; the
file tool wrote this report and nothing else.

## Step 1 — driven from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p4-relocate.bGtmyW` (written `<W>`). Every root is under `<W>/roots`, minted by
`dev/jigc-rig` with `TMPDIR` pointed there. **One rig per state and per binary** — 25 rigs: one probe,
twelve states on each binary. No root of any other reporter was read or reused.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables, stdin
from `/dev/null`. Around each invocation: a manifest of every entry under the repository outside `.git`
(kind, mode, size, inode, mtime, and sha256 or link target); `git status --porcelain --untracked-files=all
--ignored`; the invocation, stdout and stderr each to a file, **the exit status read from the bare command**;
the two again, `HEAD` and the commit count. No pipe stands between a driven command and its exit status, and
no output was cut.

**What had to be reconstructed, and what I changed.** The finding gives a paragraph, not a block. The earlier
report drove *the fixture of `crates/cli/tests/relocate.rs`* — a hand-made `.jigc/config/` with no
`jigc setup`. The clause's scope is *a healthy repository used as documented*, so the cells here are built on
the rig state `fresh` (`jigc setup` only), and the manifest-less doctype reaches it the way
`design/multi-pack.md` describes: a `packs:` list appended to the `packs.yaml` that `jigc setup` wrote. The
suite's own fixture is kept as one cell (`B-F`).

**The setup of every cell but `B-F`:**

- `<rig>/note-pack/schemas/note.yaml` — `type: note`, `location: notes/`, `id-from: title`, one section
  `body` with a slot; a second schema, `memo` at `memos/`, beside it.
- `packs:` and `  - <rig>/note-pack` appended to `.jigc/config/packs.yaml`.
- `docs/legacy-notes/cache-benchmarks.md` — the 60 bytes of the suite's managed note
  (sha256 `e63e6850…38dc71`).
- `git add` of those two paths, `git commit -q -m "strand the note"` (exit 0 in all 24 rigs; the commit runs
  through the `pre-commit` hook `jigc setup` installed).
- then the state of the row, and `jigc relocate note --from docs/legacy-notes/`, twice.

### The cells

The destination is `docs/notes/cache-benchmarks.md`. **Every invocation of the door exits 0, on both
binaries, with nothing on stderr.**

| cell | the destination, before | candidate | `1.0.0-rc.24` |
|---|---|---|---|
| `N` | free — the control | `1 moved, 0 displaced, 0 blocked` | the same bytes |
| `F` | an untracked regular file, 61 bytes (sha256 `fb9b4c71…b9f94f`) | `1 moved, 1 displaced, 0 blocked`; the file is `.jigc/displaced/cache-benchmarks.md`, the same inode, mode, mtime and sha256 | the same bytes |
| `Fj` | the same, the door run with `--format json` | `"displaced": [["docs/notes/cache-benchmarks.md", ".jigc/displaced/cache-benchmarks.md"]]`, `"moved"` one pair, `"blocked": []` | the same bytes |
| `L` | an untracked dangling link, `-> nowhere-target` | `1 moved, 1 displaced`; the **link itself** is parked, the same inode, still `-> nowhere-target`; no `nowhere-target` appears anywhere | `0 moved, 0 displaced, 1 blocked` — git's `fatal: destination exists`; **nothing moves**, the link stands |
| `V` | an untracked link `-> sibling.txt`, to an untracked file beside it | `1 moved, 1 displaced`; the link itself is parked, the same inode, still `-> sibling.txt`; `docs/notes/sibling.txt` is untouched | the same bytes |
| `M` | an untracked regular file at mode 000 | `1 moved, 1 displaced`; parked, the same inode, still mode 0 and 61 bytes; `cat` of it exits 1 before and after | the same bytes |
| `D` | an untracked directory of that name holding `inner.txt` and `sub/deep.txt` | `1 moved, 1 displaced`; the directory is parked whole — both files the same inode and sha256 | the same bytes |
| `I` | an untracked file git ignores (`.git/info/exclude` names it) | `1 moved, 1 displaced`; parked, the same inode and sha256 | the same bytes |
| `T` | two doctypes: `docs/notes/todo.md` (41 bytes) and `docs/memos/todo.md` (42 bytes), both untracked; `relocate note …`, then `relocate memo --from docs/legacy-memos/` | the first is `.jigc/displaced/todo.md`, the second **`.jigc/displaced/todo.md.2`**, and the ack says so; both survive | the second ack also prints `.jigc/displaced/todo.md`; the 41 bytes of the first (sha256 `aa7b820e…76a2fc`) are **replaced** — gone at exit 0 |
| `B-F` | the suite's fixture — rig state `bare`, a hand-made `.jigc/config/packs.yaml`, no `jigc setup` — and the file of `F` | `1 moved, 1 displaced`; parked, the same inode and sha256 — and `git status` shows `?? .jigc/displaced/cache-benchmarks.md` | the same bytes |
| `S` | the file of `F`; and the **source**, the stranded doc, at mode 000 | `0 moved, 0 displaced, 1 blocked` — `reading the stranded doc docs/legacy-notes/cache-benchmarks.md: Permission denied (os error 13)`; **the file is parked all the same**, the same inode and sha256, and no line names it | the same bytes |
| `H` | the stranded doc itself, moved there with a plain `mv` after one line was appended to it (85 bytes, sha256 `03f7d81f…e5aca0`) | `0 moved, 0 displaced, 1 blocked` — `reading the stranded doc …: No such file or directory (os error 2)`; **the doc is parked all the same**, the same inode and sha256, and no line names it | the same bytes |

**The second run of the door** in each rig: `0 moved, 0 displaced, 0 blocked` wherever the first run landed
the doc, and the same `blocked` row again in `L` on the previous release, in `S` and in `H`. It changes no
entry of any manifest.

**The two binaries, compared.** 24 paired invocations of the door, 72 files of exit status, stdout and
stderr: 69 identical, 3 differ — the two stdouts of `L` and the second stdout of `T`, as the table has them.

### What moves, and what does not

- **The move is a rename of the entry.** In every cell the parked entry has the inode, mode, size and mtime
  the entry at the destination had, and a link has its target string. Nothing is copied, nothing is opened,
  nothing is written through a link.
- **Nothing is committed by the door.** `HEAD` and the commit count are the same before and after every
  invocation. Where the doc landed, the index holds one entry —
  `R100 docs/legacy-notes/cache-benchmarks.md docs/notes/cache-benchmarks.md` — and two in `T`.
- **The commit a user makes next holds no byte of the entry.** `git commit -m "relocate the note"` after the
  door: exit 0, `1 file changed … rename docs/{legacy-notes => notes}/cache-benchmarks.md (100%)`; the blob at
  `HEAD:docs/notes/cache-benchmarks.md` is the managed note's 60 bytes; `git ls-files` names no path under
  `.jigc/displaced/`.
- **The parking home is ignored where `jigc setup` ran.** `.jigc/.gitignore`, line 7, `displaced/`:
  `git status --ignored` shows the parked entry as `!!`. In `B-F`, where nothing wrote that file, it is `??`.
- **The gitignored file-state record** is written with the destination's one entry where the doc landed, and
  is not written at all in `S` or `H`.

### The cell the ack does not name — `H`, read closely

Before: `git status` reads ` D docs/legacy-notes/cache-benchmarks.md` and
`?? docs/notes/cache-benchmarks.md`. The command exits 0 and prints three lines: the summary
`0 moved, 0 displaced, 1 blocked`, the `blocked` row with the path that no longer exists, and the footer.
After: `git status --short` reads ` D docs/legacy-notes/cache-benchmarks.md` and nothing else; `docs/notes/`
is empty; the 85 bytes are at `.jigc/displaced/cache-benchmarks.md`. `git hash-object` of them is
`98986c05…`, and `git cat-file -e` of that id exits 1 — **no git object holds them**; `HEAD` holds the 60-byte
original. The bytes are intact, and nothing the command printed says where they are, or that anything moved.

Why, **read and not driven further**: `relocate_one` (`crates/cli/src/relocate.rs`) parks the destination's
entry first and then reads the source; an error after the park returns through `?`, and `relocate_stranded`
turns it into a `blocked` row built from the error alone, so the `(from, to)` pair of the park is dropped.
The door passes no undo (`relocate_freeze_exempt`: *`jigc relocate` is not a transaction*), which is where
the sibling door keeps such a pair — commit 242341bb, *a refused re-point names the foreign file it parked
and did not put back*. Two of the errors that path can meet were driven: the source unreadable (`S`) and the
source absent (`H`). The others were not.

## Step 2 — is it what the finding says

The finding says three things.

- **That `jigc relocate` moves an untracked entry it finds at a destination into `.jigc/displaced/`, a
  dangling link and an unreadable file included, named in the ack.** So it is, in `F`, `Fj`, `L`, `M` and in
  the four further shapes — where the doc's move lands. In `S` and `H` it is moved and **not** named.
- **That for the link the candidate does this where `1.0.0-rc.24` printed a `blocked` row.** So it is (`L`).
- **That `design/finalize.md` words the store's rule as "neither writes through a link nor moves one", that
  `relocate.rs` says the opposite on purpose, and that the two were not reconciled.** The sentence is not in
  `design/finalize.md` at this commit. It is the sentence `store.home-not-regular-file` prints
  (`crates/engine/src/store.rs`, `home_shape_refusal`): "jigc keeps a managed doc as a regular file at exactly
  its home, and neither writes through a link nor moves one". Its subject is the entry at **a managed doc's
  own home**. The design section that owns this door states both halves in one paragraph and reconciles
  them — `design/reconciliation.md` → *Relocation collisions*: at the destination "A link is a foreign entry
  like any other: it is parked in the workbench **as the link it is** (the move is a rename of the entry — it
  is never followed, and nothing is written through it)", and "The mirror question is asked of the
  **source**: a stranded entry that is itself a link is not carried to the new home". Read alone, the printed
  sentence does say more than the product does at this door. That is wording, and it is all that is left of
  the third point.

**Against stale state, a pipe, a cut, another binary.** Every rig was minted for this report; the control
`N` is built the same way; exit statuses are read bare and outputs are whole files; both hashes were asserted
first and `PATH` held before each rig.

**Against a settled decision.**

- **The park where the move lands is intended.** `design/reconciliation.md` → *Relocation collisions (M39)*:
  "A *foreign* (untracked/unmanaged) file at the destination → move-into-workbench … **out of** the
  destination and **uncommittable**, so the managed instance can land *and* a stray working file is never
  accidentally committed and never silently clobbered. The foreign bytes are preserved verbatim in the
  workbench". `design/storage.md` names the home: "the parking home for bytes jigc moved rather than
  destroyed". `design/command-output-contract.md` names the key: "**`displaced`** the foreign files that
  squatted a destination and were moved into the gitignored workbench rather than clobbered".
- **The link is intended.** The same section, *the rc.24 fix pass*; and `DECISIONS.md` → *2026-10-06 — One
  code for a managed home that is not an ordinary file* lists it among what that ruling left as it was: "the
  relocation's destination, where a squatting link is parked like any other foreign entry".
- **The `.2` is intended.** The same section: "it is parked beside what is already parked there, never over
  it"; `DECISIONS.md`, round 4 of the fix pass, commit 685a4c55.
- **The un-named park is not.** No design sentence says the door may park an entry and not say so. Three
  sentences say the opposite: the contract's definition of `displaced` above; *Relocation collisions* — "the
  ack names the path the file actually went to"; and a comment in the first test of
  `crates/cli/tests/relocate.rs`, the suite that holds the landed cell — "a move into a gitignored tree that
  nothing narrates is a deletion as far as the reader is concerned".

## Step 3 — does it break `no-lost-files`, inside the clause's scope

The clause, `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We do not lose files or writhe / update
incorrect things." Its scope, the first sharpening: "In a healthy repository used as documented — which
includes ordinary git configuration … — no jigc command at exit 0 destroys bytes no git object holds or
commits content the user did not ask for. Where jigc cannot tell (git fails, the index is unreadable) it
refuses before writing. Races against a non-jigc writer inside a millisecond window, and deliberately planted
states, are **declared bounds**". And the rule: "A finding blocks the 1.0.0 call only if it breaks a clause
inside its scope; everything else is recorded with its tier." The run's opening declares no bound, and none
is used here.

**Destroys bytes no git object holds.** No cell on the candidate. Every entry is at its parking path with
the inode and the bytes it had; `T`, the one cell in which `1.0.0-rc.24` destroys such bytes, is the one the
candidate keeps both files in.

**Commits content the user did not ask for.** No cell, on either binary. The door commits nothing, and what
it stages is the rename it was asked for.

**So the clause, as its scope words it, is not broken** — by the park the finding describes, and not by the
un-named park either.

*The reading this rests on, said so that it can be overruled.* I held the clause to its sharpened scope:
*destroys* and *commits*. Cell `H` is the cell on which another reading turns. A doc holding an edit no git
object holds leaves the working tree at exit 0 for a directory git ignores, and no line of output says so; to
the person at the keyboard that file is gone, and a comment in the project's own suite calls an un-narrated
move of this kind a deletion. If *we do not lose files* is read past the sharpening to cover that, this
finding is `confirmed`, with `regression: false` — the cell is byte-identical on `1.0.0-rc.24` (sha256
accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d). The state needs nothing planted: a doc
moved by hand, and then the door that exists to move it. The block is `Repro V-4b`, and the row is *Left
open* 1 so that it is triaged as itself whichever way this one is read.

**Verdict on the clause: `no-lost-files` is not broken inside its scope by `jigc relocate` parking an
untracked entry at a destination.** `refuted` as a blocker, basis `breaks-no-clause`.

## The coverage of what was driven

The finding carries no coverage claim. For the blocks' `pinned-by` only, at eeffe347: thirteen files under
`crates/cli/tests` and `tooling-tests` spell the argv `"relocate"`; **one was read whole**,
`crates/cli/tests/relocate.rs`, with the in-module tests of `crates/cli/src/relocate.rs`. What they hold:

- an **untracked** regular file at the destination, through the binary, the ack's path and the bytes
  asserted — `relocate::a_parked_squatter_never_replaces_a_file_already_parked_under_its_name` (with a file
  already parked: the `.2` rule);
- a **committed** unmanaged file there, through the binary —
  `relocate::a_foreign_squatter_is_displaced_into_the_workbench_and_the_ack_names_it`;
- an untracked file and a dangling link there, in-module, through `relocate_freeze_exempt` —
  `a_foreign_squatter_at_the_destination_moves_into_the_workbench_and_the_managed_lands` and
  `a_dangling_link_squatting_the_destination_is_displaced_like_any_squatter`.

Not found in what was read: a mode-000 entry, a directory, a live link or an ignored file at the destination;
and any cell in which the move fails after the park. The string `reading the stranded doc` is spelled in one
test file, in a comment about the `config set` door. The other twelve files were not read for it.

## The repro blocks

### Repro V-4

```yaml
claim: "`jigc relocate` moves an untracked entry it finds at a relocation destination into `.jigc/displaced/`, and that breaks the closing condition's `no-lost-files` clause (ledger key r1-p4-relocate-parks-untracked-entry-at-destination)"
verdict: REFUTED            # as a blocker — basis breaks-no-clause. The entry is moved whole, named, and in no commit. The cell the ack does not name is Repro V-4b.
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same bytes, but for the two variants marked D"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
setup:            # HOME the rig's own; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1
  - fixture: fresh                         # `jigc setup` only
  - write: { path: "<rig>/note-pack/schemas/note.yaml", text: "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n" }
  - append: { path: ".jigc/config/packs.yaml", text: "packs:\n  - <rig>/note-pack\n" }
  - write: { path: "docs/legacy-notes/cache-benchmarks.md", text: "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n" }
  - ["git", "add", ".jigc/config/packs.yaml", "docs/legacy-notes/cache-benchmarks.md"]
  - ["git", "commit", "-q", "-m", "strand the note"]
  - write: { path: "docs/notes/cache-benchmarks.md", text: "# Someone else's notes\n\nHand-written, never managed by jigc.\n" }   # untracked, never added
repro:
  - ["jigc", "relocate", "note", "--from", "docs/legacy-notes/"]
  - ["jigc", "relocate", "note", "--from", "docs/legacy-notes/"]
  - ["git", "commit", "-m", "relocate the note"]
expect:
  - exit: 0
    stderr: ""
    stdout_lines:
      - "freeze-exempt relocation: 1 moved, 1 displaced, 0 blocked"
      - "  moved     docs/legacy-notes/cache-benchmarks.md -> docs/notes/cache-benchmarks.md"
      - "  displaced docs/notes/cache-benchmarks.md -> .jigc/displaced/cache-benchmarks.md (foreign squatter → workbench)"
    after:
      - ".jigc/displaced/cache-benchmarks.md holds the untracked file's 61 bytes — the same inode it had at the destination"
      - "docs/notes/cache-benchmarks.md holds the managed note's 60 bytes"
      - "HEAD has not moved; `git diff --cached --name-status` is the one line `R100 docs/legacy-notes/cache-benchmarks.md docs/notes/cache-benchmarks.md`"
      - "`git check-ignore .jigc/displaced/cache-benchmarks.md` exits 0"
  - exit: 0
    stdout_first_line: "freeze-exempt relocation: 0 moved, 0 displaced, 0 blocked"
  - exit: 0
    stdout_contains: "rename docs/{legacy-notes => notes}/cache-benchmarks.md (100%)"
    after: "`git ls-files` names no path under .jigc/displaced/; `git show HEAD:docs/notes/cache-benchmarks.md` is the managed note"
variants:         # each a rig of its own; the destination's entry replaces the last setup line; the door exits 0 with `1 moved, 1 displaced, 0 blocked` and the same `displaced` line
  - "`--format json` on the door -> \"displaced\": [[\"docs/notes/cache-benchmarks.md\", \".jigc/displaced/cache-benchmarks.md\"]], \"blocked\": []"
  - "`ln -s nowhere-target docs/notes/cache-benchmarks.md` -> the link itself parked, still `-> nowhere-target`   # D: rc.24 prints `0 moved, 0 displaced, 1 blocked` with git's `fatal: destination exists`, and moves nothing"
  - "`ln -s sibling.txt docs/notes/cache-benchmarks.md`, an untracked docs/notes/sibling.txt beside it -> the link itself parked, still `-> sibling.txt`; sibling.txt untouched"
  - "the file, then `chmod 000` -> parked, still mode 0 and 61 bytes"
  - "a directory docs/notes/cache-benchmarks.md/ holding inner.txt and sub/deep.txt -> parked whole, both files intact"
  - "the file, and `docs/notes/cache-benchmarks.md` written into .git/info/exclude -> parked"
  - "a second schema `memo` (location: memos/), stranded docs/legacy-notes/todo.md and docs/legacy-memos/todo.md, untracked docs/notes/todo.md and docs/memos/todo.md; `relocate note --from docs/legacy-notes/` then `relocate memo --from docs/legacy-memos/` -> the second `displaced` line ends `.jigc/displaced/todo.md.2`, and both untracked files survive   # D: rc.24 prints `.jigc/displaced/todo.md` twice and the first file's bytes are gone"
control: "no entry at the destination: `1 moved, 0 displaced, 0 blocked`, exit 0, no .jigc/displaced/ at all"
clause: "no-lost-files — NOT broken: no byte of the entry is destroyed (a rename, the same inode) and none is committed (the door commits nothing; the next commit is the one rename). The reading is stated in the report."
design: "design/reconciliation.md → Relocation collisions — the foreign file at the destination is moved into the workbench; a link is parked as the link it is; a name already taken gets `<name>.<n>`"
observed: "<W>/out/{N,F,Fj,L,V,M,D,I,T}-{cand,prev}/"
pinned-by: "PARTLY — relocate::a_parked_squatter_never_replaces_a_file_already_parked_under_its_name holds an untracked regular file through the binary; the in-module a_dangling_link_squatting_the_destination_is_displaced_like_any_squatter holds the link. UNPINNED in what was read: the mode-000 file, the directory, the live link, the ignored file (one suite file of the thirteen that name the door was read whole)"
```

**Pinnable as it stands: yes.** A named fixture state, four file writes, literal argv, exit statuses and
whole lines. What a converter needs to know:

1. **The pack is listed, not copied.** `<rig>/note-pack` is a directory outside the repository; the list is
   appended to the `packs.yaml` that `jigc setup` wrote, and that file is committed with the stranded doc so
   that the tree is clean before the entry goes in.
2. **Each variant is a rig of its own.** They were not driven joined.
3. **These are facts a fix must not break**, so they pin as they are — the fix *Left open* 1 asks for
   included.
4. **The inode is the sharpest assertion and the least portable.** The bytes and, for a link, the target
   string say the same thing on any filesystem.
5. **Platform edges.** Unix only for the links and the mode; the mode-000 variant is not reachable as root.

### Repro V-4b — the park the ack does not name, so that it is not re-derived

```yaml
claim: "where the stranded doc's move fails after `jigc relocate` has parked the destination's untracked entry, the door exits 0, the entry is in `.jigc/displaced/`, and no line of the output names it"
verdict: REFUTED            # as a break of `no-lost-files` BY THIS FINDING, under the scope's own words (nothing destroyed, nothing committed) — the effect is real; see Step 3 for the reading, and Left open 1
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — byte-identical, both variants"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
setup:
  - fixture: fresh
  - write: { path: "<rig>/note-pack/schemas/note.yaml", text: "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n" }
  - append: { path: ".jigc/config/packs.yaml", text: "packs:\n  - <rig>/note-pack\n" }
  - write: { path: "docs/legacy-notes/cache-benchmarks.md", text: "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n" }
  - ["git", "add", ".jigc/config/packs.yaml", "docs/legacy-notes/cache-benchmarks.md"]
  - ["git", "commit", "-q", "-m", "strand the note"]
  - append: { path: "docs/legacy-notes/cache-benchmarks.md", text: "An edit no commit holds.\n" }
  - ["mkdir", "-p", "docs/notes"]
  - ["mv", "docs/legacy-notes/cache-benchmarks.md", "docs/notes/cache-benchmarks.md"]     # a plain mv, not `git mv`
repro:
  - ["jigc", "relocate", "note", "--from", "docs/legacy-notes/"]
  - ["git", "status", "--short"]
expect:
  - exit: 0
    stderr: ""
    stdout_lines:
      - "freeze-exempt relocation: 0 moved, 0 displaced, 1 blocked"
      - "  blocked   docs/legacy-notes/cache-benchmarks.md"
      - "    reading the stranded doc docs/legacy-notes/cache-benchmarks.md: No such file or directory (os error 2)"
    stdout_lacks: ".jigc/displaced"
    after:
      - "docs/notes/cache-benchmarks.md is gone"
      - ".jigc/displaced/cache-benchmarks.md holds the 85 bytes, the edit in them — the same inode; `git cat-file -e` of their blob id exits 1"
  - exit: 0
    stdout: " D docs/legacy-notes/cache-benchmarks.md\n"
the-same-with: "an untracked file of somebody else's at the destination and `chmod 000 docs/legacy-notes/cache-benchmarks.md` in place of the last three setup lines — `Permission denied (os error 13)` in the blocked row, the file parked and unnamed"
control: "Repro V-4: where the move lands, the `displaced` line is printed"
design: "design/command-output-contract.md — `displaced` is 'the foreign files that squatted a destination and were moved into the gitignored workbench'; design/reconciliation.md → Relocation collisions — 'the ack names the path the file actually went to'"
observed: "<W>/out/{H,S}-{cand,prev}/"
pinned-by: "UNPINNED: no cell with a failed move after the park was found in crates/cli/tests/relocate.rs or the in-module tests; the other twelve suite files that name the door were not read"
```

Pinnable as it stands. As a test it is the red one of a fix that either names the parked entry or does not
park before the source can be read; the `stdout_lacks` line is the assertion that fix inverts.

## What was driven, and what was not

- **Driven:** `jigc relocate` over an untracked entry at a relocation destination in eight shapes — a
  regular file (agent and JSON output), a dangling link, a live link, a mode-000 file, a directory, an
  ignored file, and two same-named files at two doctypes' homes — each with the door run twice and the
  commit a user makes next; a no-entry control; the suite's own fixture once; and two cells in which the
  stranded doc's move fails after the park. Twelve states, each on both binaries.
- **Not driven:** Linux, and root. A **tracked** entry at the destination, with or without uncommitted
  edits (the suite's first cell is that state). A managed, baselined doc at the destination. A stranded
  source that is itself a link. A placement destination (`VISION.md`). The door from a linked worktree. The
  other errors the move can meet after the park — a destination git cannot track, an operation opened in
  between, a failing `git mv`, a file-state record that cannot be saved. The other doors that reach the same
  park (`jigc config set docs-root` and `placement-root`). **What `jigc uninstall` does with an entry parked
  under `.jigc/displaced/`** — still driven by nobody.
- **Not established:** how many doors reach `displace_foreign_squatter`; that no suite outside the one read
  holds the un-named park.
- **Nothing was fixed, graded or decided.**

## Left open — seen on the way, not pursued

1. **`jigc relocate` parks the destination's untracked entry and does not name it when the doc's move then
   fails.** Exit 0, `0 moved, 0 displaced, 1 blocked`, the entry in a directory git ignores, the `blocked`
   row code-less and route-less. Reached with nothing planted: the stranded doc moved by hand with `mv`, an
   uncommitted edit in it. Both binaries, byte for byte. No bytes are destroyed. `Repro V-4b`; the reading on
   which it breaks `no-lost-files` is under *Step 3*.
2. **Where `jigc setup` never ran, the parking home is not ignored.** With a hand-made `.jigc/config/` — the
   layout `crates/cli/tests/relocate.rs` builds — the parked file is `?? .jigc/displaced/cache-benchmarks.md`,
   one `git add -A` away from a commit, where `design/reconciliation.md` says *uncommittable*. Both binaries.
   Cell `B-F`. Whether that layout is a supported one was not established.

## Where the evidence is

`<W>` is `<scratch>/verify-p4-relocate.bGtmyW`. Per state and binary:
`<W>/out/<state>-<cand or prev>/<NN>-<label>.{argv,rc,stdout,stderr,man.before,man.after,mandiff,porc.before,porc.after,head.after}`,
with `root` (the rig it ran in), `00-setup-commit.{out,rc}`, `03-staged.out`, `03-check-ignore.{out,rc}`,
`03-displaced.ls`, `05-show-head.out`, `05-ls-files.out`, `05-landed-blob.{out,rc}`, `06-final.man` and
`06-final.porc`. The rigs: `<W>/roots/`. The tools: `<W>/tools/{lib.sh,snap.py,drive.sh,probe0.sh}`. Nothing
was torn down, and the one file written for the repository is this report.

<!-- end of report -->
