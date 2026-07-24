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

mod support;

#[path = "pinned_facts/address_grammar.rs"]
mod address_grammar;
#[path = "pinned_facts/file_state_transactional.rs"]
mod file_state_transactional;
#[path = "pinned_facts/finalize_json.rs"]
mod finalize_json;
#[path = "pinned_facts/form_vision_grounding.rs"]
mod form_vision_grounding;
