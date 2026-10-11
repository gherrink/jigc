# Recurrence census — does how often an item was noticed predict whether it was confirmed?

Read-only, 2026-10-11, branch `fix/rc24-tier1`. One run: the small canary of 2026-10-08. Nothing was driven,
built or edited; no verdict was re-checked. Every number names its script; the scripts are committed in
the directory `recurrence-census/` beside this file, with the tables they printed, and re-run with
`CENSUS_REPO=<repository root> sh recurrence-census/run-all.sh` (each script writes `<name>.out` there, and
three intermediate `.json` files that are not committed).

**Nothing here is a recommendation.** The last section says what the numbers carry and what they do not.

## 1. What was counted, and from which files

All under `completions/artifacts/M55/stabilization-build/canary-one/` unless a path says otherwise.

| what | file | unit |
|---|---|---|
| the triage rows | `data/entries.tsv`, `run/ledger.md` (source, door, repro pointer), `run/r1/reports/test/triage-p{1..4}.a1.md` (the lines keyed to each row) | 372 rows |
| what each row was made from | the ledger's `source` cell, split by script | 442 source references, 90 reporting agents |
| the reporters' own words | the `Left open…` section of each report under `run/r1/reports/test/` and `run-pending-a2/` | 424 bullets: 335 in 64 reports of the first invocation, 89 in the 19 of the second |
| the hunt findings | the seven instrument reports, by finding id (`RC-6`, `S3`, `DL-1`, `F1`, `ACR-2` …) | 47 sections, 47 of 47 located |
| the verdicts | `data/verified-attempt-1.tsv`, `data/verified-attempt-2-not-on-record.tsv` | 80 verifier runs, 79 keys |
| what a verifier was told to re-drive | the `To verify` sections of triage passes 1–3, `run-pending-a2/triage-p1.a2.md` | per run |
| the whole reports | the same two directories | 90 agents |
| the product's codes | `crates/cli/tests/route_catalog/sites.tsv`; every code-shaped string literal of `crates/cli/src` and `crates/engine/src` | 177 codes; 332 literals |
| the leaf verbs | `VERB_KINDS` in `crates/cli/src/cli.rs` | 48 |
| the earlier census's per-run sort | `runs.json`, `groups.json` — scratch files of `reshape/repro-census.md`, **not committed** with it; committed here as `recurrence-census/earlier-census-scratch/` | 80 runs |

Built by `items.py` (rows), `bullets.py` (bullets and their link to a row), `keys.py` (the key), `model.py`
(the join every count script reads).

**Agents.** `h01`–`h07` the instrument reporters (source pass, driver, reconciler, cross-cutting, the trial arm's
three steps), `s01` the scope step, `a01` the advocate, `d01` the independent drive, `v01`–`v61` the first
invocation's verifiers in the row order of `data/verified-attempt-1.tsv`, `w01`–`w19` the second invocation's
in the row order of the other table. One id is both a verifier (of the row it was handed) and a reporter (of the
rows made from what it left open). `recurrence-census/agents.tsv` maps each id to the record's own reporter name.

**Hop** = triage pass − 1, as in `repro-census.md` → E: 0 an instrument's finding, 3 a row of pass 4.

**Horizon of a run** = the last triage pass whose rows existed when its verifier was launched: the row's own
pass in the first invocation, pass 4 in the second. Only rows up to the horizon count as *earlier*.

### Reused from the two earlier counts, and where this one differs

| figure | earlier | here | why |
|---|---|---|---|
| verifier runs · confirmed · did not reproduce · reproduced, no break · halted | 80 · 18 · 10 · 48 · 4 | the same | read from the same two tables |
| follow-up entries (a verifier's, the advocate's, the drive's left-open rows) | 338 | 338 (320 + 11 + 7) | — |
| findings handed to triage · merged · rows | 437 · 65 · 372 | 442 source references on 372 rows | one finding of the independent drive is a second source of five rows; 442 − 5 = 437 |
| items left open by first-invocation verifiers | 366 (363 handed on) | 315 bullets in those 58 reports' sections | the structured return held more items than the report's section in 32 of 58 reporters; the returns are not committed |
| *names a printed route* | 30 runs, 15 confirmed | taken as it stands | the sort is the earlier census's, **made after its author had read the verdicts** |
| refusal sites · codes | 257 sites, 175 codes, 24 dynamic (`route-registry-facts.md`, a scan at `95f77b57`) | 261 rows, 177 codes, 26 rows `dynamic` | `sites.tsv` is a later list from another scanner; not reconciled here |
| code-shaped literals | 289 (production code only) | 332 | this scan does not blank `#[cfg(test)]` items; it only drops file-name shapes |

## 2. The mechanical key and its coverage

**The key** (`keys.py`, `model.py`). A *code-shaped token* is two or three lowercase hyphenated words joined by
dots, not part of a longer word, path or call. It counts as a **code** when it is a `code` of `sites.tsv`, or a
code-shaped string literal of production source that is no file name (`store.not-staged`, `store.unparseable`
and the other families built through a `code()` method are in `sites.tsv` only as `dynamic`). One extension:
a code spelled with hyphens inside a row's key (`…-setup-install-hook-…` → `setup.install-hook`). The
**command** is a `jigc <leaf verb>` spelled in the ledger's door cell. No step reads.

**Two text depths.** *Row* = the row's key, triage's lines for it and its door cell — what the harness holds
as data. *Row + bullet* = that plus the reporter's own left-open bullet, where the ledger pointer says
`item N` and bullet N is, of all bullets of that section, the one sharing most words with triage's line
(219 of 234 pointers pass; 110 left-open rows point at a heading or block and get no bullet).

Coverage — `q1_coverage.py`:

| item class, text | items | any code-shaped token | … that is no file name | **carry a code** | a code of `sites.tsv` | only a source literal |
|---|---:|---:|---:|---:|---:|---:|
| hunt findings — the reporter's own section | 47 | 32 | 15 | **12 (26%)** | 9 | 3 |
| triage rows of pass 1 (hunt, scope, seed) — row | 34 | 7 | 2 | **2 (6%)** | 2 | 0 |
| triage rows, all — row, dotted tokens only | 372 | 66 | 38 | **36 (10%)** | 30 | 6 |
| **triage rows, all — row (the key)** | 372 | | | **41 (11%)** | 34 | 7 |
| triage rows, all — row + bullet | 372 | | | **65 (17%)** | 58 | 7 |
| follow-up rows (338) — row + bullet | 338 | 94 | 65 | **59 (17%)** | 53 | 6 |
| left-open bullets as written, first invocation | 335 | 85 | 69 | **63 (19%)** | 56 | 7 |
| left-open bullets as written, second invocation | 89 | 24 | 17 | **17 (19%)** | 14 | 3 |
| the 80 verifier runs — row + re-drive line | 80 | 30 | 21 | **21 (26%)** | 19 | 2 |
| the 80 verifier runs — the verifier's basis, *after* the verdict | 80 | 39 | 34 | **34 (42%)** | 28 | 6 |
| whole reports | 90 | 90 | 85 | **81 (90%)** | 78 | 3 |

- **Which codes.** 56 distinct code-shaped tokens in the rows: 19 codes of `sites.tsv`, 6 source literals
  outside it, 29 file names (`doc.rs`, `x.md`, report names), 2 other (`core.ignorecase`, `mig.ascii`). With the
  key's hyphen form: **28 codes at row depth, 21 of them in `sites.tsv`; 40 at row + bullet, 33 in `sites.tsv`.**
  The seven outside it: `store.not-staged`, `store.unparseable`, `write.non-reparseable`, `write.wrong-shape`,
  `write.target-escape`, `finalize.rollback-conflict`, `finalize.left-out` (the last is, by
  `route-registry-facts.md`, no finding code). The 90 reports name 79 codes; 41 of them are in no row.
- **The hyphen form** adds five rows — among them the second regression's row, whose text never spells
  `setup.install-hook` with a dot. One of the five (`setup.install-commit`) may be a coincidence of words.
- **By hop** (row / row + bullet): 2 / 2 of 34 · 5 / 7 of 56 · 7 / 11 of 73 · 27 / 45 of 209 (`q3c_reverse.py`).
  **By triage grade** (row + bullet): *breaks* 3 of 7 · *unclear* 46 of 171 · *needs-bound* 3 of 47 · *no-break* 13 of 147.
- **The command.** 335 of 372 rows spell a leaf verb in the door cell, 298 exactly one; all 41 coded rows do.
  The door is triage's cell, not the code's family: where a coded row has one door verb and a `sites.tsv`
  code, the door matches that code's `command` column in 24 cases and differs in 10
  (`store.not-found` at `start`, `create.gate-blocked` at `doc show`, …).
- **Rows that name a printed route** (the earlier sort, 30 runs): **15 carry a code at row depth, 18 with the
  re-drive line or the bullet — half to three fifths.** Of the 50 others: 2, 4, 5. Blind over all 372 rows: the
  key contains `route` → 22 of 62 carry a code; it does not → 19 of 310.
- **Having a code at all already splits the verdicts** (row + bullet): confirmed 10 of 23 with a code, 8 of 57
  without. It is the route split again: inside the route-named runs a code adds nothing (9 of 18 with, 6 of 12
  without, row + re-drive); outside them 0 of 4 with, 3 of 46 without.

## 3. Recurrence distribution

`q2_recurrence.py`. An *occurrence* is a row whose text carries the code; its agents are the reporters in the
row's source cell (54 rows have several; 3 of those carry a code).

| | codes | noticed once | twice | three times | more |
|---|---:|---:|---:|---:|---:|
| row depth — by rows | 28 | 17 | 7 | 2 | 2 |
| row depth — by **distinct agents** | 28 | 16 | 7 | 4 | 1 |
| row + bullet — by rows | 40 | 21 | 10 | 5 | 4 |
| row + bullet — by **distinct agents** | 40 | 22 | 8 | 6 | 4 |
| row + bullet — by distinct roots (the instrument finding the chain starts from) | 40 | 33 | 4 | 3 | 0 |
| code + command, row + bullet — by distinct agents | 57 pairs | 43 | 8 | 4 | 2 |
| whole reports — by reports naming the code | 79 | 25 | 16 | 9 | 29 |

- An agent repeating itself: 3 codes at row depth, 6 at row + bullet, have two rows from one reporter
  (`reconciliation.conflict-block` and `file-state.baseline-adopt` are two rows of one agent and nothing else).
- **Where the occurrences sit** (row + bullet, 75 occurrences): hop 0: 2 · hop 1: 8 · hop 2: 11 · hop 3: 54.
  Rows per hop: 34 · 56 · 73 · 209. Recurrence is a late thing: of the ten codes that reach three agents, one
  does so at pass 3 (`uninstall.remove-precommit` — the advocate, the drive, one verifier) and nine only at
  pass 4, the pass nobody verified in the first invocation.

Codes noticed by two or more distinct agents, row + bullet:

| code | in `sites.tsv` | rows | distinct agents | hops of the rows | distinct roots | whole reports naming it (of 90) |
|---|---|---:|---:|---|---:|---:|
| `file-state.unregistered-doc` | yes | 5 | 6 | 2 3 3 3 3 | 1 | 7 |
| `store.not-found` | yes | 6 | 5 | 1 2 3 3 3 3 | 3 | 24 |
| `migrate.source-untracked` | yes | 4 | 4 | 2 3 3 3 | 2 | 9 |
| `schema-conformance.unadopted-instance` | yes | 3 | 4 | 3 3 3 | 1 | 12 |
| `setup.install-hook` | yes | 4 | 3 | 1 2 3 3 | 1 | 9 |
| `file-state.orphaned-doc` | yes | 3 | 3 | 1 2 3 | 1 | 10 |
| `reconciliation.rename` | yes | 3 | 3 | 3 3 3 | 3 | 10 |
| `store.not-staged` | no | 3 | 3 | 3 3 3 | 3 | 15 |
| `store.malformed-slug` | yes | 2 | 3 | 0 3 | 1 | 5 |
| `uninstall.remove-precommit` | yes | 2 | 3 | 1 2 | 1 | 3 |
| `write.non-reparseable` | no | 3 | 2 | 2 3 3 | 1 | 6 |
| `conformance.section-missing` | yes | 2 | 2 | 1 2 | 1 | 9 |
| `create.gate-blocked` | yes | 2 | 2 | 3 3 | 1 | 6 |
| `file-state.staged-copy` | yes | 2 | 2 | 3 3 | 1 | 7 |
| `finalize.no-task` | yes | 2 | 2 | 2 3 | 2 | 12 |
| `schema-conformance.orphaned-instance` | yes | 2 | 2 | 0 2 | 2 | 7 |
| `store.unparseable` | no | 2 | 2 | 1 3 | 2 | 11 |
| `setup.install-commit` | yes | 1 | 2 | 3 | 1 | 0 |

The other 22 codes: one agent each. The full table at both depths is `q2_recurrence.out`.

## 4. Recurrence against verdict

### 4a. Forward — confirm rate by how often the code had been noticed at launch

`q3_predict.py`. A run's band is, over the codes its row carries, the largest number of **distinct agents** with
a row naming that code up to the run's horizon; the row's own reporter counts, so **1 is a first sighting**.

| band — row + bullet | runs | confirmed | reproduced, no break | did not reproduce | halted |
|---|---:|---:|---:|---:|---:|
| the row carries no code | 57 | 8 | 37 | 8 | 4 |
| 1 — first sighting | 14 | **4** | 8 | 2 | 0 |
| 2 | 3 | **1** | 2 | 0 | 0 |
| 3 or more | 6 | **5** | 1 | 0 | 0 |

The same question asked other ways:

| variant | no code | 1 | 2 | 3+ |
|---|---|---|---|---|
| row depth, distinct agents | 9 / 63 | 3 / 9 | 2 / 3 | 4 / 5 |
| row depth, rows instead of agents | 9 / 63 | 3 / 10 | 3 / 3 | 3 / 4 |
| row + bullet, rows instead of agents | 8 / 57 | 4 / 15 | 2 / 3 | 4 / 5 |
| key from row + re-drive line; counted over row + bullet | 9 / 58 | 3 / 13 | 1 / 3 | 5 / 6 |
| strict — agents in an *earlier* pass than the horizon (0 · 1 · 2) | 8 / 57 | 4 / 16 | 2 / 3 | 4 / 4 |
| whole reports returned before launch that name the code | 8 / 57 | 1 / 8 | 2 / 4 | 7 / 11 |

Three splits of the main table (`q3b_extras.py`):

- **By route** (the earlier sort). Names a route: no code 5 / 12 · band 1 4 / 10 · band 2 1 / 2 · band 3+ 5 / 6.
  Names none: 3 / 45 · 0 / 4 · 0 / 1 · —.
- **By invocation.** First: 7 / 46 · 3 / 11 · 1 / 3 · 1 / 1. Second: 1 / 11 · 1 / 3 · — · 4 / 5.
- **Band 3+ is six runs, and they are not six independent things.** Five are of the second invocation's one
  batch; four of those sit under one root (`r1-doc-list-unreadable-entry-fails-listing`) and are 4 / 4; three
  of the four carry the same code, `store.not-found`. Under that root the second invocation confirmed 6 of 8
  whatever the band (band 1: 1 / 1, no code: 1 / 3); under its other root 0 of 11 (band 3+: 0 / 1). The sixth
  run is `r1-p3-uninstall-writability-route-not-driven`, confirmed at three agents.
- **Which 19 of 118 the second invocation verified was not a sample:** they are positions 1–17, 20 and 21 of
  the 118 in ledger order, so two roots of six. Of the 99 never verified, 26 carry a code: 9 in band 1, 4 in
  band 2, **13 in band 3+** — unverified.

**Per code, in launch order** (`q3b_extras.py`; agents at launch → verdict; C confirmed, n reproduced/no break, d did not reproduce):

| code | verified rows carrying it |
|---|---|
| `store.not-found` | 1→n · 5→C · 5→C · 5→C |
| `migrate.source-untracked` | 1→n · 4→C |
| `setup.install-hook` | 1→C · 2→C |
| `store.unparseable` | 1→C · 2→C |
| `conformance.section-missing` | 1→C · 2→n |
| `store.not-staged` | 3→C · 3→n |
| `uninstall.remove-precommit` | 3→C |
| `reconciliation.rename` | 3→C |
| `store.malformed-slug` | 2→n |
| `file-state.unregistered-doc`, `file-state.orphaned-doc` | 1→d each |
| nine more codes, one verified row each at one agent | 2 C, 7 n |

Twenty codes had a verified row; twenty more had none (one of those, `schema-conformance.unadopted-instance`,
noticed by four agents). A first verdict on a code said little about the next: `store.not-found` and
`migrate.source-untracked` were *no break* at the first sighting and confirmed later on other rows.

**Raw — the 23 runs whose row carries a code** (`^` = the code is in the reporter's bullet only; rows / agents are per code, at launch):

| run | key | hop | codes | rows / agents at launch | whole reports at launch | verdict |
|---|---|---:|---|---|---|---|
| v05 | `r1-validate-non-utf8-path-drops-blocking-orphan` | 0 | `schema-conformance.orphaned-instance` | 1 / 1 | 1 | reproduced, no break |
| v06 | `r1-doc-list-prints-id-doc-show-refuses` | 0 | `store.malformed-slug` | 1 / 2 | 1 | reproduced, no break |
| v15 | `r1-setup-install-hook-route-other-causes-unexamined` | 1 | `setup.install-hook` | 1 / 1 | 4 | **confirmed, regression** |
| v17 | `r1-doc-show-lowercase-address-prints-path-no-file-has` | 1 | `store.unparseable` | 1 / 1 | 1 | **confirmed** |
| v18 | `r1-adoption-route-for-non-doc-id-name-not-run` | 1 | `conformance.section-missing`^, `ingest.unaddressable-identity`^ | 1 / 1, 1 / 1 | 3, 1 | **confirmed** |
| v19 | `r1-doc-show-stranded-doc-not-found-route` | 1 | `store.not-found` | 1 / 1 | 2 | reproduced, no break |
| v20 | `r1-orphaned-doc-route-after-strand-not-run` | 1 | `file-state.orphaned-doc`^ | 1 / 1 | 1 | did not reproduce |
| v23 | `r1-promote-clobber-route-commits-staged-blob` | 1 | `finalize.promote-clobber` | 1 / 1 | 1 | reproduced, no break |
| v28 | `r1-p3-milestone-merged-area-writer-follows-a-link` | 2 | `finalize.no-task`^ | 1 / 1 | 6 | reproduced, no break |
| v35 | `r1-p3-uninstall-writability-route-not-driven` | 2 | `uninstall.remove-precommit` | 2 / 3 | 2 | **confirmed** |
| v37 | `r1-p3-setup-install-hook-undriven-producers-and-causes` | 2 | `setup.install-hook` | 2 / 2 | 5 | **confirmed, regression** |
| v44 | `r1-p3-doc-author-nothing-persisted-yet-task-bound` | 2 | `write.non-reparseable` | 1 / 1 | 1 | reproduced, no break |
| v45 | `r1-p3-refused-doc-rename-leaves-staged-foreign-copy` | 2 | `conformance.section-missing`^ | 2 / 2 | 5 | reproduced, no break |
| v47 | `r1-p3-adoption-of-staged-source-no-deletion-row` | 2 | `migrate.source-untracked`^ | 1 / 1 | 2 | reproduced, no break |
| v54 | `r1-p3-unregistered-doc-route-after-unmanage-not-run` | 2 | `file-state.unregistered-doc`^ | 1 / 1 | 1 | did not reproduce |
| w02 | `r1-p4-unparseable-adoption-route-refuses-untracked-source` | 3 | `migrate.source-untracked`, `store.unparseable` | 4 / 4, 2 / 2 | 8, 8 | **confirmed** |
| w03 | `r1-p4-not-staged-route-mode-000-repro-b-not-redriven` | 3 | `store.not-found` | 6 / 5 | 18 | **confirmed** |
| w05 | `r1-p4-nothing-staged-route-git-add-fails-unreadable-file` | 3 | `finalize.nothing-staged` | 1 / 1 | 2 | **confirmed** |
| w08 | `r1-p4-not-found-not-staged-routes-loop-at-resume-door` | 3 | `store.not-found`, `store.not-staged` | 6 / 5, 3 / 3 | 18, 9 | **confirmed** |
| w09 | `r1-p4-reconciliation-rename-route-jigc-rename-not-found` | 3 | `reconciliation.rename`, `store.not-found` | 3 / 3, 6 / 5 | 8, 18 | **confirmed** |
| w15 | `r1-p4-finalize-refuses-tracked-link-home-previous-release-landed` | 3 | `store.home-not-regular-file` | 1 / 1 | 5 | reproduced, no break |
| w18 | `r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc` | 3 | `milestone.staged-prose`, `store.not-staged` | 1 / 1, 3 / 3 | 1, 9 | reproduced, no break |
| w19 | `r1-p4-milestone-finalize-missing-provenance-route-no-command` | 3 | `join.missing-provenance` | 1 / 1 | 1 | reproduced, no break |

### 4b. Reverse — each confirmed finding: how often had its code been noticed before it was verified?

`q3c_reverse.py`. *Other agents* = distinct agents, other than the row's own reporters, with a row (row +
bullet) naming the code at launch. `*` = the code is named only by the verifier's basis, **after** the verdict;
the workflow could not have used it.

| run | key | hop | code | other rows | other agents | whole reports at launch |
|---|---|---:|---|---:|---:|---:|
| v04 | `r1-setup-installs-outside-work-tree` | 0 | `setup.install-hook`* | 0 | 0 | 3 |
| v15 | `r1-setup-install-hook-route-other-causes-unexamined` — **regression** | 1 | `setup.install-hook` | 0 | 0 | 4 |
| v17 | `r1-doc-show-lowercase-address-prints-path-no-file-has` | 1 | `store.unparseable` · `migrate.source-untracked`* | 0 · 0 | 0 · 0 | 1 · 0 |
| v18 | `r1-adoption-route-for-non-doc-id-name-not-run` | 1 | `conformance.section-missing` · `ingest.unaddressable-identity` · `write.non-reparseable`* | 0 · 0 · 0 | 0 · 0 · 0 | 3 · 1 · 0 |
| v24 | `r1-p3-not-staged-route-task-less-read-refuses` | 2 | `store.not-staged`* · `store.unparseable`* | 0 · 1 | 0 · 1 | 4 · 3 |
| v34 | `r1-p3-setup-writability-route-printed-for-other-causes` | 2 | none, before or after | – | – | – |
| v35 | `r1-p3-uninstall-writability-route-not-driven` | 2 | `uninstall.remove-precommit` | 1 | 2 | 2 |
| v37 | `r1-p3-setup-install-hook-undriven-producers-and-causes` — **regression** | 2 | `setup.install-hook` | 1 | 1 | 5 |
| v40 | `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` | 2 | `migrate.source-untracked`* | 1 | 1 | 2 |
| v43 | `r1-p3-lowercase-address-other-doors-not-driven` | 2 | `finalize.commit-rejected`* | 0 | 0 | 0 |
| v49 | `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc` | 2 | `store.not-staged`* | 0 | 0 | 4 |
| v59 | `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path` | 2 | `finalize.amend-index-dirty`* | 0 | 0 | 1 |
| w02 | `r1-p4-unparseable-adoption-route-refuses-untracked-source` | 3 | `migrate.source-untracked` · `store.unparseable` | 3 · 1 | 3 · 1 | 8 · 8 |
| w03 | `r1-p4-not-staged-route-mode-000-repro-b-not-redriven` | 3 | `store.not-found` · `store.not-staged`* | 5 · 3 | 4 · 3 | 18 · 9 |
| w04 | `r1-p4-not-staged-block-other-unservable-states-not-enumerated` | 3 | `store.not-found`* · `store.not-staged`* · `store.unparseable`* | 6 · 3 · 2 | 4 · 3 · 1 | 18 · 9 · 8 |
| w05 | `r1-p4-nothing-staged-route-git-add-fails-unreadable-file` | 3 | `finalize.nothing-staged` | 0 | 0 | 2 |
| w08 | `r1-p4-not-found-not-staged-routes-loop-at-resume-door` | 3 | `store.not-found` · `store.not-staged` | 5 · 2 | 4 · 2 | 18 · 9 |
| w09 | `r1-p4-reconciliation-rename-route-jigc-rename-not-found` | 3 | `reconciliation.rename` · `store.not-found` | 2 · 5 | 2 · 4 | 8 · 18 |

**Of the 18 confirmed, with what the workflow held at launch:** 8 carried no code; 4 were a first sighting
(nobody else had named the code: `v15`, `v17`, `v18`, `w05`); 1 a second (`v37`); 1 a third (`v35`); 4 had been
named by three or four other agents (`w02`, `w03`, `w08`, `w09` — one root, one batch). **Both regressions were
verified at the first and the second sighting of `setup.install-hook`.**
With the basis codes added (hindsight): 17 of 18 have a code; no other agent had named it for 9, one for 3,
two for 1, three or more for 5. An earlier whole report had named one of its codes for 16 of 18.

**The ten that did not reproduce:**

| run | key | hop | code | other rows | other agents | whole reports at launch |
|---|---|---:|---|---:|---:|---:|
| v01 | `canary-seeded-claim` (the plant) | 0 | none | – | – | – |
| v08 | `r1-finalize-may-commit-redirected-log-record` | 0 | none | – | – | – |
| v12 | `r1-staging-area-writers-not-enumerated` | 1 | none | – | – | – |
| v20 | `r1-orphaned-doc-route-after-strand-not-run` | 1 | `file-state.orphaned-doc` | 0 | 0 | 1 |
| v29 | `r1-p3-staging-area-writers-undriven-and-unread-remainder` | 2 | none | – | – | – |
| v42 | `r1-p3-doc-show-lowercase-address-case-sensitive-volume` | 2 | `store.not-found`* | 2 | 2 | 7 |
| v48 | `r1-p3-adoption-route-title-upper-undriven-cells` | 2 | none | – | – | – |
| v54 | `r1-p3-unregistered-doc-route-after-unmanage-not-run` | 2 | `file-state.unregistered-doc` | 0 | 0 | 1 |
| v55 | `r1-p3-finalize-after-orphaned-doc-route-arms-not-driven` | 2 | `file-state.un-baselined` · `schema-conformance.ref-resolves`* | 0 · 0 | 0 · 0 | 1 · 0 |
| v56 | `r1-p3-orphaned-doc-placement-arm-and-json-not-driven` | 2 | `file-state.orphaned-doc`* | 2 | 1 | 3 |

Seven carried no code at launch; the three that did were first sightings.

| verdict | runs | row carried a code at launch | a code by row or basis | no other agent had named it | 1 other | 2 | 3+ | an earlier whole report had named one |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| confirmed | 18 | 10 | 17 | 9 | 3 | 1 | 5 | 16 |
| did not reproduce | 10 | 3 | 5 | 8 | 1 | 1 | 0 | 5 |
| reproduced, no break | 48 | 12 | 17 | 44 | 2 | 2 | 0 | 14 |
| halted | 4 | 0 | 0 | 4 | 0 | 0 | 0 | 0 |

(The four columns from *no other agent* on use row **and** basis codes, so they contain hindsight.)

## 5. Duplicates, and what a merge by code would have done

`q4_duplicates.py`.

**Verifications of something already verified under the same key.**

| key | pairs of verified rows sharing it | runs in a pair | runs launched after a same-key verdict existed | pairs launched in one batch | both confirmed | confirmed + no break |
|---|---:|---:|---:|---:|---:|---:|
| code, row depth | 9 | 9 | 5 | 4 | 5 | 4 |
| code, row + bullet | 11 | 12 | 6 | 4 | 5 | 6 |
| code + command, either depth | 3 | 6 | 3 | 0 | 2 | 1 |

The six runs launched after a verdict on the same code (row + bullet), earlier verdict → theirs:
`v37` C → **C** (`setup.install-hook`) · `v45` C → n (`conformance.section-missing`) · `w02` C and n → **C**
(`store.unparseable`, `migrate.source-untracked`) · `w03`, `w08`, `w09` n → **C** (`store.not-found`).
In four of the six the earlier verdict on the code was the opposite of the later one. No pair has two
verdicts of *no break* or *did not reproduce*: under this key, no verification repeated a refuted one.

**What a mechanical merge would have folded.** Rows taken in pass order; a row folds into the earliest
earlier row sharing a key.

| merge by | rows folded | inside one pass | across passes | were to be verified | were verified |
|---|---:|---:|---:|---:|---:|
| what triage did (its judgment) | 65 of 437 handed in | 65 | 0 | | |
| code, row depth | 19 | 10 | 9 | 15 | 7 |
| code + command, row depth | 12 | 3 | 9 | 8 | 4 |
| code, row + bullet | 34 | 20 | 14 | 25 | 8 |
| code + command, row + bullet | 20 | 7 | 13 | 13 | 4 |

- Triage's *about one in seven* is all inside one pass. Across passes it folded nothing: its reports of passes
  2 to 4 say the ledger it was handed held one row, so *found again* could not be judged. The mechanical
  merge is mostly late: 28 of the 34 folds are rows of pass 4.
- **The eight verified rows a merge by code would have folded, and into what** (row + bullet):

  | folded row — its verdict | into — its verdict |
  |---|---|
  | `r1-p3-setup-install-hook-undriven-producers-and-causes` — confirmed, regression | `r1-setup-install-hook-route-other-causes-unexamined` — confirmed, regression |
  | `r1-p4-unparseable-adoption-route-refuses-untracked-source` — confirmed | `r1-doc-show-lowercase-address-prints-path-no-file-has` — confirmed |
  | `r1-p3-uninstall-writability-route-not-driven` — confirmed | `r1-uninstall-refuses-and-force-half-removes-no-work-tree-home` — *needs-bound*, never verified |
  | `r1-p4-not-staged-route-mode-000-repro-b-not-redriven` — confirmed | `r1-doc-show-stranded-doc-not-found-route` — reproduced, no break |
  | `r1-p4-not-found-not-staged-routes-loop-at-resume-door` — confirmed | the same row — reproduced, no break |
  | `r1-p4-reconciliation-rename-route-jigc-rename-not-found` — confirmed | the same row — reproduced, no break |
  | `r1-p3-refused-doc-rename-leaves-staged-foreign-copy` — reproduced, no break | `r1-adoption-route-for-non-doc-id-name-not-run` — confirmed |
  | `r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc` — reproduced, no break | `r1-p4-not-found-not-staged-routes-loop-at-resume-door` — confirmed |

  **Six of the 18 confirmed findings would have been folded away** — three into a row verified *no break*,
  one into a row nobody verified, two into another confirmed finding — and three further confirmed rows
  are rows something is folded into. By the verdicts alone — no reading — five of the eight folds join rows
  whose verdicts differ, one joins a confirmed finding to a row nobody verified, and two join two findings
  the record lists separately (the two regressions; two of the confirmed that are no regression).
  With the command added, four folds remain: the two confirmed/confirmed pairs, the uninstall row, and one
  confirmed into a *no break* (`store.not-found` at `jigc doc show`).

  **[Added 2026-10-11 by the orchestrating session, which committed the scripts and ran them again.]** One
  of the six, `r1-p4-unparseable-adoption-route-refuses-untracked-source`, carries two codes, and which
  earlier row it folds into depends on which is read first: `store.unparseable` joins it to a confirmed
  finding, as counted above, and `migrate.source-untracked` to a row verified *no break*. Read the other
  way the six are four into a *no break*, one into a row nobody verified and one into another confirmed.
  The scripts took whichever a set yielded first, so two runs could print either; `run-all.sh` now fixes
  the seed, and a re-run prints the committed tables byte for byte. Over six seeds a label in
  `q5_nocode.out` and the order of some rows and dictionaries moved the same way, and no other figure did.
- **Against triage's own notes of restatement.** Triage named, without folding, 37 pairs where a row restates
  an earlier one (passes 2, 3, 4 and the second invocation's *kin rows*). **Two of the 37 share a code**; 14
  have a code on either side at all; 22 share a door verb. So the key finds one in eighteen of the
  restatements an agent saw, and what it does fold is mostly not on that list.

**How many of the folds would have been wrong — my reading, not a count.** I read triage's one line for each
of the 54 rows in the 19 code groups of two or more (row + bullet); no report was opened. Per group, rows →
distinct defects as I read them:
`store.not-found` 6 → 4 · `file-state.unregistered-doc` 5 → 4 or 5 · `setup.install-hook` 4 → 3 or 4 ·
`migrate.source-untracked` 4 → 3 · `file-state.orphaned-doc`, `store.not-staged`, `reconciliation.rename`,
`schema-conformance.unadopted-instance` 3 → 3 each · `write.non-reparseable` 3 → 2 · `store.malformed-slug`
2 → 1 · the other nine groups 2 → 2 each. **About 6 of the 35 folds would join the same defect; about 29
would join different ones.** The right ones are restatements (`…-seen-again` rows, the mode-000 loop named
three times).
On the two regressions: both rows are `setup.install-hook` at `jigc setup`, and by
`route-registry-facts.md` → 6 they are the same two producers under different causes (an absolute against a
relative link; different causes of the writability route). Folded, the second verification — which added the
relative link and three more causes — would not have run. Whether that is one defect or two depends on
whether the unit is the producer or the cause; the code cannot tell. And two rows about the same code carry
no code at all: `r1-p3-setup-writability-route-printed-for-other-causes` (confirmed) and
`r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` (graded *breaks*).

## 6. Items without a code

`q5_nocode.py`. 331 of 372 rows carry no code at row depth; 63 of the 80 runs are of such rows (9 confirmed).
Each key below is a regular expression over the row text or a committed column.

| key tried | no-code rows that have it | distinct values | values on one row only | largest value | rows sharing a value |
|---|---:|---:|---:|---|---:|
| the door verb (triage's door cell) | 294 (89%) | 26 | 1 | `task finalize`, 55 rows | 294 |
| `jigc <verb>` spelled in the row text | 296 (89%) | 26 | 1 | `task finalize`, 55 | 296 |
| a source file named (`x.rs`) | 13 (4%) | 15 | 12 | `doc.rs`, 2 | 6 |
| a backticked `snake_case` identifier | 7 (2%) | 7 | 7 | — | 0 |
| a git config name | 7 (2%) | 5 | 4 | `core.hookspath`, 3 | 3 |
| a design or implementation doc named | 10 (3%) | 5 | 2 | `implementation/decisions-pending.md`, 4 | 8 |
| the repro block the pointer names (report + block id) | 90 (27%) | 60 | 42 | one block, 5 | 48 |
| three consecutive words of the row's key | 331 (100%) | 1,564 | 1,455 | `no-work-tree`, 6 | 152 |
| two consecutive words of the row's key | 330 (100%) | 1,132 | 965 | `doc-list`, 29 | 254 |

Confirm rate of the 63 no-code runs by how many distinct agents had a row with the same value at launch:

| key | no value | 1 | 2 | 3–5 | 6+ |
|---|---|---|---|---|---|
| door verb | 0 / 8 | 1 / 4 | 0 / 1 | 2 / 8 | 6 / 42 |
| repro block named | 5 / 48 | 3 / 12 | 0 / 1 | 1 / 2 | – |
| three key words | – | 7 / 40 | 1 / 13 | 1 / 10 | – |
| two key words | – | 6 / 30 | 1 / 9 | 1 / 13 | 1 / 11 |

- **The keys that cover are topics, not items.** A door verb is one of 26 values with a median of five rows;
  key words group by subject (`doc-list` 31 rows, `non-utf8` 19, `unreadable-entry` 15). Under both, more
  recurrence goes with *fewer* confirmations: the most-noticed door over all 372 rows is the round's own,
  `jigc doc list` — 50 rows, 29 agents, **0 of 12** verified rows confirmed — and `jigc doc show`, 18 rows,
  has 6 of 9. The most frequent key words are the round's door and the two planted states.
- **The keys that are specific do not cover**: a named source file, identifier or config name is on 2–4% of
  the rows. The named repro block is on 27% and says only that two rows came from one block of one report.
- **No mechanical key found that says two no-code rows are the same item.** I made no grouping by reading.
  The only grouping by judgment on record is triage's own 37 restatement pairs (§5); 23 of them have no code
  on either side.

## 7. The confound — is a much-noticed code simply on the path?

`q6_confound.py`. Two different things are both called *noticed*:

- **Named anywhere in an agent's report** — met on the way. 79 codes over 90 reports; a verifier's report
  names a median of 3 codes, a median of 3 of which its handed row did not carry; 5 of 80 name none.
- **Left open as an item** — a row. 40 codes over 65 rows.

For the most-named codes, over the 80 verifier reports. *Sent for it* = the handed row carried the code.

| code | verifier reports naming it | sent for it | not sent for it | … and handed a row at a door no row with this code has | rows naming it | reports per row |
|---|---:|---:|---:|---:|---:|---:|
| `store.not-found` | 24 | 4 | 20 | 10 | 6 | 4.0 |
| `store.not-staged` | 14 | 2 | 12 | 5 | 3 | 4.7 |
| `schema-conformance.unadopted-instance` | 12 | 0 | 12 | 6 | 3 | 4.0 |
| `store.unparseable` | 11 | 2 | 9 | 5 | 2 | 5.5 |
| `file-state.orphaned-doc` | 10 | 1 | 9 | 7 | 3 | 3.3 |
| `migrate.source-untracked` | 9 | 2 | 7 | 3 | 4 | 2.2 |
| `store.home-not-regular-file` | 8 | 1 | 7 | 7 | 1 | 8.0 |
| `reconciliation.rename` | 8 | 1 | 7 | 4 | 3 | 2.7 |
| `finalize.no-task` | 7 | 0 | 7 | 5 | 2 | 3.5 |
| `write.identity-change` | 7 | 0 | 7 | 6 | 1 | 7.0 |
| `file-state.unregistered-doc` | 7 | 1 | 6 | 2 | 5 | 1.4 |
| `setup.install-hook` | 6 | 2 | 4 | 0 | 4 | 1.5 |

How many of the verifiers handed a row at a door name the code, whatever they were sent to verify (runs with
one door verb; doors with four or more runs):

| code | `doc list` (12) | `validate` (10) | `task finalize` (10) | `doc show` (9) | `setup` (6) |
|---|---:|---:|---:|---:|---:|
| `store.not-found` | 2 | 5 | 1 | **8** | 0 |
| `store.not-staged` | 2 | 0 | 0 | **6** | 0 |
| `store.unparseable` | 2 | 0 | 0 | **6** | 0 |
| `migrate.source-untracked` | 0 | 2 | 1 | 4 | 0 |
| `file-state.orphaned-doc` | 1 | 3 | 1 | 3 | 0 |
| `setup.install-hook` | 0 | 0 | 0 | 0 | **5** |

**The answer has two halves.**

- **At the report level, yes.** `store.not-found` is named by 8 of the 9 verifiers handed a `jigc doc show`
  row and `setup.install-hook` by 5 of the 6 handed a `jigc setup` row, whatever the row said; 20 of the 24
  reports naming `store.not-found` were not sent for it. Counting reports that name a code measures which
  door the fan-out is standing at. (`refs-post-hoc`, one fixture, is named in 42 of the 80 reports;
  `store.not-found` in 18 of those and 6 of the other 38 — a co-occurrence, nothing more.)
- **At the row level, less so — but the rows are not independent either.** Most agents that met a code did
  not leave it open (four reports name `store.not-found` for every row that does). And 65 of the 75
  row-occurrences come from a reporter that had **not** been handed that code, so the count is not an echo of
  the brief — except for `setup.install-hook`, where 3 of 4 rows come from a verifier sent for that very code.
  But 33 of the 40 codes sit under one root, and the six runs of band 3+ are five of one batch and four of one
  root (§4a): the agents that agree are children of the same few findings, walking the same door.
- **Under the keys that cover everything, recurrence points the wrong way** (§6): the round's door is the
  most-noticed and the least confirmed.

## 8. Bounds

- **One run, one door, one reviewer's roots.** The round's door was `jigc doc list`; all 18 confirmed findings
  descend from four findings of one instrument report, and none is at the round's door.
- **Sample sizes.** 80 runs, 18 confirmed. 23 runs carry a code at launch (row + bullet); band 2 is 3 runs,
  band 3+ is 6 — five of one batch, four of one root, three of one code. No interval is given because the
  runs are not independent; a difference of 5 of 6 against 4 of 14 on this structure is not a rate.
- **Which rows were verified was not random.** The harness verified three passes and then the first 19 of 118
  in ledger order. 13 rows in band 3+ were never verified; their verdicts could move every figure of §4.
- **A verdict is its verifier's word.** Nothing was re-driven. 48 of the 62 non-confirmed are *reproduced, not
  a break* — a reading of a clause, not a failed reproduction (`repro-census.md` → 3).
- **Pass-granular time.** A horizon is a triage pass; inside a pass verifiers ran eight at a time and their
  returns were not read until the next triage, so this is what the workflow could have known, not wall time.
- **Where an agent's judgment entered what the key reads.** Whether a row carries a code depends on triage's
  wording of it: the reporters' own bullets carry a code in 19%, triage's rows in 11%. The door cell is
  triage's. A row's agents include those triage merged in. The second invocation's 99 left-open items have no
  row and were counted only as bullets.
- **Where a mechanical step may be wrong.** A bullet is tied to a row by position and a word-overlap check
  (219 of 234 pass; not read). The structured returns the pointers number are not committed, and 32 of 58
  reports hold fewer bullets than their return held items. The source-literal vocabulary includes test modules
  and at least one name that is no finding code. `sites.tsv` is this branch's list, not the canary
  candidate's tree; the two were not compared.
- **Where my judgment entered.** One place: the count of distinct defects per code group in §5. Two more are
  inherited and marked: the route sort (the earlier census's, made after the verdicts) and triage's 37
  restatement pairs.
- **Not done.** No blind re-sort of *names a route*. No resolution of a pointer to a repro block's text. No
  check that a code named in a row is the code the refusal actually printed.

## 9. What the numbers do and do not support

A code is a key for about one row in six at best (17% with the reporter's own words, 11% from triage's row,
half to three fifths of the rows that name a printed route), and 8 of the 18 confirmed findings carried no
code when their verifier was launched.

Among the 23 verified rows that did carry one, the rows whose code several agents had already named were
confirmed more often (5 of 6 at three or more agents, against 4 of 14 at a first sighting) — but those six are
one batch, mostly one root and largely one code, both regressions were found at the first and second sighting
of their code, and a row merely *having* a code already separates 10 of 23 from 8 of 57, which is the earlier
census's route split under another name. So this run does not show that recurrence adds anything beyond
"names a route with its code", and it does not show that it adds nothing; 13 rows that would decide it were
never verified.

"The same item" cannot be decided by the code: a merge by code would have folded away six of the 18
confirmed findings — four or five of them into a row that was refuted or never verified, by which of
one finding's two codes is read first — while finding 2 of the 37
restatements triage itself named; adding the command removes half of those folds and still joins a
confirmed finding to a refuted row.

For the rows without a code, no mechanical key identifies an item, and the keys that do cover them — the
door, the words of the row's key — count where the fan-out is standing: by those, the most-noticed thing in
the run is the round's own door, where 0 of 12 verified rows were confirmed.
