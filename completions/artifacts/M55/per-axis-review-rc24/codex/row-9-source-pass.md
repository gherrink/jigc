<!-- The unseeded Codex source pass for ROW 9 (adopter docs & help), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

## Claims

1. **The new amend seam contradicts `task amend --help`: help says the message is authored “from scratch” and HEAD’s message is not read back, but a human amend silently reads HEAD and preserves its declared co-author trailer.**

   - Evidence: `amend_long_about` says “HEAD’s message is not read back into the doc: you author the new message from scratch” at `crates/cli/src/task.rs:355-367`. The new seam nevertheless reads `git log -1 --format=%B HEAD`, detects the profile’s trailer, and injects it into the replacement message at `crates/cli/src/task.rs:8044-8055`; `git_commit_amend` selects this `SessionOrHead` behavior at `crates/cli/src/task.rs:8009-8019`.
   - Proposed reproduction: first run `CLAUDECODE=1 jigc task finalize <task>` to create an agent-signed commit. Then, without `CLAUDECODE`, run `jigc task amend`, author a replacement commit doc containing no trailers, and finalize it. Expected from help: the replacement message consists solely of the newly authored doc. Actual source behavior: the old `Co-Authored-By` trailer is copied from HEAD into the amended commit.
   - Classification: **tier 3**, a help/behavior contradiction inside rc.24’s new trailer code; it does not meet the exit rule’s tier-1 loss/repository-harm predicate because the design explicitly intends the preservation.
   - Confidence: **high**.

2. **Baseline `(8, N-2)` remains open: successful retitle-only `jigc rename` still omits the changed title from both its text acknowledgement and permanent commit subject.**

   - Evidence: the operation retains the new title in `RenameReport.title` at `crates/cli/src/rename.rs:300-315`, but the successful text renderer prints only identities, paths, and referrer count at `crates/cli/src/render.rs:4758-4766`. The commit subject remains `rename {old_rel} -> {new_rel}` at `crates/cli/src/rename.rs:981-989`; on a retitle-only operation those paths are equal.
   - Proposed reproduction: on a fixed-identity doc, run `jigc rename vision:vision --to "New Vision" --slug vision`. Expect the success acknowledgement and commit record to name “New Vision”; instead both describe `VISION.md -> VISION.md`, while the H1 changes.
   - Classification: the previously triaged **tier-3 row remains STILL-OPEN for 1.x**, as expected; no regression or new tier-1 row.
   - Confidence: **high**.

## Baseline dispositions

- **`(8, N-1)` — CLOSED.** The uninstall long help now enumerates all four guards, including `uninstall.foreign-bytes`, and says `--force` deletes all four at `crates/cli/src/cli.rs:290-314`. The flag help still abbreviates the population at `cli.rs:316-319`, but no longer claims the omitted fourth guard does not exist; its “all three guards” wording refers to the three categories in that flag sentence. The implementing seam likewise documents four guards at `crates/cli/src/setup.rs:2952-2963`.
- **`(8, N-2)` — STILL-OPEN**, as Claim 2 above. This is a known tier-3 row triaged to 1.x and does not block the call.

## Consistent source read

- The rc.24 trailer is named in `design/assistant-adapter.md:157-168` and `.cargo/config.toml:1-8`, but **no adopter guide, CLI help, or composed step explains that jigc now adds it automatically**. `step:author-commit` still tells agents to add a `Co-Authored-By` item manually at `crates/cli/packs/dev/steps/author-commit.yaml:39-44`. That remains operationally compatible because the seam de-duplicates the same email at `crates/cli/src/adapter.rs:167-208`; the instruction is therefore redundant, not double-writing.
- Every registered committing door funnels through the signing seam: `COMMITTING_DOORS` is consumed by the trailer axis test, while setup’s `--no-verify` exclusion signs separately at `crates/cli/src/setup.rs:2678-2691`. I found no bypassing production `git commit` site.
- The M55 surfaces are registry-backed: `doc show` renders its key list from `WHOLE_DOC_KEYS`, including `title`, at `crates/cli/src/doc.rs:5661-5679,5710-5718`; `task finalize` renders gate coverage and amend commit wording from their registries at `crates/cli/src/task.rs:199-224`; committing-door clauses live in `crates/cli/src/invocation_log.rs:188-245`.
- `report-inconsistency` is router-visible and carries a `when:` hint at `crates/cli/packs/methodology/workflows/report-inconsistency.yaml:1-8`; its description agrees with its create-only gate and doc-only finalize steps at lines 10-12.
- The guide seam embeds the two crate-local guides and no root copies at `crates/cli/src/setup.rs:94-99`; it strips repository-relative links before installation at `setup.rs:131-189`, stamps the artifact at `setup.rs:192-210`, and refuses to overwrite a user-modified body at `setup.rs:213-220`.
- `crates/cli/README.md` is byte-identical to `dev/crate-readme --stdout`; Cargo names it at `crates/cli/Cargo.toml:11-15`. The independent fence checks transform equality, rewritten-link order, absence of relative crate links, and target existence at `crates/cli/tests/crate_readme.rs:104-163`.
- The install line occurs only in QUICKSTART, the allowed root README copy, and the generated crate README; the census and byte-equality fence are at `crates/cli/tests/install_line.rs:80-131`.
- MIGRATING correctly identifies the orphaned `doc-code` left by installs through rc.21. I found no surviving clone-based install instruction, second-binary requirement, false `--no-verify`, dry-run, migration-key, or manifest-vocabulary claim in the scoped guides.

## Schema boundary and bounds

The boundary holds. M55 adds exactly the `jigc-feedback` and `inconsistency` methodology manifest rows at schema-version 1; M54 moves no hash. The rc.23→rc.24 diff changes only the presentation text of `planning-record.yaml`; no manifest hash, schema version, or pinned JSON key moves. The trailer changes no schema.

This was a **source-only pass against tag `jigc-v1.0.0-rc.24` (`91834b5e`)**. I drove no binary and wrote nothing. The crate-README comparison invoked only the generator’s read-only `--stdout` mode. The fences do not prove remote HTTP availability, markdown forms outside their declared lexer grammar, or runtime behavior; those remain for the independent binary driver.