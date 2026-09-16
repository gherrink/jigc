<!-- M51 per-axis review — axis 8 · Codex source pass, verbatim · read against `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

## Claims

1. **`migrate-corpus --dry-run` does write to disk when the invocation log is enabled, contradicting both its help and the installed migration guide’s unconditional “changes nothing” claim.**

   - Evidence: `crates/cli/src/cli.rs:74-75` says `--dry-run` “writes nothing at all”; `MIGRATING.md:27` says it “changes nothing.” However, every invocation appends a record when logging is enabled (`crates/cli/src/main.rs:92-98`), creating/appending `.jigc/logs/invocations.jsonl` (`crates/cli/src/invocation_log.rs:431-461`). The supported enabling command is evidenced by `crates/cli/tests/invocation_log.rs:94-97`.
   - Proposed reproduction: in a configured repository run `jigc config set invocation-log true`; record the size or hash of `.jigc/logs/invocations.jsonl`; run `jigc migrate-corpus --dry-run`; observe that the log file grew despite help saying the invocation writes nothing at all.
   - Confidence: **High**.

2. **The installed quickstart falsely says every `jigc setup` rewrites the guide, although setup deliberately leaves a modified or pre-existing user guide untouched.**

   - Evidence: `QUICKSTART.md:57-62` says setup writes the combined guide and “every `jigc setup` rewrites it.” Ownership checking classifies unstamped, edited, unreadable, or non-UTF-8 content as user-modified (`crates/cli/src/setup.rs:172-222`). Such a file is explicitly left untouched (`crates/cli/src/setup.rs:260-274`), excluded from the write and install pathspec (`crates/cli/src/setup.rs:1336-1357`). The guide is embedded directly from QUICKSTART/MIGRATING (`crates/cli/src/setup.rs:66-71`), so the false universal reaches the installed skill.
   - Proposed reproduction: create a fresh git repository; create `.claude/skills/jigc/SKILL.md` containing user text; run `jigc setup`; expect exit 0 with `adapter-guide.user-modified`, while the file remains unchanged instead of being rewritten as the guide promises. The same mismatch appears after editing a previously installed guide and rerunning `jigc setup`.
   - Confidence: **High**.

3. **The installed quickstart presents a source-tree-relative Cargo command as the single install and upgrade command, but that command is normally invalid from the adopter repository where setup installs the guide.**

   - Evidence: `QUICKSTART.md:15-26` declares one install command, `cargo install --path crates/cli`, and says an upgrade uses the same line. Setup embeds those exact guide bytes (`crates/cli/src/setup.rs:66-71`) and installs them in the adopter repository at `.claude/skills/jigc/SKILL.md` (`crates/cli/adapters/claude-code.yaml:45-49`). The generated preamble expressly says the other jigc project files do not live in the adopter repository (`crates/cli/src/setup.rs:73-87`), which also means the referenced `crates/cli` source path ordinarily is not there.
   - Proposed reproduction: install/copy `jigc` into `PATH`; in a fresh unrelated git repository run `jigc setup`; from that repository follow the installed skill’s upgrade command, `cargo install --path crates/cli`; expect Cargo to fail because `<repo>/crates/cli` does not exist.
   - Confidence: **High**.

## Read and found consistent

- Read `CLAUDE.md`, M51 Increments 9–10 in `implementation/roadmap.md`, `crates/cli/src/adapter.rs`, the setup guide-generation/ownership/commit consumers, all `about`/`long_about` declarations in `crates/cli/src/cli.rs`, and both guides line by line. Grepped all consumers of the named registries and guide symbols.
- `migrate-corpus --help` derives its complete change-kind list from `SchemaChangeKind::ALL` (`crates/cli/src/cli.rs:45-85`).
- `validate --help` derives its family list from `STORE_FAMILIES` (`crates/cli/src/cli.rs:88-115`; registry at `crates/engine/src/validate.rs:473`).
- `task finalize --help` uses `whats_left_coverage()` rather than duplicating the gate set (`crates/cli/src/task.rs:198-212`; registry at `crates/cli/src/gate_coverage.rs:118-214`).
- `doc show --help` renders `WHOLE_DOC_KEYS`, including all six committed keys and the separately declared staged key (`crates/cli/src/doc.rs:5146-5189,5193-5220`).
- The guide artifact is stamped with binary version and its body hash, and ownership is checked before replacement (`crates/cli/src/setup.rs:151-222`).
- The scoped `--no-verify` account is consistent: setup alone builds its install commit with `--no-verify` (`crates/cli/src/setup.rs:2424-2427`), matching `QUICKSTART.md:180-196` and `MIGRATING.md:50-51`.
- The migration triage vocabulary, ahead-version direction, manifest tag vocabulary, and widened setup/finalize carryover-gate subject are present in `MIGRATING.md:25-30,47-52` and `QUICKSTART.md:96-109,198-219`; I found no omitted typed list behind those statements.