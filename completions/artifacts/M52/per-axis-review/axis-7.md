<!-- M52 per-axis review (re-run) — axis 7 · composed surfaces — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit a3eb026b), 2026-09-21. -->

<!-- M52 per-axis review RE-RUN — axis 7 · freeze & migration — the OPUS DRIVER table.
     Driven on the installed `jigc 1.0.0-rc.16`, 2026-09-21. No fixes, no commits, no repo edits. -->

# M52 per-axis review (RE-RUN) — AXIS 7 · freeze & migration — the Opus driver

**Binary:** `/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.16`** — asserted before any drive
(`jigc --version` → `jigc 1.0.0-rc.16`, exit 0). **RELEASE posture**: the debug route-fence panics do
not exist here.
**Repo HEAD while driving:** `a3eb026b` (clean tree) — the commit that closed M52's audit, so all
seven audit fixes (`79e54c75 6d95756c fe8f29c4 c96137e4 b9ab6a70 1b036264` + the record commit) are
in this binary. Confirmed **by driving**, not by reading: F1's brownfield control is silent (§4 C-4),
F2's stranded-doc sweep fires (§3.2), F5's `relocate.frozen-doctype` is minted (§3.2), F6's three
removal routes are distinct (§4).
**Rigs:** `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
— two-step eval throughout, no `rm -rf` on a variable path anywhere, no teardown (every root is a
`mktemp -d`). The Codex source pass for this axis was **not read**.

---

## 1 · The door set and the registries, read from the code at `a3eb026b`

Counts I **read** (not taken from the design doc's numerals):

| registry | file:symbol | rows I counted | how |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1834` | **47** leaves | parsed the const block's `(&[…], VerbKind::…)` rows |
| `PATH_ARG_OCCURRENCES` | `cli.rs:3110` | **14** | `PathArgOccurrence {` rows |
| `DOCTYPE_DOORS` | `cli.rs:2522` | **16** rows, **10** `DoctypeArg::Address` | rows / `DoctypeArg::Address` hits inside the block |
| `SLUG_DOORS` | `cli.rs:2829` | **6** | `SlugDoor {` rows |
| `WORK_UNIT_ID_DOORS` | `cli.rs:2589` | **25** | `WorkUnitIdDoor {` rows |
| `BEHALF_DOORS` | `cli.rs:2003` | **47** (total classification, one row per leaf) | `BehalfDoor {` rows |
| `COMMITTING_DOORS` | `invocation_log.rs:130` | **10** | `CommittingDoor {` rows |
| `DESTROYING_DOORS` | `milestone.rs:3275` | **6** (`[&DestroyingDoor; 6]`) | the array type — **was 4 at M51** |
| `STORE_EXIT_FLIPS` | `render.rs:968` | **7** — `probe-unreliable · oob-rename · unmigrated-corpus · ahead-corpus · orphaned-instance · home-vacated · foreign-squatter` | `id:` literals in the block — **`home-vacated` is the new 7th** |
| `ManifestKind::ALL` | `render.rs:1985` | **6** | the array type |
| `SchemaChangeKind::ALL` | `schema_diff.rs:757` | **18** | the array type |
| `LOCI` | `schema_diff.rs:144` | **3** (derived `MAX_NESTING_DEPTH + 1`) | computed, not written down |
| `ROLLBACK_POPULATIONS` | `rollback.rs:169` | **11** | `Population {` rows |
| `TASK_AREA_FILES` | `engine/src/state.rs:129` | **13** | entries in the const |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **3** — `cwd-unreadable · packs-yaml-malformed · pack-resource-missing` | `Fault {` rows (test-side registry, `pub(crate)`, read by flow-53 arm 6) |
| `ENVELOPE_OWED_CODES` | `render.rs:5415` | **4** — `store.not-found · store.no-such-leaf · store.unknown-type · task::FIXED_IDENTITY` | the const's members |
| `RelocateRefusal::ALL` | `relocate.rs:91` | **10** members / 9 codes | the array |
| `InProgress::ALL` | `repo.rs:234` | **9** | `[InProgress; 9]` |

**Axis 7's door set** is the eight the acceptance design names, each a `VERB_KINDS` leaf:
`describe` · `doc schema` · `validate` · `doc list` · `migrate-corpus` · `migrate` · `ingest` ·
`unmanage`. Of the caller-token registries, `migrate`/`unmanage` are `PATH_ARG_OCCURRENCES` members,
`doc schema`/`doc list` are `DOCTYPE_DOORS` members, and `migrate-corpus` is a `COMMITTING_DOORS`
member — which is why its posture refusals had to be driven (§5) and its exit code asserted.

Two **evidence doors** are driven below and carry falsifying data for axis-7 rows, but are not cells
of this axis's own table: **`doc show`** and **`relocate`**.

**The cell set**, three unions, exactly as the design writes it:
**(a)** the six freeze/adoption conditions + the clean control → 7 conditions × 8 doors = **56** rows;
**(b)** `SchemaChangeKind::ALL × LOCI` = 18 × 3 = **54** cells at the `migrate-corpus` door;
**(c)** `ManifestKind::ALL` = **6**.

---

## 2 · M51 rows: CLOSED / STILL-OPEN

The first deliverable. Every §A row of M51's axis-7 ledger, re-driven on rc.16.

| M51 row | verdict on rc.16 | argv + observed |
|---|---|---|
| **D-1** — `jigc ingest` durably absorbs an out-of-band edit and surfaces nothing | **CLOSED** | `jigc ingest` → exit 0 and now prints `advisory · file-state.absorbed — out-of-band edit to \`VISION.md\` absorbed…` with an **Informational** route naming the finding it retired; `--format json` carries it in the top-level `findings` array with key `{code: file-state.absorbed, target: VISION.md}`. The undrifted control run prints nothing and `findings: []`. Repro §4 R-D1. |
| **D-2** — a `Relocated`-only bump is invisible to `migrate-corpus` (both home kinds), and the refusal that routes there is a dead end | **CLOSED** on all three legs | **placement arm:** `jigc migrate-corpus` → exit 0, `1 migrated / migrated HISTORY.md / committed 22d78da`; `doc list` → `changelog:changelog HISTORY.md managed`; `CHANGELOG.md` gone from disk. **location arm:** `migrate-corpus` → `1 migrated docs/adrs/use-sqlite.md`, committed. **route leg:** `jigc relocate adr --from docs/decisions` → exit 1, **`relocate.frozen-doctype`** (a code, where rc.15 had none) with a route naming `jigc migrate-corpus`, and that verb, run verbatim, **acts**. Repro §3.2. |
| **D-3** — `ValueRemapped` on an `id-from` enum falls through to `migrate-corpus.prose-needed`, a dead end | **CLOSED** | `jigc migrate-corpus` → exit 1, **`migrate-corpus.fold-refused`** (never `prose-needed`), cause names the `id-from` role verbatim (*"`category` is that block's `id-from` … remapping it would re-mint the identity of every item"*) and states *"no prose authored into `CHANGELOG.md` and no re-run changes it"*; the route is actionable (re-author the bump). `git status` empty; bytes untouched. Repro §3.2. |
| **D-4 half 1** — the orphan finding claims *"no schema in the composed set says what this file is"* | **CLOSED** | `jigc validate` → exit 1, message now reads *"…sits at a jigc-managed home and carries a `schema-version:` stamp, but **no resolved doctype claims this path** — its type may be defined by nothing in the composed set, or defined and homed elsewhere"*. The false clause is gone. Repro §3.1 (d). |
| **D-4 half 2** — `ingest` contradicts `validate` at the same commit (*fine to stay plain — no action needed*) | **STILL-OPEN** | At one commit: `jigc validate` → exit 1 with two `schema-conformance.orphaned-instance` rows, and `jigc ingest` → exit 0 filing both under `unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)` + legend *"staying a plain file is a legitimate end-state — **no action needed**"*; `--format json` rows carry `finding: null, annotations: []`. Reported as **A7-F2**. Repro §4 R-F2. |
| **C-1** — `ingest` and `unmanage` disagree on a placement document's identity; the edge from the hidden identity survives the unmanage | **CLOSED**, both halves | `jigc ingest` on a committed `VISION.md` carrying `grounded-in: [research:context-loss]` writes `.jigc/index/edges.json` with `"from": "vision:vision"` (rc.15 wrote `vision:VISION`); the following `jigc unmanage VISION.md` leaves `"edges": []`. Runs 2 and 3 of `unmanage` are honest no-ops (`no-op: … is not managed (nothing to drop)`). Repro §4 R-C1. |
| **C-2** — the orphan territory bound misses a **root-placement** orphan (declared residual 2) | **STILL-OPEN, as declared** | Departed-doctype fixture, `VISION.md` stamped and formerly managed at a repo-root placement home: mentions of `VISION.md` — `validate` **0**, `doc list` **0**, `ingest` **0**, `migrate-corpus` **0**; `head -3 VISION.md` still shows `schema-version: 1`. This is `orphan.rs`'s declared residual 2, carried by decision on the namespaced-stamp trigger and recorded `UNPINNED` in M51's README (a standing test would pin the invisibility as expected output, which `pinning.md` §5 refuses). **Not re-reported as a new finding.** Repro §4 R-C2. |
| **O-1** (M51 observation) — the two arms of the freeze check print two different surfaces: the project-layer arm carries a `route:`, the pack-layer arm carries **none**, and `--format json` returns a bare `{"error": …}` | **STILL-OPEN, unchanged, and still not a defect** | Driven at all eight doors (project arm) and at five (pack arm): both are `anyhow` channels, so the route floor does not bind them (`engine::finding::is_route_exempt` disposes the class by name). Recorded again as an asymmetry. §3.1 (f)/(g). |
| **O-2** (M51 observation) — the F1 territory narrowing is real and both residuals are live | **holds** | A stamped `.md` inside `docs/` fires `orphaned-instance`; the repo-root placement home is silent — see C-2 above. |

**Score: 5 CLOSED · 2 STILL-OPEN (one of them a declared, decided residual) · 2 observations carried.**

---

## 3 · The table — every row DRIVEN on the installed rc.16

Route kind: `M` = Mechanical (an argv), `H` = Human, `I` = Informational, `—` = none.

### 3.1 · Cell set (a) — the eight doors × the clean control + six freeze/adoption conditions

Fixture per condition in §4. Rows are grouped by cell; the argv differs only by door within a group.

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| a1 | `describe` | clean control | `jigc describe` | 0 | none | I | the catalog body renders | matches |
| a2 | `doc schema` | clean | `jigc doc schema changelog` | 0 | none | — | `doctype: changelog (schema-version 2)` | matches |
| a3 | `validate` | clean | `jigc validate` | 0 | `schema-conformance.repeatable-populated` (advisory) | H | `1 finding(s) — report-only at store scope (exit 0)` | matches |
| a4 | `doc list` | clean | `jigc doc list` | 0 | none | — | 4 rows, all `managed` | matches |
| a5 | `migrate-corpus` | clean | `jigc migrate-corpus` | 0 | none | — | `0 migrated, 4 already current, 0 blocked` | matches |
| a6 | `migrate` | clean | `jigc migrate foreign.md --as adr` (source **staged** first) | 0 | none | I | `task minted: migrate-adr-foreign-0a629e95a3c6` | matches |
| a6′ | `migrate` | clean, **unstaged** source | `jigc migrate foreign.md --as adr` with the file untracked | **1** | `migrate.source-untracked` | M (`git add -- foreign.md`) | names the retire-at-`--approve` data-loss reason | matches (M51 Inc 1) |
| a7 | `ingest` | clean | `jigc ingest` | 0 | none | I | `8 candidate(s) classified`, 4 adoptable/adopted | matches |
| a8 | `unmanage` | clean | `jigc unmanage CHANGELOG.md` | 0 | none | H | `unmanaged CHANGELOG.md (changelog:changelog) — dropped its file-state baseline + forward edges…` | matches contract, **but see A7-F4** | **[RECONCILER: `A7-F4` is a DANGLING POINTER — no such defect is stated or reproduced anywhere in this file. The a8 row itself is driven (repro §4 R-a); the defect it forward-references does not exist. Nothing is entered in the ledger for it.]**
| b1 | `describe` | schema-version **ahead** (99) | `jigc describe` | 0 | none | I | catalog unaffected | matches (the break is corpus-side) |
| b2 | `doc schema` | ahead | `jigc doc schema changelog` | 0 | none | — | reports the **manifest** version 2, never the doc's 99 | matches |
| b3 | `validate` | ahead | `jigc validate` | **1** | `schema-conformance.schema-version-ahead` | H | trailer: *"…so the sweep could not adjudicate it and exits non-zero; upgrade jigc, or restore the stamp from git history"* | matches (`STORE_EXIT_FLIPS` member) |
| b4 | `doc list` | ahead | `jigc doc list` | 0 | none | — | row state `managed` | matches (state is registration, not currency) |
| b5 | `migrate-corpus` | ahead | `jigc migrate-corpus` | **1** | `migrate-corpus.schema-version-ahead` | H | `blocked CHANGELOG.md` + upgrade-or-restore route | matches (never a silent `already-current`) |
| b6 | `migrate` | ahead | `jigc migrate foreign.md --as adr` | 0 | none | I | mints the foreign-adoption task | matches (a foreign file is not the ahead doc's subject) |
| b7 | `ingest` | ahead | `jigc ingest` | 0 | none (row) + `file-state.absorbed` (advisory) | I | calls the ahead-stamped `CHANGELOG.md` `adoptable … (adopted)`, `finding: null` — **and now names the absorb** | **absorb half CLOSED**; the currency-blindness is recorded as **O-4** below, not a defect |
| b8 | `unmanage` | ahead | `jigc unmanage CHANGELOG.md` | 0 | none | H | the standard drop ack | matches |
| c1 | `describe` | **below**-stamp (1 < 2) | `jigc describe` | 0 | none | I | unaffected | matches |
| c2 | `doc schema` | below | `jigc doc schema changelog` | 0 | none | — | manifest version 2 | matches |
| c3 | `validate` | below | `jigc validate` | **1** | `schema-conformance.schema-version-current` | H | route *"run `jigc migrate-corpus` to upgrade it"*; trailer names the exit-flip | matches (member 3) |
| c3′ | `validate` | **unstamped** managed (v0-era) | `jigc validate` | **1** | `schema-conformance.schema-version-current` | H | *"field `schema-version` is absent; the committed doc predates the schema-version stamp"* | matches |
| c4 | `doc list` | below | `jigc doc list` | 0 | none | — | `managed` | matches |
| c5 | `migrate-corpus` | below | `jigc migrate-corpus` | 0 | none | — | `1 migrated CHANGELOG.md … committed 353805f — only the migrated paths were staged` | matches |
| c5′ | `migrate-corpus` | unstamped managed | `jigc migrate-corpus` | 0 | none | — | `2 migrated` incl. `VISION.md`; stamp injected; committed | matches |
| c6 | `migrate` | below | `jigc migrate foreign.md --as adr` | 0 | none | I | task minted | matches |
| c7 | `ingest` | below (before any migrate-corpus) | `jigc ingest` | 0 | none (row) + `file-state.absorbed` | I | `adoptable CHANGELOG.md → changelog (adopted)` + the absorb line | **absorb half CLOSED**; see **O-4** |
| c8 | `unmanage` | below | `jigc unmanage CHANGELOG.md` | 0 | none | H | baseline dropped; `validate` still exits 1 on the stamp | matches |
| d1 | `describe` | **orphaned doctype** | `jigc describe` | 0 | none | I | the three methodology doctypes are gone from the catalog (`grep -c` → 0) | matches |
| d2 | `doc schema` | orphaned | `jigc doc schema roadmap` | **1** | `store.unknown-type` | M (`jigc describe`) | `unknown doctype \`roadmap\``; `--format json` puts it on the **findings envelope** with key `{code, target}` | matches (`ENVELOPE_OWED_CODES`) |
| d3 | `validate` | orphaned | `jigc validate` | **1** | `schema-conformance.orphaned-instance` ×2 | H | *"…no resolved doctype claims this path…"*, route hands to `jigc ingest`, then re-add-the-pack, then `unmanage` + delete the stamp | **D-4 half 1 CLOSED** |
| d4 | `doc list` | orphaned | `jigc doc list` / `--format json` | 0 | none | — | `(none) docs/roadmap.md orphaned`; JSON `"id": null … "item-count": null` | matches (the declared third state) |
| d5 | `migrate-corpus` | orphaned | `jigc migrate-corpus` | 0 | none | — | `0 migrated, 1 already current, 0 blocked` — silent about both orphans | matches (no schema ⇒ not its subject; `validate` owns the exit) |
| d6 | `migrate` | orphaned | `jigc migrate foreign.md --as adr` | 0 | none | I | task minted | matches |
| d7 | `ingest` | orphaned | `jigc ingest` / `--format json` | 0 | none | I | `unmanaged docs/ — 2 file(s) … fine to stay plain` + *"no action needed"*; rows `finding: null, annotations: []` | **DEFECT A7-F2** (D-4 half 2, STILL-OPEN) |
| d8 | `unmanage` | orphaned | `jigc unmanage docs/roadmap.md` | 0 | none | — | `dropped its file-state baseline; the file is left on disk`; `validate` still exits 1 — **as the finding's own route predicted** | matches |
| e1 | `describe` | **unadopted foreign** (squatter) | `jigc describe` | 0 | none | I | unaffected | matches |
| e2 | `doc schema` | squatter | `jigc doc schema changelog` | 0 | none | — | version 2 | matches |
| e3 | `validate` | squatter | `jigc validate` | **1** | `schema-conformance.unadopted-instance` (**advisory**) | H | *"never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version"*; trailer states the exit-flip | matches (advisory finding, flipped exit) |
| e4 | `doc list` | squatter | `jigc doc list` | 0 | none | — | state `unregistered` | matches |
| e5 | `migrate-corpus` | squatter | `jigc migrate-corpus` | 0 | none | H | `0 migrated, 0 already current, 0 blocked, **1 not adopted**` + the row's own code and route | matches (M46 Inc 3) |
| e6 | `migrate` | squatter | `jigc migrate foreign.md --as adr` | 0 | none | I | task minted | matches |
| e7 | `ingest` | squatter | `jigc ingest` | 0 | `conformance.section-renamed` (blocking, row-carried) | M | `needs-reconcile` + route `jigc migrate CHANGELOG.md --as changelog` | matches (the declared triage-door exit divergence) |
| e8 | `unmanage` | squatter | `jigc unmanage CHANGELOG.md` | 0 | none | — | `no-op: … is not managed (nothing to drop)` | matches |
| f1–f8 | **all eight** | **project-layer shadow** (shape change: the `releases` section dropped) | the eight argvs | **1** each | none (anyhow) | H | `pack-load freeze check failed: doctype \`changelog\`: schema-hash mismatch (manifest declares …, recomputed …) — the project schema shadow … changes the shape of a frozen doctype, which the freeze forbids at every layer` + a `route:` naming `rm <shadow>` and the bump-and-migrate | matches (M49: the freeze binds at every layer) |
| f-json | `validate`·`doc list`·`migrate-corpus`·`ingest`·`describe`·`doc schema` | project shadow, `--format json` | `… --format json` | **1** | none | — | **one** JSON document, `{"error": …}`, on **stderr**, stdout 0 bytes, strict-parses | matches (stream discipline + one-document rule) |
| g1–g5 | `describe`·`doc schema`·`validate`·`doc list`·`migrate-corpus` | **pack-layer hash moved without a re-pin** | the five argvs | **1** each | none (anyhow) | **—** | the hash-mismatch line, **no route line at all**; `--format json` → `{"error": …}` on stderr | matches contract, with **O-1** |

**Cell set (a): 63 rows driven** — 53 of the 56 (door × condition) coordinates (the 3 un-driven ones
are `ingest`/`migrate`/`unmanage` at the (g) cell, §6.3), plus a6′, c3′, c5′ and the 7 `--format json`
arms at the two freeze-block cells.

**Total for this axis: 118 driven rows** — 63 here + 14 in §3.2 + 41 across §3.4's registry cells
(9 posture × `migrate-corpus`, 6 posture controls, 6 pre-dispatch, 4 `home-vacated`, 4 D-1 absorb,
6 C-1 identity, 2 `doc schema` projection, 2 `unversioned-doctype`, 2 envelope-owed).

### 3.2 · Cell set (b) — `SchemaChangeKind::ALL × LOCI` at `migrate-corpus`, plus the new enumeration guard

Every cell manufactured the same way: `dev/jigc-rig fresh --pack-from-dev`, the **real**
`schema-manifest.yaml` restored into the pack copy so the freeze is live, a conformant instance
committed at the prior stamp, then the schema edited, the prior shape shipped as
`schema-snapshots/<ty>.v<k>.yaml`, the manifest `schema-version` bumped and the `schema-hash`
re-pinned from the binary's **own** freeze message until pack-load is clean.

| # | kind | locus | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| m1 | `AddedItemField` (`releases.tag`) | 2 (item) | `jigc migrate-corpus` | 0 | none | — | `1 migrated CHANGELOG.md` + commit | matches |
| m2 | `AddedOptionalSection` | 1 | `jigc migrate-corpus` | 0 | none | — | `1 migrated` + commit; `## Notes For Maintainers` minted at the schema-ordered offset; stamp 2→3 | matches |
| m3 | `EnumWidened` (`category` + `chore`) | 1/2 | `jigc migrate-corpus` | 0 | none | — | `1 migrated` + commit; committed values untouched | matches |
| m4 | `RemovedField` (`releases.link`) | 2 | `jigc migrate-corpus` | **1** | `migrate-corpus.removed-field` | H | *"the migration never strips a value (no data loss) … This is a schema-authoring gap, not a doc problem"* + restore-or-build route | matches |
| m5 | **`ValueRemapped` on an `id-from` enum** | 2 | `jigc migrate-corpus` | **1** | **`migrate-corpus.fold-refused`** | H | names the `id-from` role and states *"no prose … and no re-run changes it"*; re-run byte-identical; `git status` empty | **D-3 CLOSED** |
| m6 | **`Relocated`, placement→placement** (`CHANGELOG.md` → `HISTORY.md`) | doctype level | `jigc migrate-corpus` | **0** | none | — | `1 migrated / migrated HISTORY.md / committed 22d78da`; `doc list` → `HISTORY.md managed`; `CHANGELOG.md` gone | **D-2 CLOSED** |
| m7 | **`Relocated`, location→location** (`decisions/` → `adrs/`) | doctype level | `jigc migrate-corpus` | **0** | none | — | `1 migrated docs/adrs/use-sqlite.md` + commit; `doc list` → the new home `managed` | **D-2 CLOSED** |
| m7a | *(pre-migration state of m7 — F2's subject)* `validate` | — | `jigc validate` | **1** | `schema-conformance.schema-version-current` | H | names the doc **at the stale home** `docs/decisions/use-sqlite.md` and routes at `jigc migrate-corpus`; trailer names the exit-flip | matches (M52 F2) |
| m7b | *(same commit)* `doc list` | — | `jigc doc list` / `--format json` | 0 | none | — | the row is **`managed` at the stale home**, `item-count: 0`, no new declared key | matches (M52 F2's stated shape) |
| m7c | *(same commit)* `migrate-corpus --dry-run` | — | `jigc migrate-corpus --dry-run` | 0 | none | I | `1 would migrate / would migrate docs/adrs/use-sqlite.md` + the no-write exception sentence | matches |
| m7d | *(same commit)* `ingest` | — | `jigc ingest` | 0 | `ingest.wrong-location` (blocking, row-carried) | M | route *"move … into docs/adrs/, then re-run `jigc ingest`"* — **not** `migrate-corpus` | matches the **declared bound** in M52's VERDICT F2 (carried with a trigger) |
| m7e | *(same commit, evidence door)* `doc show` | — | `jigc doc show adr:use-sqlite` | **1** | `store.not-found` | H | reads at the **resolved** home and refuses; route names only *create / fix the reference / read it with `--task`* | **DEFECT A7-F3** |
| m8 | `Relocated` **route leg** (evidence door) | — | `jigc relocate adr --from docs/decisions` | **1** | **`relocate.frozen-doctype`** | H → a verb that acts | *"relocate it through the version-gated `jigc migrate-corpus`"*; route: *"`jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare and lands the move under the freeze"* — and it does (m7) | **CLOSED** (M52 F5 minted the code) |
| m9 | **missing prior snapshot** (new M52 guard) | enumeration | `jigc migrate-corpus` | **1** | **`migrate-corpus.missing-snapshot`** | M | `blocked **changelog@v2**` — *"no prior-schema snapshot `schema-snapshots/changelog.v2.yaml` is shipped — the corpus walk cannot enumerate the homes schema-version 2 declared, so no `changelog` document is this run's subject at all"*; route: ship the snapshot | matches flow-53 arm 4 |

**Cell set (b): 14 rows driven** over 7 distinct `SchemaChangeKind` cells + the enumeration guard +
4 same-commit cross-door rows. **40+ of the 54 `kind × locus` cells were not driven** — see §6.

### 3.3 · Cell set (c) — `ManifestKind::ALL`: **0 of 6 driven, and why**

`ManifestKind` is the **finalize manifest's** row tag (`render.rs:1985`), rendered at the boundary
doors. Driven: **no axis-7 door renders a manifest** — `migrate-corpus`, the only `COMMITTING_DOORS`
member in this door set, acks with `committed <sha> — only the migrated paths were staged` and **no
manifest line** (observed at rows c5, c5′, m1, m2, m3, m6, m7). Reaching `Promoted`/`Modified`/
`Added`/`Deleted`/`Untracked`/`CarriedOver` needs `task finalize` / `milestone finalize`, which are
axes 2/3/4's doors. **48 (door × kind) coordinates are therefore n/a with a driven reason**, handed
back rather than double-counted.

### 3.4 · The M52-minted registries this axis touches

| registry | what I drove at axis-7 doors | result |
|---|---|---|
| `InProgress::ALL` (9) | **all nine** git states × `migrate-corpus` (the axis's committing door), via `dev/jigc-rig committed-singletons --git-state <m>` | 9/9 exit 1, `repo.operation-in-progress` naming the operation, `H` route naming the concluding git command (`merge --continue` / `--abort` etc.). §4 R-P |
| `InProgress` × the non-committing axis-7 doors | `validate`·`doc list`·`ingest`·`describe`·`doc schema`·`unmanage` under `merge` | 6/6 exit 0 and silent — the `Neither` class control holds |
| `ENVELOPE_OWED_CODES` (4) | `store.unknown-type` at `doc schema nosuch --format json`; `task::FIXED_IDENTITY` at `doc show changelog:notcanonical`; `store.not-found` at `doc show adr:use-sqlite` | 3 of 4 driven: unknown-type on the `{schema_version, findings}` envelope with key `{code, target}`; fixed-identity with a route that runs; not-found — see A7-F3. `store.no-such-leaf` not reachable from an axis-7 door (not driven) | **[RECONCILER: DEMOTED — no repro block in §4 for either cell; the table line carries an argv and a paraphrase, no observed output. RE-DRIVEN by the reconciler and both hold — verbatim envelopes in the ledger, §R-R3. The rows stand on the reconciler's evidence, not the driver's.]**
| `PRE_DISPATCH_FAULTS` (3) | `packs-yaml-malformed` × `describe`·`validate`·`doc list`·`ingest`·`migrate-corpus` + the `--format json` arm | 5/5 exit 1, one repo-relative located message; JSON = **one** `{"error": …}` document on stderr, stdout 0 bytes | **[RECONCILER: DEMOTED — no repro block in §4. RE-DRIVEN and it holds, 5/5 doors + the JSON arm — §R-R4.]**
| `STORE_EXIT_FLIPS` (7) | `unmigrated-corpus` (c3) · `ahead-corpus` (b3) · `orphaned-instance` (d3) · `foreign-squatter` (e3) · **`home-vacated`** (§4 R-HV) · `oob-rename` (§4 R-HV) — 6 of 7 | each flips `validate` to exit 1; `probe-unreliable` not driven (needs a crashing pack probe — axis 5/6's fixture) |
| `home-vacated` × `orphan::Removal` (3 states) | committed removal · worktree-only removal · staged removal, + the **never-adopted brownfield control** | 3/3 carry **distinct** routes (`git show <sha> -- <path>` + re-ingest · `git checkout -- <path>` · `git restore --source=HEAD --staged --worktree -- <path>`); the brownfield control is **silent at exit 0** (F1's fix). One cross-cell defect: **A7-F1** |
| `RelocateRefusal::ALL` (10) | `FrozenDoctype` only (m8) | 1 of 10 — the other nine are axis-1/axis-3 cells; not driven, stated |
| the fixed-identity doctype set (`placement ‖ singleton`) | `doc schema changelog --format json` (placement) and `doc schema adr --format json` (slugged control) | `contract-version: 7`, `identity: {kind: fixed, address: changelog}` / `{kind: slugged, address: "adr:<slug>"}`, `home: {kind: placement, path: CHANGELOG.md}` / `{kind: location, path: "docs/decisions/<slug>.md"}` — matches D5 |
| `unversioned-doctype` (M51 Inc 8's sibling cause) | a manifest-less pack (`--pack-from-dev`, manifest **not** restored) × `validate` + `migrate-corpus` | advisory, **exit 0** (flips no exit — "an absent manifest entry means unchecked by design"); `migrate-corpus` → `0 migrated, 0 already current, 0 blocked` | **[RECONCILER: DEMOTED — no repro block in §4. RE-DRIVEN and it holds verbatim — §R-R5.]**
| `ROLLBACK_POPULATIONS` (11) · `TASK_AREA_FILES` (13) · `DESTROYING_DOORS` (6) × `Disposition` · the `suppressed.door` set | **not driven here** | no axis-7 door is a member of any of them: `DESTROYING_DOORS` is `milestone provision/discard/finalize/uninstall/task discard/task finalize`; `ROLLBACK_POPULATIONS`/`TASK_AREA_FILES` live behind the task and milestone transactions. Axes 3 and 4 own them. Stated, never presented as driven |

---

## 4 · Repro blocks

All rigs: `rig=$(/Users/maurice/projects/gherrink-jigc/dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval; absolute rig path because the eval `cd`s into `$REPO`). No teardown, no `rm -rf` of a
variable path anywhere. **Note for the next driver:** zsh's builtin `echo` expands `\n`, so
`echo "$(jigc … --format json)"` *manufactures* a raw newline inside the JSON string and makes a valid
document look invalid. I hit that and re-measured by redirecting to a file; every JSON verdict above
is from a file, strict-parsed.

### R-a · the clean control (a1–a8)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf '# Foreign\n\nsome prose\n' > foreign.md; git add foreign.md
$JIGC describe                     -> 0   catalog body
$JIGC doc schema changelog         -> 0   "doctype: changelog (schema-version 2)"
$JIGC validate                     -> 0   1 advisory (repeatable-populated)
$JIGC doc list                     -> 0   4 rows, all `managed`
$JIGC migrate-corpus               -> 0   "0 migrated, 4 already current, 0 blocked"
$JIGC ingest                       -> 0   "8 candidate(s) classified", 4 adoptable/adopted
$JIGC migrate foreign.md --as adr  -> 0   "task minted: migrate-adr-foreign-0a629e95a3c6"
$JIGC unmanage CHANGELOG.md        -> 0   "unmanaged CHANGELOG.md (changelog:changelog) — dropped its
                                           file-state baseline + forward edges; …"
# a6' — the same argv with foreign.md UNSTAGED:
$JIGC migrate foreign.md --as adr  -> 1   blocking · migrate.source-untracked
                                           route: git add -- foreign.md
```

### R-b / R-c / R-d / R-e — the four corpus-state fixtures

```
# AHEAD (b): committed-singletons, then
sed -i '' 's/^schema-version: 2$/schema-version: 99/' CHANGELOG.md; git add -A; git commit -m ahead
# BELOW (c): the same with `schema-version: 1`
# UNSTAMPED (c3'/c5'): strip the whole `---\nschema-version: 1\n---\n\n` block from VISION.md, commit
# ORPHANED DOCTYPE (d):
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml; git add -A; git commit
# SQUATTER (e): rig `fresh`, then a Keep-a-Changelog CHANGELOG.md committed at the managed home
```
Observed exits/codes per door are the table rows b1–b8 / c1–c8 / d1–d8 / e1–e8 above; each was run
in its own rig and read from a file.

### R-f / R-g — the two freeze arms

```
# f — project-layer shadow (rig: committed-singletons)
mkdir -p .jigc/config/schemas
# copy crates/cli/pack/schemas/changelog.yaml with the whole `releases` section deleted
$JIGC <each of the eight>            -> 1, the identical pack-load block + a `route:` naming `rm <shadow>`
$JIGC validate --format json         -> 1, stdout 0 bytes, stderr ONE {"error": …} doc, strict-parses

# g — pack-layer hash moved and NOT re-pinned (rig: fresh --pack-from-dev, real manifest restored,
#     placement.file edited, manifest version bumped, hash left at the old value)
$JIGC describe / doc schema / validate / doc list / migrate-corpus  -> 1 each
   "pack-load freeze check failed: doctype `changelog`: schema-hash mismatch
    (manifest declares `9e1130df…`, recomputed `d9ee5fad…`)"        <- and NO route line (O-1)
$JIGC migrate-corpus --format json   -> 1, stderr ONE {"error": …} doc
```

### R-m — the manufactured-pack migration cells (m1–m9)

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
cp <repo>/crates/cli/pack/config/schema-manifest.yaml "$JIGC_PACK_DIR/config/"   # freeze live again
cp <golden CHANGELOG.md, stamped 2> CHANGELOG.md; git add -A; git commit -m base
# CONTROL (every cell):  $JIGC migrate-corpus -> "0 migrated, 1 already current, 0 blocked"
cp "$JIGC_PACK_DIR/schemas/changelog.yaml" "$JIGC_PACK_DIR/schema-snapshots/changelog.v2.yaml"
<manifest: changelog schema-version 2 -> 3>
<the one schema edit for this cell>
<re-pin: read `recomputed <hash>` out of `jigc describe`'s own freeze message, write it back, repeat>
$JIGC doc schema changelog -> 0 "doctype: changelog (schema-version 3)"     # pack loads clean
$JIGC migrate-corpus       -> the row's exit/code
```

**m6 — the `Relocated` placement arm (D-2), with its control in the same run:**
```
# the ONLY edit: placement: { file: CHANGELOG.md } -> { file: HISTORY.md }
$JIGC migrate-corpus -> exit 0
  corpus migration: 1 migrated, 0 already current, 0 blocked
    migrated   HISTORY.md
  committed 22d78da — only the migrated paths were staged
$JIGC validate  -> exit 0  (1 advisory: file-state.un-baselined on HISTORY.md — see O-3)
$JIGC doc list  -> exit 0  "changelog:changelog  HISTORY.md  managed"
head -3 CHANGELOG.md -> No such file or directory ;  ls HISTORY.md -> HISTORY.md
```

**m7 — the `location:` arm and its four same-commit cross-door rows:**
```
# the ONLY edit: location: decisions/ -> adrs/   (adr v2 -> v3, snapshot adr.v2.yaml shipped)
# BEFORE migrate-corpus (F2's subject):
$JIGC validate  -> 1  schema-conformance.schema-version-current @ docs/decisions/use-sqlite.md
                      route: "run `jigc migrate-corpus` to upgrade it"
$JIGC doc list  -> 0  "adr:use-sqlite  docs/decisions/use-sqlite.md  managed"  (json: item-count 0)
$JIGC migrate-corpus --dry-run -> 0  "1 would migrate / would migrate docs/adrs/use-sqlite.md"
$JIGC ingest    -> 0  ingest.wrong-location, route "move … into docs/adrs/, then re-run `jigc ingest`"
$JIGC doc show adr:use-sqlite -> 1  store.not-found @ docs/adrs/use-sqlite.md      <- A7-F3
$JIGC relocate adr --from docs/decisions -> 1  relocate.frozen-doctype
      route: "`jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare…"
# AFTER, running that route verbatim:
$JIGC migrate-corpus -> 0  "1 migrated docs/adrs/use-sqlite.md / committed fc63fea"
$JIGC doc list       -> 0  "adr:use-sqlite  docs/adrs/use-sqlite.md  managed"
```

**m5 — `ValueRemapped` on an `id-from` enum (D-3):**
```
# the ONLY edit: of: [added, changed, …] -> of: [added, altered, …]  (`category` is the id-from of
# both repeatable blocks)
$JIGC migrate-corpus -> exit 1
  blocked    CHANGELOG.md
    migrate-corpus.fold-refused: … `category` is that block's `id-from` — each item's heading *is*
      its committed value and its `{#id}` anchor is slugged from it, so remapping it would re-mint
      the identity of every item this doc carries, which no migration performs. This is a
      schema-authoring gap, not a doc problem — no prose authored into `CHANGELOG.md` and no re-run
      changes it
    route: re-author the bump so the `id-from` field `category` … keeps its declared members …
$JIGC migrate-corpus (again) -> 1, BYTE-IDENTICAL ; git status --short -> (empty)
```

**m9 — `migrate-corpus.missing-snapshot`:**
```
# manifest bumped 2 -> 3, a real shape change made, and NO changelog.v2.yaml shipped
$JIGC migrate-corpus -> exit 1
  blocked    changelog@v2
    migrate-corpus.missing-snapshot: `changelog` is at schema-version 3, but no prior-schema
      snapshot `schema-snapshots/changelog.v2.yaml` is shipped — the corpus walk cannot enumerate
      the homes schema-version 2 declared, so no `changelog` document is this run's subject at all
    route: ship the prior-schema snapshot `schema-snapshots/changelog.v2.yaml`, then re-run …
```

### R-D1 · D-1 re-driven — the absorb surface (CLOSED)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf '\nAn out-of-band sentence.\n' >> VISION.md ; git add -A ; git commit -m oob
$JIGC validate  -> the drift is really there: blocking (gates at finalize) · file-state.hash-matches
$JIGC ingest    -> exit 0, and the LAST block of the report is now:
  advisory · file-state.absorbed — out-of-band edit to `VISION.md` absorbed — its file-state
    baseline now records the on-disk bytes
    at: VISION.md
    route: no action needed — this retires the `file-state.hash-matches` finding for `VISION.md`;
           review the edit in git history if it was not yours
$JIGC ingest --format json  -> findings[0] = {code: "file-state.absorbed",
                                              key: {code: …, target: "VISION.md"}, route: …}
$JIGC ingest --format json (control, second run, no new drift) -> findings: []
$JIGC validate  -> exit 0, the drift row is gone
```

### R-C1 · C-1 re-driven — the placement identity (CLOSED)

```
rig=$(dev/jigc-rig refs-post-hoc --binary ~/.local/bin/jigc) || exit; eval "$rig"
# plant a committed edge on the placement doc without finalizing:
<insert `grounded-in: [research:context-loss]` into VISION.md's front matter>; git add -A; git commit
$JIGC unmanage VISION.md -> 0 ; edges.json -> "edges": []
$JIGC ingest             -> 0 ; edges.json -> [{ "from": "vision:vision",      <- canonical, not VISION
                                                 "relation": "grounded-in",
                                                 "to": "research:context-loss" }]
$JIGC unmanage VISION.md -> 0 ; edges.json -> "edges": []                       <- the edge is GONE
# and the repeat-run control, on `changelog` (rig committed-singletons):
run 1 -> "unmanaged CHANGELOG.md (changelog:changelog) — dropped …"
run 2 -> "no-op: CHANGELOG.md is not managed (nothing to drop)"
run 3 -> "no-op: CHANGELOG.md is not managed (nothing to drop)"
```

### R-C2 · C-2 re-driven — the root-placement orphan (STILL-OPEN, declared residual)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml; git add -A; git commit
$JIGC validate  -> 1, 2× orphaned-instance (docs/decisions-log.md, docs/roadmap.md)
# for VISION.md — a stamped, formerly-managed doc at a ROOT placement home:
mentions of VISION.md:  validate 0 · doc list 0 · ingest 0 · migrate-corpus 0
head -3 VISION.md -> ---/schema-version: 1/---   (still there, still stamped)
```

### R-HV · `home-vacated` × the three `orphan::Removal` states + the brownfield control

```
# CELL 1 — committed removal (rig committed-singletons)
git rm -q CHANGELOG.md; git commit -m 'remove the changelog'
$JIGC validate -> 1
  blocking · schema-conformance.home-vacated — `changelog` homes its one document at `CHANGELOG.md`,
    the last document the repository committed there carried a `schema-version:` stamp, and nothing
    is there now …
    route: restore the document at `CHANGELOG.md` and commit it — `27a67eb` is the commit that
           removed it (`git show 27a67eb -- CHANGELOG.md`) — then `jigc ingest` to re-register it; …

# CELL 2 — worktree-only removal
rm CHANGELOG.md                     # nothing staged, nothing committed
$JIGC validate -> 1 ; route: "the removal is not committed — restore it: `git checkout -- CHANGELOG.md`"

# CELL 3 — staged removal
git rm -q --cached CHANGELOG.md; rm -f CHANGELOG.md
$JIGC validate -> 1 ; route: "the deletion is staged and not committed — unstage it and restore it:
                              `git restore --source=HEAD --staged --worktree -- CHANGELOG.md`"

# CELL 4 — F1's fix: a NEVER-ADOPTED brownfield repo whose history held and deleted a CHANGELOG.md
rig=$(dev/jigc-rig bare --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf '# Changelog\n\nsome history\n' > CHANGELOG.md; git add -A; git commit -m 'a pre-jigc changelog'
git rm -q CHANGELOG.md; git commit -m 'and delete it'
$JIGC setup    -> 0
$JIGC validate -> 0   "no findings — the committed store validates clean"   (home-vacated mentions: 0)
```

### R-P · `InProgress::ALL` × `migrate-corpus`

```
for gs in merge rebase-merge bisect cherry-pick revert unmerged-index am sequencer dangling-sequencer
  rig=$(dev/jigc-rig committed-singletons --git-state $gs --binary ~/.local/bin/jigc) || exit
  eval "$rig"; $JIGC migrate-corpus
merge              -> 1  blocking · repo.operation-in-progress — a merge is in progress …
                          route: conclude it with `git merge --continue` … or `git merge --abort`
rebase-merge       -> 1  "a rebase is in progress"
bisect             -> 1  "a bisect is in progress"
cherry-pick        -> 1  "a cherry-pick is in progress"
revert             -> 1  "a revert is in progress"
unmerged-index     -> 1  "a conflict left unmerged paths in the index"
am                 -> 1  "a `git am` is in progress"
sequencer          -> 1  "a cherry-pick is in progress"
dangling-sequencer -> 1  "a cherry-pick or revert left a queue of commits in `sequencer/`"
# the Neither-class control, under `merge`:
validate 0 · doc list 0 · ingest 0 · describe 0 · doc schema changelog 0 · unmanage 0
```

### R-F1 · **DEFECT A7-F1** — the exit-flip trailer asserts a commit and a `git mv` that never happened

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
H0=$(git rev-parse HEAD)
rm CHANGELOG.md                                   # uncommitted, unstaged worktree deletion
$JIGC validate -> exit 1
  … blocking (gates at finalize) · reconciliation.rename — tracked managed doc changelog:changelog
      (CHANGELOG.md) is missing
      route: restore CHANGELOG.md, or confirm the deletion by dropping it from the index:
             `jigc unmanage CHANGELOG.md`                      <- this route is FINE
  … blocking · schema-conformance.home-vacated … route: `git checkout -- CHANGELOG.md`   <- also fine
  TRAILER:
  out-of-band rename detected — a structural-identity change this commit introduced; the sweep
  exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
git rev-parse HEAD                  -> unchanged (H0)
git status --short                  -> " D CHANGELOG.md"
git log -1 --name-status --diff-filter=R  -> (no rename rows in HEAD at all)
```
Same trailer, verbatim, in the **committed-removal** and **staged-removal** cells, where the act was
a `git rm` and not a `git mv`.

### R-F2 · **DEFECT A7-F2** — `ingest` says *no action needed* about the files `validate` blocks on

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml; git add -A; git commit
$JIGC validate --format json -> exit 1
  [("schema-conformance.orphaned-instance","docs/decisions-log.md"),
   ("schema-conformance.orphaned-instance","docs/roadmap.md")]
  and each route's FIRST exit is: "ask `jigc ingest`, which re-reads the file …"
$JIGC ingest -> exit 0
  unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
  …
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state —
              no action needed. …
$JIGC ingest --format json -> both rows: {'verdict': 'unmanaged', 'finding': None,
                                          'adopted': False, 'annotations': []}, findings: []
```

---

## 5 · Observations (driven, not defects)

* **O-1 — the two arms of one freeze check print two different surfaces** (carried from M51,
  unchanged). The project-layer arm appends a `route:` naming `rm <shadow>`; the pack-layer arm
  prints the hash mismatch with **no route at all** and `--format json` returns a bare `{"error": …}`.
  Both are `anyhow` channels, and `engine::finding::is_route_exempt`'s doc-comment disposes that class
  by name, so this stays an asymmetry rather than a route-floor violation. It remains the one place in
  this axis where a door blocks every verb and names no recovery.
* **O-2 — only the FIRST matching `STORE_EXIT_FLIPS` trailer prints, and that is declared.**
  `first_store_exit_flip` (`render.rs:1149`) documents *"table order **is** precedence"*. In the
  home-vacated cells two members match (`oob-rename` and `home-vacated`) and only `oob-rename`'s
  trailer renders. Nothing is lost — each finding carries its own route — but it is why **A7-F1**'s
  wrong sentence is the *only* trailer the reader gets in all three removal cells.
* **O-3 — `migrate-corpus` leaves the doc it just committed un-baselined.** After every successful
  relocation and content migration, `jigc validate` reports
  `advisory · file-state.un-baselined — committed doc `<new path>` is not yet baselined`, route *"no
  action needed — the doc is baselined on its next author or finalize"*. Honest and exit-0, but the
  door that wrote and committed the bytes is the one that could have baselined them.
* **O-4 — `ingest` is currency-blind, and stays so by design.** At the *ahead* (b7) and *below* (c7)
  cells it calls the doc `adoptable … (adopted)` with `finding: null` at the same commit `validate`
  exits 1 on `schema-version-ahead` / `schema-version-current`. `ingest`'s classifier is
  conformance-at-location, and M52's VERDICT F2 explicitly declares currency to be a different door's
  subject, so this is recorded as the declared partition rather than a defect. **It is the reason
  A7-F2 is a defect and this is not**: there, `ingest` makes a positive claim (*no action needed*)
  about a blocked path; here it makes none.
* **O-5 — `validate`'s route on the `fold-refused` cell still says *run the corpus migration*.**
  `schema-conformance.field-value-conformant` routes at `jigc migrate-corpus`, which refuses. The loop
  **terminates with information** (the refusal states *"no prose … and no re-run changes it"* and names
  the schema-authoring repair), so it is not M42's mutual dead end — recorded, not reported.

---

## 6 · What I did **not** drive, and why

Presented as un-driven, never as driven.

1. **40 of the 54 `SchemaChangeKind × LOCI` cells.** I drove 7 distinct kinds
   (`AddedItemField`, `AddedOptionalSection`, `EnumWidened`, `RemovedField`, `ValueRemapped`@id-from,
   `Relocated`×2 home kinds) plus the enumeration guard, chosen to hit every *disposition*
   (`Applied`, `Refused`, `DoctypeLevel`) and both `Relocated` home kinds — M52's arm-4 subject.
   **Specifically not driven:** `AddedRepeatableSection`, `FixedSlotToRepeatable`, `AddedItemSlot`,
   `NarrowedCardinality`/`WidenedCardinality`, `OptionalRelaxed`, `ProseNeeding`, `PresentationOnly`,
   `DisplayTitleChanged`, `Unclassified`, `ValueRemapped` on a **plain** field, and the locus-1/3
   siblings of the kinds driven at one locus. Each needs its own manufactured pack + snapshot + re-pin
   (~3 min), and the table itself is compiler-fenced by `locus_disposition`'s exhaustive match plus
   `crates/cli/tests/migrate_locus_axis.rs`.
2. **All 6 `ManifestKind::ALL` cells × all 8 doors (48 coordinates)** — n/a with the driven reason in
   §3.3: no axis-7 door renders a manifest.
3. **`ingest`, `migrate` and `unmanage` at the (g) pack-layer hash-move cell.** Pack-load precedes
   dispatch, so there is exactly one observable per door; I drove five doors at (g) and all eight at
   (f), and the (f) block is byte-identical across all eight. Stated rather than extrapolated into
   driven rows.
4. **9 of the 10 `RelocateRefusal::ALL` members** — `relocate` is an evidence door here, not an axis-7
   cell; the other nine refusals are axis-1 (token) and axis-3 (destruction) cells.
5. **`ROLLBACK_POPULATIONS` (11), `TASK_AREA_FILES` (13), `DESTROYING_DOORS` (6) × `Disposition`, and
   the `suppressed.door` set.** No axis-7 door is a member of any of them; axes 3 and 4 own them.
6. **`probe-unreliable`, the 7th `STORE_EXIT_FLIPS` member** — it needs a crashing pack probe, which is
   axis 5/6's fixture.
7. **`store.no-such-leaf`, the 2nd `ENVELOPE_OWED_CODES` member** — unreachable from an axis-7 door
   (it is a leaf-address code at the write/read verbs).
8. **`cwd-unreadable` and `pack-resource-missing`**, the other two `PRE_DISPATCH_FAULTS` rows — the
   first needs a deleted cwd and the second an empty pack dir reached through `start`; both are axis-6
   fixtures. `packs-yaml-malformed` was driven because it is the composition fault this axis is about.
9. **The methodology pack's own `schema-manifest.yaml`.** Every hash-move and bump row moved the **dev**
   pack's manifest; `--pack-from-dev` replaces the composed set with the dev pack alone, so per-origin
   manifest resolution (M40 A1) was not independently drifted.
10. **The Codex source pass for this axis** — deliberately not read, per the brief.

---

## 7 · What this adds over flow-53 arm 4 (and arms 5, 6)

Three flow-53 arms touch this axis. **Arm 4** iterates the `{location, placement}²` home-pair set ×
`{Relocated-only, Relocated+content}` plus the fixed-identity home set for `home-vacated`. **Arm 5**
iterates the fixed-identity doctype set × the address doors. **Arm 6** iterates `PRE_DISPATCH_FAULTS`
× `VERB_KINDS` and `ENVELOPE_ARMS`.

What this matrix adds:

1. **Breadth across doors, where arm 4 is deep in one door.** Arm 4 proves the walk at
   `migrate-corpus` (and the store sweep at `validate`); this table drives each of the seven
   freeze/adoption conditions at **all eight** doors — 55 rows — and that breadth is what found
   **A7-F2**: the orphan cell is correct at `validate`, `doc list`, `doc schema`, `migrate-corpus` and
   `unmanage`, and contradicts itself at `ingest`, which no single-door arm can see.
2. **The two freeze-*block* cells have no flow-53 arm at all.** The project-layer shadow and the
   pack-layer hash move are reached by no arm in the set (arm 4's fixtures all re-pin; the arm's
   subject is the walk, not `assert_schema_freeze`). Driving them at all eight doors is where **O-1**
   lives, and is the only evidence that the freeze genuinely binds every door rather than the
   migration path.
3. **`home-vacated` driven across its co-firing neighbour.** Arm 4 asserts `home-vacated` fires on
   every vacated home and stays silent on the controls — all true here. What an arm scoped to one
   finding cannot see is the **trailer** the sweep prints beside it, which is chosen from a *different*
   `STORE_EXIT_FLIPS` member by table precedence. That is **A7-F1**: M52 F6 swept `home-vacated`'s
   route over the three `Removal` states and left the sentence the reader actually gets asserting a
   committed `git mv` in all three.
4. **The route legs run, not just asserted.** `relocate.frozen-doctype`'s route was lifted out of the
   refusal and executed verbatim (m8 → m7), and the orphan finding's first route exit (`jigc ingest`)
   was followed to the door it names — which is how its contradiction surfaced.
5. **Cross-door consistency at one commit.** Rows m7a–m7e ask five different doors about *one*
   stranded document at *one* commit: `validate` blocks and routes correctly, `doc list` calls it
   `managed` at the stale home, `--dry-run` sees it at the new home, `ingest` routes by hand-move, and
   `doc show` refuses `store.not-found` with a route that names none of the above (**A7-F3**). Arm 4's
   per-cell assertion is about the walk landing the doc; the divergence is only visible when every
   door is asked at the same commit.
6. **`ManifestKind::ALL` is explicitly handed back** with the driven reason, rather than double-counted
   here.

---

## 8 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling (`cli.rs:1834`):

| door | rows |
|---|---|
| `describe` | a1 b1 c1 d1 e1 f1 g1 · the pre-dispatch fault cell · the repin probe |
| `doc schema` | a2 b2 c2 d2 e2 f2 g2 · the contract-version-7 identity/home projection · the `store.unknown-type` envelope |
| `validate` | a3 b3 c3 c3′ d3 e3 f3 g3 · m7a · R-HV cells 1–4 · R-D1 · R-F1 · R-F2 · the unversioned-doctype cell |
| `doc list` | a4 b4 c4 d4 e4 f4 g4 · m6 m7 m7b |
| `migrate-corpus` | a5 b5 c5 c5′ d5 e5 f5 g5 · m1–m9 · m7c · all nine `InProgress` rows · the unversioned-doctype cell |
| `migrate` | a6 a6′ b6 c6 d6 e6 f6 |
| `ingest` | a7 b7 c7 d7 e7 f7 · R-D1 · R-C1 · R-F2 · m7d |
| `unmanage` | a8 b8 c8 d8 e8 f8 · R-C1 · the `--format json` arm · the posture control |
| `doc show` | **evidence door** — m7e (`store.not-found` on the stranded doc) and the `store.fixed-identity` refusal |
| `relocate` | **evidence door** — m8 (`relocate.frozen-doctype` and its route, run verbatim) |
| `setup` | **fixture door** — driven once at exit 0 in R-HV cell 4, but it is the fixture's construction and not an axis-7 cell; **not claimed as covered** |

`doc show` and `relocate` are evidence doors, not cells of this axis's own (door × cell) table; they
belong to other axes' door sets and are listed so the coverage diff can see them.

---

# Reconciliation ledger

**Reconciler:** a third agent, driving the same binary (`/Users/maurice/.local/bin/jigc` →
`jigc 1.0.0-rc.16`, asserted) at repo HEAD `a3eb026b` (clean tree), 2026-09-21. Rigs the same way
(`rig=$(dev/jigc-rig <state> --binary …) || exit; eval "$rig"`, two-step, `mktemp -d` roots, no
teardown, no `rm -rf` on a variable path). **The rule applied** (acceptance-design.md → The
reconciliation rule): a claim by one pass that the other cannot reproduce is a **lead**, not a
finding. Every Codex claim was entered as `lead(codex, …)` and then **driven** — to a repro block or
to a refutation carrying the falsifying datum. Every driver **defect** was **re-driven once** by the
reconciler before being carried.

**Shell note, recorded because it silently corrupted two of my own measurements before I caught it:**
in zsh an unquoted `$v` holding `doc list` is **not** word-split, so `"$J" $v` passes `doc list` as a
single argv token and jigc answers `error: unrecognized subcommand 'doc list'` at **exit 2**. Two
early `doc list` readings of mine showed exit 2 for that reason and not for any jigc behaviour; both
were re-driven with a literal argv (`exit 0`, and `exit 1` under a pre-dispatch fault). No row below
rests on a loop-built argv.

## A · The Codex source pass — every claim, driven

### CX-1 — the departed root-placement orphan is outside `Territory`, so the store sweep returns exit 0 · **CONFIRMED (repro R-R1)**

Codex's one numbered claim, grounded at `orphan.rs:518/529/535/547/600`, `cli.rs:1533/1538`,
`orphan.rs:799`. Driven, and it reproduces exactly — **and it is the same fact as the driver's
C-2**, which the driver also drove and disposed as a *declared, decided* residual. It is therefore
**confirmed and not promoted to a new defect**: both passes independently reached one residual that
the code itself declares.

```
### R-R1 · the root-placement departed-doctype orphan (CONFIRMED — declared residual)
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml
git add -A && git commit -q -m 'drop the methodology pack'

$JIGC validate   -> exit 1
  blocking · schema-conformance.orphaned-instance  @ docs/decisions-log.md
  blocking · schema-conformance.orphaned-instance  @ docs/roadmap.md
  (VISION.md mentions in the whole report: 0)
$JIGC doc list   -> exit 0
  id                   path                    state
  changelog:changelog  CHANGELOG.md            managed
  (none)               docs/decisions-log.md   orphaned
  (none)               docs/roadmap.md         orphaned
  <- VISION.md is not listed AT ALL, not even as `orphaned`
$JIGC ingest     -> exit 0   VISION.md mentions: 0  (and see below)
$JIGC migrate-corpus -> exit 0   VISION.md mentions: 0
head -3 VISION.md -> ---/schema-version: 1/---     <- stamped, formerly managed, silent everywhere
```

**One datum beyond both passes**, found while driving this: `ingest --format json` does not merely
stay silent about the stamped root-placement doc — it **positively classifies it**:

```
{"file": "VISION.md", "best_match": null, "verdict": "unmanaged", "finding": null,
 "adopted": false, "annotations": []}
```

so the same door that says *"unmanaged — … staying a plain file is a legitimate end-state — no
action needed"* says it about a file carrying jigc's own `schema-version:` stamp. That is the C-2
territory miss and **A7-F2** intersecting on one row. Recorded here as evidence, not minted as a
third defect: both parents are already on the table.

**Declared in code, verified:** `crates/cli/src/orphan.rs`, `Territory::resolve` —
`// A placement doctype's home is one file; its *directory* is the territory. // A home at the repo
root has none — residual 2 above.` Codex's *"same declared residual as M51 C-2, not a new defect"*
is correct.

### CX-2 — M51 **D-1** CLOSED · **CONFIRMED (concordant, not independently re-driven)**
Codex cites `ingest.rs:243/249/262/271`; the driver drove it (§4 R-D1) with the verbatim
`file-state.absorbed` block, its Informational route, the JSON key and the no-drift control. Two
passes agree and neither contradicts the other, so the reconciliation rule raises no drive
obligation. Carried CLOSED on the driver's repro.

### CX-3 — M51 **D-2** CLOSED · **CONFIRMED (repro R-R2, independently re-driven on the placement arm)**
Codex cites `migrate_corpus.rs:2354/2368/2375/2383` and `cli.rs:1332/1357`. I rebuilt the
`Relocated` **placement** arm from scratch and drove it; the walk lands the move and commits.
See R-R2 below (it is the same fixture that carries A7-F3).

### CX-4 — M51 **D-3** CLOSED · **CONFIRMED (concordant, not independently re-driven)**
Codex cites `transform.rs:702/737` and `migrate_corpus.rs:1953/1966`. The driver drove it (row m5):
`migrate-corpus.fold-refused`, never `prose-needed`, byte-identical on re-run, `git status` empty.
Concordant; no drive obligation.

### CX-5 — M51 **D-4** CLOSED · **REFUTED IN PART (datum: R-R6)**
Codex disposes D-4 as CLOSED citing `orphan.rs:646/666/671` — which is the **finding's own text**,
i.e. **half 1** only. Half 2 of M51's D-4 is *`ingest` contradicts `validate` at the same commit*,
and Codex's pass is **silent on the `ingest` door**. Driven, half 2 is **live**: at one commit
`validate` exits 1 blocking on two paths whose route's first exit is *"ask `jigc ingest`"*, and
`jigc ingest` exits 0 filing both under *"fine to stay plain … no action needed"* with
`findings: []`. Falsifying datum in **R-R6**. So: **half 1 CONFIRMED CLOSED, half 2 REFUTED** — the
driver's A7-F2 stands.

### CX-6 — M51 **C-1** CLOSED · **CONFIRMED (concordant, not independently re-driven)**
Codex cites `ingest.rs:231/257/265`, `engine/src/ingest.rs:222/240`, `unmanage.rs:94/98`. The driver
drove both halves (§4 R-C1): `edges.json` carries `"from": "vision:vision"` and the following
`unmanage` empties it. Concordant.

### CX-7 — M51 **C-2** NOT CLOSED in full · **CONFIRMED** — identical to CX-1 (R-R1). Both passes,
independently, reach the same declared residual and both decline to mint it as new.

### CX-8 — "the freeze assertion walks every manifest-owning origin pack; project shadows are resolved with origin-pack field types and hash-checked; **I found no schema-changing layer bypass**" · **CONFIRMED for the project layer (repro R-R7) and for the pack layer (corroborated datum); OPEN LEAD for the third origin pack**

Driven: a project-layer shadow that drops the `releases` section blocks **8 of 8** axis-7 doors, and
the `--format json` arm emits exactly one `{"error": …}` document on stderr with **0 bytes** on
stdout (R-R7). The pack-layer arm is corroborated by an independent datum: building the relocation
fixture myself, the binary's own freeze message recomputed
`d9ee5fad7732b4b84c4d2bfc8c0307e929531ab912e66ec8f1533a09242d1422` against a declared
`9e1130df…` — **byte-identical to the two hashes the driver's §4 R-g block quotes**, from a rig I
built independently.

**OPEN LEAD (both passes):** neither pass drifted the **methodology pack's own**
`schema-manifest.yaml`. The driver states this (§6.9); my own rigs inherit it, because
`--pack-from-dev` replaces the composed set with the dev pack alone. Per-origin-pack manifest
resolution (M40 A1) is therefore **read but not driven** on this axis. Reason it stays open: no rig
state ships a reshapeable *methodology* pack copy, so the cell needs a fixture neither
`--pack-from-dev` nor `--repin` builds.

### CX-9 — `SchemaChangeKind::ALL` carries all 18 variants with exhaustive matches; "no undispositioned kind/locus path" · **CONFIRMED as a count · OPEN LEAD as behaviour**
The count matches the driver's independent read of `schema_diff.rs:757` (18) and `LOCI` = 3.
The *behavioural* half — every one of the 54 `kind × locus` cells actually transforms or refuses —
is **not driven by either pass**: the driver drove 7 distinct kinds (§6.1) and Codex drove nothing.
Stays an OPEN LEAD with the reason the driver gives: each undriven cell needs its own manufactured
pack + snapshot + re-pin, and the table is compiler-fenced by `locus_disposition`'s exhaustive match
plus `crates/cli/tests/migrate_locus_axis.rs` — a fence, not a drive.

### CX-10 — `ManifestKind::ALL` contains all six variants, exhaustively tagged and partitioned · **CONFIRMED as a count · n/a as behaviour at this axis**
Count matches the driver's read of `render.rs:1985`. Behaviour is **not reachable from an axis-7
door**, driven: `migrate-corpus` — the only `COMMITTING_DOORS` member in this door set — acks
`committed <sha> — only the migrated paths were staged` and renders **no manifest line**. I saw that
ack again myself in R-R2 (`committed d59f215 — only the migrated paths were staged`). Handed to
axes 2/3/4, not double-counted.

### CX-11 — the fixed-identity predicate is exactly `placement || singleton`, consumed by adoption, unmanage, store-home enumeration and address parsing · **CONFIRMED (repro R-R3)**
Driven at `doc show`: `changelog:notcanonical` → `store.fixed-identity` with a route that **runs**
(`jigc doc show changelog:changelog`), on the findings envelope with a non-null `(code, target)` key.

### CX-12 — "the Axis 7 intersections of `STORE_EXIT_FLIPS` / `ENVELOPE_OWED_CODES` … I found no missing Axis 7 envelope or exit-flip member" · **CONFIRMED as membership · the driver's A7-F3 is orthogonal and untouched by it**
Membership drives clean (R-R3: both envelope codes carry `schema_version`, `findings`, a
`(code, target)` key and a route). Codex's question is membership, not route *content*; A7-F3 is a
route-content defect at `store.not-found`, on which the source pass is **silent**. No conflict —
recorded so the silence is not read as a refutation.

### CX-13 — "the other minted registries (`ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `PRE_DISPATCH_FAULTS`, `InProgress::ALL`, `RelocateRefusal::ALL`, `DESTROYING_DOORS`, `suppressed.door`, `workflow.verb-routed`) do not introduce an additional Axis 7 corpus-walk bypass" · **PARTIALLY CONFIRMED · the rest an OPEN LEAD**
`PRE_DISPATCH_FAULTS` and `InProgress::ALL` **are** reachable at axis-7 doors and both drive clean
(the driver's R-P for the nine git states; my R-R4 for `packs-yaml-malformed`). The remainder is a
**negative completeness claim over registries no axis-7 door is a member of** — `DESTROYING_DOORS`
is `milestone provision/discard/finalize`, `uninstall`, `task discard/finalize`; `ROLLBACK_POPULATIONS`
and `TASK_AREA_FILES` live behind the task and milestone transactions. Undrivable from this axis by
construction → OPEN LEAD, owned by axes 3 and 4.

### CX-14 — **zero schema-hash movement**, M52's stated boundary, not violated · **CONFIRMED (datum)**
Driven against git, not read:

```
git diff 35195f56..HEAD -- '*.yaml' | grep -E '^[+-].*schema-hash'      -> no output
git diff 35195f56..HEAD -- crates/cli/pack/config/schema-manifest.yaml
                           packs/methodology/config/schema-manifest.yaml -> no output
git diff --stat 35195f56..HEAD -- crates/cli/pack/schemas packs/methodology/schemas -> no output
```

Zero `schema-hash` lines moved, zero manifest bytes changed, zero schema files changed between
M51's reviewed commit and HEAD. The workspace stamps `1.0.0-rc.16`, asserted from the binary.

## B · The driver's defects — each re-driven once by the reconciler

| defect | source pass says | reconciler's verdict | evidence |
|---|---|---|---|
| **A7-F1** — the exit-flip trailer asserts a commit and a `git mv` that never happened | **silent** | **CONFIRMED** — and over all three `orphan::Removal` cells, not one | R-R8 |
| **A7-F2** — `ingest` says *no action needed* about the files `validate` blocks on | **silent**, and its D-4 disposition (CX-5) implies closure → **refuted in part** | **CONFIRMED** | R-R6 |
| **A7-F3** — `doc show` dead-ends on a doc stranded by a home relocation | **silent** | **CONFIRMED**, and the class is **wider than the driver's row**: it reproduces on the **placement** home kind too, not only the driver's `location` arm | R-R2 |
| **A7-F4** — referenced at row a8 (*"matches contract, **but see A7-F4**"*) | n/a | **DOES NOT EXIST** — no statement, no repro block, no §5 entry anywhere in the driver file. A **dangling pointer**, annotated in place at row a8. Nothing is carried. The a8 row's own drive is intact (§4 R-a) | grep: one hit in the whole file |

### R-R6 · **A7-F2** re-driven (fixture as R-R1, same commit)

```
$JIGC validate --format json -> exit 1
  [('schema-conformance.orphaned-instance', {'address': 'docs/decisions-log.md', …}),
   ('schema-conformance.orphaned-instance', {'address': 'docs/roadmap.md',       …})]
  each route's first exit: "ask `jigc ingest`, which re-reads the file …"
$JIGC ingest -> exit 0
  unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
  …
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state —
              no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.
$JIGC ingest --format json -> findings: []
  {"file": "docs/decisions-log.md", "best_match": null, "verdict": "unmanaged",
   "finding": null, "adopted": false, "annotations": []}
  {"file": "docs/roadmap.md",       "best_match": null, "verdict": "unmanaged",
   "finding": null, "adopted": false, "annotations": []}
```

The loop is closed and lying: `validate` blocks, routes the reader **at `ingest`**, and `ingest`
answers *no action needed* about the exact two paths, with `findings: []`.

### R-R2 · **A7-F3** re-driven — on the **placement** home kind (the driver drove `location`)

Fixture manufactured from scratch by the reconciler:

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
cp <repo>/crates/cli/pack/config/schema-manifest.yaml "$JIGC_PACK_DIR/config/"   # freeze live
cp <a conformant CHANGELOG.md stamped schema-version 2> CHANGELOG.md
git add -A && git commit -q -m 'a stamped changelog at the declared home'
$JIGC doc list    -> 0  changelog:changelog  CHANGELOG.md  managed      # control
$JIGC doc show changelog -> 0  (renders)                                # control

cp "$JIGC_PACK_DIR/schemas/changelog.yaml" "$JIGC_PACK_DIR/schema-snapshots/changelog.v2.yaml"
# the ONLY schema edit:  placement: { file: CHANGELOG.md } -> { file: HISTORY.md }
# manifest: changelog schema-version 2 -> 3, then re-pin the hash from the binary's own
#   freeze message: declared 9e1130df… / recomputed d9ee5fad…   (== the driver's R-g hashes)
$JIGC doc schema changelog -> 0  "doctype: changelog (schema-version 3)"   # pack loads clean
git status --short -> (empty)   HEAD=d7e1603

# ---- five doors, ONE commit, one document ----
$JIGC validate  -> 1  blocking · schema-conformance.schema-version-current
                      "`CHANGELOG.md`: field `schema-version` is schema-version 2, below the
                       current schema-version 3"     at: changelog:changelog
                      route: "run `jigc migrate-corpus` to upgrade it"            <- CORRECT
$JIGC doc list  -> 0  changelog:changelog  CHANGELOG.md  managed        <- stale home, "managed"
$JIGC ingest    -> 0  blocking · ingest.wrong-location — "conformant `changelog` at `CHANGELOG.md`
                      sits outside `HISTORY.md` — relocate to adopt (jigc never auto-moves)"
                      route: "move CHANGELOG.md into HISTORY.md, then re-run `jigc ingest`"
                                                     <- routes by HAND-MOVE, not migrate-corpus
$JIGC doc show changelog -> 1
  blocking · store.not-found — could not read `changelog:changelog` at `HISTORY.md`:
    No such file or directory (os error 2)
    at: changelog:changelog
    route: create the referenced doc, or fix the reference to an existing one; a doc staged in an
           open task is not committed yet — read it with
           `jigc doc show changelog:changelog --task <task-id>` …          <- A7-F3
# the route `validate` named, run verbatim:
$JIGC migrate-corpus     -> 0  "1 migrated / migrated HISTORY.md / committed d59f215 —
                                only the migrated paths were staged"
$JIGC doc show changelog -> 0  renders, schema-version: 3
ls -> HISTORY.md (CHANGELOG.md gone)
```

Three facts the driver's row could not show, because it drove one home kind:

1. **A7-F3 is not a `location`-arm quirk** — the identical dead end appears on the **placement**
   arm. The document is tracked, on disk, and `doc list` calls it `managed` in the same second that
   `doc show` says it *"could not read"* it, naming a path (`HISTORY.md`) that does not exist and
   offering *create it / fix the reference / read it with `--task`* — three exits, none of which is
   the one `validate` names one command earlier.
2. The **`ingest` divergence the driver recorded at the `location` arm (row m7d) reproduces on the
   placement arm too** — `ingest.wrong-location` routes at a hand `mv`, not at `migrate-corpus`.
   The driver carried that as M52's declared bound; this shows the bound spans both home kinds.
3. **D-2 / CX-3 independently re-driven**: the relocation walk lands and commits.

### R-R8 · **A7-F1** re-driven over all three `orphan::Removal` cells

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
H0=$(git rev-parse HEAD)          # 93b84b40…

# CELL 2 — worktree-only removal, NOTHING staged, NOTHING committed
rm CHANGELOG.md
$JIGC validate -> exit 1
  blocking (gates at finalize) · reconciliation.rename … route: restore …/`jigc unmanage …`  [fine]
  blocking · schema-conformance.home-vacated … route: "the removal is not committed — restore it:
            `git checkout -- CHANGELOG.md`"                                              [fine]
  TRAILER: "out-of-band rename detected — a structural-identity change this commit introduced;
            the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`)."
git rev-parse HEAD -> 93b84b40…  == H0          <- NO commit happened at all
git status --short -> " D CHANGELOG.md"         <- a worktree delete, not a rename
git log -1 --name-status --diff-filter=R -> 0 rename rows

# CELL 3 — staged removal
git rm -q --cached CHANGELOG.md
$JIGC validate -> 1 ; home-vacated route correctly says "the deletion is staged and not committed"
  SAME TRAILER, verbatim: "…this commit introduced … revert the `git mv` …"

# CELL 1 — committed removal
git commit -q -m 'remove the changelog'
$JIGC validate -> 1 ; SAME TRAILER, verbatim
git log -1 --name-status -> "D\tCHANGELOG.md"   <- the commit's ONLY row is a deletion
git log -1 --name-status --diff-filter=R -> 0 rename rows
```

Confirmed in all three cells: the one trailer the reader is given asserts (a) *a commit* where two
of three cells committed nothing, and (b) *a `git mv`* where all three acts were removals, and it
offers `jigc rename` — a verb for an act that did not occur. The driver's **O-2** explains why this
is the *only* trailer the reader gets: `first_store_exit_flip` documents *"table order is
precedence"*, and `oob-rename` precedes `home-vacated`.

## C · The three driver rows DEMOTED for carrying no repro block

Per the brief: *a row marked driven that carries no repro block is **not** driven.* Three §3.4
registry rows carry an argv and a paraphrase of the outcome but **no observed output anywhere in
§4**. All three are annotated in place above and demoted from *driven by the driver*. I re-drove
each; **all three hold**, so they are carried — on the reconciler's evidence, not the driver's.

### R-R3 · `ENVELOPE_OWED_CODES` — the two cells the driver paraphrased

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
$JIGC doc show changelog:notcanonical            -> exit 1
  blocking · store.fixed-identity — "`changelog:notcanonical` is not an address `changelog` can
    have: its identity is fixed — the CLI supplies the slug, so `changelog:changelog` is the one
    instance this doctype has"
    route: "`jigc doc show changelog:changelog` — a fixed-identity doctype has one instance at a
            fixed slug"                                                   <- the route RUNS
$JIGC doc show changelog:notcanonical --format json -> 1, stdout 0 bytes, stderr ONE document:
  {"schema_version": 3, "findings": [{"severity":"blocking","probe":"store",
    "check":"fixed-identity","code":"store.fixed-identity",
    "key":{"code":"store.fixed-identity","target":"changelog:notcanonical"}, …,
    "location":{"address":"changelog:notcanonical","line":1,"col":1}, "route": …}]}
$JIGC doc schema nosuch --format json            -> 1, stdout 0 bytes, stderr ONE document:
  {"schema_version": 3, "findings":[{… "code":"store.unknown-type",
    "key":{"code":"store.unknown-type","target":"nosuch"},
    "message":"unknown doctype `nosuch`",
    "route":"list the available doctypes with `jigc describe`"}]}
```

### R-R4 · `PRE_DISPATCH_FAULTS` — `packs-yaml-malformed` × five doors + the JSON arm

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf 'compose-embedded-methodology: [unterminated\n' > .jigc/config/packs.yaml
$JIGC describe / validate / doc list / ingest / migrate-corpus  -> 1 each (5/5), one message:
  ".jigc/config/packs.yaml is not a valid pack-set list: did not find expected ',' or ']' at
   line 2 column 1, while parsing a flow sequence at line 1 column 31"       <- repo-relative
$JIGC validate --format json -> 1 ; stdout 0 bytes ; stderr ONE {"error": …} document
```

### R-R5 · `unversioned-doctype` — a manifest-less pack

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
# manifest deliberately NOT restored -> the pack declares no schema-manifest.yaml
cp <a conformant CHANGELOG.md> CHANGELOG.md ; git add -A ; git commit -q -m …
$JIGC validate -> exit 0
  advisory · file-state.un-baselined …
  advisory · schema-conformance.unversioned-doctype — "`CHANGELOG.md`: doctype `changelog` is
    defined by a pack that declares no `schema-manifest.yaml` entry for it, so no `schema-version`
    is ever stamped on its instances and the version-currency check cannot tell a current doc from
    an unmigrated one"
    route: "…or state that `changelog` is deliberately unfrozen: an absent manifest entry means
            unchecked by design, so `CHANGELOG.md` is reported here and gated nowhere"
  "2 finding(s) — report-only at store scope (exit 0); each gates nowhere"
$JIGC migrate-corpus -> exit 0  "0 migrated, 0 already current, 0 blocked"
```

Flips no exit — exactly as the driver summarized, and exactly as the finding's own route explains.

### R-R7 · the project-layer shadow, 8 of 8 doors (driver rows f1–f8 / f-json, independently re-driven)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
mkdir -p .jigc/config/schemas
# crates/cli/pack/schemas/changelog.yaml with the whole `releases` section deleted
$JIGC describe · doc schema changelog · validate · doc list · migrate-corpus ·
     migrate CHANGELOG.md --as adr · ingest · unmanage CHANGELOG.md     -> 1 each  (8/8)
  "pack-load freeze check failed: doctype `changelog`: schema-hash mismatch (manifest declares
   `9e1130df…`, recomputed `5eccbcbc…`) — the project schema shadow <path> changes the shape of a
   frozen doctype, which the freeze forbids at every layer (`design/corpus-migration.md` → The freeze)"
  route: "`rm <path>` restores the frozen shape — a project schema shadow may only reword the
          authored presentation keys (`description:`, `usage:`, a slot `hint:`) …"
$JIGC validate --format json -> 1 ; stdout 0 bytes ; stderr ONE {"error": …} document
```

The freeze binds every door. Codex's *"no schema-changing layer bypass"* holds at the layer that
outranks every pack.

## D · Observations the reconciliation added (neither pass reported them; not promoted)

* **R-OBS-1 — the project-layer freeze block prints an ABSOLUTE host path, twice.** In R-R7 both the
  message and the `rm …` route name
  `/private/var/folders/nj/…/repo/.jigc/config/schemas/changelog.yaml`, not a repo-relative path.
  Law 1's printed-path rule has one home (`render::repo_relative`) and M50 left **60 remaining
  producers countable** in an `UNSWEPT_PRODUCERS` table; this is a pack-load `anyhow` channel, the
  same class the driver's **O-1** disposes by name (`engine::finding::is_route_exempt`). **Recorded,
  not minted** — it is almost certainly a known unswept producer, and confirming that is a source
  question this reconciliation does not own. Flagged so the next pass can price it rather than
  rediscover it.
* **R-OBS-2 — CX-1 and A7-F2 intersect on one row.** `ingest --format json` classifies the stamped,
  formerly-managed root-placement `VISION.md` as `{"verdict": "unmanaged", "finding": null}` under
  the legend *"no action needed"*. Neither parent defect is new; the intersection is the sharpest
  single line of evidence for both and is recorded in R-R1.
* The driver's **O-1 through O-5** are carried unchanged. O-1's pack-layer arm was independently
  corroborated (the `d9ee5fad…` recompute); O-2's precedence rule is what R-R8 exercises.

## E · Score

**Codex claims:** 14 entered · **8 CONFIRMED** (CX-1, CX-2, CX-3, CX-4, CX-6, CX-7, CX-11, CX-14)
· **2 CONFIRMED-as-count with the behavioural half open or n/a** (CX-9, CX-10) · **2 PARTIAL**
(CX-8 confirmed at two layers with the third an open lead; CX-13 confirmed where drivable)
· **1 CONFIRMED-as-membership with an orthogonal defect untouched** (CX-12) · **1 REFUTED IN PART**
(CX-5 — D-4's half 2). **Zero Codex claims were promoted on the source read alone.**

**Driver defects:** 3 carried (**A7-F1, A7-F2, A7-F3** — each re-driven; A7-F3's class widened) ·
**1 dangling pointer** (A7-F4 — does not exist).

**Driver rows demoted:** 3 (no repro block), all three re-driven and carried on the reconciler's
evidence.

**Open leads (never dropped, never promoted):** the methodology pack's own manifest (no rig builds a
reshapeable methodology pack); the 40 undriven `SchemaChangeKind × LOCI` cells; `ManifestKind::ALL`
behaviour (no axis-7 door renders a manifest — n/a with a driven reason); the registries no axis-7
door is a member of (`ROLLBACK_POPULATIONS`, `TASK_AREA_FILES`, `DESTROYING_DOORS` × `Disposition`,
`suppressed.door`, 9 of 10 `RelocateRefusal::ALL`); `probe-unreliable`, the 7th `STORE_EXIT_FLIPS`
member; `store.no-such-leaf`, the 2nd `ENVELOPE_OWED_CODES` member; `cwd-unreadable` and
`pack-resource-missing`.

---

# Doors covered (reconciled)

Every clap leaf that is the door of ≥1 **driven** row after reconciliation, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1834`). *(r)* marks a door the **reconciler** also drove independently.

| door | driven rows |
|---|---|
| `describe` | a1 b1 c1 d1 e1 f1 g1 · R-R4 *(r)* · R-R7 *(r)* |
| `doc schema` | a2 b2 c2 d2 e2 f2 g2 · the contract-version-7 identity/home projection · R-R3 *(r)* `store.unknown-type` envelope · R-R2 *(r)* v3 pack-load probe · R-R7 *(r)* |
| `validate` | a3 b3 c3 c3′ d3 e3 f3 g3 · m7a · R-HV 1–4 · R-D1 · R-F1 · R-F2 · unversioned-doctype · R-R1 *(r)* · R-R2 *(r)* · R-R4 *(r)* · R-R5 *(r)* · R-R6 *(r)* · R-R7 *(r)* · R-R8 *(r)* |
| `doc list` | a4 b4 c4 d4 e4 f4 g4 · m6 m7 m7b · R-R1 *(r)* · R-R2 *(r)* · R-R4 *(r)* · R-R7 *(r)* |
| `migrate-corpus` | a5 b5 c5 c5′ d5 e5 f5 g5 · m1–m9 · m7c · the nine `InProgress` rows · unversioned-doctype · R-R1 *(r)* · R-R2 *(r)* · R-R4 *(r)* · R-R5 *(r)* · R-R7 *(r)* |
| `migrate` | a6 a6′ b6 c6 d6 e6 f6 · R-R7 *(r)* |
| `ingest` | a7 b7 c7 d7 e7 f7 · R-D1 · R-C1 · R-F2 · m7d · R-R1 *(r)* · R-R2 *(r)* · R-R4 *(r)* · R-R6 *(r)* · R-R7 *(r)* |
| `unmanage` | a8 b8 c8 d8 e8 f8 · R-C1 · the `--format json` arm · the posture control · R-R7 *(r)* |
| `doc show` | **evidence door** (axis 5's cell) — m7e · the `store.fixed-identity` refusal · R-R2 *(r)* · R-R3 *(r)* |
| `relocate` | **evidence door** (axes 1/3's cells) — m8 `relocate.frozen-doctype`, route run verbatim |

**Not claimed as covered:** `setup` — driven once at exit 0 as fixture construction in R-HV cell 4,
which is the fixture, not a cell of this axis. The driver's own §8 says the same and the
reconciliation does not widen it.

`doc show` and `relocate` remain **evidence doors**: they carry falsifying data for axis-7 rows
(A7-F3 lives at `doc show`) but belong to other axes' door sets, and are listed so the coverage diff
can see them rather than to claim them.
