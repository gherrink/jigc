# triage, pass 2 — run `canary-one`, round 1, the `test` stage (reporter `triage-p2`, attempt 1)

Status: **graded**. 60 findings in, from 11 reporters — every one a `left open` entry of an agent
of pass 1. 56 entries, all under keys minted here; 4 findings merged, each merge named below.
Nothing was driven, built, edited, staged or committed; the one file written is this report.

## The arithmetic

| reporter | findings in | entries | merged away |
|---|---|---|---|
| `verify-p1-r1-doc-list-unreadable-entry-fails-listing` | 6 | 6 | 0 |
| `verify-p1-r1-doc-list-staged-arm-unreadable-entry` | 5 | 5 | 0 |
| `verify-p1-r1-setup-installs-outside-work-tree` | 5 | 5 | 0 |
| `advocate-p1-r1-setup-installs-outside-work-tree` | 11 | 11 | 0 |
| `proposal-p1-r1-setup-installs-outside-work-tree` | 9 | 7 | 2 (left open 7, left open 8) |
| `verify-p1-r1-validate-non-utf8-path-drops-blocking-orphan` | 3 | 3 | 0 |
| `verify-p1-r1-doc-list-prints-id-doc-show-refuses` | 5 | 5 | 0 |
| `verify-p1-r1-non-utf8-path-other-orphan-walk-consumers` | 7 | 7 | 0 |
| `verify-p1-r1-finalize-may-commit-redirected-log-record` | 4 | 3 | 1 (left open 4) |
| `verify-p1-r1-uninstall-log-append-through-live-link` | 3 | 2 | 1 (left open 2) |
| `verify-p1-r1-scope-other-registries-unread-two-changed` | 2 | 2 | 0 |
| **total** | **60** | **56** | **4** |

60 in; 56 entries; 60 − 56 = 4 merged. New keys: 56. Found again: 0 (why, under *Three things
this pass could not do as its contract words them*).

By grade: `breaks` 0 · `unclear` 13 · `no-break` 25 · `out-of-scope` 0 · `needs-bound` 18.

## The merges, each named

1. **`proposal-p1-…` left open 7 → `r1-setup-typed-beside-bare-git-dir-installs-at-exit-0`**,
   the entry of the advocate's left open 8. One door (`jigc setup`), one behaviour: typed in the
   folder that holds a bare `.git`, it installs there at exit 0. The advocate drove it (its
   `Driven`, row 16), the independent drive drove it again (its row 23). Two reporters, one
   finding.
2. **`proposal-p1-…` left open 8 → `r1-unmanage-bare-error-or-false-no-file-no-work-tree-home`**,
   the entry of the advocate's left open 6. One door (`jigc unmanage`), one behaviour: exit 0
   saying no file was at a path where one is. The advocate has it in a submodule on the candidate
   and behind a bare `.git` on the previous release only; the independent drive adds the cell
   behind a bare `.git` on the candidate (its row 8).
3. **`verify-p1-r1-finalize-may-commit-redirected-log-record` left open 1** and
4. **the same reporter's left open 4**, both **→ `r1-log-writer-follows-link-at-log-path`**, the
   entry of `verify-p1-r1-uninstall-log-append-through-live-link` left open 2. All three are, in
   their reporters' own words, facts about one write — the invocation log's writer following a
   link at the log's path: its class (*every other verb's minting arm writes through the same
   live link*, driven at `jigc doc list`), its reach (a stage the user makes afterwards puts the
   records into a commit; *for the row that owns the write through the link*), and its scope
   (planted or healthy; *it belongs to F1's row*). One door, one behaviour, three facets.

**One finding that is not a merge and needs saying: `proposal-p1-…` left open 6.** It is one
finding in and one entry out. It names six behaviours of the candidate at six doors; five of the
six are the advocate's left open 1 to 5, each of which has its own entry here and carries this
finding as a second source. The sixth — `jigc doc list` printing *no committed docs* over a doc
the same binary just committed — is in no other finding handed to this pass, so the entry of
left open 6 is keyed to it: `r1-doc-list-no-committed-docs-where-home-is-no-work-tree`, at the
door it is about. Folding it into a `jigc task finalize` row would have moved a finding of the
round's one door to a door outside the round.

## What was read

- `completions/artifacts/canary-one/opening.md`, whole, and the entry it names as the closing
  condition: `DECISIONS.md` → *2026-10-04 — The exit rule, revised* — the four clauses, the
  scope of the first, the one instrument of each.
- The run's state, by `dev/stabilize-record state --run canary-one` (exit 0): the ledger holds
  **one** row, `canary-seeded-claim`, ungraded; **no bound is declared** (`bounds` is empty); the
  round's doors are **one included**, `jigc doc list`, and 47 excluded leaf verbs; every clause's
  status is `void`; `test_reports` is 24.
- The eleven reports the prompt lists, each whole, beside its list of left-open entries.
- The stage's report directory holds 24 files of attempt 1, and they are the 24 reporters the
  prompt names: no report is missing and none is extra.

## Three things this pass could not do as its contract words them

1. **`found again` could not be judged against the rows of pass 1.** Those rows are not on
   record yet — the ledger this pass was handed holds the seeded row and nothing else — and the
   report of the pass-1 triage was not handed to this pass and was not read. (One count was
   taken over that file and no line of it was read: each of the 56 keys below, as a fixed
   string, occurs in it 0 times. A key that collided would have been folded into the pass-1
   entry by the harness, with no trace.) The ten keys the verifiers' own reports name were not
   reused either: `implementation/stabilization-workflow.md` lists *a second triage pass that
   keys what was left open to a verified finding* among the endings that halt the stage after
   its instruments ran. **So every entry here is a new key, and some of them sit beside a row of
   pass 1 that this pass could not see.** They are named so that whoever holds both lists can
   fold them:
   - `r1-log-writer-follows-link-at-log-path` — the cross-cutting pass's `F1`, as two verifiers
     cite it (*the reporter's F1*, *F1's row*). Very likely one finding with that row.
   - `r1-doc-list-unreadable-entry-linux-not-driven` — *the reconciler's lead LD-3 (one
     platform)*, as its verifier cites it.
   - `r1-validate-clean-over-docs-stranded-by-root-knob` — the behaviour of
     `r1-validate-non-utf8-path-drops-blocking-orphan`, seen after another door.
   - `r1-setup-separate-git-dir-not-driven-by-verifier` — a variant the block of
     `r1-setup-installs-outside-work-tree` already names.
   - `r1-root-knob-lands-over-undecodable-git-listing` and
     `r1-orphan-walk-consumers-previous-release-not-driven` — the behaviour of
     `r1-non-utf8-path-other-orphan-walk-consumers`, read against another clause, and its
     missing regression fact.
   - `r1-route-clause-reading-refusal-without-route` and
     `r1-working-product-reading-false-green-of-validate` — the reading on which the verdicts of
     `r1-doc-list-unreadable-entry-fails-listing`, `r1-doc-list-staged-arm-unreadable-entry` and
     `r1-validate-non-utf8-path-drops-blocking-orphan` rest.
2. **A missing repro block is said here and in the entry's `why`, and is no entry.** The
   contract asks for an entry against the reporter's report; the harness holds entries plus
   named merges to the number of findings handed in, and the repair plan's `O1` settles it this
   way. A left-open entry is one line by its schema, so most came without a block of their own.
   Those graded `unclear` and lacking one: every `unclear` entry below except
   `r1-doc-show-stranded-doc-not-found-route` and
   `r1-orphan-walk-consumers-previous-release-not-driven`, whose cells are in `Repro VR-LD-2`.
   Entries graded otherwise say in their `why` or `repro` where they stand on a table, a
   transcript or a sentence and on no block.
3. **`out-of-scope` was not available.** The run declares no bound, so no finding could cite
   one. Everything this pass reads as outside a clause's scope is `needs-bound`, and goes to the
   human.

## How the grades were drawn — said so that each line can be overruled

A reporter's severity was not used. Five lines, applied to every entry:

- **A driven behaviour that hits a clause's words, in a state whose place in the clause's scope
  nobody has ruled → `needs-bound`.** This is the largest group, and the choice that most wants
  saying: the other candidate was `unclear`. `unclear` was not taken where the facts are already
  driven on both binaries — for the layouts whose home is no work tree, by the advocate and
  again by the independent drive — because what is open there is not a fact a verifier's run
  settles but the scope, which the reports themselves call *the human's call* and *not
  derivable from the record*. `breaks` was not taken for the same reason: it would have been a
  ruling on scope by triage.
- **A question about how a clause is read, which a verifier said its verdict rests on and the
  human can overrule → `needs-bound`.** Two rows. As this pass reads the clause — by its one
  instrument, as the verifiers did — the findings are outside it; that reading is nobody's
  ruling.
- **An undriven cell, an untraced route, or a missing regression fact, where a clause could be
  broken and a verifier holding the two binaries can find out → `unclear`.**
- **The same, where the cell hangs on a link somebody planted at the invocation log's path, or
  on reach no agent of this run has (another platform, another file system, a root caller) →
  `needs-bound`.**
- **A driven behaviour the report shows hitting neither measure of the second clause and
  neither verb of the first (identical on the previous release, no refusal that prints a route,
  nothing destroyed or committed); a sentence of a design doc, a record or a report; a pin
  note; a count; a fact about the advocate's proposal, which is not the candidate →
  `no-break`.** Still a row.

**Five `no-break` rows rest on the instrument reading that the two reading rows put to the
human.** If he reads the second clause wider, these re-grade with them:
`r1-validate-clean-over-store-doc-list-cannot-enumerate`,
`r1-validate-clean-over-docs-stranded-by-root-knob`,
`r1-doc-list-no-committed-docs-where-home-is-no-work-tree` (the round's own door),
`r1-milestone-create-fails-where-home-is-no-work-tree`,
`r1-finalize-ack-names-main-checkout-where-none-exists`.

**Four `no-break` rows carry something a reporter called the human's**, and are listed here
because `no-break` puts nothing on his list:
`r1-unregistered-row-prints-address-doc-show-refuses-design` (*a design fork*),
`r1-proposal-submodule-linked-worktree-home-is-a-choice` and
`r1-proposal-strands-open-task-under-old-home` (both ride the fork on
`r1-setup-installs-outside-work-tree`), and
`r1-regression-list-holds-no-row-for-two-ruled-differences` (whether the second clause's
instrument owes a row for a difference a ruling made).

## The entries

Quotes are the reports' words with their inner backticks and emphasis dropped. A `repro` names a
report by its file under `completions/artifacts/canary-one/r1/reports/test/` (the structured
return carries the whole path), then the heading or the block. Reporter names are shortened to
their role and key where the line would not fit.

### From `verify-p1-r1-doc-list-unreadable-entry-fails-listing` (6)

**`r1-doc-read-surface-silent-on-unreadable-instance`** — `no-break` · `inconsistency` · door `jigc doc list` · clause none
- source: left open 1
- repro: `verify-p1-r1-doc-list-unreadable-entry-fails-listing.a1.md` — `Left open — seen on the way, not pursued`, item 1; the behaviour is block `Repro V-1`
- why: a design sentence false of the binary ("A row is emitted for every instance the enumerator yields") and a state the section does not describe — a doc and binary inconsistency; no clause's instrument reads the design doc, and the behaviour itself is the row `r1-doc-list-unreadable-entry-fails-listing`.

**`r1-validate-clean-over-store-doc-list-cannot-enumerate`** — `no-break` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 2
- repro: the same report — block `Repro V-1` (its fifth `repro` line and `expect`), and `Left open`, item 2
- why: `jigc validate` exits 0 with *no findings* over the store the listing cannot enumerate, and on the previous release "Every exit status, every stdout and every other stderr is the same": nothing that works there stops working, and no refusal prints a route. Rests on the reading put to the human under `r1-working-product-reading-false-green-of-validate`.

**`r1-doc-read-surface-stale-index-line-citations`** — `no-break` · `inconsistency` · door: none named - design/doc-read-surface.md, the line citations of its `jigc doc list` section · clause none
- source: left open 3
- repro: the same report — `Left open`, item 3 (no block: a citation, not a behaviour)
- why: the section cites `index.rs:718-752` and `:724-731` for `committed_instances`, which stands at `crates/engine/src/index.rs:789-817` at eeffe347 — a stale citation in a design doc, and no clause.

**`r1-doc-list-unreadable-entry-linux-not-driven`** — `needs-bound` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 4
- repro: the same report — `Verdict` (the platform line) and `Left open`, item 4 (no block: nothing was driven)
- why: "Nothing here was driven on Linux: the binary handed over is a macOS binary and a verifier builds none" — one platform is the reach of every agent of this run, no bound declares it, and no verifier of this stage can drive the cell.

**`r1-route-clause-reading-refusal-without-route`** — `needs-bound` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 5
- repro: the same report — `Step 3 — does it break working-product, inside the clause's scope` (the paragraph *The reading this verdict rests on*), over block `Repro V-1`
- why: "If the sentence were read as every refusal has a route, and it works, all fourteen exit-1 cells would break it" — whether a refusal that prints no route is inside the second clause's measure is a boundary of the clause nobody has ruled, and the verifier states it so that the human can overrule it.

**`r1-doc-list-unreadable-entry-undriven-shapes-and-consumers`** — `unclear` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 6
- repro: the same report — `What was driven, and what was not` (the bullets *Not enumerated* and *Not driven*); no block
- why: no block — not driven: "a dangling link at a placement home", "the orphan-row read", "any other verb over the same state"; not enumerated: "the mechanism's other consumers". A consumer that writes over the same unreadable entry is not settled by the report. (The staged arm was driven by its own verifier.)

### From `verify-p1-r1-doc-list-staged-arm-unreadable-entry` (5)

**`r1-staged-note-route-exits-1-under-unreadable-staged-entry`** — `needs-bound` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 1
- repro: `verify-p1-r1-doc-list-staged-arm-unreadable-entry.a1.md` — block `Repro V-1`, the first `variants` line (labels T3-committed, T3-task), and `Left open`, item 1
- why: "it is a printed route whose command fails" — and "The note is not a refusal, the state is hand-made, and the previous release does the same": it hangs on an entry written by hand into the gitignored workbench, a state no declared bound covers.

**`r1-staging-area-writers-not-enumerated`** — `unclear` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 2
- repro: the same report — `Does ordinary use reach the state` (*Read*: not enumerated) and `Left open`, item 2; no block
- why: no block — the answer that only a hand reaches the state "rests on one primitive read and three attempts"; 46 hits of `instance_path(` were counted and not read, and a rename and a copy were "seen and not followed". Whether a jigc door can leave such an entry is not settled.

**`r1-doc-list-staged-arm-linux-and-root-not-driven`** — `needs-bound` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 3
- repro: the same report — `Platform, and how each cell was measured`, and `Left open`, item 3 (no block: nothing was driven)
- why: "Nothing was driven on Linux. The mode-000 cell depends on the caller not being root" — another platform and a root caller are outside what any agent of this run can drive, and no bound declares that reach.

**`r1-staged-doc-ids-other-callers-not-driven`** — `unclear` · `jigc-feedback` · door: jigc task discard and jigc uninstall (the other callers of staged_doc_ids) · clause `no-lost-files`
- source: left open 4
- repro: the same report — `Scope of what was verified`, and `Left open`, item 4; the plants are those of block `Repro V-1`; no block of its own
- why: no block — the enumerator answers an unreadable entry with an `Err` "on purpose, so that the doors which destroy a task area can fail closed", and those doors "were not driven under these plants". Whether they fail closed is not settled.

**`r1-doc-list-staged-arm-pin-limits`** — `no-break` · `jigc-feedback` · door `jigc doc list` · clause none
- source: left open 5
- repro: the same report — the paragraph *Pinnable as it stands*, under `Repro V-1`
- why: a note for whoever pins the block — its comparative half is the regression set's, and "the block pins what the binary does today, which a later repair of this row would change on purpose". No behaviour is claimed and no clause is touched.

### From `verify-p1-r1-setup-installs-outside-work-tree` (5)

**`r1-setup-outside-work-tree-exit-pinned-as-parity`** — `no-break` · `inconsistency` · door `jigc setup` · clause none
- source: left open 1
- repro: `verify-p1-r1-setup-installs-outside-work-tree.a1.md` — `The coverage claim — "UNPINNED"`
- why: "nothing pins this is wrong for the exit and the code, which a standing test holds as intended parity" (`setup_install_pathspec_guard::git_that_answers_is_never_refused_as_git_that_does_not`) — a correction of the reports' coverage claim, which a fix of the fork must know; no behaviour and no clause.

**`r1-setup-in-submodule-writes-superproject-hooks`** — `needs-bound` · `jigc-feedback` · door `jigc setup` · clause `no-lost-files`
- source: left open 2
- repro: the same report — block `Repro VR-1`, `variants` (the submodule), and `A submodule — candidate`
- why: "a setup typed in a submodule writing into the superproject's hooks — is a question of the first clause's shape, which the finding does not claim and I did not grade" — exit 0 on both binaries, the user's hook body kept byte for byte, so neither verb of the first sharpening by its letter, in a layout whose place in the scope nobody has ruled.

**`r1-superproject-hook-effect-not-established`** — `unclear` · `jigc-feedback` · door `jigc setup` · clause `no-lost-files`
- source: left open 3
- repro: the same report — `The finding's claims, one by one` (the row on the superproject's commits) and `Left open — noticed, not pursued`, third bullet; no block
- why: no block — "not established here — a commit in the superproject exits 0 and prints the user's line; I did not trace what the managed block ran". What the hook jigc wrote into the superproject does to that repository's commits is not settled.

**`r1-setup-install-hook-route-other-causes-unexamined`** — `unclear` · `jigc-feedback` · door `jigc setup` · clause `working-product`
- source: left open 4
- repro: the same report — `A worktree of a bare repository — candidate` (the paragraph *The route's precondition already holds, and the route still fails*) and `Left open`, fourth bullet; no block
- why: no block — "Whether the same route is printed, and is as wrong, for other causes of that code was not looked at". A refusal under `setup.install-hook` in an ordinary checkout whose route does not work as printed would break the second clause, and the report does not settle it.

**`r1-setup-separate-git-dir-not-driven-by-verifier`** — `no-break` · `jigc-feedback` · door `jigc setup` · clause `working-product`
- source: left open 5
- repro: the same report — `The class`, and `Left open`, fifth bullet; the layout is driven in `advocate-p1-r1-setup-installs-outside-work-tree.a1.md` — `What the fork did not have: these layouts are one doc deep` (the column `sep`) and in `proposal-p1-r1-setup-installs-outside-work-tree.a1.md` — `Driven — one row per next step`, row 7
- why: a statement of this verifier's coverage and no behaviour — the layout it left out was driven by the advocate and by the independent drive (`setup` exits 1 and 1, the same refusal, both binaries), and is a variant the row `r1-setup-installs-outside-work-tree` already carries, with its fork.

### From `advocate-p1-r1-setup-installs-outside-work-tree` (11)

**`r1-finalize-destroys-untracked-file-where-home-is-no-work-tree`** — `needs-bound` · `jigc-feedback` · door `jigc task finalize` · clause `no-lost-files`
- source: advocate, left open 1; proposal-driver, left open 6 (the loss, reproduced: its row 9)
- repro: `advocate-p1-r1-setup-installs-outside-work-tree.a1.md` — block `Repro AD-2`; reproduced in `proposal-p1-r1-setup-installs-outside-work-tree.a1.md` — `Driven — one row per next step`, row 9
- why: "an untracked file of the user's at a doc's home is destroyed at exit 0 ... in no file and in no git object afterwards", on both binaries, in all four layouts, reproduced by the independent drive — the first clause's own words. But "The first clause's scope sentence names four ordinary configurations and these are not among them", the record lists *the layouts whose home is no work tree* among bounds still to be written, and this run declares none. **The gravest row of this pass; it is the human's because of its scope, not because it is in doubt.**

**`r1-second-finalize-blocked-where-home-is-no-work-tree`** — `needs-bound` · `jigc-feedback` · door `jigc task finalize` · clause `working-product`
- source: advocate, left open 2; proposal-driver, left open 6 (reproduced: its rows 7 and 8)
- repro: the advocate's report — block `Repro AD-1`; the proposal-driver's report — `Driven`, rows 7 and 8
- why: the second doc's finalize "exits 3 reporting the first, committed doc as missing, and neither arm of its route leads out" — a refusal whose route fails as printed, on both binaries; whether these layouts are supported is the open row (D) of `implementation/decisions-pending.md`, the human's call, and no bound of this run says.

**`r1-validate-home-vacated-for-homes-never-occupied`** — `needs-bound` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: advocate, left open 3; proposal-driver, left open 6 (reproduced: its row 7)
- repro: the advocate's report — `What the fork did not have: these layouts are one doc deep` (the table's `jigc validate` row and the bullet on it); no block of its own; the proposal-driver's report — `Driven`, row 7
- why: "six blocking findings, five of them about files that were never there", each routed at restoring a document — routes that cannot be followed, identical on the previous release, in three of the layouts whose support nobody has ruled. No block of its own: a table row and quoted findings, reproduced by the independent drive.

**`r1-milestone-create-fails-where-home-is-no-work-tree`** — `no-break` · `jigc-feedback` · door `jigc milestone create` · clause `working-product`
- source: advocate, left open 4; proposal-driver, left open 6 (reproduced: its row 7)
- repro: the advocate's report — the same heading (the table's `jigc milestone create` row and its bullet); no block of its own; the proposal-driver's report — `Driven`, row 7
- why: exit 1 in all four layouts — "the previous release answers the same in every cell of this table" — in two of them with a bare git error, no code and no route: nothing that works on rc.24 stops working, and no printed route fails. Rests on the reading put to the human under `r1-route-clause-reading-refusal-without-route`; the layouts are the fork's.

**`r1-uninstall-refuses-and-force-half-removes-no-work-tree-home`** — `needs-bound` · `jigc-feedback` · door `jigc uninstall` · clause `working-product`
- source: advocate, left open 5; proposal-driver, left open 6 (reproduced: its row 7)
- repro: the advocate's report — the same heading (the table's `jigc uninstall` row and the bullet *jigc uninstall, with no task open*); no block of its own; the proposal-driver's report — `Driven`, row 7
- why: the refusal is routed at conditions "both of which already hold", and `jigc uninstall --force` is "exit 1 in bare and sep, at uninstall.remove-precommit, after .jigc/ is removed", its route looping — refusals whose routes fail as printed, on both binaries, in layouts whose support nobody has ruled. No block of its own.

**`r1-unmanage-bare-error-or-false-no-file-no-work-tree-home`** — `needs-bound` · `jigc-feedback` · door `jigc unmanage` · clause `working-product`
- source: advocate, left open 6; proposal-driver, left open 8 (merged: the same door and behaviour, behind a bare `.git` on the candidate — its row 8)
- repro: the advocate's report — block `Repro AD-1` (its second `repro` line, and `variants`); the proposal-driver's report — `Driven`, row 8
- why: the second arm of the blocked finalize's route — in two layouts "it exits 1 with one line and no code or route", and in a submodule and behind a bare `.git` it exits 0 saying there was no file "about a file that is on disk", un-managing a committed doc — on both binaries, in layouts whose support nobody has ruled.

**`r1-finalize-ack-names-main-checkout-where-none-exists`** — `no-break` · `jigc-feedback` · door `jigc task finalize` · clause none
- source: advocate, left open 7
- repro: the advocate's report — the same heading, the bullet *The finalize's own ack*; no block
- why: the ack says the commit is "not in the main checkout jigc's workbench binds to" where "There is no main checkout" — a false sentence in an exit-0 ack: wording, with no refusal, no route, and nothing written wrongly by it.

**`r1-setup-typed-beside-bare-git-dir-installs-at-exit-0`** — `needs-bound` · `jigc-feedback` · door `jigc setup` · clause `no-lost-files`
- source: advocate, left open 8; proposal-driver, left open 7 (merged: the same door and behaviour — its row 23)
- repro: the advocate's report — `Driven`, row 16, and `What the proposal does not do`, third bullet; the proposal-driver's report — `Driven`, row 23; no block
- why: `jigc setup` typed in the folder that holds a bare `.git` — "the standing directory itself no work tree — still installs there at exit 0", on the candidate and on the proposal alike: an exit-0 write where no work tree stands, neither verb of the first sharpening by its letter, in a posture nobody has ruled in or out. No block; the previous release is not stated for this cell.

**`r1-decisions-pending-d-row-true-one-doc-deep`** — `no-break` · `inconsistency` · door: none named - implementation/decisions-pending.md, the M57 list's row (D) on a worktree of a bare repository · clause none
- source: advocate, left open 9
- repro: the advocate's report — `The two stock rationalizations`, first bullet, and `Left open — not this fork, each a finding to triage`, item 9; over block `Repro AD-1`
- why: the row's correction of 2026-10-05 says the previous release creates and finalizes a doc in the three layouts — "That is true, and one doc deep". A record that owes a sentence, which the fork's ruling rewrites; no clause reads it.

**`r1-start-resolves-home-five-times`** — `no-break` · `jigc-feedback` · door `jigc start` · clause none
- source: advocate, left open 10 (a lead)
- repro: the advocate's report — `What the proposal does not do`, last bullet (counted with `GIT_TRACE`); the proposal-driver's report — `Driven`, row 24; no block
- why: a lead on cost — "The home is resolved five times in one jigc start on the candidate already". No clause of the closing condition measures the number of git calls.

**`r1-advocate-report-underclaims-doc-list-cell-behind-bare`** — `no-break` · `inconsistency` · door: none named - the advocate's own report · clause none
- source: advocate, left open 11 — in its structured return only; the report's own list has ten items
- repro: the advocate's report — `What the fork did not have: these layouts are one doc deep` (the sentence that excepts the `doc list` cell of `behind`); the cell is driven in the proposal-driver's report — `Driven`, row 25
- why: the report says the previous release's `doc list` cell behind a bare `.git` was not driven; the advocate's return says it was driven afterwards and answers the same, and the independent drive's row 25 shows it. A sentence of a report, corrected.

### From `proposal-p1-r1-setup-installs-outside-work-tree` (9 in, 7 entries)

**`r1-proposal-hook-inert-in-no-work-tree-layouts`** — `no-break` · `jigc-feedback` · door: the pre-commit hook `jigc setup` installs - on a build of the advocate's proposal, not on the candidate · clause none
- source: left open 1 (its finding PD-1)
- repro: `proposal-p1-r1-setup-installs-outside-work-tree.a1.md` — `Finding PD-1 — the hook the proposal installs is inert in three of its four layouts` (its block)
- why: "Not a regression against the candidate" — "The state is new with the proposal". It is why the independent drive of the fork on `r1-setup-installs-outside-work-tree` differs from the advocate's report; it is about a build that is nobody's candidate, and breaks no clause on the candidate.

**`r1-proposal-bare-worktrees-share-one-hooks-directory`** — `no-break` · `jigc-feedback` · door: none named - jigc uninstall, the ack of jigc setup and jigc task list, on a build of the advocate's proposal · clause none
- source: left open 2
- repro: the same report — `What else I found — about the proposal, none of it a step that failed`, item 1; no block
- why: "Each is a consequence of a home is a checkout the proposal states for the doc store and does not state for the hook or the task list" — about the proposal's build ("on the candidate all of them share the one, wrong, home"); it rides the fork and breaks nothing on the candidate.

**`r1-proposal-submodule-linked-worktree-home-is-a-choice`** — `no-break` · `jigc-feedback` · door: none named - the home of a user's linked worktree of a submodule, on a build of the advocate's proposal · clause none
- source: left open 3
- repro: the same report — the same heading, item 2, and `Driven`, row 22
- why: for a submodule git can name the first checkout, so the proposal's rule there "is a choice, not the absence of a source" — a fact the human needs when he rules the fork; it describes the proposal and no behaviour of the candidate.

**`r1-proposal-leaves-source-comments-stating-old-rule`** — `no-break` · `inconsistency` · door: none named - the doc comment of home_is_a_checkout (crates/cli/src/repo.rs) and InstallSubject::NoWorkTree (crates/cli/src/setup.rs), under the advocate's proposal · clause none
- source: left open 4
- repro: the same report — the same heading, item 3
- why: "Source comments the proposal leaves saying the old rule" — "(C) lists design docs and guides only". A gap in a proposal that is not applied; the comments are true of the candidate as it stands.

**`r1-proposal-strands-open-task-under-old-home`** — `no-break` · `jigc-feedback` · door: none named - jigc task list after the advocate's proposal, over an install the candidate wrote · clause none
- source: left open 5
- repro: the same report — `Driven`, row 18, and `Left open — not this proposal`, item 4
- why: "An adopter's open task under the old home is unreachable after the change" — "Whether a note is enough for prose somebody wrote is the human's to weigh". A cost of the proposal, driven on its build: nothing is destroyed ("every one of the old home's 20 files has the hash it had") and nothing of the candidate is touched; it rides the fork.

**`r1-doc-list-no-committed-docs-where-home-is-no-work-tree`** — `no-break` · `jigc-feedback` · door `jigc doc list` · clause `working-product`
- source: left open 6 — keyed to the one behaviour of it that no other finding carries; its five other behaviours are second sources of the five rows above that name it
- repro: the same report — `Driven — one row per next step`, rows 7 and 25; the advocate's report — block `Repro AD-1` (`expect`, the line `proj_wt`) and the table row *jigc doc list right after*
- why: `jigc doc list` prints *no committed docs* over a doc the same binary just committed, in all four layouts, "identical on both binaries" — exit 0, no refusal and no route, nothing written: neither measure of the second clause by its letter. Rests on the reading put to the human under `r1-working-product-reading-false-green-of-validate`; the layouts are the fork's.

**`r1-advocate-call-site-count-not-reproducible`** — `no-break` · `inconsistency` · door: none named - the advocate's report, the sentence on 36 call sites in 9 files · clause none
- source: left open 9
- repro: the same report — `What else I found — about the proposal, none of it a step that failed`, item 5
- why: a search for `jigc_home(` under `crates/cli/src` finds 17 lines in 5 files — "It is not a step and nothing rests on it". A count in a report.

(left open 7 and left open 8 are merges 1 and 2, above.)

### From `verify-p1-r1-validate-non-utf8-path-drops-blocking-orphan` (3)

**`r1-working-product-reading-false-green-of-validate`** — `needs-bound` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 1
- repro: `verify-p1-r1-validate-non-utf8-path-drops-blocking-orphan.a1.md` — `Verdict` (the bullet *The reading is mine*) and `Left open`, first bullet; over block `Repro RC-9v`
- why: "Whether rely on reaches a false green that the previous release prints too is the human's to say. If it does, this row is not refuted" — the verdict stands on reading the clause by its instrument's two limbs, a boundary of the clause only the human rules.

**`r1-non-utf8-tracked-path-supported-or-planted`** — `needs-bound` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 2
- repro: the same report — `Two facts that bound the finding's reach — driven, and narrower than the report says`, item 1, and `Left open`, second bullet; over block `Repro RC-9v`
- why: "Whether a tracked path whose name is not UTF-8 is a supported layout. Ruled nowhere I read, and under no declared bound of this run" — on the host driven the state exists only through git plumbing; on a file system that admits the name it is an ordinary tracked file, and that was not driven.

**`r1-validate-unreadable-orphan-variant-not-driven`** — `unclear` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 3
- repro: the same report — `Left open`, third bullet (no block: not driven); the variant is recorded, the verifier says, with `Repro RC-9` of `row-doc-list-reconciler.a1.md`, a report this pass was not handed
- why: no block — "chmod 000 docs/zzz/orphan.md in place of the index entry, which the reporter records as giving the same exit 0. It is a different mechanism": an ordinary accident and no plumbing, driven by no verifier and on no previous release.

### From `verify-p1-r1-doc-list-prints-id-doc-show-refuses` (5)

**`r1-doc-show-lowercase-address-prints-path-no-file-has`** — `unclear` · `jigc-feedback` · door `jigc doc show` · clause `working-product`
- source: left open 1
- repro: `verify-p1-r1-doc-list-prints-id-doc-show-refuses.a1.md` — `Left open — hit on the way, not pursued`, item 1 (no block: one cell of rig 2)
- why: no block — `jigc doc show research:upper` "exits 1 with store.unparseable, saying that research:upper at docs/research/upper.md does not parse; the only file there is UPPER.md": a refusal that names a path no file has, its route not run, the previous release not driven, on an ordinary case-insensitive volume.

**`r1-validate-calls-untracked-file-committed`** — `no-break` · `jigc-feedback` · door `jigc validate` · clause none
- source: left open 2
- repro: the same report — the same heading, item 2 (no block: rig 2, the `validate` row)
- why: `jigc validate` calls an untracked file a *committed file* — "Wording only, as far as I looked": one false word in a finding's sentence; the finding and its route are otherwise as the M50 decision has them.

**`r1-adoption-route-for-non-doc-id-name-not-run`** — `unclear` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 3
- repro: the same report — the same heading, item 3 (no block: rig 2, the rows `validate` and `ingest`)
- why: no block — the way out of the pair is the route `jigc migrate <path> --as research`, which `jigc validate` (exit 1) and `jigc ingest` both print, and "Whether that migrate lands such a file under an addressable name is undriven": a printed route nobody ran, in an ordinary state.

**`r1-unregistered-row-prints-address-doc-show-refuses-design`** — `no-break` · `inconsistency` · door `jigc doc list` · clause none
- source: left open 4
- repro: the same report — the same heading, item 4; over block `Repro V-1`
- why: "Under the M50 entry this is as decided; whether the later rule should reach this row is a design fork, not a verdict of mine" — two rules of `design/doc-read-surface.md` that pull apart on the `unregistered` row: a design question for the human, which breaks no clause as decided.

**`r1-doc-list-prints-id-coverage-claim-half-true`** — `no-break` · `inconsistency` · door `jigc doc list` · clause none
- source: left open 5
- repro: the same report — `Step 5 — the coverage claim`
- why: "Half true" — the two halves are held by `address_slug_head_axis`; the three steps in sequence, an untracked plant and a name with a space are held by no test found. A statement about the suites, with its method and its bound, and no behaviour.

### From `verify-p1-r1-non-utf8-path-other-orphan-walk-consumers` (7)

**`r1-root-knob-lands-over-undecodable-git-listing`** — `needs-bound` · `jigc-feedback` · door `jigc config set` · clause `no-lost-files`
- source: left open 1
- repro: `verify-p1-r1-non-utf8-path-other-orphan-walk-consumers.a1.md` — block `Repro VR-LD-2`, the doors `jigc config set docs-root` and `jigc config set placement-root`; and `Left open`, item 1
- why: "Both config set doors could not read git's listing and wrote the knob at exit 0", against the scope's *where jigc cannot tell it refuses before writing*; no byte was destroyed and nothing was committed, and "Two questions decide it and neither is a verifier's" — whether that sentence binds on its own, and whether a tracked name that is not UTF-8 is a healthy repository or a planted state. Under no declared bound.

**`r1-validate-clean-over-docs-stranded-by-root-knob`** — `no-break` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 2
- repro: the same report — block `Repro VR-LD-2`, door `jigc config set placement-root` (its third `repro` line; cell P2), and `Left open`, item 2
- why: *no findings* over a store two docs have dropped out of — "The walk it would find them with is the same emptied listing": the false green that `r1-validate-non-utf8-path-drops-blocking-orphan` was driven for on both binaries and found identical, seen after a second door; no refusal, no route. Rests on the reading put to the human under `r1-working-product-reading-false-green-of-validate`.

**`r1-doc-show-stranded-doc-not-found-route`** — `unclear` · `jigc-feedback` · door `jigc doc show` · clause `working-product`
- source: left open 3
- repro: the same report — block `Repro VR-LD-2`, door `jigc config set docs-root` (its fourth `repro` line: exit 1, `store.not-found`), and `Left open`, item 3
- why: a refusal whose route — create the referenced doc, or fix the reference — is printed for a doc that exists, at its prior home: "A sentence for a human; not run", and not driven on the previous release. Whether the route works as printed is not settled.

**`r1-orphaned-doc-route-after-strand-not-run`** — `unclear` · `jigc-feedback` · door `jigc validate` · clause `working-product`
- source: left open 4
- repro: the same report — `What stands afterwards (cells A1, A2)`, second bullet, and `Left open`, item 4; no block
- why: no block — once the entry is gone "the strand is seen, and nothing I ran moves it", and "That advisory's own route was not run": whether any printed route lands the stranded doc at the new root is not settled, and the stranded store needs no odd byte to stay stranded.

**`r1-orphan-walk-consumers-previous-release-not-driven`** — `unclear` · `jigc-feedback` · door: jigc config set and jigc relocate · clause `working-product`
- source: left open 5
- repro: the same report — block `Repro VR-LD-2`, and `The regression fact`
- why: the previous release's binary was not driven — "how these doors behave there is not known". A knob that lands over docs it left behind is a command that stopped working if rc.24 moved them, and that fact is a verifier's.

**`r1-non-utf8-path-reach-not-driven`** — `needs-bound` · `jigc-feedback` · door: jigc config set and jigc relocate · clause `no-lost-files`
- source: left open 6
- repro: the same report — `Does it break migration-works, inside that clause's scope?` (the bullet *This host could not hold such a file*) and `Left open`, item 6; no block
- why: on a filesystem that admits the name it is an ordinary committed file, and a clone on the host driven would hold the index entry without the file — "Neither was driven: one macOS host, one git". Reach no agent of this run has, under no declared bound.

**`r1-git-capture-other-callers-not-enumerated`** — `unclear` · `jigc-feedback` · door: none named - every caller of task::git_capture (crates/cli/src/task.rs), not enumerated · clause `no-lost-files`
- source: left open 7
- repro: the same report — `Class`, and `Left open`, item 7; no block
- why: no block — of every caller of `task::git_capture`, each of which shares the UTF-8 step, "I enumerated nothing and give no count". A writing door that reads an undecodable listing as empty and goes on is the first clause's *where jigc cannot tell it refuses before writing*, and nobody has looked.

### From `verify-p1-r1-finalize-may-commit-redirected-log-record` (4 in, 3 entries) and `verify-p1-r1-uninstall-log-append-through-live-link` (3 in, 2 entries)

**`r1-log-writer-follows-link-at-log-path`** — `needs-bound` · `jigc-feedback` · door `jigc doc list` · clause `no-lost-files`
- source: verify-p1-r1-uninstall-log-append-through-live-link, left open 2; verify-p1-r1-finalize-may-commit-redirected-log-record, left open 1 and left open 4 (both merged: the same write, its reach and its scope)
- repro: `verify-p1-r1-uninstall-log-append-through-live-link.a1.md` — `Control — a minting verb against the same link`; `verify-p1-r1-finalize-may-commit-redirected-log-record.a1.md` — `The control — the read is not blind, and the line reaches a commit only through a stage the user makes`, and `The verdict` (the paragraph *The clause's scope was not reached*)
- why: "the write through a live link is the log writer's behaviour for every verb with the knob on"; a stage the user then makes "puts the records then standing in the file into the task's commit, at exit 0"; and whether such a link is a healthy repository used as documented or a deliberately planted state was not reached — "a link at that gitignored path is placed by the operator", and no bound is declared.

**`r1-finalize-log-link-into-path-finalize-stages`** — `needs-bound` · `jigc-feedback` · door `jigc task finalize` · clause `no-lost-files`
- source: verify-p1-r1-finalize-may-commit-redirected-log-record, left open 2
- repro: that report — `Left open — not pursued`, item 2 (no block: not driven, not read)
- why: no block — a link at the log's path pointing at a path the door stages itself "was not driven and not read", and "Whether it is inside the clause's scope or a planted state is not mine to say": a link somebody aims at a path finalize stages, under no declared bound.

**`r1-log-link-other-committing-doors-not-driven`** — `needs-bound` · `jigc-feedback` · door: the other committing doors - jigc task amend, the doc-only finalize, the milestone doors, jigc setup, jigc rename, jigc migrate-corpus · clause `no-lost-files`
- source: verify-p1-r1-finalize-may-commit-redirected-log-record, left open 3
- repro: that report — `Left open — not pursued`, item 3 (no block: none driven)
- why: no block — "None was driven with the link in place. The design section itself says some of them commit without the carryover question": the same planted link at the log's path, at doors nobody drove, under no declared bound.

**`r1-uninstall-help-says-touches-nothing-outside`** — `no-break` · `jigc-feedback` · door `jigc uninstall` · clause none
- source: verify-p1-r1-uninstall-log-append-through-live-link, left open 1
- repro: `verify-p1-r1-uninstall-log-append-through-live-link.a1.md` — block `Repro V1` (the cells `plain` and `help`, `observed_not_asserted`) and `Against the claim, and against the design`, last bullet
- why: "it is a help-truth matter and breaks no clause of the closing condition as scoped" — no byte destroyed and nothing committed; the write itself is the row `r1-log-writer-follows-link-at-log-path`.

**`r1-uninstall-force-warning-overstates-link-loss`** — `no-break` · `jigc-feedback` · door `jigc uninstall` · clause none
- source: verify-p1-r1-uninstall-log-append-through-live-link, left open 3
- repro: that report — `Cell C — jigc uninstall --force`, and block `Repro V1`, the cell `forced`
- why: the forced run calls a link at the log's path the only copy of bytes that are not recoverable, where the link's target survives byte-identical — "Nothing is lost by it". A warning that overstates.

(`…finalize-may-commit…` left open 1 is carried by the first entry of this group, and its left open 4, with `…uninstall-log-append…` left open 2, are merges 3 and 4, above — three findings, one entry.)

### From `verify-p1-r1-scope-other-registries-unread-two-changed` (2)

**`r1-regression-list-holds-no-row-for-two-ruled-differences`** — `no-break` · `inconsistency` · door: none named - completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv, the second clause's instrument · clause `working-product`
- source: left open 1
- repro: `verify-p1-r1-scope-other-registries-unread-two-changed.a1.md` — `Left open`, item 1; the two differences are `Drive A` and `Drive B`
- why: `jigc doc create` from a user-made linked worktree — exit 0 on rc.24, exit 1 on the candidate — "is held by its ruling and by no row of the instrument": the difference is the human's decision of 2026-10-04, and the list's header owes a row only for a test of rc.24's own suite that goes red. Whether the instrument owes one is its owner's to say.

**`r1-promote-clobber-route-commits-staged-blob`** — `unclear` · `jigc-feedback` · door `jigc task finalize` · clause `no-lost-files`
- source: left open 2
- repro: the same report — `Drive C - the reworded note's behaviour, finalize.promote-clobber over a path git holds`, and `Left open`, item 2; no block of its own
- why: no block of its own — the refusal says the file git holds is left as it is "and that is no commit", and the route's own `jigc task finalize` then committed that staged blob, its summary listing the path under both `added` and `left-out`; "this may be exactly as intended; the two sentences were not reconciled against design/finalize.md here". Candidate only.

## To verify — the 13 keys graded `unclear`, and what each asks a verifier to re-drive

No entry is graded `breaks`. Each line below is the `redrive` of the structured return.

1. `r1-doc-list-unreadable-entry-undriven-shapes-and-consumers` — on both binaries, in fresh roots: `jigc doc list`, plain and `--format json`, with a dangling link at a placement home (after `jigc setup`, a link named `CHANGELOG.md` whose target is absent); the orphan-row read over an entry that cannot be read (a dangling link, a mode-000 file); then enumerate the other consumers of `engine::index::committed_instances` and of the read at `crates/cli/src/doc.rs:4866` and drive each door they reach over the same plant. Report any that writes, commits or destroys at exit 0, any exit that differs from the previous release, and every printed route run as printed. The staged arm is already driven.
2. `r1-staging-area-writers-not-enumerated` — read every writer into a task's staging area: the 46 textual hits of `instance_path(` across the two crates, the `std::fs::rename` at `crates/cli/src/doc.rs:3322` and the `std::fs::copy` at `crates/cli/src/milestone.rs:8553` (lines at eeffe347). For each that is not `write_atomic`: can it leave a link, or a file its owner cannot read, under ordinary use (a committed doc that is a link, a read-only or mode-000 committed doc, a strict umask, a fan-out copy)? Drive each candidate, and where one produces the state, the listing over it on both binaries.
3. `r1-staged-doc-ids-other-callers-not-driven` — fixture `refs-post-hoc`, its live task; with each plant of the staged-arm report's `Repro V-1` alone in the task's staging area (a dangling link named `research:dangling.md`; a mode-000 file named `research:locked.md`), on both binaries: `jigc task discard <id>`, `jigc uninstall` and `jigc uninstall --force`. Does each fail closed, or destroy the task area or the plant at exit 0; what does the ack list; is every printed route runnable as printed. A snapshot of `.jigc/` before and after each.
4. `r1-superproject-hook-effect-not-established` — the submodule layout of `Repro VR-1`'s variant (a superproject with a 41-byte `pre-commit` hook of its own), after `jigc setup` typed in the submodule, on both binaries: trace what the managed block does on a commit in the superproject (`GIT_TRACE` to a file, or the hook run under `sh -x`) — does it run `jigc validate`, against which home, with what exit; can it block or change a superproject commit (a staged `git mv` of a tracked file there); what does it print.
5. `r1-setup-install-hook-route-other-causes-unexamined` — enumerate the producers of `setup.install-hook` in `crates/cli/src/setup.rs` and the route each prints. For every cause reachable in an ordinary checkout (a read-only hooks directory; `pre-commit` a link; `core.hooksPath` naming a missing path, a file or an unwritable directory), run `jigc setup`, then the printed route as printed, on both binaries: does the route name the real cause, and does the re-run succeed once its condition is met.
6. `r1-validate-unreadable-orphan-variant-not-driven` — the setup of `Repro RC-9v` through its control (exit 1, a blocking `schema-conformance.orphaned-instance`), then `chmod 000 docs/zzz/orphan.md` in place of the index entry; `jigc validate`, plain and `--format json`, on both binaries, as a caller that is not root; then the mode restored, as a control.
7. `r1-doc-show-lowercase-address-prints-path-no-file-has` — fixture `fresh`; `docs/research/UPPER.md` holding one heading line, untracked and then committed. On both binaries: `jigc doc show research:upper`, then every command its route prints, as printed: what each writes and where, and whether a file is created, replaced or left under a second spelling. Say whether the volume is case-insensitive.
8. `r1-adoption-route-for-non-doc-id-name-not-run` — fixture `fresh`; `docs/research/UPPER.md` and `docs/research/has space.md`, untracked and then committed. On both binaries: `jigc validate` and `jigc ingest`, then the route each prints, as printed — `jigc migrate <path> --as research` for each file: does it land the file under an addressable name, refuse (with what route), or leave the listing as it was; `git status` before and after.
9. `r1-doc-show-stranded-doc-not-found-route` — `Repro VR-LD-2`, the door `jigc config set docs-root`, through its fourth `repro` line, on both binaries; then the refusal's route run as printed: does following it reach the doc that still stands at its prior home, or mint a second one. Say whether the stranded state can be reached without the non-UTF-8 index entry.
10. `r1-orphaned-doc-route-after-strand-not-run` — the same state after `git update-index --force-remove` of the entry (the report's cell A1), where `jigc validate` raises `file-state.orphaned-doc`; run both arms of that advisory's route as printed, each on a fresh rig, on both binaries — re-point `docs-root` to cover the doc, and `jigc unmanage` of its path — then `jigc doc list`, `jigc doc show research:context-loss` and `jigc validate`.
11. `r1-orphan-walk-consumers-previous-release-not-driven` — `Repro VR-LD-2` as written, all three doors in the three states (no plant, the ASCII plant, the 0xFF plant), on the previous release's binary: every exit, stdout, stderr and the index afterwards, cell for cell against the candidate's. The regression fact for the three doors.
12. `r1-git-capture-other-callers-not-enumerated` — enumerate every caller of `task::git_capture` (`crates/cli/src/task.rs:10087` at eeffe347), by door. For each caller on a door that writes, commits or removes, drive the door with the 0xFF index entry of `Repro VR-LD-2` and with its ASCII control, on both binaries: does a capture that cannot be decoded read as empty and let the door write, commit or destroy at exit 0, or does the door refuse before writing. `jigc task finalize` has a declared behaviour of its own for such a name (`DECISIONS.md`, 2026-09-17, M52 Increment 4 / T3).
13. `r1-promote-clobber-route-commits-staged-blob` — the report's Drive C, on both binaries: a `record-decision` task with an ADR titled *Second decision*; a file written at the ADR's home, added, then removed from disk; `jigc task finalize` (exit 3, `finalize.promote-clobber`); then the route as printed. Set the refusal's sentence about the file git holds against what the route's finalize commits (`git show --name-status HEAD`) and against the summary's `added` and `left-out` lists, and reconcile both with `design/finalize.md` → 4. Promote, *A home git holds is occupied*, and → *Dirty-tree policy*.

## For the human — the 18 `needs-bound` rows, by the question each waits on

The run declares no bound. Each group is one question; each row under it is a separate finding.

1. **The layouts whose home is no work tree** — a worktree beside a bare repository, behind a
   bare `.git`, a `--separate-git-dir` checkout, a submodule. The fork on
   `r1-setup-installs-outside-work-tree` is the same question, and row (D) of
   `implementation/decisions-pending.md` holds it open as his call. Seven rows:
   `r1-finalize-destroys-untracked-file-where-home-is-no-work-tree` (a loss at exit 0),
   `r1-second-finalize-blocked-where-home-is-no-work-tree`,
   `r1-validate-home-vacated-for-homes-never-occupied`,
   `r1-uninstall-refuses-and-force-half-removes-no-work-tree-home`,
   `r1-unmanage-bare-error-or-false-no-file-no-work-tree-home`,
   `r1-setup-in-submodule-writes-superproject-hooks`,
   `r1-setup-typed-beside-bare-git-dir-installs-at-exit-0`.
2. **A tracked path whose name is not UTF-8** — supported layout or planted state:
   `r1-non-utf8-tracked-path-supported-or-planted`,
   `r1-root-knob-lands-over-undecodable-git-listing`,
   `r1-non-utf8-path-reach-not-driven`.
3. **A link at the invocation log's path** — healthy repository or planted state:
   `r1-log-writer-follows-link-at-log-path`,
   `r1-finalize-log-link-into-path-finalize-stages`,
   `r1-log-link-other-committing-doors-not-driven`.
4. **An entry written by hand into a task's staging area**:
   `r1-staged-note-route-exits-1-under-unreadable-staged-entry`.
5. **One platform** — nothing in this run drives Linux or a root caller:
   `r1-doc-list-unreadable-entry-linux-not-driven`,
   `r1-doc-list-staged-arm-linux-and-root-not-driven`.
6. **How the second clause is read** — whether a refusal that prints no route, and a false green
   the previous release prints too, are inside it:
   `r1-route-clause-reading-refusal-without-route`,
   `r1-working-product-reading-false-green-of-validate`.

## Tree state

Branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short`
shows the one untracked directory `completions/artifacts/canary-one/r1/`, as before this pass.
No commit, no stage, no build, no binary driven, no rig. The record script was called for
`state` and for this report, and for nothing else.

<!-- end of report -->
