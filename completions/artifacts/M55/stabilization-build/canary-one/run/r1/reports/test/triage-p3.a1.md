# triage, pass 3 — run `canary-one`, round 1, stage `test`, attempt 1

Reporter `triage-p3`. Status: **graded**. 88 findings in, from 13 reporters; 73 entries; 15 merged, each
named below. Every entry is a new key: none of the 88 is the one row the ledger holds. I drove no binary,
built nothing, edited nothing in the repository and committed nothing; the one file I wrote is this report.

## What I read

- The opening record, `completions/artifacts/canary-one/opening.md`, and the closing condition it names by
  reference: `DECISIONS.md`, the entry *2026-10-04 — The exit rule, revised* — four clauses, two
  sharpenings, and the rule that a finding blocks only if it breaks a clause inside its scope.
- The run's state, by `dev/stabilize-record state --run canary-one` (exit 0): the ledger holds **one row**,
  `canary-seeded-claim`, ungraded; **no bound is declared** (`bounds: []`); round 1 has one included door,
  `jigc doc list`, and 47 excluded doors; the four clauses are void, none tested yet.
- The 13 reports the prompt hands over, each whole (the table below), beside each reporter's structured
  return as the prompt lists it.
- The stage's directory holds 38 reports of attempt 1, and the prompt lists 38 launched reporters; the two
  sets are the same by name. No report is missing, cut off or unlisted, so nothing here is a halt.
- I read no other reporter's report. In particular I did **not** read `triage-p1.a1.md` or
  `triage-p2.a1.md`: the prompt does not hand them over. What follows from that is under *Two limits of
  this pass*, and it matters for whoever records these rows.

| alias | reporter | report (under `completions/artifacts/canary-one/r1/reports/test/`) | in |
|---|---|---|---|
| A | `verify-p2-r1-doc-list-unreadable-entry-undriven-shapes-and-consumers` | `verify-p2-r1-doc-list-unreadable-entry-undriven-shapes-and-consumers.a1.md` | 11 |
| B | `verify-p2-r1-staging-area-writers-not-enumerated` | `verify-p2-r1-staging-area-writers-not-enumerated.a1.md` | 7 |
| C | `verify-p2-r1-staged-doc-ids-other-callers-not-driven` | `verify-p2-r1-staged-doc-ids-other-callers-not-driven.a1.md` | 6 |
| D | `verify-p2-r1-superproject-hook-effect-not-established` | `verify-p2-r1-superproject-hook-effect-not-established.a1.md` | 7 |
| E | `verify-p2-r1-setup-install-hook-route-other-causes-unexamined` | `verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md` | 8 |
| F | `verify-p2-r1-validate-unreadable-orphan-variant-not-driven` | `verify-p2-r1-validate-unreadable-orphan-variant-not-driven.a1.md` | 7 |
| G | `verify-p2-r1-doc-show-lowercase-address-prints-path-no-file-has` | `verify-p2-r1-doc-show-lowercase-address-prints-path-no-file-has.a1.md` | 5 |
| H | `verify-p2-r1-adoption-route-for-non-doc-id-name-not-run` | `verify-p2-r1-adoption-route-for-non-doc-id-name-not-run.a1.md` | 8 |
| I | `verify-p2-r1-doc-show-stranded-doc-not-found-route` | `verify-p2-r1-doc-show-stranded-doc-not-found-route.a1.md` | 7 |
| J | `verify-p2-r1-orphaned-doc-route-after-strand-not-run` | `verify-p2-r1-orphaned-doc-route-after-strand-not-run.a1.md` | 7 |
| K | `verify-p2-r1-orphan-walk-consumers-previous-release-not-driven` | `verify-p2-r1-orphan-walk-consumers-previous-release-not-driven.a1.md` | 6 |
| L | `verify-p2-r1-git-capture-other-callers-not-enumerated` | `verify-p2-r1-git-capture-other-callers-not-enumerated.a1.md` | 8 |
| M | `verify-p2-r1-promote-clobber-route-commits-staged-blob` | `verify-p2-r1-promote-clobber-route-commits-staged-blob.a1.md` | 1 |

A finding is written `A4`: reporter A, entry `left open 4` **of the structured return as the prompt lists
it**. Where a report's own *Left open* list numbers its items differently, the repro cell names the heading.

## The arithmetic

88 in (11 + 7 + 6 + 7 + 8 + 7 + 5 + 8 + 7 + 7 + 6 + 8 + 1). 73 entries. 88 − 73 = 15 merged.

| grade | entries |
|---|---|
| `breaks` | 2 |
| `unclear` | 36 |
| `no-break` | 20 |
| `needs-bound` | 15 |
| `out-of-scope` | 0 — no bound is declared, so none can be cited |

New 73, found again 0. To verify (`breaks` or `unclear`): 38.

## The merges — 15, each named

A merge is made only where the door and the broken behaviour are the same; the entry carries every source.

| folded | into | why they are one finding |
|---|---|---|
| B6 | B5 → `r1-p3-staging-area-writers-undriven-and-unread-remainder` | both are what the enumeration of the writers into a task's staging area left undriven or unread — one door (`jigc doc list`, its `--task` arm), one possible behaviour (a writer nobody drove leaves an entry the listing cannot read) |
| C5 | the same entry | "whether any jigc verb can leave a link or an unreadable file in a staging area" is that same question, asked by the neighbouring verifier; B's report answers it but for this remainder |
| D3 | D2 → `r1-p3-setup-in-submodule-no-backstop-install-elsewhere` | the ack's "local to this checkout" is a sentence of the same `jigc setup` typed in a submodule whose hook lands in the superproject's hooks directory |
| D5 | the same entry | "the install is in no commit and in no work tree" is the same `jigc setup`, the same misplaced install, seen from its files |
| E2 | E1 → `r1-p3-setup-writability-route-printed-for-other-causes` | a note on the pinnability of that entry's own block, `Repro VR-2` |
| E5 | the same entry | one more cause of the same producer and the same fixed route string (`core.hooksPath` under a read-only parent), which its reporter graded neither way |
| F2 | F1 → `r1-p3-validate-clean-over-unreadable-orphan-reading-and-layout` | two questions about one behaviour at one door: whether the clause's "rely on" reaches the false green, and whether the layout is supported |
| F4 | the same entry | the same door and mechanism: beside a readable orphan the unreadable one is still named nowhere |
| F5 | A5 → `r1-p3-doc-list-drops-unreadable-stamped-orphan-silently` | F5 says the listing under the mode-000 orphan was not driven; A5 is that drive, on both binaries — one door, one behaviour |
| F7 | the same entry | the second call site of `orphaned_instances(` that F7 says was not driven is the listing's, which A5 drove |
| H7 | H6 → `r1-p3-adoption-route-title-upper-undriven-cells` | both are undriven cells of one route under one title: another volume, the other binary |
| J4 | J5 → `r1-p3-finalize-after-orphaned-doc-route-arms-not-driven` | J4's undriven step is the author or finalize that baselines the moved doc; J5 is "a task finalize after each arm was not driven" — the same undriven door after the same arms |
| K2 | I1 → `r1-p3-doc-show-stranded-generic-route-non-utf8-layout` | K2 is the generic `store.not-found` route over the stranded doc, seen from the neighbouring door and "not run"; I's report ran it, and I1 is the scope question that result rests on |
| K4 | K1 → `r1-p3-root-knob-doors-non-utf8-name-no-lost-files-reading` | the layout question for the same three doors in the same state |
| L5 | the same entry | the same three doors read against the same clause, with the same added fact (identical on the previous release) |

One finding names two doors and is entered at one: **A5** says an unreadable stamped orphan is dropped from
`jigc doc list` at exit 0 *and* `jigc validate` stops naming it. Its entry stands at `jigc doc list`, the door
the item names first; the `jigc validate` half is the behaviour of F1's entry, whose `why` says so. Nothing
of A5 is in neither.

## How the grades were given

The closing condition's instruments, as the exit rule's entry words them: for `no-lost-files`, *in a healthy
repository used as documented … no jigc command at exit 0 destroys bytes no git object holds or commits
content the user did not ask for. Where jigc cannot tell … it refuses before writing*, with races and
deliberately planted states as declared bounds; for `working-product`, *no command that works on rc.24 in a
supported layout stops working, and every refusal's route works as printed*.

Four rules, applied to every entry, a reporter's severity never used:

1. **`breaks`** — the report establishes, with a block and on both binaries, behaviour that is the letter
   of a clause's measure in an ordinary layout.
2. **`unclear`** — it may break a clause and the report does not settle it: a cell nobody drove, a fact
   seen once or on one binary, or a reading its reporter left open in an ordinary layout. A finding that
   asserts product behaviour and came with no block is graded here and said to lack one.
3. **`no-break`** — taken at its word it touches neither measure of either clause in any layout: nothing is
   lost or committed unasked at exit 0, the two binaries agree as reported, and no printed route fails — or
   it is not about the product's behaviour at all (a reporter's own slip, a fact about a report).
4. **`needs-bound`** — a measure **is** touched as reported (a route that fails as printed, a false green,
   a write beside what it should have moved) or its reporter hands the reading to the human, and every
   instance sits in a state I read as outside the clause's scope: a file its owner cannot read, a name
   that is not UTF-8 planted by plumbing, a umask that masks the owner's read bit, a layout whose support
   `implementation/decisions-pending.md` holds open. The run declares no bound, and I infer none: these go
   to the human.

One reading I held throughout, because every verifier whose report I was handed held it and one of them
(A) names it as the human's to overrule: **a refusal that prints no route has no route that fails as
printed.** A loud, route-less refusal that is byte-identical on the previous release is therefore
`no-break` where its verifier graded it so on both binaries (A4, A6, B3), and `needs-bound` or `unclear`
where its verifier said outright that it was a `working-product` question it did not grade (I2, L6, L1).

## The entries

`RA` … `RM` are the reports of the table above, by alias. Doctype is `jigc-feedback` for every entry but one
(`r1-p3-adoption-report-table-names-rig-not-driven`, an `inconsistency` between a report's table and its own
drive). Every entry is new.

### From A — the unreadable entry at a managed home

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-ingest-route-fails-beside-unreadable-entry` | A1 | jigc validate | working-product | RA → `Repro V-2b` (setup-a, repro-a, control-a) | needs-bound | "the store sweep's `schema-conformance.orphaned-instance` route, exit 0 in the control and exit 1 beside an unreadable sibling … Under the other reading of the clause's second half this is a route that fails as printed" — only beside a file nobody may read, under no declared bound |
| `r1-p3-not-staged-route-task-less-read-refuses` | A2 | jigc doc show | working-product | RA → `Repro V-2b` (setup-b, repro-b, control-b) | unclear | "that read exits 1 for a readable file that is no managed doc … The first needs no unreadable entry at all" — a printed route whose command refuses in an ordinary state; its reporter left the reading open |
| `r1-p3-not-found-over-unreadable-file-routes-in-a-loop` | A3 | jigc doc show | working-product | RA → `Repro V-2b` (repro-b) | needs-bound | "`store.not-found` over a file that exists and cannot be read … with item 2 the two refusals route to each other" — a route loop that exists only over a mode-000 file |
| `r1-p3-unreadable-entry-stops-commit-boundaries-no-route` | A4 | jigc task validate, jigc task finalize and jigc milestone finalize | working-product | RA → `Repro V-2` | no-break | "Every cell that exits 1 over a plant on the candidate exits 1 with the same bytes on rc.24 … The refusals … print no route, so none fails as printed" — graded so by its verifier over 277 paired cells; nothing lost |
| `r1-p3-doc-list-drops-unreadable-stamped-orphan-silently` | A5, F5, F7 | jigc doc list | working-product | RA → `O, P — the orphan-row read and the prior-home rows (fresh roots)`, and `Repro V-2` (`shapes-that-do-not-fail`) | needs-bound | "`old.md` set to mode 000: 0 — one row, `kept.md`; `old.md` is gone from the listing" on both binaries — a silent omission at exit 0 over a tracked file the caller cannot read; whether that layout is supported is ruled nowhere and under no declared bound |
| `r1-p3-validate-two-postures-over-unreadable-entry` | A6 | jigc validate | working-product | RA → `B, X, Y — every door over the plant`, the `jigc validate` row | no-break | "silent over the link, exit 1 over the file — because its code-anchor walk keeps a directory read of its own" — two postures, one route-less refusal, the same on both binaries |
| `r1-p3-unreadable-entry-class-door-list-traced-by-hand` | A7 | unlisted: every door an unreadable `*.md` at a managed home reaches (the reporter names two read sites, `crates/engine/src/target_surface.rs:319` and `crates/cli/src/ingest.rs:991`, and no single verb) | working-product | RA → `The consumers — enumerated, and what each does with an entry it cannot read` — no block | unclear | "the **doors** they reach were derived by tracing callers by hand, which no registry fences" — what a door outside the hand trace does is not settled; lacks a repro block |
| `r1-p3-refusing-doors-leave-own-cache-files-at-exit-1` | A8 | jigc ingest, jigc task validate and jigc milestone finalize | no-lost-files | RA → `Does any door write, commit or destroy at exit 0`, last bullet | no-break | "What a refusing door leaves behind is jigc's own and ignored" — at exit 1, rebuildable, none of the user's bytes |
| `r1-p3-finalize-left-out-hint-over-unreadable-file` | A9 | jigc task finalize | working-product | RA → `Left open — seen on the way, not pursued`, item 8 — no block | no-break | "git's refusal, under a hint that does not expect it" — an exit-0 summary heading, no refusal and no route of jigc's |
| `r1-p3-unreadable-entry-undriven-platform-shapes-and-doors` | A10 | unlisted: the doors of the unreadable-entry class on Linux, with a directory or a named pipe as the entry, at a code-anchor doctype's home, over a tracked managed doc, through the `pre-commit` hook, and `jigc migrate-corpus` and `jigc relocate` over a corpus that folds | working-product | RA → `What was driven, and what was not` (*Not driven*, *Not established*) — no block | unclear | "Not driven: Linux. A directory named `x.md` and a named pipe at any door but the listing … A tracked managed doc made unreadable" — nothing says what those cells do; lacks a repro block |
| `r1-p3-reporter-slip-helper-scripts-by-file-tool` | A11 | none: the stage's reporter contract (the file tool writes the report alone) | none | RA → `A slip of mine, said plainly` | no-break | "None of them is in the repository or the run's directory" — a reporter's process slip, disclosed; no behaviour of the product |

### From B — the writers into a task's staging area

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-umask-masking-owner-read-unreadable-staged-doc` | B1 | jigc doc list | working-product | RB → `Left open`, item 1 (labels Z.1 to Z2.listj) — no block | needs-bound | "A umask that masks the owner's own read bit reaches the state through jigc's verbs alone — declared here as a boundary, not as ordinary use" — a bound is the human's to declare, and none is; `jigc doc create` and `jigc doc set-slot` are the doors that leave the state |
| `r1-p3-doc-list-tracked-dangling-link-raw-os-error` | B2 | jigc doc list | working-product | RB → `Left open`, item 2 (label K4.5-committed) — no block | unclear | "exits 1, stdout empty … a raw OS error, no route, the machine's absolute path … One cell, recorded as hit; nothing further was driven" — a tracked link whose target moved is no planted state on its face; lacks a repro block |
| `r1-p3-copy-in-doors-raw-os-error-unreadable-committed-doc` | B3 | jigc doc create and jigc doc set-slot | working-product | RB → `Repro W-1` (variant "the committed home mode 000") | no-break | "both doors exit 1 with `Permission denied (os error 13)`; nothing is staged and no temp file is left" — a refusal before any write, route-less, the same exit on both binaries |
| `r1-p3-milestone-merged-area-writer-follows-a-link` | B4 | unlisted: the milestone merged area's writer, `crates/engine/src/milestone.rs:2323` (the reporter names no verb) | no-lost-files | RB → `Left open`, item 4 — no block | unclear | "writes each merged body with `std::fs::write`, which follows a link at its destination … Read, not driven with a plant" — whether anything can leave a link there, and where the write then lands, is not settled; lacks a repro block |
| `r1-p3-staging-area-writers-undriven-and-unread-remainder` | B5, B6, C5 | jigc doc list | working-product | RB → `The enumeration` (*The bound of this enumeration*) and `Left open`, items 5 and 6 — no block | unclear | "A write reached through a spelling none of those patterns match would be missed", the spawned `git` call sites "were not each read", and "Not driven: Linux; a root caller; a fan-out that lands; `jigc migrate`'s authoring path and a re-opened (amend) task"; lacks a repro block |
| `r1-p3-reporter-slip-driver-by-file-tool-report-patched` | B7 | none: the stage's reporter contract | none | RB → `Platform, and how each cell was measured` (*Two things about the tooling*) | no-break | "It is a scratch file, in no repository" — a reporter's tooling deviation, disclosed; no behaviour of the product |

### From C — the other callers of `staged_doc_ids`

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-staged-prose-refusal-read-route-says-not-staged` | C1 | jigc task discard and jigc uninstall | working-product | RC → `The printed routes, run as printed`, first row; `Repro V-2` (variant mode-000) | needs-bound | "The refusal says the task stages it; the read it routes at says it does not" — a printed route whose command fails, over a mode-000 file made by hand in the workbench, under no declared bound |
| `r1-p3-task-finalize-raw-os-error-unreadable-staged-file` | C2 | jigc task finalize | working-product | RC → `The printed routes, run as printed`, third row | needs-bound | "a raw OS error, the machine's absolute path, no route … It is a route both staged-prose refusals print" — the route's command fails, in the same hand-made state |
| `r1-p3-forced-doors-name-no-staged-docs-under-dangling-link` | C3 | jigc task discard and jigc uninstall | no-lost-files | RC → `Repro V-2` (expectations 3 and 4) | no-break | "The bytes are destroyed by consent … What is wrong is what a surface says, not what a command destroys" — the forced doors take what `--force` consents to, as the design names it; the false sentence at `uninstall --force` is candidate-only wording |
| `r1-p3-staged-doc-ids-orientation-and-milestone-callers` | C4 | unlisted: jigc start (orientation's staged list, `orient.rs:212`) and the milestone door (`milestone.rs:6099`, `milestone.rs:5897`) | no-lost-files | RC → `Scope of what was verified` — no block | unclear | "`orient.rs:212` was not driven, and … the milestone door's two … were not driven either. The class is not bounded here"; lacks a repro block |
| `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` | C6 | jigc task discard and jigc uninstall | no-lost-files | RC → `Left open`, item 6 — no block | unclear | "Linux, and a root caller, were not driven" — the fail-closed cells stand for one platform and one uid; lacks a repro block |

### From D — the superproject's hook

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-superproject-with-own-install-shared-hook-not-driven` | D1 | jigc setup and jigc uninstall | working-product | RD → `Left open — noticed, not pursued`, first bullet — no block | unclear | "a `jigc uninstall` typed in the submodule would be removing a block of the superproject's hook. Whether either leaves the superproject without its own backstop … is not established"; lacks a repro block |
| `r1-p3-setup-in-submodule-no-backstop-install-elsewhere` | D2, D3, D5 | jigc setup | working-product | RD → `Repro VR-2` (its setup, through `jigc setup`) and `Left open — noticed, not pursued`, bullets 2, 3 and 5 | needs-bound | "`super/.git/modules/vendor/lib/hooks` holds no `pre-commit` after `setup` on either binary, while the ack lists a `pre-commit` hook … as installed" — whether a submodule is a supported layout is the row (D) the record holds open for the human, and no bound of this run says |
| `r1-p3-validate-in-submodule-runs-at-git-dir` | D4 | jigc validate | working-product | RD → `Left open — noticed, not pursued`, fourth bullet — no block | needs-bound | "exits 0 with no findings, its git children running at `super/.git/modules/vendor`, a directory that is no work tree. Row (D) already records that store reads are taken at the wrong place in this layout" |
| `r1-p3-hook-in-another-repository-layouts-not-enumerated` | D6 | jigc setup | no-lost-files | RD → `The class` — no block | unclear | "I did not enumerate the layouts in which a hook lands in a repository other than the one `setup` was typed in … where that fact does not hold, nothing here speaks"; lacks a repro block |
| `r1-p3-superproject-hook-block-unpinned` | D7 | jigc setup | no-lost-files | RD → `Coverage`, and the `pinned-by` of `Repro VR-2` | no-break | "None commits in a superproject after a `setup` typed in its submodule" — a refuted block that no suite holds; a fact about coverage, no behaviour of the product |

### From E — the `setup.install-hook` routes

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-setup-writability-route-printed-for-other-causes` | E1, E2, E5 | jigc setup | working-product | RE → `Repro VR-2 — the writability route where the hooks directory is already writable` | breaks | "The route's condition holds before the first run, the re-run exits 1 with the same bytes, and the door recovers only by an act the route does not name" — in a plain checkout, with a block, red the same way on both binaries (its reporter: `regression: false`, a fact the pass-2 row's one field could not carry) |
| `r1-p3-uninstall-writability-route-not-driven` | E3 | jigc uninstall | working-product | RE → `Left open — noticed, not pursued`, first bullet — no block | unclear | "`jigc uninstall` prints the same writability sentence under `uninstall.remove-precommit` … Not driven"; lacks a repro block |
| `r1-p3-display-hook-path-other-callers-not-driven` | E4 | jigc setup and jigc uninstall | working-product | RE → `Left open — noticed, not pursued`, second bullet — no block | unclear | "the warning tells its reader to *delete the file yourself* at whatever path that function prints" — a path that is the link's end where the hook is a link; not driven; lacks a repro block |
| `r1-p3-setup-hook-refusal-after-install-files-written` | E6 | jigc setup | no-lost-files | RE → `Causes where the route is right, or where there is no refusal`, the paragraph under the table | no-break | "the first run leaves the install's files written and uncommitted … and the re-run commits them" — at exit 1, on both binaries, nothing destroyed and nothing committed |
| `r1-p3-previous-release-wrote-through-hook-link` | E7 | jigc setup | no-lost-files | RE → `Repro VR-1` (`on-previous-release`) | no-break | "Both are closed on the candidate by P3; recorded here only because the regression fact above rests on that exit 0" — a fact about the previous release that the candidate refuses |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | E8 | jigc setup | working-product | RE → `The class` — no block | unclear | "Not driven: P1; … `make_executable` failing; a hook that is a directory or unreadable; an outward link written relatively; `--force`; the route as `--format json` renders it"; lacks a repro block |

### From F — the unreadable orphan at the store sweep

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-validate-clean-over-unreadable-orphan-reading-and-layout` | F1, F2, F4 | jigc validate | working-product | RF → `Repro RC-9u — the block, in the pipeline's schema` | needs-bound | "Whether *rely on* reaches a false green that the previous release prints too is the human's to say" and "Whether a tracked file the caller cannot read is a supported layout. Ruled nowhere I read, and under no declared bound of this run" (A5 reports the same silence at this door from the listing's side) |
| `r1-p3-validate-clean-over-orphan-absent-from-work-tree` | F3 | jigc validate | working-product | RF → `Left open`, third bullet — no block | unclear | "With the orphan moved out of the work tree … `jigc validate` on the candidate exits 0 … It was not driven on the previous release" — an ordinary state, one binary; lacks a repro block |
| `r1-p3-validate-unreadable-orphan-platforms-and-variants` | F6 | jigc validate | working-product | RF → `Bounds — what this verification did not do` — no block | unclear | "Nothing on Linux, nothing as root, no ACL or ownership variant of *unreadable*, no unreadable parent directory"; lacks a repro block |

### From G — the lower-cased address

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` | G1 | jigc migrate | working-product | RG → `Repro V-1` (the arm *committed*, steps 2 to 4) | breaks | "*git holds no copy of it* is said of a file that is in the index and in `HEAD` as `docs/research/UPPER.md`", and the refusal's printed `git add` "exits 0 and stages nothing" before the printed re-run "refuses byte-identically" — a refusal's route that does not work as printed, on the platform's default volume, both binaries |
| `r1-p3-validate-says-no-address-reaches-file-one-does` | G2 | jigc validate | working-product | RG → `Rig G — neighbours`, the `validate` row — no block | no-break | "Wording, as far as I looked" — a sentence of an advisory whose route works under the true spelling |
| `r1-p3-migration-destination-is-source-on-case-folding-volume` | G3 | jigc task finalize | no-lost-files | RG → `Left open — hit on the way, not pursued`, item 3 — no block | unclear | "the destination and the source to retire are one file on this volume. I stopped at the mint and claim nothing about what finalize does there" — H's report drove one such title to a refusal at `jigc doc author` and never to finalize; lacks a repro block |
| `r1-p3-doc-show-lowercase-address-case-sensitive-volume` | G4 | jigc doc show | working-product | RG → `Left open — hit on the way, not pursued`, item 4 — no block | unclear | "A case-sensitive volume was not driven, on either binary"; lacks a repro block |
| `r1-p3-lowercase-address-other-doors-not-driven` | G5 | unlisted: the doors that take a `<type>:<slug>` address — the `#fragment` reads, the `--task` arm, the write verbs, jigc rename, jigc unmanage | working-product | RG → `Left open — hit on the way, not pursued`, item 5 — no block | unclear | "Other doors that take a `<type>:<slug>` address were not driven with a lower-cased address over an upper-cased file" — write verbs among them; lacks a repro block |

### From H — the adoption route

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-doc-author-nothing-persisted-yet-task-bound` | H1 | jigc doc author | working-product | RH → `Repro V-1` (steps 2 and 3) | unclear | "`write.non-reparseable` says *nothing was persisted*, and the task is bound afterwards … Whether that binding is what the sentence disclaims is the design's to say" |
| `r1-p3-refused-doc-rename-leaves-staged-foreign-copy` | H2 | jigc doc rename | no-lost-files | RH → `Left open — hit on the way, not pursued`, item 2; `Repro V-1` (step 4) — a block of another finding | unclear | "After step 7's exit 1 the task's working area holds `research:upper.md` (the foreign bytes)" — a refused verb that leaves a staged copy under a managed identity; what it does to a later finalize is not settled; it has no repro block of its own |
| `r1-p3-rename-bare-git-mv-error-no-code-no-route` | H3 | jigc rename | working-product | RH → `Repro V-1` (step 6) | unclear | "no finding code, no `at:`, and a closing instruction to re-run the command that just failed" — whether that instruction is a route that fails as printed, in an ordinary layout, the report does not settle |
| `r1-p3-finalize-approve-advisory-for-file-it-retires` | H4 | jigc task finalize | working-product | RH → `Left open — hit on the way, not pursued`, item 4 — no block | no-break | "prints an `unadopted-instance` advisory for the file that same call retires" — an advisory at exit 0 of a call that lands; no refusal and nothing lost |
| `r1-p3-adoption-of-staged-source-no-deletion-row` | H5 | jigc task finalize | no-lost-files | RH → `3. The workflow that route opens, carried to its end` (rigs D, PA); `Left open …`, item 5 | unclear | "*1 file committed*, the foreign file gone from the worktree … I did not examine it against `no-lost-files`" |
| `r1-p3-adoption-route-title-upper-undriven-cells` | H6, H7 | jigc validate | working-product | RH → `Left open — hit on the way, not pursued`, items 6 and 7 — no block | unclear | "A volume that does not fold case was not driven … what the route does then is unknown to me", and the untracked plant and the way out "on the candidate only"; lacks a repro block |
| `r1-p3-adoption-report-table-names-rig-not-driven` | H8 | none: the report RH, the heading of its section 1 | none | RH → `1. validate and ingest (rigs A, B, D, PA, PB …)` | no-break | the reporter's own correction: "`doc list` was not run in rig A before the migrate route; it was run in rigs B, D, PA and PB" — a table of a report against its own drive (`inconsistency`); no behaviour of the product |

### From I — the stranded doc's read

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-doc-show-stranded-generic-route-non-utf8-layout` | I1, K2 | jigc doc show | working-product | RI → `Repro VR-DS-1` (first cell) | needs-bound | "The run declares no bound and no text rules on it. Ruled supported, the second half of the clause is broken as driven here and this row is confirmed, `regression: false`" |
| `r1-p3-start-workflow-bare-refusal-non-utf8-index-entry` | I2 | jigc start | working-product | RI → `Repro VR-DS-1` (first cell, fourth `repro` line) | needs-bound | "exits 1 with one bare line … no code, no `at:`, no route. Both binaries, byte-identical. A refusal with no route at the task-minting door" — ungraded by its reporter, in the layout whose support is unruled |
| `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc` | I3 | jigc doc show | working-product | RI → `The route, run as printed`, second row — a block of another finding | unclear | "says *has no committed copy* over a doc committed at its prior home … not driven without it" — without the entry the strand is reached by two ordinary commands, and that cell is undriven; it has no repro block of its own |
| `r1-p3-second-doc-of-one-identity-beside-stranded-doc` | I4 | jigc doc create and jigc task finalize | no-lost-files | RI → `Repro VR-DS-1` (fourth cell, F) | unclear | "No door on that path said that the identity already had a committed doc … Candidate only. No byte was lost" — whether a second doc of one identity at exit 0 is an incorrect write, and what the previous release does, the report does not settle |
| `r1-p3-doc-list-omits-stranded-doc` | I5 | jigc doc list | working-product | RI → `Left open`, item 5 (cell D2) — no block | unclear | "exit 0, four rows, no `research:context-loss` … seen once, not pursued, not driven without the entry or on the previous release" (K's and J's reports show the same four rows on both binaries, with and without the entry); lacks a repro block |
| `r1-p3-start-orientation-host-temp-path-findings-unknown` | I6 | jigc start | working-product | RI → `Left open`, item 6 (cell D2) — no block | unclear | "prints an absolute temporary host path inside *findings: unknown* … Not driven on the previous release" — whether orientation worked there is not known; lacks a repro block |
| `r1-p3-config-set-then-git-reset-strands-relocated-docs` | I7 | jigc config set | working-product | RI → `Repro VR-DS-1` (third cell, G and PG) | no-break | "The read then routes correctly" — the strand comes of a `git reset --hard` after the door, on both binaries, and J's report ran every arm of the repair to exit 0 |

### From J — the route after a strand

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-orphaned-doc-route-unmanage-without-path` | J1 | jigc validate | working-product | RJ → `The route's arms — what stands after each` (cell *unmanage-bare*) | no-break | "`file-state.orphaned-doc` is an advisory at exit 0, not a refusal … The path is one line above, on `at:`, and with it the arm works" — I do not read a verb named in a prose advisory as a route that fails as printed |
| `r1-p3-doc-show-offers-unmanage-after-unmanage` | J2 | jigc doc show | working-product | RJ → `Left open`, item 2 — no block | unclear | "still says a re-point stranded the doc and offers `jigc unmanage docs/research/context-loss.md` — which by then is the no-op" — one repair of a refusal's route that changes nothing; lacks a repro block |
| `r1-p3-unregistered-doc-route-after-unmanage-not-run` | J3 | jigc validate | working-product | RJ → `Left open`, item 3 — no block | unclear | "`jigc migrate <the doc's absolute path> --as research`, or ignore it. Not run"; lacks a repro block |
| `r1-p3-finalize-after-orphaned-doc-route-arms-not-driven` | J5, J4 | jigc task finalize | working-product | RJ → `Left open`, items 4 and 5 — no block | unclear | "whether a task's finalize behaves after each arm was not driven", and the baseline the hand move is promised "on its next author or finalize" was driven by neither; lacks a repro block |
| `r1-p3-orphaned-doc-placement-arm-and-json-not-driven` | J6 | jigc validate | working-product | RJ → `Class`, and `Left open`, item 6 — no block | unclear | "1 of those 2 was driven … The placement arm's route was not run", nor `--format json` of any cell; lacks a repro block |
| `r1-p3-reporter-slip-count-corrected-by-shell` | J7 | none: the stage's reporter contract | none | RJ → `Coverage` (the corrected count) | no-break | the reporter's own disclosure: one count of its report replaced by a shell substitution after the file tool wrote it; no behaviour of the product |

### From K — the orphan walk on the previous release

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-root-knob-doors-non-utf8-name-no-lost-files-reading` | K1, K4, L5 | jigc config set and jigc relocate | no-lost-files | RK → `Repro VR-P2-ORPHAN-PREV` | needs-bound | "they exit 0, move nothing, say nothing about it, and the two `config set` doors still land the knob" on both binaries; "It does not say the behaviour breaks no clause of the rule" — graded against this clause by nobody, in a layout ("a tracked name that is not UTF-8") that no bound and no ruling places |
| `r1-p3-validate-clean-after-placement-root-strand-non-utf8` | K3 | jigc validate | working-product | RK → `Repro VR-P2-ORPHAN-PREV` (door `jigc config set placement-root`, third `repro` line) | needs-bound | "prints `no findings — the committed store validates clean` … over a store two docs have dropped out of. Identical on both binaries" — a false green, in the same unruled layout |
| `r1-p3-git-capture-callers-enumerated-by-neighbouring-report` | K5 | unlisted: the callers of `task::git_capture` (the reporter names no door) | no-lost-files | RK → `Left open`, item 5; answered in RL → `The enumeration` | no-break | L's report is that enumeration — "34 production call sites … No cell, on either binary, shows a capture that could not be decoded letting a door destroy bytes no git object holds" — and what it left undriven has entries of its own below |
| `r1-p3-orphan-walk-aftermath-on-previous-release` | K6 | jigc config set and jigc relocate | working-product | RK → `Left open`, item 6; answered in RJ → `The state every cell starts from (both binaries, identical text)` | no-break | J's report drove that aftermath on both binaries: "The candidate and the previous release agree on every cell, byte for byte" |

### From L — the other callers of `task::git_capture`

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-task-finalize-refuses-staged-non-utf8-text-file` | L1 | jigc task finalize | working-product | RL → `Observed, and not to be pinned green` (third cell) | unclear | "One byte 0xE9 in a 13-byte file, default git configuration, nothing planted by plumbing … it is a door that does not run, which is the second clause's question (`working-product`), and it was not graded" |
| `r1-p3-rename-mention-advisory-silent-undecodable-listing` | L2 | jigc rename | working-product | RL → `Observed, and not to be pinned green` (first cell) | unclear | "two mentions of the old slug go unreported at exit 0. And, read and not driven: … under default quoting a tracked file whose name git C-quotes is never opened" — the second half is an ordinary state nobody drove |
| `r1-p3-uninstall-force-names-nothing-undecodable-listing` | L3 | jigc uninstall | no-lost-files | RL → `Observed, and not to be pinned green` (second cell); `Repro VR-GC-1` (door `jigc uninstall`) | needs-bound | "the untracked file is destroyed under consent and neither warning block is printed. The refusal that precedes it … routes to *make sure `git` is on PATH and the `.jigc/` tree is readable*; git was on `PATH` and the tree was readable" — a route whose condition already holds, in the layout whose support is unruled |
| `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path` | L4 | jigc task finalize | working-product | RL → `Left open`, item 4 — no block | unclear | "A printed route that spells a C-quoted name as a path … Not run" — git C-quotes any name with a byte above 0x7F by default, so the state need not be the planted one; lacks a repro block |
| `r1-p3-milestone-doors-bare-refusal-non-utf8-index-entry` | L6 | jigc milestone create and jigc milestone finalize | working-product | RL → `The jigc milestone verbs — rig fresh` | needs-bound | "`jigc milestone create` exits 1 (control 0), `jigc milestone finalize` exits 1 (control 0) … both binaries. None is a `git_capture` site, none loses anything, none was enumerated" — a `working-product` question its reporter did not grade, in the unruled layout |
| `r1-p3-post-commit-capture-sites-host-that-holds-the-name` | L7 | jigc task finalize and jigc milestone finalize | no-lost-files | RL → `Left open`, item 7 — no block | unclear | "Each propagates its error, so what is open is a door exiting non-zero *after* its commit landed. Read, not driven: one macOS host"; lacks a repro block |
| `r1-p3-git-capture-commit-message-sites-not-driven` | L8 | unlisted: the 5 commit-message call sites of `task::git_capture` (class B; in `jigc task finalize`, the commit seam and `jigc milestone finalize`) | no-lost-files | RL → `Left open`, item 8 — no block | unclear | "A commit message that is not UTF-8 — the 5 class-B sites. Not driven"; lacks a repro block |

### From M — the promote-clobber route

| key | source | door | clause | repro | grade | why |
|---|---|---|---|---|---|---|
| `r1-p3-finalize-text-left-out-list-no-kind` | M1 | jigc task finalize | working-product | RM → `The drive, candidate` (steps 7 to 9) and `Left open`, item 1 — a block of another finding | no-break | "the path is listed under `left-out (unstaged/untracked — git add to include)` with nothing saying that what was left out is a *deletion*" — a line of an exit-0 summary, no refusal's route; no byte is lost, and the JSON forecast carries the kind |

## To verify — 38

Each with what the verifier must re-drive. Both binaries wherever the previous release has the door.

| key | re-drive |
|---|---|
| `r1-p3-setup-writability-route-printed-for-other-causes` | `Repro VR-2` and its variants on fresh `bare` rigs: `core.hooksPath=/dev/null`, `core.hooksPath` a regular file, a read-only hook file in a writable hooks directory, a hook with one non-UTF-8 byte — and the cause its reporter graded neither way, `core.hooksPath` a missing directory under a read-only parent. Per cause: is the route's condition already true, and does the re-run exit 0 once it is |
| `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` | on a case-folding volume, a committed `docs/research/UPPER.md`; `jigc migrate` handed `docs/research/upper.md` typed by hand, relative and absolute, with no `jigc doc show` in front; the refusal's `git add` and re-run as printed; the control with matching case |
| `r1-p3-not-staged-route-task-less-read-refuses` | `Repro V-2b`, control-b: a readable file that is no managed doc at a doctype's home; `jigc doc show <address> --task <id>`, then the route's task-less read as printed; whether a route that ends in a further routed refusal is what `design/doc-read-surface.md` gives that read |
| `r1-p3-unreadable-entry-class-door-list-traced-by-hand` | derive the doors from the call graph of the fourteen `committed_instances` call sites and of the two read sites outside it; drive each door the report's list lacks over the dangling link and the mode-000 file, with a no-plant control; look for an exit-0 write, commit or removal |
| `r1-p3-unreadable-entry-undriven-platform-shapes-and-doors` | the cells the report lists as not driven: a directory and a named pipe named `x.md` at the doors beyond the listing; the plant at a code-anchor doctype's home with anchors present; a tracked managed doc made unreadable; a commit through the installed `pre-commit` hook over the plant; `jigc migrate-corpus` and `jigc relocate` over a corpus that folds; and Linux, where a verifier has one |
| `r1-p3-doc-list-tracked-dangling-link-raw-os-error` | a committed doc's home replaced by a tracked link whose target is then removed, committed; `jigc doc list`, both formats; the block this finding lacks, and whether the listing answered there on the previous release |
| `r1-p3-milestone-merged-area-writer-follows-a-link` | whether any jigc verb, or an ordinary act, can leave a link under `.jigc/milestones/<id>/merged/docs/` before `jigc milestone join` or `finalize` writes there (the report says `clear_staged_bodies` runs first); with one planted, where the write lands and at what exit |
| `r1-p3-staging-area-writers-undriven-and-unread-remainder` | the inventory of `Repro W-1` after `jigc migrate`'s authoring path and after a re-opened (amend) task; the spawned `git` call sites read as possible writers into `.jigc/tasks/<id>/docs/`; a root caller and Linux where available |
| `r1-p3-staged-doc-ids-orientation-and-milestone-callers` | the two plants of `Repro V-2` (a dangling link, a mode-000 file named as a staged doc) under `jigc start` (orientation) and under the milestone door that reads `staged_task_prose`: does each fail closed, and does a forced form name what it takes |
| `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` | `Repro V-2` as root (the mode-000 variant binds nobody there) and on Linux |
| `r1-p3-superproject-with-own-install-shared-hook-not-driven` | the layout of `Repro VR-2` with `jigc setup` run in the superproject first: the hook's one block and its `jigc=` line after `setup` in the submodule; then `jigc uninstall` typed in the submodule — does the superproject keep a block that reaches its own install |
| `r1-p3-hook-in-another-repository-layouts-not-enumerated` | enumerate the layouts in which `git rev-parse --git-path hooks` leaves the repository `setup` is typed in (a linked worktree, a `--separate-git-dir` checkout, a worktree of a bare repository, a shared `core.hooksPath`); per layout, what the block's `jigc validate` does on a commit of the other repository |
| `r1-p3-uninstall-writability-route-not-driven` | the four causes of `Repro VR-2` and the outward hook link of `Repro VR-1` at `jigc uninstall` after a clean `jigc setup`: the refusal `uninstall.remove-precommit`, its route as printed, the re-run |
| `r1-p3-display-hook-path-other-callers-not-driven` | a `pre-commit` hook that is a link to a script of the repository and one to a sibling: the path the install's ack names, and the path the teardown's *left the `pre-commit` hook in place* warning tells its reader to delete |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | `make_executable` failing; a hook that is a directory; a hook that cannot be read; an outward link written relatively; each refusal under `--force` and under `--format json`: does the route name the cause and does it work as printed |
| `r1-p3-validate-clean-over-orphan-absent-from-work-tree` | `Repro RC-9u`'s setup, the orphan then removed from the work tree and not staged; `jigc validate`, both formats, on both binaries; set against the comment on `carries_stamp` and `design/validation.md` |
| `r1-p3-validate-unreadable-orphan-platforms-and-variants` | `Repro RC-9u` with the file unreadable by an ACL, by ownership and by an unreadable parent directory; as root; on Linux where available |
| `r1-p3-migration-destination-is-source-on-case-folding-volume` | on a case-folding volume, a committed `docs/research/UPPER.md` that **conforms** to the `research` schema; `jigc migrate` it, author under the title `Upper`, finalize and approve: is the file there afterwards, and what does the commit hold. The non-conformant file is H's `Repro V-1` and stops at `jigc doc author` |
| `r1-p3-doc-show-lowercase-address-case-sensitive-volume` | `Repro V-1` on a volume that does not fold case, both binaries — a case-sensitive disk image suffices |
| `r1-p3-lowercase-address-other-doors-not-driven` | a lower-cased address over `docs/research/UPPER.md` (made conformant where the door needs it) at `jigc doc show <address>#<fragment>`, the `--task` read, `jigc doc set-slot`, `jigc doc set-field`, `jigc rename` and `jigc unmanage`; the tree and the index before and after each |
| `r1-p3-doc-author-nothing-persisted-yet-task-bound` | `Repro V-1`, steps 1 to 3, with the task's working area and its provenance listed before and after the refused `jigc doc author`; the same refusal on a volume-independent non-conforming payload, to see whether the binding comes of the refusal or of the case fold |
| `r1-p3-refused-doc-rename-leaves-staged-foreign-copy` | `Repro V-1` through step 4, the task's `docs/` listed around the refused `jigc doc rename … --task`; then `jigc doc list research --task <task>` and `jigc task finalize <task>`; both binaries |
| `r1-p3-rename-bare-git-mv-error-no-code-no-route` | `Repro V-1`, steps 5 and 6: the whole stderr of the failing `jigc rename`, set against `design/write-commands.md` → `jigc rename` and the surface contract's refusal shape; both binaries |
| `r1-p3-adoption-of-staged-source-no-deletion-row` | an untracked foreign source staged by `migrate.source-untracked`'s route and adopted (rig D's chain): after the approve, does any ref, the index or the commit hold the source's bytes; set against `design/auto-migration.md` → *Trackedness precondition* |
| `r1-p3-adoption-route-title-upper-undriven-cells` | `Repro V-1` on a volume that does not fold case, both binaries; the untracked plant under the title `Upper` and the way out (discard, re-migrate, author under another title first) on the previous release |
| `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc` | the strand of `Repro VR-DS-1`'s third cell (`jigc config set docs-root notes`, then `git reset --hard`, no planted entry), then `jigc doc show research:context-loss --task <task>`; both binaries |
| `r1-p3-second-doc-of-one-identity-beside-stranded-doc` | from that same strand, with no planted entry: `jigc start --workflow do-research`, `jigc doc create research --title "Context Loss"`, its finalize; what the index holds, what `jigc doc list` and `jigc validate` say of the first doc; both binaries |
| `r1-p3-doc-list-omits-stranded-doc` | `jigc doc list` over that strand, with and without the non-UTF-8 entry, both binaries, both formats; set against `design/doc-read-surface.md` on what the listing owes a committed doc outside every resolved home |
| `r1-p3-start-orientation-host-temp-path-findings-unknown` | `jigc start` with the entry of `Repro VR-DS-1` standing, on both binaries: the *findings* line and the path in it |
| `r1-p3-doc-show-offers-unmanage-after-unmanage` | `Repro VR-ROUTE-1`, the unmanage arm, then `jigc doc show research:context-loss`; each of the three repairs its route names, run as printed; both binaries |
| `r1-p3-unregistered-doc-route-after-unmanage-not-run` | after that arm, `jigc validate`'s `file-state.unregistered-doc` route as printed — the `jigc migrate <path> --as research` — to wherever it ends; both binaries |
| `r1-p3-finalize-after-orphaned-doc-route-arms-not-driven` | after each arm of `Repro VR-ROUTE-1` (re-point, unmanage, hand move): finalize the rig's live task; what is committed, and whether `file-state.un-baselined` clears after the move arm |
| `r1-p3-orphaned-doc-placement-arm-and-json-not-driven` | a `placement-root` strand: the advisory's placement arm and its `jigc unmanage <path>` as printed; `--format json` of the precondition and of each arm |
| `r1-p3-task-finalize-refuses-staged-non-utf8-text-file` | the third cell of *Observed, and not to be pinned green* on both binaries; the doc-only, amend and migration arms with the same staged file; whether any refusal there carries a route |
| `r1-p3-rename-mention-advisory-silent-undecodable-listing` | a tracked file with a non-ASCII name (which git C-quotes by default) holding a mention of the slug, default git configuration: does `jigc rename` report the mention; both binaries |
| `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path` | a staged file with a non-ASCII name that the filesystem holds, at the amend arm: the refusal's `git … restore --staged -- …` run as printed |
| `r1-p3-post-commit-capture-sites-host-that-holds-the-name` | needs a filesystem that holds a name that is not UTF-8: `jigc task finalize` and `jigc milestone finalize` with such a tracked file under `core.quotePath=false` — the exit after the commit lands |
| `r1-p3-git-capture-commit-message-sites-not-driven` | a `HEAD` commit whose message is not UTF-8 (`git commit-tree` with raw bytes), then `jigc task finalize`, its amend arm and `jigc milestone finalize`: the exit, and whether `HEAD` moved before it |

## The findings that came without a repro block

My contract makes the missing block an entry against the reporter's report. **I made no such entries**, and
say so here instead: the harness holds a pass to *entries + merged = findings handed*, so an entry beyond
the 88 would fail the stage's count. The 32 entries above whose repro cell ends "no block" are that list, by
reporter: A (3), B (4), C (2), D (3), E (3), F (2), G (4), H (2), I (2), J (4), L (3). Three more — H2, I3 and
M's one entry — lean on a block that is another finding's and have none of their own. Each of the 35 that
is graded `unclear` says so in its `why`; the verifier's first act on it is the block.

## Two limits of this pass — for the record step and the human

**1. No finding could be marked found again, and several restate a finding an earlier pass of this stage
keyed.** The ledger the state hands me holds one row. The keys that passes 1 and 2 minted are in no table
yet — the record step writes them — and the prompt hands me neither those passes' entries nor their
reports. I know 22 of those keys, and only by name: from the launched verifiers' names and from the reports
I was handed. So every entry here is new, and its key carries the infix `p3` so that it cannot collide with
a key I could not see (the harness merges a later pass's entry over an earlier one of the same key, and a
collision would overwrite a grade without a word). The entries that, as their reporters say, are the same
door and behaviour as an earlier key, or the question an earlier verdict rests on:

| entry of this pass | earlier key of this stage, named by the report |
|---|---|
| `r1-p3-doc-list-tracked-dangling-link-raw-os-error` | `r1-doc-list-unreadable-entry-fails-listing` — the tracked-link variant of the same listing failure |
| `r1-p3-doc-list-drops-unreadable-stamped-orphan-silently`, `r1-p3-unreadable-entry-stops-commit-boundaries-no-route` and the other entries from A | `r1-doc-list-unreadable-entry-undriven-shapes-and-consumers` — "the defect is real, and it is wider than the row that named it" |
| `r1-p3-umask-masking-owner-read-unreadable-staged-doc`, `r1-p3-staging-area-writers-undriven-and-unread-remainder` | `r1-staging-area-writers-not-enumerated` |
| `r1-p3-forced-doors-name-no-staged-docs-under-dangling-link` | `r1-staged-doc-ids-other-callers-not-driven` — "the defect this verdict leaves on the ledger" |
| `r1-p3-setup-in-submodule-no-backstop-install-elsewhere`, `r1-p3-validate-in-submodule-runs-at-git-dir` | `r1-setup-installs-outside-work-tree` — the same misplaced install, which the report says it does not grade |
| `r1-p3-setup-writability-route-printed-for-other-causes` | `r1-setup-install-hook-route-other-causes-unexamined` — its second block, with the other regression fact |
| `r1-p3-validate-clean-over-unreadable-orphan-reading-and-layout` | `r1-validate-unreadable-orphan-variant-not-driven` — the reading and the layout its refuted verdict rests on |
| `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` | `r1-doc-show-lowercase-address-prints-path-no-file-has` — the same chain, at the next door |
| `r1-p3-doc-author-nothing-persisted-yet-task-bound`, `r1-p3-rename-bare-git-mv-error-no-code-no-route` | `r1-adoption-route-for-non-doc-id-name-not-run` — steps of its confirmed block, each at a door of its own |
| `r1-p3-doc-show-stranded-generic-route-non-utf8-layout` | `r1-doc-show-stranded-doc-not-found-route` — the scope question its refuted verdict rests on |
| `r1-p3-root-knob-doors-non-utf8-name-no-lost-files-reading` | `r1-non-utf8-path-other-orphan-walk-consumers` "and its first open item" — two reports say the reading against `no-lost-files` belongs there |
| `r1-p3-git-capture-callers-enumerated-by-neighbouring-report` | `r1-git-capture-other-callers-not-enumerated` |
| `r1-p3-orphan-walk-aftermath-on-previous-release` | `r1-orphaned-doc-route-after-strand-not-run` |

Where an earlier pass already holds one of these under a key of its own, the run has two rows for one
finding; folding them is a ruling on the ledger and not mine. I did not put such a finding under the earlier
key: I do not hold that row's cells, an entry of mine would replace them, and a question that a *refuted*
verdict rests on would go off the human's list with that verdict.

**2. Fifteen entries ask the human for a bound, and they are three states.** A file or a staged doc its
owner cannot read (seven: A1, A3, A5, B1, C1, C2, F1); an index entry or a tracked name that is not UTF-8
(six: I1, I2, K1, K3, L3, L6); `jigc setup` typed in a submodule (two: D2, D4). A ruling on each state, with
its reach, settles its whole group. A fourth planted state, a link made by hand in the workbench, asks for
none here: its one finding (C3) is `no-break` on the consent `--force` gives, whatever the state's standing.

## What I did not do

- I confirmed and refuted nothing, and established no regression fact: every fact above is its reporter's.
- I computed no finding's place inside or outside the round's test set; each entry carries its door in the
  reporter's words — `jigc doc list` exactly where that is the door, the listed door's text where a door is
  one of the 47 excluded, and a plain description marked *unlisted* where the reporter names no single verb.
- I proposed no fix and ranked nothing by effort.
- I called no writer of the record script but `report`, for this file.

## Tree state

Branch `fix/canary-one` at `eeffe347`; `git status --short` shows the run's untracked
`completions/artifacts/canary-one/r1/` and nothing else, as before I started. Nothing built, edited, staged
or committed.

<!-- end of report -->
