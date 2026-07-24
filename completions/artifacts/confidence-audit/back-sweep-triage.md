# M39–M44 back-sweep triage — hand-verified facts vs the pinning substrate (2026-07-24)

Scope: the pre-1.0 confidence-audit session's third leg (DECISIONS → 2026-07-24 "The road to 1.0").
Sources enumerated: `completions/artifacts/M39..M44/VERDICT.md`, the four trial findings-verification
files those waves consumed (`RC-adoption/rerun-rc4/`, `RC-adoption/impl-rc5/`, `RC-lacon/`,
`RC-adoption/rerun-rc7/`), spot-checks against the shipped test tree at HEAD `14bb1e1`.
Vocabulary: `implementation/pinning.md` (compose goldens · contract property suites · repro blocks
in `crates/cli/tests/pinned_facts/` · the trial-shaped fixture corpus).

**Definition applied:** a "hand-verified fact" = a claim established by a one-off manual/live
verification (audit repro, trial probe, adversarial refutation), not by a standing test at the time
it was established. Facts whose *fix* landed test-first are (a) by that fix's red test. Facts the
M45 golden sweep / property suites now enforce are (a) by those. The sibling-hunt's 8 findings +
minor residue (`completions/artifacts/M45/sibling-hunt.md`) are **deliberately excluded** — they are
the same session's other third and already ranked (note: item 1 is already fixed at `1b3f1b5`;
`--dry-run` landed at `d61f8fe`).

**Counts: (a) 58 · (b) 21 · (c) 2 · unclassified 1.**

---

## (a) Already pinned — a standing test/golden/property suite enforces it

| # | Fact (one line) | Pinned by |
|---|---|---|
| a1 | M39: slug 50-char backstop survives single-word id-sources | hardened proptest (fix `02213f3`), `slug.rs` proptests |
| a2 | M39: `doc show` field-leaf + id-from slices resolve; absent leaf blocks | `doc_read_surface.rs` address round-trip (M45) + fix `e2a4c28` tests |
| a3 | M39: non-canonical singleton slug on `doc show` routes not-found | fix `0d8fcb1` test |
| a4 | M39: pinned JSON `base` renders structured `{sha, short}` | fix `ac37fb6` test + `doc_show.rs` |
| a5 | M39: base-guard advances over a record-only range, foreign commit blocks | `milestone_finalize_base_guard.rs` (fix `0790bcb`) |
| a6 | M39: fresh-clone re-derive/resume of a milestone record | `milestone_record_fresh_clone.rs` |
| a7 | M39: invocation-log `output_bytes` = exact stdout+stderr total (fd-tee) | `invocation_log.rs::output_bytes_equals_emitted_stdout_plus_stderr` |
| a8 | M39: compose renders ALL grounded sources, order-invariant | flipped M37 pinning tests + `form_vision_grounding.rs` (pinned_facts) |
| a9 | M40: field-less item templates scan stray field groups at every depth | fix `f15eef6` tests |
| a10 | M40: milestone-record retitle refused (machine-maintained) | `retitle_item_milestone_record.rs` (fix `fb3fade`) |
| a11 | M40: control-char titles rejected at all mint/retitle doors | fix `3d8faad` shared-guard tests |
| a12 | M40: two nested repeatables per block refused at load | fix `7ae939d` loader test |
| a13 | M40: gitignored trees never enter the ingest funnel | ingest tests + flow41 |
| a14 | M40: methodology manifest freeze-asserts ten schemas at pack-load | pack-load fence tests + `corpus_migration.rs` v0-stamp arms |
| a15 | M40: `doc schema` byte-determinism | compose goldens (`doc schema` × 6 states × 2 forms) |
| a16 | rc4 V1: block-scalar templates are fold-safe end-to-end | `migration_slot_fidelity.rs` (parametrized, all slot-bearing templates) + M43 `{{schema:}}` seam fences |
| a17 | rc4 V2: `start --format json` carries `{task, text}` (id / null on router) | `flow42_acceptance.rs:438-468` |
| a18 | rc4 V3/V6: enum members + field→section in `doc schema` | `doc_schema.rs` contract tests + goldens; rc7 R2 re-verified live |
| a19 | rc4 V4: fabricated `.vue` script symbol blocks; real one resolves | M41 fork-3 tests + `resolve.rs` vue unit tests + flow42 |
| a20 | rc4 V5: `--unset` splice-removes; `[]` non-ref-scalar rejected; empty scalar rejected | M41 V5/N2 tests, engine `write.rs` tests |
| a21 | rc4 V7/W3: slug word-boundary retreat + edge-stopword drop, idempotency held | slug proptests incl. the restored `idempotent` invariant (`fe53318`) |
| a22 | rc4 V8/V13: advisory routes name the minting step / `jigc describe` (no dangling verbs) | M41 route repairs' tests + the M43 route fence (mechanical argv parses against the real CLI) |
| a23 | rc4 V9: ingest collapses `unmanaged` rows per-directory (agent arm), itemized rows kept | `ingest.rs:227-251` |
| a24 | rc4 V10: arch-doc `cites` committed-first warning present | M41 papercut test; preview golden-pinned |
| a25 | M41: stable `(code, target)` finding key, full `<type>:<slug>` identity, no collisions | `duplicate_field_finding_keys.rs`, `finalize_finding_keys.rs`, the `Findings` Serialize assert (M42) |
| a26 | M41: `ValueRemapped` D→Decision migration byte-faithful + idempotent | `migrate_corpus_value_remap.rs` |
| a27 | M41: hostile input never panics the code-anchor resolvers (all grammars + Vue) | `resolve.rs::*_hostile_input_does_not_panic` |
| a28 | rc5 U1: binary-mismatch route names `migrate-corpus` when the corpus is stale | `setup.rs` two-arm route + its tests |
| a29 | rc5 U2: validate exits non-zero over an unmigrated *placement* corpus; stock brownfield stays exit-0 | flow43 + `managed_vs_foreign.rs` + `schema-conformance.schema-version-current` tests |
| a30 | rc5 U4: `migrate-corpus --dry-run` exists (post-back-sweep: `d61f8fe`); validate is the real detector | flow43 + `corpus_migration.rs:1026` dry-run arms |
| a31 | rc5 U5: migrate-corpus lands through the tool (commit boundary), exits non-zero on blocked | fix `f3cd145` tests + flow43 |
| a32 | rc5 W2: `task bind`/`task discard` emit acks (the `op` enum, two task acks) | M42 finding-key/ack seam tests |
| a33 | rc5 W4/W5/M10: the two lying help sentences fixed; `task minted: <id>` printed | M42 papercut tests + compose goldens |
| a34 | rc5 W6: `locate-from-spec` demands the `maps-to-test` wiring per satisfied criterion | pack text golden-pinned (workflow previews × states) |
| a35 | rc5 R1: validate's field-leaf address round-trips on the read side; header `#section` renders fields | M42 read-contract tests + `doc_read_surface.rs` parity walk (15 doctypes) |
| a36 | rc5 R2: item `id` + `{#id}` anchor in the JSON read surface | M42 tests + `doc_show_item_leaf.rs` / `doc_show_nested.rs` |
| a37 | rc5 R4: `describe` paragraph breaks, still non-parseable | `describe.rs` format predicate + describe golden |
| a38 | rc5 R5/L2: bare singleton addresses resolve on the doc verbs | M42 read-contract tests |
| a39 | rc5 R6/L3: `jigc doc list` exists, contract-pinned index read | `doc_list.rs` |
| a40 | rc5 M1: methodology implement/finalize carry the staging contract; `left-out` printed pre-commit | `methodology_staging_contract.rs` + preview goldens |
| a41 | rc5 M2: hook-reject frame says the task survives + re-run route; no `--no-verify` invariant | `flow37_rename.rs:370` + M42 frame tests |
| a42 | rc5 M4/M5: `milestone discard` settles the record; terminal state refuses all workbench verbs (lifecycle-exercised) | `milestone_discard.rs` + flow43 discard arm (de-masked) |
| a43 | rc5 M6: agent-format `doc schema` prints the owning section | M42 papercut test + goldens |
| a44 | rc5 M9: minted id echoed (`task minted:`) in agent/human compose | M42 test + `compose_task_minted.rs` |
| a45 | rc5 L1: blocked validate/finalize log their finding codes; hook-rejected finalize identifiable (`finalize.commit-rejected`); exit-4 hold logs `migrate.review-pending` | `invocation_log.rs:189-490` |
| a46 | lacon A2: `--slug` on `jigc migrate` threads to the doc mint | M43 tests + flow44 |
| a47 | lacon A3: migrate templates render the full resolved schema (no understating) | the `{{schema:}}` seam + `workflow-refs.schema-ref-resolves` fence tests |
| a48 | lacon A4: same-path migration reviews at exit 4, `--approve` lands `M`; clobber route names `--approve` | flow44 same-path arm + migrate suites |
| a49 | lacon A14: findings print repo-real paths; file-state skips transient staging docs | M43 A14 fix tests |
| a50 | lacon B7/rc5 R7: `doc show --task` serves the staged copy; transient `commit:<task>` staged-readable; stale read routed | `doc_show_staged.rs` + flow44 |
| a51 | lacon B12: `decided-task` selectable; every `selectable: false` needs `suppressed: {reason, expires}` | pack-load fence tests + describe golden + `milestone-execution` suppressed test (M45) |
| a52 | lacon B3 (refutation): `--task` optional under single-active-task; two-task reject names both ids | write-path resolution tests (since `70611e4`) + M43 composed statement (golden) |
| a53 | lacon B11 (refutation): `#type` enum vocabulary on write-reject + `doc schema` + composed render (generated from `Field.of`) | M43 generation-seam tests + goldens |
| a54 | lacon B4 (refutation): `start --task <id>` resumes; `task validate <id>` = what's-left; both named in composed output | `compose_statefulness.rs` + M43 footer (golden) |
| a55 | rc7 R1 (refutation): `jigc migrate --format json` carries the minted id at `.task` | `migrate_adr.rs::migrate_format_json_carries_the_minted_task_id` + contract doc §1 names migrate |
| a56 | rc7 C1/L1: path-hash task ids — distinct, deterministic, resumable per source path | flow45 arm 1 (`blake3` disambiguator) |
| a57 | rc7 C2: `write.not-present` routes to the followable staged read | M44 fix test (route re-run verbatim in e2e, then pinned) |
| a58 | rc7 C3/C4, D1–D5: fidelity boundary-guard + report split (`v1.0.0` kept, `project-alpha-2.0` not flagged) · `record-decision` from-knowledge adr · AGENT.md machine-output + read-rule paragraphs · `workflow --preview` mutation-free · stated-at fence | flow45 arms 2–7 + `stated_at_fence.rs` + preview goldens; M44's two post-audit fixes (`7ee77d1`, `9f6a67b`) test-first |

Also (a), M43-audit cluster: the `unknown-type` route (`d47991b`), the three Display-rendered
route-less siblings (`a1696d4`), the ambush-const honesty split (`56cc853`) — each landed red-first;
the carryover gate incl. deletions + `--carry-staged` labeling + fail-open-on-missing-snapshot
(`carryover_gate.rs`, incl. `:993 milestone_missing_snapshot_fails_open`).

## (b) Superseded, declared-bound, or deferred-with-record — pinning would be churn

| # | Fact | Why not pinned |
|---|---|---|
| b1 | rc4 V4's "a doc CAN commit `AppLayout.vue#DoesNotExist` clean" | superseded — M41 fork 3 makes it block (now (a19)) |
| b2 | rc4 V14/rc5 R7: `doc show` reads committed-only, `task diff` is the staged read | superseded — M43 staged read revised the R7 record on its rationale (now (a50)) |
| b3 | rc4 V15's measured steady-state advisory floor (9 × `unsupported-language`) | superseded — the Vue arm drained it; per-instance acknowledge-ledger deferred to 1.1, trigger-keyed (decisions-pending) |
| b4 | rc5 U3: setup re-stamps `.jigc/version` unconditionally | declared — the stamp is binary provenance; the harm ran through U1/U2, both fixed |
| b5 | rc5 W1: no conditional compose (empty resolution ⇒ empty text) | declared dialect invariant (`workflow-dialect.md`) — a design law, not a driftable fact |
| b6 | rc5 W7/W8: spec has no lifecycle/closure, no open-decisions structure | deferred with record — post-1.0 doctype-completeness (`ideas/spec-criterion-status.md`, `ideas/spec-open-decisions.md`) |
| b7 | rc5 M3: sub-tasks pinned to a shared base (sequential-with-deps outside the primitive) | declared design (one-bounded-primitive invariant); discoverability wording shipped + golden-pinned |
| b8 | rc5 M8 (refutation): the `reset: moving to HEAD` reflog signature was git's own `stash`, not jigc | the refuted claim has no shipped surface; the underlying invariant is the unclassified item below |
| b9 | rc5 R8: store scope resolves code against the live working tree while reading committed docs | declared (`validation.md`); the label fix ("gates at finalize") shipped M42; implicit coverage via store-scope probe tests |
| b10 | lacon A1: the slug rule appears on no user-facing surface | superseded — M43/M45 state the cap at `add-item` + composed surfaces (golden-pinned); the rule itself proptest-pinned |
| b11 | lacon A6: no "task never touched this" discriminator at finalize | deferred — recorded re-open data against M42 fork 2 (print-over-refuse); carryover gate covers the pre-mint half |
| b12 | lacon A7/A10: back-out/reconcile boundary undocumented user-facing | superseded — M43 MIGRATING/QUICKSTART reconcile + back-out chapter (docs; reconcile machinery engine-tested) |
| b13 | lacon A9: fidelity scan checks only dotted version tokens (nothing on ADRs) | declared bound (Framing A, display-only); parked `ideas/migration-content-coverage.md` |
| b14 | lacon A13: no freeform reference doctype | deferred with record — anchor of the post-1.0 doctype-completeness milestone |
| b15 | lacon B5: full runbook at t=0, no staged reveal | declared (no-runtime invariant); parked `ideas/state-aware-compose.md` |
| b16 | lacon B9: five informational "no action needed" routes | recorded design (M41 route-floor reification); noise question rides the 1.1 ledger deferral |
| b17 | lacon B10: changelog gate fires on docs-only commits | works-as-decided (M42 fork 6); any change engages that record — a Settle fork, not a pin |
| b18 | lacon B13/B14: test-first "yours to police"; Settle gate convention-only, clearance unrecorded | declared (ergonomics-not-prevention bet); planning gate-record stays condition-keyed in decisions-pending |
| b19 | rc7 R4: ingest JSON is full-rows by contract; the collapse is the agent arm | working-as-contracted; the additive JSON `summary` block papercut was chartered, small, unshipped — a gap, not a fact |
| b20 | M43: seam fences are `debug_assert`-posture; debug==release byte-identity | recorded per-class posture; debug==release is the one UNPINNED-with-reason fact (pinning.md §3 permits it) |
| b21 | M39→project-alpha-3.0: the *live agent spawn* fan-out confirmation | inherently unpinnable in CI (requires a live Task-tool spawn); the honest bound is the record, re-confirmed per trial — but see (c1) for the sim half |

## (c) Still load-bearing and unpinned — the deliverable, ranked by consequence-if-silently-broken

### c1 — Concurrent N-process fan-out → join determinism (HIGH)

**The fact:** N *concurrent* OS processes driving the real binary through provisioned worktree
sub-tasks (`provision → execute → finalize` × N, divergent completion orders) join into one
deterministic, by-task-id, byte-stable commit with no cross-worktree contamination.
**How it was established:** only by hand, three times — the M39 e2e auditor's N-process binary sim +
verbatim spawn-template (M39 VERDICT → Honest bounds), re-proven one-off by the M43 e2e audit
(flow-44 arm list), live-confirmed once at the project-alpha-3.0 trial (5 parallel worktrees,
2026-07-22).
**Standing coverage today:** sequential-only. `milestone.rs`/`flow9_milestone_join.rs`/flow31-32
drive provision/join serially; `file_state_concurrency.rs` is thread-level and file-state-scoped;
no test spawns concurrent jigc processes against one milestone. The M45 boundary gate and join
collision detector are tested — serially.
**Consequence if silently broken:** the *one* concurrency primitive (an architectural invariant)
regresses undetected until the next live trial; a nondeterministic or contaminated join corrupts
committed milestone records — the highest-blast-radius silent failure left in the M39–M44 span.
**Proposed pin (cheapest honest form):** one integration arm, property-suite flavor — a `fan-out`
fixture procedure (built fresh per arm, never copied: pinning.md §4's worktree caveat) provisioning
a 3-sub-task milestone; the three `task finalize` runs spawned as concurrent child processes in two
divergent completion orders; assert (i) the join commit's tree hash identical across orders and to a
serial baseline, (ii) sub-task ordering by task id, (iii) no worktree writes escape their keyed
areas. This is the standing form of the sim the e2e auditors have been re-writing per wave — the
spawn-template already exists in the audit method.

### c2 — Cross-order byte-determinism of the multi-source sweeps (MEDIUM)

**The fact:** the audited "byte-identical across divergent orders" witnesses — the v0 stamp
migration across seed orders (M40 e2e), finding emission across divergent source orders (M41 e2e),
the carryover manifest across staging orders (M43 e2e) — hold only by construction (sorted walks)
plus one-off audit runs; no standing test drives any of them in two orders.
**Standing coverage today:** the suites assert sets/keys/symmetry in a *single* order
(`duplicate_field_finding_keys.rs`, `corpus_migration.rs`, `carryover_gate.rs:577` forecast/landed
symmetry); flow45's path-hash arm pins per-path determinism but ids are pure functions of the path,
not an ordering witness. Goldens cannot catch order variance (one order per state).
**Consequence if silently broken:** a refactor that swaps a sorted walk for hash-map iteration
ships driver-visible nondeterminism — a direct violation of the reproducibility claim and the
finding-key contract's declared order stability — with every existing test green.
**Proposed pin (cheapest honest form):** three repro-block conversions (pinning.md §3 — each block
cites its audit run), one two-order arm per suite: (i) `corpus_migration.rs` — commit the same v0
corpus in two seed orders, assert byte-identical migrated trees; (ii) the finding-emission sweep —
two divergent source orders, assert byte-identical `--format json` findings arrays; (iii)
`carryover_gate.rs` — two staging orders, assert byte-identical manifests. Small, mechanical,
convertible from the audit transcripts' shapes.

## Unclassified (flagged as such)

- **"jigc never destructively rewrites HEAD"** (rc5 M8's refutation substrate; the M30
  non-destructive-landing invariant: rollback via `restore`/captured pre-images, landing via
  `merge --ff-only`, never `git reset --hard`). *Why unclassified:* it is held today by
  **composition** — WIP-survival arms (`finalize_index_scoping.rs`, milestone-record suites) + the
  M45 rollback-axis tests — not by any single fence, and the global negative has no honest pin (a
  source grep is the barred pattern; per-op reflog assertions only sample). The one known live
  inconsistency (promotions rollback on the pre-correction primitive) is already sibling-hunt
  item 2. Disposition suggestion: treat as covered-by-composition once sibling-hunt item 2 lands,
  optionally adding one repro block asserting the trial's exact observable (the reflog after a
  hook-rejected finalize + aborted milestone combine contains only `commit:` entries).

## Coverage note

DECISIONS.md per-wave entries were consumed *through* the VERDICTs and findings-verification files
(which cite them); they were not independently re-swept line-by-line. The four trial-record files'
raw claims were triaged via their findings-verification adjudications, which is those files'
declared purpose. Spot-verification against the test tree was performed for every fact whose
pinning was non-obvious (migrate JSON `.task`, carryover fail-open, invocation-log finding codes,
fold-safety, panic-safety, ingest collapse, fresh-clone re-derive, concurrent fan-out — the last
being the one genuine hole found).
