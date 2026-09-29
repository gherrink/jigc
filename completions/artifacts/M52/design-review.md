# M52 — independent pre-decompose design review

**Verdict: NOT READY.** 4 blocking · 7 significant · 10 advisory. Reviewed 2026-09-17 against
HEAD `7637a46f` and the installed release `~/.local/bin/jigc` = `1.0.0-rc.15`. Fixtures via
`dev/jigc-rig <state> --binary ~/.local/bin/jigc`; nothing written into the working repository.
Each finding is marked **driven** (executed on the release binary) or **read** (source at the
cited line). I authored none of what I review and I settle nothing.

Reviewed: `completions/artifacts/M52/settle-record.md`, `acceptance-design.md`, `DECISIONS.md`
→ 2026-09-17; evidence base `baseline-ledger.md` + 7 companions, `gap-findings.md` + 4
companions, `advocates/*.md`, the charter at `implementation/decisions-pending.md:22-100`, the
M51 per-axis review; invariants `CLAUDE.md`, `design/{finalize,storage,command-output-contract,
doc-read-surface,surface-contract,team-ready-state,validation,reconciliation,corpus-migration}.md`,
`implementation/pinning.md`, `completions/artifacts/M51/settle-record.md`.

**What held.** The zero-schema-hash claim is sound (`crates/engine/src/manifest.rs:76-89` —
`erase_presentation` exhaustively destructures `Schema`; no decision touches a `Schema` field;
the five shipped `singleton: true` doctypes are all `placement:` so D5 moves no schema). The
pre-pin window admits D6.1's `findings` key and D3.3's `displaced` key
(`design/command-output-contract.md` → *Evolution posture*, the additive + reshape rules).
`parse_verb_addr` really is the one funnel for nine `doc.rs` doors (`crates/cli/src/doc.rs:6120`
+ 9 call sites, read). `projection_home_line` really answers all three home shapes
(`crates/engine/src/compose.rs:1194-1215`, read). `operational_failure(format, &err)` really is a
one-line substitution at the 24 `current_dir()` sites — all hold `format` and return `Outcome`
(`crates/cli/src/cli.rs:540,695,…,1610`, read); `refuse_on_posture`'s `?` really can become
`Some(...)` in place (`cli.rs:534-559`, read). The M45 history-gate predicate exists and is
CLI-side (`crates/cli/src/task.rs:5268`), so D7 can call it. `Route::mechanical` placeholders
survive both fences (`strip_unemitted`, `crates/engine/src/finding.rs:926-938`; `migrate.rs:195`
ships one today) — D9's mechanism transfers. D5 landing at `parse_verb_addr` does **not** redden
the `doc_read_surface` law-1 fence: that fence drives advertised addresses through
`engine::address::Address::parse` (`crates/cli/tests/doc_read_surface.rs:1022`), which D5 leaves
untouched — the concern is checked and does not hold.

---

## BLOCKING

### B1 — D1.4's `MintedSet` under-counts the `milestone create` area; the decided discipline
false-blocks every rejected run and bricks the re-run it prescribes. **DRIVEN.**

`settle-record.md:79-80` (the advocate's warrant: *"a mint writes exactly three constant files
(`state.rs:633-649`)"*) and `:107-110` (`MintedSet(<the mint's own constant list>)` at **both**
`unwind_mint` sites). Driven on `committed-singletons` with a pre-commit hook that lists the
area and exits 1, the area at the moment `unwind_mint` runs at `jigc milestone create` holds
**four** files:

```
.jigc/milestones/probe-the-mint-area/staged-snapshot.json
.jigc/milestones/probe-the-mint-area/record-commit-msg.txt
.jigc/milestones/probe-the-mint-area/base.json
.jigc/milestones/probe-the-mint-area/tasks.json
```

`mint_milestone` writes **two** (`crates/engine/src/milestone.rs:283-296` — `base.json`,
`tasks.json`); the door writes `staged-snapshot.json` three lines later
(`crates/cli/src/milestone.rs:562-568`) and `materialize_and_commit_record` writes
`record-commit-msg.txt` (`milestone.rs:737`) before the hook runs. So under D1.4 the
non-recursive `remove_dir` returns `ENOTEMPTY` over **jigc's own bytes** on *every* rejected
`milestone create`, not on a raced one: a blocking finding naming a third party that is not
there (a law-1 lie the wave exists to close), and — because `mint_milestone` refuses a
pre-existing area (`milestone.rs:279-281`, `milestone.serial-collision`) — the recovery the
door's own frame prescribes (*"Fix the hook's complaint, then re-run `jigc milestone create …`"*,
driven verbatim) is permanently blocked. D1.4's stated honest cost (*"a re-opened
serial-collision on the **raced** re-run"*) is therefore understated by the ordinary path.

The sibling **does** transfer: driven at `jigc milestone add-task`, the sub-task area holds
exactly `intent` · `workflow` · `base.json` — `mint_task`'s three (`state.rs:620-649`) — and
`unwind_unrecorded_seeds` delegates per area (`milestone.rs:1562-1580`), so N sub-tasks is N ×
three, not an N-file set. The prompt's over-design worry there is unfounded.

*Fix:* the `MintedSet` row's member set is the **area's jigc-written set for that door** (mint ∪
the door's own writes), not the mint's constant list — or the discipline becomes *remove exactly
what this transaction wrote*, recorded as it writes. Note the milestone area is a different home
from D3.1's `TASK_AREA_FILES`, so no registry in the Settle covers `staged-snapshot.json` +
`record-commit-msg.txt` at `.jigc/milestones/<id>/`; the registry that feeds `MintedSet` has to.

### B2 — D2.6's `task validate` posture row contradicts `Door::Previewed`'s own code-side
statement, and the "declared exit flip" never names the exit. **READ + DRIVEN.**

`settle-record.md:170-174`. `Door::Previewed`'s doc-comment is *"Previewed at `jigc task
validate` and re-run at `finalize`, same check, same severity, **same exit code**"*
(`crates/cli/src/gate_coverage.rs:91-94`). A posture refusal at `finalize` goes through
`refuse_on_posture` → `operational_failure` → `Outcome::with_findings(EXIT_ERROR, …)` =
exit **1** (`cli.rs:555-558`; `task.rs:102`) — driven at a sibling door (`jigc milestone execute`
on an un-set-up repo, exit 1). A blocking preview finding at `jigc task validate` is exit **3**
(`task.rs:114` `EXIT_VALIDATION_BLOCKED`; `command-output-contract.md` → the exit taxonomy,
*3 = blocking findings at a task-scope gate*, *never* an operational reject). So a
`Door::Previewed` posture row ships **two exits for one condition** and falsifies the doc-comment
the `GATE_COVERAGE` fence blesses.

*Fix:* the Settle names the exit `task validate` takes for the posture row and reconciles it:
either a `Door::FinalizeOnly(LaterPhase)` row (states the member, no flip, no contradiction), or
exit 1 at `task validate` for this row with the divergence written into `gate_coverage.rs`'s
statement, or exit 3 with the finalize side moved — each is a different build. As written the
build resolves the fork.

### B3 — D7's `home-vacated` subject is a fork with a false-fire arm, and it is a new blocking
`STORE_EXIT_FLIPS` member. **READ.**

`settle-record.md:349-356`: *"at each resolved doctype home whose committed history is non-empty
… while the index holds nothing there"*, then *"It names one exact path per declared home"*. The
two halves scope differently. Its evidence is the **placement** case (C-2's `git mv CHANGELOG.md
HISTORY.md`, an exact file). Read over **every** resolved home, the 12 located doctypes' homes are
*directories* (`docs/decisions/`, `docs/specs/`, …), and *"the index holds nothing there"* is true
of any corpus that has zero instances of a doctype it once had — `git log HEAD -1 --
docs/decisions/` is non-empty after the last ADR is legitimately retired or `jigc unmanage`d. That
corpus turns `jigc validate` **exit 1** on a blocking finding routed at `jigc ingest`/`rename`,
with no doc to ingest. This is adopter-visible and it is the seventh `STORE_EXIT_FLIPS` member.

*Fix:* state the scope — placement/exact-file homes only (the evidence's scope, and what *"one
exact path per declared home"* means), or every home **with** the zero-instances carve-out stated
— plus the check's exit and its route per arm.

### B4 — D3.1's subject has no cell for the `docs/` subtree, and the cited registry is 12
filenames. **READ.**

`settle-record.md:198-204`: *"The destroying-door subject in a task area is the complement of that
set"*, where the set is the 12 cited **filename** consts (verified: `state.rs:32,37,42,51,60,68,
77,215,788` = `base.json`·`roles.json`·`renames.json`·`intent`·`workflow`·`source-path`·
`slug-override`·`provenance.json`·`staged-snapshot.json`; `validate.rs:66,71` =
`probe-snapshot.json`·`base-probe-snapshot.json`; `migrate.rs:63` = `source`). The area's
`docs/` **directory** is in none of them, and it holds the staged `<type>:<slug>.md` docs plus
`docs/provenance.json`. Taken literally the complement flags `docs/` as a foreign byte at every
door on **every** task — the 100 %-false-fire the fork was re-posed to avoid, one level up. And
the C-1 class's own cell is left unadjudicated: a non-`.md` **inside** `docs/` (`docs/notes.txt`),
and a `docs/*.md` that is not a staged doc id — which is exactly the `staged_doc_ids` seam D3.6
touches (L-3).

*Fix:* state the set as **area-relative paths** including the `docs/` subtree, and say how a
`docs/` member that resolves to no staged doc id is classified. Without it the guard either
blocks every task or silently keeps the reported class open.

---

## SIGNIFICANT

### S1 — D5.2's four landing sites: two hold no schema at the cited line. **READ.**
`settle-record.md:284-287`. `rename::parse_addr(addr: &str)` takes no pack
(`crates/cli/src/rename.rs:970-987`), and `milestone.rs:1410`'s guard is deliberately *"Ahead of
`jigc_home`, the workflow check and every mutation"* (`milestone.rs:1397-1409`) — the cascade is
not resolved, so no `Schema` is in hand for `placement.is_some() || singleton`. `parse_verb_addr`
(holds the pack) and `TaskArea::bind` (holds `self.pack`) are fine. The check is implementable
at the callers (`rename.rs:334`; after `jigc_home_or_repo` in `add-from-spec`) or by loading the
pack earlier — but the record's line-cited landing is wrong on two of four, and the plan will cut
tasks at those lines. This is the M51 shape the pre-decompose review exists to catch.

### S2 — a dependency order the *Owed at decompose* ordering inverts. **READ.**
`settle-record.md:460-467` orders risk-first *"D1 and D3's loss cells and D2's exit-0 conclusions
first"*, while D1.3/D1.7 carry their findings *"on the reject envelope (D6)"* (`:105-106`,
`:118-121`) and D2.4 lands *"inside D6's pre-dispatch funnel"* (`:165-167`). Built in the stated
order, D1's `<door>.rollback-conflict` has no machine surface at the moment it is minted. State
the order that must hold: **D6.1 + D6.3 ⊑ D1.3/D1.7 and D2.4** · **D1.1 ⊑ D1.2/D1.3** ·
**D3.1 ⊑ D3.2/D3.3** · **D5.1/D5.2 ⊑ D8's `rename --slug`** · **D4.1 ⊑ D4.5/D4.6**.

### S3 — D4.2's fail-closed site must be named, and its sibling exempted. **READ.**
`settle-record.md:250-253` names *"the `.filter_map(|k| load_prior_schema(…).ok())`"*. That binder
disambiguates `crates/cli/src/migrate_corpus.rs:2103` (inside `candidate_docs`) from
`crates/cli/src/pack.rs:199-212` (`prior_doctype_schemas`, binder `|version|`) — but only to a
reader who checks. `pack.rs:199-212` is a **declared** best-effort feeding the read-only store
sweep's managed-vs-foreign classifier: *"never an error — the sweep is read-only and must not
fail on a pack that ships no snapshot store."* A fixer told to sweep the class (this repo's
standing rule, and the reason five consecutive audits widened their findings) would redden
`jigc validate` on every manifest-less or project pack — contradicting
`schema-conformance.unversioned-doctype`'s own advisory rationale. Say the sibling is exempt and
why, in the decision.

### S4 — D3.5's registry shape cannot express what D3 decides. **READ.**
`settle-record.md:217-221`. `DestroyingDoor { verb, code: Option<&str> }`
(`crates/cli/src/milestone.rs:2589-2628`) carries **one** code, and the discriminator is
refuse (`Some`) vs narrate (`None`). D3 gives `task discard` **two** refusal causes
(`task-discard.staged-prose`, shipped, plus the new `task-discard.foreign-bytes`), and gives
`task finalize` a **third disposition** — *displace* — which the refuse/narrate axis has no cell
for and which arm 3's per-cell assertion branches on. Name the row shape (a code *set*; a
`Disposition` with a `Displaces` member) or `flow49_acceptance.rs:999-1051` grows against a
matrix the code cannot express.

### S5 — D5.4 spends a one-way `contract-version` 6→7 without naming the keys. **READ.**
`settle-record.md:293-294`. `projection_home_line` is a **prose** renderer consumed by the
`{{schema:…}}` text projection (`compose.rs:1114`); `doc schema --format json` is a separate
serde struct (`crates/cli/src/doc.rs:4569-4794`, `contract_version: 6`). *"Projects the
identity/home fact"* is not a key set, and `doc-read-surface.md:108` forbids additive carve-outs
after the pin — this is the last wave that can name them for free. Arm 5's per-cell assertion
(*"projects the identity/home fact for every doctype"*) is not writable as it stands. Settle the
key names, types and the `base` compound's shape here.

### S6 — D8's LD-3 answer ships eight code-less refusals, against the wave's own claim. **DRIVEN.**
`settle-record.md:362-366` reuses `locate::not_set_up()` (`crates/cli/src/locate.rs:52-57`), a
bare `anyhow!` with a route in prose. Driven on the `bare` rig: `jigc milestone create Probe`
mints at **exit 0** with no `jigc setup` (LD-3 confirmed), and `jigc milestone execute probe`
answers `this project isn't set up — run `jigc setup` …` at exit 1 — **no code, no `at:`, no
findings envelope** — where the same door's sibling refusal (`milestone.unknown`) is a coded,
located, routed finding (driven side by side). The claim reads *"every route a caller can reach
answers with a **code** and a followable route"*, and D6 is spending the pre-pin window precisely
to get identities onto the machine surface. *Fix:* name the code, severity and exit for the
not-set-up refusal. Rider: LD-4 (an unreadable `.jigc/` reported as *"not set up"*, baseline
contracts §4) is the **same producer** — D8 routes it to a tier-2 one-liner, so unless the two
land together the lie propagates from one door to eight.

### S7 — acceptance arm 5's derivation has no witness for half its predicate. **DRIVEN (census).**
`acceptance-design.md:23`: *"the fixed-identity doctype set (derived from both packs' schemas:
`placement.is_some() ‖ singleton`, stated as a derivation)"*. Censused over the shipped packs,
every `singleton: true` doctype is also `placement:` — `changelog`, `decisions-log`,
`deferral-ledger`, `roadmap`, `vision`, and no other — so the derivation enumerates the placement
set and **never** exercises the `singleton`-only disjunct, which is the cell G-21 was driven on
(a manufactured `adr` with `location:` + `singleton: true`). Add a manufactured cell (arm 4
already manufactures packs with `--pack-from-dev --schema … --repin`) or the new disjunct ships
unpinned and `storage.md:216`'s revised converse is asserted by nothing.

---

## ADVISORY

- **A1** — `settle-record.md:96-98`: *"the ~150-line `restore` body (`task.rs:4190-4243`)
  unchanged"*. It is **51 lines** (`crates/cli/src/task.rs:4192-4242`), and it cannot be literally
  unchanged under D1.3: it calls `rollback_conflict_finding`, which hard-codes
  `"finalize.rollback-conflict"` and finalize-specific prose (*"nothing was committed and …"*,
  *"this finalize's pre-image"*, `task.rs:4270-4310`). The generalization is spec→identity,
  home→path, **plus** a door-keyed code and door-keyed route prose. Read.
- **A2** — `settle-record.md:210-212`: the *"shipped `fs::rename` idiom"* of
  `relocate::displace_foreign_squatter` (`relocate.rs:300-307`, flattens to `file_name()`, no
  collision suffix, `bail!`s on a managed occupant) and `park_pre_image` (`task.rs:4253-4262`,
  flattens to the last `/` segment + a nanos suffix) preserves **no** relative path. *"Relative
  paths preserved"* is new code, not a shipped idiom — price it. Read.
- **A3** — `.jigc/displaced/` is already shared (`relocate.rs:266` `WORKBENCH_SUBDIR =
  "displaced"`; `park_pre_image` writes there). After D3.4 any file there **permanently gates
  `uninstall`** and the wave adds no verb that clears it (boundary: no new capability). Say what
  the refusal's route names (the exact path + `--force`), and decide whether jigc's own parked
  pre-image counts as a foreign byte at that door.
- **A4** — counts in a record that fences counts: D3.1's *"13 scattered consts"* cites **12**
  (verified above) and G-13's *"11 private"* matches neither; `task.rs:2687`'s own comment says
  *"the **ten** `DoctypeArg::Address` doors"* where D5.2 says nine `parse_verb_addr` call sites
  (also verified: nine). D11 puts the `design/` homes into `count_fences.rs` — these three belong
  inside a fence or should be dropped to a qualifier. Read.
- **A5** — D2.1's *"every acting door … under every member"* crosses the one row carrying a
  stated exemption: `jigc setup` (`cli.rs:1856-1862`, the unborn member; every other
  `BEHALF_DOORS` row has `exempt: &[]`). `setup` commits, so a `setup` under an un-concluded
  merge concludes it — say in the decision that the exemption is member-scoped and `setup`
  refuses under the widened `InProgress`. Read.
- **A6** — the git-2.54 bound (`settle-record.md:44-47`) is a genuine bound, but its failure mode
  is the wave's own tier-0 shape: a future git that writes a marker this enumeration does not
  know silently returns the exit-0 conclusion. Consider a written reopening trigger (a git
  version outside the tested matrix) rather than a pure declaration.
- **A7** — the departed-doctype residual (`settle-record.md:50-53`, D7.2) rides the trigger *the
  next `schema-version` bump of any doctype*, which **this wave cannot fire** (zero bumps,
  declared). So the un-namespaced `schema-version:` stamp — a **stored format** — crosses the
  1.0 pin. That is M51's decision and defensible; since M52 is the last pre-pin wave, the bound
  should say so in those words rather than leaving it to be re-derived.
- **A8** — D9's new structured `suppressed` key (`settle-record.md:373-381`): the mechanism
  transfers (placeholders survive both fences — `strip_unemitted`, `finding.rs:926-938`; the
  shipped `jigc migrate <path> --as <doctype>` route, `migrate.rs:195`), but the key's **value
  grammar** — a literal argv vs a template carrying `<…>` — is load-bearing for
  `Route::mechanical`'s argv parse and is owed with the name, not after it.
- **A9** — two new codes state severity but no **route**: `file-state.absorbed` (D10.1, advisory)
  and `migrate-corpus.missing-snapshot` (D4.2, blocking). The universal advisory-route floor
  (M41) requires every advisory to carry one; the shipped route-less form is the literal *"no
  action needed"*. One line each.
- **A10** — D2.6's exit flip is adopter-visible and, unlike D7.1's and D4.3's, is routed to no
  adopter-facing home. Say where an adopter reads it (MIGRATING.md's batch, with the others).

---

## Bound-vs-deferral, judged

| bound | verdict |
|---|---|
| git 2.54.0's on-disk contract | **bound** (untestable against an unshipped git) — A6 asks only for a trigger |
| `GIT_DIR` the one declared posture residual | **bound**, unchanged from M51 §3, with a reopening condition; D2.4 genuinely reduces the residual count to one |
| the departed-doctype orphan half | **deferral with a trigger** — honest, but the trigger cannot fire in M52 and the stamp key crosses the pin (A7) |
| D3's *behaviour reversible, key one-way* | **broadly honest**; the un-priced half is that `.jigc/displaced/<task-id>/` becomes a location adopters depend on and a permanent `uninstall` gate (A3) |
| zero schema-hash / schema-version / corpus | **verified** (`manifest.rs:76-89`; the shipped singleton census) |

## Prior-art reconciliation

No contradiction found between the settled design and a prior locked doc on direction, storage
side or shape. Checked: `command-output-contract.md` (pre-pin additive/removal/reshape rules —
D3.3 and D6.1 are inside them), `doc-read-surface.md:90,108` (contract-version only increases —
D5.4 legal, keys unnamed, S5), `design/finalize.md:174,193,195,200` (D1.8 re-scopes `:195` with
its falsifying datum and builds `:174` rather than striking it — the right direction),
`reconciliation.md:144,186` (D1.8/D10.1), `team-ready-state.md:94,96` (D3.1 writes the rule that
exists in no locked doc and says so — leg 0 argued, correctly), `validation.md:663,710,723`
(D2.1/D7.2/D10.4), `storage.md:216,300-310` (D5.1 revises with the driven converse; D1.8 states
what *Concurrent writers* forbids and does not). The one place a prior doc is **overridden** —
M47 D1's ground for excluding a preview member — is engaged on its rationale with a datum
(`task finalize --dry-run` already refuses under a merge) and recorded as a revise, which is the
right form; B2 is about the exit it leaves unnamed, not about the override.
