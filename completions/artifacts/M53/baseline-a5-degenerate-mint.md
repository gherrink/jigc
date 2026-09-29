# Baseline ledger fragment — row `(5, DEFECT 1)`, M53 Scope

Verified at **HEAD `155054cc`**, driven on the **installed release `jigc 1.0.0-rc.16`** (`~/.local/bin/jigc`). A map, not gospel. No edits made.

---

## 1 · The row itself — CONFIRMED, and larger than the review states

**Status: latent defect / shape-limited guard — DRIVEN.** The review's two doors reproduce exactly. Three widenings the review and the charter do not carry.

### 1a · The two named doors (DRIVEN, rig `fresh`)

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ jigc milestone create ""
exit=0
minted milestone:milestone (shared base 7910c6c)
record: docs/milestone-records/milestone.md   — the committed record this milestone's state lives in
record commit: 9411a34   — the record on its own; anything else you had staged stayed staged
next: `jigc milestone add-task milestone "<intent>"`   — add the milestone's first sub-task

$ jigc milestone add-task milestone ""
exit=0
added task:task to milestone:milestone
record commit: 42cd3da   — task:task on the milestone record, committed on its own; …

$ git log --stat -2
42cd3da chore(milestone): record task:task on milestone:milestone
 docs/milestone-records/milestone.md | 7 +++++++
9411a34 chore(milestone): open record for milestone:milestone
 docs/milestone-records/milestone.md | 9 +++++++++

$ jigc doc list
milestone-record:milestone  docs/milestone-records/milestone.md  managed
```

### 1b · Widening 1 — the degenerate cell is **not** punctuation-only; it is every non-Latin-script title and every stopword-only title (DRIVEN)

One `milestone create` per fresh rig, nine titles, **all nine** landed `docs/milestone-records/milestone.md` at `milestone:milestone`, H1 `# milestone`, exit 0, committed:

| title given | minted id | record committed |
|---|---|---|
| `""` · `"   "` · `"!!!"` · `"..."` · `"-"` | `milestone` | yes |
| `"日本語"` | `milestone` | yes |
| `"🎉🎉"` | `milestone` | yes |
| `"the of a"` · `"the"` · `"A"` | `milestone` | yes |

`EDGE_STOPWORDS` (`crates/engine/src/slug.rs:159`) = `a an the of to in on at by for`; `slugify` step 4 strips every byte outside `[a-z0-9-]`, so **any title in Cyrillic, Greek, CJK, Hebrew, Arabic, or emoji slugs to nothing.** The review's `"!!!"` framing reads as an adversarial cell; it is an ordinary one for a non-English project. This matters for the fix's *message and route*, not just for the guard's placement.

### 1c · Widening 2 — a **third** committing mint door has the identical fallback: `jigc milestone add-from-spec` (DRIVEN)

Not named in the review row, not named in the charter, not a `MINT_DOORS` site of its own (it reaches the same `engine::milestone::add_task`). A committed spec with one ordinary criterion and one whose title slugs to nothing:

```
# human hand-edits the committed spec through git (the storage invariant), then:
$ jigc --format json milestone add-from-spec probe-milestone spec:probe-spec
exit=0
{ "text": "seeded 2 sub-task(s) into milestone:probe-milestone from spec:probe-spec: real-one, task
           record commit: 0d7ed34  … task:real-one …
           record commit: 0ef0566  … task:task …" }

$ cat docs/milestone-records/probe-milestone.md
### real-one  {#real-one}
- intent: Real one
### task  {#task}
- intent: 日本語          <- degenerate id, committed, two record commits landed
```

**If M53 guards `run_create` and `run_add_task` at the CLI layer only, this door survives.** The guard must sit at `engine::milestone::add_task` (`crates/engine/src/milestone.rs:348`) — or at both CLI doors *plus* `run_add_from_spec` — or the re-review finds it as the next tier-1 row.

*(Sub-case: a spec with **two** degenerate criteria aborts the whole pass on `milestone.sub-task-collision — sub-task \`task\` is already in milestone \`x\`` with route *"add the sub-task with a distinct intent"* — the criteria **were** distinct. Unwind is clean, record and `.jigc/tasks` both empty. DRIVEN.)*

### 1d · Widening 3 — `add-task ""` commits a record the tool's own sweep calls **corrupt**, with a dead-end route (DRIVEN)

```
$ jigc validate                                        # bare, exit captured directly
exit=0
blocking (gates at finalize) · schema-conformance.field-value-conformant —
  `docs/milestone-records/milestone.md`: field `intent` in item `tasks/task`: "intent" must not be empty
  at: milestone-record:milestone#tasks/task/intent
  route: corrupt — `docs/milestone-records/milestone.md` is at the current schema-version 3
         but does not conform; review it by hand
```

So the empty-intent cell is not only a fabricated identity — it **commits a non-conformant milestone record that gates at `milestone finalize`**, and the only route offered is a hand edit. The `"!!!"` cell by contrast commits a *conformant* record (`- intent: !!!`) and `validate` reports *no findings — the committed store validates clean*. **Two different consequences behind one fallback** (DRIVEN both).

### 1e · Corrections to the review's own wording (a source read reaching a cleaner conclusion than the binary supports)

- The review's claim (b) — *"the committed record's H1 is `# milestone`, a title the caller never typed"* — is true but mis-mechanized. Driven: `jigc milestone create "Real Milestone"` also lands H1 `# real-milestone`. **The milestone-record H1 is always the slug, never the title.** The lie is the fabricated *id*; the H1 is downstream of it.
- The review's claim (c) route is followable-as-written: the second unslugable create refuses with `milestone.record-exists … route: continue it with \`jigc milestone add-task milestone "<intent>"\`, or create this one under a different title`. The genuine harm is that a **legitimate** later title is refused — driven, `jigc milestone create "The milestone"` exits 1 against the squatted id.

### 1f · What later doors do with the degenerate id (DRIVEN)

`milestone list-tasks milestone` → exit 0 · `doc show milestone-record:milestone --format json` → exit 0 · `task list` → exit 0, lists `task [sub-task]` · `milestone discard milestone` → exit 0, clean teardown · `milestone finalize` → blocks on `milestone.zero-contribution`, unrelated. **No further harm downstream; the harm is the committed identity itself.** There is no `milestone list` verb (exit 2, clap).

---

## 2 · The code, file:line

| thing | location | what it is |
|---|---|---|
| `mint_id` (milestone) | `crates/engine/src/milestone.rs:2470` | `let slug = slugify(title); if slug.is_empty() { slugify("milestone") } else { slug }` — `pub` for the CLI's pre-mint id-is-taken guard |
| `mint_sub_id` | `crates/engine/src/milestone.rs:727` | same shape, fallback `SUB_TASK_TYPE = "task"` (`:722`) |
| the two callers | `mint_milestone` `:282` (`let id = mint_id(title)` `:287`) · `add_task` `:348` (`let sub_id = mint_sub_id(intent)` `:371`) · `add_from_spec` `:438` (`:481`) | |
| `mint_id` (task) | `crates/engine/src/state.rs:1315` | `mint_id(intent, type_name)`, same fallback — reached by `mint_task` `:985`/`:995` |

**The doc-comment the charter cites**, `crates/engine/src/state.rs:2041–2045` (verbatim):

```rust
/// The empty-title block for a non-singleton `create` whose title slugs to nothing
/// (`--title ""`, `--title "!!!"`): left unguarded it mints a degenerate `<ty>:<ty>`.
/// Mirrors `rename`'s slug-derivation guard (`crates/cli/src/rename.rs`); routes to
/// supply a non-empty `--title` (its slug becomes the doc id). Keys at the
/// [bare doctype id](doctype_scoped_location).
fn empty_title_finding(type_name: &str) -> Finding {
```

**The guard it names** — `crates/engine/src/state.rs:1590`, inside `mint_instance` (`:1576`):

```rust
if !schema.singleton && slug_override.is_none() && crate::slug::slugify(id_source).is_empty() {
    return Err(empty_title_finding(type_name));
}
```

Returns `Finding::graded(Blocking, "create.empty-title", "`jigc doc create {type_name}` needs a title that yields an id, but the given title is empty or slugs to nothing", Some(Location::addressed(type_name,1,1)), Some("re-run with a non-empty `--title` (its slug becomes the doc id)"))`. **Parameterised on `type_name` only** — the verb name and the `--title` flag are hardcoded literals.

### `work-unit.malformed-id` today

- Constant: `crates/cli/src/task.rs:1467` `MALFORMED_WORK_UNIT_ID`.
- Producer — **exactly one**: `crates/cli/src/task.rs:1492` `reject_malformed_work_unit_id(id) -> Result<()>`. `if engine::slug::is_slug(id) { return Ok(()) }` else `Err(render::finding_error(&Finding::graded(Blocking, MALFORMED_WORK_UNIT_ID, format!("{id:?} is not a valid work-unit id"), **None**, Some(Route::human(WORK_UNIT_ID_GRAMMAR)))))`. `location: None` ⇒ the stable key is `(code, null)`. Message is parameterised on the token; route is the **fixed** constant `WORK_UNIT_ID_GRAMMAR` (`task.rs:1462`) = *"use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)"*.
- How the 25 doors reach it: **not per door.** `WORK_UNIT_ID_DOORS` (`crates/cli/src/cli.rs:2589`, 25 rows) is the *door* side; the guard sits at **five resolve seams** below clap (its own doc-comment says so at `task.rs:1468–1490`), reached because each door's id resolves through one of them.
- Its doc-comment states the predicate choice explicitly: *"`slugify(x) == x` would be the wrong test: the mint-time word cap and edge-stopword drop make it reject ids the mint itself produced."*

### Are the two mint doors rows in `WORK_UNIT_ID_DOORS`? — **No, and they cannot become rows.**

`WORK_UNIT_ID_DOORS` is **derived** from the clap tree by `work_unit_id_arg_ids()` (`cli.rs:2485`) — the args classified `ArgToken::WorkUnitId` (`id`, `task`, `milestone_id`) — and fenced **⇔** by `cli_parse::every_work_unit_id_door_is_registered`. `milestone create` has **no** work-unit-id argument at all (positional `<TITLE>` only — verified against `jigc milestone create --help`, no `--title`, no `--slug`). `milestone add-task` and `add-from-spec` *are* rows, but for their `<MILESTONE_ID>` argument — the id they **consume**, not the prose they **derive one from**. Adding a title-bearing row would falsify the ⇔ fence.

**The distinction that settles it:** the 25 doors ask *is this caller token an id?* The mint doors ask *does this caller prose yield one?* Applying `reject_malformed_work_unit_id` to the derived id is **inert** — `mint_id("") == "milestone"`, and `is_slug("milestone")` is `true`. Applying it to the *title* renders `"日本語" is not a valid work-unit id` with the route *"use lowercase letters, digits, and single hyphens"* — advice that, followed, produces a committed H1 of `# use-mysql`-shaped kebab where jigc's whole title convention is free prose (`Use MySQL`). **That is a law-1 lie at a mint door.**

---

## 3 · The halt question — answered explicitly

**The charter's prescribed fix as literally written (*"refusing … with the shipped `work-unit.malformed-id` identity … applied at the two mint doors it missed"*) does not fit, and is not what M50 closed.** It is not a new-mechanism halt either — **the right piece already ships.**

**The right seam is a title predicate, not a guard on the derived id** (§2 above proves the derived-id guard is inert).

**The right identity already exists with two producers: `write.unslugable-title`.** It is the product's shipped code for *this exact condition* — a caller title that yields no id — and it is already a **multi-producer code with per-producer message and route**, which is precisely what a third producer needs:

| producer | file:line | message (driven, verbatim) | route (driven, verbatim) |
|---|---|---|---|
| engine write path (`doc add-item`, `doc author` items) | `write.rs:2665`, `:3239` → text at `:7687`, route at `:6300` | `write rejected: title "!!!" has no slug-able content for an item id` | `re-run the write with a title carrying at least one word character (the item id is slugged from it)` |
| `jigc rename` | `crates/cli/src/rename.rs:201` (`RefusalKind::UnslugableTitle`), repair at `:225` | ``blocking · write.unslugable-title — `--to "!!!"` slugs to nothing — a rename derives the new id from the title, and this one carries no slug-able content`` | `re-run with a title carrying at least one word character, or name the id yourself` |

Precedent is on the record: `implementation/roadmap.md:1609` (M13 completion audit, LOW) — *"`add_item` accepted an unslugable `--title` (`""`/`"###"`), minting an empty `{#}` anchor → now rejected with a routed `write.unslugable-title` finding."* Identical class, identical resolution.

### Size, and where the guard sits relative to the first write

| # | what | file:line | size |
|---|---|---|---|
| 1 | a `slugify(title).is_empty()` reject in `run_create`, at the **top** — before `discover_repo_root`, `shipped_schemas`, `guard_record_free` (`:645`), `gitignore::ensure`, `read_head`, `mint_milestone` | `crates/cli/src/milestone.rs:629` | ~6 lines. **Refuses before the first write of any kind**, so M47's record-only pre-image machinery is untouched and needs nothing. |
| 2 | the same reject for the sub-task intent, at **`engine::milestone::add_task`** before `mint_sub_id` (`:371`) — this covers `run_add_task` **and** `run_add_from_spec` in one site | `crates/engine/src/milestone.rs:348–371` | ~5 lines. Sits after the unknown-milestone reject, before `read_base_pin`, before `mint_task`, before the CLI's `append_and_commit_record`. **No write precedes it.** |
| 3 | the engine has no `Route`-carrying producer for this today at the milestone module; `write.unslugable-title`'s engine text is `GenerateError`-shaped (`write.rs:7687`) and its route comes from `write_route`'s per-code map (`:6300`), whose text says *"the item id is slugged from it"* | — | a **third producer function** is owed, ~12 lines, in the crate that owns the condition — the same shape `rename.rs` already is. **This is a new producer, not a new mechanism**: no new code, no new family, no new registry, no schema-hash movement. |

**Three route texts are wrong for these doors if copied rather than written:**
- `create.empty-title`'s says *"re-run with a non-empty `--title`"* — `milestone create` has no `--title` (positional `<TITLE>`). Driven: `doc author`'s degenerate `title:` already prints that misdirected route today (tier-3 sibling, noted not claimed as my row).
- the engine `write.unslugable-title` route says *"the item id is slugged from it"* — wrong noun at a milestone door.
- `work-unit.malformed-id`'s route is the **id grammar**, which as a title instruction is a law-1 lie (§2).

**So: the honest answer to the halt question is "no new mechanism, but the charter's named identity is the wrong one."** That is a **Settle fork for the human**, not a build decision:
> *Fork:* does the mint-door refusal take `write.unslugable-title` (shipped, two producers, exact condition, per-producer route — but a `write.*` code at a work-unit mint door), or `work-unit.malformed-id` (the charter's word, the right *family*, but a predicate that is inert on the derived id and a route that lies when pointed at a title)? **Recommendation: `write.unslugable-title`**, with a third producer writing this door's own sentence — because it is the code the product already raises for *"this title yields no id"*, at two doors, and the alternative needs either a new predicate or a false route.
> *Second fork, smaller:* `jigc start --workflow X ""` refuses with a **bare `anyhow`** (`crates/cli/src/start.rs:92`), code-less and route-less — driven: `{"error": "intent must contain at least one letter or digit (got \"!!!\")"}`. If M53 gives two doors a coded identity and leaves the fourth mint door on a bail, the class ships with two wire shapes. Converging it is ~4 lines at one site and is the shape M49/M51 used for exactly this ("leaving those would have shipped two wire shapes for one code").

*(Aside, driven: that bail's text is itself false for non-Latin input — `intent must contain at least one letter or digit (got "日本語")` and `(got "Кэш")`. Tier-3; flagged, not claimed.)*

---

## 4 · The sibling cells — every mint that derives an identity from caller prose

Inputs driven at each door: `""`, `"   "`, `"!!!"`, `"日本語"`, stopword-only (`"the"` / `"the of a"`), and where a `--slug` exists, `--slug ""` / `--slug "!!!"`.

| door | input | outcome | verdict | |
|---|---|---|---|---|
| `jigc milestone create "<title>"` | all six | **degenerate `milestone:milestone` COMMITTED**, exit 0 | **UNSAFE** | DRIVEN |
| `jigc milestone add-task <m> "<intent>"` | all six | **degenerate `task:task` COMMITTED**, exit 0; `""` also commits a record `validate` calls *corrupt* | **UNSAFE** | DRIVEN |
| `jigc milestone add-from-spec <m> <spec>` | criterion title slugging to nothing | **degenerate `task:task` COMMITTED**, exit 0, per-criterion record commit | **UNSAFE — not in the charter** | DRIVEN |
| `jigc start --workflow X "<intent>"` | all six | REFUSED exit 1, **bare `anyhow`, no code, no route** (`start.rs:92`) | SAFE-for-identity, route-floor gap | DRIVEN |
| `jigc start "<intent>"` (default workflow) | all six | exit 0, mints **nothing** — the shipped default is the `creates-task: false` router | SAFE | DRIVEN |
| `jigc start … --slug ""` / `--slug "!!!"` | — | REFUSED, `is_slug` at `start.rs` boundary | SAFE | DRIVEN |
| `jigc migrate <path> --as <ty>` | filenames `日本語.md`, `!!!.md` | mints `migrate-adr-2b019b63f9ff` / `migrate-adr-d1f265ef7b06` — `slug_override` = `migrate-<ty>-<blake3-12>`, never empty, distinct | **SAFE** (M44's path-hash) | DRIVEN |
| `jigc doc create <ty> --title` | `""`, `"!!!"`, `"日本語"`, `"the"` | REFUSED `create.empty-title`, blocking, exit 1 | SAFE | DRIVEN (adr, spec) |
| `jigc doc author <ty>` payload `title:` | `"!!!"` | REFUSED `create.empty-title` (same `mint_instance` seam) | SAFE | DRIVEN |
| `jigc doc author` item `title:` | `"!!!"` | REFUSED `write.unslugable-title` | SAFE | DRIVEN |
| `jigc doc add-item <sec> --title` | `"!!!"`, `"日本語"`, `"the"` | REFUSED `write.unslugable-title` | SAFE | DRIVEN |
| `jigc doc add-item … --title ""` | `""` | REFUSED `schema-conformance.field-value-conformant` (*different code, same condition* — a third spelling) | SAFE, surface-tier wobble | DRIVEN |
| `jigc doc add-item --slug` | — | `SLUG_DOORS` guard, `write.malformed-slug` | SAFE | READ (`cli.rs` `slug_arg_ids`, `slug_override_axis.rs`) |
| `jigc doc rename <addr> --to` (in-task) | `"!!!"`, `"日本語"`, `"the"` | REFUSED — ``--to "X" slugs to nothing — pass an explicit `--slug <slug>``` | SAFE | DRIVEN |
| `jigc rename <addr> --to` (committed) | `"!!!"`, `"日本語"` | REFUSED `write.unslugable-title`, routed | SAFE | DRIVEN |
| `jigc rename … --slug ""` / `"!!!"` | — | REFUSED `write.malformed-slug`, routed | SAFE | DRIVEN |
| `jigc doc retitle-item … --title "!!!"` | `"!!!"` | exit 0 — **by design**: the `{#id}` anchor is frozen at M40, the title is not an id source. *(Aside: a `commit#trailers` item retitled this way feeds `trailer_lines`' key from `item.title` — M45's commit-trailer key-shape rule; out of this row's class, not driven to a commit.)* | SAFE-for-identity | DRIVEN |
| `jigc ingest` over a conformant doc at an unslugable filename | `docs/decisions/日本語.md` | not adopted here (non-conformant body); `doc list` shows `adr:日本語 … unregistered`; `validate` → `schema-conformance.unadopted-instance` with the identity leg. **M50's declared bound** (*"`jigc ingest` still adopts an unaddressable identity"*) is the open half. | pre-existing declared bound, not this class | DRIVEN |
| `engine::milestone::reseed_sub_task_areas` | — | `slug_override = Some(&item.id)` read from the committed record — a frozen id, no derivation | SAFE (can *resurrect* a degenerate id, never mint one) | READ `milestone.rs:1117` |
| `fix-task` / `fix-finding` | — | `packs/methodology/workflows/fix-task.yaml:4` `creates-task: true` ⇒ minted through `start --workflow` ⇒ the `start.rs:92` guard | SAFE | READ |
| `record-decision` / `park-idea` / `do-research` / `form-vision` | — | mint docs through `doc create`/`doc author` ⇒ the shared `mint_instance` guard | SAFE | READ + DRIVEN at the shared seam |
| `config set docs-root` / `placement-root` | — | M50 `ROOT_KNOBS` home + value rules | SAFE | READ |

**The one-line summary of the class:** every `slugify(…).is_empty()` fallback in production is at `state.rs:1315` (`mint_id`, guarded at the CLI door `start.rs:92` and at the engine door `state.rs:1590`), `milestone.rs:2470` (`mint_id`, **unguarded**) and `milestone.rs:727` (`mint_sub_id`, **unguarded**). Three fallbacks, two unguarded, reached by **three** committing doors.

---

## 5 · Existing tests, registries and doc homes

### The axis is `MINT_DOORS`, not `WORK_UNIT_ID_DOORS`

`engine::state::MINT_DOORS` (`crates/engine/src/state.rs:1234`) — *"every production call that opens a working area"*, five rows, count-fenced source-level over both crates by `crates/cli/tests/mint_doors.rs` (call-site set of `mint_task ∪ mint_milestone` **must equal** the `site` set; a sixth mint is a red test, not a silent door), and driven one cell per member (`mint_doors.rs:688–714`):

| `door` | `site` | `mint` | `snapshot` |
|---|---|---|---|
| `jigc start "<intent>"` | `crates/cli/src/start.rs::mint_in_repo` | `mint_task` | `Written` |
| `jigc migrate <path> --as <doctype>` | `crates/cli/src/start.rs::mint_migration_in_repo` | `mint_task` | `Written` |
| `jigc milestone create "<title>"` | `crates/cli/src/milestone.rs::run_create` | `mint_milestone` | `Written` |
| `jigc milestone add-task <m> "<intent>"` | `crates/engine/src/milestone.rs::add_task` | `mint_task` | `Exempt(…)` |
| record-driven re-seed (any operating milestone op on a fresh clone) | `crates/engine/src/milestone.rs::reseed_sub_task_areas` | `mint_task` | `Exempt(…)` |

`add-from-spec` is **not** a sixth row — it reaches `add_task`, which is why an engine-side guard covers it and a CLI-side pair does not.

### The fence that is already green over this defect — the natural acceptance

`crates/cli/tests/work_unit_id_axis.rs:485` — **`every_mint_door_produces_an_id_every_door_accepts`**. It already:
- dispatches **`MINT_DOORS` by `site`**, hard-panicking on an undriven member;
- feeds each door `HOSTILE = "  ÄÖÜ Straße —— re-DO the *WHOLE* thing!!  "` (`:399`);
- asserts every resulting area name satisfies `engine::slug::is_slug`, and feeds the id back through a real door.

**`is_slug("milestone")` is `true`, so the fence passes over the defect.** The axis is present; the missing **cell** is *the input slugs to nothing*, and the missing **predicate** is *the id is derived from the caller's prose rather than fabricated*. Adding that cell to this suite's existing `MINT_DOORS` loop is the acceptance — a cell in a shipped registry-iterating suite, no new registry, which is exactly the charter's "no new mechanism" boundary.

### Tests that pin the current behaviour and must move with the fix

- `crates/engine/src/milestone.rs:4801` `empty_title_falls_back_to_type_name` — asserts `mint_milestone(root, "!!!___---")` → `minted.id == "milestone"` and the directory exists. **Pins the defect as expected engine behaviour.** Stays green if the guard goes at the CLI door (`run_create`); must change if it goes in `mint_milestone`.
- `crates/engine/src/state.rs:2400–2412` `…falls_back_to_type_name` / `whitespace_intent_falls_back_to_type_name`, and the proptest `mint_id_is_slugify_for_non_empty` (`:2419`) — the task-side equivalents; intact today because `start.rs:92` guards at the **CLI boundary** and the fallback stays correct for `mint_migration_in_repo`'s deliberately-empty intent. **That is the precedent for where to put the milestone guard.** (Counter-pull: putting the sub-task guard in `engine::milestone::add_task` is the only single site that also covers `add-from-spec`; no engine test drives `add_task` with a degenerate intent, so that placement is free.)

### Related suites
`crates/cli/tests/malformed_work_unit_id.rs` (`CODE` at `:44`) · `crates/cli/tests/work_unit_id_axis.rs` (25 doors × 4 cells; `:378` asserts `driven.len() == WORK_UNIT_ID_DOORS.len()`) · `crates/cli/tests/blocked_finding_log_axis.rs:62–95` (six `work-unit.malformed-id` rows) · `crates/cli/tests/slug_override_axis.rs` (`SLUG_DOORS`, six doors, `--slug` cells) · `crates/cli/tests/flow51_acceptance.rs:568–650` · `crates/cli/tests/flow52_acceptance.rs:1848–1980` · `crates/cli/tests/write_finding_keys.rs`, `flow37_rename.rs` (the `write.unslugable-title` keys).

### Doc homes
- `crates/engine/src/state.rs:2041` — the only place the rule is stated, and it is scoped to `doc create`.
- `design/write-commands.md:209–211` — *"mints a milestone work-unit (id = frozen slug from the title)"*; **silent on a title that slugs to nothing**, for all three milestone verbs. This is the doc home the fix owes a sentence.
- `design/team-ready-state.md:141` — the record-owns-the-id rule (`milestone.record-exists`), the guard the degenerate id squats behind.
- `design/validation.md:75` and `design/write-commands.md:139` — the `write.unslugable-title` taxonomy homes (both already list it as a shared, multi-producer code).
- `implementation/roadmap.md:1609` — the M13 precedent for this exact fix.
- `design/command-output-contract.md:237` — why `create.empty-title` keys at the bare doctype id; the milestone analogue would key at `milestone:<?>`, which does not exist — **so the new producer's `target` should be `None` or a bare `milestone` token, a question the Settle owes an answer to** (the `--format json` arm at these doors is the flattened `{"error": …}` envelope anyway — driven — which is M52's declared bound for the milestone family, not something this row introduces).

---

## 6 · What I did not drive
- `milestone provision` / `execute` / `join` against a degenerate id (the `list-tasks`/`discard`/`finalize` doors were driven; `provision` needs worktrees).
- The commit-trailer key-shape consequence of `doc retitle-item --title "!!!"` on a `commit#trailers` item all the way to a rendered commit message.
- Any `cargo` build or test run (prohibited for this pass — the `MINT_DOORS` fence and the pinning suites are **READ**, not executed).
- `ingest`'s adoption of a *conformant* doc at an unslugable filename (my fixture's body was non-conformant, so it routed to `needs-reconcile` before the identity question landed). M50's declared bound says the hole is open; I did not close the question.
