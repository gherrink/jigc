# Mutation testing — the fixed seams (confidence-audit wave, 2026-07-24)

Tool: cargo-mutants 27.1.0 (installed via `cargo install --locked cargo-mutants`).
Working tree: never modified by the runs (cargo-mutants copies the tree to /var/tmp); `git status` at finish shows only the pre-existing untracked `completions/artifacts/confidence-audit/` dir.

**Substrate note (important):** the audit spans TWO revisions, because the back-sweep fix
wave landed *while the run was in flight*:

- **Run 1** copied the tree at **271454b** (the assigned HEAD) — 123/174 mutants adjudicated
  before the run was killed (collateral of stopping a progress-monitor task).
- **Run 2** (`--iterate`, same output dir) copied the tree at **e393dfb**, which includes
  **c71f181** `fix(cli): rollback captures survive a mid-promote/mid-retire finalize failure`
  (+267 lines in `crates/cli/src/task.rs` — one of the mutated seam files). Its discovery
  grew to **192** mutants (the fix added code inside mutated functions: 174 + 18 new).
  Run 2 tested **76** = 51 not-yet-tested + 7 run-1 MISSED retests + 18 new-from-c71f181.
  The 116 run-1 caught/unviable were excluded by `--iterate` and NOT retested against the
  new tree — their verdicts are 271454b verdicts (caveat below).

## Command lines

```
# discovery/count (271454b):
cargo mutants --list -f <9 seam files> -F '<seam-fn regex>'        # → 174

# run 1 (killed at 123 outcomes) and run 2 (--iterate) — identical except --iterate:
TMPDIR=/var/tmp cargo mutants -j 4 --timeout-multiplier 2 --test-workspace true [--iterate] \
  --output <scratchpad>/mutants-out \
  -f crates/engine/src/validate.rs -f crates/engine/src/file_state.rs \
  -f crates/engine/src/compose.rs -f crates/engine/src/schema.rs \
  -f crates/engine/src/parse.rs -f crates/engine/src/write.rs \
  -f crates/cli/src/task.rs -f crates/cli/src/doc.rs -f crates/cli/src/migrate_corpus.rs \
  -F 'version_currency_break|version_ahead_break|ahead_route|route_schema_conformance|schema_conformance_store|id_from_enum_violation|read_schema_version_stamp|schema_version_from_front_matter|detect_rename|rename_strong_finding|rename_weak_finding|rename_dangling_baseline_finding|detect_committed_store_renames|load_workflow_def|is_machine_maintained_absolute|is_reserved_depth|ceiling_violations|validate_after|set_gated_item_slot|reparse_or_reject|locate_item_region|set_slot_validated|set_field_validated|fold_hook_streams|capture_owner_artifact_index|rollback_owner_artifact_index|rollback_promotions|git_commit_capture|trailer_key_refusal|retitle_id_from_refusal|bind_role_on_copy_in|id_from_enum_block|future_stamp_finding|migrate_committed_corpus'
```

`--test-workspace true` is load-bearing: many engine-seam pinning tests live in
`crates/cli/tests/` (file_state_history_gate.rs, flow46_acceptance.rs, migrate_rollback.rs…);
cargo-mutants' default per-package test scope would have produced false MISSED for engine
mutants. Baseline: 28s build + 200s test (all workspace tests, green); auto timeout 402s.

## Seam → function map (located by grep before the run)

| Seam (charter item) | Functions mutated | File |
|---|---|---|
| version-currency / ahead arms | `version_currency_break`, `version_ahead_break`, `ahead_route`, `route_schema_conformance`, `schema_conformance_store` (store-sweep stamp arms), `read_schema_version_stamp`, `schema_version_from_front_matter` | engine/src/validate.rs |
| history gate + rename oracle | `detect_rename`, `rename_strong_finding`, `rename_weak_finding`, `rename_dangling_baseline_finding`, `detect_committed_store_renames` | engine/src/file_state.rs |
| rollback seams | `rollback_promotions`, `capture_owner_artifact_index`, `rollback_owner_artifact_index`, `git_commit_capture`, `fold_hook_streams` | cli/src/task.rs |
| trailer-door adjudicators + copy-in binding | `trailer_key_refusal`, `retitle_id_from_refusal`, `id_from_enum_block`, `bind_role_on_copy_in` | cli/src/doc.rs |
| allows-create uniqueness fence | `load_workflow_def` (the fence lives at parse) | engine/src/compose.rs |
| `s > current` blocked arm | `migrate_committed_corpus`, `future_stamp_finding` | cli/src/migrate_corpus.rs |
| settability predicate (M45 Inc 3) | `is_machine_maintained_absolute` | engine/src/schema.rs |
| item-slot ceiling seam (M45 Inc 2) | `is_reserved_depth`, `ceiling_violations` (read side); `validate_after`, `set_gated_item_slot`, `locate_item_region`, `set_slot_validated`, `set_field_validated`, `reparse_or_reject` (write side) | engine/src/parse.rs, engine/src/write.rs |
| shared id-from adjudicator | `id_from_enum_violation` | engine/src/validate.rs |

## Results — run 1 (271454b substrate, 123/174 tested)

**caught 87 · missed 7 · unviable 29 · timeout 0**

### The 7 MISSED, adjudicated

#### 1. `cli/src/task.rs:2763` — `rollback_promotions`, retirement un-stage: `*spec == path` → `!=` — **(a) GENUINE GAP, rank #1**

The retirement index-axis guard: un-stage a planned retirement's deletion only when jigc's
own stage staged it (`staged.contains(path)`). Mutated, the condition is true whenever
`staged` contains *any other* entry — and it always does (promotion destinations +
`.jigc/config` + `.jigc/version` are always in `staged`, see `stage_migration`). So under
the mutant, a commit-failure rollback runs `git restore --staged <retirement>` on a
**user's pre-staged `git rm`** — resurrecting the deletion on the index axis, the exact
regression M40 F7 fixed ("no failure path resurrects a pre-staged deletion").

Why the pinning test passes over it:
`migrate_rollback::pre_staged_git_rm_survives_a_hook_rejection_rollback` asserts
`porcelain.starts_with("D ")` — but the test's `git()` helper **`.trim()`s stdout**
(crates/cli/tests/migrate_rollback.rs:59-74). `git status --porcelain <path>` yields
`"D  <path>"` for a staged deletion (X=D) and `" D <path>"` for an *un-staged* deletion
(Y=D); after trim **both** start with `"D "`. The oracle is a tautology across exactly the
axis the mutant flips — the fix's own test passes over the broken implementation, the
charter's named failure mode, verified mechanically (the per-mutant log shows the test ran
and passed: `test pre_staged_git_rm_survives_a_hook_rejection_rollback ... ok`).

Fix direction: assert the index state through a trim-immune oracle — untrimmed porcelain,
or `git diff --cached --name-status` (must list `D <path>`), or `git ls-files -- <path>`
non-empty-at-stage-0. One-line test change. (Class note: this is the only
`starts_with("D ")` porcelain assertion in the tree, but any porcelain X-column assertion
routed through a trimming helper has the same blindness.)

Substrate caveat: c71f181 (landed mid-run) reworked this function's captures; run 2 retests
the sibling against the new code — see the run-2 section for whether it now dies.

#### 2. `cli/src/doc.rs:3594` — `bind_role_on_copy_in`: `e.doc_type == addr.type && !e.as_role.is_empty()` → `||` — **(a) GENUINE GAP, rank #2**

The M45 Inc 5 T2 copy-on-write role binding. Mutated, the gate-entry lookup matches the
first `allows-create` entry with a **non-empty role for any doctype** — so an edit verb
that copy-on-writes a committed doc of a doctype *outside* the gate (e.g. `set-field` on a
committed `spec` inside a task whose workflow declares `{type: adr, as: decision}`) binds
`decision → spec:<slug>`. Original behavior: binds nothing (the doctype has no entry). The
wrong binding then resolves `task.decision` to the spec on every later re-compose —
`@task.decision`-slices render the wrong doc. Reachable through any ordinary edit verb; no
test drives a copy-in of an out-of-gate doctype and asserts the roles record stayed empty.

Fix direction: one test — task under `single-task` (gate: adr-as-decision), `doc set-field`
a committed non-adr doc, assert the roles record has no `decision` binding (and the
compose still renders the pending form).

#### 3. `engine/src/validate.rs:646` — `schema_conformance_store`: `w.id == format!("migrate-{ty}")` → `!=` — **(a) GENUINE GAP, rank #3**

The `migratable` flag feeds `adoption_route`, which obeys the M40 two-tier rule: name
`jigc migrate <path> --as <ty>` **only when** the composed pack ships `migrate-<ty>`.
Mutated, `migratable` is true whenever *any* workflow id differs from `migrate-<ty>` —
i.e. always. The shipped non-migratable persisted doctypes are `dogfood-record` and
`milestone-record` (no `migrate-*` workflow in either pack): a foreign file squatting at
their home would get an advisory commanding a verb that **hard-errors** ("not
migratable") — the exact route-lie the two-tier rule exists to prevent (law 1). No test
pins the `migratable == false` wording of the store-sweep advisory (the migratable=true
wording is pinned, but the mutant keeps that arm true, so those tests pass).

Fix direction: one store-sweep test — plant a foreign `.md` at the `milestone-record` (or
`dogfood-record`) home, assert the `schema-conformance.unadopted-instance` advisory's route
names `jigc ingest` and does NOT name `--as milestone-record`.

#### 4. `cli/src/migrate_corpus.rs:775` — `migrate_committed_corpus`: `already_current.retain(|k| k != target)` → `==` — **(a) GENUINE GAP, rank #4**

The completed-interrupted-move dedup: the destination was enumerated `already-current` by
the union walk, and its completion supersedes that line. Mutated, `retain` keeps ONLY the
superseded line and drops every *other* already-current entry — the report then
double-reports the moved doc (both `already-current` and `migrated`) and silently omits
every genuinely-current doc from the summary, in any run that completes an interrupted
move. The pinning test (`corpus_migration::migrate_corpus_completes_an_interrupted_relocation`)
asserts only `out.contains("1 migrated") && out.contains("CHANGELOG.md")` — contains-only,
so both the double-report and the dropped entries pass unseen.

Fix direction: extend that test — assert the already-current count is 0 (or the summary
line exactly), and/or add a second already-current doc to the fixture and assert it stays
reported.

#### 5. `engine/src/parse.rs:1383` — `ceiling_violations`: `range.start < end` → `<=` — **(a) genuine gap, rank #5 (low severity)**

The slot-span boundary of the heading-depth-ceiling sweep (the read side of the M45
item-slot ceiling class). A heading can start exactly at `span.end` only in the
empty-trimmed-span shape: zero bytes between a section/item heading line and the next
structural heading (`## A\n## B` — no blank line, no content; `trim_span` collapses the
empty region to `(content_start, content_start)` and the next heading's `range.start`
equals it). The parser accepts that non-canonical (OOB-edited) shape; under the mutant the
next section's own `##` heading is falsely reported as a blocking
`conformance.slot-heading-depth` violation *inside* the empty slot. Over-blocks, never
corrupts, and the canonical writer always emits separating blank lines — hence low — but
the boundary is untested and the divergence is reachable on committed OOB docs.

Fix direction: one parse test — a two-section doc with no blank line between and an empty
first slot must produce zero ceiling findings.

#### 6. `cli/src/doc.rs:1621` — `retitle_id_from_refusal`, id-from field lookup guard `f.id == repeatable.id_from` → `true` — **(b) equivalent under every shipped schema**

Mutated, the lookup returns the block's **first** `Leaf::Field` instead of the id-from
field. Checked all 15 shipped schemas (both packs): in **every** repeatable, the id-from
field is the first Field leaf of its block — `commit` trailers (`key` first), `changelog`
change-group (`category` first), `arch-doc`/`spec`/`prd`/`completion-record`/
`decisions-log`/`deferral-ledger`/`roadmap` (`title` first), `milestone-record`
(`task-id` first). `find_map` skips non-Field leaves (slots) either way, so the mutant is
behaviorally identical on every schema the cli can load (tests drive the embedded packs;
no mutated-filesystem-pack arm ships a non-first id-from). **Latent hazard, not a current
bug:** a third-party pack with a non-first id-from would mis-adjudicate the enum-retitle
refusal — but a public pack platform is a declared non-goal pre-1.0. If cheap insurance is
wanted, an engine-level unit test with a synthetic non-first-id-from schema would kill it.

#### 7. `cli/src/migrate_corpus.rs:535` — `migrate_committed_corpus`: `Some(s) if s > dt.version` → `>=` — **(b) equivalent, provable by arm order**

The match at migrate_corpus.rs:527 tests `Some(s) if s == dt.version => already_current`
**before** the mutated `Some(s) if s > dt.version => blocked` arm. Rust match arms are
tried in order, so `s == dt.version` never reaches the mutated guard; `>=` and `>` are
extensionally identical at that point. No input can distinguish them. (The `s > current`
*behavior* itself — the future-stamp block — is pinned caught: the sibling mutants on the
arm were killed, see caught.txt.)

## Results — run 2 (e393dfb substrate, `--iterate`, 76 mutants, 62 min)

**caught 42 · missed 8 · unviable 13 · timeout 13**

Of the 7 run-1 MISSED retested: 5 re-missed identically (the adjudications above
replicate on the new tree); `task.rs:2763`'s shifted sibling (`2795`) TIMED OUT (see
pass 3); `doc.rs:1621` was reported CAUGHT — **a false catch**: the failing test was
`engine::write::promote_slot_to_repeatable_tests::promotion_is_byte_stable_and_lossless`,
an *engine proptest* that a cli mutant cannot influence. Its random `title()` generator
produced `"The"` — an edge-stopword title whose slug is empty — and
`promote_slot_to_repeatable` refused with `UnslugableTitle`, while the test `.expect`s
success unconditionally. The run-1 MISSED verdict (and the (b) adjudication) stands.

**Bonus finding (standalone, not a mutant):** that proptest is nondeterministically red —
the `title()` generator emits titles the slug rule cannot slug (single edge-stopword,
e.g. `"The"`, M41's edge-stopword drop), and the property treats promotion success as
universal. Either the generator must exclude unslugable titles or the property must
accept the typed `UnslugableTitle` refusal for them. A flaky standing property suite also
poisons mutation/CI verdicts (observed here as a false catch).

### The 3 NEW MISSED (write-side item-slot ceiling seam), adjudicated

#### 8. `engine/src/write.rs:5575` — `validate_after` clause (b): second `&&` → `||` — **(a) GENUINE GAP, rank #3**

Mutated: `confined = (starts_with(prefix) && ends_with(suffix)) || edited.len() >= prefix.len()+suffix.len()`.
The length disjunct alone now satisfies confinement, so any **length-preserving or
growing** target-escape is accepted — clause (b), the re-verified surgical-on-edit
contract (M13 audit HIGH lineage; M45's item-region confinement rides the same gate), is
effectively dead for that shape. A direct unit test exists
(`write::validate_after::target_escape_is_blocking_finding`) but its tampered buffer
**shrinks** (replaces a 38-char sentence with 15 chars), so `len_ok` is false and the
mutant still rejects it — the test survives on an accident of its fixture's arithmetic.
No test covers the length-preserving escape shape.

Fix direction: extend the escape unit test (or add a sibling): tamper with a same-length
byte flip outside the target span (`len_ok` true) → must still return
`write.target-escape`.

#### 9. `engine/src/write.rs:5791` — `reparse_or_reject` → `Ok(())` — **(c) survivable-by-design (defense-in-depth tripwire), with a residual doubt**

The stubbed gate is the `write.non-reparseable` (a)-half shared by the two `--unset`
splice-remove paths (`unset_field_validated` :5712, `unset_item_field_validated` :5740).
What pins the adjacent behavior: the `unset_eligibility_finding` guard admits only fields
whose **absence is a conformant state** (not author-required, not defaulted, not
`set:`-derived), and the byte-stable `unset_field`/`unset_item_field` splices are pinned
by the unset suites (`crates/cli/tests/set_field_unset.rs`, engine `write::roundtrip`) —
so no reachable input through the gated callers should produce a non-reparseable buffer;
the gate is a tripwire for a splice-primitive bug that does not currently exist.
Residual doubt (why this is not a clean (b)): I could not prove that removing the *last*
optional field of a group can never leave a non-parsing empty-sentinel state; if it can,
this is a reachable (a). Cheap insurance either way: a 5-line direct unit test calling
`reparse_or_reject` with a hand-corrupted buffer pins the tripwire itself.

#### 10. `engine/src/write.rs:5910` — `set_gated_item_slot`: delete the `[item]` match arm — **(b) equivalent (redundant dispatch)**

With the single-hop arm deleted, all chains route through `set_nested_item_slot`.
Code-walk: `nested_parsed_item` resolves a 1-element chain to the top-level item
(`physical_item_chain` + the walk loop), and both arms converge on
`locate_item_path` → `render_item_at` → splice → `validate_after` — the same one
item-bytes path. Empirical: the byte-stability round-trip suites and every cli item-slot
write test passed under the mutant, so the two paths are byte-identical on the tested
surface. The `[item]` specialization is redundant dispatch, kept (presumably) for
clarity; not a test gap. (If exact error-message parity on the absent-item reject ever
matters, a direct parity assert would firm this to proven-equivalent.)

## Results — pass 3 (timeout re-adjudication, `-j 2 --timeout-multiplier 4`, 27 min)

All 13 run-2 timeouts re-tested at low parallelism with an 814s timeout:
**10 caught · 3 missed · 0 timeout** — every run-2 timeout was load-induced (4 workers
against a timeout calibrated on an idle baseline + the sibling session's gates), none was
a genuine hang.

The 3 pass-3 MISSED:

#### 11. `cli/src/task.rs:2795` — `rollback_promotions` un-stage guard `==` → `!=` (the e393dfb sibling of finding #1) — **(a), merges into finding #1**

Confirms finding #1 against the POST-c71f181 code: the guard is byte-identical at HEAD
(only shifted +32 lines) and the pinning test's trimmed-porcelain oracle still cannot see
the index axis. One finding, verified on both substrates.

#### 12+13. `engine/src/validate.rs:1345` — `route_schema_conformance` ahead arm: guard `s > current` → `false`, and `>` → `<` — **(a) GENUINE GAP (one gap, two sibling mutants), rank #6**

Both mutants make the labeler's **ahead arm** unreachable (guard→false directly; `>`→`<`
is shadowed by the `s < current` arm at :1337), so a future-stamped doc's *accompanying*
`schema-conformance.*` findings fall through to `Some(_) =>` and get labeled
`corrupt — at the current schema-version {current}` — the exact "at-current corrupt lie"
the 2026-07-24 sibling-hunt item-1 arm exists to prevent. The arm is observable only for
a future-stamped doc that is ALSO non-conformant (a clean future-stamped doc has no other
findings to label; the break itself carries its own route via `ahead_route`, which IS
pinned — `ahead_route → "xyzzy"` and the whole-fn stub and the at-current-boundary
mutants `>`→`==`/`>=` were all caught, showing the migrate and corrupt labelings are
pinned). The uncovered cell is precisely (stamp > current) × (has structural findings).

Fix direction: one store-sweep test — a future-stamped doc with a structural break must
carry the `ahead — …` label on its conformance findings, not `corrupt — …`.

## Per-seam FINAL counts (all three passes reconciled; 192 mutant identities)

| File (seam) | total | killed/unviable | missed | missed adjudication |
|---|---|---|---|---|
| engine/validate.rs (version-currency/ahead + store sweep + id-from adjudicator) | 54 | 51 | 3 | #3 migratable (a) · #12+13 ahead-label pair (a, one gap) |
| engine/file_state.rs (history gate + rename oracle) | 19 | 19 | 0 | clean sweep |
| engine/compose.rs (allows-create uniqueness fence) | 7 | 7 | 0 | clean sweep |
| engine/schema.rs (settability predicate) | 2 | 2 | 0 | clean sweep |
| engine/parse.rs (read-side ceiling) | 15 | 14 | 1 | #5 boundary (a, low) |
| engine/write.rs (write-side ceiling + validate-after) | 27 | 24 | 3 | #8 escape-conjunct (a) · #10 [item] arm (b) · #9 reparse tripwire (c) |
| cli/task.rs (rollback seams) | 37 | 36 | 1 | #1/#11 un-stage oracle (a) — confirmed on both substrates |
| cli/doc.rs (trailer doors + copy-in binding) | 17 | 15 | 2 | #2 bind-role (a) · #6 retitle guard (b); run-2's "caught" for #6 was a proptest false catch |
| cli/migrate_corpus.rs (`s > current` arm + report) | 14 | 12 | 2 | #4 retain (a) · #7 shadowed `>=` (b) |
| **Total** | **192** | **180** | **12** | **7 distinct (a) gaps · 3 (b) · 1 (c)** (the 1345 pair is one gap; 2763/2795 is one finding across substrates) |

Killed/unviable includes run-1's merged 116 (87 caught + 29 unviable; the per-mutant
split of run 1 was lost to the `--iterate` dir rotation — aggregate from the live run-1
counts). Timing: run 1 ~43 min (killed at 123/174) · run 2 62 min · pass 3 27 min.

## Adjudication summary — the (a) findings, ranked (these become fix work)

1. **The pre-staged-`git rm` rollback test's oracle is blind** (task.rs un-stage guard;
   both substrates). `migrate_rollback::pre_staged_git_rm_survives_a_hook_rejection_rollback`
   asserts `porcelain.starts_with("D ")` through a `.trim()`ing helper — staged `"D  x"`
   and un-staged `" D x"` are indistinguishable after trim, so the index half of the M40
   F7 contract ("never resurrect a user's pre-staged deletion") is unpinned. Fix: a
   trim-immune index oracle (`git diff --cached --name-status` or untrimmed porcelain).
2. **`bind_role_on_copy_in` wrong-doctype binding unpinned** (doc.rs:3594). No test
   copy-on-writes a committed doc of an out-of-gate doctype and asserts no role binds;
   the mutated `||` silently binds e.g. `decision → spec:<slug>`.
3. **`validate_after` clause (b) survives a length-preserving escape** (write.rs:5575).
   The only escape unit test uses a shrinking tamper; a same-length byte flip outside the
   target passes the mutated check. This is the safety net for the item-slot corruption
   class — add the length-preserving escape arm.
4. **The non-migratable adoption route is unpinned** (validate.rs:646). A foreign file at
   the `milestone-record`/`dogfood-record` home must get an ingest-only route; the
   mutated flag commands `jigc migrate --as <ty>`, which hard-errors — the M40 two-tier
   route lie, untested.
5. **Interrupted-move completion report integrity** (migrate_corpus.rs:775). The pinning
   test is contains-only; the mutated `retain` double-reports the moved doc and drops
   every other already-current entry unseen.
6. **The ahead-label cell (stamp > current) × (non-conformant) is unpinned**
   (validate.rs:1345 pair). Accompanying findings of a future-stamped broken doc would be
   labeled `corrupt` — the documented lie — with no test reddening.
7. **Read-side ceiling boundary off-by-one** (parse.rs:1383, low). A zero-gap
   (`## A\n## B`) OOB shape would false-block under `<=`; the boundary is untested.

Plus the standalone **bonus finding**: `promote_slot_to_repeatable_tests::
promotion_is_byte_stable_and_lossless` is nondeterministically red — its `title()`
generator emits unslugable single-stopword titles (`"The"`) and the property `.expect`s
success; observed as a false mutant catch. Fix the generator or accept the typed
`UnslugableTitle` refusal in the property.

## (b)/(c) confidence

- **#7 (`>` → `>=` shadowed by the `==` arm): high** — provable from match-arm order;
  no input reaches the mutated guard with `s == version`.
- **#10 (delete `[item]` arm): high** — code-walk shows `nested_parsed_item` resolves
  1-element chains to the top-level item and both arms converge on the same
  locate/render/splice/validate path; the byte-stability suites passed under the mutant.
  Residual: error-message parity on the absent-item reject was not proven byte-identical.
- **#6 (retitle guard → first field): high for the shipped corpus** — verified in all 15
  schema files (both packs) that the id-from field is the first Field leaf of its block.
  Named latent hazard for third-party packs (a pre-1.0 non-goal); one synthetic-schema
  engine unit test would retire it entirely.
- **#9 (reparse tripwire stub, the one (c)): medium-high** — the unset eligibility guard
  + byte-stable unset suites pin the adjacent behavior, and no reachable failing input
  through the two gated callers is known; but I could not PROVE the empty-sentinel-
  after-last-optional-field state is unreachable, so a 5-line direct red-path unit test
  is recommended rather than trusting the (c).

## Caveats

- The 116 run-1 caught/unviable verdicts were not retested against e393dfb (c71f181
  changed cli/src/task.rs). For the non-task.rs seams the two trees are identical, so
  those verdicts transfer; the task.rs caught-set should be re-read as "caught at
  271454b" — run 2's 18 new mutants cover the code c71f181 added.
- Run 2's 13 timeouts were load-suspicious (4 parallel workers against a timeout
  calibrated on an idle baseline, plus the sibling fix-wave session's gates running
  concurrently): nine of them are pure route-string mutants in `route_schema_conformance`
  that cannot plausibly hang a test suite. Pass 3 re-tests all 13 at `-j 2` with a 814s
  timeout.
- One false catch was observed (see run 2) via a nondeterministically-red proptest; the
  caught-lists were not exhaustively re-audited for further flake-catches (probability
  low — a false catch requires an unrelated test failure — and every *missed* verdict was
  verified individually, which is the direction that matters for the audit).
