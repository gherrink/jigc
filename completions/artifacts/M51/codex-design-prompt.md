<!-- M51 · the verbatim prompt that produced [codex-design-review.md](codex-design-review.md), persisted so the review's scope and its seven directed questions (A–G) are auditable rather than inferred from the answer. -->

You are an independent, adversarial reviewer of a SETTLED design before it is decomposed into build increments. Repository: jigc (Rust CLI), HEAD bd348a83 plus uncommitted planning files. Read-only; do not run cargo; do not modify files.

Read in this order: completions/artifacts/M51/settle-record.md (fifteen decisions D1–D15), completions/artifacts/M51/charter.md, completions/artifacts/M51/baseline-ledger.md, completions/artifacts/M51/gap-findings.md, the 2026-09-11 entry at the top of DECISIONS.md, and completions/artifacts/evidence-check-1.0/VERDICT.md for the underlying findings (your earlier review is archived as completions/artifacts/evidence-check-1.0/codex-source-review.md — your migrate-path claim was reproduced live and became D1). Then the code each decision touches: crates/cli/src/{migrate.rs,task.rs,setup.rs,milestone.rs,config.rs,render.rs,invocation_log.rs,trackable.rs,repo.rs,gitignore.rs,cli.rs}, crates/engine/src/{finalize.rs,state.rs,milestone.rs}, crates/cli/tests/{format_json_success_axis.rs,text_json_parity_axis.rs,foldback_truth.rs}.

Try to break the settled design. Specifically:
A. D1 (path-argument registry + CLI-side validation of plan.retirements): is there any path by which an out-of-repo, .git/, or symlinked file is still deleted or read after D1 as specified? Is the six-member family complete — grep every clap argument that becomes a Path::join or a git argv element.
B. D2 (posture family of three, refuse, a superset registry holding setup, DedicatedWorktree as a typed argument, probe at the door and at the ff): find a committing/moving door the registry would miss, a legitimate workflow the refusal breaks, or a place the exemption can be forged.
C. D3 (setup refuses over bytes it did not write): can setup distinguish "bytes I did not write" from "bytes my previous install wrote" idempotently across re-runs and upgrades? What happens on a fresh clone of a repo where setup already ran?
D. D4 (config-layer worktree pre-image + gitignore amend-to-union): any path where the restore clobbers a concurrent user edit, or where amend-to-union is not byte-idempotent across the four callers?
E. D5 (pinned-envelope registry over verb × arm): does any verb have more arms than the design counts? Is the (path, arm) bijection sound against clap's tree? Which of the 17 undeclared keys would you delete rather than declare, and why?
F. Contradictions between decisions, missing decisions (a fork the Settle picked silently), and any decision that moves a pinned JSON shape or a frozen schema hash despite the wave's zero-bump bound.
G. Verdict: is this design ready to decompose? Rank findings by severity with file:line and a concrete correction each. Concrete, unsoftened; say what you could not verify.

