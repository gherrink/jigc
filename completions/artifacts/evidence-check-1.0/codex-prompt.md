<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: OpenAI Codex; see VERDICT.md -->
You are an independent, adversarial reviewer. The repository is jigc (a Rust CLI, a "context compiler for coding agents"). The maintainer is about to decide whether to ship 1.0.0 from 1.0.0-rc.14 (HEAD bd348a83). Your job: try to break the case for 1.0.0. Read-only; do not run cargo; do not modify files.

Read, in this order:
1. CLAUDE.md — the "Project state" paragraph's last third (from "M50 —" to the end) and "Architectural invariants".
2. completions/artifacts/RC-rc14/trial-record.md, findings-verification.md, coverage.md, session-findings.md, pre-trial-findings.md.
3. implementation/decisions-pending.md → the section "The rc.14 trial's findings — owed dispositions" and the six carried M50-close defects (N20, N23, N26, N27, N28 + one more).
4. implementation/pinning.md §3 and design/command-output-contract.md, design/doc-read-surface.md (what 1.0 pins).
5. Then the code: crates/cli/src (the verb modules, especially task.rs, milestone.rs, doc.rs, rename.rs, config, uninstall, the finalize/rollback path) and crates/engine/src (write.rs, file_state.rs, milestone.rs).

Answer, with file:line evidence for every claim:

A. Does the trial evidence support "zero data loss, zero corruption, zero regressions, nothing blocking" on rc.14? Where does the record's own prose outrun its evidence? The trial found seven defects in its own instrument — could any of them have contaminated a scored figure?

B. Is the conversion ledger substantively open (a CONFIRMED defect with no standing test hiding behind an "UNPINNED: <why>") or only formally open?

C. Reading the code, not the record: find any path where a user- or agent-supplied token (id, slug, path, config value, address) becomes a filesystem path or a git argument without the validation the M50 wave claims (grep for `work-unit.malformed-id`, `store.malformed-slug`, `write.malformed-slug`, `config.unusable-root`, `WORK_UNIT_ID_DOORS`, `SLUG_DOORS`, `ROOT_KNOBS`). Name any door the registries do not cover. Look especially at: `remove_dir_all`, `fs::rename`, `git rm`, `git mv`, `git checkout`, `git reset`, `git clean`, worktree removal, and every `Path::join` on a caller-supplied string.

D. Finalize/rollback: is there a failure point where the index or worktree is left different from the pre-finalize state, or a pre-image is captured for one staged-path family but not another (added / modified / deleted / renamed / untracked-foreign / owner-artifact)?

E. Would fixing any of the 13 rc.14 findings require breaking a contract 1.0 pins (JSON shapes, contract-versions, frozen schema hashes, error-code registry)? Which?

F. Your verdict: ship 1.0.0 from rc.14 as-is / ship after fixing X / do not ship, because Y. Rank what you found by severity. Be concrete and unsoftened; say what you could not verify.
