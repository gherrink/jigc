<!-- M51 baseline · companion 4 of 4 — the prose surfaces 1.0 ships, and the fences that keep them true. Driven 2026-09-10 by one Opus capability-auditor against the release binary `1.0.0-rc.14` at HEAD `bd348a83`; no cargo run, no repo file edited. Verbatim as returned; consolidated in [baseline-ledger.md](baseline-ledger.md). -->

# M51 baseline — area: the prose surfaces 1.0 ships, and the fences that keep them true

Verified at HEAD `bd348a83` against the release binary
`/Users/maurice/projects/gherrink-jigc/target/release/jigc` (`jigc 1.0.0-rc.14`).
**A map, not gospel.** No repo file was edited; nothing here is settled.
Rigs built with `dev/jigc-rig <state> --binary target/release/jigc`, two-step eval,
`TMPDIR` under the session scratchpad. Six throwaway repos (`committed-singletons` ×5,
`vendored` ×1).

---

## 1 · The law-1 fences that already exist for prose — the reusable menu

Nine distinct mechanisms. For each: the **set it reads**, the **prose it binds**, and its
**reach**. This is the menu a "counts are generated, not written" fix can draw on.

| # | mechanism | reads (the set) | binds (the prose) | status |
|---|---|---|---|---|
| M1 | `engine::slug::mint_statement(&str)` — `crates/engine/src/slug.rs:139` | `MAX_WORDS`, `MAX_CHARS`, `EDGE_STOPWORDS` (`slug.rs:159`), interpolated | the slug-mint sentence at **3 sites**: `doc create --help`, `doc add-item --help`, the `{{schema:}}` projection | **built + proven** |
| M2 | `engine::write::slot_ceiling_statement(&schema_slot_ceilings(schema))` — `write.rs:7474/7387`, called at `compose.rs:1135` + `4169` | the schema's own `(section, item-chain) → SlotCeiling` map — the *same* derivation `slot_ceiling_finding` enforces with | the reserved-heading-depth sentence inside every composed authoring step and every `{{schema:}}` skeleton | **built + proven** |
| M3 | the `{{schema:<T>}}` compose seam + payload-skeleton renderer | the loaded `Schema` | the whole payload skeleton in the 12 migrate/author step bodies — hand-enumerated schema structure was deleted | **built + proven** |
| M4 | `cli::pack::assert_singleton_copy_in_stated` (`pack.rs:955`) — the `states-constraints:` pack-load fence | owe-set **derived**: every step whose body carries `{{schema:<T>}}` where `T` is a singleton, over both packs' manifest-shipping constituents | forces the step to *declare* `create.singleton-copy-in`; biconditional since M47 Inc 9 T3 | **built + proven** |
| M5 | `cli::pack::CONSTRAINT_REQUIRED_TOKENS` (`pack.rs:1242`, **6 rows**) + `COPY_IN_APPEND_TOKENS` (`pack.rs:881`, 4 tokens) — the **named-fact guard** | `constraint-code → required-token(s)`, compared against `normalized_body` (whitespace-collapsed, case-folded) | the declaring step's own prose must *contain* the phrases; a declaration cannot outlive the statement | **built + proven**, with a **declared bound**: presence, never content; a project-layer forked step and a manifest-less pack are outside it |
| M6 | `crates/cli/tests/foldback_truth.rs` (662 lines, 5 arms) | (a) the `MIGRATING.md` back-out ladder's own numbering; (b) `setup.rs`'s `include_str!` sites — the **shipped-guide set is derived, not listed**; (c) the `**M50 —` span of CLAUDE.md; (d) `STEP_COUNT_HOMES`/`RESTATEMENTS`/`RECORDS` — hand-listed | CLAUDE.md's milestone claim (must say `built + audited`, must cite an **existing** `VERDICT` file, may not say `1.0.0 is called`); MIGRATING's hook-rejection gate in meeting order; QUICKSTART's cross-ref; **both shipped guides must name `task-discard.staged-prose` + `--force` in every unit that names the door**; the pack-step count lives in one home and names its measurement point | **built + proven** — the only fence whose subject *is* the shipped guide bytes |
| M7 | `crates/cli/tests/doctype_map_versions.rs` (390 lines, 4 arms) — **the count-fence exemplar** | both `schema-manifest.yaml`s, every declared entry, keyed **per (pack, doctype)**; diverging versions *redden* rather than collapse | `implementation/doctype-map.md`'s table: every row states its declared `schema-version`; **row set == declared set**; and the freeze scope pin must name each above-v1 methodology doctype *with* its version and may not say `ten` beside a version | **built + proven** — the exact shape a stale-count fix reuses |
| M8 | `crates/cli/tests/planning_gate_home.rs` (296 lines) | the gate ids read out of the **shipped `planning-record` schema** through the production `load_pack_schema` path — nothing hand-listed | the single-home rule over `design/` + `implementation/` + `.claude/`: no second file may restate a **majority** of the gate set | **built + proven** — calibrated on two real data points (13-of-14 red, 5-of-14 green) |
| M9 | `crates/cli/tests/gate_coverage_fence.rs` + `cli::gate_coverage::GATE_COVERAGE` (12 rows × `Tier::{Previewed, LaterSummary, LaterCause}`) | the code-side registry | **8 sites**, incl. `QUICKSTART.md → the core loop`, `design/command-output-contract.md`, `design/finalize.md`, two composed pack steps, the `task` tip, the `validate-task` hint, the generated `what's-left:` line | **built + proven**, but see §2 gap: **`jigc task finalize --help` is not a site**, and it states a *different* count |
| M10 | `crates/cli/tests/read_surface_naming.rs` | the fence's **own** owe-set (`cli::pack::solicits_managed_doc_write` over `doc_write_command_ids`); dispositions derived from each step's bytes; `the_derivation_is_not_vacuous` reddens an empty partition | every composed workflow (driven through `jigc workflow <id> --preview`) **and the installed `SKILL.md` read off disk after a real `jigc setup`** | **built + proven** |
| M11 | `crates/cli/tests/help_truth.rs` (462 lines, 12 arms) | mostly **hand-written token lists**; one arm is axis-iterating (`no_verb_help_carries_a_rust_path` over `leaf_verb_paths()` = the real clap tree) | 8 specific help texts | **shape-limited** — see §2 |
| M12 | `design/surface-contract.md` → *The surface style guide* (the judgment tier) | nothing — it is prose about prose | the tier no fence holds (one-line leads, scope honesty, register) | **deferred-by-design**, stated as such in the doc |

**The shape all the green ones share, and the one a count-fix must copy:** *the subject is
derived from a code-side artifact, never listed in the test.* M7 is the cleanest instance
(both manifests → the map's rows); M6's guide arm is the cleanest instance over **shipped
prose** (`setup.rs`'s `include_str!` sites → the guide set). M11 is the outlier and is
exactly where §2's defects live.

---

## 2 · Help text — where it lives, what fences it, and the EC-11 axis

### Where the prose lives

- **No `about =` attribute anywhere.** `crates/cli/src/cli.rs` (4691 lines) carries **981
  `///` doc-comments**; clap derives every `about`/`long_about` from them. Same for
  `doc.rs`/`milestone.rs`.
- **Five generated `long_about`s**, the only non-doc-comment help: `doc::set_slot_long_about`
  (`doc.rs:73`), `set_field_long_about` (`:100`), `add_item_long_about` (`:153`),
  `create_long_about` (`:169`), `rename_long_about` (`:204`), plus
  `milestone::create_long_about` (`milestone.rs:67`). These are the sites that can carry a
  seam-generated statement, and `create`/`add-item` already do (M1).
- **There is no table.** Help prose is 47 leaf verbs' worth of doc-comments with no registry.

### What compares help text to a code-side set

| test | what it compares | to what |
|---|---|---|
| `help_truth.rs::no_verb_help_carries_a_rust_path` | every leaf verb's `--help` bytes | the **real clap tree** (`leaf_verb_paths()`) — the one genuinely axis-iterating help arm |
| `help_truth.rs::doc_create_help_states_the_slug_caps` / `..add_item..` | the emitted help | `engine::slug::mint_statement(...)` — **byte-equality against the seam** |
| `help_truth.rs` (9 other arms) | the emitted help | **hand-written token lists** (`"router"`, `"--workflow"`, `"write.title-ignored"`, …) |
| `doc_author_help.rs`, `code_anchor_grammar_sites.rs`, `clap_error_kind_axis.rs` | help bytes | hand-written tokens / a declared site list |

**Nothing compares a help sentence's *scope*, *shape*, *count* or *transform space* to the
code-side set it names.** That is the EC-11 hole, and it is a hole in *M11*, not in M1/M2.

### EC-11's three cases — all three reproduced

**D4 · `jigc validate --help` understates its own sweep — CONFIRMED, driven.**

```
$ jigc validate --help
Re-check every committed doc's code anchors against the codebase and report drift — ...
```

Driven on `committed-singletons`:

```
$ jigc validate
advisory · schema-conformance.repeatable-populated — `docs/decisions-log.md`: ...
1 finding(s) — report-only at store scope (exit 0)
EXIT=0
```

The finding is `schema-conformance`, the **fifth** store family. `design/validation.md`
declares **five**: doc↔code (:324), workflow↔refs + file↔CLI-state (:354), store-wide
forward-ref integrity (:426), store-scope schema-conformance (:435). The help names one.
It is also the verb `MIGRATING.md` tells adopters to CI-gate on.

**D5 · `jigc migrate-corpus --help` names a transform space that does not exist — CONFIRMED, driven.**

```
$ jigc migrate-corpus --help
... applies the deterministic v0→v1 transform (the schema-version stamp + any structural splice) ...
```

`engine::schema_diff::SchemaChangeKind::ALL` is **`[SchemaChangeKind; 18]`**
(`schema_diff.rs:757`), and no shipped doctype is at v1-from-v0: the two manifests declare
`adr` 2, `changelog` 2, `completion-record` 2, `deferral-ledger` 2, **`milestone-record` 3**.
Four waves have falsified this sentence.

**D6 · `jigc doc show --help` understates its own pinned shape — CONFIRMED, driven.**

Help says: `a whole-doc object { type, slug, fields, sections }` (**4 keys**).

```
$ jigc doc show vision:vision --format json | python3 -c "..."
['fields', 'item-count', 'schema-version', 'sections', 'slug', 'type']     # 6
```

`design/doc-read-surface.md:59/62` declares all six correctly. The sibling
`jigc doc list --help` names `item-count` correctly — **two read surfaces describing
themselves to different standards**, which is the tell.

### The EC-11 axis — every help sentence that states a scope, a shape, a count or a transform space

Derived by driving `--help` on **all 47 leaf verbs** of the real clap tree and scanning the
emitted bytes for scope/count/shape words. **62 candidate sentences.** The ones that state
something checkable against a code-side set:

| verb | the claim | the set it names | verdict |
|---|---|---|---|
| `validate` | "every committed doc's **code anchors**" | 5 store families (`design/validation.md`) | **FALSE — understates** (driven) |
| `migrate-corpus` | "the deterministic **v0→v1** transform" | `SchemaChangeKind::ALL` = 18; versions up to 3 | **FALSE** (registry-read) |
| `doc show` | "`{ type, slug, fields, sections }`" | the 6 emitted keys | **FALSE — understates** (driven) |
| `config set` | "every committed doc under it, **managed or not**" | the `location:` doctype set only — placement doctypes excluded | **FALSE — over-claims** (driven; = EC-16, §4) |
| `task finalize` | "It forecasts **three** gates: validation findings, the empty-commit guard, and the carryover gate"; "**Every other gate** … is decided only by the real finalize" | `GATE_COVERAGE ▸ Tier::Previewed` has **4** members (`content-findings`, `carryover`, `owner-artifact-unstaged`, `changelog-gate`); the dry-run's own 3 have **no registry** | **shape-limited** — the three-gate set is hand-written, is a *different* set from the fenced one, and the help is **not** a `gate_coverage_fence.rs` site (= EC-20's mechanism) |
| `uninstall` | "**Three** states it refuses"; "**all three** guards" | `uninstall.dirty-worktree` · `uninstall.staged-prose` · `uninstall.untracked-workbench-file` = 3 | **TRUE today, unfenced** |
| `milestone discard` | "**both** guards" | `milestone.dirty-worktree` + the staged-prose guard = 2 | **TRUE today, unfenced** |
| `doc author` | "writes **three** ways … The **fourth** thing it does NOT do" | `write-commands.md:45`'s four-way write | **TRUE** — deliberately reconciled; checked and clean |
| `doc create` / `doc add-item` / `doc rename` | the slug mint rule | `slug::mint_statement` | **TRUE, seam-generated + byte-fenced** (M1) |
| `doc set-slot` | "Setext headings are rejected at every depth; a rejected write names the shallowest depth free at that address" | the schema-relative rule; **deliberately names no depth** | **TRUE** — the declared M45 carve-out (help renders before an address exists) |
| `describe` | "**every** workflow and doc-type" | the catalog union + off-catalog reasons (M49) | **TRUE** |
| `ingest` | "adopt **every** conformant one (register-only — never moving or rewriting a file)" | — | **TRUE** |
| `upgrade` | "Re-check **every** recorded config delta" | — | **TRUE** |
| `milestone finalize` | the two-half fold | fenced by `help_truth.rs::milestone_finalize_about_names_the_code_fold_beside_the_doc_bodies` | **TRUE, fenced** |
| `start --slug` / `migrate --slug` / `rename --slug` / `doc create --slug` | "validated as a well-formed slug — a malformed value is rejected, never silently re-slugified" | `SLUG_DOORS` (M50) | **TRUE** |

**The axis a fix must sweep:** *47 leaf verbs × every help sentence that names a code-side
set*, with the four false rows fixed at their sets (`STORE_EXIT_FLIPS` is **not** the right
set for `validate --help` — the five **probe families** are; `SchemaChangeKind::ALL`; the
`doc show` pinned key list; the `location:`-vs-`placement` split). The mechanism to reuse is
**M1's**: put the sentence in a `long_about()` generated from the set, and assert
byte-equality — as `doc create --help` already does.

---

## 3 · The installed guide

### The composition and the stamp — confirmed by source read

- `crates/cli/src/setup.rs:69-70`:
  `const QUICKSTART_GUIDE: &str = include_str!("../../../QUICKSTART.md");`
  `const MIGRATING_GUIDE: &str = include_str!("../../../MIGRATING.md");`
- Used at `setup.rs:95-96` inside `guide_body`, behind `unlink_in_repo_links(...)`, preceded
  by `guide_preamble()` (the ownership + version paragraph).
- Stamp: `GUIDE_HASH_KEY = "jigc-body-blake3:"` (`setup.rs:63`) — the blake3 of the artifact's
  **own body**, so a later read tells jigc's bytes from a hand edit. Reserved against the
  profile (`adapter::GUIDE_RESERVED_KEYS`).
- Installed at `.claude/skills/jigc/SKILL.md`; confirmed present in every rig (`git ls-files`
  shows it after `jigc setup`).

**So every sentence in these two files ships into every adopter repo.** They are product
surface, not documentation.

### Which tests read them

| test | what it buys |
|---|---|
| `foldback_truth.rs` | the hook-rejection ladder's contiguity + tokens; QUICKSTART's cross-ref; **the discard-refusal + `--force` tokens in every unit of both guides**, over a set derived from `setup.rs`'s `include_str!` sites |
| `read_surface_naming.rs` | the installed artifact (read off disk after a real `setup`) names both under-named read surfaces |
| `code_anchor_grammar_sites.rs` | `QUICKSTART.md` is a declared `code-anchor` grammar site; both guides are scanned roots |
| `gate_coverage_fence.rs` | QUICKSTART's core-loop region names every `Tier::Previewed`/`LaterSummary` member |
| `release_smoke.rs:147` | QUICKSTART names the three loop commands |
| `adapter_artifact.rs`, `author_batch_scaling.rs` | reference only (module docs) |

**No test asserts a guide sentence against a code-side enum, key list or door registry.**
That is the EC-19/22/23/24 hole.

### The four divergences — all driven

**Q1 · CONFIRMED.** `QUICKSTART.md:174-175`: *"run `jigc task finalize <id> --dry-run`: it
prints the manifest and stops, changing nothing."* Driven on a freshly minted task — the
exact state the surrounding narrative puts a first-time reader in:

```
$ jigc task finalize add-a-padding-layer --dry-run
blocking · schema-conformance.field-value-conformant — commit:...#header/type: "" is not a member of enum "type"
blocking · schema-conformance.required-slot-present — required slot in section `summary` is empty
EXIT=3
```

Two blocking findings, **no manifest**, exit 3. `MIGRATING.md:35` states the rule correctly
(*"It forecasts no green it would refuse"*); QUICKSTART does not.

**Q2 · CONFIRMED (= EC-20).** Same task, same moment, back to back:

```
$ jigc task validate add-a-padding-layer      →  3 findings (2 blocking + changelog-recording.gate-granted-unused advisory), EXIT=3
$ jigc task finalize add-a-padding-layer --dry-run  →  2 findings (advisory DROPPED),                                        EXIT=3
```

QUICKSTART presents them as one preview surface;
`design/command-output-contract.md`'s third clause says *"every check `task validate`
runs… is the same check `finalize` runs, at the same severity."* Advisory-only, so no gate
moves — but the mechanism is named in §2: `changelog-gate` is a `GATE_COVERAGE ▸ Previewed`
member and the dry-run's own three-gate set is a hand-written list in a help text that no
fence reaches.

**M1 · CONFIRMED.** `MIGRATING.md:35` lists the tag vocabulary as four
(`promoted`/`modified`/`deleted`/`carried-over`). Driven with an ordinary staged new file:

```
$ git add src/pad.ts && jigc task finalize add-a-padding-layer --dry-run
finalize --dry-run — pre-commit manifest (nothing committed)
would commit — feat: add a padding layer
  added src/pad.ts
```

`ManifestKind` (`render.rs:1717`) has **six** members — `Promoted`, `Modified`, `Deleted`,
**`Added`**, `Untracked`, `CarriedOver`; five reach the commit set, `Untracked` tags the
left-out list. **There is no `ManifestKind::ALL`** — grep returns zero hits. So the fix's
first act is minting the registry.

**M2 · CONFIRMED.** `MIGRATING.md` names three triage keys. Driven:

```
$ jigc migrate-corpus --dry-run --format json | keys
['already_current', 'blocked', 'commit', 'dry_run', 'hook_output', 'migrated', 'unadopted', 'unfilled']
```

**Eight** envelope keys, **five** triage keys. `unadopted` and `unfilled` are declared in
`design/command-output-contract.md` and absent from the guide — and `unadopted`'s **emptiness
is the exit rule** (it is `STORE_EXIT_FLIPS` member 5).

**EC-24 · CONFIRMED (absence, by grep).**
`grep -in "ahead|newer jigc|downgrade|schema-version-ahead" MIGRATING.md QUICKSTART.md`
returns **zero hits**. The behaviour is documented only at `design/validation.md:453`, and
`design/` never ships. An adopter who downgrades a binary, or clones a repo written by a
newer jigc, meets blocking `schema-conformance.schema-version-ahead` (a `STORE_EXIT_FLIPS`
member, so it flips their CI to non-zero) with no verb-side repair and no paragraph anywhere
they can read.

**EC-21 · CONFIRMED.** Three install paths across two documents for step zero:
`QUICKSTART.md:19` `cargo install --path crates/cli`; `:20` `cp target/release/jigc
/usr/local/bin/`; `CLAUDE.md:10` `install -m755 target/release/jigc ~/.local/bin/jigc`.
None wrong; no single answer. `release_smoke.rs` fences the *loop commands*, not the install
stanza.

**EC-12 · CONFIRMED, and the behavioural half is driven.** Both guides say the discard door
makes no commit — `MIGRATING.md:40` (*"no commit, the committed store exactly as it was"*),
`QUICKSTART.md:180` (*"no commit is made and the committed store is untouched"*). Driven on a
**milestone sub-task**:

```
HEAD before: 520283b
$ jigc task discard measure-the-hit-rate --force
discarded task measure-the-hit-rate — dropped staged edits to: decisions-log:decisions-log
EXIT=0
HEAD after:  1152124
1152124 chore(milestone): discard task:measure-the-hit-rate on milestone:cache-rework
```

`cli::invocation_log::COMMITTING_DOORS` has **ten** members and `jigc task discard` is the
tenth. **A second law-1 gap found here, in no ledger row:** the ack itself names no commit —
the door lands a commit and its own success line is silent about it, where every other
committing door prints its sha.

### The axis for EC-19/22/23/24 — every guide sentence that names a verb's output or a key list

`QUICKSTART.md` + `MIGRATING.md`, the ones that state something checkable:

| site | claim | the set | verdict |
|---|---|---|---|
| `QUICKSTART.md:174` | `--dry-run` "prints the manifest and stops, changing nothing" | the three forecast gates | **FALSE unconditionally** (Q1) |
| `QUICKSTART.md` core loop | `task validate` and `finalize --dry-run` as one preview surface | `GATE_COVERAGE ▸ Previewed` (4) vs the dry-run's 3 | **FALSE** (Q2) — *and this region IS a `gate_coverage_fence.rs` site, which passes: the fence buys the member list, not the two-doors-agree claim* |
| `QUICKSTART.md:180` | `task discard` — "no commit is made" | `COMMITTING_DOORS` (10) | **FALSE** (EC-12, driven) |
| `QUICKSTART.md:19-20` | install stanza | `CLAUDE.md:10` | **contradicts** (EC-21) |
| `MIGRATING.md:35` | 4 manifest tags | `ManifestKind` (6 / 5-in-commit-set) | **FALSE — understates** (M1) |
| `MIGRATING.md` step 4 | 3 migrate-corpus triage keys | the 8-key envelope / 5 triage keys | **FALSE — understates** (M2) |
| `MIGRATING.md:39` | "**All nine** committing doors carry the frame" | `COMMITTING_DOORS` = **10** | **FALSE** |
| `MIGRATING.md:40` | `task discard` — "no commit" | `COMMITTING_DOORS` | **FALSE** (EC-12) |
| `MIGRATING.md:47` | wall-clock figures ("77 s", "under 7 s", "under 30 s", "800-item", "1500-item") | a machine, not a registry | **unpinnable by construction** — perf numbers in an adopter doc |
| both guides | the ahead-stamp direction | `STORE_EXIT_FLIPS` member 4 | **ABSENT** (EC-24) |
| `MIGRATING.md` step 1 | "It forecasts no green it would refuse" | — | **TRUE** — the correct statement of Q1's rule, in the wrong file |

**The reusable fix mechanism is M6's**: `foldback_truth.rs` already derives the guide set from
`setup.rs`'s `include_str!` sites and asserts per **prose unit** (a blank-line block, and each
list item as its own unit — so one gate's honesty is not paid for by its neighbour's). Adding a
`ManifestKind::ALL` / triage-key / `COMMITTING_DOORS` arm is the same shape, one registry each.

---

## 4 · The narrations

### EC-17 · `LeftoverShape` × the four `DESTROYING_DOORS` — CONFIRMED, driven at three doors; the fourth cell **cannot exist**

The registry: `cli::milestone::DESTROYING_DOORS: [&DestroyingDoor; 4]` (`milestone.rs:2514`) =
`PROVISION_DOOR` · `DISCARD_DOOR` · `UNINSTALL_DOOR` · `FINALIZE_DOOR` (the fourth carries
`code: None` — it never refuses).

**The root cause is two derivations of one subject that disagree about symlinks:**

- `probe_leftover` (`milestone.rs:~2640`) uses **`symlink_metadata(path).is_dir()`** → for a
  symlink-to-directory this is **false** → `LeftoverShape::File` → the refusal says
  *"the file itself — it is a file, not a worktree"*.
- `doomed_at` (`milestone.rs:5091`) uses **`path.is_dir()`**, which **follows** the link → the
  narration enumerates *through* it and calls it *"the leftover directory"*.
- `PendingLoss::narrate_taken` then filters lines by `symlink_metadata(&line.at).is_err()`;
  after the symlink is removed, `<symlink>/<child>` no longer resolves, so **every enumerated
  child is reported as taken** — while `remove_dir_all` took only the link.

Driven on a `committed-singletons` rig, symlink at the sub-task worktree path pointing at
`<repo>/docs` (three committed managed docs), sha256 before and after:

**Door 1 · `jigc milestone provision` — refusal then `--force`:**

```
$ jigc milestone provision padding-wave
blocking · milestone.leftover-holds-work — ... would delete 1 path(s) it cannot prove are disposable:
  .jigc/worktrees/add-the-padding-layer: the file itself — it is a file, not a worktree, ...
EXIT=1

$ jigc milestone provision padding-wave --force
warning: removing the leftover directory .jigc/worktrees/add-the-padding-layer discards work that is not in git:
    decisions-log.md
    milestone-records
    roadmap.md
  note: the leftover directory is the only copy of these bytes — they are not recoverable.
provisioned 1 worktree(s) ...
EXIT=0

sha256 docs/roadmap.md      503040b6… (UNCHANGED)
sha256 docs/decisions-log.md 38d887a3… (UNCHANGED)
ls docs/  → decisions-log.md  milestone-records  roadmap.md
```

Three of the repo's own committed, tracked docs named as unrecoverable; nothing was
destroyed; **and the same run calls the same path a *file* two lines up and a *directory*
two lines down.**

**Door 2 · `jigc uninstall --force` — identical narration**, byte-for-byte the same warning,
same three names, shas unchanged, `docs/` intact. (Its own subject derivation
`setup::fanout_worktree_paths` is correctly shape-blind since M49 — it does **not** filter on
`is_dir()`; the defect is entirely in `doomed_at`.)

**Door 3 · `jigc milestone discard --force` — prints nothing at all** about the leftover, and
leaves the symlink on disk. Its **refusal** route is honest here (*"a path nothing vouches for
is left behind on disk for you to deal with"*), so this door is *correct* and the other two
are wrong — three doors, three answers about one path.

**Door 4 · `jigc milestone finalize` — the cell is structurally unreachable.**
`remove_worktrees` (`milestone.rs:4337`) filters its subject against `git worktree list`'s
**registered** set and `continue`s on a miss. A symlink cannot be a registered worktree, so
finalize neither narrates nor destroys it — it survives the boundary in silence.
`DISCARD_DOOR` shares that function, which is why door 3 was silent.

**So the axis is not one axis.** It is `LeftoverShape × {regular file, directory,
symlink→file, symlink→dir, symlink-into-the-repo}` crossed with **two subject derivations**:
on-disk walk (provision, uninstall) vs registered set (discard, finalize). A fix that sweeps
only `DESTROYING_DOORS` will miss that half the doors ask a different question.

**The code-side set the narration should be derived from:** one probe. `probe_leftover`
already returns a `LeftoverHold { verdict, shape, entries }` that is exactly what a narration
needs; `doomed_at` re-derives all three with different predicates. Collapsing `doomed_at` onto
`probe_leftover`'s answer makes the refusal and the narration structurally incapable of
disagreeing — which is the property `hold_line`'s own doc-comment already claims for the three
refusing doors.

### EC-16 · the relocation ack over-claims its subject — CONFIRMED, driven

The sentence lives in **four** places, one of them a **test that pins it**:

- `crates/cli/src/config.rs:782` — the `docs-root` relocation ack (stderr)
- `crates/cli/src/config.rs:66` — the `jigc config set --help` doc-comment (§2)
- `crates/cli/src/config.rs:878` — the `placement-root` sibling ack
- `crates/cli/src/render.rs:3351` — the `jigc unmanage` ack, which restates it
- `crates/cli/tests/unmanage.rs:291-293` — **asserts the phrase is present**

Driven on `committed-singletons` + one `milestone create`:

```
$ git ls-files docs/
docs/decisions-log.md
docs/milestone-records/padding-wave.md
docs/roadmap.md

$ jigc config set docs-root project-docs
relocating 1 committed doc(s) stranded by the `docs-root` re-point to `project-docs`
  (every committed doc under the prior resolved root, managed or not; each move is a staged `git mv` …):
  - docs/milestone-records/padding-wave.md → project-docs/milestone-records/padding-wave.md
EXIT=0

$ git status --porcelain
R  docs/milestone-records/padding-wave.md -> project-docs/milestone-records/padding-wave.md
```

**Three committed `.md` under `docs/`, one moved.** Leaving the other two is *correct* —
`decisions-log` and `roadmap` are **placement** doctypes (`placement: { file: docs/… }`,
`location: None`) and resolve through `placement-root`, not `docs-root`. The parenthetical is
what is wrong, and it is the sentence an operator reads to decide whether the move was
complete.

**The code-side set the ack should be derived from:** `stranded_under_docs_root` already walks
`old_schemas.values()` filtered to `location:` doctypes. The ack should name *that* subject
("every committed doc of a `docs-root`-homed doctype"), and the CLAUDE.md-named cross-cutting
gotcha — *every `schema.location` consumer must also handle the placement case* — is the axis:
every "every X" clause in a relocation, promotion or teardown narration, checked against the
set the code actually walks.

### EC-15 · the join summary's `no docs staged from:` — CONFIRMED on both blocking classes, driven, with a mixed control

The derivation is `render::doc_less_sub_tasks` (`render.rs:~3840`), reading
**`outcome.overlay`**. Its own doc-comment: *"One derivation for both surfaces, so the text
and the wire cannot disagree about who staged nothing."* They agree with each other and both
disagree with the disk.

The engine (`milestone.rs:2096`) `continue`s a clashing group **before** it can `claim` into
the overlay, so a sub-task blocked out of a group is invisible to the overlay — and therefore
counted as having contributed nothing.

**Class A — `join.same-doc-clash` (both sub-tasks edit `vision:vision`):**

```
$ cat .jigc/tasks/tune-the-eviction-policy/docs/provenance.json
{ "docs": { "vision:vision": "edited-from-base" } }
$ cat .jigc/tasks/measure-the-hit-rate/docs/provenance.json
{ "docs": { "vision:vision": "edited-from-base" } }
$ jigc doc list --task tune-the-eviction-policy   → vision:vision  VISION.md  managed
$ jigc doc list --task measure-the-hit-rate       → vision:vision  VISION.md  managed

$ jigc milestone join cache-rework
blocking · join.same-doc-clash — sub-tasks [measure-the-hit-rate, tune-the-eviction-policy] each write `vision:vision` …
join blocked: milestone:cache-rework — 1 blocking finding(s); 0 doc(s) would merge, nothing committed
  no docs staged from: measure-the-hit-rate, tune-the-eviction-policy      ← FALSE, both staged it
EXIT=1

$ jigc milestone join cache-rework --format json
  "no_docs_from": ["measure-the-hit-rate", "tune-the-eviction-policy"],
  "overlay": {},
```

**Class B — `combine.code-collision` (distinct docs, one shared code file):**

```
$ jigc milestone join cache-rework
blocking · combine.code-collision — `src/cache.ts` (sub-tasks [measure-the-hit-rate, tune-the-eviction-policy]) …
join blocked: milestone:cache-rework — 1 blocking finding(s); 2 doc(s) would merge, nothing committed
  - decisions-log:decisions-log  (edited-from-base · from measure-the-hit-rate)
  - vision:vision  (edited-from-base · from tune-the-eviction-policy)
EXIT=1
```

Overlay computed, no false line. **The two blocking join classes disagree about whether the
overlay is populated under a block.**

**Mixed control (a third sub-task staging a distinct doc):**

```
join blocked: … 1 doc(s) would merge, nothing committed
  - decisions-log:decisions-log  (edited-from-base · from log-the-evictions)
  no docs staged from: measure-the-hit-rate, tune-the-eviction-policy      ← still FALSE, and now precise
```

So the misreport is **per blocked group**, not a global short-circuit — which pins the axis
exactly: *every short-circuit path that renders a summary derived from an overlay it did not
compute*. The set the narration should be derived from is the **staged-doc set per sub-task**
(the `provenance.json` the join already reads to build its groups), not the post-merge overlay.

---

## 5 · Hand-written counts (EC-14) — doc:line · stated · actual · registry

Every "actual" re-derived at HEAD from the named registry.

| doc:line | stated | actual | the registry that defines it | verdict |
|---|---|---|---|---|
| `MIGRATING.md:39` | "All **nine** committing doors" | **10** | `cli::invocation_log::COMMITTING_DOORS` (`invocation_log.rs:130`) | **STALE** |
| `CLAUDE.md:7` | "the whole **nine-door** committing axis" | **10** | same | **STALE** |
| `design/worked-examples.md:3005` | "**9** doors, **9** distinct identities" | 10 / 11 | `COMMITTING_DOORS` / `ERROR_CODE_REGISTRY` | **STALE** (= carried defect **(g)**) |
| `crates/cli/tests/flow47_acceptance.rs:21` | "all **nine** pairwise distinct" | 10 | same — and `:18` of the **same file** says "10 doors since M49 Inc 2 T3" | **STALE, self-contradicting four lines apart** |
| `implementation/decisions-pending.md:320` | "drove all **nine** committing doors" | 10 | same | **STALE** (historical framing, but present-tense verb) |
| `CLAUDE.md` (M50 para) | "**thirteen** `CELLS` rows" | **63** | `crates/cli/tests/support/write_miss_cells.rs::CELLS` | **FALSE — and it was false when written**: at `32de1121`, the very commit that wrote the sentence, `CELLS` already had 63 rows |
| `design/doc-read-surface.md:175` (heading) + `:177` | "**five** regimes" / "**Five** independently-governed … regimes" | **6** table rows | the table two lines below it (`doc show`, `doc list`, `doc schema`, command-output contract, result contract, `describe`) | **STALE — the doc contradicts itself in one screen** |
| `design/corpus-migration.md:281` | "all **sixteen** shipped doctypes (**6/6** dev + **10/10** methodology are manifest-listed)" | dev **6**, methodology **11** (17 entries, 16 distinct types) | both `schema-manifest.yaml`s | **STALE in the parenthetical** (`planning-record` joined at M49); the headline "sixteen" is right by distinct-type count and wrong by entry count — the sentence is ambiguous about which it means |
| `crates/cli/pack/config/schema-manifest.yaml:48` | "all **16** doctype hashes" | 17 entries today | both manifests | **historical** (a true statement about M47 Inc 1), but reads as a live count in a file an author edits |
| `packs/methodology/config/schema-manifest.yaml:79` | same sentence, verbatim | same | same | **same** — and it is the *methodology* manifest restating the *dev* manifest's history: one fact, two homes |
| `design/surface-contract.md:129` | "member-for-member — **eleven**" | **11** rows in the table, **11** in `ERROR_CODE_REGISTRY` | `cli::invocation_log::ERROR_CODE_REGISTRY` (`invocation_log.rs:191`) | **TRUE today — but UNFENCED.** *This corrects EC-14, which lists it as stale; it is not.* The registry test hardcodes `11` and names this file in its assert message while no test reads it (= carried defect **(h)**) |
| `design/write-commands.md:94` | "the **nine-cell** `{set-field, set-slot, doc author} × {section leaf, top-level item leaf, nested item leaf}` table" | **9** `ROWS` | `crates/cli/tests/undeclared_address_writes.rs::ROWS` | **TRUE today, unfenced** |
| `CLAUDE.md`, `dev-workflow.md`, `machine-setup.md`, `.claude/agents/build-executor.md`, `.claude/agents/increment-validator.md` | "**twelve** group targets" | **12** `[[test]]` in `crates/cli/Cargo.toml` | the Cargo manifest | **TRUE today; the fence is blind** — `test_target_registration.rs:355` builds its needle as a **decimal** (`format!("{} group targets", …)`) and checks 2 files, so all five spelled-out sites are unreachable by it (= carried defect **(f)**) |
| `MIGRATING.md:35` | 4 manifest tags | 6 `ManifestKind` (5 in-commit) | `cli::render::ManifestKind` — **no `::ALL` exists** | **STALE** (= EC-22) |
| `MIGRATING.md` step 4 | 3 triage keys | 8 envelope / 5 triage | the pinned envelope | **STALE** (= EC-23) |
| `jigc doc show --help` | 4 pinned keys | 6 | `design/doc-read-surface.md:59/62` | **STALE** (= EC-11 D6) |
| `design/doc-read-surface.md:90` + `command-output-contract.md:446` | both headed "**The window closes here** — (M48, the rc.11 wave)" | a **later declared spend** sits below each (M49) | — | **presentation, not a lie** — the paragraph itself says the close is keyed to the 1.0 pin, not a wave name, and the M49 spend is explicitly declared. *Softens EC-14's reading of this item.* |
| `MIGRATING.md:47` | "77 s", "under 7 s", "under 30 s", "800-item", "1500-item" | wall-clock, machine-dependent | none | **unpinnable by construction** — perf figures in an adopter-facing shipped guide |

**The class, stated for the fix:** a number over a set the code can move. Two shapes are
available and both already ship — **generate it** (M1/M2: the sentence is rendered from the
constant) or **fence it** (M7: a test reads the registry and asserts the doc's rows). A third,
cheapest option is **iterate rather than count** (`worked-examples.md:3066`'s shape). Note the
count-fence must also handle the **historical** claims (the two manifest headers, the
`decisions-pending` row): those must be dated-bracketed, not re-pinned — the discipline
`foldback_truth.rs::dated_correction_spans` already implements.

---

## 6 · N15 and N23 — both driven

### N15 — CONFIRMED, and worse *and* better than recorded

**Exact surface:** `jigc doc show <miss-address> --task <id>`, `store.not-found` arm.

```
$ jigc doc show 'vision:vision#thesis' --task tune-the-eviction-policy
The eviction policy is now LRU with a 2-minute floor.          ← the staged copy exists

$ jigc doc show 'vision:wrong-slug' --task tune-the-eviction-policy   2>err_task
$ jigc doc show 'vision:wrong-slug'                                   2>err_plain
$ diff err_task err_plain
IDENTICAL BYTES
```

Both print:

```
blocking · store.not-found — `vision:wrong-slug` names no committed doc: `vision` is a singleton, so its only address is `vision:vision`
  at: vision:wrong-slug
  route: read `vision:vision` — a singleton doctype has one instance at a fixed slug
```

Two lies in one block under `--task`: the word **committed**, and a route that **drops
`--task`**.

**The route's consequence, driven verbatim:**

```
$ jigc doc show 'vision:vision#thesis'
A deterministic CLI assembles exactly the context a task needs.      ← stdout: the COMMITTED prose
note: `vision:vision` is also staged in open task tune-the-eviction-policy — this read served
the committed copy, so any edits staged there are not shown; …       ← stderr
```

**A mitigation the ledger does not record, and why it does not save it:** there *is* a
`note:` — but it rides **stderr**, and `jigc doc show --help` explicitly tells the reader
*"Stdout carries the addressed content and nothing else … so a read redirects or pipes
straight into a file. Diagnostics, blocks, and the staged-elsewhere note ride stderr."* The
help invites exactly the disposal that loses the mitigation.

**The sibling arm that is already correct** — `store.not-staged`, same verb, same `--task`:

```
$ jigc doc show 'decisions-log:decisions-log' --task tune-the-eviction-policy
blocking · store.not-staged — `decisions-log:decisions-log` is not staged in this task — only its committed copy exists
  at: decisions-log:decisions-log
  route: `jigc doc show decisions-log:decisions-log` — the task-less read serves the committed copy
```

It names the copy it is about, and its route **explains which copy it serves**. So the defect
is one arm of one resolver on a **1.0-pinned** read surface, and the correct shape is three
inches away in the same file.

### N23 / EC-38 — CONFIRMED, driven

**State:** `vendored` rig (committed, tracked `docs/architecture/padding-layer.md` +
`docs/specs/padding.md`), then `JIGC_PACK_DIR` pointed at a dev-pack copy with
`schemas/arch-doc.yaml`, `workflows/architecture-documentation.yaml`,
`workflows/migrate-arch-doc.yaml` and the manifest's `arch-doc` entry removed.

```
$ jigc validate
no findings — the committed store validates clean
EXIT=0                                                     ← not one word about the orphan

$ jigc doc list
id             path                    state
spec:padding   docs/specs/padding.md   managed             ← the arch-doc row is GONE, not `unregistered`

$ jigc migrate-corpus --dry-run
corpus migration (dry run — nothing written): 0 would migrate, 0 already current, 0 blocked

$ jigc doc show arch-doc:padding-layer
blocking · store.unknown-type — unknown doctype `arch-doc` for `arch-doc:padding-layer`
  route: list the available doctypes with `jigc describe`
EXIT=1                                                     ← the one surface that tells the truth

$ git ls-files docs/
docs/architecture/padding-layer.md
docs/specs/padding.md
$ shasum -a 256 docs/architecture/padding-layer.md
02f91670…                                                  ← UNCHANGED
```

**Registration loss, not byte loss** — the ledger's classification is right. Its consequence
is a **false green on the verb `MIGRATING.md` tells adopters to CI-gate on**, over a corpus
whose bytes are all still tracked. Reachable by an ordinary adopter act since PB-1 (M49) lets
a project own its own doctypes. **A control worth recording:** a project-*listed* pack that
defines no doctypes does **not** reproduce it — composition means the embedded pair still
provides `arch-doc` and `doc list` shows both rows. The state needs a pack that *replaces*
the dev pack (`JIGC_PACK_DIR`) or a project pack that drops a doctype it owns.

---

## 7 · Known deferred / latent in this area (from `implementation/decisions-pending.md`)

The four carried rows that are **exactly** this area's shape (Tier 2/3 of the M49 close's
carried defects, `:144-156`) — all re-checked at HEAD:

| id | claim | HEAD check |
|---|---|---|
| **(f)** | the "twelve group targets" fence is blind twice over — decimal needle, 2 files checked, 5 spelled-out sites unreachable | **holds** — `test_target_registration.rs:355` `format!("{} group targets", …)`; `Cargo.toml` has 12 `[[test]]`; 5 prose sites spell "twelve" |
| **(g)** | `COMMITTING_DOORS` 10 in code, 9 in `worked-examples.md:3005`, and `flow47_acceptance.rs` says both at `:18`/`:21` | **holds verbatim at HEAD** |
| **(h)** | `ERROR_CODE_REGISTRY`'s doc mirror (`surface-contract.md`) is unfenced; the registry test hardcodes 11 and names that file in its assert while nothing reads it | **holds** — and the mirror is currently *accurate* (11 = 11), which is what makes it (g)'s twin on the next door added |
| **(m)** | `dev-workflow.md:13` heads "all four pass" over five bullets; `:22` says "the five commands above, in order" — false on the order too | **holds** — `dev/gate` runs probe · fmt · clippy · build · test; the doc lists fmt · clippy · probe · test · build |

Adjacent, same class, also carried: **(j)** `doctype-authoring.md:30`'s forward-looking
"Reading this before rc.6 ships" caveat (rc.6 = M42, seven waves ago) sitting directly above
the transform-kind matrix a doctype author reads *at the moment of bumping a schema*;
**(k)** `pinning.md:15`'s present-tense claim that `lib.rs` exposes neither the pack registry
nor the clap tree (both are `pub mod` since M45, and the module doc cites that very section as
its reason); **(l)** `roadmap.md:839`'s superseded `jigc doc rename` charter, now a plausible
false description of a verb that shipped at M48, with its retraction *below* it.

**Deferred-by-design in this area:** the **surface style guide's judgment tier**
(`design/surface-contract.md` → *The surface style guide*) — explicitly the tier no fence
holds; and the **named-fact guard's A-3 bound** (`pack.rs:1235`) — presence of a named fact,
never prose quality, register or order, stated in the const's own doc-comment.

---

## 8 · The ledger, by class

**Built + proven** (verified by exercise, for the shapes in use)
- M1 the seam-generated slug statement — three sites, byte-fenced (`help_truth.rs`)
- M2 the seam-generated heading-ceiling statement — rendered in composed steps (driven:
  `jigc workflow record-change --preview`)
- M3 the `{{schema:<T>}}` payload skeleton — driven: `jigc workflow migrate-changelog --preview`
  renders per-address reserved depths from the schema
- M4 the `states-constraints:` pack-load fence, biconditional
- M5 the named-fact guard (`CONSTRAINT_REQUIRED_TOKENS`, 6 rows + `COPY_IN_APPEND_TOKENS`)
- M6 `foldback_truth.rs` — incl. the **derived** shipped-guide set
- M7 `doctype_map_versions.rs` — the count-fence exemplar
- M8 `planning_gate_home.rs` — the single-home rule, subject read from the shipped schema
- M9 `gate_coverage_fence.rs` — 8 prose sites against `GATE_COVERAGE`
- M10 `read_surface_naming.rs` — incl. the installed `SKILL.md` read off disk
- `setup.rs`'s guide composition + `jigc-body-blake3:` stamp
- `uninstall --help`'s "three states", `milestone discard --help`'s "both guards",
  `doc author --help`'s three-plus-one, `write-commands.md:94`'s nine cells,
  `surface-contract.md:129`'s eleven, the "twelve group targets" — **all numerically correct
  at HEAD**

**Shape-limited** (works for the proven shape, not a plausible adjacent one)
- **`help_truth.rs`** — proves 8 specific help texts by hand-written token; the **unexercised
  shape** is a help sentence that states a *count or scope* over a code-side set. Only one arm
  iterates an axis, and it iterates for a *leaked Rust path*, not for a false count.
- **`gate_coverage_fence.rs`** — reaches QUICKSTART and two design docs; the **unexercised
  shape** is `jigc task finalize --help`, which states a *different, unregistered* three-gate
  set and is not a site. That is EC-20's mechanism.
- **`foldback_truth.rs`'s guide arm** — derives the guide set, asserts per prose unit; the
  **unexercised shape** is a guide sentence enumerating a *code-side enum* (`ManifestKind`,
  the triage keys, `COMMITTING_DOORS`, `STORE_EXIT_FLIPS`).
- **`DESTROYING_DOORS` as an axis** — the four doors run **two** subject derivations (on-disk
  walk vs `git worktree list`); the **unexercised shape** is a symlink, which one derivation
  enumerates through and the other cannot see at all.
- **`doc_less_sub_tasks`** — one derivation for text and wire, proven on the merging path; the
  **unexercised shape** is a **blocked group**, where the overlay it reads was never computed.

**Latent defect** (green in tests, broken in real usage)
- **EC-15** — the join summary and its `no_docs_from` key are false on `join.same-doc-clash`;
  the fixture suites drive the clash and assert the *finding*, never the summary line.
- **EC-16** — the relocation ack's "managed or not" universal is over-claimed, **and
  `unmanage.rs:291` asserts the false phrase is present** — a test that pins the lie.
- **EC-17** — two doors narrate a destruction they do not perform and name tracked committed
  files as unrecoverable; a third is silent; a fourth cannot reach the cell.
- **N15** — the `--task` miss arm says *committed* and routes without `--task`, on a 1.0-pinned
  read surface, with its mitigating note on the stream the help says to discard.
- **N23/EC-38** — a retired doctype's committed corpus is orphaned at exit 0 with a
  *"validates clean"* line.
- **EC-12** — both shipped guides say `task discard` makes no commit; it is the tenth
  committing door, driven (HEAD moved). **New, in no ledger row:** the door's own success ack
  names no commit either.
- **EC-11 D4/D5/D6, EC-19 Q1/Q2, EC-22, EC-23, EC-24, EC-21, EC-25, EC-14** — see §2/§3/§5.

**Deferred-by-design**
- The surface style guide's judgment tier (`design/surface-contract.md`) — no fence, by design.
- The named-fact guard's A-3 presence-never-content bound (`pack.rs:1235`).
- `MIGRATING.md:47`'s wall-clock figures — unpinnable by construction; a decision about whether
  perf numbers belong in a shipped adopter guide, not a fence.
- The conversion ledger (`pinning.md` §3 refuses a `pinned-by:` symbol parser by name) — every
  citation is verified by reading test content.

---

## 9 · Claim-vs-reality corrections to the inputs I was handed

Three, each with the falsifying datum:

1. **EC-14 lists `surface-contract.md:129`'s "eleven" as a stale count. It is correct.**
   `ERROR_CODE_REGISTRY` (`invocation_log.rs:191`) has 11 members and the table at
   `surface-contract.md:131-142` has 11 rows. The real defect there is carried defect **(h)** —
   it is *unfenced*, not wrong.
2. **EC-14's "both posture homes headed *The window closes here — (M48)* immediately above two
   later declared spends" reads as a lie; it is not.** Both paragraphs state in their own text
   that the close is keyed to **the 1.0 pin, not a wave name**, and the M49 spend below each is
   explicitly declared as *"The last spend before the pin."* Presentation, not law 1.
3. **EC-17's axis is stated as `LeftoverShape × {shapes} at all four DESTROYING_DOORS`; the
   fourth cell cannot exist.** `remove_worktrees` (`milestone.rs:4337`) filters on
   `git worktree list`, and a symlink is never registered — so `milestone finalize` and
   `milestone discard` both `continue` past it. A fix scoped to the four-member registry will
   miss that the doors run **two different subject derivations**.

---

## 10 · Bounds of this ledger

- `DESTROYING_DOORS` door 4 (`milestone finalize`) was reached by **call path, not by drive** —
  I drove `milestone discard`, which shares `remove_worktrees` and the `registered` filter, over
  the identical planted state and observed the predicted silence.
- The `LeftoverShape` axis was driven for **symlink→directory** only. `{regular file,
  directory, symlink→file, symlink-into-the-repo}` were not driven.
- EC-14's *"three `DECISIONS.md` citations into the posture homes now point at blank lines"*
  was **not checked** — out of time, and it is a record claim rather than a shipped surface.
- The `migrate-corpus --help` "v0→v1" falsity is established from the manifests and
  `SchemaChangeKind::ALL`, **not** by driving an actual multi-kind migration.
- N23 was reproduced through `JIGC_PACK_DIR` (a pack *replacement*), not through a project-pack
  doctype retirement; the control showing a listed empty pack does **not** reproduce it is
  recorded above.
- No `cargo test` was run; every finding is against the release binary at HEAD.
