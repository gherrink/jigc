<!-- M52 baseline — area `freeze` (axis 7 rows D-1 … D-4, C-1, C-2). A map at the sha below, not gospel. -->

# M52 baseline — AREA `freeze` (the corpus walk · the absorb doors · orphan territory · the migrate routes)

**Binary:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`, sha256
`126f1584f183636bb6cd9e782b1dc26aca28dd1afaf2fb83da1fd0e5febc8fa9` (asserted before the first drive).
RELEASE posture. **No cargo was run.** **Repo HEAD:** `7637a46f`, tree clean, unmodified.
**Date:** 2026-09-16. **Rig states used:** `fresh --pack-from-dev [--repin]` · `committed-singletons` ·
`refs-post-hoc`. Every root from `mktemp -d`; no teardown performed or needed; nothing written into the
working repository or its `.jigc/`.
**Scripts + captured output:** `<scratchpad>/freeze/{build-corpus,lib,cell,cell-pr,d3,fs-cell,id,terr,misc,a2}.sh`
and `out-*.txt` (one file per cell, each holding the full argv → exit → verbatim output).

---

## §1 · The class enumerations (grep, hit count, what the pattern misses)

### 1.1 The corpus walk's key — `candidate_docs` (`crates/cli/src/migrate_corpus.rs:2075-2131`)

Read, not grepped (one function). The walk has **two branches**, selected on the **current (`to`)**
schema:

* `to.location.is_some()` → `committed_slugs(to.location)` only. **No prior schema is loaded at all.**
* else `to.placement.is_some()` → `⋃{ prior.location | prior ∈ snapshots 1..version }` ∪ `{to.placement.file}`.
  `prior.placement` is **never read** (`.filter_map(|prior| prior.location)`, line 2104).

Therefore the class is the **2×2 of (from-home kind × to-home kind)**, not "the `Relocated` kind":

| from → to | prior home walked? | verdict |
|---|---|---|
| `location:` → `placement:` | yes (prior `location`) | the M38 shipped case — **covered** |
| `location:` → `location:` | **no** (location branch reads no snapshot) | **invisible** |
| `placement:` → `placement:` | **no** (prior `placement` never read) | **invisible** |
| `placement:` → `location:` | **no** (location branch reads no snapshot) | **invisible** |

**3 of 4 cells uncovered.** The review's D-2 drove 2 of the 3 (`L→L` via `adr`, `P→P` via `changelog`).

### 1.2 Durable file-state writers outside a landed `finalize` (D-1's class)

`grep -rn "\.save(\|FileStateRecord::save" crates/cli/src crates/engine/src` → **29 hits**; after dropping
`#[cfg(test)]` seeds, task-dir (`self.dir` / `task.dir`) writers, `roles.json` and `EdgeIndex::save`, the
**production writers of the repo-root `.jigc/state/file-state.json`** are **7 sites / 6 doors**:

`migrate_corpus.rs:968,974` · `unmanage.rs:75,78` · `rename.rs:763` · `ingest.rs:246` ·
`relocate.rs:86` · `milestone.rs:935,1040` · `task.rs:5010` (finalize).
*What the grep misses:* a writer that persists through a helper rather than `.save(` — I found none, but
`engine::state::persist`'s wrapper (`state.rs:501`) is one such shape and is named as a bound.

### 1.3 `file-state.*` codes

`grep -rhon "file-state\.[a-z-]*"` → 6 distinct codes (plus `file-state.json` / `.severity`, not codes):
`hash-matches` (blocking, tunable) · `un-baselined` · `baseline-adopt` · `staged-copy` · `orphaned-doc` ·
`unregistered-doc` (the last four advisory/informational). Cross-checked against
`design/validation.md:412-413,545`.

### 1.4 The "what is this file?" producer family (D-4)

`grep -rn "orphaned-instance\|unadopted-instance\|wrong-location\|unversioned-doctype\|orphaned-doc\|unregistered-doc"`
→ **7 production producers across 4 doors**:
`engine/validate.rs:1116` (`schema-conformance.unadopted-instance`, store sweep) ·
`migrate_corpus.rs:2425` (same code, the M46 `unadopted` key) ·
`orphan.rs:373` (`schema-conformance.orphaned-instance`) ·
`ingest.rs:680` (`ingest.wrong-location`) ·
`cli.rs:1319` (`file-state.orphaned-doc`) · `cli.rs:1343` (`file-state.unregistered-doc`) ·
`engine/validate.rs:1621` (`schema-conformance.unversioned-doctype`).

### 1.5 Placement doctypes and the identity mint (C-1)

`grep -rn "^placement:" crates/cli/pack/schemas packs/methodology/schemas` → **5**: `changelog`
(`CHANGELOG.md`) · `vision` (`VISION.md`) · `roadmap` (`docs/roadmap.md`) · `decisions-log`
(`docs/decisions-log.md`) · `deferral-ledger` (`docs/deferral-ledger.md`).
`engine::ingest::adopt` (`crates/engine/src/ingest.rs:226-228`) mints `from` as
`<type>:<file-stem>`. The canonical identity is `<type>:<type>` for a placement doctype
(`cli::ingest::home_identity`, `crates/cli/src/ingest.rs:555-567`). **The two disagree iff
`stem(placement.file) != ty`** — i.e. **2 of the 5**: `vision` (`VISION`) and `changelog` (`CHANGELOG`).
`grep -rn "type: ref"` → **5** ref fields total; the only one on a placement doctype is
`vision.grounded-in`. So the mis-key is **persisted into `edges.json` for exactly 1 doctype today**.

### 1.6 Routes that name `jigc migrate-corpus` as an operator action

`grep -rn "migrate-corpus\`" crates/cli/src crates/engine/src` → 32 sites; 26 inside
`migrate_corpus.rs` itself. The **cross-door** action routes are **4**:
`engine/validate.rs:1539-1541` (`schema-conformance.schema-version-current`) + its trailer
(`render.rs:1550`) · `relocate.rs:176-181` (the frozen refusal — a real `Route::mechanical(["jigc",
"migrate-corpus"])` embedded in an `anyhow::bail!`) · `ingest.rs:507-509` (the already-managed
reconcile route, parenthetical + conditioned) · `setup.rs:375-386` (`store-version.binary-mismatch`
with `corpus_stale`). `doc.rs:1235` names the verb but is a statement of fact, not a route.

### 1.7 `SchemaChangeKind::ALL × LOCI`

`SchemaChangeKind::ALL` = **18** (`schema_diff.rs:757`); `LOCI` = **3** (derived,
`schema_diff.rs:144`). `locus_disposition` makes `ValueRemapped` **`Applied` at all three loci** — so its
reachable cell set is 3, crossed with the field role {plain, `id-from`} = **6 cells**, of which 5 are
constructible from the shipped dev pack (see §2.3).

---

## §2 · The drives

### 2.1 The home-kind × bump-kind × door matrix (D-2's class)

Construction, identical for every cell: `dev/jigc-rig fresh --pack-from-dev --repin`, one `adr`
(`docs/decisions/use-sqlite.md`) and one `changelog` (`CHANGELOG.md`) **authored through the binary**
(`doc author` + `task finalize`, so both carry real canonical bytes and a real stamp), a control run of
`migrate-corpus`/`doc list`/`validate` **before** the bump, then `cp schemas/<ty>.yaml
schema-snapshots/<ty>.v2.yaml`, one schema edit, manifest `schema-version 2→3`, hash re-pinned from the
binary's own freeze message, `jigc describe` asserted exit 0.

**Control, every cell, before the bump:** `migrate-corpus` → exit 0 `0 migrated, 2 already current,
0 blocked / current CHANGELOG.md / current docs/decisions/use-sqlite.md`; `doc list` → both `managed`;
`validate` → exit 0 `no findings`.

| cell | bump | migrate-corpus | validate | doc list | doc show | ingest | relocate |
|---|---|---|---|---|---|---|---|
| **W1** `adr` `decisions/`→`adrs/` | Relocated only | **0/0/0**, doc absent from every array | **exit 1** `schema-conformance.orphaned-instance` | `(none) docs/decisions/use-sqlite.md orphaned` | exit 1 `store.not-found … at docs/adrs/use-sqlite.md` | `needs-reconcile … ingest.wrong-location … sits outside docs/adrs/` + followable route | exit 1 "`adr` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`" |
| **W2** same + an added optional section | Relocated + content | identical to W1 | identical to W1 | identical | identical | identical | identical |
| **W3** `adr` + optional section only (**control**) | content only | **exit 0 `1 migrated docs/decisions/use-sqlite.md` · committed `b9d7cb1`** | exit 0 clean | `managed` | exit 0 | `adoptable` | — |
| **W4** `changelog` `CHANGELOG.md`→`HISTORY.md` | Relocated only | **0/0/0**, all arrays empty, `commit: null` | **exit 0 `no findings — the committed store validates clean`** | **CHANGELOG.md absent from the listing entirely** | exit 1 `store.not-found` | `needs-reconcile … sits outside HISTORY.md` | frozen refusal → migrate-corpus |
| **W5** same + an added optional section | Relocated + content | 0/0/0 | **exit 0 clean** | absent | exit 1 | **`unmanaged ./ — 3 file(s) … fine to stay plain`** (the doc no longer parses as a changelog, so even `ingest` loses it) | frozen refusal |
| **W6** `CHANGELOG.md`→`docs/history.md` (directory placement) | Relocated only | 0/0/0 | **exit 0 clean** | absent | exit 1 | `needs-reconcile … sits outside docs/history.md` | frozen refusal |
| **W7** `changelog` `placement`→`location: changelog/` | Relocated only | 0/0/0 | **exit 0 clean** | absent | exit 1 | `needs-reconcile … sits outside docs/changelog/` | frozen refusal |
| **W8** `adr` `decisions/`→`placement: DECISIONS.md` | Relocated only | **exit 0 `1 migrated DECISIONS.md` · committed `df2c579`** | exit 0 clean | `adr:adr DECISIONS.md managed` | exit 0 | `adoptable` | — |
| **W9** `changelog` `docs/history.md`→`docs/story.md` under `placement-root: site` | Relocated only | 0/0/0 | **exit 1** `orphaned-instance` at `site/history.md` | `(none) site/history.md orphaned` | exit 1 `… at site/story.md` | `needs-reconcile … sits outside site/story.md` | frozen refusal |
| **W9c** same fixture, an added optional section (**control**) | content only | **exit 0 `1 migrated site/history.md` · committed `aa26dad`** | exit 0 clean | `managed` | exit 0 | — | — |

**Classification.** W3/W8/W9c **built + proven**. W1/W2/W4/W5/W6/W7/W9 **latent defect (D-2's class,
wider than the row)** — the row names *"a `Relocated`-only bump"*; driven, the **bump kind is
irrelevant** (W2 and W5 carry a content change and behave identically) and the **home-kind pair is the
whole discriminator**. `placement-root` does **not** change the walk (W9c migrates fine; W9 does not).

**The amplifier is the home, not the kind.** Where the stranded doc's directory is inside
`orphan::Territory` (`docs/`, `site/`) `validate` at least exits 1 with the *wrong* diagnosis (D-4).
Where it is at the **repo root** (W4, W5, W6, W7 — the `CHANGELOG.md` / `VISION.md` shape) **no surface
in the product mentions the document**: `migrate-corpus` 0/0/0 at exit 0, `validate` *"validates clean"*
at exit 0, `doc list` omits the row. That is C-2's declared residual doing the amplifying, driven.

### 2.2 `ingest` and the file-state family (D-1's class)

Rig `committed-singletons` throughout.

| cell | state | door | observed | class |
|---|---|---|---|---|
| **F1** | conformant OOB edit to committed `VISION.md`, committed | `ingest` | exit 0 · `adoptable VISION.md → vision (adopted — indexed + baselined, no file moved)` · JSON `"finding": null, "adopted": true` · the **blocking** `file-state.hash-matches` row is gone from the next `validate`, no absorb line anywhere | **latent defect = D-1, reproduced** |
| **F3** | same on `docs/roadmap.md` (directory placement) | `ingest` | identical | D-1 is **home-kind-independent** |
| **F2** | **non-conformant** OOB edit (`## Invariants` → `## Invariants Renamed`) | `ingest` | exit 0 · `needs-reconcile VISION.md → vision` · `blocking · conformance.section-renamed` · route *"`VISION.md` is already managed, so adoption is not the repair — reconcile it against the `vision` schema … then re-run `jigc ingest`"* · **the drift finding survives** the next `validate` | **the absorb REFUSES** — built + proven |
| **F4** | `.jigc/state/` emptied (fresh-clone shape) | `ingest` | exit 0 · 4× `file-state.un-baselined` cleared · the ack *"adopted — indexed + baselined"* is **true** here | built + proven |
| **F5** | conformant OOB edit, then the sibling doors | `migrate-corpus` | exit 0 `4 already current` · **the drift finding survives** | built + proven (no absorb) |
| **F5** | " | `unmanage docs/roadmap.md` | exit 0 · baseline dropped, drift replaced by `file-state.un-baselined` — honestly narrated | built + proven |
| **A1** | conformant OOB edit to a committed `adr`, then `jigc rename adr:use-sqlite --to "Use Postgres"` | `rename` | exit 0 · `renamed adr:use-sqlite -> adr:use-postgres (…), repointed 0 referrer(s)` · **commits** (`27c28ed`) · the record is re-keyed with the **post-drift** hash · the blocking `file-state.hash-matches` row is gone from the next `validate` · **no absorb line** | **latent defect — D-1's second member, in no §A row** |
| **A2** | same on a placement singleton | `rename roadmap:roadmap --to "The Plan"` | exit 1 `write.identity-change` with a followable route — never reaches the baseline | built + proven |

**So D-1's class is `{ingest, rename}` of the 6 candidate doors** — `migrate-corpus` driven *not* to
absorb, `unmanage` narrates, `relocate` no-ops on an unmoved home, `milestone`/`task` are finalizes.
**And the classification is right in both offenders — only the surface is missing:** `reconciliation.md:44-47`'s
absorb arm (parse + schema pass) is exactly what `engine::ingest::adopt` re-gates on, and F2 proves the
block arm fires. What is violated is `reconciliation.md:186` (*"every absorb surfaces"*) and
`:51` (*"the **durable** file-state is persisted **only by a landed `finalize`**"* — `ingest` lands no
commit at all).

### 2.3 `ValueRemapped` × locus × field role (D-3's class)

Same manufactured-pack machinery; the base shape is edited **before** the snapshot so the extra plain
enum field exists at both versions, and the instance is authored through the binary at each locus.

| cell | locus | field role | arm taken | route | class |
|---|---|---|---|---|---|
| m9 (review) | 1, section field | plain | `migrate-corpus.fold-refused` | *"declare the missing old→new value mapping for `status` in … `authored_remap`"* | correct |
| **D3a** | 2, `releases/1-0-0/impact` | plain | `migrate-corpus.fold-refused` | *"…for `releases` in `crates/cli/src/migrate_corpus.rs` → `authored_remap`"* | **built + proven (new cell)** |
| **D3d** | 2, `unreleased-changes/changed/category` | **`id-from`** | `migrate-corpus.prose-needed` | *"author the new required prose … then re-run"* — re-run reproduces byte-identically at exit 1 | **latent defect = D-3, reproduced** |
| **D3c** | 3, `releases/1-0-0/changes/changed/impact` | plain | `migrate-corpus.fold-refused`, route names the locus path `releases/changes` | correct | **built + proven (new cell)** |
| **D3b** | 3, `releases/1-0-0/changes/changed/category` | **`id-from`** | `migrate-corpus.prose-needed`, message names `releases/1-0-0/changes/changed` | dead end, re-run identical | **latent defect — D-3's SECOND member, in no §A row** |

**D-3's class is 2 cells, not 1.** The discriminator driven is the **`id-from` role**, and it is
locus-independent: it fires at locus 2 *and* locus 3. Both plain-field cells take the correct arm at both
loci, so the row's implicit "locus 2" framing is not the axis.

**A measured constraint on any fix** (not a recommendation): `authored_remap`
(`migrate_corpus.rs:1194-1202`) is a **hard-coded match in jigc's own source with exactly one entry**
(`deferral-ledger/entries/kind`). The `fold-refused` route therefore names a jigc source file. Under
M49's PB-1 a **project pack** may ship its own doctype — and a project-pack enum rename has **no
declarable map at all**; the route it would print points into jigc's source tree. Not driven (no project
pack built); recorded as a source-read lead with its site.

### 2.4 Orphan territory (C-2's class) and the D-4 producer family

| cell | state | validate | doc list | ingest | migrate-corpus |
|---|---|---|---|---|---|
| **T3** (the review's fixture) | methodology pack dropped, default `placement-root` | exit 1, **2** `orphaned-instance` (`docs/decisions-log.md`, `docs/roadmap.md`); **`VISION.md` silent** | 2 rows `orphaned`; VISION.md **absent** | `unmanaged docs/ — 2 file(s) … fine to stay plain` + legend *"no action needed"* | `0 migrated, 1 already current` — silent about all three |
| **T1** | `jigc config set placement-root .` (which **moved** `docs/roadmap.md`→`roadmap.md`, `docs/decisions-log.md`→`decisions-log.md` — the M49 floor, exit 0, correct), **then** the pack departs | **exit 0 · "no findings — the committed store validates clean"** | only `CHANGELOG.md`; **all three orphans absent** | `unmanaged ./ — 5 file(s) … fine to stay plain` | `0 migrated, 1 already current` |
| **T2** | stamped foreign `.md` at `notes/team.md` and `root-note.md`, pack loaded | silent (correct — outside `Territory`) | absent | `unmanaged` | silent | **built + proven** (the M51 narrowing, as declared) |
| **T4/T5** | `config set docs-root documentation` with a committed `location:` doc | the floor moved `docs/research/context-loss.md` → `documentation/research/context-loss.md`, exit 0; `validate` clean; `doc list` `managed` | | | | **built + proven** |
| **U1** | manifest dropped (freeze-exempt dev pack) | exit 0 · advisory `schema-conformance.unversioned-doctype` with a two-arm route | `managed` | `adoptable` | `0/0/0` (correct — no frozen doctype to migrate) | **built + proven** |

**C-2 is bigger than "a root-placement orphan".** The residual is *"a repo-root home contributes no
directory"* (`orphan.rs:303-308`, `:346`), so its size is **the number of doctypes whose resolved
placement home has no directory component** — **2 at the pack default** (`vision`, `changelog`), and
**5 under `placement-root: .`**, a shipped, documented, floor-supported knob value that is exactly what
M38's *"ecosystem-idiomatic ones managed directly at repo root"* convention invites. Driven (T1): with
that knob set, a departed pack produces **`jigc validate` → exit 0, "the committed store validates
clean"** over three stamped orphans.

**D-4 — the cross-door disagreement, driven at 5 states.** Two doors, two stories about one file:

* W1/W2 (`L→L` relocation): `validate` *"no schema in the composed set says what this file is"* vs
  `doc schema adr` → exit 0 `doctype: adr (schema-version 3)` vs `ingest` → `ingest.wrong-location —
  conformant \`adr\` … sits outside docs/adrs/` with a followable route. **The schema IS composed.**
* W9 (`placement-root` relocation): identical pair at `site/history.md`.
* W4/W6/W7 (root-placement relocation): `validate` says **nothing** while `ingest` raises a **blocking**
  `ingest.wrong-location` at the same commit — the *sharper* half of D-4 and one the row does not state.
* T3 (departed doctype): `validate` exit 1 blocking vs `ingest` *"fine to stay plain — no action needed"*.
* Squatter (review's e-row): `validate` advisory → `ingest` → `migrate --as` — **the one cell where the
  family agrees**; the chain runs end to end.

### 2.5 The placement identity (C-1's class)

Rig `refs-post-hoc`, the staged `grounded-in` edge landed by `task finalize` first, so `VISION.md` is a
committed doc **with** an edge.

* **New, and stronger than the row: a plain `jigc ingest` over an already-managed, clean corpus
  DUPLICATES the edge.** No `unmanage` needed.
  `jigc ingest` → exit 0; `.jigc/index/edges.json` then holds **both**
  `{"from":"vision:VISION",…}` and `{"from":"vision:vision",…}` pointing at `research:context-loss`.
  The review's repro reached the mis-key only after an `unmanage` had removed the canonical row.
* **`unmanage` then says *nothing to drop* about a doc that has a live edge.** After the mis-key exists,
  `jigc unmanage VISION.md` → exit 0 `unmanaged VISION.md (vision:vision) — dropped its file-state
  baseline + forward edges` while `vision:VISION` **survives**; a third run → exit 0 `no-op: VISION.md is
  not managed (nothing to drop)` with the edge **still in the index**. Both are law-1 lies about the same
  file at the same commit.
* **The class is `stem(placement.file) != ty`, i.e. 2 of 5 placement doctypes** — `vision` and
  `changelog`. Driven: `unmanage docs/roadmap.md` → `(roadmap:roadmap)` and `ingest` re-keys it
  `roadmap:roadmap` — **they coincide**, so `roadmap`/`decisions-log`/`deferral-ledger` do not diverge.
  Only `vision` carries a `ref`, so only `vision` puts the wrong key on disk today.
* **A measured constraint on any fix:** the correct identity is already computed in the same call chain —
  `cli::ingest::classify_row` → `home_identity` (`ingest.rs:555`) mints `<type>:<type>` for a placement
  doctype and is what M51 Inc 9's `ingest.unaddressable-identity` keys on. `engine::ingest::adopt`
  (`engine/src/ingest.rs:226-228`) then **re-derives** it from the filename stem two lines later. One
  producer exists; the adopt seam does not take it.
* **Half (c) — a user-visible wrong answer from the mis-keyed edge — NOT reached**, same as the review.
  Added datum: `grep "inverse-card"` → the only declaration is `spec.derived-from`'s `"1..*"`, which is
  **unbounded above**, so no shipped inverse cardinality can be violated by a duplicate edge.
  `engine::index::load_committed` (`index.rs:237-252`) rebuilds on any `stamp != HEAD`, so the pollution
  lives only until the next commit.

### 2.6 The routes that name `migrate-corpus` (the M46 PT-1 class)

| route | producer | state driven | followed, it… |
|---|---|---|---|
| *"`<ty>` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`"* | `relocate.rs:176-181` (a real `Route::mechanical(["jigc","migrate-corpus"])` inside an `anyhow::bail!`) | W1 W2 W4 W6 W7 W9 (every uncovered relocation cell) | **changes nothing** — `0 migrated, 0 already current, 0 blocked`. **PT-1, 6 driven instances** |
| *"migrate — `<doc>` is a managed doc below the current schema-version N; run `jigc migrate-corpus` to upgrade it"* | `engine/validate.rs:1539-1541` + `render.rs:1550` trailer | D3d, D3b (`ValueRemapped` on an `id-from` enum) | **loops**: `migrate-corpus` → `prose-needed` → *"author the prose then re-run"* → `validate` → *"run `jigc migrate-corpus`"*. **PT-1 in those 2 cells; correct in every other cell driven** (W3, W8, W9c, D3a, D3c all migrate) |
| *"…(a corpus left on an older schema-version migrates with `jigc migrate-corpus`)…"* | `ingest.rs:507-509` | F2 (already-managed, non-conformant, at current version) | the clause is **conditioned** on being below-version, and the doc was not; `migrate-corpus` correctly reports `already current`. **built + proven** |
| *"run `jigc migrate-corpus` to upgrade the committed docs, then re-run `jigc setup`"* | `setup.rs:375-386` | **not driven** — needs a store stamped by a different binary version | see §5 |

---

## §3 · What changed against the review's rows

**The walk's key — the exact sentences contradicted, with lines.**

1. `design/corpus-migration.md:68` — *"The corpus walk keys on the **`from`** home (where the instances
   actually sit), not the empty `to`."* **Driven false for 3 of the 4 (from × to) home-kind pairs.** It
   is true for exactly the one pair M38 shipped (`location:` → `placement:`).
2. `design/corpus-migration.md:299` (the Relocation-safety acceptance row) — *"the corpus walk enumerates
   the **`from`** home"*. Same falsification; this row is the one an acceptance is derived from.
3. `crates/cli/src/migrate_corpus.rs:2072-2074` (the doc-comment M42's completion audit wrote) —
   *"*Wherever* a partially-completed migration left them is a claim over **every** home the doctype has
   ever declared, so the walk is the union of all of them."* **Driven false**: the union is over every
   prior **`location:`** home, in the **placement branch only**.
4. `design/corpus-migration.md:90` — *"a **missing** snapshot **blocks** that doc with a route (**never a
   silent `already-current`**)"*. Worse than a silent `already-current` is driven: in W1/W4/W6/W7/W9 the
   doc appears in **no** bucket at all, and `migrate-corpus --format json` returns every array empty with
   `commit: null`.
5. `design/storage.md:208` (the M49 placement census) lists `candidate_docs`' prior-home read as
   *"verified at HEAD, no action"*. That row's claim is only that a **placement branch exists** — which it
   does — so it is **not** falsified; the completeness claim lives in (1)–(3). Stated so the plan does not
   over-read it.
6. `design/storage.md:257` — *"relocating a **frozen** doctype trips the version gate"*. True and
   **driven**; what is not true is that tripping the gate reaches the instances.

**What a `from`-keyed walk would need to know — measured, not assumed.**

* **The prior home is recorded, in full.** `schema-snapshots/<ty>.v<k>.yaml` is the **whole prior
  `Schema`**, `location:` **and** `placement:` included. `crate::pack::load_prior_schema`
  (`pack.rs:89-102`) loads one; `crate::pack::prior_doctype_schemas` (`pack.rs:199-213`) already loads
  **every** shape `1..current` for every versioned doctype and is consumed at **7 sites** (`migrate_corpus.rs:645`,
  `task.rs:1596`, `milestone.rs:1019,4872`, `doc.rs:4127,4238`, `cli.rs:1209`). Nothing new has to be
  stored. Driven corroboration: in W1 the `adr.v2` snapshot carrying `location: decisions/` was **present
  and correct** and the walk still found nothing, because the `to.location` branch loads no snapshot at all.
* **The manifest carries no prior entry** (checked: `config/schema-manifest.yaml` holds one
  `schema-version` + one `schema-hash` per doctype, no history) — so the manifest is *not* the source; the
  snapshot store is.
* **The snapshot obligation is already on the checklist**: `implementation/doctype-authoring.md:21` —
  *"Any later shape **or home** change = version bump + versioned corpus migration … add
  `schema-snapshots/<ty>.v<k>.yaml`"*. So a relocation bump ships its own prior shape by construction.
* **Cost against the zero-schema-hash-movement rule: none.** Closing D-2 needs **no shipped schema's hash
  to move, no `schema-version` to bump and no corpus to migrate** — the change is entirely inside
  `candidate_docs`' enumeration (read `prior.placement` as well as `prior.location`; run the prior-home
  union in the `to.location` branch too). Two second-order facts the plan needs: (a) a prior **`location:`**
  is stored raw and re-rooted through `docs_root` at `migrate_corpus.rs:2112-2116`, while a prior
  **`placement.file`** with a leading component would need the **`placement-root`** knob applied — and
  `DoctypeMigration` (`migrate_corpus.rs:76-81`) carries `docs_root` **only**; (b) widening the walk
  widens the **destination-collision** surface `destination_collision_route`
  (`migrate_corpus.rs:1982`) already exists for, since prior-placement and current-placement can both be
  populated.

**Row-by-row deltas.**

* **D-2** — the row says *"a `Relocated`-only schema bump"*. **Driven: the bump kind is irrelevant** (W2,
  W5 carry content changes and behave identically); the class is the **(from-home kind × to-home kind)
  pair**, 3 of 4 uncovered, and the row drove 2 of the 3 (the `placement:`→`location:` cell, W7, is new).
  The row also says *"on both home kinds"* — driven, it is **four** home shapes: located dir, root
  placement, directory placement, and a `placement-root`-rerooted placement.
* **D-3** — the row names one cell. **Driven: two** (`id-from` at locus **2** *and* locus **3**), and the
  two plain-field cells at those loci are **correct**, so the axis is the field's `id-from` role, not the
  locus.
* **D-1** — the row names `ingest`. **Driven: `jigc rename` does it too**, and it *commits* while doing
  it. The row also does not say that the classifier is **right** (F2: a non-conformant drift is refused) —
  which narrows what a fix has to touch to the *surface*, not the gate.
* **D-4** — the row's falsifying pair is `doc schema` vs `validate`. **Driven, the sharper pair is
  `ingest` vs `validate` at a root-placement home**, where `validate` says nothing at all while `ingest`
  raises a **blocking** finding about the same file at the same commit.
* **C-1** — the row's repro needs an `unmanage` first. **Driven: plain `jigc ingest` on a healthy corpus
  duplicates the edge**, and the class is `stem(placement.file) != ty` (**2** of 5 placement doctypes),
  not "placement doctypes" (5).
* **C-2** — the row calls it a declared residual at the repo root. **Driven: its size is a function of a
  shipped knob** — 2 doctypes at the default, **5** under `placement-root: .`, where a departed pack
  yields `jigc validate` → **exit 0, "validates clean"** over three stamped orphans.

---

## §4 · Latent defects not in any §A row (each driven)

* **L-1 · `jigc rename` silently absorbs an out-of-band edit and commits it.**
  `jigc rename adr:use-sqlite --to "Use Postgres"` over a committed `adr` carrying a committed OOB edit →
  exit 0, `renamed adr:use-sqlite -> adr:use-postgres (…), repointed 0 referrer(s)`, commit `27c28ed`,
  `.jigc/state/file-state.json` re-keyed to the **post-drift** hash, and the previously `blocking
  (gates at finalize) · file-state.hash-matches` row absent from the next `jigc validate`. No line on any
  surface says an external edit was absorbed. Contradicts `design/reconciliation.md:186`.
* **L-2 · `jigc ingest` duplicates a placement doctype's forward edges on a healthy corpus.**
  `dev/jigc-rig refs-post-hoc` → land the staged edge → `jigc ingest` → exit 0; `.jigc/index/edges.json`
  holds `vision:VISION` **and** `vision:vision`, same relation, same target. No `unmanage` involved.
* **L-3 · `jigc unmanage` reports *nothing to drop* about a doc whose edge it is leaving in the index.**
  Third consecutive `jigc unmanage VISION.md` → exit 0 `no-op: VISION.md is not managed (nothing to
  drop)` while `edges.json` still carries `{"from":"vision:VISION","relation":"grounded-in",…}`.
* **L-4 · `ValueRemapped` on an `id-from` enum dead-ends at the NESTED locus too.**
  `jigc migrate-corpus` over a `changelog` whose `releases/1-0-0/changes/changed` sits under a renamed
  `category` enum → exit 1 `migrate-corpus.prose-needed … id-from field \`category\` in item
  \`releases/1-0-0/changes/changed\``, route *"author the new required prose … then re-run"*; the re-run
  is byte-identical, `git status --short` empty.
* **L-5 · `placement-root: .` silently empties the orphan sweep.**
  `jigc config set placement-root .` (exit 0, moves two docs correctly) → drop the methodology pack →
  `jigc validate` → **exit 0 · "no findings — the committed store validates clean"** over
  `VISION.md`, `roadmap.md` and `decisions-log.md`, all committed and all carrying `schema-version:`.
  `jigc doc list` lists none of them. This is C-2's declared residual, but its **size is knob-dependent**
  and the record states it as one fixed cell.
* **L-6 · `migrate-corpus` loses the doc to `unmanaged … fine to stay plain` when a relocation carries a
  content change (W5).** With `CHANGELOG.md`→`HISTORY.md` **plus** an added optional section, `ingest` —
  the one door that still saw the file in W4/W6/W7 — files it under *"parse against no schema (left
  untouched — fine to stay plain)"*. So the *last* surface that named the document goes quiet, and the
  state is reachable from an ordinary bump that changes both a home and a shape.
* **O-1 (observation, not a defect) · the stated reason for `conformance.section-renamed`'s route
  exemption is falsified by jigc's own producer.** `engine::finding::CONFORMANCE_PARSE_DIAGNOSTICS`'
  doc-comment justifies the exemption as *"there is no address to route at"*; driven, the store sweep
  prints the finding with **no route** and `"route": null` in the pinned JSON (F2), while `jigc ingest`
  prints the **same code** with a followable `Route::human` at the same commit. The exemption is
  declared, so this is a record-truth item, not a floor breach.

---

## §5 · Honest bounds — what I did **not** drive, and why

1. **`setup.rs:375-386`'s `corpus_stale` route** — needs a store stamped by a *different* jigc build; I
   had one binary and the brief forbids cargo. Listed in §1.6, undriven.
2. **44+ of the 54 `SchemaChangeKind × LOCI` cells.** I drove the `ValueRemapped` row completely
   (5 of its 6 role×locus cells) and `Relocated` completely; every other kind is the review's §5 bound,
   unchanged. **The one `ValueRemapped` cell I could not construct** is *locus 1 × `id-from`* — a
   **doc-level** `id-from` pointing at an enum field. No shipped doctype declares one (`adr`/`changelog`
   use `id-from: title`), and manufacturing it would also move the slug rule's input, so I left it.
3. **C-1 half (c)** — a user-visible wrong answer caused by the mis-keyed edge. Not reached, same as the
   review, now with the datum that no shipped `inverse-card` is bounded above.
4. **`deferral-ledger`** (the fifth placement doctype) was never materialised in any rig state, so its
   identity was inferred from the schema (`stem == ty`, so no divergence) rather than driven. Named here
   rather than presented as driven.
5. **The project-pack (`PB-1`) arm of §2.3's `authored_remap` constraint** is a **source read**, not a
   drive: I built no project pack. It is stated as a lead with its site.
6. **`ManifestKind::ALL`** — no door in this area renders a finalize manifest (driven: `migrate-corpus`'s
   ack is `committed <sha> — only the migrated paths were staged`, no manifest line). Handed to the
   milestone/task-finalize area, as the review did.
7. **The methodology pack's own manifest was never independently drifted** — `--pack-from-dev` replaces
   the dev pack only, so every freeze/bump cell here moved the **dev** manifest. A per-origin-pack
   divergence is undriven.
8. **Concurrency** — every drive is single-process. The `file-state.json` merge behaviour under two
   simultaneous absorbing doors is untested here.
9. **`engine::state::persist`'s wrapper** could hide a seventh durable file-state writer that my
   `.save(` grep would miss; I read its doc-comment (`state.rs:501`, "this wraps
   `FileStateRecord::save`") but did not enumerate its callers.
