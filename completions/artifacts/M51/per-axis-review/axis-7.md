<!-- M51 per-axis review — axis 7 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 577a0099), 2026-09-16 -->

# M51 per-axis review — AXIS 7 · freeze & migration — RECONCILED

> This file is the Opus driver table for axis 7, **unchanged except for the demotion marks
> added in §2**, followed by the Reconciliation ledger (§8) that adjudicates the Codex source
> pass against it. Reconciler binary: `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`
> (asserted). Every ledger verdict below was DRIVEN by the reconciler on that binary; scripts
> and captured output live in `../recon/`. No fixes, no commits, no repo edits.

**Binary:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15` (asserted before any drive; RELEASE
posture — the debug route-fence panics do not exist here).
**Repo HEAD at review time:** `67c0c369` (clean tree). Every audit fix named in
`completions/artifacts/M51/VERDICT.md`
is in this binary — confirmed by driving them, not by reading: the orphan **territory** narrowing
(`da5173a1`) is live (§Cell 4 rows: the repo-root placement orphan is silent, the docs-tree one fires),
the route/quoting sweep (`6c2391c0`) is live, and the `setup` guard (`ff2bde99`) is out of this axis.
**No fixes, no commits, no repo edits were made.**

---

## 1 · The door set, derived from the code (not from the design doc's numbers)

Registries read at `67c0c369`, with the count I read:

| registry | file:symbol | rows I counted | how |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1669` | **47** leaves | `grep -cE '^    \(&\['` over the const block |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs:2935` | **14** occurrences | `PathArgOccurrence {` rows |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2357` | **16** rows, **10** of them `DoctypeArg::Address` | rows / `DoctypeArg::Address` hits |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2664` | **6** | `SlugDoor {` rows |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2424` | **25** | `WorkUnitIdDoor {` rows |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:1838` | **47** (the total classification — one row per leaf) | `BehalfDoor {` rows |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:130` | **10** | `CommittingDoor {` rows |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:2625` | **4** (`[&DestroyingDoor; 4]`) | the array type |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:948` | **6** — `probe-unreliable` · `oob-rename` · `unmigrated-corpus` · `ahead-corpus` · `orphaned-instance` · `foreign-squatter` | `StoreExitFlip {` rows |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1834` | **6** — `Promoted`·`Modified`·`Deleted`·`Added`·`Untracked`·`CarriedOver` | the array type |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs:757` | **18** | the array type |
| `LOCI` | `crates/engine/src/schema_diff.rs:144` | **3** — derived: `MAX_NESTING_DEPTH + 1`, and `MAX_NESTING_DEPTH = (MAX_FRAGMENT_HOPS - 1) / 2 = (6-1)/2 = 2` (`schema.rs:748`, `address.rs:198`) | computed from the two constants, not written down |

**Axis 7's door set** is the eight the acceptance design names, and each is a `VERB_KINDS` leaf:
`describe` · `doc schema` · `validate` · `doc list` · `migrate-corpus` · `migrate` · `ingest` · `unmanage`.
None of the caller-token registries above *selects* this door set (they key the other axes); I read them to
state the counts and to confirm that of the eight, `migrate` and `unmanage` are `PATH_ARG_OCCURRENCES`
members, `doc schema`/`doc list` are `DOCTYPE_DOORS` members, and `migrate-corpus` is a `COMMITTING_DOORS`
member — which is why its refusals had to be checked for an exit code as well as a message.

**Axis 7's cell set**, as the design writes it, is three unions:

* **(a) the six freeze/adoption conditions** — hash moved without a version bump · schema-version ahead ·
  unstamped (or below-stamped) managed · orphaned doctype · unadopted foreign · project-layer shadow.
* **(b) `SchemaChangeKind::ALL × LOCI`** — 18 × 3 = **54** cells, of which `locus_disposition`
  (`schema_diff.rs:206`) makes **26 `Applied`**, **10 `Refused`**, **2 kinds `DoctypeLevel`**
  (`Relocated`, `DisplayTitleChanged` — locus-free), and the remainder `Unreachable`.
* **(c) `ManifestKind::ALL`** — 6.

---

## 2 · The table — every row DRIVEN on the installed rc.15

Route kind: `M` = Mechanical (an argv), `H` = Human (prose naming a command or an act), `I` = Informational,
`—` = none. Every row's repro block is in §3, keyed by the ids in the left column.

### 2.1 · The eight doors × the six freeze/adoption conditions (cell set (a)) — 48 rows, 48 driven

| # | door | cell | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| a1 | `describe` | clean control | `jigc describe` | 0 | none | I | catalog body, 105 lines | matches |
| a2 | `doc schema` | clean control | `jigc doc schema changelog` | 0 | none | — | `doctype: changelog (schema-version 2)` | matches |
| a3 | `validate` | clean control | `jigc validate` | 0 | `schema-conformance.repeatable-populated` (advisory) | H | `1 finding(s) — report-only at store scope (exit 0)` | matches |
| a4 | `doc list` | clean control | `jigc doc list` | 0 | none | — | 4 rows, all `managed` | matches |
| a5 | `migrate-corpus` | clean control | `jigc migrate-corpus` | 0 | none | — | `0 migrated, 4 already current, 0 blocked` | matches |
| a6 | `migrate` | clean control | `jigc migrate foreign.md --as adr` | 0 | none | I | `task minted: migrate-adr-foreign-…` | matches |
| a7 | `ingest` | clean control | `jigc ingest` | 0 | none | I | `7 candidate(s) classified` | matches |
| a8 | `unmanage` | clean control | `jigc unmanage README.md` | 0 | none | — | `no-op: README.md is not managed` | matches |
| b1 | `describe` | schema-version **ahead** | `jigc describe` | 0 | none | I | catalog unaffected | matches (pack-load is clean; the break is corpus-side) |
| b2 | `doc schema` | ahead | `jigc doc schema changelog` | 0 | none | — | reports the **manifest** version 2, not the doc's 99 | matches |
| b3 | `validate` | ahead | `jigc validate` | **1** | `schema-conformance.schema-version-ahead` | H | trailer: *"…so the sweep could not adjudicate it and exits non-zero; upgrade jigc, or restore the stamp from git history"* | matches (`STORE_EXIT_FLIPS` member 4) |
| b4 | `doc list` | ahead | `jigc doc list` | 0 | none | — | row state `managed` | matches (state is registration, not currency) |
| b5 | `migrate-corpus` | ahead | `jigc migrate-corpus` | **1** | `migrate-corpus.schema-version-ahead` | H | `blocked CHANGELOG.md` + *"upgrade jigc … or restore the stamp"* | matches (never a silent `already-current`) |
| b6 | `migrate` | ahead | `jigc migrate foreign.md --as adr` | 0 | none | I | mints the foreign-adoption task | matches (a foreign file is not the ahead doc's subject) |
| b7 | `ingest` | ahead | `jigc ingest` | 0 | none | I | calls the **ahead-stamped** `CHANGELOG.md` `adoptable … (adopted — indexed + baselined)` | **DEFECT D-1** (below) — **DEMOTED→RE-DRIVEN** (see §8.3) |
| b8 | `unmanage` | ahead | `jigc unmanage CHANGELOG.md` | 0 | none | H | `unmanaged CHANGELOG.md (changelog:changelog) — dropped its file-state baseline…` | matches |
| c1 | `describe` | **below**-stamp / unstamped managed | `jigc describe` | 0 | none | I | unaffected | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| c2 | `doc schema` | below | `jigc doc schema changelog` | 0 | none | — | manifest version 2 | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| c3 | `validate` | below (stamp 1 < 2) | `jigc validate` | **1** | `schema-conformance.schema-version-current` | H | route: *"run `jigc migrate-corpus` to upgrade it"*; trailer names the exit-flip | matches (member 3) |
| c3′ | `validate` | **unstamped** managed (v0-era) | `jigc validate` | **1** | `schema-conformance.schema-version-current` | H | *"field `schema-version` is absent; the committed doc predates the schema-version stamp"* | matches |
| c4 | `doc list` | below | `jigc doc list` / `--format json` | 0 | none | — | `managed`, `item-count: 2` | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| c5 | `migrate-corpus` | below | `jigc migrate-corpus` | 0 | none | — | `1 migrated … committed ac6e0a4 — only the migrated paths were staged` | matches |
| c5′ | `migrate-corpus` | unstamped managed | `jigc migrate-corpus` | 0 | none | — | `migrated VISION.md`, stamp injected, committed | matches |
| c6 | `migrate` | below | `jigc migrate foreign.md --as adr` | 0 | none | I | task minted | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| c7 | `ingest` | below | `jigc ingest` | 0 | none | I | the below-stamp `CHANGELOG.md` is `adoptable … (adopted)` with no currency mention | **DEFECT D-1** (second instance) — **DEMOTED→RE-DRIVEN** (see §8.3) |
| c8 | `unmanage` | below | `jigc unmanage CHANGELOG.md` | 0 | none | H | baseline dropped; `validate` still exits 1 on the stamp | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| d1 | `describe` | **orphaned doctype** | `jigc describe` | 0 | none | I | the three methodology doctypes are gone from the catalog | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| d2 | `doc schema` | orphaned doctype | `jigc doc schema roadmap` | **1** | `store.unknown-type` | M (`jigc describe`) | `unknown doctype \`roadmap\`` | matches |
| d3 | `validate` | orphaned doctype | `jigc validate` | **1** | `schema-conformance.orphaned-instance` ×2 | H | one finding per orphan, each naming `jigc unmanage <path>` **and** saying `unmanage` alone does not clear it | matches (member 5; the M46 PT-1 rule honoured) |
| d4 | `doc list` | orphaned doctype | `jigc doc list` / `--format json` | 0 | none | — | `(none) docs/roadmap.md orphaned`; JSON `"id": null … "item-count": null` | matches (the declared third state) |
| d5 | `migrate-corpus` | orphaned doctype | `jigc migrate-corpus` | 0 | none | — | `0 migrated, 1 already current, 0 blocked` — silent about both orphans | matches (no schema ⇒ not its subject; `validate` owns the exit) — **DEMOTED→RE-DRIVEN** (see §8.3) |
| d6 | `migrate` | orphaned doctype | `jigc migrate foreign.md --as adr` | 0 | none | I | task minted | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| d7 | `ingest` | orphaned doctype | `jigc ingest` | 0 | none | I | the two orphans are `unmanaged docs/ — 2 file(s) … fine to stay plain` + legend *"no action needed"* | **DEFECT D-4** (below) — **DEMOTED→RE-DRIVEN** (see §8.3) |
| d8 | `unmanage` | orphaned doctype | `jigc unmanage docs/roadmap.md` | 0 | none | — | `dropped its file-state baseline; the file is left on disk`; `validate` still exits 1 — **as the finding's own route predicted** | matches |
| e1 | `describe` | **unadopted foreign** (squatter) | `jigc describe` | 0 | none | I | unaffected | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| e2 | `doc schema` | squatter | `jigc doc schema changelog` | 0 | none | — | version 2 | matches — **DEMOTED→RE-DRIVEN** (see §8.3) |
| e3 | `validate` | squatter | `jigc validate` | **1** | `schema-conformance.unadopted-instance` (**advisory**) | H | *"never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version"*; trailer states the exit-flip | matches (M46: advisory finding, flipped exit) |
| e4 | `doc list` | squatter | `jigc doc list` | 0 | none | — | state `unregistered` | matches |
| e5 | `migrate-corpus` | squatter | `jigc migrate-corpus` (+ `--format json`) | 0 | none | H | `1 not adopted` + a **declared `unadopted` key** in JSON, `blocked: []` — emptiness of `blocked` is the exit rule | matches (M46 Inc 3 exactly) |
| e6 | `migrate` | squatter | `jigc migrate CHANGELOG.md --as changelog` | 0 | none | I | the route's own exit runs: `task minted: migrate-changelog-changelog-…` | matches |
| e7 | `ingest` | squatter | `jigc ingest` | 0 | `conformance.section-renamed` (blocking, row-carried) | M | `needs-reconcile` + route `jigc migrate CHANGELOG.md --as changelog` | matches (the triage-door exit divergence is the declared one) |
| e8 | `unmanage` | squatter | `jigc unmanage CHANGELOG.md` | 0 | none | — | `no-op: … is not managed (nothing to drop)` | matches |
| f1 | `describe` | **project-layer shadow** (shape change) | `jigc describe` | **1** | none (anyhow) | H (`rm <path>` + bump-and-migrate) | `pack-load freeze check failed: doctype \`changelog\`: schema-hash mismatch … the project schema shadow … changes the shape of a frozen doctype` | matches (M49: the freeze binds at every layer) |
| f2 | `doc schema` | project shadow | `jigc doc schema changelog` | **1** | none | H | same block | matches |
| f3 | `validate` | project shadow | `jigc validate` | **1** | none | H | same block | matches |
| f4 | `doc list` | project shadow | `jigc doc list` | **1** | none | H | same block | matches |
| f5 | `migrate-corpus` | project shadow | `jigc migrate-corpus` | **1** | none | H | same block | matches |
| f6 | `migrate` | project shadow | `jigc migrate foreign.md --as adr` | **1** | none | H | same block | matches |
| f7 | `ingest` | project shadow | `jigc ingest` | **1** | none | H | same block | matches |
| f8 | `unmanage` | project shadow | `jigc unmanage CHANGELOG.md` | **1** | none | H | same block | matches |
| g1–g8 | all eight | **hash moved without a version bump** (pack layer) | the same eight argvs | **1** each | none (anyhow) | **—** | `pack-load freeze check failed: doctype \`changelog\`: schema-hash mismatch (manifest declares …, recomputed …)` — and **no route line** | matches contract, **with an observation** (O-1) |

> **Demotion pass (reconciler).** Thirteen rows above were marked *driven* but their argv +
> observed output appears in **no** §3 repro block: `b7` (R-b transcribes seven doors, not
> `ingest`), `c1 c2 c4 c6 c7 c8` (R-c transcribes only `validate` and `migrate-corpus`),
> `d1 d5 d6 d7` (R-d transcribes only `validate`, `doc list`, `doc schema`, `unmanage`), and
> `e1 e2` (R-e transcribes six doors, not `describe`/`doc schema`). Under the reconciliation
> rule those rows were **not driven as presented**, so each is marked above. The reconciler
> then **drove all thirteen itself**; every one reproduced the verdict the driver stated, and
> the repro blocks are §8.3. They therefore stand — on the reconciler's evidence, not the
> driver's.

Two notes on the (f) and (g) rows that I checked against the record rather than judging:

* The shadow arm prints an **absolute host path** in both message and route. That is **not** a law-1
  defect: `crates/cli/tests/repo_relative_paths.rs:682` disposes `assert_project_schema_shadows`
  `Disposition::DeclaredAbsolute` with a stated reason (*"pack-load has no repo-root subject… its `route:`
  span is bytes the operator pastes into a shell of unknown cwd"*), and `crates/cli/src/pack.rs` sits in
  `UNSWEPT_PRODUCERS` with 9 counted sites. Verdict: matches contract.
* The **route floor does not bind** these two rows: they are `anyhow` error paths, not `Finding`s, and
  `engine::finding::is_route_exempt`'s doc-comment already disposes that class by name (*"it is an anyhow
  error path, not a `Finding` … so it never reaches this seam and needs no entry here"*). Recorded as
  observation **O-1**, not a defect.

### 2.2 · `SchemaChangeKind::ALL × LOCI` at the `migrate-corpus` door (cell set (b)) — 10 cells driven of 54

Every cell below was driven by **manufacturing a pack** (`dev/jigc-rig … --pack-from-dev`, the real
`schema-manifest.yaml` restored into it, the prior shape shipped as `schema-snapshots/<ty>.v<k>.yaml`,
the manifest `schema-version` bumped and its `schema-hash` re-pinned from the binary's own freeze message)
and then driving `jigc migrate-corpus` over a committed instance stamped at the prior version.

| # | kind | locus | disposition (code) | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| m1 | `AddedItemField` | 2 (item) | `Applied` | 0 | none | — | `1 migrated` + commit; stamp 2→3, **byte no-op** below the stamp | matches (M46's `set:`-without-default byte fold) |
| m2 | `AddedItemField` | **3 (nested)** | `Applied` | 0 | none | — | `1 migrated` + commit; stamp 2→3 | matches (M50 Inc 6/7's third locus) |
| m3 | `AddedOptionalSection` | 1 | `Applied` | 0 | none | — | `## Notes For Maintainers` minted at the schema-ordered offset; stamp 2→3 | matches (the M36 rationale) |
| m4 | `EnumWidened` | 1 | `Applied` | 0 | none | — | `1 migrated`; committed value `accepted` untouched, stamp 2→3 | matches — and **refutes my own lead** that `corpus-migration.md:122`'s *"an enum widening is misclassified as a rename"* is stale: that bullet sits inside *The classifier's holes (M42)*, whose closing paragraph says M42 closed it. Driven: closed. |
| m5 | `RemovedField` | 2 | `Refused` | **1** | `migrate-corpus.removed-field` | H | *"…the migration never strips a value (no data loss)… This is a schema-authoring gap, not a doc problem"* + restore-or-build route | matches |
| m6 | `RemovedItemSlot` | 2 | `Refused` | **1** | `migrate-corpus.removed-item-slot` | H | names `unreleased-changes.notes` | matches |
| m7 | `RemovedItemSlot` | **3 (nested)** | `Refused` | **1** | `migrate-corpus.removed-item-slot` | H | names the **locus path** `releases/changes.notes`, not a bare section id | matches (M50 Inc 6's locus-path requirement, driven) |
| m8 | `Unclassified` | 2 (nested repeatable block dropped) | `Refused` | **1** | `migrate-corpus.unclassified-change` | H | *"an empty diff is not a no-op: migrating would stamp the doc 3 while leaving it non-conformant"* | matches (the empty-diff backstop) |
| m9 | `ValueRemapped` | 1, plain enum field, **no authored map** | `Applied`→refusal | **1** | `migrate-corpus.fold-refused` | H | *"declare the missing old→new value mapping for `status` in … `authored_remap`"*; bytes rolled back | matches (the map-gap refusal fires and names a real key) |
| m10 | `ValueRemapped` | 2, **`id-from` enum**, no authored map | `Applied`→wrong refusal | **1** | `migrate-corpus.prose-needed` | H, **dead end** | *"author the new required prose in `CHANGELOG.md` through the write verbs, then re-run"* — there is no new prose and the re-run reproduces the block verbatim (exit 1 both times, measured unpiped) | **DEFECT D-3** |
| m11 | `Relocated` | doctype level, **placement** doctype | `DoctypeLevel` | **0** | none | — | `0 migrated, 0 already current, 0 blocked`; JSON all-empty, `commit: null` | **DEFECT D-2** |
| m12 | `Relocated` | doctype level, **`location:`** doctype | `DoctypeLevel` | **0** | none | — | identical: the walk sees nothing | **DEFECT D-2** (second member of the same class) |

**Control that isolates `Relocated` as the failure, driven on the same fixture machinery:** the *same*
`adr` v2→v3 bump with a **content** change (an added optional section) migrates and commits at exit 0
(`1 migrated docs/decisions/use-sqlite.md`, stamp 2→3). So the snapshot naming, the manifest re-pin and
the walk are all sound; only the relocation kind is invisible.

### 2.3 · `ManifestKind::ALL` (cell set (c)) — 0 of 6 driven, and why

`ManifestKind` is the **finalize manifest's** row tag (`render.rs:1885`, rendered at the four boundary
sites). **No door in axis 7's door set renders a manifest**: `migrate-corpus`, the only committing door
here, acks with `committed <sha> — only the migrated paths were staged` and no manifest (driven, rows c5 /
m1 / m3). Reaching `Promoted`/`Modified`/`Added`/`Deleted`/`Untracked`/`CarriedOver` needs
`task finalize` / `milestone finalize`, which are axis 2/4's doors and **flow-52 arm 7**'s subject. I did
**not** drive them and do not present them as driven. See §5.

---

## 3 · Repro blocks

All rigs: `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval; `SCRATCH` exported to this session's scratchpad). No teardown, no `rm -rf` of a variable
path anywhere.

### R-a · the clean control (rows a1–a8)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
$JIGC validate            -> exit 0   (1 advisory: schema-conformance.repeatable-populated)
$JIGC doc list            -> exit 0   4 rows, all `managed`
$JIGC describe            -> exit 0   105 lines
$JIGC doc schema changelog-> exit 0   doctype: changelog (schema-version 2)
$JIGC migrate-corpus      -> exit 0   0 migrated, 4 already current, 0 blocked
$JIGC ingest              -> exit 0   7 candidate(s) classified
$JIGC migrate foreign.md --as adr -> exit 0  task minted: migrate-adr-foreign-0a629e95a3c6
$JIGC unmanage README.md  -> exit 0   no-op: README.md is not managed (nothing to drop)
```

### R-b · schema-version **ahead** (rows b1–b8)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
perl -0pi -e 's/^schema-version: 2$/schema-version: 99/m' CHANGELOG.md
printf '# Foreign note\n\nSome prose.\n' > foreign.md
git add -A && git commit -q -m 'ahead stamp + foreign file'      # 0f474fc

$JIGC validate -> exit 1
  blocking · schema-conformance.schema-version-ahead — `CHANGELOG.md`: field `schema-version` is
    schema-version 99, above the current schema-version 2 …
    route: ahead — … `jigc migrate-corpus` cannot fix a future stamp
  a committed doc is stamped above this build's schema-version … exits non-zero

$JIGC migrate-corpus -> exit 1
  corpus migration: 0 migrated, 3 already current, 1 blocked
    blocked    CHANGELOG.md
      migrate-corpus.schema-version-ahead: … this jigc build has no schema to migrate it to
      route: upgrade jigc to a build whose `changelog` schema-version is at least 99, or restore …

$JIGC doc list  -> exit 0   changelog:changelog  CHANGELOG.md  managed
$JIGC describe  -> exit 0 ; $JIGC doc schema changelog -> exit 0 (reports version 2)
$JIGC migrate foreign.md --as adr -> exit 0 ; $JIGC unmanage CHANGELOG.md -> exit 0
```

### R-c · **below**-stamp and **unstamped** managed (rows c1–c8)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
perl -0pi -e 's/^schema-version: 2$/schema-version: 1/m' CHANGELOG.md
git add -A && git commit -q -m below

$JIGC validate -> exit 1
  blocking · schema-conformance.schema-version-current — … is schema-version 1, below the current 2
    route: migrate — … run `jigc migrate-corpus` to upgrade it
$JIGC migrate-corpus -> exit 0   1 migrated / committed ac6e0a4 — only the migrated paths were staged
```

The v0 arm, same rig, `VISION.md` front matter deleted entirely:

```
perl -0pi -e 's/\A---\nschema-version: 1\n---\n\n//' VISION.md ; git add -A && git commit -q -m unstamped
$JIGC validate -> exit 1
  blocking · schema-conformance.schema-version-current — `VISION.md`: field `schema-version` is absent;
    the committed doc predates the schema-version stamp (below the current schema-version 1)
$JIGC migrate-corpus -> exit 0   migrated VISION.md / committed 03171f5
```

### R-d · **orphaned doctype** (rows d1–d8)

Constructed the honest way — a doctype genuinely leaves the resolved set:

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml
printf '# Foreign note\n\nSome prose.\n' > foreign.md
git add -A && git commit -q -m 'drop methodology pack'

$JIGC validate -> exit 1
  blocking · schema-conformance.orphaned-instance — committed doc `docs/decisions-log.md` sits at a
    jigc-managed home and carries a `schema-version:` stamp, but no resolved doctype claims it …
  blocking · schema-conformance.orphaned-instance — … `docs/roadmap.md` …
$JIGC doc list -> exit 0
  (none)  docs/decisions-log.md  orphaned
  (none)  docs/roadmap.md        orphaned
$JIGC doc list --format json -> "id": null … "item-count": null
$JIGC doc schema roadmap -> exit 1  blocking · store.unknown-type  route: `jigc describe`
$JIGC unmanage docs/roadmap.md -> exit 0 ; $JIGC validate -> exit 1 (still 2 orphan findings)
```

**Two declared-residual confirmations in the same run, both driven:** `VISION.md` — a *root-level
placement* home — is **silent**: it appears in no `validate` finding and in **no `doc list` row at all**,
at exit 0. That is residual 2 of the F1 fix (`orphan.rs:303-308`), driven rather than assumed.

### R-e · **unadopted foreign** (rows e1–e8)

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf '# Changelog\n\nAll notable changes.\n\n## [1.0.0] - 2026-01-01\n### Added\n- thing\n' > CHANGELOG.md
git add -A && git commit -q -m 'foreign changelog'

$JIGC validate -> exit 1   advisory · schema-conformance.unadopted-instance … route: adopt — run
  `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` …
$JIGC doc list -> exit 0   changelog:changelog  CHANGELOG.md  unregistered
$JIGC migrate-corpus -> exit 0   "0 migrated, 0 already current, 0 blocked, 1 not adopted"
$JIGC migrate-corpus --format json -> "blocked": [], "unadopted": [ { … "code":
  "schema-conformance.unadopted-instance", "key": {"code": …, "target": "CHANGELOG.md"} … } ]
$JIGC ingest -> exit 0  needs-reconcile CHANGELOG.md → changelog / blocking ·
  conformance.section-renamed / route: `jigc migrate CHANGELOG.md --as changelog`
$JIGC migrate CHANGELOG.md --as changelog -> exit 0  task minted: migrate-changelog-changelog-b83309faa8b0
$JIGC unmanage CHANGELOG.md -> exit 0  no-op: CHANGELOG.md is not managed (nothing to drop)
```

The whole route chain runs: `validate` → `ingest` → `migrate`. No dead end.

### R-f · **project-layer shadow** (rows f1–f8)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
mkdir -p .jigc/config/schemas
# a whole-file shadow of `changelog` with the `releases` section DROPPED
cat > .jigc/config/schemas/changelog.yaml <<'YAML'  ... (sections: unreleased-changes only) ... YAML
git add -A && git commit -q -m 'project schema shadow drops a section'

# all EIGHT doors, each exit 1, each printing the identical block:
pack-load freeze check failed: doctype `changelog`: schema-hash mismatch (manifest declares
  `9e1130df…`, recomputed `5eccbcbc…`) — the project schema shadow <abs>/.jigc/config/schemas/
  changelog.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer
  route: `rm <abs>/.jigc/config/schemas/changelog.yaml` restores the frozen shape — … changing the
  shape or the home of a manifest-governed doctype means bumping its `schema-version` … and shipping
  a corpus migration
```

### R-g · **hash moved without a version bump**, pack layer (rows g1–g8)

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
# --pack-from-dev DROPS the freeze manifest; restore it so the freeze binds, then move a shape
cp <repo>/crates/cli/pack/config/schema-manifest.yaml "$JIGC_PACK_DIR/config/schema-manifest.yaml"
# add `- { id: tag, type: string, optional: true }` to the `releases` block, leave schema-version at 2

# all EIGHT doors, each exit 1:
pack-load freeze check failed: doctype `changelog`: schema-hash mismatch (manifest declares
  `9e1130df…`, recomputed `ca33acf5…`)
$JIGC validate --format json -> {"error": "pack-load freeze check failed: …"}   exit 1
```

### R-m · the migration cells (rows m1–m12)

Common construction, driven once per cell (only the schema edit differs):

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
cp <repo>/crates/cli/pack/config/schema-manifest.yaml "$JIGC_PACK_DIR/config/schema-manifest.yaml"
cp <repo>/crates/cli/pack/schemas/<ty>.yaml        "$JIGC_PACK_DIR/schema-snapshots/<ty>.v<k>.yaml"
# commit a conformant instance stamped <k>, then: edit the schema, bump the manifest to <k+1>,
# read the recomputed hash out of the binary's own freeze message, re-pin it, assert pack-load clean
$JIGC migrate-corpus
```

Observed, verbatim:

```
m1  AddedItemField @2   -> exit 0   1 migrated / committed 80784bf   (stamp 2->3, no other byte moved)
m2  AddedItemField @3   -> exit 0   1 migrated / committed 74ae9d7
m3  AddedOptionalSection@1 -> exit 0  1 migrated / committed 8a794b0 ; `## Notes For Maintainers` minted
m4  EnumWidened @1      -> exit 0   1 migrated / committed 7c0d8d5 ; `status: accepted` untouched
m5  RemovedField @2     -> exit 1   migrate-corpus.removed-field  (route: restore, or build the strip arm)
m6  RemovedItemSlot @2  -> exit 1   migrate-corpus.removed-item-slot  `unreleased-changes.notes`
m7  RemovedItemSlot @3  -> exit 1   migrate-corpus.removed-item-slot  `releases/changes.notes`
m8  Unclassified @2     -> exit 1   migrate-corpus.unclassified-change
m9  ValueRemapped @1 (plain enum, no map) -> exit 1  migrate-corpus.fold-refused
      route: declare the missing old→new value mapping for `status` in
             `crates/cli/src/migrate_corpus.rs` → `authored_remap`, then re-run `jigc migrate-corpus`
```

### R-D2 · `Relocated` is silently skipped — the defect repro (rows m11, m12)

**placement doctype** (`changelog`, `CHANGELOG.md` → `HISTORY.md`):

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
cp <repo>/crates/cli/pack/config/schema-manifest.yaml "$JIGC_PACK_DIR/config/schema-manifest.yaml"
cp <repo>/crates/cli/pack/schemas/changelog.yaml "$JIGC_PACK_DIR/schema-snapshots/changelog.v2.yaml"
# commit a conformant CHANGELOG.md stamped 2 (the rig's own corpus bytes)
perl -0pi -e 's/^placement: \{ file: CHANGELOG\.md \}$/placement: { file: HISTORY.md }/m' \
  "$JIGC_PACK_DIR/schemas/changelog.yaml"
# manifest changelog -> schema-version 3, schema-hash re-pinned; pack loads clean (exit 0)

$JIGC migrate-corpus  -> exit 0
  corpus migration: 0 migrated, 0 already current, 0 blocked
$JIGC migrate-corpus --format json -> {"migrated":[],"already_current":[],"blocked":[],
  "unadopted":[],"unfilled":[],"commit":null,"hook_output":"","dry_run":false}
$JIGC validate        -> exit 0   "no findings — the committed store validates clean"
$JIGC doc list        -> exit 0   "jigc doc list — no committed docs"
ls *.md               -> CHANGELOG.md still there, still stamped 2
```

**`location:` doctype** (`adr`, `decisions/` → `adrs/`), same shape, with a **binary-shaped** instance
(front-matter `status`/`date`/`schema-version`, verified by `doc show --format json` returning
`"status": "accepted"` before the bump):

```
$JIGC migrate-corpus -> exit 0   corpus migration: 0 migrated, 0 already current, 0 blocked
$JIGC validate       -> exit 1
  blocking · schema-conformance.orphaned-instance — committed doc `docs/decisions/use-sqlite.md` sits
    at a jigc-managed home and carries a `schema-version:` stamp, but no resolved doctype claims it —
    no schema in the composed set says what this file is
$JIGC doc list       -> exit 0   (none)  docs/decisions/use-sqlite.md  orphaned
$JIGC doc show adr:use-sqlite --format json -> exit 1  store.not-found
$JIGC relocate adr --from docs/decisions -> exit 1
  `adr` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`,
  not the freeze-exempt path
$JIGC doc schema adr -> exit 0   doctype: adr (schema-version 3)     # the schema IS in the composed set
$JIGC ingest         -> exit 0
  needs-reconcile docs/decisions/use-sqlite.md → adr
    blocking · ingest.wrong-location — conformant `adr` at `docs/decisions/use-sqlite.md` sits outside
      `docs/adrs/` — relocate to adopt (jigc never auto-moves)
    route: move docs/decisions/use-sqlite.md into docs/adrs/, then re-run `jigc ingest`
```

**Control (same fixture, content change instead of a relocation):**

```
# adr v2->v3 adding an optional `followups` slot section, everything else identical
$JIGC migrate-corpus -> exit 0   1 migrated   docs/decisions/use-sqlite.md / committed f03f303
head -3 docs/decisions/use-sqlite.md -> schema-version: 3
```

### R-D3 · `ValueRemapped` on an `id-from` enum dead-ends (row m10)

```
# changelog v2->v3, enum member renamed: [added, changed, …] -> [added, altered, …]
# `category` is the `id-from` field of both repeatable blocks.
$JIGC migrate-corpus  -> exit 1
  blocked    CHANGELOG.md
    migrate-corpus.prose-needed: `CHANGELOG.md` does not gate clean under its new schema, so its bytes
      are rolled back untouched — the conformance gate reports: id-from field `category` in item
      `unreleased-changes/changed`: `changed` is not an enum member (at unreleased-changes/changed/category)
    route: author the new required prose in `CHANGELOG.md` through the write verbs, then re-run
      `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)

# following the route: there is no new prose to author. Re-running, measured UNPIPED:
$JIGC migrate-corpus > /tmp/rr.txt 2>&1 ; echo $?   -> 1     (byte-identical block)
$JIGC validate       > /tmp/rv.txt 2>&1 ; echo $?   -> 1
  route: migrate — `CHANGELOG.md` is a managed doc below the current schema-version 3; run
  `jigc migrate-corpus` to upgrade it
git status --short -> (empty)   # bytes genuinely untouched — no corruption, only a closed loop
```

Contrast, driven in the same session, that isolates `id-from` as the axis: the **same kind on a plain
(non-`id-from`) enum field** takes the correct arm — `migrate-corpus.fold-refused`, route
*"declare the missing old→new value mapping for `status` in … `authored_remap`"* (row m9).

### R-D1 · `jigc ingest` silently absorbs a blocking out-of-band edit (rows b7, c7, and its own probe)

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
printf '\nAn out-of-band paragraph.\n' >> VISION.md
git add -A && git commit -q -m 'oob edit'

$JIGC validate -> exit 0, and it reports:
  blocking (gates at finalize) · file-state.hash-matches — on-disk content of `VISION.md` differs from
    the recorded state
    route: review the out-of-band edit to `VISION.md` and re-author it through the owning workflow

$JIGC ingest   -> exit 0
  adoptable VISION.md → vision  (adopted — indexed + baselined, no file moved)
$JIGC ingest --format json -> { "file": "VISION.md", "verdict": "adoptable", "finding": null,
                                "adopted": true, "annotations": [] }

$JIGC validate -> exit 0, and the blocking finding is GONE (grep -c file-state.hash-matches == 0)
```

The baseline is durably re-written (`.jigc/state/file-state.json`), and nothing on either surface says
an external edit was absorbed.

---

## 4 · Defects

### D-1 — `jigc ingest` durably absorbs an out-of-band edit and surfaces nothing

* **Door / cell:** `ingest` × {clean-with-drift, ahead, below} (rows b7, c7, R-D1).
* **Contract it contradicts:** `design/reconciliation.md:186` — ***"No silent discard, ever. Every block
  surfaces; every absorb surfaces."*** — and `:44-47`, which names the absorb surface verbatim
  (*"surface `external edit absorbed: <doc>` to the agent"*). Also `:51`: *"the **durable** file-state is
  persisted **only by a landed `finalize`**"*; `ingest` is not a finalize.
* **Observed:** a `blocking (gates at finalize) · file-state.hash-matches` finding, whose own route says
  *"re-author it through the owning workflow"*, is cleared by a `jigc ingest` that prints the ordinary
  first-adoption line and emits `"finding": null, "adopted": true` on the pinned JSON. A later `validate`
  is green. No `external edit absorbed:` line exists on either surface, and nothing distinguishes an
  already-managed doc from a newly-adopted one.
* **Why it matters here:** `ingest` is the route target of `schema-conformance.unadopted-instance` and is
  reached from three of this axis's six cells, so the absorption is on a path the axis's own routes send
  operators down.
* **Severity read:** no data loss (the edit is conformant and the bytes are the user's), but a blocking
  finding is retired by a door that never says it did.

### D-2 — a `Relocated`-only bump is invisible to `migrate-corpus`, and the refusal that routes there is a dead end

* **Door / cell:** `migrate-corpus` × `SchemaChangeKind::Relocated` (rows m11, m12; repro R-D2). Confirmed
  on **both** home kinds — a `placement` doctype and a `location:` doctype.
* **Contracts it contradicts:**
  1. `design/corpus-migration.md` → *"**`relocated { from, to }`** … The corpus walk keys on the **`from`**
     home (where the instances actually sit), not the empty `to`."* Driven, the walk finds **zero**
     candidates, i.e. it keys on the `to` home.
  2. Same doc → *"a **missing** snapshot **blocks** that doc with a route (**never a silent
     `already-current`**)"*. Here the snapshot is present and the doc is not even reported as
     `already-current` — it is reported as nothing at all.
  3. `jigc relocate`'s own refusal and long help: *"A frozen doctype is refused (**use `migrate-corpus`**)"*
     / *"`adr` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`"*. Followed
     exactly, that route changes nothing — the M46 **PT-1** class (a printed route that does nothing),
     reopened at the relocation door.
  4. `design/storage.md` → Placement / M38: the shipped `changelog` v1→v2 root relocation is *the* proof
     case for this kind. The kind is in `SchemaChangeKind::ALL` and `locus_disposition` answers
     `DoctypeLevel` for it, so the table says it is live.
* **Observed, placement arm — a total false green:** `migrate-corpus` exit **0** with all five arrays
  empty and `commit: null`; `validate` exit **0**, *"no findings — the committed store validates clean"*;
  `doc list` *"no committed docs"* — while a committed, managed, below-version doc sits on disk.
  The orphan sweep cannot see it either, because a repo-root placement home is the F1 fix's declared
  residual 2 — so **no surface in the product mentions the document**.
* **Observed, `location:` arm:** `migrate-corpus` exit 0 and silent; `validate` exit 1 but with the
  **wrong diagnosis** (D-4); `doc show` exit 1 `store.not-found`.
* **Isolated by control:** the identical bump with a content change migrates and commits at exit 0.

### D-3 — `ValueRemapped` on an `id-from` enum falls through to `migrate-corpus.prose-needed`, whose route is a dead end

* **Door / cell:** `migrate-corpus` × (`ValueRemapped`, locus 2, `id-from` field) (row m10; repro R-D3).
* **Contract it contradicts:** `design/corpus-migration.md` names this exact failure shape as one M42
  closed — *"a **permanent mutual dead-end** — `validate` says *run the corpus migration*,
  `migrate-corpus` says *author the prose then re-run*, and **neither instruction is actionable**"* — and,
  for the value-remap kind specifically, *"a committed value the map does not cover … **surfaces a hard
  error**"* with *"the map-gap refusal's route [that] tells the operator to declare [the key]"*. The
  binary takes the map-gap arm on a **plain** enum field (driven, m9) and the `prose-needed` arm on an
  **`id-from`** one.
* **Observed:** route says *"author the new required prose … then re-run"*; there is no new prose in the
  schema change, and the re-run reproduces the identical block at exit 1. `validate` exit 1 routes back to
  `migrate-corpus`. Closed loop; bytes untouched (no corruption).
* **Shape:** M45's complete-fix lens — the class was fixed at the cell it was reported in (plain field)
  and left standing at the sibling cell (`id-from` field), where the value is the item's identity and no
  value-span splice exists to rewrite it.

### D-4 — the orphan finding asserts *"no schema in the composed set says what this file is"* about a doc whose schema is in the composed set, and `ingest` contradicts it at the same commit

* **Door / cell:** `validate` (and `ingest`) × {orphaned doctype, and the `Relocated` aftermath} (rows d3,
  d7; repro R-D2 `location:` arm).
* **Contract it contradicts:** `design/surface-contract.md` law 1 (*nothing lies*), and the M51 audit's own
  F1 correction, which rewrote the *other* clause of this same message on exactly this ground (*"what was
  **not** computed is whose stamp it is"*). What was likewise not computed is whether a schema for the
  file exists: the producer computes `claimed` = `committed_instances` over resolved schemas, i.e. *no
  resolved doctype **claims this path***, and the message upgrades that to *no schema says what this file
  is*.
* **Falsifying datum, driven at the same commit:** `jigc doc schema adr` → exit 0,
  `doctype: adr (schema-version 3)`; and `jigc ingest` → `ingest.wrong-location — conformant \`adr\` at
  \`docs/decisions/use-sqlite.md\` sits outside \`docs/adrs/\` — relocate to adopt`, with a followable
  route. Two doors, two stories about one file — the class M46 Increment 3 closed for
  `migrate-corpus` vs the discriminator (*"one producer, so the two doors cannot disagree again"*).
* **Second half, at the `ingest` door (row d7):** in the genuine departed-doctype fixture, `ingest` files
  both orphans under `unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay
  plain)` and its legend says *"staying a plain file is a legitimate end-state — **no action needed**"* —
  about the two files `validate` is exiting 1 over, with a route that says the opposite.
* **Note on scope:** the orphan finding's *route* is not itself a dead end (its "move it to a resolved
  doctype's home" exit works, and `unmanage`'s no-clear behaviour is honestly stated — driven, row d8).
  The defect is the message's claim and the cross-door contradiction.

---

## 5 · What I did **not** drive, and why

Presented as un-driven, never as driven.

1. **44 of the 54 `SchemaChangeKind × LOCI` cells.** I drove 10, chosen to hit every *disposition* at
   every reachable locus: `Applied` at loci 1, 2 **and 3**; `Refused` at loci 2 **and 3** and at locus 1
   via the `Unclassified` backstop's sibling; `DoctypeLevel`; and both arms of `ValueRemapped`. Each
   remaining cell needs its own manufactured pack + snapshot + re-pin (≈3 minutes each), and the table
   itself is compiler-fenced by `locus_disposition`'s exhaustive match plus
   `crates/cli/tests/migrate_locus_axis.rs`. Specifically **not driven**: `AddedRepeatableSection`,
   `FixedSlotToRepeatable`, `AddedItemSlot`, `NarrowedCardinality`, `OptionalRelaxed`, `ProseNeeding`,
   `PresentationOnly`, `WidenedCardinality`, `DisplayTitleChanged`, `Unclassified@1`, and the locus-1/3
   siblings of the kinds I drove at one locus.
2. **All 6 `ManifestKind::ALL` cells at every axis-7 door** — because **no axis-7 door renders a
   manifest** (driven: `migrate-corpus`'s commit ack is `committed <sha> — only the migrated paths were
   staged`, with no manifest line, in rows c5/m1/m3). The manifest's doors are `task finalize` /
   `milestone finalize` / `milestone join`, which belong to axes 2 and 4 and to flow-52 arm 7.
3. **`describe` × `hash moved` / `project shadow` beyond the pack-load block.** Both doors die at
   pack-load before any door-specific behaviour, so there is one observable per door, and I drove that
   one; I did not drive `describe --kind`, `doc schema --format json`, etc. under a red pack.
4. **The `--dry-run` arm of `migrate-corpus`** on any cell (the envelope carries `dry_run: false` in every
   row above; `--dry-run` is axis 5's `EnvelopeArm` subject).
5. **A second manifest-shipping constituent.** Every hash-move row moved the **dev** pack's manifest.
   The methodology pack's own `schema-manifest.yaml` (M40's per-origin resolution) was not independently
   drifted — the `--pack-from-dev` rig replaces the dev pack only.
6. **The Codex source pass for this axis** — deliberately not read, per the brief.

---

## 6 · Observations (driven, not defects)

* **O-1 — the two arms of one freeze check print two different surfaces.** The **project-layer** arm
  (rows f1–f8) appends a `route:` line naming `rm <shadow>` and the bump-plus-migration; the **pack-layer**
  arm (rows g1–g8) prints the hash mismatch with **no route at all**, and `--format json` returns a bare
  `{"error": …}`. This **matches** the stated posture — both are `anyhow` error channels, and
  `engine::finding::is_route_exempt`'s doc-comment disposes that class by name — so it is recorded as an
  asymmetry, not a defect. It is the one place in this axis where a door blocks every verb and names no
  recovery.
* **O-2 — the F1 territory narrowing is real and both its residuals are live.** Driven: a stamped foreign
  `.md` inside `docs/` fires; a departed doctype's instance at a **repo-root placement home** (`VISION.md`)
  is silent at exit 0 **and absent from `doc list`**. Both are the VERDICT's declared residuals 1 and 2,
  confirmed rather than assumed — and residual 2 is what makes D-2's placement arm a *total* false green.
* **O-3 — two leads of my own, refuted with their falsifying data.** (i) I first read a `ValueRemapped`
  no-map bump as a silent committed corruption (stamp flipped, out-of-enum value kept, `validate` green).
  **Refuted:** my fixture was malformed — a hand-written `<!-- fields -->` bullet group where the `adr`
  schema's `header: true` section renders into the **front matter**, so the mutated value was unmodelled
  content. Re-driven with a binary-shaped instance, the map-gap refusal fires correctly (row m9) and
  `validate` emits `schema-conformance.field-value-conformant` (blocking). (ii) I read
  `corpus-migration.md:122`'s enum-widening bullet as a stale claim. **Refuted:** it sits inside *The
  classifier's holes (M42)*, whose own closing paragraph says M42 closed it; driven, a widening migrates
  clean (row m4).

---

## 7 · What this review adds over flow-52 arms 7 and 9

Two flow-52 arms touch this axis. Arm **7** iterates `ManifestKind::ALL` and the fenced-count family
(`COMMITTING_DOORS`' count, `ERROR_CODE_REGISTRY`'s doc mirror) — *"each manifest kind is driven to the
surface that names it, and the numeral every prose home states is the numeral the registry carries."*
Arm **9** iterates `STORE_EXIT_FLIPS` at the **orphan** member — *"a doctype leaves the resolved set:
`jigc validate` exits non-zero … `jigc doc list` prints the row in state `orphaned` … and the D10 sibling
cause answers under its own code at the same surface."*

What this matrix adds:

1. **Breadth across doors, where the arms are deep in one door each.** Arm 9 proves the orphan cell at
   `validate` and `doc list`; this table drives that cell at **all eight** doors and finds the two that
   disagree with it (`ingest` says *no action needed*, `migrate-corpus` says nothing) — a cross-door
   contradiction no single-door arm can see.
2. **The five freeze conditions arm 9 does not iterate.** `ahead` · `below/unstamped` · `unadopted
   foreign` · **project-layer shadow** · **pack-layer hash move** are each driven at all eight doors, 48
   rows. The two freeze-block cells in particular have no flow-52 arm at all: the arm set's registry rows
   are `EnvelopeArm`, `WORK_UNIT_ID_DOORS`, `ManifestKind`, the ambush owe-set and `STORE_EXIT_FLIPS` —
   none of them reaches `assert_schema_freeze`.
3. **The migration transform table driven end-to-end through the release binary.** `SchemaChangeKind ×
   LOCI` is compiler-fenced and unit-fenced (`migrate_locus_axis.rs`), and **no flow-52 arm drives it**.
   Ten cells here were driven by manufacturing a pack, a manifest bump, a snapshot and a re-pin, and
   watching `migrate-corpus` actually rewrite (or refuse) committed bytes. That is where **D-2** and
   **D-3** live — both invisible to a table-shaped fence, because the table says `Relocated` is
   `DoctypeLevel` (true) and says nothing about whether any walk ever reaches it.
4. **`ManifestKind::ALL` is explicitly handed back to arm 7** with the driven reason (no axis-7 door
   renders a manifest), rather than being double-counted here.
5. **The audit's declared residuals are driven, not cited.** Residuals 1 and 2 of the F1 fix are asserted
   in suites per the VERDICT; here they are observed in a live corpus, and residual 2 turns out to be the
   amplifier that makes D-2's placement arm silent on *every* surface.

---

# 8 · Reconciliation ledger

Inputs: the driver table above, and the Codex source pass
`../codex/axis7-codex.md` (prompt beside it). **The rule applied:** a claim by one side that
the other cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as
`lead(codex, …)` and then **driven** — to a repro block, or to a refutation with the driven
datum that falsifies it. Every driver defect was **re-driven once by the reconciler** before
being carried. Nothing below is promoted on a source read.

Scripts + captured output: `../recon/{c1b,c1d,d2,d3,orphan,m12,demoted}.sh` and their `.out`.

## 8.1 · Codex claims

### C-1 — `ingest` and `unmanage` disagree on a placement document's identity → **CONFIRMED (origin codex)**

> *Codex:* "`ingest` … calls the engine adopter without passing that identity …; the adopter
> independently derives the slug from the filename stem … thus `CHANGELOG.md` is indexed as
> `changelog:CHANGELOG`, not canonical `changelog:changelog`. Conversely, `unmanage` explicitly
> derives placement identity as `<type>:<type>` … Expected wrong behaviour: `unmanage` reports the
> baseline dropped, but the edge originating from the hidden … identity remains."

**Driven, and it reproduces — with one correction to the subject.** The literal repro Codex
proposes (`CHANGELOG.md`) is **unobservable**: `changelog` declares no `ref` field, so the
adopted doc contributes zero edges (falsifying datum: `grep -n "type: ref"` over both packs
returns exactly five ref fields — `adr.supersedes`, `vision.grounded-in`, `arch-doc.cites`,
`commit.implements`, `spec.derived-from`). The only **placement** doctype carrying a `ref` is
`vision`, and there the class fires exactly as claimed.

```
# ../recon/c1b.sh — rig: refs-post-hoc, binary rc.15
rig=$(dev/jigc-rig refs-post-hoc --binary ~/.local/bin/jigc) || exit; eval "$rig"
# land the rig's staged `grounded-in` edge so VISION.md is a committed doc WITH an edge
<fill the commit doc> ; $JIGC task finalize "$RIG_TASK"      -> exit 0, promoted VISION.md
# VISION.md front matter now: grounded-in: [research:context-loss]

$JIGC unmanage VISION.md   -> exit 0
  unmanaged VISION.md (vision:vision) — dropped its file-state baseline + forward edges; …
  .jigc/index/edges.json:  { "stamp": "cad077d3…", "edges": [] }

$JIGC ingest               -> exit 0
  adoptable VISION.md → vision  (adopted — indexed + baselined, no file moved)
  .jigc/index/edges.json:  edges: [ { "from": "vision:VISION",        <<-- NOT vision:vision
                                      "relation": "grounded-in",
                                      "to": "research:context-loss" } ]

$JIGC unmanage VISION.md   -> exit 0
  unmanaged VISION.md (vision:vision) — dropped its file-state baseline + forward edges; …
  .jigc/index/edges.json:  edges: [ { "from": "vision:VISION", … } ]   <<-- SURVIVES

git rev-parse HEAD -> cad077d3…   # unmoved, so the stamp matches and nothing rebuilds
```

Two halves, separately adjudicated:

* **Half (a) — the wrong key is persisted: CONFIRMED.** `engine::ingest::adopt` derives the
  `from` identity from the *filename stem* (`crates/engine/src/ingest.rs:226-228`), so a
  placement doctype is indexed under `vision:VISION` — an identity **no other door in the
  product uses**; the very same command sequence has `unmanage` printing the canonical
  `vision:vision` two lines earlier.
* **Half (b) — `unmanage` then lies about it: CONFIRMED.** The ack says *"dropped its
  file-state baseline **+ forward edges**"* at exit 0 while the edge is still in
  `.jigc/index/edges.json` after **two** consecutive `unmanage` runs. (Driven aside, same
  fixture: the "+ forward edges" clause is keyed on *having an identity*, not on an edge
  having existed — row c8 prints it for `changelog`, a doctype with no `ref` field at all.)
* **Half (c) — "continues influencing index consumers": NOT REACHED, stated as a bound.**
  I drove the strongest consumer I could construct — a genuinely dangling ref
  (`grounded-in: [research:never-existed]`, committed) — and `schema-conformance.ref-resolves`
  rendered `vision:vision#grounded-in` **identically** before and after the polluting `ingest`
  (`../recon/c1d.sh`, `diff` of the two `validate` outputs differs only in D-1's absorbed
  `file-state.hash-matches` row). The edge index also self-heals: `load_committed`
  (`crates/engine/src/index.rs:237-252`) rebuilds on any stamp≠HEAD, so the pollution lives
  only in the window before the next commit. **Severity read: a store-lying persisted cache
  plus a law-1 false ack, no reachable wrong answer found.**

### C-2 — the territory bound misses a root-placement orphan → **CONFIRMED (origin codex; declared residual)**

```
# ../recon/orphan.sh — rig: committed-singletons, binary rc.15
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
$JIGC doc list            -> vision:vision VISION.md managed   (before the pack departs)
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml
git add -A && git commit -q -m 'drop the methodology pack'

$JIGC validate  -> exit 1 · 2× schema-conformance.orphaned-instance
                   (docs/decisions-log.md, docs/roadmap.md)
# and for VISION.md — a stamped, formerly-managed doc at a ROOT placement home:
validate       mentions VISION.md: 0
doc list       mentions VISION.md: 0
ingest         mentions VISION.md: 0
migrate-corpus mentions VISION.md: 0
head -3 VISION.md -> ---/schema-version: 1/---    (still there, still stamped)
```

Grounding verified: `crates/cli/src/orphan.rs:293-300` states both residuals in the source, and
`:346` carries the in-line comment *"A home at the repo root has none — residual 2 above."* So
this is **confirmed behaviour that is already on the record as a declared residual** (M51
Inc 8 / VERDICT residual 2) — not a new defect, but it is exactly the amplifier that makes
**D-2**'s placement arm a *total* false green, and it independently corroborates the driver's
observation **O-2**.

## 8.2 · Driver defects — each re-driven by the reconciler

| defect | re-driven | source-pass posture | status |
|---|---|---|---|
| **D-1** `ingest` durably absorbs an out-of-band edit, surfaces nothing | yes (`../recon/c1d.sh`, and row b7 in `../recon/demoted.sh`) | **silent** — Codex read the store sweep's stamp partition and the adoption discriminator, not the file-state baseline write | **CONFIRMED, carried** |
| **D-2** a `Relocated`-only bump is invisible to `migrate-corpus` (both home kinds) | yes (`../recon/d2.sh` placement arm, `../recon/m12.sh` `location:` arm) | **silent, and not contradicted** — Codex's bullet 3 (*"no reachable cell maps to `Unbuilt`"*) is a read of `locus_disposition`'s match, which is about the **disposition**, not about whether the corpus walk ever reaches the doctype; that is precisely the blind spot | **CONFIRMED, carried** |
| **D-3** `ValueRemapped` on an `id-from` enum dead-ends at `migrate-corpus.prose-needed` | yes (`../recon/d3.sh`) | **silent** | **CONFIRMED, carried** |
| **D-4** the orphan finding claims *"no schema in the composed set says what this file is"* about a doc whose schema **is** in the composed set, and `ingest` contradicts it at the same commit | yes (`../recon/m12.sh` first half, `../recon/orphan.sh` second half) | **silent** — Codex's bullet 5 calls the stale/ahead/unadopted partition *"consistently wired"*, which is a claim about the **partition**, not about the sentence the finding prints | **CONFIRMED, carried** |

**D-2 · re-driven, placement arm** (`../recon/d2.out`) — the control is in the same run:

```
rig=$(dev/jigc-rig fresh --pack-from-dev --binary ~/.local/bin/jigc) || exit; eval "$rig"
cp <recon>/CHANGELOG.golden.md CHANGELOG.md && git add -A && git commit -q -m '… stamped 2'
cp <repo>/crates/cli/pack/config/schema-manifest.yaml "$JIGC_PACK_DIR/config/"
cp <repo>/crates/cli/pack/schemas/changelog.yaml "$JIGC_PACK_DIR/schema-snapshots/changelog.v2.yaml"

# CONTROL, before the shape edit — the walk DOES see the doc:
$JIGC migrate-corpus -> exit 0  "0 migrated, 1 already current, 0 blocked / current CHANGELOG.md"
$JIGC doc list       ->         "changelog:changelog  CHANGELOG.md  managed"

# the ONLY edit: placement: { file: CHANGELOG.md } -> { file: HISTORY.md }
# manifest changelog 2 -> 3, schema-hash re-pinned from the binary's own freeze message
$JIGC doc schema changelog -> exit 0  "doctype: changelog (schema-version 3)"   # pack loads clean

$JIGC migrate-corpus -> exit 0
  corpus migration: 0 migrated, 0 already current, 0 blocked
$JIGC migrate-corpus --format json -> {"migrated":[],"already_current":[],"blocked":[],
  "unadopted":[],"unfilled":[],"commit":null,"hook_output":"","dry_run":false}
$JIGC validate  -> exit 0   "no findings — the committed store validates clean"
$JIGC doc list  -> exit 0   "jigc doc list — no committed docs"
head -3 CHANGELOG.md -> schema-version: 2     # still on disk, still below-version
```

**D-2 · re-driven, `location:` arm, which is also D-4's falsifying pair** (`../recon/m12.out`):

```
# same machinery, adr v2 -> v3, the ONLY edit: location: decisions/ -> adrs/
# CONTROL before the edit: doc list -> "adr:use-sqlite docs/decisions/use-sqlite.md managed";
#                          migrate-corpus -> "1 already current"; doc show --format json -> status accepted

$JIGC migrate-corpus -> exit 0   "0 migrated, 0 already current, 0 blocked"
$JIGC validate       -> exit 1
  blocking · schema-conformance.orphaned-instance — committed doc `docs/decisions/use-sqlite.md`
    … no resolved doctype claims it — no schema in the composed set says what this file is
$JIGC doc list       -> exit 0   "(none)  docs/decisions/use-sqlite.md  orphaned"
$JIGC doc show adr:use-sqlite --format json -> exit 1
$JIGC relocate adr --from docs/decisions -> exit 1
  `adr` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`,
  not the freeze-exempt path                       # the route that, followed exactly, does nothing
# --- the two falsifying data for D-4, at this same commit:
$JIGC doc schema adr -> exit 0   "doctype: adr (schema-version 3)"        # the schema IS composed
$JIGC ingest         -> exit 0
  needs-reconcile docs/decisions/use-sqlite.md → adr
    blocking · ingest.wrong-location — conformant `adr` at `docs/decisions/use-sqlite.md` sits
      outside `docs/adrs/` — relocate to adopt (jigc never auto-moves)
    route: move docs/decisions/use-sqlite.md into docs/adrs/, then re-run `jigc ingest`
```

**D-3 · re-driven** (`../recon/d3.out`) — changelog v2→v3, the only edit being the enum member
rename `changed` → `altered` on `category`, which is the `id-from` of both repeatable blocks:

```
$JIGC migrate-corpus -> exit 1
  blocked    CHANGELOG.md
    migrate-corpus.prose-needed: `CHANGELOG.md` does not gate clean under its new schema, so its
      bytes are rolled back untouched — … id-from field `category` in item
      `unreleased-changes/changed`: `changed` is not an enum member
    route: author the new required prose in `CHANGELOG.md` through the write verbs, then re-run
      `jigc migrate-corpus` …
# follow the route: there IS no new prose. Re-run, measured unpiped:
$JIGC migrate-corpus -> exit 1 ; diff of the two captures: BYTE-IDENTICAL
$JIGC validate       -> exit 1 ; route: "run `jigc migrate-corpus` to upgrade it"
git status --short   -> (empty)      # bytes genuinely untouched — a closed loop, no corruption
```

**D-1 · re-driven twice** — once as the driver framed it (`../recon/c1d.sh`: a committed OOB edit
to `VISION.md` raises `blocking (gates at finalize) · file-state.hash-matches`; after `jigc ingest`
that row is **gone** from `jigc validate` and the ack is the ordinary first-adoption line), and once
at the *ahead* cell (`../recon/demoted.sh`, row b7): `jigc ingest` → exit 0,
`adoptable CHANGELOG.md → changelog (adopted — indexed + baselined, no file moved)`, and
`--format json` → `{'file': 'CHANGELOG.md', 'best_match': 'changelog', 'verdict': 'adoptable',
'finding': None, 'adopted': True, 'annotations': []}` — over a doc `jigc validate` is exiting **1**
on for `schema-conformance.schema-version-ahead` at the same commit.

## 8.3 · The thirteen demoted rows, re-driven (`../recon/demoted.out`, `../recon/orphan.out`)

Every one reproduced the driver's stated verdict, so all thirteen **stand on this evidence**:

```
# AHEAD fixture (committed-singletons + `schema-version: 99` committed)
b7  $JIGC ingest -> exit 0  "adoptable CHANGELOG.md → changelog (adopted — indexed + baselined…)"
                  json row: finding=None, adopted=True   ; $JIGC validate still exit 1 (ahead)

# BELOW fixture (committed-singletons + `schema-version: 1` committed)
c1  $JIGC describe            -> exit 0, 105 lines
c2  $JIGC doc schema changelog-> exit 0, "doctype: changelog (schema-version 2)"
c4  $JIGC doc list            -> exit 0, all four rows `managed`; json CHANGELOG "item-count": 2
c6  $JIGC migrate foreign.md --as adr -> exit 0, "task minted: migrate-adr-foreign-0a629e95a3c6"
c7  $JIGC ingest              -> exit 0, "adoptable CHANGELOG.md → changelog (adopted…)"  [D-1]
c8  $JIGC unmanage CHANGELOG.md -> exit 0, "(changelog:changelog) — dropped its file-state
      baseline + forward edges; the file is left on disk…" ; validate still exit 1 on the stamp

# ORPHANED-DOCTYPE fixture (committed-singletons + compose-embedded-methodology: false)
d1  $JIGC describe            -> exit 0; methodology doctypes remaining in the catalog: 0
d5  $JIGC migrate-corpus      -> exit 0, "0 migrated, 1 already current, 0 blocked / current
      CHANGELOG.md"  — silent about both orphans
d6  $JIGC migrate foreign.md --as adr -> exit 0, task minted
d7  $JIGC ingest              -> exit 0, "unmanaged docs/ — 2 file(s) parse against no schema
      (left untouched — fine to stay plain)" + legend "…no action needed."          [D-4 half 2]

# SQUATTER fixture (fresh + a foreign Keep-a-Changelog CHANGELOG.md committed)
e1  $JIGC describe            -> exit 0, 105 lines
e2  $JIGC doc schema changelog-> exit 0, "doctype: changelog (schema-version 2)"
```

## 8.4 · Codex "consistent findings" — checked, none contradicted

| Codex read | reconciler's position |
|---|---|
| freeze seam centralized in `make_pack`; `FREEZE_DOORS` bijected with the clap leaf registry, no undispositioned door | **consistent** with rows f1–f8 / g1–g8 (all eight doors block at pack-load, exit 1) and with observation **O-1** |
| project-layer whole-file shadows hashed with the origin pack's field types + governing manifest | **consistent** — rows f1–f8 drove the block and its `rm <shadow>` route |
| `SchemaChangeKind × LOCI` exhaustively dispositioned; no reachable cell maps to `Unbuilt` | **consistent as stated, and orthogonal to D-2** — the disposition table is right; the *walk* never reaches a `Relocated` doctype's instances |
| `ManifestKind::ALL` has all six members, exhaustive wire-name + committed/left-out partitions; no unnamed kind | **consistent**, and un-driven here by design — §2.3 hands all six to flow-52 arm 7 with the driven reason that no axis-7 door renders a manifest |
| store sweep's stale/ahead/unadopted partition consistently wired | **consistent** for the partition; **silent** on the *sentence* the orphan finding prints (D-4) and on `ingest`'s file-state write (D-1) |

## 8.5 · Open leads

**None.** Both Codex claims were driveable on the shipped rigs and were driven. The one thing
left un-reached is **C-1 half (c)** — a *user-visible* consumer of the mis-keyed edge — which is
recorded above as a stated bound inside a CONFIRMED finding, not as an open lead.

## 8.6 · Doors covered

Every clap leaf that is the door of ≥1 driven row, in `VERB_KINDS` spelling
(`crates/cli/src/cli.rs:1669`):

| door | rows |
|---|---|
| `describe` | a1 b1 c1 d1 e1 f1 g1 |
| `doc schema` | a2 b2 c2 d2 e2 f2 g2 · D-2/D-4 falsifying datum (`doc schema adr` → exit 0) |
| `validate` | a3 b3 c3 c3′ d3 e3 f3 g3 · m11 m12 · D-1 · D-3 |
| `doc list` | a4 b4 c4 d4 e4 f4 g4 · m11 m12 |
| `migrate-corpus` | a5 b5 c5 c5′ d5 e5 f5 g5 · m1–m12 · D-2 · D-3 |
| `migrate` | a6 b6 c6 d6 e6 f6 g6 |
| `ingest` | a7 b7 c7 d7 e7 f7 g7 · D-1 · D-4 half 2 · C-1 |
| `unmanage` | a8 b8 c8 d8 e8 f8 g8 · C-1 |
| `doc show` | evidence door of m12 / D-4 (`doc show adr:use-sqlite --format json` → exit 1 `store.not-found`) and of the C-1 fixture |
| `relocate` | evidence door of D-2 (`relocate adr --from docs/decisions` → exit 1, the route that does nothing) |

The last two are **evidence doors**, not cells of the axis's own (door × cell) table: they carry
the falsifying data for D-2 and D-4 and belong to other axes' door sets.

## 8.7 · Notes

* The reconciler's fixtures are the driver's, rebuilt independently: `dev/jigc-rig
  {committed-singletons, fresh, refs-post-hoc}`, plus the manufactured-pack machinery
  (`--pack-from-dev` + the real `schema-manifest.yaml` restored + a `schema-snapshots/<ty>.v<k>.yaml`
  + a manifest bump + a hash re-pin read out of the binary's own freeze message). Two construction
  facts worth recording for the next run: `--pack-from-dev` **replaces the composed pack set with
  the dev pack alone**, so `committed-singletons` cannot be built on it (the `planning` workflow is
  gone — `workflow-refs.unknown-workflow`); and because that rig **drops the freeze manifest**, a doc
  authored under it carries **no `schema-version` stamp**, so a stamped fixture instance must have
  the stamp restored before the freeze is re-armed.
* No teardown was performed and none is needed — every rig root comes from `mktemp -d`.
* Everything in `../recon/` is a script plus its captured output; nothing was written into any
  jigc workbench, and the working repository was not modified.
