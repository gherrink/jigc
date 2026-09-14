<!-- 2026-09-10 · Opus gap-detector · dimension: doctypes · HEAD bd348a83 · target/release/jigc 1.0.0-rc.14 · no cargo, no repo edits · copied verbatim -->

# M51 gap probe — dimension: doctypes and packs

Probed at HEAD `bd348a83`, release binary `target/release/jigc` = `1.0.0-rc.14`.
Every row marked **driven** was produced by running that binary on throwaway rigs
(`dev/jigc-rig committed-singletons --binary …`, two-step eval) or by loading a copied pack
tree through `JIGC_PACK_DIR`. No repo file edited, no cargo run. Rows marked *source-read*
are leads, and say so.

Ranked: blocking first, then forks, then advisory.

---

## BLOCKING

### D1 — The stated-at fence's owe-set is a hand-listed const, and M51's Tier-0 scope fires the deferral that says so

`crates/cli/src/pack.rs:779` — `const AMBUSH_CLASS_CODES: [&str; 4]` = `finalize.promote-clobber`,
`finalize.left-out`, `finalize.nothing-staged`, `finalize.carried-staged`. `assert_stated_at`
(`pack.rs:802`) requires **every manifest-shipping pack, loaded alone**, to carry a step declaring
each member; `assert_named_facts_stated` (`pack.rs:1303`) then requires that step's prose to contain
that code's tokens (`CONSTRAINT_REQUIRED_TOKENS`, `pack.rs:1242`, 6 rows).

**Both fences have teeth — driven.** Copying `crates/cli/pack` and dropping one code from
`steps/finalize.yaml`'s front-matter:

```
$ JIGC_PACK_DIR=<copy> jigc describe          # rc=1
pack-load stated-at fence failed: no step of this pack declares `states-constraints:` for the
ambush-class code(s) `finalize.carried-staged` — the contract would first appear in its block
message, an ambush
```

and keeping the declaration while renaming the token in the body:

```
$ JIGC_PACK_DIR=<copy2> jigc describe          # rc=1
pack-load named-fact fence failed: step `finalize` declares `finalize.carried-staged` but its
prose never says "--carry-staged"
```

**The gap:** the *membership* of `AMBUSH_CLASS_CODES` is hand-written and derived from nothing.
The rule it encodes is *"a contract that first appears in its own block message is an ambush."*
Three M51 Tier-0/Tier-3 items mint exactly that shape:

- **EC-2** — a HEAD-posture refusal at the ten `COMMITTING_DOORS`. An agent walking
  `step:finalize` meets it for the first time in the block.
- **EC-26** — `setup` joining the carryover gate. `finalize.carried-staged` is *already* a member;
  its declarer is `step:finalize`, and **no pack step of either pack solicits `jigc setup`**, so a
  setup-side carryover contract has no home the fence's own route (*"state each contract in the step
  that solicits the write it gates"*) can name.
- **EC-1** — a refusal (or a named retire target) at the destructive migrate sink. Its natural home
  is `steps/migration-finalize.yaml`, which already declares `finalize.promote-clobber`.

**This fires a recorded trigger, verbatim.** `implementation/decisions-pending.md:601` (N26) ends:
*"Trigger: … **or the stated-at fence's owe-set is next opened** — at which point the question to
settle is whether the fence's subject is *packs that ship steps* rather than *packs that ship a
manifest*, and whether an unstamped managed corpus deserves a store-surface answer of its own."*
M51 opens that owe-set. The deferral is due at this Settle, not after it.

**What the wave owes:** for each new blocking contract at a committing/destroying door, a decision
on (a) whether it joins `AMBUSH_CLASS_CODES`, (b) which step declares it, (c) what tokens join
`CONSTRAINT_REQUIRED_TOKENS`, and (d) whether the set becomes derived rather than listed. Omitting
(a) is silent: nothing reddens when a new ambush-class contract stays off a hand-list.

---

### D2 — No schema-shape **or home** change is admissible anywhere in this wave, and the razor's word "frozen" covers **both** packs' 17 manifest entries

Driven, on a copied dev pack loaded through `JIGC_PACK_DIR` against the release binary:

| edit to `crates/cli/pack/schemas/adr.yaml` | `jigc describe` |
|---|---|
| control (unmodified copy) | rc=0 |
| every slot `hint:` reworded | **rc=0** |
| `description:` reworded | **rc=0** |
| a YAML comment inserted | **rc=0** |
| `location: decisions/` → `decisions-probe/` | **rc=1** — `pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares 2d860240…, recomputed 29de3c25…)` |

The mechanism is `engine::manifest::schema_hash` (`crates/engine/src/manifest.rs:52`) over
`erase_presentation` (`:76`), whose `Schema` destructure is exhaustive: **`ty`, `location`,
`id_from`, `display_title`, `placement`, `singleton` and every `section` are inside the hash**;
only `description`, `usage` and every slot `hint:` (both loci) are erased.

**Scope of the freeze at HEAD: 17 manifest entries, 16 distinct doctypes** —
`crates/cli/pack/config/schema-manifest.yaml` (6: commit 1, adr 2, spec 1, prd 1, arch-doc 1,
changelog 2) and `packs/methodology/config/schema-manifest.yaml` (11: commit 1, completion-record 2,
decisions-log 1, deferral-ledger 2, dogfood-record 1, idea 1, milestone-record 3, planning-record 1,
research 1, roadmap 1, vision 1). The charter's razor says *"any schema-shape change to a frozen
doctype … fails leg 0 as a class"*; read against the manifests that is **all 16**, methodology
included — not just the six dev-pack ones the phrase "frozen v1" evokes.

**Applied to the three items the task named:**

| candidate | touches a hash? | owed if taken | razor |
|---|---|---|---|
| a **review-hold retire statement** naming the file `--approve` will delete (EC-1) | **no** — the surface is `crates/cli/src/render.rs`'s migration-review render + the pinned hold keys `["review","rewrites","source","task"]`, plus `steps/migration-finalize.yaml` prose (a step body, outside every hash) | nothing | admitted |
| an N15 **`staged` note** / a `--task`-preserving route on `store.not-found` | **no** — `crates/cli/src/doc.rs` resolver + route text | nothing | admitted |
| a **`milestone-record` field for HEAD posture** (EC-2) | **YES** — `packs/methodology/schemas/milestone-record.yaml` has no branch/posture leaf (`meta`: `base`, `status`; `tasks` item block: `task-id`, `intent`, `workflow`, `status`). A new leaf is an `AddedItemField`/added header field ⇒ hash moves | `schema-version` **3→4**, a fifth `schema-snapshots/milestone-record.v3.yaml`, a shipped corpus migration, a `Manifest-Repin`-free co-moved version, and both manifest re-pins | **refused by the razor** |

**And EC-2 does not need it.** A `git symbolic-ref -q HEAD` probe at the committing seam is a
refusal, not a record. Nothing in Tier 0–3 requires a schema field. **The wave's honest statement
is: zero schema-hash movement, zero `schema-version` bumps, zero corpus migrations** — and any plan
that reaches for one has left the charter.

**One trap inside the free set:** the erase list covers `hint:` at *both* loci but nothing else in
a slot. `optional: true`, `set:`, `default:`, `of:`, `check:`, `title-names-symbol` are all inside
the hash. A Tier-2 "reword" that also flips `optional:` is a version-gated change wearing a prose
costume.

---

### D3 — `e2e_audit::floor_patterns()` scrapes **20 of the profile's 22** deny entries, and the two it misses are the two the repo hard-pins by hand

`crates/cli/tests/e2e_audit.rs:261-281`. The scraper walks `crates/cli/adapters/claude-code.yaml`
from `deny:` and `break`s on the first line that is not `- `. The profile carries a **comment** at
`claude-code.yaml:32-34` immediately above the two M50 entries, so the loop stops there.

Replicated exactly (driven, python replica of the scrape): **20 patterns**, ending at
`Bash(cat ./**/.npmrc:*)`. `Bash(jigc uninstall:*)` (`:35`) and `Bash(jigc milestone discard:*)`
(`:36`) are **not in the scraped set**.

This sharpens EC-7 in two directions at once:

- **Narrower than stated in one respect** (baseline3 §5 is right): the whole 22-pattern floor *is*
  independently pinned twice as inline `insta` literals in `crates/cli/src/adapter.rs` —
  `claude_code_profile_bytes_are_canonical` (`:1495`) and `deny_floor_added_then_idempotent`
  (`:2623`) — plus `the_deny_floor_carries_the_two_human_owned_destroyers` (`:1569`) against the
  *loaded* profile. The gitignored `crates/cli/src/.adapter.rs.pending-snap` (318 KB, first record
  is `claude_code_profile_bytes_are_canonical` with a pre-M50 body) is live evidence a profile edit
  really does redden it.
- **Wider in another, and this is new:** the e2e assertion's subject is **truncated by a comment**,
  not merely self-referential. Any deny entry the wave adds *below a comment* is invisible to the
  one test whose job is *"every profile floor pattern lands in `.claude/settings.json`"*. A fix that
  only replaces `include_str!` with a `const SECRETS_FLOOR` and leaves the parser will still be
  blind at the seam that already blinded it.

**And the profile-bytes question the wave has to answer:** the deny floor's stated rule is *"a
refusal an agent can consent past is not a floor"* (M50). `permit: ["Bash(jigc:*)", …]`
(`claude-code.yaml:10`) blanket-permits **`jigc migrate <path> --as <T>` and `jigc task finalize
<id> --approve`** — the pair EC-1 proves is the binary's only exit-0 byte-destructive path,
confirmed deleting an unversioned external file and `.git/config` (baseline1 §2a) — while
`uninstall` and `milestone discard`, which refuse before destroying, are denied. If the Settle
decides that asymmetry is wrong, the profile gains a third deny entry: profile bytes move,
**both** `insta` snapshots redden (inline, re-acceptable), the SKILL.md/settings.json e2e arms
re-run, and the new entry must sit **above** `:32`'s comment or the scraper still cannot see it.
If the Settle decides it is right, that reasoning belongs in the record — silence here is a
by-omission blessing of the cheap read.

---

### D4 — The pack-step-count fence bans the number that is now **true**

`crates/cli/tests/foldback_truth.rs:454-469`,
`the_stale_pack_step_count_survives_only_inside_a_dated_correction`: for each of the eight
`STEP_COUNT_HOMES` (`:311`) it collects every count claim whose value is `== 69` (`:458`) outside a
dated `[Corrected …]` bracket and fails with *"`69` pack step files was never the number — it is
`ls <dir> <dir> | wc -l` counting its own two directory headers and blank separator over a tree of
66."*

Counted at HEAD: `ls crates/cli/pack/steps/*.yaml` = **30**, `ls packs/methodology/steps/*.yaml` =
**39**. **The tree is 69.** The fence's own prose is stale in two places —
`foldback_truth.rs:42` (*"HEAD is already **67**"*) and `:534` (*"the tree is already at 67"*).

Consequences for M51, both real:

1. Any M51 doc, record or fold-back that states the current pack-step count correctly **reddens the
   gate** with a message asserting that number was never the number. EC-14's own axis is *"hand-written
   numbers over sets the code can move"* — this is the fence built for that axis having become an
   instance of it.
2. `the_pack_step_count_is_stated_once_and_names_its_measurement_point` (`:472`) additionally asserts
   the rc.11 charter row in `implementation/decisions-pending.md` states **66** exactly once and names
   `1.0.0-rc.10` + `pre-1.0.0 trial`. That row (`decisions-pending.md:355`) is inside EC-14's sweep
   subject. Touching it without touching the fence is a red gate; touching the fence without keeping
   the historical measurement point is the record-truth violation the wave exists to prevent.

If EC-10's fix is a new pack step (below), the tree goes to **70** and cell 1 disappears by accident
— which is the worst outcome, because the fence stays wrong and nobody notices.

---

### D5 — `steps/migration-finalize.yaml` states a fact the binary breaks in exactly EC-1's cells, and it is a **fenced named fact**

Both packs ship a near-identical `migration-finalize` step
(`crates/cli/pack/steps/migration-finalize.yaml`, `packs/methodology/steps/migration-finalize.yaml`),
each declaring `states-constraints: [finalize.promote-clobber]` and stating:

> *"Retired means removed: the foreign file is deleted from the worktree **and the deletion staged**,
> so the removal lands in the same commit as the canonical doc (a `git rm` in effect)."*

Driven in baseline1 §2a/§4: `cli::task::stage_migration` (`crates/cli/src/task.rs:3064`) stages the
deletion **only if `path_in_index`**, so for an absolute out-of-repo source, a `.git/` component
source, or a tampered `source-path`, the file is deleted and the deletion is **not** staged, not in
the manifest, and not in the commit. The step's sentence is false in every EC-1 cell.

Two obligations follow:

- The prose is not free-form: `CONSTRAINT_REQUIRED_TOKENS` (`pack.rs:1242`) pins
  `finalize.promote-clobber` → `["--approve", "retire", "fidelity diff"]`, and the named-fact fence
  reddens on a rewrite that drops one (driven in D1). Any repair keeps all three tokens.
- **EC-1's shape decides whether the sentence needs repair at all.** If F1 lands the robust reading
  (a classified path family at the door **plus** re-validation at the sink, `engine::finalize::plan_retirements`
  / `cli::task::retire`), the out-of-repo cells become unreachable and the step's claim becomes true
  again with no pack edit. If F1 lands the cheap reading (the `migrate` door only), the sink cell
  driven in baseline1 §2a.1 survives — benign mint, hand-edited `.jigc/tasks/<id>/source-path`,
  `--approve` at rc=0 deleting an arbitrary host file — and **this step keeps shipping a false
  statement about the one destructive op**, in both packs, under a fence that certifies its facts
  are named.

Also unfixed by either: the step is the *only* surface in the migrate flow that describes the
retire, and neither it nor the exit-4 hold names the **path** (baseline1 §4 — `grep -c "$EXT"
hold.txt` → 0; the pinned hold JSON keys are `["review","rewrites","source","task"]`, no retire key;
`jigc task validate` silent). Naming it is a render change, not a schema change, and it is the
cheapest half of EC-1.

---

## FORKS

### F-a — `fork · foreclosed-by-doc`: the razor's blanket refusal of frozen-schema changes may **invert** the human's own criterion

The charter (razor, *What it refuses by construction*): *"Any new verb, any schema-shape change to a
frozen doctype, and any second execution shape fail leg 0 **as a class**: each is **additive after
the pin**, so none is a one-way door."*

The human's criterion, quoted in the same charter: *"everything that becomes impossible or expensive
to change **once people start using the product**. Now it's cheap."*

For a **new** doctype the razor's inference holds — the methodology manifest header says so in its
own words (*"A NEW doctype is FREE AT THE FREEZE"*). For a **shape change to an existing frozen
doctype** it runs the other way: `adr` 2→3 today ships a migration that must run over **zero**
adopter corpora; the same bump after 1.0.0 must run over every adopter's committed store, and
`schema-conformance.schema-version-current` flips their CI red until it does
(`design/corpus-migration.md`; `STORE_EXIT_FLIPS`). That is the textbook *expensive after the pin*.

Both readings are defensible — F-8's own recorded trigger (*"whichever wave first needs a
cross-doctype ref"*) is a real necessity leg the razor can fail on, and *no adopter needs it today*
is a genuine leg-0 refusal. But the charter states the **cost** conclusion (*"additive after the
pin"*) as though it were leg-0-decisive, and for shape changes it is the wrong direction. Quote the
rationale, name the inversion, let the human settle it. **I am not arguing F-3/F-8 back in** — I am
flagging that the ground on which they are out is stated backwards, and that leaving it unstated
means a future wave will cite it as precedent.

### F-b — `fork · cheap-vs-robust`: N23/EC-38, doctype retirement × the committed corpus

Driven at baseline4 §6 and re-confirmed as classification-correct by EC-38. With a pack that
*replaces* the dev pack (`JIGC_PACK_DIR`) minus one doctype, `jigc validate` prints *"no findings —
the committed store validates clean"* at **exit 0** while `doc list` **drops the rows entirely** and
`git ls-files` still carries every file, unchanged.

- **Cheap:** leave it. The reachable population is narrow — baseline4's control shows a project-*listed*
  pack defining no doctypes does **not** reproduce it (composition means the embedded pair still
  provides the type), so it needs `JIGC_PACK_DIR` or a PB-1 project pack that drops a doctype it owns.
- **Robust:** a doctype leaving the resolved set routes its orphaned instances — the M38/M39
  detect-route floor applied to **deregistration** rather than relocation, joining the existing
  `orphan.rs` machinery and the M42 managed-vs-foreign discriminator.
- **The long-run cost of cheap, stated:** the false green is on `jigc validate`, the exact verb
  `MIGRATING.md` ships into every adopter repo telling them to **CI-gate on**. After 1.0.0 the
  reachable population stops being narrow — PB-1 (M49) is the shipped, documented way for an adopter
  to own doctypes, and every adopter who evolves one crosses this cell. Reversibility: a new finding
  code + a route is additive, so this is *not* a one-way door; it is a *known hole in a declared
  surface* (razor leg 0's second clause), which is the leg it must be argued on.

### F-c — `fork · cheap-vs-robust`: EC-10's home — a pack step, a fence, or neither

**A pack step is the wrong home, and the pack layer says so.** `packs/methodology/workflows/completion.yaml`
is a **methodology-pack** artifact that ships into every adopter repo; the version bump it would
name is **jigc's own `Cargo.toml`**. A step telling an adopter's agent to bump a workspace version
is a law-1 lie for every reader who is not this repo. `packs/methodology/steps/re-verify.yaml`
already models the right register (*"the project's full gate green … whatever toolchain it uses; do
not assume a particular one"*).

What adding a step **would** move, if taken anyway (each verified):
- `crates/cli/tests/goldens/compose/methodology/workflow-preview--completion--*.txt` — **6** files
  (17 methodology workflows × 6 rig states = the 102 `workflow-preview` goldens).
- the pack-step count → 70 (D4).
- **not** `packs/methodology/config/schema-manifest.yaml` — the manifest declares doctypes only; no
  hash moves, no version bumps. Confirmed against the file and `engine::manifest::check`.
- the read-back fence (`pack.rs:1164`) and the singleton copy-in fence (`pack.rs:955`) only if the
  new step solicits a managed-doc write (a lone-line `{{cli.<id>}}` resolving to a `jigc doc
  <write-verb>`, or a `{{schema:<T>}}` payload ref — `pack::solicits_managed_doc_write`,
  `pack.rs:1124`). A `Run:`-only step owes neither.

**The robust home is a fence, and the repo already has its mold.** `crates/cli/tests/foldback_truth.rs`
is the one suite whose subject *is* repo prose; `doctype_map_versions.rs` is the count-fence
exemplar. The weakest checkable claim that would have caught all five misses is *the version this
fold-back names is the version `Cargo.toml` carries* — which `foldback_truth.rs:223`'s recorded
refusal does **not** cover (it declines choosing a **numeral**, not comparing two strings). Note
`release_smoke.rs:100` cannot help: it derives from `CARGO_PKG_VERSION` and is green at any version.

---

## ADVISORY

### A1 — The fourteen `planning-record` gates, for M51's hand-built gate-record

`packs/methodology/schemas/planning-record.yaml` — `location: planning-records/`, `id-from: title`,
header-less, **one required slot per gate**, no `optional:` anywhere. In schema order, with line
numbers:

| # | gate id | line |
|---|---|---|
| 1 | `reuse-exercised` | `:53` |
| 2 | `cheap-vs-robust` | `:57` |
| 3 | `foreclosed-by-doc` | `:61` |
| 4 | `prior-art-reconciled` | `:65` |
| 5 | `census` | `:69` |
| 6 | `integration-seam` | `:73` |
| 7 | `check-scope-pinned` | `:77` |
| 8 | `design-complete` | `:81` |
| 9 | `acceptance-spiked` | `:85` |
| 10 | `value-flow-exercised` | `:89` |
| 11 | `deliverable-reachable` | `:93` |
| 12 | `strategic-claim-fresh` | `:97` |
| 13 | `quote-attributed` | `:101` |
| 14 | `claim-driven` | `:105` |

Each gate's `hint:` is the *"What it requires recorded"* cell of `design/methodology-docs.md` →
*The planning gate-record*, verbatim; `crates/cli/tests/planning_gate_home.rs` and
`planning_record_schema.rs` hold the doc and the schema in lockstep, both deriving the gate ids from
the **shipped** schema through `load_pack_schema`.

**Writing `completions/artifacts/M51/planning-gate-record.md` is safe.** `planning_gate_home.rs`'s
`GOVERNED_TREES` is `["design", "implementation", ".claude"]`; its module doc states *"`completions/`
holds the **filled instances** of the gate-record: a milestone's own `planning-gate-record.md`
legitimately answers all fourteen rows."* No fence reddens.

**Slots this wave will strain on, named now rather than at the gate:**

- **`acceptance-spiked`** (*"every acceptance flow's behaviour and commands spiked on the real
  binary — driven through the shipped verb's entry point"*). M51's acceptance is the **per-axis
  review** (charter → Acceptance), which is a cell matrix, not a flow with commands. Filling this
  honestly means spiking the eight axes' cell matrices, and satisfying the review's own fence — *every
  leaf in `VERB_KINDS` (`cli.rs:1412`, 47 leaves) appears in at least one axis's matrix or is named
  uncovered*. Fork 9 (whether a blind trial also runs) is open, so this cell cannot close before the
  Settle resolves it.
- **`claim-driven`** (*"filled by naming what was driven"*). Six ledger rows are source- or repo-read
  only and must be written in **as relayed** or driven first: EC-6, EC-7 (fence-quality grades from
  reading test bodies), EC-8 and EC-9 (absence claims by grep), EC-24 (absence by grep), and EC-30's
  **milestone** arm, explicitly not driven. Additionally `EC-14`'s *"three `DECISIONS.md` citations
  point at blank lines"* was **not checked** (baseline4 §10). Cheapest discharge: drive EC-6/EC-7
  with one applied mutation each — baseline3 §5 says so in its own words (*"EC-7's correction in
  particular deserves one applied mutation before it is acted on"*), and D3 above shows the reported
  grade was wrong in both directions.
- **`value-flow-exercised`** — a fix wave has no net-new value flow. Answer it as the pre-1.0 loop
  re-driven at baseline, or `N/A — <why>`; the schema treats an honest `N/A` as a complete answer
  and jigc grades nothing inside a slot.
- **`prior-art-reconciled`** — EC-8 (release-versioning policy) and EC-9 (publishing floor) have
  **no design doc at all**, so there is no prior art to reconcile and the cell must say so rather
  than being left thin.

### A2 — `{{schema:<T>}}` and `slot_ceiling_statement` reach **no** Tier-2 row; the seams that do are `long_about()` and the M7 fence mold

Answering question 6 directly: the `{{schema:<T>}}` compose seam only renders inside **pack step
bodies** (`pack::schema_refs`, `pack.rs:891`), and `engine::write::slot_ceiling_statement`
(`write.rs:7474`) / `slot_ceiling_rule_statement` (`:7523`) / `engine::slug::mint_statement`
(`slug.rs:139`) are already wired at every site they can reach (baseline4 M1/M2/M3, all
*built + proven*). **Not one EC-11/EC-14/EC-22/EC-23 sentence lives in a step body.** They live in
`--help` texts and in the two shipped guides, which the `{{schema:}}` seam cannot see.

The two seams that *are* available:

- **Generated `long_about()`** — the proven mechanism, `#[command(long_about = expr())]`, already
  used six times (`doc.rs:231/256/325/351/371`, `milestone.rs:81`). Candidates, with the set each
  would ride:

  | help text | claim | set to generate from | exists? |
  |---|---|---|---|
  | `jigc migrate-corpus --help` | *"the deterministic v0→v1 transform"* | `engine::schema_diff::SchemaChangeKind::ALL` (`schema_diff.rs:757`, **18** members) + the manifests' declared version range | **yes** |
  | `jigc doc show --help` | *"`{ type, slug, fields, sections }`"* (`cli.rs:238-239`) against **6** emitted keys + `staged` | the serialize struct's own `#[serde(rename)]` set (`doc.rs` ~`:4435-4463`) | **yes**, as a struct — needs a const or a destructure |
  | `jigc task finalize --help` | *"It forecasts **three** gates"* | `crate::gate_coverage::GATE_COVERAGE ▸ Tier::Previewed` (**4** members) — and `whats_left_coverage()` (`gate_coverage.rs:268`) **already generates this exact sentence** for the composed `what's-left:` line | **yes — the cleanest ride available** |
  | `jigc validate --help` | *"every committed doc's **code anchors**"* | the **five probe families** — `grep -rn "PROBE_FAMILIES\|ProbeFamily" crates/` returns **zero**. `STORE_EXIT_FLIPS` is not the right set | **NO — must be minted** |

  Two mints, not one: the probe-family registry above, and `ManifestKind::ALL` for EC-22
  (`cli::render::ManifestKind`, 6 members, no `::ALL` — baseline4 M1).

- **The M7 fence mold for the guides.** `QUICKSTART.md`/`MIGRATING.md` are static markdown
  `include_str!`'d at `crates/cli/src/setup.rs:69-70`; they cannot be generated. The mold is
  `doctype_map_versions.rs` (read the registry, assert the doc's rows) applied through
  `foldback_truth.rs`, which already derives the **guide set** from `setup.rs`'s `include_str!`
  sites and asserts per prose unit.

### A3 — Every Tier-2 guide edit has two pack-layer consequences the batch must price

1. **`unlink_in_repo_links` flattens relative links** (`setup.rs:111-148`): a `[label](target)` whose
   target carries no `://` and no leading `#` ships as **bare `label`**. EC-24's fix (the ahead-stamp
   paragraph) cannot cite `design/validation.md` — the shipped SKILL.md would carry a bare path an
   adopter's repo does not contain. The paragraph must be self-contained prose. Same constraint on
   EC-11/EC-22/EC-23 wording.
2. **The stamp / clobber-refusal bounds the fixes' reach.** `setup.rs:63` `GUIDE_HASH_KEY =
   "jigc-body-blake3:"` is the blake3 of the artifact's own body, so any guide byte moves it.
   `jigc setup` replaces an **owned** SKILL.md and **refuses** to clobber an edited one
   (`adapter-guide.user-modified`, `crates/cli/tests/adapter_artifact.rs:449`), and `jigc upgrade`
   only *reads* ownership (`upgrade.rs:119-136`) — it never re-installs. So the whole Tier-2 batch
   reaches only adopters who (a) re-run `jigc setup` and (b) never edited the guide. Correct
   behaviour; it just means "we fixed the guide" is not "adopters see the fix," and the record
   should say which. No golden pins SKILL.md bytes (it is fenced by property), so no byte golden
   reddens.

### A4 — Two live prior-art contradictions in the doctype/pack layer, both inside EC-14's subject

- **`design/corpus-migration.md:281`** — *"all sixteen shipped doctypes (**6/6 dev + 10/10
  methodology** are manifest-listed)"*. Counted at HEAD: dev **6**, methodology **11**
  (`planning-record` joined at M49). The headline *sixteen* is right by **distinct type** (commit is
  in both manifests) and wrong by **entry** (17). The sentence never says which it means — it must
  say, or the correction re-rots.
- **Both manifest headers** — `crates/cli/pack/config/schema-manifest.yaml:~48` and
  `packs/methodology/config/schema-manifest.yaml:~79` carry the same sentence, *"the narrowing
  re-pinned **all 16 doctype hashes** at unchanged schema-versions."* This is a **true historical
  claim about M47 Increment 1**, not a live count. A count fence that reddens on it would be wrong,
  and the discipline the repo already has for this is the dated bracket
  (`foldback_truth::dated_correction_spans`). **Two homes, one fact** is the second defect: the
  methodology manifest restates the dev manifest's history verbatim.
  `doctype_map_versions.rs` fences only `implementation/doctype-map.md` — neither of these files is
  in its subject, so EC-14's sweep must extend the site set or mint a second fence.

### A5 — The M51 fold-back owes a fence re-aim that has been missed before, by name

`crates/cli/tests/foldback_truth.rs:242` `claude_md_names_m50_and_claims_exactly_what_the_audit_reached`
is pinned to the `**M50 —` span of CLAUDE.md and currently asserts the **post-audit** direction
(requires `built + audited`, requires a `VERDICT` citation whose file must exist on disk, forbids
`built, not audited` / `1.0.0 is called` / `1.0.0 shipped`). At M51's build close it must be
**re-aimed** to `**M51 —` and **inverted** back to the pre-audit direction, then inverted again at
M51's completion fold-back. Its own doc-comment records that this was done **late** last time
(*"Inverted 2026-09-09, late — and the lateness is the finding"*: the audit landed at `95c79be6`,
the fence went red on the spot and stayed red through two commits while the handover recorded a
green gate). The `1.0.0 is called` / `1.0.0 shipped` prohibition is the one to watch: if the human
takes the 1.0.0 call after M51, the fold-back and this fence must move together.

### A6 — Driven: a milestone sub-task's `resume:` composes in the **shared checkout** at exit 0 when `base == HEAD`

The task named this as a baseline amendment (*"sub-task `resume:` not provisioning"*). Driven end to
end on a `committed-singletons` rig:

```
$ jigc milestone create "sub task resume probe"; jigc milestone add-task … ; jigc milestone provision …
$ jigc milestone execute sub-task-resume-probe
Spawn: `cd .jigc/worktrees/tune-the-eviction-policy && jigc workflow sub-task --task tune-the-eviction-policy`

# the composed sub-task text's own recovery line, rendered inside that worktree:
resume: `jigc start --task tune-the-eviction-policy`   — re-composes this workflow if context is lost
task scope: … this task is a sub-task of milestone `sub-task-resume-probe`, whose `jigc milestone
finalize …` is its only commit boundary — every door here stays pinned to the milestone's base
```

- From the **main checkout with HEAD moved** (the ordinary state — every `milestone create` /
  `add-task` lands a record commit): correctly refused and routed by
  `start.rs:1937 blanket_base_pin_refusal` — *"a sub-task's work happens in its own worktree at
  .jigc/worktrees/<id> … run `jigc milestone provision <m>` … then re-run this from that worktree."*
  Holds with the worktree present **and** after `git worktree remove --force`.
- From the **main checkout with `base == HEAD`** (`git reset --hard <base>`; also reachable from a
  fresh clone at the pin, or after a squash fold): **rc=0, no worktree mention, no advisory** — the
  sub-task workflow composes in the shared checkout while its provisioned worktree sits beside it.
  The guard is conditioned on moved history; at base parity there is no guard at all.

Neither the `resume:` line (`render.rs:480`) nor the `task scope:` sub-task clause
(`render.rs:466-470`) names the worktree, while the `Spawn:` line does. Since `milestone finalize`
folds each sub-task's code from **that worktree's** staged index (stated in the composed `execute`
text), code staged in the shared checkout is outside the boundary.

**Which layer owns it:** this is `crates/cli/src/render.rs::task_state_lines` + `start.rs`'s guard
predicate — **not** a pack change and **not** a schema change. None of `read_surface_naming.rs`, the
`states-constraints:` fence or the named-fact guard has to learn the provisioning fact: the
`resume:` line is renderer-appended and outside every pack-load fence. `milestone.rs:2895
partial_worktree_advisories` is the shipped precedent for the advisory shape (per-sub-task, id-ordered,
routed at the idempotent `provision_route`).

### A7 — `finalize.yaml`'s `--carry-staged` prose vs the driven merge-conclusion cell

`packs/methodology/steps/finalize.yaml` (and its dev sibling) declares
`states-constraints: [finalize.left-out, finalize.nothing-staged, finalize.carried-staged]` and says
*"pass `--carry-staged` to declare the carryover deliberate."* Driven (baseline2 §2e): with a merge
in progress and `base == HEAD`, `jigc task finalize --carry-staged` exits **0** and **concludes the
merge** — HEAD gains **two parents**, `MERGE_HEAD` is consumed, the subject is jigc's own commit
message, and the ack says *"1 file committed"* with no mention of a merge. If the wave narrates or
guards that (it is the same `COMMITTING_DOORS` registry EC-2 and EC-26 open), the fenced step owes
the new fact and `CONSTRAINT_REQUIRED_TOKENS`'s `finalize.carried-staged` row owes its token.

### A8 — `gate_coverage_fence.rs`'s eight sites include two composed pack steps

If EC-20's fix moves the previewed set — e.g. `finalize --dry-run` gains a `findings` key, or a
member joins `Tier::Previewed` — every site must gain that member's `token`. Two of the eight are
pack steps (`packs/methodology/steps/finalize.yaml`'s *"previews part of what finalize gates on
(this task's content findings, the carryover gate, the owner-artifact causes that need no staging,
and the granted-but-unused changelog gate)"* and its dev sibling), and the composed `what's-left:`
line is **generated** from `gate_coverage::whats_left_coverage()`. `jigc task finalize --help` is
**not** a site and states a *different, unregistered* three-gate set — that is the un-swept cell,
and the fix is A2's `long_about()` ride.

### A9 — Small, checked, no action beyond recording

- A new root `README.md` / `CHANGELOG.md` (EC-9) is outside `code_anchor_grammar_sites::SCANNED_ROOTS`
  (`crates/engine/src`, `crates/cli/src`, `crates/cli/probes/doc-code/src`, `crates/cli/pack`,
  `packs`, `QUICKSTART.md`, `MIGRATING.md`). No fence reddens; but a README that states the
  `code-anchor` grammar would be an **undeclared** site of it, which is precisely the class that
  test exists to catch — so if the README explains anchors, it joins `SOURCE_SITES`.
- This repo does not self-host, so a root `CHANGELOG.md` never meets the `changelog` doctype's
  `placement: { file: CHANGELOG.md }` home. No conflict. (It would in any adopter repo — which is
  what makes writing one here a *product-shaped* act, not just a release chore.)
- Changing any workflow's `description:`/`usage:` front-matter moves the six `describe--*` and ten
  `start-orient*` goldens. Routine; name it in the close increment so the regeneration is not
  discovered at the gate.

---

## What I did **not** reach (so it is not cleared)

- **No cargo run, no applied mutation.** Every fence-quality grade above is from reading the test
  body plus driving the release binary through `JIGC_PACK_DIR`. D3's claim that
  `claude_code_profile_bytes_are_canonical` reddens on a deny-list edit rests on the pending-snap
  residue and on baseline3's read, not on a run.
- **The `grep -rn include_str! crates/cli/tests/`** sweep for EC-7's wider *self-referential fences*
  axis — not run here either (baseline3 also flagged it unrun).
- **The methodology pack loaded alone** (the dogfood path `assert_stated_at` names) was not driven;
  the copied-tree spikes ran against the dev pack via `JIGC_PACK_DIR`.
- **Whether a new ambush-class code should join `AMBUSH_CLASS_CODES`** is a Settle question, not a
  finding — D1 names that the *absence of a mechanism to notice* is the gap, not that a specific
  code belongs.
- **A6's consequence** (code staged in the shared checkout is outside the milestone boundary) is
  read off the composed `execute` text's own statement plus `milestone finalize`'s documented fold;
  I drove the silent compose, not a full fan-out landing that demonstrates the loss.
