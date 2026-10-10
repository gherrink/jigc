# verify-real — `r1-p3-orphaned-doc-placement-arm-and-json-not-driven`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: the
heading *Class* and item 6 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-orphaned-doc-route-after-strand-not-run.a1.md`
— the `placement:` arm of `file-state.orphaned-doc`, whose route nobody had run, and
`--format json` of every cell, which nobody had read. Door: `jigc validate`. Clause it is said
to break: `working-product`. Triage's grade: *unclear*. The finding carries no repro block.

## Verdict in one paragraph

**`refuted` — basis `does-not-reproduce`.** The state is real and I reached it from nothing on
both binaries, three ways: a `placement-root` strand of the two re-rootable singletons the rig
commits (`docs/roadmap.md`, `docs/decisions-log.md`). `jigc validate` exits 0 over it with one
`file-state.orphaned-doc` per doc, each in the placement arm's wording. **What does not
reproduce is anything that fails.** Every arm of that route, on its own fresh rig, exits 0 and
clears the advisory: the printed `jigc unmanage <path>`, extracted from the report and typed
through a shell exactly as printed — single-quoted by jigc itself where the path holds a space
— drops the doc; `jigc config set placement-root docs` re-covers it and the read comes back;
the hand move plus `jigc ingest` lands it at the resolved home, staged or committed. Under
`--format json` the precondition and every arm emit exactly one JSON document: 208 documents
across 32 cells, every one parsing, on stdout for an adjudication and on stderr with stdout
empty for the one reject in the set, which is what the stream discipline states. **The
candidate and the previous release agree on every cell** — thirteen of the sixteen pairs are
byte-identical once the rig's directory name and the commit ids are normalised, and the other
three differ in one line that belongs to another door. So no command that works on the
previous release has stopped working here, and no route fails as printed.

## The binary, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` —
  the hash the prompt gives for the candidate (commit
  `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` —
  the previous release's (1.0.0-rc.24). **It was driven**, because triage asked for both
  binaries.
- In every cell the driven binary's directory went first on `PATH`, and the driver stops
  unless `command -v jigc` prints that binary's path, unless the rig — built with
  `dev/jigc-rig committed-singletons --binary <that path>` — reports the same path as `$JIGC`,
  and unless the rig's repository lies under my own scratch directory. No cell stopped.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was driven

Thirty-two cells — sixteen per binary — each on its own fresh rig of the state
`committed-singletons`, minted under `<scratch>/verify-p3-placement.PvUOKT/cells/`. 1158
commands in all; every one whose exit status is read ran bare, its standard output and
standard error each to a file of its own; nothing was read through a pipe. Nothing of any
earlier reporter's scratch root was used.

**What I changed from the finding as handed.** It carries no block, so the whole setup is
mine, reconstructed as the smallest state that reaches the placement arm:

- The rig state is `committed-singletons`, not the earlier report's `refs-post-hoc`: the
  placement arm needs a baselined instance of a re-rootable `placement:` doctype and nothing
  else, and that state commits two (`roadmap`, `decisions-log`) with no live task beside them.
- Three strands, because *a placement-root strand* names no one way to make it:

| setup | how the strand comes about | cells per binary |
|---|---|---|
| `manifest` | `.jigc/config/manifest.yaml` written as `scalar:` / `  placement-root: notes` — the standing suite's way (`placement_override.rs`, `a_hand_edited_repoint_strands_the_committed_doc_and_no_door_reports_it_clean`); the door's move floor never runs | 10 |
| `plant` | the earlier report's mechanism, at this knob: a non-UTF-8 index entry planted, `jigc config set placement-root notes` run, the entry removed | 3 |
| `spacedir` | the docs first homed by the door under a root named `my docs` and committed, `jigc ingest`, then the manifest hand-written to `notes` — so the strand's path needs quoting | 3 |

- The `jigc unmanage …` commands are **not typed by me**: the driver reads every backticked
  `jigc unmanage …` span off the `route:` lines of the text report (and, separately, off every
  string of the JSON report) and runs each as `sh -c '<the span>'`, the bare `jigc` in it
  resolving through `PATH` to the binary under test.
- The route's other two arms name no argv, so they are spelled from its text: *re-point
  `placement-root` to cover where it sits* as `jigc config set placement-root docs` (the doc
  sits at `docs/roadmap.md`), and *move it to `notes/roadmap.md` and re-run `jigc ingest`* as
  `mkdir -p notes`, a `git mv` per doc, `jigc ingest`.

In the tables below a message is quoted without the backticks jigc prints around names; the
one block marked verbatim keeps them.

### The precondition (both binaries, identical text)

| step | exit | what it printed |
|---|---|---|
| `jigc config get placement-root` | 0 | `placement-root = notes  (project)` |
| `jigc validate` | 0 | two advisories, both `file-state.orphaned-doc`, verbatim below; trailer `2 finding(s) — report-only at store scope (exit 0)` |
| `jigc validate --format json` | 0 | one document on stdout: `findings` of two, each `code` `file-state.orphaned-doc`, `severity` `advisory`, `key.target` and `location.address` the stranded path, `message` and `route` the same two strings the text report prints; `report_only: true`, `scope: "store"`, `schema_version: 3` |
| `jigc doc list` | 0 | two rows — `changelog:changelog`, `vision:vision`; neither stranded singleton has one |
| `jigc doc list --format json` | 0 | one document on stdout, `docs` of the same two |
| `jigc doc show roadmap:roadmap` | 1 | `blocking · store.not-found — could not read roadmap:roadmap at notes/roadmap.md: No such file or directory (os error 2)`, on stderr |
| `jigc doc show roadmap:roadmap --format json` | 1 | stdout empty; one findings envelope on stderr, `code` `store.not-found` |
| `git ls-files -- docs notes` | 0 | `docs/decisions-log.md`, `docs/roadmap.md` |
| the file-state record's keys | - | `CHANGELOG.md`, `VISION.md`, `docs/decisions-log.md`, `docs/roadmap.md` |

The advisory, verbatim, the same on both binaries (its twin for `docs/decisions-log.md` differs
only in the names):

    advisory · file-state.orphaned-doc — committed doc `docs/roadmap.md` sits outside `roadmap`'s resolved home `notes/roadmap.md` — a `placement-root` change likely stranded it (the home moved, the committed instance did not)
      at: docs/roadmap.md
      route: move it to `notes/roadmap.md` and re-run `jigc ingest`, re-point `placement-root` to cover where it sits, or drop it with `jigc unmanage docs/roadmap.md`

Under `spacedir` the last span is printed as `jigc unmanage 'my docs/roadmap.md'` — in the text
report and in the JSON `route` string alike.

### The route's arms — what stands after each

One fresh rig per row and per binary; where a cell holds one value it is the value on both.
*Orphan advisory* is `file-state.orphaned-doc` in `jigc validate`, text and JSON.

| setup · arm | the arm, as run | arm's exit | orphan advisory | `jigc doc list` | `jigc doc show roadmap:roadmap` |
|---|---|---|---|---|---|
| manifest · none (control) | nothing | - | still there, twice | exit 0, two rows | exit 1, `store.not-found` |
| **manifest · unmanage** | the two printed commands, as printed | 0, 0 | **gone**; `file-state.unregistered-doc` for each path in its place | exit 0, two rows | exit 1, `store.not-found` |
| manifest · unmanage-json | the same, each with ` --format json` | 0, 0 | as *unmanage* | as *unmanage* | as *unmanage* |
| manifest · unmanage-twice | the *unmanage* arm, then both again, then both again with ` --format json` | 0 × 6 | as *unmanage* | as *unmanage* | as *unmanage* |
| **manifest · repoint** | `jigc config set placement-root docs` | 0 | **gone** | exit 0, **four rows**, both singletons `managed` at `docs/` | **exit 0**, the doc |
| manifest · repoint-json | the same with ` --format json` | 0 | gone | as *repoint* | as *repoint* |
| manifest · repoint-then-set | *repoint*, then `jigc config set placement-root notes` | 0, 0 | gone | exit 0, four rows, both at `notes/` | exit 0, the doc |
| **manifest · move-ingest** | `mkdir -p notes`, two `git mv`, `jigc ingest` | 0 × 4 | **gone** | exit 0, **four rows**, both `managed` at `notes/` | **exit 0**, the doc |
| manifest · move-ingest-json | the same, `jigc ingest --format json` | 0 × 4 | gone | as *move-ingest* | as *move-ingest* |
| manifest · move-commit-ingest | the moves, `git commit`, `jigc ingest` | 0 × 5 | gone | as *move-ingest* | as *move-ingest* |
| plant · none | nothing | - | still there, twice | two rows | exit 1 |
| plant · unmanage | the two printed commands | 0, 0 | gone; `unregistered-doc` | two rows | exit 1 |
| plant · repoint | `jigc config set placement-root docs` | 0 | gone | four rows | exit 0 |
| spacedir · none | nothing | - | still there, twice | two rows | exit 1 |
| **spacedir · unmanage** | `jigc unmanage 'my docs/decisions-log.md'`, `jigc unmanage 'my docs/roadmap.md'`, as printed | 0, 0 | gone; `unregistered-doc` | two rows | exit 1 |
| spacedir · unmanage-json | the same, each with ` --format json` | 0, 0 | as above | as above | as above |

What each arm printed:

- **unmanage.** `unmanaged docs/roadmap.md — dropped its file-state baseline; the file is left
  on disk`, stderr empty. Under `--format json`: `{"path": "docs/roadmap.md", "identity": null,
  "dropped": true}`. The file is on disk and tracked afterwards and the record's keys are
  `CHANGELOG.md`, `VISION.md`. A second run prints `no-op: docs/roadmap.md is not managed
  (nothing to drop)` at exit 0, and in JSON the same object with `"dropped": false`.
- **repoint.** `config: set placement-root = docs — written to .jigc/config/, uncommitted —
  commit it with your next commit`, stderr empty, no relocation line. In JSON: `{"committed":
  false, "key": "placement-root", "op": "config-set", "relocated": [], "value": "docs"}`.
- **repoint-then-set.** The second command's stderr carries `relocating the committed doc(s)
  stranded by the placement-root re-point to notes …` with `- docs/decisions-log.md →
  notes/decisions-log.md` and `- docs/roadmap.md → notes/roadmap.md`; `git status --porcelain`
  then shows both as `R`, and the record's keys are the `notes/` paths.
- **move-ingest.** `jigc ingest` prints `adoptable notes/roadmap.md → roadmap  (adopted —
  indexed + baselined, no file moved)` and the same for `notes/decisions-log.md`. In JSON: a
  `rows` array of seven, those two with `"verdict": "adoptable", "adopted": true`, and
  `"findings": []`.

**No byte moved or lost in any cell.** `git hash-object` printed
`f206c8c4f74abad710ad83aa3165cdf1d2cf9b7d` for the roadmap and
`cc4787596d1b7995ed905292e7c2ace235852b9c` for the decisions log at every one of the 192
readings, at whichever path each then sat. `git rev-parse HEAD` is the same before the strand
and after the arm in every cell but the ones where I committed myself (*move-commit-ingest*,
and the two setup commits of `spacedir`). No jigc command committed anything.

### `--format json`, the half the finding names

Every `--format json` command in the set — `validate`, `doc list`, `doc show`, `unmanage`,
`config set`, `ingest` — was read by the predicate
`design/command-output-contract.md` → *Stream discipline* gives a driver: parse stdout; if
stdout is empty, parse stderr. **208 documents, 158 on stdout and 50 on stderr, every one
parsing as exactly one JSON value**; the 50 are all `jigc doc show roadmap:roadmap --format
json` at exit 1 over the strand, a reject, whose row in that table is *stderr, exactly one
document; stdout empty*. The JSON report carries the route as one prose string, and the
`jigc unmanage …` spans read off it are the same strings the text report prints, quoting
included — the `unmanage-json` cells ran the JSON's spans, not the text's.

### Candidate against previous release

For each of the sixteen cell kinds the candidate's log and the previous release's were compared
whole after replacing the rig's directory name, the binary's directory and every commit id.
**Thirteen pairs: no difference at all** (456 to 577 lines each). The three `spacedir` pairs
differ in one line each, and it is not this door's: the setup's unrestricted `git ls-files`
lists a tracked `.jigc/settings-entries.json` on the candidate's rig that the previous
release's rig does not have — a file of `jigc setup`. It also shows the two labels ran two
different binaries. A control diff between two different cells of one binary is non-empty, so
the comparison is not vacuous.

## Is it what the finding says?

The finding says the placement arm's route was not run and its JSON not read. Run and read,
both hold. Against the design that owns the behaviour:

- `design/validation.md` → *Orphan detection* → *The M49 placement arm*: the detector keys on
  the declared home's remainder past its first component, and both tiers *word themselves on
  the matched doctype's current home shape*. The advisory names `placement-root` and the
  resolved home, never `docs-root`.
- → *M40 two-tier route*: a registered path is a strand routed at relocate / `jigc unmanage`;
  an unregistered one draws `file-state.unregistered-doc`, and *only the route is tiered,
  never the emission*. The *unmanage* arm ends exactly there — the orphan advisory gone, the
  unregistered one in its place, the read still at exit 1. Dropping the doc from the store is
  that arm's stated purpose, so the failing read afterwards is the arm having worked.
- → *Exit semantics*: both codes are report-only and outside `render::STORE_EXIT_FLIPS`; the
  sweep exits 0 in every cell.
- `design/command-output-contract.md` → *Stream discipline*: as above.

Nothing here argues that a settled decision is wrong, so nothing is contested.

## Does it break `working-product`, inside that clause's scope?

No — and because nothing fails, the basis is `does-not-reproduce`, not `breaks-no-clause`.

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is *a working product others can use and rely on*; its instrument, in the
  second sharpening, is *no command that works on rc.24 in a supported layout stops working,
  and every refusal's route works as printed*. The run's opening names it `working-product`.
- **First half — nothing stopped working.** Same exits, same output, same files on both
  binaries in every cell of this door.
- **Second half — the route works as printed.** `file-state.orphaned-doc` is an advisory at
  exit 0, not a refusal, so the sentence does not strictly reach it; held to it anyway, the
  one arm the route prints as a command runs as printed at exit 0 — with a plain path and with
  one that needs quoting — and the two it prints as prose do what their words say.

## The regression fact

**Not owed, and so not returned**: step 4 runs with `confirmed` only. What was established on
the way stands as evidence all the same — every cell was driven on the previous release too,
and it is green there exactly as on the candidate.

## Class

**Driven: the `placement:` arm of the one producer, at two docs — `instance, unbounded`.** The
advisory is built at one place, `crates/cli/src/cli.rs:1651-1683`, with two arms; this report
drove the second (`Home::Placement`), the earlier one the first. Within it: two of the three
re-rootable placement doctypes the design says ship (`roadmap`, `decisions-log`; the third was
not instantiated by the rig and I did not look for it), one new value of the knob (`notes`),
two values of where the doc sat (`docs`, `my docs`), three ways of making the strand, and two
of the three output formats (`agent`, `json`; `human` not driven). I did not enumerate the
mechanism's consumers.

## Coverage

The finding is a claim of the form *this was not driven*. Checked against the suites, never a
diff: `crates/cli/tests` (511 `.rs` files, read recursively) and `tooling-tests` (22) were
searched for the advisory's code; 6 files name it (`doc_show_relocated`, `flow52_acceptance`,
`orphan_detection`, `orphaned_instance`, `placement_override`, `unclaimed_file_family`). Two of
them build a placement strand and assert the advisory: `placement_override.rs:1020-1071` holds
the route's **text** (it contains the resolved home and `jigc unmanage`), and
`doc_show_relocated.rs:186-205` holds that the sweep reports it. Every line of both trees that
carries the quoted token `"unmanage"` was listed — 28 lines in 14 files, argv executions,
registry rows and three route-text negations among them; the one in those 6,
`placement_override.rs:882`, runs it on a doc **at** its resolved home, not on a strand. No
test in the 6 runs an arm of the placement route and reads the store back, and none reads
`file-state.orphaned-doc` out of `validate --format json` — the two `validate --format json`
reads in `unclaimed_file_family.rs` look up `schema-conformance.orphaned-instance`. The engine
crate has no `tests/` directory; the in-module tests were not enumerated. So the claim holds as
a statement about coverage: the route's text is held, its outcome and its JSON are held by
nothing — the block below is that test in waiting.

## Repro VR-PLACEMENT-1

```yaml
claim: "the placement arm of file-state.orphaned-doc prints a route that fails, or a --format json that does not hold, after a placement-root strand — breaking the clause working-product"
verdict: "REFUTED — does-not-reproduce. Every arm exits 0 and clears the advisory, and every --format json run emits exactly one parsing document, on the candidate and on the previous release alike."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
setup:                      # one fresh rig per arm
  - fixture: committed-singletons   # dev/jigc-rig committed-singletons --binary <the binary>
  - "write .jigc/config/manifest.yaml = `scalar:\n  placement-root: notes\n`"
precondition:
  repro:
    - ["jigc", "validate"]
    - ["jigc", "validate", "--format", "json"]
    - ["jigc", "doc", "list"]
    - ["jigc", "doc", "show", "roadmap:roadmap", "--format", "json"]
  expect:
    - exit: 0
      stdout_contains: "advisory · file-state.orphaned-doc — committed doc `docs/roadmap.md` sits outside `roadmap`'s resolved home `notes/roadmap.md` — a `placement-root` change likely stranded it"
      stdout_contains_also: "drop it with `jigc unmanage docs/roadmap.md`"
    - exit: 0
      stdout_json: { "report_only": true, "scope": "store", "findings": [ { "code": "file-state.orphaned-doc", "severity": "advisory", "key": { "code": "file-state.orphaned-doc", "target": "docs/decisions-log.md" } }, { "code": "file-state.orphaned-doc", "severity": "advisory", "key": { "code": "file-state.orphaned-doc", "target": "docs/roadmap.md" } } ] }
      stdout_json_route_contains: "drop it with `jigc unmanage docs/roadmap.md`"
    - exit: 0
      stdout_lacks: "roadmap:roadmap"
    - exit: 1
      stdout: ""
      stderr_json: { "findings": [ { "code": "store.not-found", "key": { "target": "roadmap:roadmap" } } ] }
arms:
  - arm: "drop it with the printed jigc unmanage <path> — the argv is the route's own backticked span"
    repro:
      - ["jigc", "unmanage", "docs/decisions-log.md"]
      - ["jigc", "unmanage", "docs/roadmap.md", "--format", "json"]
      - ["jigc", "validate", "--format", "json"]
      - ["git", "ls-files", "--", "docs"]
      - ["jigc", "unmanage", "docs/roadmap.md", "--format", "json"]
    expect:
      - exit: 0
        stdout_contains: "unmanaged docs/decisions-log.md — dropped its file-state baseline; the file is left on disk"
      - exit: 0
        stdout_json: { "path": "docs/roadmap.md", "identity": null, "dropped": true }
      - exit: 0
        stdout_lacks: "file-state.orphaned-doc"
        stdout_contains: "file-state.unregistered-doc"
      - exit: 0
        stdout: "docs/decisions-log.md\ndocs/roadmap.md"     # still tracked, still on disk
      - exit: 0
        stdout_json: { "path": "docs/roadmap.md", "identity": null, "dropped": false }
  - arm: "re-point placement-root to cover where it sits"
    repro:
      - ["jigc", "config", "set", "placement-root", "docs", "--format", "json"]
      - ["jigc", "doc", "list"]
      - ["jigc", "doc", "show", "roadmap:roadmap"]
      - ["jigc", "validate"]
    expect:
      - exit: 0
        stdout_json: { "op": "config-set", "key": "placement-root", "value": "docs", "relocated": [], "committed": false }
      - exit: 0
        stdout_contains: "roadmap:roadmap  docs/roadmap.md  managed"
      - exit: 0
      - exit: 0
        stdout_lacks: "file-state.orphaned-doc"
  - arm: "move it to the resolved home and re-run jigc ingest"
    repro:
      - ["mkdir", "-p", "notes"]
      - ["git", "mv", "docs/roadmap.md", "notes/roadmap.md"]
      - ["git", "mv", "docs/decisions-log.md", "notes/decisions-log.md"]
      - ["jigc", "ingest"]
      - ["jigc", "doc", "list"]
      - ["jigc", "doc", "show", "roadmap:roadmap"]
      - ["jigc", "validate"]
    expect:
      - exit: 0
      - exit: 0
      - exit: 0
      - exit: 0
        stdout_contains: "adoptable notes/roadmap.md → roadmap  (adopted — indexed + baselined, no file moved)"
      - exit: 0
        stdout_contains: "roadmap:roadmap  notes/roadmap.md  managed"
      - exit: 0
      - exit: 0
        stdout_lacks: "file-state.orphaned-doc"
quoting-arm:                # the same unmanage arm where the printed path needs a quote
  setup:
    - fixture: committed-singletons
    - ["jigc", "config", "set", "placement-root", "my docs"]     # the door moves both docs, staged
    - ["git", "commit", "-q", "-a", "-m", "home under a spaced root"]
    - ["git", "add", "-A"]
    - ["git", "commit", "-q", "-m", "the manifest", "--allow-empty"]
    - ["jigc", "ingest"]
    - "write .jigc/config/manifest.yaml = `scalar:\n  placement-root: notes\n`"
  repro:
    - ["jigc", "validate"]
    - ["sh", "-c", "jigc unmanage 'my docs/roadmap.md'"]      # the span as printed
    - ["jigc", "validate"]
  expect:
    - exit: 0
      stdout_contains: "drop it with `jigc unmanage 'my docs/roadmap.md'`"
    - exit: 0
      stdout_contains: "unmanaged my docs/roadmap.md — dropped its file-state baseline"
    - exit: 0
      stdout_lacks: "committed doc `my docs/roadmap.md` sits outside"
invariant: "in every arm `git hash-object` of each doc is unchanged at whichever path it sits, and no jigc command moves HEAD"
observed: "<scratch>/verify-p3-placement.PvUOKT — logs/cand.<setup>.<arm>.log and logs/prev.<setup>.<arm>.log for the sixteen cell kinds; norm/ holds the normalised pairs and their diffs"
pinned-by: "UNPINNED: no standing test runs an arm of the placement route and reads the store back, and none reads file-state.orphaned-doc out of validate --format json — see Coverage"
```

**Pinnable as it stands: yes.** The setup is one fixture state `trial_corpus.rs` already
builds plus one file write the standing suite already uses, every step is plain argv, and
every `expect` states behaviour this verdict wants to stay true. Three notes for whoever
converts it: the `stdout_json` objects list the keys that were read, not the whole documents
(the validate findings also carry `check`, `probe`, `location`, `message`, `route`); the
quoting arm's `sh -c` is there because the fact under test is that the printed span survives
a shell, which an argv vector would bypass; and the unmanage arm above is one sequence I
assembled from three driven cells (*unmanage*, *unmanage-json*, *unmanage-twice*) — each step
was driven, the sequence as one run was not.

## Left open

1. **`jigc config set placement-root notes` over a planted non-UTF-8 index entry exits 0,
   lands the knob, prints no relocation line and moves neither doc** — the `plant` setup, on
   both binaries. It is the mechanism of the earlier report's handed block, seen here at the
   second knob; whether the finding that owns that defect already counts `placement-root`
   among its doors I cannot say, having read no other report. Not pursued.
2. **After the hand move and `jigc ingest`, the file-state record keeps the two prior keys** —
   `docs/roadmap.md` and `docs/decisions-log.md` beside the two `notes/` keys, staged or
   committed — where the door's own move (*repoint-then-set*) leaves only the `notes/` keys.
   `jigc validate` says nothing about them, at exit 0 with the one unrelated advisory. The
   same on both binaries. Whether any door later reads those records was not driven.
3. **After the unmanage arm, `jigc doc show roadmap:roadmap` still says a re-point stranded
   the doc and offers `jigc unmanage docs/roadmap.md`** — by then the no-op the second run
   printed. Another door (`jigc doc show`); the same on both binaries; the placement twin of
   the earlier report's item 2. Not pursued.
4. **`file-state.unregistered-doc`'s own route after the unmanage arm** — `jigc migrate <the
   doc's absolute path> --as roadmap` (single-quoted where the path holds a space), or ignore
   it. Not run.
5. **`jigc unmanage` over a stranded doc reports `"identity": null`.** The rig's edge index is
   empty (`"edges": []`), so whether a stranded placement doc's edges are dropped with its
   baseline was not exercised.
6. **Nothing was finalized after any arm**, and the knob sits in an uncommitted manifest in
   every cell; a task's finalize after each arm was not driven.
7. **`.jigc/settings-entries.json` is tracked in the candidate's rig and absent from the
   previous release's** — the one line the two binaries' logs differ in. `jigc setup`'s door,
   not this one; I did not look at why.

## Bounds — what this verification did not do

- Two docs of two doctypes, one new root (`notes`), two old ones (`docs`, `my docs`); the
  formats `agent` and `json`, never `human`.
- A strand whose doc sits deeper than one directory, or at the repository root under
  `placement-root: .`, was not built.
- `jigc config set placement-root docs` leaves a project-layer value where the rig began with
  the pack default; clearing the override instead was not driven.
- The `plant` and `spacedir` setups ran three arms each, not all ten.
- The reject envelope `jigc doc show … --format json` writes to stderr ends without a newline
  on both binaries; it parses, and I did not hold it to anything.
- I read the handed report, the exit rule's entry, the run's opening, `design/validation.md`
  → *Orphan detection* through *Exit semantics*, the M49 rows of `design/storage.md` →
  *Placement*, and `design/command-output-contract.md` → *Stream discipline*. I read no other
  finding's report and no other verifier's.

## Where the evidence is

- `<scratch>/verify-p3-placement.PvUOKT/scripts/drive.sh` — the driver; `norm.py`, `heads.py`,
  `summ.py`, `summ2.py` — the readers.
- `<scratch>/verify-p3-placement.PvUOKT/logs/` — 32 logs, one per cell: every command, its
  exit status, its standard output and standard error, in order.
- `<scratch>/verify-p3-placement.PvUOKT/cells/<cell>/` — one file per stream per step,
  `printed.txt` and `printed.json.txt` (the spans the arm ran), and the rig itself.
- `<scratch>/verify-p3-placement.PvUOKT/norm/` — the normalised logs and the sixteen
  candidate-against-previous diffs.
- `<scratch>/verify-p3-placement.PvUOKT/explore/`, `runs/` and the one log beside them — my
  first exploratory rig and a first control cell under an earlier driver; counted nowhere
  above.

<!-- end of report -->
