# triage — run `canary-one`, round 1, stage `test`, attempt 2, pass 1

**Status: graded.** 118 findings in, from one source (`ledger`); 118 entries, each under the
key its ledger row already holds, each found again; 0 new keys; 0 merged. By grade: `breaks` 3,
`unclear` 115, `no-break` 0, `out-of-scope` 0, `needs-bound` 0. One grade differs from the grade
the row carried in (row 65, below). 118 keys go to the verifier.

This pass was invoked only to finish the round's triage. I drove no binary, built no rig,
edited nothing, committed nothing, and called no writer of the record script but `report`.

## What I read

- The opening record, `completions/artifacts/canary-one/opening.md`, and the closing condition
  it names by reference: `DECISIONS.md`, *2026-10-04 — The exit rule, revised* — the four
  clauses, the two sharpenings (the first clause's scope; one instrument per clause), and the
  sentence *a finding blocks the call only if it breaks a clause inside its scope*.
- The run's state, `dev/stabilize-record state --run canary-one` (exit 0): no declared bound;
  one included door, `jigc doc list`; 47 excluded doors; clauses `no-lost-files`,
  `working-product`, `usable-by-agents` green and `migration-works` void; 372 ledger rows.
- The read the prompt names for the source `ledger`: `dev/stabilize-record untriaged --run
  canary-one` (exit 0), `count: 118`, every row `why: unverified`, `passes: 1`, `retry: due`;
  114 carried in as `unclear`, 4 as `breaks`; all `jigc-feedback`, disposition `open`.
- The two reports of attempt 2 the prompt lists, whole: `attempt.a2.md` and `preflight.a2.md`.
  Both end with their last line. No other report of attempt 2 is in the stage's directory.
  Neither holds a finding, a lead or a left-open entry.
- The 37 reports the 118 repro cells and source cells name (one `verify-p2-…`, thirty-six
  `verify-p3-…`, all attempt 1). **Each ends with its last line; none is cut off.**

**What I did not read whole, said plainly.** Those 37 reports are about 980 kB together. Of
each I read: the whole *Left open* section; the heading list; every heading a repro cell names
in place of a numbered item (*What was driven, and what was not* · *Scope of what was
verified* · *The class*, five times · *What I did not drive*); and, of the named repro
blocks, `Repro V-1` (migration), `Repro N-1` and `Repro N-2` with the rig-CB table,
`Repro VR-DST-1` and `Repro VP-2` whole, and the other named blocks by their `claim`,
`verdict` and the lines that carry the row's cell. A grade below rests on the sentence quoted
in its row, never on a part of a report I did not read. I read no triage report of an earlier
pass: the prompt hands me none.

## The arithmetic

| source | in |
|---|---|
| `ledger` | 118 |

118 in · 118 entries · 118 − 118 = 0 merged. No finding is in no entry. Twenty of the rows
carry two or three left-open items in one source cell; those folds were made when the rows
were minted, they are the rows' own, and I made none.

## How the grades were given

- **Door, clause, doctype, source and repro are each row's own cells, returned unchanged.**
  Whether a row is inside the round's test set is the script's; I supply no such word.
- **`breaks`** only where the report states, as driven, that a refusal's printed route fails
  as printed (`working-product`: *every refusal's route works as printed*), in a state the
  report does not make by a plant, and a repro block in the report carries the cell.
- **`unclear`** for everything the report does not settle — and for every row whose repro
  cell names no block: a verdict needs a block, so such a row is graded `unclear` and said to
  lack one. 96 rows lack a block (column *block*: `-`).
- **No `no-break`.** No row's report says squarely that no clause can be reached; most say
  *not driven* or *not graded*. In doubt the grade is `unclear`; a wrong `no-break` is silent.
- **No `out-of-scope`:** the run declares no bound, so there is none to cite.
- **No `needs-bound`.** Many rows sit over a state a verifier made by hand (a mode-000 file,
  a planted link, a planted index entry, a `chmod`). Whether such a state is outside a
  clause's scope is the human's ruling and no bound is declared; the rows that put exactly
  that question are on the ledger already under keys of their own, graded `needs-bound` by
  earlier passes. For the rows here the report does not settle whether the behaviour is real
  on both binaries in a supported layout, so they stay `unclear` and the plant is named in
  the row's *why*.

### The one grade that differs from the grade carried in

- **Row 65, `r1-p4-identity-change-route-doc-rename-exits-1-over-foreign-file`: carried in
  `breaks`, graded `unclear`.** Its repro cell names `Repro N-1`. That block holds the setup
  (the two files) and the first two steps, then goes another way (`jigc task discard`, a
  second migrate); the two steps the claim rests on — the author refused
  `write.identity-change`, and its printed `jigc doc rename … --task` exiting 1 — are rows 3
  and 4 of the rig-CB table under *Triage's question*, in no block. The report also lists the
  onward route that step 4 prints as not driven, and says of the chain that *it is the row
  `r1-adoption-route-for-non-doc-id-name-not-run`, already on the ledger with its own
  verdict*. The key is kept; nothing is merged. It still goes to the verifier.

The other three rows carried in as `breaks` keep the grade (46, 52, 82).

### A rule of the contract that has no row here

The triage contract says a missing repro block is itself an entry against the reporter's
report. I minted none. The source of this pass is the ledger, which is no reporter of the
stage, and the harness holds a pass to *entries + merged = findings handed*, so an entry
beyond the 118 would be refused. The fact is not dropped: the 96 block-less rows are marked
in the table, and they sit in all 37 reports named above.

### Kin rows, named so that nobody takes them for new observations

Not merged — each row keeps its key — but the report itself says the item was seen before:

- row 56 beside `r1-doc-list-prints-id-doc-show-refuses` (refuted);
- row 65 beside `r1-adoption-route-for-non-doc-id-name-not-run` (confirmed, outside);
- row 79 beside `r1-p3-adoption-of-staged-source-no-deletion-row` (refuted);
- row 83 beside `r1-p3-second-doc-of-one-identity-beside-stranded-doc` (refuted);
- row 46 is the `Repro VP-1` mechanism of `r1-p3-setup-install-hook-undriven-producers-and-causes`
  (confirmed, a regression) with another printed path.

## The entries

Every key below is prefixed `r1-p4-` except row 1, whose key is written whole. Column
*block*: the block the repro cell names, or `-`. Clause: `nlf` = `no-lost-files`, `wp` =
`working-product`. The *why* is the sentence of the report the grade rests on, shortened only
by `…`.

| # | key | clause | grade | block | why |
|---|---|---|---|---|---|
| 1 | `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` | nlf | unclear | - | "Linux, and a root caller, were not driven." |
| 2 | `unparseable-adoption-route-refuses-untracked-source` | wp | unclear | - | the adoption route "exits 1 `migrate.source-untracked` when the foreign file is not in git's index" — its own route then ran; candidate only, "not graded" |
| 3 | `not-staged-route-mode-000-repro-b-not-redriven` | wp | unclear | - | "The handed block's repro-b … was not re-driven" |
| 4 | `not-staged-block-other-unservable-states-not-enumerated` | wp | unclear | - | "I did not derive which other states reach its first arm without a servable doc behind it" |
| 5 | `nothing-staged-route-git-add-fails-unreadable-file` | wp | unclear | V-3c | "git answers 128, *unable to index file*, and the re-run repeats the refusal" — over a mode-000 file the verifier planted |
| 6 | `relocate-parks-untracked-entry-at-destination` | nlf | unclear | - | "`jigc relocate` moves an untracked entry it finds at a destination into `.jigc/displaced/` … The two sentences were not reconciled here." |
| 7 | `unreadable-entry-class-linux-root-remaining-cells-not-driven` | wp | unclear | - | "Linux and root — still driven by nobody for this class." |
| 8 | `not-found-not-staged-routes-loop-at-resume-door` | wp | unclear | V-3c | "`store.not-found` and `store.not-staged` route to each other over a doc that exists and cannot be read — now also at the resume door" — a mode-000 plant |
| 9 | `reconciliation-rename-route-jigc-rename-not-found` | wp | unclear | - | "The first, `jigc rename arch-doc:padding-layer --to …` … exits 1, `store.not-found`"; the second alternative works |
| 10 | `doc-list-dangling-link-reach-by-jigc-verb-not-driven` | wp | unclear | - | "Whether a jigc verb on the candidate can still carry a live link at a home into a dangling one was not driven." |
| 11 | `doc-list-dangling-link-undriven-cells` | wp | unclear | - | "Not driven: Linux; a root caller; a placement doctype's home as a dangling link …" |
| 12 | `milestone-merged-link-plant-undriven-cells` | nlf | unclear | - | "Not driven: Linux; a root caller; a link whose target is given relative …" |
| 13 | `following-writes-class-not-walked` | nlf | unclear | - | "other following writes in jigc were not walked (`instance, unbounded`)" |
| 14 | `staging-area-writers-root-and-linux-not-driven` | wp | unclear | - | "A root caller and Linux were asked for and not driven." |
| 15 | `finalize-refuses-tracked-link-home-previous-release-landed` | wp | unclear | - | "the candidate's finalize refuses where the previous release's landed (… the link is this verifier's plant …) … that route was not driven" |
| 16 | `finalize-approve-read-only-destination-raw-os-error` | wp | unclear | - | "The approve over a mode-0444 destination fails with a raw OS error … The mode is this verifier's `chmod`; the re-run was not driven." |
| 17 | `tracked-path-under-task-staging-area-git-writer` | nlf | unclear | - | "Read, not driven; no jigc verb creates it." |
| 18 | `staging-area-writers-undriven-doors` | wp | unclear | - | "Not driven: migration into any doctype but `adr`; an amend over a commit jigc did not make; …" |
| 19 | `staging-area-enumeration-bound-and-other-readers` | wp | unclear | - | "a spawn reached through a spelling neither sweep matches would be missed" |
| 20 | `milestone-staged-prose-route-not-staged-unreadable-doc` | wp | unclear | - | "`milestone.staged-prose` … routes at `jigc doc show <address> --task <sub-task-id>`; that command exits 1 with `store.not-staged`" — under the mode-000 plant |
| 21 | `milestone-finalize-missing-provenance-route-no-command` | wp | unclear | - | "`join.missing-provenance`, whose route … names no command" — under the mode-000 plant |
| 22 | `staged-doc-ids-other-plants-linux-root-not-driven` | nlf | unclear | - | "Other plants at these two doors … were not driven."; "Linux, and a root caller, were not driven." |
| 23 | `staged-doc-ids-plant-reach-not-driven` | nlf | unclear | - | "Reach was not driven." |
| 24 | `uninstall-submodule-refusal-first-route-does-not-clear` | wp | unclear | VR-3 | "one more bare `jigc uninstall` in the submodule … exits 1 with the identical stderr" — driven once, candidate only, in a superproject-and-submodule layout |
| 25 | `superproject-submodule-undriven-orders` | wp | unclear | - | "The reverse order was not driven" |
| 26 | `setup-separate-git-dir-commits-into-enclosing-repository` | nlf | unclear | VR-3 (L5) | "`jigc setup` at exit 0 commits into a repository it was not typed in" — "did not look at how ordinary the layout is, and grade nothing of it" |
| 27 | `hook-rename-block-absent-in-linked-worktree` | wp | unclear | VR-3 (L1r) | "a bare `git mv` of a managed doc committed in a linked worktree lands unrefused … what is open is whether the backstop is meant to see it" |
| 28 | `hook-git-dir-export-validate-children-not-examined` | wp | unclear | - | "What those children read under that variable … was not examined." |
| 29 | `setup-separate-git-dir-refusal-route-not-driven` | wp | unclear | VR-3 (L2, L3) | "its route … names a directory that is writable … I did not re-run `setup` as the route says." |
| 30 | `setup-bare-repository-pointer-installs-outside-work-tree` | nlf | unclear | VR-3 (L3b, L3c) | "`setup` exits 0 and installs at a directory that is no work tree, with no install commit" |
| 31 | `two-installs-one-hook-other-layouts-not-driven` | wp | unclear | - | "each `setup` repoints the other's backstop … Neither was driven" |
| 32 | `hook-layouts-undriven-cells-and-previous-release` | nlf | unclear | - | "I drove one of them, the install; `jigc uninstall` in any of these layouts was not driven" |
| 33 | `uninstall-writability-sentence-other-causes-not-driven` | wp | unclear | - | "Whether it is printed there for causes that are not writability was not driven." |
| 34 | `setup-over-compiled-program-hook-not-driven` | wp | unclear | - | "A hook that is a compiled program would reach the same read … Not driven." |
| 35 | `setup-writability-route-undriven-causes` | wp | unclear | - | "Not driven: a failing resolution of the hooks path; a failing change of mode; …" |
| 36 | `uninstall-six-other-teardown-refusals-same-route-shape` | wp | unclear | - | "any error of the step, then *ensure `<x>` is writable*. Read, not driven." |
| 37 | `uninstall-refusal-leaves-teardown-half-done` | wp | unclear | - | "A refusal at step 6 leaves the teardown half done at exit 1 … I looked no further" |
| 38 | `uninstall-writes-through-outward-hook-link` | nlf | unclear | VU-2 | "writes through a hook that is a link to a file outside the repository, at exit 0 … I saw no bytes lost … whether the teardown should is nobody's ruling that I found" |
| 39 | `uninstall-ignores-installed-hook-after-hookspath-change` | wp | unclear | - | "A knob that names another existing directory was not driven; by the source that is exit 0 with the hook left behind and nothing said." |
| 40 | `uninstall-removes-hook-link-leaves-block-at-links-end` | nlf | unclear | VU-2 | "the teardown removes the link at exit 0 and the script outside keeps jigc's block, under *removed pre-commit hook*" |
| 41 | `uninstall-writability-refusal-undriven-causes` | wp | unclear | - | "Not driven: the three `resolve_hooks_dir` failures; a hook that is a directory, or unreadable; …" |
| 42 | `uninstall-warning-outward-link-outside-file-not-driven` | wp | unclear | - | "the warning would print the outside file's absolute path under *delete the file yourself*. Not driven" |
| 43 | `uninstall-kept-hook-sentence-not-tested` | wp | unclear | - | "was not put to the test: no commit was made through the kept hook after the teardown" |
| 44 | `display-hook-path-undriven-states` | wp | unclear | - | "States: `instance, unbounded`. … Not driven: a link's end that holds the adopter's own lines …" |
| 45 | `setup-make-executable-failure-not-driven-on-candidate` | wp | unclear | - | "`make_executable` failing on the candidate is still undriven." |
| 46 | `setup-hook-linked-to-dev-null-route-names-dev-null` | wp | **breaks** | (VP-1, its `/dev/null` variant) | "On the candidate, a hook linked to `/dev/null` is told to *remove the link at `/dev/null`*" — the refusal's route names a device node; the block's variant line says the cure is removing `.git/hooks/pre-commit` |
| 47 | `setup-writability-route-directory-or-unreadable-hook` | wp | unclear | VP-2 | "the directory is writable already, and the re-run fails with the same bytes" — over a directory at the hook path or a mode-000 hook, states the block makes by hand |
| 48 | `setup-install-hook-undriven-sites` | wp | unclear | - | "Not driven: the first construction site (`current_exe` failing); …" |
| 49 | `doc-list-over-orphan-absent-from-work-tree-not-driven` | wp | unclear | - | "`jigc doc list` under this state. … Another door; not driven." |
| 50 | `validate-unreadable-orphan-ownership-root-linux-not-driven` | wp | unclear | RC-9u-variants | "ownership, root and Linux NOT driven" |
| 51 | `doc-list-over-unreadable-orphan-not-driven` | wp | unclear | - | "`jigc doc list` under these plants. … the round's one door. Not driven here." |
| 52 | `migration-of-upper-case-file-stage-failed-route-refuses` | wp | **breaks** | V-1 | "Neither named cause is present; the re-run, as printed, exits 3 again … Both binaries." |
| 53 | `conflict-block-route-after-failed-migration-not-driven` | wp | unclear | - | "Its second route … I did not drive" |
| 54 | `ingest-case-rename-route-not-driven` | wp | unclear | - | "was not driven. It is the other command of the route in item 1 and may be the one that works." |
| 55 | `migration-source-equals-destination-undriven-states` | nlf | unclear | - | "Undriven states of the same mechanism: a case-sensitive volume; …" |
| 56 | `doc-list-prints-id-no-door-takes-seen-again` | wp | unclear | - | "`jigc doc list` prints `research:UPPER` in its `id` column, and no door takes that as an address … I read no design text that settles" |
| 57 | `two-case-spellings-side-by-side-not-driven` | wp | unclear | - | "The cell only a case-sensitive volume can hold was not driven" |
| 58 | `doc-show-lowercase-ignorecase-mismatch-and-linux-not-driven` | wp | unclear | - | "A repository whose `core.ignorecase` disagrees with its volume was not driven"; "A Linux file system was not driven." |
| 59 | `doc-show-not-found-route-task-read-not-driven` | wp | unclear | - | "the placeholder command of the `store.not-found` route, was not driven with an open task" |
| 60 | `address-doors-case-sensitive-volume-not-driven` | wp | unclear | - | "Other doors that take a `<type>:<slug>` address were not driven on this volume" |
| 61 | `finalize-exit-0-commits-without-doc-lowercase-address` | nlf | unclear | - | "the same finalize exits 0 and commits without the doc … No byte is lost … which clause it reaches, if any, is a grade I was not sent to give" |
| 62 | `finalize-base-mismatch-after-ingest-case-rename` | wp | unclear | - | "`finalize.base-mismatch`, exit 3, whose route is *resolve the overlap* with no command, or discard" |
| 63 | `copy-in-records-baseline-under-spelling-no-file-has` | nlf | unclear | - | "The candidate's copy-in records a file-state baseline under a spelling no file has. … The previous release writes no record at copy-in." |
| 64 | `lowercase-address-six-doors-not-driven` | wp | unclear | - | "Six address doors of the registry were not driven" |
| 65 | `identity-change-route-doc-rename-exits-1-over-foreign-file` | wp | unclear (was `breaks`) | N-1 (setup and two steps only) | "its printed route `jigc doc rename research:upper --to 'Has space' --task <task>` exits 1, on both binaries" — the two steps are table rows, in no block; the onward route is "not driven" |
| 66 | `non-reparseable-route-names-neither-broken-thing` | wp | unclear | - | "neither arm of `write.non-reparseable`'s route names the thing that is broken … the task started over under the same title meets the same refusal" |
| 67 | `refused-doc-author-leaves-file-state-lock` | wp | unclear | - | "leaves an empty `.jigc/state/file-state.json.lock` … Whether a lock file that stays is by design I did not read." |
| 68 | `provenance-row-of-rolled-back-doc-survives` | wp | unclear | - | "whether the by-task-id join reads such a row I did not examine" |
| 69 | `doc-author-nothing-persisted-undriven-cells` | wp | unclear | - | the list under *What I did not drive* — "A volume that does not fold case, on either binary. …" |
| 70 | `doc-list-task-reports-foreign-file-managed` | wp | unclear | P-1 | "reports a foreign, non-conformant file as `managed` at a path git has no entry for, at exit 0 … I graded it against `no-lost-files` only" |
| 71 | `refused-doc-rename-conformant-foreign-file-not-driven` | nlf | unclear | - | "This is the cell in which the mechanism could reach the first clause; I did not go looking for it." |
| 72 | `refused-doc-rename-undriven-volume-and-previous-release` | nlf | unclear | - | "A volume that does not fold case was not driven"; "driven on the candidate only" |
| 73 | `rename-moves-and-rewrites-unadopted-file` | nlf | unclear | - | "`jigc rename` moves a file jigc never adopted. … whether its target must be a managed one is the design's to say. Candidate only." |
| 74 | `rename-refusal-undriven-volume-and-previous-release` | wp | unclear | - | "A volume that does not fold case was not driven"; "driven on the candidate only" |
| 75 | `source-untracked-route-cures-one-of-two-reasons` | nlf | unclear | V-1 | "After that route the adopting commit still carries no deletion … whether the refusal's sentence is such a claim is not mine to grade" |
| 76 | `staged-source-edited-after-git-add-not-driven` | nlf | unclear | - | "what the retire does to bytes only the worktree holds I do not know" |
| 77 | `unreachable-blob-lifetime-not-driven` | nlf | unclear | - | "Nothing in the chain pruned it; I ran no `git gc` and read no prune configuration." |
| 78 | `adoption-chain-upper-previous-release-not-driven` | nlf | unclear | - | "The whole chain for `UPPER.md` was driven on the candidate only" |
| 79 | `staged-source-adoption-no-deletion-row-seen-again` | nlf | unclear | - | "seen again … I did not examine it against `no-lost-files`" |
| 80 | `setup-install-commit-carries-settings-entries-json` | nlf | unclear | - | "The candidate's install commit carries `.jigc/settings-entries.json` and the previous release's does not … I did not look into what it is." |
| 81 | `adoption-route-upper-other-platform-not-driven` | wp | unclear | - | "The runner CI uses was not driven." |
| 82 | `not-staged-route-create-or-author-gate-blocked` | wp | **breaks** | VR-DST-1 | "*Create or author the doc in this task first* is printed for a task whose workflow grants no create for the doctype; both acts then exit 1 with `create.gate-blocked`. … Both binaries." |
| 83 | `doc-create-over-strand-second-doc-seen-again` | nlf | unclear | - | "stages a second doc of a committed identity at exit 0 … what its finalize commits was not driven by this verification" |
| 84 | `unregistered-doc-never-adopted-after-reset-strand` | wp | unclear | - | "says of a doc jigc itself created and relocated that it *was never adopted* … Its route was not run." |
| 85 | `reconciliation-rename-routes-unmanage-of-staged-path` | wp | unclear | - | "The route was not run, so what it does to the task's staged doc is not known." |
| 86 | `config-set-root-knob-moves-nothing-non-utf8-entry` | nlf | unclear | - | "lands the knob and moves nothing when the index holds a name that is not UTF-8 — exit 0 … seen as the setup of this block, not pursued" |
| 87 | `validate-silent-about-strand-under-non-utf8-entry` | wp | unclear | - | "`jigc validate` is silent about the strand while the entry stands … Not pursued" |
| 88 | `orientation-run-directives-not-driven-non-utf8-entry` | wp | unclear | - | "The other three `Run:` directives of that block … were not driven in this state on either binary." |
| 89 | `unregistered-doc-route-names-moved-away-path` | wp | unclear | - | "routes at `jigc migrate <old path> --as research` — a file no longer on disk … A state of my own arm's making … the route was not run" |
| 90 | `doc-show-stale-unmanage-offer-placement-and-json` | wp | unclear | - | "The `placement:` home kind of the same route … and `--format json` of every cell. Not driven." |
| 91 | `finalize-after-unmanage-arms-of-show-route-not-driven` | wp | unclear | - | "whether a task's finalize behaves after each arm was not driven" |
| 92 | `live-task-finalize-after-migrate-route-not-driven` | wp | unclear | - | "whether its finalize behaves once the doc sits at `notes/research/` was not driven" |
| 93 | `unregistered-doc-route-undriven-cells` | wp | unclear | - | "A title that slugs differently, the route pasted from a directory outside the repository … none driven." |
| 94 | `unregistered-doc-route-placement-fallback-json-not-driven` | wp | unclear | - | "The other arms of the same producer: the `placement:` wording, the ignore-or-human fallback, and `--format json` of every step." |
| 95 | `ref-resolves-route-create-arm-gate-blocked` | wp | unclear | VR-FIN-1 | "The *drop the field* arm works. If triage reads *every refusal's route works as printed* as every arm of a prose route, this is a row of its own" — one arm of three; the report leaves the reading open |
| 96 | `finalize-ref-resolves-over-stranded-target-names-no-strand` | wp | unclear | - | "the finalize refusal does not name the strand … nothing is committed" |
| 97 | `finalize-after-orphaned-doc-route-remaining-arms` | wp | unclear | - | "Not driven: the *author* half of the un-baselined route on the moved doc …" |
| 98 | `ingest-after-hand-move-keeps-prior-file-state-keys` | nlf | unclear | - | "Whether any door later reads those records was not driven." |
| 99 | `unmanage-stranded-placement-doc-edges-not-exercised` | wp | unclear | - | "whether a stranded placement doc's edges are dropped with its baseline was not exercised" |
| 100 | `finalize-after-placement-strand-arms-not-driven` | wp | unclear | - | "a task's finalize after each arm was not driven" |
| 101 | `migration-finalize-capture-helper-callers-not-enumerated` | wp | unclear | - | "Its other callers were not enumerated and no other door of it was driven." |
| 102 | `migration-approve-commits-staged-file-not-the-migrations` | nlf | unclear | VR-NU-1 (`mig.ascii`) | "the landed commit holds `VISION.md`, `docs/direction.md` and `legacy.txt` … I did not read that policy through or decide whether this is what it intends" |
| 103 | `finalize-non-utf8-text-file-undriven-shapes` | wp | unclear | - | "Other encodings and other shapes of the same state … Not driven." |
| 104 | `rename-mention-scan-other-quoted-names-not-driven` | wp | unclear | - | "by reading the same scan skips such a file under any configuration. Read, not driven." |
| 105 | `line-oriented-git-listing-consumers-not-traced` | wp | unclear | - | "Which of them use the line as a path was not traced here." |
| 106 | `amend-refusal-route-preview-door-not-driven` | wp | unclear | - | "Read, not driven: the preview would print the same route." |
| 107 | `amend-refusal-route-undriven-name-shapes` | wp | unclear | - | "was not driven. Its route was never run by anyone." |
| 108 | `finalize-doors-refuse-staged-non-utf8-name` | wp | unclear | VR-PC-1 | "on a host that holds the name this is a door that does not run. The second clause's question; not graded." |
| 109 | `amend-arm-no-file-state-record-when-note-fires` | nlf | unclear | - | "whether the record is rebuilt was not followed further" |
| 110 | `landed-summary-c-quoted-name-and-no-hash-record` | nlf | unclear | - | "the `file-state` record then holds no hash for it … Not pursued." |
| 111 | `milestone-finalize-removes-worktree-with-untracked-file` | nlf | unclear | - | "Whether a genuinely untracked file in a live sub-task's worktree goes the same way, and which ruling owns that, I did not examine" |
| 112 | `post-commit-capture-real-host-previous-release-not-driven` | nlf | unclear | - | "A host that holds the name was not driven, and neither was the previous release." |
| 113 | `amend-arm-drops-co-author-trailer-undecodable-message` | nlf | unclear | VR-GCMSG-1 | "drops the profile's co-author trailer when `HEAD`'s message does not decode … at exit 0, with no word about it" — a planted object on both binaries, Latin-1 log output on the candidate |
| 114 | `task-finalize-exits-1-after-commit-latin1-log-output` | wp | unclear | VR-GCMSG-1 | "exits 1 after its commit landed when git prints the new subject in a non-UTF-8 encoding … the re-run answers `finalize.no-task`" |
| 115 | `milestone-finalize-exits-1-after-commits-latin1-log-output` | wp | unclear | - | "exits 1 after two commits landed … and leaves the sub-task's worktree registered … What removes that worktree was not driven." |
| 116 | `amend-empty-commit-frame-says-fix-the-hooks-complaint` | wp | unclear | - | "jigc's frame ends *Fix the hook's complaint* where no hook ran. Candidate only, one run" |
| 117 | `milestone-aggregate-subject-site-not-reached` | nlf | unclear | - | "was not reached with an undecodable capture. … not driven." |
| 118 | `commit-encoding-non-utf8-not-driven` | nlf | unclear | - | "`i18n.commitEncoding` set to a non-UTF-8 encoding … Not driven." |

## To the verifier

All 118 keys: 3 graded `breaks`, 115 `unclear`. What each must re-drive is its row's repro
cell. Beyond that:

- **46** — `Repro VP-1` with the hook a link to `/dev/null`; act on no route that names a
  device node; establish what the route prints and what clears the refusal, on both binaries.
- **52** — `Repro V-1` to its sixth step, then that refusal's route as printed (the same
  `--approve`), with neither named cause present.
- **65** — rig CB, steps 1 to 4 of the table under *Triage's question*, and then the route
  step 4 prints, which nobody drove.
- **82** — the second cell of `Repro VR-DST-1`, and the never-written address beside it.
- **24** — `Repro VR-3` does not hold the bare re-run; add it after the refusal.
- **95** — `Repro VR-FIN-1`, the cell `unmanage-create-target`, each arm of the route.
- A row whose *block* column is `-` has no block: its verifier writes one or says why none
  can be written.
- The rows that say *Linux* or *root* (1, 7, 11, 12, 14, 22, 50, 58, 81) need a binary or an
  image the verifier is handed; a verifier handed none says so and returns no verdict.

## Tree state

Clean of anything of mine: nothing edited, staged or committed. The one file I wrote is this
report, in the scratch root, handed over by the one call of the `REPORT:` line.

<!-- end of report -->
