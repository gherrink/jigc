<!-- The unseeded Codex source pass for ROW 10 (co-author trailer), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

## Claims

1. **A commit can be attributed to a different co-author profile than the profile used by `jigc setup`, because every commit reloads the current `JIGC_ADAPTERS_DIR` source rather than retaining the installed profile’s identity.**

   - Evidence: setup loads the profile selected at installation at `crates/cli/src/setup.rs:1165`; commit attribution independently calls `declared_co_author()` at `crates/cli/src/adapter.rs:250-257`; that function reloads the profile, and `load_profile` reads the process’s current `JIGC_ADAPTERS_DIR` at `crates/cli/src/adapter.rs:1537-1557`. The resulting identity is applied at `crates/cli/src/task.rs:8162-8188` and independently by setup’s commit at `crates/cli/src/setup.rs:2681-2690`.
   - Reproduction: run `JIGC_ADAPTERS_DIR=/tmp/profile-a jigc setup`, where profile A declares `Claude <a@example.com>`; then start and author a normal task, and finalize it with `CLAUDECODE=1 JIGC_ADAPTERS_DIR=/tmp/profile-b jigc task finalize <id>`, where profile B declares `Mallory <b@example.com>`. Expected wrong behavior: the commit exits 0 and carries `Co-Authored-By: Mallory <b@example.com>`, although setup installed profile A. The same substitution applies to every seam-backed door and to amend carry-over. An unreadable or malformed profile B instead silently contributes no trailer because `declared_co_author()` discards the load error at `adapter.rs:256-257`.
   - Confidence: **high** from the data flow; not driven, per the source-pass boundary. This is an exit-0 false-attribution path through committing doors and is therefore relevant to the tier-1 exit rule.

## Design-sentence dispositions

For `design/assistant-adapter.md` → “The co-author trailer”:

- **Opening claim:** carried except for “the profile its agent names” meaning the profile setup installed; commits use whichever profile the current environment reloads. The eleven registered constructions route through the seam (`task.rs:4924-4969`, `8432-8472`, `8490-8508`; `milestone.rs:1350-1396`; `rename.rs:981-993`; `migrate_corpus.rs:499-510`), and setup signs separately (`setup.rs:2681-2690`). `COMMITTING_DOORS` contains eleven rows at `invocation_log.rs:188-245`.
- **“The commit doctype is untouched…”:** carried. Signing occurs after render, immediately before Git, in `task.rs:8162-8198`; no trailer field was added to the commit schema.
- **Which profile, sentence 1:** **not carried under an environment change**, per Claim 1. `SETUP_ASSISTANT` is consistently `claude-code` (`setup.rs:1046-1049`), but its bytes are reselected live.
- **Which profile, sentence 2:** carried: there is no stored per-repository assistant selection; setup and commit use the constant above.
- **Which profile, sentence 3:** prospective and not source-testable in this range.
- **When, sentence 1:** carried by `active_under`, which requires a present non-empty value (`adapter.rs:160-165`).
- **When, sentences 2–4:** carried by the same conditional and profile-owned `when_env`; the engine is uninvolved (`adapter.rs:103-106`, `244-257`).
- **When, sentence 5:** documentation/external-environment rationale, not independently source-verifiable.
- **Where, sentence 1:** carried: `git_commit_capture` delegates to the owning seam (`task.rs:8147-8163`), which owns `-F`/`-m` (`8165-8198`).
- **Where, sentence 2:** carried by setup’s separate `session_co_author().sign(...)` before `--no-verify` (`setup.rs:2681-2690`).
- **Placement, sentence 1:** carried: local rendering and block placement are at `adapter.rs:171-192`; no `git --trailer` call exists.
- **Placement, sentence 2:** carried: address and key are compared case-insensitively, within the detected final trailer block (`adapter.rs:171-173`, `194-208`, `224-242`); an already-matching block returns the original bytes (`181-184`).
- **Amend sentence:** carried: `SessionOrHead` reads `HEAD` and preserves a declared address already present (`task.rs:8033-8055`), then signs the re-authored `-F` file.
- **No-key sentence:** carried by `Option<CoAuthor>` and `declared_co_author()` (`adapter.rs:73-77`, `256-257`).
- **Malformed-declaration sentence:** carried at setup through `setup.profile-load` (`setup.rs:1165-1171`) and `TryFrom<RawCoAuthor>` validation (`adapter.rs:109-147`). At commit, malformed/unreadable live overrides contribute nothing because the error is swallowed (`256-257`).

Declared bounds:

- Human finalization without the session variable adds no trailer (`adapter.rs:163-165`, `250-252`).
- A human command inside an agent environment is indistinguishable and is signed by that same check.
- Signing cannot change `subject` or `committed.subject`: it appends after the first line (`adapter.rs:181-192`); the success renderer continues to use `landed.subject` (`render.rs:2973-2985`).
- Conventional-Commit headers remain the first line for the same reason.
- Cargo forces `CLAUDECODE` empty for subprocesses at `.cargo/config.toml:1-8`.
- The integration suite overrides the child environment per cell at `agent_co_author.rs:56-81`.

The DECISIONS claim, **“the one commit seam, all eleven COMMITTING_DOORS, plus setup’s install commit,” is carried as a construction census**: eleven rows at `invocation_log.rs:188-245`; all production hook-capable `git commit` calls converge at `task.rs:8191-8198`; setup is the sole direct production commit at `setup.rs:2689-2691`. The only production `merge` is `--ff-only`, which creates no new commit (`task.rs:8397-8403`); no production `commit-tree` remains.

## Consistent completeness read

I read the requested history/verdict bounds, M55 §12, both settled decisions, the complete trailer design, finalize amend/rendering sections, all named registries and consumers, both author-commit steps, the co-author tests, Cargo environment, and every production `commit`, `merge`, and `commit-tree` occurrence. I found no unregistered committing door and no commit bypass beyond setup’s declared exclusion. Path-scoped doc-only commits and every `squash=false` per-sub-task plus aggregate commit use the seam.

`agent_co_author.rs` covers the registry sweep, human/agent sessions, setup, same-address/different-name dedupe, amend preservation, fan-out modes, doc-only commits, and absent key. It does **not** cover an installed-profile/current-profile mismatch, empty `CLAUDECODE` through the binary, unreadable profile at commit, hook rejection followed by a differently sourced profile, CRLF/trailing-whitespace integration behavior, or pinned JSON output directly.

No schema-boundary violation found: the trailer changed no schema, manifest row, or pinned JSON key. The rc.24 planning-record change is the declared hint-only reword; no schema-manifest hash moved. M55’s two methodology rows remain the only stated additions, both schema-version 1. No files were written and no binary was driven.