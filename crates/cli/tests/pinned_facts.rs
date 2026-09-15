//! M45 Increment 11 / T2 — the **pinned-facts module**: the fact-drift mechanic
//! ([pinning.md](../../../implementation/pinning.md) §3, §5).
//!
//! A fact established by hand — a trial triage, a wave re-verify — was observed once
//! and held by nothing, so a later wave could move it silently (the `#type` alias,
//! JSON purity, the form-vision "regression"). This module is the standing home for
//! those facts: **every CONFIRMED *and* REFUTED verdict arrives with a repro block
//! (the delivery-format rule), and a REFUTED block's obligation is to become a
//! standing test** — because the refuting fact is exactly what drifts when nothing
//! tests it (pinning.md §3; [milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md)
//! → Audit).
//!
//! **Fact drift is its own mechanic** (pinning.md §5 — contract drift → property
//! suites, surface drift → compose goldens, **fact drift → repro blocks**). So these
//! tests co-exist with — and cross-reference — the property/acceptance suites that
//! also touch each area: a suite there pins the *contract* (the address grammar as a
//! registry-derived property, JSON purity as a per-outcome-class property); a test
//! *here* pins the *specific refuted trial claim* with its findings-verification
//! citation, over the trial-shaped fixture corpus. That is not a one-file-one-purpose
//! violation — it is the two mechanics §5 deliberately separates.
//!
//! **These are regression guards, not fix drivers, so there is no red phase** — the
//! refuted facts are already *true* on the shipped binary; the pin keeps them true.
//! Each test is written to fail if the behaviour drifts (a negative control or a
//! before/after contrast, never a bare happy path), so a green pass is *the fact
//! exercised*, not a vacuous one.
//!
//! ───────────────────────────────────────────────────────────────────────────────
//! ## Provenance ledger — the disposition of every §1/§2/baseline fact
//!
//! `pinned-by:` is **declared list-enforced** — audit-checked prose, not a symbol
//! parser (a grep is not a fence; the checkable half — a test *name* exists — is not
//! the load-bearing half — the test *content* pins the claim; pinning.md §3). This
//! ledger is that list. Every finding in
//! [RC-alpha3/findings-verification.md](../../../completions/artifacts/RC-alpha3/findings-verification.md)
//! §1–§2, plus the M45 planning baseline's decisive probe facts (pinning.md §1),
//! carries a disposition; nothing is UNPINNED without a stated reason.
//!
//! ### §1 CONFIRMED — pinned by the wave's fix increments' red tests (NOT re-converted here)
//!
//! The confirmed blocks already live as their fix's red test (the dev workflow demands
//! it; pinning.md §3), so re-converting them here would be duplication:
//!
//! | §1 row | fact | pinned-by (fix red test) |
//! |---|---|---|
//! | 1.1 | item-slot writes bypass the heading ceiling | `item_slot_corruption_acceptance.rs` · `item_slot_ceiling_axis.rs` |
//! | 1.2 | commit trailers dropped on the real write path | `commit_trailer_roundtrip.rs` |
//! | 1.3 | dangling file-state cache blocks every task | `file_state_history_gate.rs` · `file_state_concurrency.rs` |
//! | 1.4 | owner-artifact prose contradicts the gate | `owner_artifact_gate.rs` · `owner_artifact_rollback.rs` |
//! | 1.5 | form-vision empty on the set-field-first path (role-binding) | `flow_role_binding.rs` |
//! | 1.6 | `join` is docs-only; a clean join is not safe-to-finalize | `milestone_join_collision.rs` · `milestone_boundary_gate.rs` |
//!
//! ### §2 REFUTED (3) + the corrected attribution (+1) — converted here, the new obligation
//!
//! | §2 row | refuted claim | pinned-by (here) | contract-drift sibling |
//! |---|---|---|---|
//! | 2 · row 1 | "workflow-printed `#type`/`#scope` commit addresses error" | [`address_grammar`] | `doc_read_surface::documented_alias_forms_round_trip` |
//! | 2 · row 2 | "finalize `--format json` emits non-JSON" | [`finalize_json`] | `machine_output::landed_finalize_under_chatty_hooks_carries_hook_output_on_stdout_and_relays_on_stderr` |
//! | 2 · row 3 | "form-vision grounding render regressed" | [`form_vision_grounding`] | `flow_form_vision` (M39 acceptance) · `flow_role_binding` (the real defect) |
//! | 2 · row 4 | "Bug C — dangling state proves the transactional invariant broke" | [`file_state_transactional`] | `file_state_history_gate.rs` (the cache-lifetime fix) |
//!
//! ### Baseline facts (pinning.md §1 decisive probe facts) — disposition
//!
//! Every M45 planning-baseline probe fact is *already carried*, so this task adds no
//! baseline test — the obligation is "not already carried," and the set is empty:
//!
//! | baseline fact | pinned-by | disposition |
//! |---|---|---|
//! | registry breadth (dev 17/6 · methodology 16/10 · composite 33/15) | `registry_seam.rs` + the compose goldens | carried |
//! | the two `commit` schemas differ in bytes; dev wins by precedence | the compose golden (doubles as the record of which won) | carried |
//! | the 4 `creates-task: false` workflows refuse `start` — exit 1, empty stdout, refusal on stderr | the compose goldens (stderr + exit code captured) | carried |
//! | compose determinism (two-repo byte-identical; TZ/locale/git-identity/branch-invariant) | `golden_harness.rs` | carried |
//! | ids never serial-suffix — a slug collision hard-rejects with a resume route | `start_compose::serial_re_run_of_the_same_intent_blocks` | carried |
//! | debug and release output are byte-identical | — | **UNPINNED: not test-pinnable in-suite** — the harness runs only the `CARGO_BIN_EXE_jigc` **debug** build; asserting release-parity would need a second build the test cannot produce. Recorded, not pinned. |
//!
//! ### §3 full classification + §5 candidate scope — disposition (by pointer)
//!
//! (Confidence-audit minor item 6: the ledger stopped at §2, leaving §3's dispositions
//! findable only in the trial artifact.) §3's rows are **scope routings, not
//! test-pinnable facts** — each names a class and where it goes, so its disposition
//! lives where it was routed rather than as a repro block here:
//!
//! - **§3-B (incomplete-fix siblings):** every B row chartered into M45 is pinned by
//!   its fix increment's axis-iterating red tests (the §1 table above carries the
//!   headline overlap); the rest rode the §5 tier-2 law batch (M45 Inc 10).
//! - **§3-C (deferrals with fired/trigger-adjacent triggers):** routed at the M45
//!   Settle — the fired capability tiers are owed at the **M46 capability wave**
//!   ([decisions-pending.md](../../../implementation/decisions-pending.md) → the
//!   capability wave); a deferral there carries its trigger, which is its pin.
//! - **§3-D (declared bounds re-reported):** each bound stays declared at its owning
//!   doc/oracle; the one testable nuance is pinned at
//!   `file_state_history_gate::sparse_checkout_absence_classifies_as_weak_deletion_block`.
//! - **§3-E (new findings) and the §5 not-chartered remainder:** dispositioned in the
//!   Settle record ([DECISIONS.md](../../../DECISIONS.md) → 2026-07-23 M45 planning:
//!   the Settle) against the trial artifact — charter decisions, not facts a repro
//!   block can hold.

//! ### Back-sweep additions (the confidence-audit wave, 2026-07-24)
//!
//! The M39–M44 back-sweep triage (its item c2) added [`finding_emission_order`] —
//! provenance the **M41** completion audit's cross-order determinism witness, not
//! this ledger's RC-alpha3 rows; its sibling cross-order repro blocks live
//! beside their suites (`corpus_migration.rs` — the M40 seed-order witness;
//! `carryover_gate.rs` — the M43 staging-order witness), each citing its audit run.

//! ───────────────────────────────────────────────────────────────────────────────
//! ## The RC-alpha4 ledger (M47 Inc 11 / T3) — the disposition of all 31 rows
//!
//! The final unseeded trial's verification
//! ([RC-alpha4/findings-verification.md](../../../completions/artifacts/RC-alpha4/findings-verification.md);
//! 20 CONFIRMED + 1 confirmed-as-decided · 6 PARTIAL · 4 REFUTED, every verdict with
//! a repro block) is the
//! evidence base the **rc.10 wave** was chartered on. Per pinning.md §3 the
//! **confirmed** blocks are already their fix increment's red test and are *not*
//! re-converted here; the standing obligation is the **refuted** set, plus a stated
//! disposition for every remaining row. **Nothing is UNPINNED without a reason.**
//!
//! Two boundaries govern the UNPINNED reasons, and neither is a shrug: the wave
//! *"changes what existing surfaces **say**, never what surfaces **exist**"* (the
//! 2026-07-25 charter razor — a capability gap routes to the **M46 capability wave**,
//! [decisions-pending.md](../../../implementation/decisions-pending.md)), and a
//! **capability gap has no false statement to fence** — pinning its absence would
//! pin the gap as expected output, the exact anti-pattern the wave's ordering rule
//! exists to prevent.
//!
//! ### Part A — the validate/finalize contract
//!
//! | row | verdict | disposition |
//! |---|---|---|
//! | A1 | CONFIRMED | Inc 4's red tests — `validate_previews_the_gate.rs` (the carryover probe at the `task validate` seam) · `owner_artifact_cause_axis.rs` (the six staging-independent causes) |
//! | A2 | REFUTED | [`finalize_left_out_stream`] **(new here)** — the agent-text `left-out` block P2 read as "three lines before the JSON" is stderr's; its stdout sibling stands at `finalize_manifest::json_manifest_on_dry_run_and_landed_run`, and the chatty-hook purity arm at [`finalize_json`] |
//! | A3 | CONFIRMED | Inc 10 T1 — the `BOOTSTRAP_OUTPUT_CONTRACT` body assertion in `crates/cli/src/adapter.rs` (which pinned the lie verbatim until the fix) + the six `agent-md` compose goldens |
//! | A4 | PARTIAL | **UNPINNED (residue only), reason stated:** the row finds no false statement — the gating semantics *are* stated (`task.rs` → `Validate`: *"exit non-zero iff any blocks"*, behaviourally pinned at `exit_codes::blocked_task_validate_exits_task_gate_blocked`), and the store/task label asymmetry is drained by Inc 8's severity-keyed trailer (`validate_envelope.rs`) + Inc 4 T4's scoped promise surfaces. The remaining wish — an *output* line restating the exit rule — was not chartered into tier 2's ~16 items |
//! | A5 | CONFIRMED | Inc 3's red tests — `commit_rejected_axis.rs` (one error code per committing door, the whole axis **[Corrected 2026-09-15 (M51 Increment 7, T3):** *the whole nine-door axis* — as M47 drove it; **ten** doors since M49 Increment 2 T3.**]**) + the `migrate-corpus` re-run recovery that replaced *"already current"* |
//!
//! ### Part B — the write surface
//!
//! | row | verdict | disposition |
//! |---|---|---|
//! | B1 | CONFIRMED | Inc 10 T7 — `author_write_contract.rs` (the append/update/**collision** three-way, aligned on all five surfaces) |
//! | B2 | REFUTED | **`batch_author_rerun.rs`** (Inc 11 T1) — `create` → `author` → `create` → `author` all exit 0 on one task, and no composed step text or help surface claims a rejection. *Named here on purpose:* this increment's own internal punt is tracked to a landing, not left to "a later task" |
//! | B3 | REFUTED | Already standing before the trial ran — `set_field_unset::unset_clears_an_optional_scalar_byte_stable_and_reconforms` (the capability M41 shipped) + `::empty_value_reject_routes_to_unset` (the route that names `--unset`). §3's obligation is that a refuted fact **has** a standing test, not that a duplicate is minted |
//! | B4 | CONFIRMED | **UNPINNED:** a capability gap (no `--before`/`--after`/`--position`, no reorder verb) — outside the wave's boundary, routed to M46 |
//! | B5 | CONFIRMED | **UNPINNED:** a capability gap (no slot `--append`; set-slot replaces) — same boundary, same routing |
//! | B6 | PARTIAL | Inc 7 — `task_diff_envelope.rs` (the `--format json` half P3 was right about) + `format_json_success_axis.rs`. The *author dry-run* half is a capability → M46 |
//! | B7 | CONFIRMED | Inc 10 T8 — `slug::mint_statement_states_the_whole_mint_rule` (the statement built from the enforcing constants) + `help_truth.rs` |
//!
//! ### Part C — the read surface
//!
//! | row | verdict | disposition |
//! |---|---|---|
//! | C1 | REFUTED (capability) / CONFIRMED (route) | [`doc_show_address_depth`] **(new here)** — every documented depth resolves, and the section-eliding address fails identically on the **write** side. The confirmed route half is Inc 10 T4's, pinned at `store::store_no_such_section_route_names_the_real_sections_and_teaches_the_item_form` |
//! | C2 | CONFIRMED | **UNPINNED:** no search surface exists — a capability gap ([ideas/doc-search.md](../../../ideas/doc-search.md)), and the standing evidence under M46's read-side entry |
//! | C3 | PARTIAL | Inc 10 T5 — `doc_show.rs` (the anchor-is-the-id sentence now on `doc show --help`, where the reader stands) |
//! | C4 | CONFIRMED | **UNPINNED:** reading a managed doc at a past revision is a capability the surface does not have — outside the boundary, M46 |
//! | C5 | CONFIRMED | **UNPINNED:** a new key on the pinned `doc list` projection is a *new surface* (and a contract-version bump), which the razor routes to M46 |
//! | C6 | CONFIRMED | Inc 10 T5 — `doc_show.rs` (the leaf-purity guarantee stated on `doc show --help` **and** driven, with the whole-doc newline wart named) |
//! | C7 | CONFIRMED | **UNPINNED (residue), reason stated:** the served-read half is closed by C6's guarantee (a served read carries the addressed node alone); reconsidering the footer's last-line position on *findings envelopes* changes a surface's shape and was not chartered |
//!
//! ### Part D — routing & misleading messages
//!
//! | row | verdict | disposition |
//! |---|---|---|
//! | D1 | CONFIRMED | Inc 10 T6 — `describe.rs` + `compose_create_gates.rs` + the orientation goldens (the affordance advertised over the members it serves) |
//! | D2 | CONFIRMED | The **ambush** half is Inc 4 T2's — `owner_artifact_cause_axis.rs`: the six staging-independent causes now surface at `task validate`, so the constraint is met before the finalize block P1 paid twice. The soliciting steps already state the owned home (`packs/methodology/steps/author-*-record.yaml`); that prose carries no fence of its own — **UNPINNED prose residue, stated** |
//! | D3 | CONFIRMED | Inc 10 T4 — `target_surface::title_names_symbol_states_the_comparison_it_made_not_a_rename` (the check narrates its comparison, never a rename it never looked for) |
//! | D4 | CONFIRMED | Inc 10 T2 — `setup.rs` (the hook path threaded from `resolve_hooks_dir` through `SetupSummary`, so `core.hooksPath` and worktree cases print repo-real) |
//! | D5 | PARTIAL | Inc 10 T5 — `doc_show_staged.rs` (the stale-copy note phrased for a reader who may *be* the staging task) |
//! | D6 | CONFIRMED | **UNPINNED:** marking store-vs-task scope inside a per-task finding list was not chartered; Inc 8's severity-keyed store trailer (`validate_envelope.rs`) is the adjacent fix, not this one |
//! | D7 | CONFIRMED-as-decided | **UNPINNED:** works-as-decided (M42 fork 6) — a contested *value* judgment is a charter question, not a fact a repro block can hold. Its young-corpus sibling (`repeatable-populated` false-alarming on a fresh changelog) did ship, at Inc 10 T9 (`validate_envelope.rs`) |
//! | D8 | CONFIRMED | The collision **root** closed at M44 (the `blake3(source-path)` task-id disambiguator, `flow45_acceptance.rs`); the live-collision arm's loud refusal is pinned at `start_compose::serial_re_run_of_the_same_intent_blocks` (a slug collision hard-rejects with a resume route); the mint rule now states itself at every mint site (Inc 10 T8). **UNPINNED residue:** naming the *disambiguators* (`--slug`, the path hash) at mint time was not chartered |
//! | D9 | CONFIRMED | Inc 5 — `commit_solicit_axis.rs` (the commit doc is solicited by its own step, so its first appearance is no longer the block message) |
//! | D10 | PARTIAL | The **refuted** half is [`code_anchor_bare_path`] **(new here)** — a bare path is accepted at write, `task validate`, `finalize` and the store sweep, with a dangling path still blocking. The **confirmed** projection half (both `code-anchor` fields rendering indistinguishably, the check-activation difference stated nowhere) is a separate finding, **UNPINNED**: it was not among tier 2's items, and the pin here must not be read as blessing it |
//! | D11 | CONFIRMED | **UNPINNED:** works-as-designed — the agent-is-the-router invariant (the CLI does no selection by construction; the residue is [ideas/spec-router-matching.md](../../../ideas/spec-router-matching.md)). The byte-shape of both surfaces is carried by the `start-orient*` / `start-intent*` compose goldens; substituting the real intent into the re-run line is a surface change that was not chartered |
//! | D12 | CONFIRMED | Inc 10 T6 (the severity label says whose grading it is, stated where the label prints) + Inc 8's `validate_envelope.rs` (the trailer counts by severity and names every gating door) |
//!
//! ### The M47 baseline's §2 premises — disposition
//!
//! The [planning baseline](../../../completions/artifacts/M47/baseline.md) corrected
//! eight charter premises against the real binary; each is carried by the increment
//! that acted on it, and **premise 6 is this task's own**:
//!
//! | premise | disposition |
//! |---|---|
//! | 1 · the `--dry-run` "third arm" is dead | Inc 4 T3 — `validate_previews_the_gate.rs` (the forecast stops greening a state finalize refuses) |
//! | 2 · the promise is on nine surfaces, not four | Inc 4 T4 — the five printed sites + their compose goldens |
//! | 3 · no construction makes the promise literally true | Inc 4 T4/T5 — the scoped prose + the eight reconciled doc sites |
//! | 4 · P5-4's "field-group-absent miss" premise is false | Inc 6 — `write_miss_shape_axis.rs` (the 7-cell write-verb × miss-shape matrix) |
//! | 5 · `task diff` falsifies a recorded clean confirmation | Inc 7 — `task_diff_envelope.rs` + `format_json_success_axis.rs` (all leaf verbs bijected against the clap tree) |
//! | 6 · tier 3's framing is half wrong — the non-empty-`left_out` **stdout** arm is pinned, the **stderr** arm is not | **This task.** Re-checked at HEAD as an explicit red-step obligation rather than assumed: no suite asserted the advisory's *stream*, so [`finalize_left_out_stream`] is a new pin, not a disposition |
//! | 7 · P3-2 narrows — the append prose is accurate; the defect is the collision case | Inc 10 T7 — `author_write_contract.rs` |
//! | 8 · the golden count is 612 on disk, and `pinning.md` contradicts itself | Inc 11 T2 — the whole-tree regeneration measured the fixed point and corrected the doc |

//! ───────────────────────────────────────────────────────────────────────────────
//! ## The RC-pre-1.0 ledger (the pre-1.0.0 trial, 2026-08-13) — disposition of the refuted set
//!
//! The acceptance trial for M47
//! ([RC-pre-1.0/findings-verification.md](../../../completions/artifacts/RC-pre-1.0/findings-verification.md);
//! 16 CONFIRMED · 4 REFUTED, every verdict with a repro block) refuted four claims.
//! **Not one of them mints a test here, and that is the conversion, not a shortcut** —
//! §3's obligation is that a refuted fact **has** a standing test, not that a duplicate
//! is minted (the B3 precedent above). Each row below was checked **by test content,
//! not by test name**, because the module doc's own honesty note says the checkable
//! half (a name exists) is not the load-bearing half (the content pins the claim).
//!
//! | row | refuted claim | pinned-by (already standing) | checked |
//! |---|---|---|---|
//! | R1 | "nothing lets you ask a doctype's schema; adr slots must be read off workflow prose" | `doc_schema::doc_schema_json_is_the_pinned_contract` · `::doc_schema_plain_listing_surfaces_write_addresses` | drives `doc schema adr` in **both** formats |
//! | R2 | "no verb inventories what jigc manages" (`find docs VISION.md` used instead) | `doc_list::doc_list_projects_the_store_surface_with_its_registration_state` | asserts the exact row set **and** the `managed`/`unregistered` split |
//! | R3 | "`doc show` is committed-only by design, so in-flight authoring cannot be reviewed" | `doc_show_staged::staged_read_serves_plain_json_and_slice` | drives `doc show <addr> --task <id>` over **uncommitted** bytes and asserts the staged bytes serve |
//! | R4 | "`doc show` never rendered my authored changelog item" | — | **No pin, and none owed:** the operator's own measurement error (`head -8` truncated the render), not a fact about the product. Recorded so the near-miss is auditable |
//!
//! ### Why the refuted set needed no new pin, and what that means
//!
//! All three product refutations are **shipped capabilities the sessions did not find**
//! — M40's `doc schema`, M42's `doc list`, M43's `doc show --task`. Each already
//! carries a dedicated contract suite, so the fact cannot drift silently. **What is
//! unpinned is not the capability but its reachability**: of 69 pack step files exactly
//! one names `doc show`, and it is not an authoring step (F1). That is a *fix*, and its
//! fence — a pack-load assert that an authoring step soliciting a write also states the
//! read-back — lands with the fix, not here. Pinning the gap now would pin the gap as
//! expected output, the anti-pattern the RC-alpha4 ledger's ordering rule names.
//!
//! ### The CONFIRMED set — UNPINNED, with the reason stated once
//!
//! All 16 confirmed rows (F1–F16) are **UNPINNED**, and uniformly so: the trial ran
//! under *no mid-trial fixes*, so no fix and therefore no red test exists to carry
//! them. Their conversion table lives with the verdicts
//! ([findings-verification.md](../../../completions/artifacts/RC-pre-1.0/findings-verification.md)
//! → The conversion ledger), and **the 1.0.0 call is gated on it closing**
//! ([decisions-pending.md](../../../implementation/decisions-pending.md) → Acceptance).
//! Two of them are axis gaps in M47's own work and name the sibling that *is* fenced:
//! F3 (`provision` destroys a leftover's uncommitted work, while `milestone_discard.rs`
//! and `uninstall_worktree_guard.rs` fence the same destruction at two other doors) and
//! F6 (a same-slug **same-H1** rename, whose same-slug/**different**-H1 sibling is
//! fenced at `flow37_rename::placement_same_slug_retitle_succeeds`).

// This file only aggregates; the `support` module it used to declare is now declared
// once by the group root, and each repro block below reaches it as `crate::support`.

#[path = "pinned_facts/address_grammar.rs"]
mod address_grammar;
#[path = "pinned_facts/code_anchor_bare_path.rs"]
mod code_anchor_bare_path;
#[path = "pinned_facts/doc_show_address_depth.rs"]
mod doc_show_address_depth;
#[path = "pinned_facts/file_state_transactional.rs"]
mod file_state_transactional;
#[path = "pinned_facts/finalize_json.rs"]
mod finalize_json;
#[path = "pinned_facts/finalize_left_out_stream.rs"]
mod finalize_left_out_stream;
#[path = "pinned_facts/finding_emission_order.rs"]
mod finding_emission_order;
#[path = "pinned_facts/form_vision_grounding.rs"]
mod form_vision_grounding;
