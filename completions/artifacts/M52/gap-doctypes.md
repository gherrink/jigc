# M52 gap probe — dimension `doctypes`

Binary `~/.local/bin/jigc` = `jigc 1.0.0-rc.15`, repo HEAD `7637a46f`, tree unmodified, **no cargo run**.
Rig states: `fresh`, `fresh --pack-from-dev --schema adr <manufactured>`, `committed-singletons`. Every
root from `mktemp -d`; nothing written into the working repo. *driven* / *read* marked per claim.

---

## 0 · The headline for this dimension

**No M52 fix needs a schema-hash to move, and no pack workflow/step edit can move one** — `schema_hash`
digests the `Schema` value alone, exhaustively destructured (`crates/engine/src/manifest.rs:77-89`:
`ty, location, id_from, display_title, placement, singleton, sections`; `description`/`usage`/slot
`hint` erased). Step and workflow files are pack resources outside every `Schema`, so the whole tier-2
composed-surface batch, the 12 `migrate-*` steps and `sub-task`/`milestone-execution`/`implement-from-spec`
are **hash-free** (*read*).

What the dimension *does* cost is a **read-contract** move: `doc schema --format json` is pinned at
`contract-version 6` under an explicit **no-additive-carve-out** rule (`design/doc-read-surface.md:108`),
and the tier-0 headline (A1-D1) cannot be made discoverable without touching that projection. **That is
the wave's one-way door on this dimension**, and the window closes at the 1.0 pin.

---

## BLOCKING

### B1 · The pinned schema projection teaches the exact bogus address A1-D1 exploits — and the pack points the agent at it
*driven.* `jigc doc schema vision --format json` (contract-version 6) projects
`"set-slot": "vision:<slug>#thesis"` and `"set-field": "vision:<slug>#meta/grounded-in"`; the text arm
prints the same. There is **no** `singleton`, `placement`, `location` or canonical-identity key anywhere
in the projection, for **any** of the 16 doctypes (baseline-ledger §1.4, re-driven here). So the pinned
surface a driver is told to read **instructs** it to construct `vision:<anything>` — the token five write
doors then accept and `task finalize` silently drops (`axis-1.md` §3).

It is not hypothetical routing: the methodology pack's own `author-vision` step composes
`jigc doc schema vision` as the authority ("The `vision` schema is the authority on what you write into
it") **two lines after** it correctly says "The slug is fixed (`vision`)" (*driven*, composed
`form-vision`). The pack states the rule; the surface it hands off to contradicts it.

**Why it blocks:** fork 5 as written decides *where the write-door check lands*. If that is all it
decides, M52 fixes the door and **ships the 1.0 pin with a read contract that still teaches the wrong
address** — and after the pin the repair is a versioned-extension act, not an additive one.

**The robust fix has a shipped producer.** `engine::compose::projection_home_line`
(`crates/engine/src/compose.rs:1194-1215`) already answers the identity question for **all three**
home shapes, including the one nothing else handles:
```
placement            -> "the managed singleton at `<placement.file>`"
location + singleton -> "the managed singleton at `<location><ty>.md`"
location             -> "each instance a managed file at `<location><slug>.md`, its `<slug>` minted from `<id-from>`"
```
So the projection fix is *ask the existing producer*, not invent a fact. Cost: `contract-version` 6→7
(`doc-read-surface.md:108` — no additive carve-out), spendable only pre-pin.

### B2 · `singleton` and `placement` are two predicates for one rule, and they diverge on a shape the product fully supports
*driven, manufactured pack.* I built `adr.yaml` with `location: decisions/` **and** `singleton: true`
(`dev/jigc-rig fresh --pack-from-dev --schema adr …`). Every door accepted it end to end:

- `doc create adr --title "Use Sqlite"` → **blocking `write.title-ignored`**, "`adr` is a singleton — its
  `# H1` is supplied by the schema" — i.e. the *create* door reads `schema.singleton` and is correct;
- `doc create adr --title adr` → `adr:adr`; slots authored; `task finalize` → promoted
  **`docs/decisions/adr.md`**, exit 0;
- `jigc doc show adr:bogus` (committed) → generic `store.not-found — could not read \`adr:bogus\` at
  \`docs/decisions/bogus.md\`: No such file or directory`, route *"create the referenced doc"* — **a
  route that cannot be followed**, since `doc create adr --title bogus` is refused by the same
  `write.title-ignored` guard above. The M39 singleton-slug block ("`adr` is a singleton, so its only
  address is `adr:adr`") **never fires**.

Cause (*read*): the read guard keys on `schema.placement.is_some()`
(`crates/engine/src/store.rs:254`), and so does `cli::doc::is_singleton_type` (`crates/cli/src/doc.rs:6259-6262`)
— a function *named* `is_singleton_type` that never reads `Schema::singleton`. Meanwhile `Schema::fixed_title`
(`crates/engine/src/schema.rs:132`), the mint slug-fix (`crates/engine/src/state.rs:1226`), the pack-load
copy-in fence (`crates/cli/src/pack.rs:1153`) and `projection_home_line` all key on **`singleton`**.
`design/storage.md:216` declares the pair unfenced — *"nothing enforces placement ⇒ singleton at pack-load.
It needs no fence"* — and its rationale covers only that direction; the **converse** (a `location:`
singleton) is a supported, driven, shipping-capable shape the `placement`-keyed guards miss entirely.

**Why it blocks:** the baseline scopes fork 5 as *"Boundary `placement:`, subject 5 doctypes"*
(baseline-ledger §1.4). That is true of today's *corpus*, not of the *rule*. A fix keyed on `placement`
is complete over the shipped set and incomplete over its class — exactly M45's complete-fix lens. The
Settle must state which predicate is the class, in those words, and whichever it picks, the two names
should stop lying about each other.

### B3 · Three locked records say `milestone-record.base` is `set: on-transition`; the shipped schema says `on-create`
*read + driven.* `packs/methodology/schemas/milestone-record.yaml:62`:
`- { id: base, type: string, set: on-create }`. Driven: `jigc doc schema milestone-record --format json`
returns `"set": "on-create"` for `base` and `"on-transition"` for `status`.

Contradicted, all three naming `base` as the worked example of a machine-maintained **absolute**:
- `design/doc-read-surface.md` settability table → *"**not settable** (a machine-maintained absolute) …
  milestone-record `base` (`set: on-transition`)"*;
- `design/write-commands.md:84` → *"A machine-maintained absolute — `set: schema-version` … or
  `set: on-transition` (**the milestone base**) — is rejected"*;
- `DECISIONS.md:6397` → *"`set: on-transition` (milestone-record `base`/`status`/task fields only)"*.

Under the shipped `set:` kind split (`engine::schema::SetKind`, `schema.rs:790-825`;
`is_machine_maintained_absolute` true for `schema-version`/`on-transition`, **false for `on-create`**),
`base` is classified **author-overridable**. Nothing bad happens today only because `milestone-record` is
refused **whole**, one layer up, by the doctype-level `machine_maintained_guard` — i.e. the right outcome
for a reason none of the three records states.

**Why it blocks:** M52 touches the settability surface at LD-2 (below) and fork 6. Any fix that reasons
from the doc — *"`base` is an absolute, so …"* — reasons from a false premise, and any fix that widens the
per-leaf split without the doctype-whole guard opens a forged base-SHA pin at exit 0. Settle picks one:
correct the three records to `on-create`, or bump the schema (which **is** a hash move and needs the
words).

### B4 · A missing snapshot silently narrows the corpus walk — one layer above the fold's fail-closed block
*read.* `candidate_docs` (`crates/cli/src/migrate_corpus.rs:2104`):
```rust
let prior_locations: BTreeSet<String> = (1..dt.version)
    .filter_map(|k| crate::pack::load_prior_schema(pack, &dt.ty, k).ok())   // <- .ok()
    .filter_map(|prior| prior.location)
    .collect();
```
and `pack::prior_doctype_schemas` (`crates/cli/src/pack.rs:211`) is best-effort by declaration
(*"a doctype whose snapshot is absent or malformed simply contributes no prior shape … never an error"*).
The **fold** is fail-closed — `migrate-corpus.missing-snapshot` (`migrate_corpus.rs:1511`) — but it only
ever sees docs the **walk** already found. `design/corpus-migration.md:90` promises *"a **missing**
snapshot **blocks** that doc with a route (**never a silent `already-current`**)"*; the enumeration's
`.ok()` makes that promise unkeepable for a home the missing snapshot was the only record of.

Today this cannot fire in the shipped packs: `crates/cli/tests/snapshot_store_two_snapshots.rs:168-213`
(`every_version_below_current_ships_a_snapshot_and_nothing_above_it_does`) asserts the store equals
`1..current` **in both directions**, iterating both manifests rather than a hand list, and resolves each
out of the **embedded** pack. (*Verified by reading what the test asserts* — this is the `(f)` answer:
**yes, fenced, complete, both directions.**) But the fence's subject is the two **shipped** packs' trees.
A **project pack** under M49's PB-1 shipping a doctype at `schema-version > 1` is inside
`prior_doctype_schemas` and outside the fence.

**Why it blocks fork 4:** the fix widens this enumeration to three more `(from-home × to-home)` cells.
Widening a best-effort enumeration multiplies the silent-narrowing surface by the same factor. The
Settle must decide whether the walk's snapshot read becomes fail-closed **in the same increment**, or the
new cells inherit the old silence.

---

## MAJOR

### M1 · `doc schema` and `doc show` disagree on `milestone-record.base`, and only one side is declared
*driven.* Same corpus, same commit:
```
$ jigc doc schema milestone-record --format json     ->  "id":"base","type":"string", … "set":"on-create"
$ jigc doc show milestone-record:probe-wave --format json
      "fields": { "base": { "sha": "a893328…", "short": "a893328" }, … }
$ cat docs/milestone-records/probe-wave.md
      base: a89332806fd67def5381101ae8f15852baddf750 a893328     <- one space-joined string field
```
The `doc show` side **is** declared, twice (`design/doc-read-surface.md:66` — *"The one **compound** field
in the pinned surface is the milestone-record's `base` pin"*; `design/team-ready-state.md:195`), and the
producer is a hardcoded doctype+field special case (`crates/cli/src/doc.rs:5589-5611`,
`COMPOUND_BASE_DOCTYPE`/`COMPOUND_BASE_FIELD`). The **`doc schema` side has no way to say it**: its
declared key set (`doc-read-surface.md:100-105`) carries `type`, `of`, `to`, `required`,
`author-required`, `default`, `set`, `section` and the write-verb addresses — nothing that can mark a
leaf as compound-on-read. A driver reading the schema contract writes a string parser and gets an object.

Fix shapes: (a) **prose-only** — state in `doc schema`'s section that the projection reports the
*declared* type and the one compound is declared on the `doc show` side (no version moves); (b) **robust**
— project the fact, `contract-version` 6→7, poolable with B1's key. Note the baseline's correction stands:
the envelope version here is **not** `engine::result::SCHEMA_VERSION` (baseline-contracts §3), it is
`doc schema`'s own `contract-version`.

### M2 · The exit-4 review-hold promise is carried by 12 workflows, is **false in the cell the off-verb door creates**, and **no fence protects it**
*driven + read.* `jigc start --workflow migrate-adr "off verb migrate"` → exit 0; the task dir holds
`base.json docs intent staged-snapshot.json workflow` and **no `source-path`** (`ls` → No such file or
directory, driven). `crates/cli/src/task.rs:2026` is `let is_migration = source_seam.exists();` and the
hold at `:2236` is `if is_migration && !approve`. So the hold is **inert by construction** on every
off-verb mint, while the composed body carries, verbatim:

> *"a plain finalize commits NOTHING — it renders the foreign source against the canonical rewrite and
> holds (exit 4) … re-run the same finalize with `--approve` … approval is the one destructive gate."*

(`crates/cli/pack/steps/migration-finalize.yaml:4-9` and its byte-twin
`packs/methodology/steps/migration-finalize.yaml`.) baseline-surfaces §2.2 drove the consequence:
plain `task finalize` **committed and promoted at exit 0**.

**Adjudication the Settle needs, stated: the binary is right and the text is right — the *door* is wrong.**
Per `design/auto-migration.md:21`, the review gate exists because a *foreign source* was rewritten; with
no source there is nothing for a fidelity diff to compare, so an ordinary promote is correct. The text is
true in the verb-routed cell the workflow's own `suppressed.reason` names. **So the fix belongs at the
compose door (refuse or discriminate an off-verb `migrate-*`/`sub-task` compose), not in the 12 step
files** — rewording the steps to hedge the exit-4 promise would make them *less* true in their real cell.

**Fence coverage, measured (this is the `(c)` answer):**
| edit to a `migrate-*` step | reddens pack-load? |
|---|---|
| rewrite/delete the exit-4 sentence | **No.** `migration-finalize` declares `finalize.promote-clobber`, whose required tokens are `--approve`, `retire`, `fidelity diff` (`crates/cli/src/pack.rs:1423-1426`); all three survive elsewhere in the same body (lines 8, 9, 20). No token names exit 4, "commits NOTHING", or the hold. |
| delete/alter the `{{ source }}` line | **No fence at all.** `{{source}}` is **empty-not-finding** by design (`crates/engine/src/compose.rs:685-694`, *"An unfed seam … emits an empty line — empty-not-finding, mirroring an unbound role"*). |
| delete the `jigc doc show … --task` read-back | **Yes** — `read.staged-read-back`, tokens `jigc doc show` + `--task` (`pack.rs:1439`). |
| delete a `{{schema:<T>}}` ref on a **singleton** T | **Yes** — `assert_singleton_copy_in_stated` (`pack.rs:1090-1200`), both directions; it fired live in my rig build. |
| any ambush-class code | `migrate.review-pending` is **not** in `AMBUSH_CONTRACTS` (`pack.rs:823-858` — four rows: `finalize.promote-clobber`, `finalize.nothing-staged`, `finalize.carried-staged`, `setup.dirty-install-path` `Exempt`). |

**The precedent the Settle should hear:** `{{schema:}}`'s own doc-comment, eight lines below `{{source}}`'s,
says the two kinds' unfed stances are **deliberately opposite** — *"the `{{source}}` mold covers the feed
mechanics only; the unfedness stance is the opposite: a doctype absent from the fed map … **BLOCKS**
(`workflow-refs.schema-ref-resolves`), never renders silently empty"*. The robust fix for the A6-3 class
(12 of its 14 renders) already has a shipped sibling one comment away.

### M3 · `ValueRemapped` on an `id-from` enum is an **identity** migration, and no transform kind re-mints an id
*driven (corpus shape) + read.* The rc.15 corpus makes it concrete: an `id-from: category` item renders
`### changed  {#changed}` (driven, `committed-singletons` `CHANGELOG.md:9`) — the enum value **is** the
heading text **and** the `{#id}` anchor. `changelog.yaml:51`/`:63` declare `id-from: category` at two loci;
`:45` declares the `of:` set.

So D-3's repair is not a one-arm swap. `apply_value_remap` (`crates/engine/src/transform.rs:364-371`)
rewrites values; it has **no `id_from` awareness** (`grep id_from crates/engine/src/transform.rs` → zero
hits, *driven grep*). Remapping an `id-from` enum member re-mints the item id — which is:
- the act `retitle-item` **refuses unconditionally** for an enum `id-from` (M40, restated at
  `design/doc-read-surface.md` → *"A member change is an identity change"*), and
- the act both manifests' headers declare un-migratable: *"A slug-rule change CANNOT be migrated after the
  fact (**no transform kind re-mints an id**)"* (`crates/cli/pack/config/schema-manifest.yaml`, the
  `slug-rule` block).

**Why it's major:** the obvious D-3 fix ("stop dead-ending at `migrate-corpus.prose-needed`, take the
`fold-refused` arm") ships a route — *"declare the missing old→new value mapping … in `authored_remap`"* —
that is **followable to a transform that cannot legally land**, which is the same PT-1 shape M46 closed at
`migrate-corpus`. The Settle's real fork is: build id re-minting for this kind, or declare the cell
**unmigratable** and make the refusal say so. Either answer is in-scope (a route repair), the first is not.
Rider (*source read*, baseline §2.3): `authored_remap` (`migrate_corpus.rs:1194-1202`) is a hardcoded match
in jigc's own source with exactly one entry, so under M49's *fork-not-subclass* PB-1 bound a **project
pack**'s enum rename has no declarable map and the route points into jigc's source tree.

### M4 · `engine::ingest::adopt` re-derives an identity one producer already computed correctly (C-1)
*read.* `crates/cli/src/ingest.rs:555-567` `home_identity` handles `placement` (→ `<ty>:<ty>`) and
`location` (→ `<ty>:<slug>`, `is_slug`-gated), and is what M51's `ingest.unaddressable-identity` keys on.
`crates/engine/src/ingest.rs:226-228` then re-derives from the filename stem two lines later
(`let slug = rel_path.rsplit('/')… .strip_suffix(".md")`). The two disagree iff
`stem(placement.file) != ty` — **2 of 5** placement doctypes (`vision` → `VISION`, `changelog` →
`CHANGELOG`), of which only `vision` carries a `type: ref` (baseline-freeze §1.5, 5 ref fields total).
Driven by the baseline: a plain `ingest` on a healthy corpus lands **both** `vision:VISION` and
`vision:vision` in `edges.json`.

**Doctypes-dimension note the Settle needs:** *the canonical identity of a placement instance is stated
nowhere a machine reads.* It is prose at `design/storage.md:184`; it is **absent** from
`doc schema`'s projection; and `doc list --format json` (*driven*) is the only surface that prints it
(`"id": "vision:vision", "path": "VISION.md"`) — a *listing*, not a *rule*. Fixing `adopt` to take the
existing producer is the correct axis-complete fix and moves **no hash and no contract version**; the
statement half is B1's.

### M5 · The placement family, enumerated — and the one door-set that treats the identity differently
*read (schemas) + driven (`doc list`, `unmanage`/`ingest` via baseline).*

| doctype | pack | `placement.file` | `display-title` | `stem == ty` |
|---|---|---|---|---|
| `changelog` | dev | `CHANGELOG.md` | `Changelog` | **no** (`CHANGELOG`) |
| `vision` | methodology | `VISION.md` | `Vision` | **no** (`VISION`) |
| `roadmap` | methodology | `docs/roadmap.md` | `Roadmap` | yes |
| `decisions-log` | methodology | `docs/decisions-log.md` | `Decisions Log` | yes |
| `deferral-ledger` | methodology | `docs/deferral-ledger.md` | `Deferral Ledger` | yes |

All five carry `singleton: true`; **no other shipped doctype does** (`grep -rn "^singleton:"` over both
schema dirs → exactly those 5, *driven grep*). So `singleton ⇔ placement` holds **in the shipped corpus
only** — B2 is the shape that breaks it. `display-title` **is inside the hash** (`manifest.rs:84`), so any
touch of it is a bump; nothing in M52 needs one.

Door split, per the shipped predicates: `doc show`/`doc create`-read-side/`is_singleton_type` key on
`placement`; `create`'s title guard, `fixed_title`, the mint slug-fix, `projection_home_line` and the
pack-load copy-in fence key on `singleton`; `ingest` splits across **two producers** (M4);
`rename --slug` refuses a placement reslug (`design/storage.md:211`) — and baseline-tokens §4 defect 1/2
drove `jigc rename vision:alpha --to Phantom --slug alpha` landing at exit 0 anyway, which is the same
bogus-`<slug>`-head token B1 makes discoverable.

---

## MINOR

### m1 · `doctype-authoring.md` reads as if a singleton is always a placement doctype
`implementation/doctype-authoring.md:10` — *"**Exactly one home:** `location: <dir>/` (docs-root applies)
· `placement: {file: …}` + `singleton: true` (literal path, bypasses docs-root) · neither = transient"*.
`singleton: true` appears only on the placement branch. `projection_home_line` proves the fourth cell
(`location` + `singleton`) is real and supported. Candidate revise alongside B2; no hash, no version.

### m2 · The composed `{{ schema:<ty> }}` home line is the one seam that already tells the truth
*driven.* The off-verb `migrate-adr` composition rendered *"The `adr` schema — each instance a managed
file at `docs/decisions/<slug>.md`, its `<slug>` minted from `title`."* The identity rule reaches the
agent through **compose** and not through **`doc schema`**, which is the split B1 is about. Worth
recording as the positive control for B1's fix shape.

### m3 · `conformance.section-renamed`'s route exemption reason is falsified by jigc's own producer
Carried forward from baseline-freeze §4 O-1 (declared, so a record-truth item, not a floor breach). In
this dimension because the exemption's stated reason — *"there is no address to route at"* — is a claim
about **doctype addressability**, and `ingest` prints a followable `Route::human` for the same code at the
same commit.

---

## The three cross-cutting hunts

**(a) cheap-vs-robust — `fork · cheap-vs-robust`, and it is a one-way door.**
Two places where the cheap cut is a prose reword and the robust cut is a projection key:
1. **B1/M4** — cheap: fix the five write doors, leave `doc schema` projecting `vision:<slug>`, and add a
   sentence somewhere. Robust: project the identity/home fact from `projection_home_line`'s answer,
   `contract-version` 6→7. **Tell (1) fires — one-way door at the pin:** `doc-read-surface.md:108`
   forbids an additive carve-out and `:90` closes the window *at the 1.0 pin*, after which the same
   addition is a versioned-extension act for the life of `1.x`. M52 is chartered as the wave before that
   call. **Tell (2) fires too:** `doc schema` is a *declared complete* schema-read surface and the hole is
   known — a driver cannot learn from it that `vision:alpha` is not an address.
2. **M1** — cheap: declare the compound in `doc schema`'s prose. Robust: project it. Same door, and it
   pools with (1) into **one** `contract-version` move rather than two.
Counter-weight, stated honestly: the wave's boundary is *fixes + understandability, no new capability*,
and a projection key is arguably capability. The rebuttal is the charter's own razor leg 0 — *a hole in a
declared surface* — plus the fact that adding the key **requires no new producer**: the answer already
ships at `compose.rs:1194`.
A third, opposite call: **M2's cheap cut is the wrong one and the robust cut is smaller.** Rewording 12
step files to hedge the exit-4 promise is more edits *and* makes them less true in their real cell; the
door-side discriminator is one place.

**(b) foreclosed-by-doc — one candidate revise, one genuine constraint, no wall.**
- `design/storage.md:216` (`fork · foreclosed-by-doc`): *"nothing enforces placement ⇒ singleton at
  pack-load. **It needs no fence**, because placement's identity model **is** the singleton one … so a
  non-singleton placement schema is malformed rather than a supported shape this row must cover."* The
  rationale is sound **for the direction it states** and says nothing about the converse; the shape that
  actually leaks (B2 — `location:` + `singleton: true`) is *supported*, not malformed, and every
  `placement`-keyed guard misses it. Candidate **revise** (the rationale still holds; its scope is
  reopenable), not a conflict. Flagging it for the human — the doc reads as permission to key fork 5's
  fix on `placement`.
- `doctype-authoring.md:10` — same shape, one altitude down (m1).
- The **freeze rule** forecloses nothing here: no M52 fix needs a hash (§0).
- **M49's *fork, not subclass*** genuinely constrains M3 (a project pack's enum rename has no declarable
  `authored_remap` entry). Not a blocker for M52's own cells; it bounds how wide the D-3 fix can claim to be.
- `design/methodology-docs.md:42`'s universe rule blocks nothing in this dimension — it was re-tested and
  **kept** at M50 and is now mechanically fenced (`multi-pack.md` → the ref-target fence).

**(c) prior-art-reconciled — four disagreements between locked statements, one of them new.**
1. **`blocking · prior-art-contradiction` — `milestone-record.base`'s `set:` kind.** `doc-read-surface.md`
   settability table · `write-commands.md:84` · `DECISIONS.md:6397` all say **`on-transition`**; the
   shipped schema says **`on-create`** (`milestone-record.yaml:62`), driven through `doc schema`. The
   schema settled it — the record did not follow. (B3.)
2. **The two pinned read contracts on `base`'s *type*.** `doc-read-surface.md:66` + `team-ready-state.md:195`
   declare the compound `{sha, short}` on the **`doc show`** side; nothing declares it on the **`doc schema`**
   side, which emits `"type": "string"`. Not a contradiction between two docs — a fact settled in one home
   and absent from its sibling. (M1.)
3. **The corpus walk's key** (already in baseline-ledger §1.5, restated for reconciliation completeness):
   `corpus-migration.md:68` and `:299` say the walk keys on **`from`**; `candidate_docs` keys on **`to`**.
   `storage.md:208`'s census row is **not** falsified.
4. **New here — `corpus-migration.md:90`'s *"never a silent `already-current`"* vs `candidate_docs`' `.ok()`.**
   The fold is fail-closed (`migrate-corpus.missing-snapshot`); the enumeration that feeds it is
   best-effort by declaration (`pack.rs:211`). The promise is made at the doc level and kept at the fold
   level only. (B4.)
Greps run for the hunt (all *driven*): `placement`, `singleton`, `display-title`, `canonical_path`,
`snapshot`, `prior_doctype_schemas`, `contract-version`, `on-transition`, `^singleton:` across `design/`,
`implementation/`, both packs' `schemas/`, `crates/engine/src`, `crates/cli/src`. No further pairwise
disagreement surfaced.

---

## Answers to the brief's direct questions

- **(a) which fixes touch a schema-bearing surface, and does any need a hash to move?** None needs a hash
  (§0). Step text sits in no hash (`manifest.rs:77-89`, exhaustive destructure). `slug-rule-version` is
  untouched — no fix changes `engine::slug::slugify`. The **pack-load fences** that a step rewrite can
  redden are enumerated in M2's table; the exit-4 sentence is **not** among them.
- **"closing D-2 needs no bump"** — **holds, verified independently.** A `from`-keyed walk needs **no
  schema to declare anything new**: the whole prior `Schema` (`location:` *and* `placement:`) is in
  `schema-snapshots/<ty>.v<k>.yaml`, `pack::prior_doctype_schemas` already loads every shape `1..current`,
  and the store is fenced complete both directions
  (`crates/cli/tests/snapshot_store_two_snapshots.rs:168`). Two riders stand: `DoctypeMigration`
  (`migrate_corpus.rs:68-81`) carries `docs_root` only, so a prior `placement.file` with a leading
  component needs `placement-root` threaded; and B4's `.ok()`.
- **(b) what a driver would need from `doc schema` to construct only valid addresses:** per doctype, one
  of — *singleton, canonical address `<ty>:<ty>`* / *singleton at `<location><ty>.md>`* / *multi-instance,
  `<slug>` minted from `<id-from>`*. All three already exist as strings at `compose.rs:1194-1215`.
  **Additive under M51 D6?** No — `doc-read-surface.md:108` states *no additive carve-out* for this
  projection specifically, so it is a **`contract-version` 6→7** move, permitted pre-pin and only pre-pin
  (`:90`).
- **(e) `milestone-record.base`:** the schema's truth is `type: string` (one space-joined value on disk,
  driven). `doc schema` is faithful; `doc show`'s `{sha, short}` is a declared, hardcoded projection
  (`doc.rs:5589-5611`). Fixing the disagreement touches **no hash**; the robust half touches
  `contract-version`.
- **(f) the snapshot store:** fenced complete for both shipped packs, both directions, iterating the
  manifests. On a corpus whose snapshot is missing: the **fold** blocks
  (`migrate-corpus.missing-snapshot`), the **walk** silently omits that home (B4).

## Honest bounds

- No cargo, no mutation, one binary (`1.0.0-rc.15`), macOS only.
- B2 was driven on a **manufactured** pack whose manifest the rig dropped (`--pack-from-dev` without
  `--repin`) — because with the manifest kept, `assert_singleton_copy_in_stated` correctly reddened
  pack-load first (itself a driven datum, quoted in M2's table). So B2's shape is proven **legal and
  fully functional**; what is *not* proven is that a manifest-shipping pack could ship it without first
  satisfying that fence.
- M3's transform claim is a **read** of `transform.rs:364-371` plus a zero-hit grep, not a driven remap of
  an `id-from` enum; baseline-freeze §2.3 drove the observable symptom (`prose-needed`, re-run identical).
- I did not drive `unmanage`/`ingest` edge duplication myself (M4) — carried from baseline-freeze §4 L-2/L-3,
  which drove it.
- `deferral-ledger` was never materialised in any rig state; its placement row in M5 is read from the
  schema.
