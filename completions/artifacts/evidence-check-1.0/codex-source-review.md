<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: OpenAI Codex; see VERDICT.md -->
## Verdict

**Do not ship 1.0.0 from rc.14.**

There is a source-visible path traversal/data-loss defect outside the M50 registries:

> `jigc migrate <path>` accepts an absolute or escaping path, reads outside the repository, records that path as a retirement, and `finalize` can delete it.

That directly falsifies M50’s broad claim that no caller-supplied token reaches a path component without its family’s validation. It is also materially worse than any of the 13 trial findings.

## Severity ranking

1. **CRITICAL — `migrate <path>` can read and later delete a file outside the repository.**
2. **HIGH — finalize rollback does not restore the complete pre-finalize worktree state.**
3. **HIGH process risk — the conversion ledger is substantively open under the repository’s own pinning rule.**
4. **MEDIUM — “nothing blocking” and “zero regressions” exceed the trial’s coverage.**
5. **MEDIUM — F-9/F-10 force a real worker outside the adapter after jigc itself creates a stale commit message.**
6. **LOW–MEDIUM — several trial findings require post-1.0 contract-version work if deferred.**

---

## A. What the trial evidence supports

The trial supports a narrow result:

- Three scored plant-E sessions used `doc show --task` as their first plant-document read, all on invocation 3, and recorded no filesystem read of that document. [trial-record.md:30](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/trial-record.md:30>)
- The record correctly limits that to “compliance at N=3,” not reliability. [trial-record.md:62](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/trial-record.md:62>)
- B3-strict points against generalization: its first attempt was the same raw `.jigc/tasks/...` filesystem read seen in RC-m50, and the harness—not product behavior—stopped it. [trial-record.md:70](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/trial-record.md:70>)

It does **not** establish the unrestricted headline “zero data loss, zero corruption, zero regressions, nothing blocking”:

- Four changed surfaces—join’s blocked rendering, both Fix fan-out outputs, and the `1799a2d` milestone boundary—were never reached. [coverage.md:21](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/coverage.md:21>) [coverage.md:32](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/coverage.md:32>)
- The root-knob refusal was also untried. [coverage.md:14](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/coverage.md:14>)
- The trial itself says a `squash:false` boundary remains unmeasured on rc.14. [coverage.md:40](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/coverage.md:40>)
- F-10 is explicitly classified as a “blocking dead end”; it is placed outside the blocking row only because raw git is deemed an available recovery. [findings-verification.md:299](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:299>) [findings-verification.md:318](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:318>)
- F-13 prevented B4-h from completing its milestone at all. [findings-verification.md:418](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:418>)
- The supposedly green M50 handover was false: a standing test was failing at the certified SHA. [pre-trial-findings.md:14](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/pre-trial-findings.md:14>) [pre-trial-findings.md:46](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/pre-trial-findings.md:46>)

The defensible wording is: **“No data loss or corruption was observed in the reached trial paths.”** The record’s categorical wording is not defensible.

### Instrument contamination

Yes, several instrument defects could have contaminated figures:

- Thirteen reader tests had never executed, including both tests intended to classify the headline duress cell. [pre-trial-findings.md:66](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/pre-trial-findings.md:66>)
- The reader originally ignored delegated transcripts; the record says that could have yielded three falsely clean cells, including one scored headline arm. [trial-record.md:164](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/trial-record.md:164>)
- FILESYSTEM counted attempts rather than successful reads. It did not alter the three permissive scored arms, but it makes any denying-posture figure only an upper bound. [pre-trial-findings.md:154](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/pre-trial-findings.md:154>) [pre-trial-findings.md:173](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/pre-trial-findings.md:173>)
- There was initially no write channel, so B1’s raw `git reset` and `git commit` bypass scored clean. [session-findings.md:143](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/session-findings.md:143>)
- One F1 walk bar initially passed for the wrong reason. [coverage.md:62](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/coverage.md:62>)
- Seven cleanup sites initially converted the intended rc.14 refusal into apparent regression failures. [session-findings.md:137](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/session-findings.md:137>)

The corrections appear to have occurred before final scored figures, so I cannot show that the final 3/3 number is wrong. I can show that the original apparatus was capable of producing falsely clean headline cells.

There is also sloppy counting: the record says “23 arms” [trial-record.md:25](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/trial-record.md:25>), while coverage says 24 including arm 00. [coverage.md:56](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/coverage.md:56>)

---

## B. Conversion ledger

**Substantively open, not merely formally open.**

The repository’s rule says confirmed claims become the fix’s red standing test; `UNPINNED` is an audit exception, not a substitute for conversion. [pinning.md:34](</Users/maurice/projects/gherrink-jigc/implementation/pinning.md:34>) [pinning.md:54](</Users/maurice/projects/gherrink-jigc/implementation/pinning.md:54>)

Concrete examples:

- **F-5 is CONFIRMED**, affects 22 doors, and explicitly has no standing test for the unknown-ID cell. Its `UNPINNED` explanation says the existing suite covers only malformed IDs. [findings-verification.md:172](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:172>) [findings-verification.md:200](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:200>)
- **F-11 is CONFIRMED** and explicitly sits upstream of all existing write-miss tests. [findings-verification.md:333](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:333>) [findings-verification.md:367](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:367>)
- **F-9 is CONFIRMED**, but the cited rename suite asserts only rename acknowledgements/referrer repointing, not the stale commit-doc interaction. [findings-verification.md:253](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:253>) [findings-verification.md:293](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:293>)
- **F-3 is CONFIRMED** and the ledger admits no repository suite asserts it. [findings-verification.md:97](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:97>) [findings-verification.md:129](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:129>)

“Pinning the current defect would pin the gap” confuses a regression test for desired behavior with an expected-output test for broken behavior. The project’s own rule says the repro becomes the fix’s red test; it need not assert that the defect is desirable.

---

## C. Unvalidated filesystem/git door

### Critical: `jigc migrate <path>`

The clap documentation calls this a **repo-relative** path. [cli.rs:224](</Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:224>)

The implementation does not enforce that:

1. It directly computes `repo_root.join(path)` and reads it. [migrate.rs:252](</Users/maurice/projects/gherrink-jigc/crates/cli/src/migrate.rs:252>)
2. Its “repo-relative” normalizer strips the repository prefix only if present; otherwise it retains the supplied path and preserves root components. [migrate.rs:189](</Users/maurice/projects/gherrink-jigc/crates/cli/src/migrate.rs:189>)
3. Finalize accepts that recorded value as a retirement without checking it is relative, component-safe, or beneath the repository. [finalize.rs:400](</Users/maurice/projects/gherrink-jigc/crates/engine/src/finalize.rs:400>)
4. The executor joins the retirement onto `repo_root`, reads it, captures its bytes, and calls `remove_file`. [task.rs:3016](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3016>)

For an absolute `path`, Rust path joining uses the absolute operand as the result. Therefore, after the ordinary migration approval/replacement gates, finalize can delete that external file. The rollback captures the bytes if a later operation fails, but a successful commit does not restore them.

The registries miss this by design: `ArgToken::Plain` expressly exempts `path`, `from`, `file`, `from_file`, and `target`, saying their doors will adjudicate them. [cli.rs:1500](</Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:1500>) The migrate door does not.

This is the uncovered door the M50 claim overlooks.

### Other reviewed sinks

The work-unit seam itself is guarded before `.join(id)`. [task.rs:1136](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:1136>)

Owner-artifact paths do have an explicit relative/`..`-free gate before reaching git. [task.rs:3103](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3103>) [task.rs:3134](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3134>)

Relocation’s `--from` parser accepts absolute and parent-containing homes, but its source candidates come from git’s committed repo-relative enumeration, so I did not prove an external destructive path there. [relocate.rs:126](</Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:126>) [relocate.rs:176](</Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:176>)

---

## D. Finalize/rollback

The index rollback is extensively family-specific and appears to cover promotions, retirements, owner artifacts, config-layer paths, and milestone records. [task.rs:2730](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2730>) [task.rs:2856](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2856>)

The **worktree**, however, is not restored to its complete pre-finalize state:

- `gitignore::ensure` mutates `.jigc/.gitignore` before the config-index capture. [task.rs:2774](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2774>)
- `refresh_version_stamp` rewrites `.jigc/version`. [task.rs:3403](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3403>)
- The config rollback intentionally restores only the index and leaves the worktree untouched. [task.rs:3357](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:3357>)

Thus a stage failure or hook rejection can leave `.jigc/version` and potentially `.jigc/.gitignore` worktree bytes different from the pre-finalize state even though no commit landed. That contradicts a strong “as if finalize never ran” interpretation, although the design’s config-layer row explicitly promises only index restoration. [finalize.md:178](</Users/maurice/projects/gherrink-jigc/design/finalize.md:178>)

There is also a narrower failure-order concern: promotion and retirement occur before owner/promotion/config index captures. [task.rs:2760](</Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2760>) A capture failure invokes rollback with the relevant index vectors still empty. The index has not yet been modified at that point, so that ordering is safe for the index; the worktree mutations above remain.

I did not find an uncovered pre-image among the enumerated index families. I did find incomplete worktree restoration.

---

## E. Contract impact of fixing the 13 findings

Some fixes are contract-free:

- **F-4, F-9, F-12:** help/advisory/text changes.
- **F-10:** adding a new top-level verb need not change existing JSON.
- **F-13:** a new execution mode can be additive if it does not alter existing envelopes.
- **F-2:** changing staging behavior can preserve envelope shapes.

Some require deliberate contract work:

- **F-5 and F-11:** converting code-less/raw errors into structured findings changes their machine-visible error behavior. The findings envelope is pinned, and post-1.0 shapes require an explicitly versioned extension. [command-output-contract.md:446](</Users/maurice/projects/gherrink-jigc/design/command-output-contract.md:446>)
- **F-6/N27:** adding `workflow` and `intent` to `task diff --format json` changes a pinned unversioned command-output shape; after 1.0 it requires a named versioned extension. [findings-verification.md:206](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:206>) [doc-read-surface.md:184](</Users/maurice/projects/gherrink-jigc/design/doc-read-surface.md:184>)
- **F-3:** implementing conditional field/ref requirements in a frozen doctype schema likely changes the schema projection and frozen schema hash; `doc schema` requires a contract-version bump for any structural projection change. [doc-read-surface.md:183](</Users/maurice/projects/gherrink-jigc/design/doc-read-surface.md:183>)
- **F-8:** adding `adr → research` changes the frozen ADR schema and `doc schema` projection, requiring a doctype schema-version/hash update, corpus migration as applicable, and `doc schema` contract-version bump. The repository’s frozen-schema invariant explicitly requires a version bump and migration for shape changes. [CLAUDE.md:26](</Users/maurice/projects/gherrink-jigc/CLAUDE.md:26>)
- **F-1 and most of F-12 are refuted**, so no substantive product fix is owed. [findings-verification.md:17](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:17>) [findings-verification.md:373](</Users/maurice/projects/gherrink-jigc/completions/artifacts/RC-rc14/findings-verification.md:373>)

Fixing the newly found `migrate <path>` defect need not break a pinned JSON shape: reject out-of-repository paths with an existing or new structured finding. A new error code would expand the registry, but that is preferable to shipping external-file deletion.

---

## F. Release recommendation

**Do not ship. Required before 1.0:**

1. Reject absolute, escaping, symlink-escaping, and otherwise non-repository `migrate <path>` values before reading or minting a task.
2. Revalidate the recorded `source-path` at finalize, because working-area files are mutable and the destructive sink must defend itself.
3. Add standing tests proving both the read and retirement paths cannot escape the repository.
4. Close the conversion ledger with real red-to-green tests for at least F-5, F-9, and F-11.
5. Decide whether rollback promises index fidelity or full worktree fidelity; make the implementation and documentation say the same thing.
6. Exercise the unreached fan-out/join/finalize boundary before claiming “nothing blocking.”

I did not run Cargo, execute the binary, or modify files, as requested. The critical migrate finding is based on direct source tracing rather than a live reproduction. HEAD is `bd348a83`, while the trial binary was built from `21ffc0d4`; the intervening commits shown in the repository history are trial tooling, evidence, tests, and documentation rather than product implementation, but I did not rebuild either revision.