# verify-real — `r1-p3-doc-list-omits-stranded-doc`

Run `canary-one`, round 1, stage `test`, reporter
`verify-p3-r1-p3-doc-list-omits-stranded-doc`, attempt 1. One finding, door
`jigc doc list`, clause `working-product`, triage's grade *unclear*.

## Verdict in one paragraph

**REFUTED as a blocker — `breaks-no-clause`.** The behaviour is real and wider than
the finding says: over a doc a `docs-root` re-point left at its prior home,
`jigc doc list` exits 0 and prints four rows with no `research:context-loss`, and
`jigc doc list research` prints *no committed `research` docs* — in both formats,
**with the non-UTF-8 index entry, after it is removed, and on a strand that never had
one**. The entry has no bearing on the listing at all. And the previous release prints
the same bytes: every one of the 98 output files compared across the two binaries is
byte-identical, and so is every exit status. So nothing that works on `1.0.0-rc.24`
stopped working, and the listing is a success that prints no refusal and no route about
the strand — the one route it does print (the staged-listing note) runs as printed at
exit 0. Neither half of the second clause is broken. The row stays a row of the ledger:
whether the listing *owes* a knob-stranded doc a row is a design question no text rules
on (see *Left open*, item 1), and it is not mine to settle.

## The binary, asserted before anything was driven

The two calls the prompt spells, typed as they stand, each printing one line:

```text
{"act": "hash", "status": "hashed", "path": "bin/c1.a1/jigc", "bytes": 15795744, "content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc", "sha256": "2d69497a3ae508f3f6ad6737913308a8ddea060d89735de71357d037cf4ca575"}
{"act": "hash", "status": "hashed", "path": "bin/previous-91834b5e011d/jigc", "bytes": 15240064, "content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d", "sha256": "07a25005d6362085774a346fd39e2ca3524f4c2aa1f0d959a273e33dcb7efe4f"}
```

Both `content_sha256` values are the ones the prompt gives. With the candidate's
directory first on `PATH`, `command -v jigc` printed `<scratch>/bin/c1.a1/jigc`, exit 0.
Every driver below repeats that check before and after the rig's `eval`, for the binary
it drives, and holds the rig's `$JIGC` to the same path; a mismatch exits the driver
before any command runs. No build ran; nothing under `target/` was driven.

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label `c1`. Previous
release: `1.0.0-rc.24`. Host: Darwin arm64, git 2.54.0.

## What was read

- The finding: item 5 under `Left open` of
  `completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-doc-show-stranded-doc-not-found-route.a1.md`
  — and, because the item has no block and says only *that strand*, the parts of the
  same report that build the strand: its plant, its table of cells and the `setup`
  lines of its repro block. Nothing else of that report's reasoning was used, and no
  other report was read.
- Triage's one row for this key (the grade and what it asks to be re-driven).
- The run's opening record and `DECISIONS.md` → *2026-10-04 — The exit rule, revised*:
  the second clause, and its instrument as the second sharpening words it — *no command
  that works on rc.24 in a supported layout stops working, and every refusal's route
  works as printed*. The clause is in the closing condition.
- The design that owns the behaviour: `design/doc-read-surface.md` → *`jigc doc list` —
  the fourth read surface*; `design/validation.md` → *Orphan detection* and the fifth
  family's prior-home paragraph; `design/storage.md` → the root-knob rules.
- `crates/cli/src/doc.rs` → `run_list`, read only, to name the mechanism.

## What was driven

Eight rigs, each minted by `dev/jigc-rig refs-post-hoc --binary <path>` under
`<scratch>/verify-p3-list-strand.DGaojR`, none of them the reporter's. Every exit status
was read bare, the line after its command; stdout and stderr went to separate files.

The plant, as the source report gives it, needing no file on disk:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"     # exit 0

The eight reads taken at every stage: `jigc config get docs-root` ·
`git ls-files -- docs/research notes` · `jigc doc list` · `jigc doc list --format json` ·
`jigc doc list research` · `jigc doc list research --format json` ·
`jigc doc show research:context-loss` · `jigc validate`.

| rig | binary | sequence — the reads are taken at each named stage |
|---|---|---|
| `c-entry` | candidate | reads (*control*) · plant · `jigc config set docs-root notes --format json` · reads (*with-entry*) · remove the entry · reads (*entry-removed*) |
| `c-noentry` | candidate | reads (*control*) · `jigc config set docs-root notes --format json` · `git reset --hard` · reads (*no-entry*) · plant · reads (*entry-after*) |
| `c-plantonly` | candidate | reads (*control*) · plant · reads (*plant-only*) |
| `p-entry` · `p-noentry` · `p-plantonly` | previous | the same three |
| `c-route` · `p-route` | one each | plant · `config set` · `jigc doc list` · the route its stderr note prints, plain and json · remove the entry · the route again |

What I changed against the source: its cell D2 took one control read
(`jigc doc show`) before the plant; mine takes the eight reads there. They are reads,
and the *with-entry* stage's `config set` ack, `config get` line, `git ls-files` line
and `doc show` refusal came out as the source report prints them. The `c-plantonly`
and `c-noentry` sequences and the filtered listing are mine, added because the finding
says *not driven without the entry*.

### The strand stands, at every strand stage, on both binaries

- `jigc config get docs-root` — exit 0, `docs-root = notes  (project)`.
- `git ls-files -- docs/research notes` — exit 0, `docs/research/context-loss.md`.
- With the entry planted first, `config set` acks `"relocated": []` at exit 0, stderr
  empty: the knob lands and nothing moves. Without it, `config set` acks the move
  `docs/research/context-loss.md` → `notes/research/context-loss.md` and
  `git reset --hard` (exit 0) puts the doc back while the knob stays.

### The listing, at the four strand stages

*with-entry*, *entry-removed*, *no-entry*, *entry-after* — the four invocations, on both
binaries, all exit 0:

`jigc doc list`, stdout:

```text
id  path  state
changelog:changelog  CHANGELOG.md  managed
decisions-log:decisions-log  docs/decisions-log.md  managed
roadmap:roadmap  docs/roadmap.md  managed
vision:vision  VISION.md  managed
```

`jigc doc list --format json` — the same four rows, 869 bytes, no `research` id.

`jigc doc list research`, stdout: `jigc doc list — no committed `research` docs`.

`jigc doc list research --format json`, stdout: `{ "docs": [] }` (17 bytes, on three
lines).

stderr of the two unfiltered listings is one note, unchanged from the control — *docs are
also staged in open task ground-the-vision-in-research … list what it stages:
`jigc doc list --task ground-the-vision-in-research`*; stderr of the two filtered ones is
empty.

The control, on both binaries, before anything: five rows, the third
`research:context-loss  docs/research/context-loss.md  managed`; the filtered listing
prints that row; the json row carries `"state": "managed"`, `"title": "Context Loss"`.

### The comparisons, each by `cmp`, each status read bare

- **Candidate against previous:** 98 files — every listing's stdout and stderr, every
  `doc show` stdout and stderr, every `config set` ack, `config get` and `git ls-files`
  line, over the six rigs — **98 identical, 0 differing.** The exit column of all three
  sequences: identical (27, 27 and 17 lines). The eight `jigc validate` outputs:
  identical once each rig's own repository path is folded.
- **Across the strand stages:** each of the eight listing files (four invocations,
  stdout and stderr) at *with-entry* on the candidate against the same file at the three
  other stages on the candidate and at all four on the previous release — 56
  comparisons, all identical. The listing does not know whether the entry is there.
- **The entry alone:** *control* against *plant-only*, four listing stdouts, both
  binaries — 8 comparisons, all identical, five rows each. The entry removes no row.
- 64 `jigc doc list` invocations in the six rigs; 64 exit 0.

### The one route the listing prints, run as printed

`jigc doc list --task ground-the-vision-in-research`, over the strand, with the entry and
after its removal — exit 0 each, stderr empty, on both binaries, byte-identical (nine
files of the two route rigs compared, nine identical):

```text
id  path  state
commit:ground-the-vision-in-research  commit:ground-the-vision-in-research  managed
vision:vision  VISION.md  managed
```

## Is it what the finding says?

Yes, and more. The finding says *omits the stranded doc while the entry stands — exit 0,
four rows, no `research:context-loss` (candidate)*. Tried against each way it could be
wrong:

- *Stale state, or the reporter's rig* — six fresh rigs of my own; the same four rows.
- *An artefact of the non-UTF-8 entry* — no. The row is gone on a strand made by
  `config set` and `git reset --hard` with no such name anywhere, and it is present with
  the entry planted and no strand. The cause is the strand; the entry only happens to
  be how the source report made one.
- *One format only* — plain and json agree, filtered and unfiltered.
- *New on the candidate* — no. The previous release prints the same bytes.

So the finding's words *while the entry stands* understate it, and the fact it left
open — *not driven without the entry or on the previous release* — is now driven: the
same, both ways.

## What the design owes a committed doc outside every resolved home

Read from the design, not from the reporter:

- `design/doc-read-surface.md` → *`jigc doc list`* defines the listing's subject by its
  enumerator: *a row is emitted for every instance the enumerator yields*. The
  enumerator is the present-tense census (`committed_instances` — a doctype's resolved
  home) joined, since the M52 completion audit's fix 2, by *the homes the versioned
  snapshot store records the doctype having declared below its current version*. A doc a
  knob re-point left behind is at neither: its doctype's schema never changed, so the
  snapshot store records no prior home for it.
- The same section says what a knob strand is **not**: the `orphaned` state *is not the
  location orphan `file-state.orphaned-doc` names — there the doctype still resolves and
  only the home stranded*. `run_list` carries that out: it computes the strand set
  (`crate::orphan::orphaned_docs`) and uses it only to keep a strand **out** of the
  orphan rows. A strand therefore gets no row of either kind, by construction.
- `design/validation.md` gives the strand to another door: *Orphan detection* — a
  store-scope advisory of `jigc validate`, `file-state.orphaned-doc`, routed *re-point
  `docs-root` to cover it, or `jigc unmanage` it*; and the fifth family's prior-home
  paragraph assigns an at-version instance left behind to *the strand advisory*.
- `design/storage.md` → the root-knob rules treats the state itself as the config door's
  to prevent — the door *moves the committed docs the re-point would strand* — and names
  *`jigc doc list` then drops the re-rooted docs from the store surface* as the symptom
  of a knob that landed with no move.

So the design, as written, owes a knob-stranded doc **`jigc validate`'s advisory and no
listing row**. Driven, that holds where the store can still see the strand: at
*entry-removed*, `jigc validate` names `docs/research/context-loss.md` under
`file-state.orphaned-doc` and `jigc doc show` names the path and the repair.

I do not report this as `intended`, because no text decides it. The design never says *a
strand is deliberately unlisted*; the same section argues that a listing which *omits the
very file* an agent is being routed at *sends it straight to `cat`*, and it widened the
subject once already (M52) because the verb printed *no committed docs* over a corpus
holding a managed document another door could name. That is the state driven here:
`jigc doc list research` says *no committed `research` docs* while `jigc doc show`, one
command later, says the doc *is committed at `docs/research/context-loss.md`*. Whether
the subject widens again is a fork, recorded under *Left open*.

## Does it break `working-product`, inside that clause's scope?

No. The clause's measure has two halves, and the listing was held to each:

1. **No command that works on `1.0.0-rc.24` in a supported layout stops working.** The
   listing over the strand is byte-identical on the two binaries — stdout, stderr and
   exit, four invocations, four stages. Whatever one thinks of the four rows, the
   previous release prints them too. Nothing stopped.
2. **Every refusal's route works as printed.** The listing refuses nothing: exit 0, no
   finding, no `route:` line. The one command its stderr hands over — the staged
   listing — was run as printed and exits 0 with the task's staged surface, on both
   binaries.

This holds whichever way the open scope question about a tracked non-UTF-8 name is
ruled: the omission and its identity across the two binaries were driven on a strand
made by two ordinary commands with no such name.

## The regression fact

Not owed with this verdict. As a fact, since triage asked for both binaries: the block
runs on the previous release and gives the same exits and the same bytes.

## Class

**instance, unbounded.** Driven: one doctype (`research`), one doc, one knob
(`docs-root`), one fixture (`refs-post-hoc`), three ways into the strand (entry then
re-point; entry then re-point then entry removed; re-point then `git reset --hard`), two
formats, the filtered and unfiltered listing, two binaries, one host, one git. Not
derived: the `placement-root` strand, a hand-edited knob, a fresh clone of a repository
whose knob and docs disagree, any other located doctype, the `--task` arm beyond the
one route above. The consumers of `orphaned_docs` and of `committed_instances` were not
enumerated.

## Repro VR-LS-1

```yaml
claim: "over a doc a docs-root re-point left at its prior home, `jigc doc list` exits 0 and omits the doc — and that breaks the clause working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The omission reproduces, with and without the non-UTF-8 index entry, in both formats, and is byte-identical on the previous release."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
plant:                      # needs no file on disk; the name is passed as raw bytes
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
unplant:
  - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]                 # exit 0
cells:
  - cell: "the control — the doc is listed, and the entry alone removes no row"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <binary>
      - plant
    repro:
      - ["jigc", "doc", "list"]
      - ["jigc", "doc", "list", "research", "--format", "json"]
    expect:
      - exit: 0
        stdout: "id  path  state\nchangelog:changelog  CHANGELOG.md  managed\ndecisions-log:decisions-log  docs/decisions-log.md  managed\nresearch:context-loss  docs/research/context-loss.md  managed\nroadmap:roadmap  docs/roadmap.md  managed\nvision:vision  VISION.md  managed"
      - exit: 0
        stdout_json_contains: { "docs": [ { "id": "research:context-loss", "path": "docs/research/context-loss.md", "state": "managed" } ] }
    on-previous: "identical — every stdout compared byte for byte, and the same with no plant"
  - cell: "the finding — the strand with the entry standing (the source report's D2)"
    setup:
      - fixture: refs-post-hoc
      - plant
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]      # exit 0, relocated: []
    repro:
      - ["git", "ls-files", "--", "docs/research", "notes"]
      - ["jigc", "doc", "list"]
      - ["jigc", "doc", "list", "--format", "json"]
      - ["jigc", "doc", "list", "research"]
      - ["jigc", "doc", "list", "research", "--format", "json"]
    expect:
      - exit: 0
        stdout: "docs/research/context-loss.md"
      - exit: 0
        stdout: "id  path  state\nchangelog:changelog  CHANGELOG.md  managed\ndecisions-log:decisions-log  docs/decisions-log.md  managed\nroadmap:roadmap  docs/roadmap.md  managed\nvision:vision  VISION.md  managed"
        stderr_contains: "jigc doc list --task ground-the-vision-in-research"
      - exit: 0
        stdout_not_contains: "research:context-loss"       # four rows
      - exit: 0
        stdout: "jigc doc list — no committed `research` docs"
        stderr: ""
      - exit: 0
        stdout_json: { "docs": [] }
    on-previous: "identical — stdout and stderr of all four listings compared byte for byte"
  - cell: "the same strand, the entry removed"
    setup:
      - fixture: refs-post-hoc
      - plant
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]
      - unplant
    repro: "the five lines of the cell above"
    expect: "the five expectations of the cell above, byte for byte"
    then:
      - ["jigc", "validate"]            # exit 0; stdout contains file-state.orphaned-doc and docs/research/context-loss.md
    on-previous: "identical"
  - cell: "the strand with no undecodable name at any point — two ordinary commands"
    setup:
      - fixture: refs-post-hoc
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]      # exit 0, relocated: docs/research/context-loss.md -> notes/research/context-loss.md
      - ["git", "reset", "--hard"]                                                # exit 0
    repro: "the five lines of the finding's cell"
    expect: "the five expectations of the finding's cell, byte for byte"
    then: "plant, the five lines again: the same bytes"
    on-previous: "identical"
  - cell: "the one route the listing prints, run as printed over the strand"
    setup:
      - fixture: refs-post-hoc
      - plant
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]
    repro:
      - ["jigc", "doc", "list", "--task", "ground-the-vision-in-research"]
    expect:
      - exit: 0
        stdout: "id  path  state\ncommit:ground-the-vision-in-research  commit:ground-the-vision-in-research  managed\nvision:vision  VISION.md  managed"
        stderr: ""
    on-previous: "identical; and the same after unplant"
observed: "<scratch>/verify-p3-list-strand.DGaojR — c-entry, c-noentry, c-plantonly, c-route (candidate) and p-entry, p-noentry, p-plantonly, p-route (previous); the rigs are the eight jigc-rig-refs-post-hoc-* directories"
pinned-by: "UNPINNED: the strand cells hold in place a listing nobody has ruled on — see Pinnable; whether a suite asserts any of it was not checked, the finding making no coverage claim"
```

**Pinnable as it stands: no.** The first cell converts as it is — plain argv, exit 0, an
exact stdout — and pins a fact worth keeping: an index entry jigc cannot decode removes
no row from the listing. The fact the verdict rests on, *the candidate's listing over a
strand is the previous release's*, takes two binaries and is no single-binary test. The
three strand cells, pinned green, would fix the four-row listing and the line *no
committed `research` docs* in place before anyone has ruled whether the listing owes the
row; if that is ruled a defect, each is the fix's red test with its `expect` inverted.
The plant passes a name as raw bytes, so a test that uses it is Unix-only. The `date`
field of the control's json row is the day the rig was built, so that row is asserted by
containment, never whole.

## Left open

1. **Whether the listing owes a knob-stranded doc a row** — the fork under this
   finding. As written, `design/doc-read-surface.md` gives the listing the present-tense
   census plus the snapshot-recorded prior homes, and gives the knob strand to
   `jigc validate`. Driven, that leaves `jigc doc list research` saying *no committed
   `research` docs* in the state where `jigc doc show research:context-loss` says the doc
   *is committed at `docs/research/context-loss.md`* — two read surfaces, two accounts of
   one file, the shape the M52 widening was made to end for its own population. Both
   binaries. A design ruling, not a verdict.
2. **`jigc config set docs-root <value>` lands the knob and moves nothing when the index
   holds a name that is not UTF-8** — exit 0, `"relocated": []`, stderr empty, the doc
   still at its prior home. `design/storage.md` names a landed knob with no move as what
   the door's adjudication exists to prevent. Both binaries, byte-identical. Another
   door; seen as the setup of this block, not pursued.
3. **`jigc validate` is silent about the strand while the entry stands** — the
   *with-entry* sweep is byte-identical to the control's, one unrelated advisory, where
   the *entry-removed* sweep adds `file-state.orphaned-doc`. With item 2 this is what
   makes the listing's silence complete in that state: no read door names the doc but
   `jigc doc show`, and that one with the generic route. Both binaries. Not pursued; it
   may be a row of the ledger already under another key, which I did not read.
4. **`jigc validate` over the strand made by `git reset --hard` calls the doc *never
   adopted*** — `file-state.unregistered-doc`, *a basename coincidence or an un-ingested
   foreign doc, not a tracked strand*, routed at `jigc migrate <absolute path> --as
   research` — about a doc jigc managed until the reset, beside a `reconciliation.rename`
   advisory on the path it was moved to. The same on-disk strand reached through the
   entry gets `file-state.orphaned-doc` instead. Both binaries. The route was not run.

## Bounds — what this verification did not do

- One host (Darwin arm64, a volume that folds case — probed), one git (2.54.0). No
  Linux run.
- The `placement-root` strand, a hand-edited knob, a fresh clone and any doctype but
  `research` were not driven.
- No suite was read for coverage: the finding claims none.
- Nothing was built, edited, staged or committed. The tree is as it was handed over:
  branch `fix/canary-one` at `eeffe347`, the run's `r1/` directory untracked.

## Where the evidence is

`<scratch>/verify-p3-list-strand.DGaojR`: the drivers `drive.sh` (one rig, one sequence,
every exit read bare), `route.sh` and `compare.sh`. The six sequence directories
(`c-entry`, `c-noentry`, `c-plantonly`, `p-entry`, `p-noentry`, `p-plantonly`) each hold
`exits.log` and one numbered `.out` and `.err` per command. The two route directories
(`c-route`, `p-route`) hold one `.out` and `.err` per jigc command; their exits were
printed by the driver and are on no file — they are the seven `exit=0` lines per binary
this report states.

<!-- end of report -->
